use crate::db;
use anyhow;
use axum::{
    body::BoxBody,
    http::{
        header::{HeaderMap, HeaderValue, AUTHORIZATION},
        Request, StatusCode,
    },
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use entity::sea_orm::DatabaseConnection;
use futures::future::BoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{instrument, trace};

pub const JWT_SECRET: &str = "s_Ec_retg95k20ls";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct TokenAuth {
    pub auth: bool,
    pub conn: DatabaseConnection
}

impl<B> AsyncAuthorizeRequest<B> for TokenAuth
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let conn = self.conn.clone();
        let auth = self.auth.clone();

        Box::pin(async move {
            if !auth {
                let user = db::get_first_user(&conn)
                    .await
                    .map_err(|_| get_internal_server_error_res(""))?
                    .unwrap_or(
                        db::create_user(&conn, "", 0)
                            .await
                            .map_err(|_| get_internal_server_error_res(""))?,
                    );
                request.extensions_mut().insert(user);
                return Ok(request);
            } else {
                match get_user_id_from_header(request.headers()) {
                    Ok(id) => match db::find_user_by_id(&conn, id).await {
                        Ok(Some(mut user)) => {
                            if user.max_query < 0 {
                                request.extensions_mut().insert(user);
                                Ok(request)
                            } else if Utc::now() - user.reset_at > Duration::days(1) {
                                user = db::set_user_remain(&conn, user.max_query - 1, user)
                                    .await
                                    .map_err(|_| get_internal_server_error_res(""))?;
                                request.extensions_mut().insert(user);
                                Ok(request)
                            } else if user.remain > 0 {
                                user = db::set_user_remain(&conn, user.remain - 1, user)
                                    .await
                                    .map_err(|_| get_internal_server_error_res(""))?;
                                request.extensions_mut().insert(user);
                                Ok(request)
                            } else {
                                trace!(id = id, "User request limit exceeded");
                                Err(get_limit_exceeded_res("user request limit exceeded"))
                            }
                        }
                        Ok(None) => {
                            trace!("User not found");
                            Err(get_unauthorized_res("user not found"))
                        }
                        Err(e) => {
                            trace!(error = %e,"DB error");
                            Err(get_internal_server_error_res("database error"))
                        }
                    },
                    Err(e) => {
                        trace!(
                            error = %e,
                            "Error getting user id from auth header, or expire"
                        );
                        Err(get_unauthorized_res("auth user fail"))
                    }
                }
            }
        })
    }
}

#[instrument]
fn get_user_id_from_header(headers: &HeaderMap<HeaderValue>) -> anyhow::Result<i32> {
    match headers.get(AUTHORIZATION) {
        Some(actual) => Ok(decode::<Claims>(
            actual.to_str()?.trim_start_matches("Bearer "),
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &Validation::default(),
        )?
        .claims
        .id),
        _ => Err(anyhow::anyhow!("don't get AUTHORIZATION header")),
    }
}

fn get_unauthorized_res(body: &str) -> Response {
    let mut res = body.to_string().into_response();
    *res.status_mut() = StatusCode::UNAUTHORIZED;
    res
}

fn get_internal_server_error_res(body: &str) -> Response {
    let mut res = body.to_string().into_response();
    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    res
}

fn get_limit_exceeded_res(body: &str) -> Response {
    let mut res = body.to_string().into_response();
    *res.status_mut() = StatusCode::TOO_MANY_REQUESTS;
    res
}
