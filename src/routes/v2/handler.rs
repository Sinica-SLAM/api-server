use crate::{
    db::{create_result, find_result_by_id, set_result_file_path, set_result_status},
    error::{HttpError, Result},
    routes::upload::{parse_multipart, save_file, MultipartContent},
};

use super::{
    command::{get_command_output, RESULT_PATH, SCRIPT_PREFIX},
    model::{IdResponse, RecYoutubeRequest},
};

use anyhow::anyhow;
use axum::{
    body::Bytes,
    extract::{Extension, Form, Multipart, Path, State},
    response::Json,
};
use entity::{results, users};
use sea_orm::DatabaseConnection;
use std::fs;
use tokio::process::Command;
use tracing::{debug, info_span, Instrument};
use uuid::Uuid;

pub async fn rec_upload(
    State(conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    mut multipart: Multipart,
) -> Result<Json<IdResponse>> {
    let MultipartContent {
        asr_kind,
        extension,
        file,
        output_type,
        lmwt,
        min_word_gap,
    } = parse_multipart(&mut multipart)
        .await
        .map_err(|e| HttpError::MultipartError(e.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let result = create_result(&conn, id.clone(), user.id).await?;

    let file_path = save_file(&format!("{}.{}", id, extension), &file)
        .map_err(|e| HttpError::Other(anyhow!(e.to_string())))?;

    let sub_command = format!(
        "{}/run_rec_upload.sh {} {} {} {} {} {}",
        SCRIPT_PREFIX, id, asr_kind, output_type, file_path, lmwt, min_word_gap
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    get_command_output(command, &id, Some(file_path), &conn, result);

    Ok(Json(IdResponse { id }))
}

pub async fn align_upload(
    State(conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    mut multipart: Multipart,
) -> Result<Json<IdResponse>> {
    let MultipartContent {
        asr_kind,
        extension,
        file,
        ..
    } = parse_multipart(&mut multipart)
        .await
        .map_err(|e| HttpError::MultipartError(e.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let result = create_result(&conn, id.clone(), user.id).await?;

    let file_path = save_file(&format!("{}.{}", id, extension), &file)?;

    let sub_command = format!(
        "{}/run_align_upload.sh {} {} {}",
        SCRIPT_PREFIX, id, asr_kind, file_path
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    get_command_output(command, &id, Some(file_path), &conn, result);

    Ok(Json(IdResponse { id }))
}

pub async fn rec_youtube(
    State(conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    form: Form<RecYoutubeRequest>,
) -> Result<Json<IdResponse>> {
    let request: RecYoutubeRequest = form.0;

    let id = Uuid::new_v4().to_string();
    let result = create_result(&conn, id.clone(), user.id).await?;

    let sub_command = format!(
        "{}/run_rec_youtube.sh {} {} {} {} {}",
        SCRIPT_PREFIX,
        id,
        request.asr_kind.unwrap_or("sa_me_2.0".to_string()),
        request.output_type.unwrap_or("srt".to_string()),
        request.vid,
        request.lmwt.unwrap_or("10".to_string())
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    get_command_output(command, &id, None, &conn, result);

    Ok(Json(IdResponse { id }))
}

pub async fn translation(
    State(conn): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    body: Bytes,
) -> Result<Json<IdResponse>> {
    let id = Uuid::new_v4().to_string();
    let id_clone = id.clone();
    let result_model = create_result(&conn, id.clone(), user.id).await?;

    tokio::spawn(
        async move {
            let id = id_clone;
            match async {
                let resp = reqwest::Client::new()
                    .post("http://0.0.0.0:8000/api/v1/translation")
                    .body(body)
                    .send()
                    .await?;

                if resp.status().is_success() {
                    let body = resp.bytes().await?;
                    let save_path = format!("{}/{}", RESULT_PATH, id);
                    fs::write(&save_path, body)?;

                    set_result_status(&conn, results::Status::Complete, result_model.clone())
                        .await?;

                    set_result_file_path(&conn, save_path, result_model.clone()).await?;
                    anyhow::Ok(())
                } else {
                    Err(anyhow!("translation failed, code: {}", resp.status()))
                }
            }
            .instrument(info_span!("inner_async"))
            .await
            {
                Ok(_) => (),
                Err(e) => {
                    debug!("tokio spawn error: {}", e.to_string());
                    set_result_status(&conn, results::Status::Fail, result_model)
                        .await
                        .expect("set result status error");
                }
            }
        }
        .instrument(info_span!("translation tokio block")),
    );

    Ok(Json(IdResponse { id }))
}

pub async fn result(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Vec<u8>> {
    let result = find_result_by_id(&conn, id).await?;
    match result {
        None => Err(HttpError::NotFound("Result not found".to_string())),
        Some(result) => match result.status {
            results::Status::Running => Err(HttpError::Conflict),
            results::Status::Complete => {
                let path = format!("{}/{}", RESULT_PATH, result.id);
                let file = fs::read(path).map_err(|e| HttpError::Other(anyhow!(e.to_string())));
                file
            }
            results::Status::Fail => Err(HttpError::ExpectationFailed),
        },
    }
}
