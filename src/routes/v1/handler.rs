use super::super::upload::{parse_multipart, save_file, MultipartContent};
use super::command::{get_command_output, SCRIPT_PREFIX};
use super::model::RecYoutubeRequest;
use crate::error::{HttpError, Result};
use axum::extract::{ContentLengthLimit, Form, Multipart};
use std::fs::{self, read, remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;
use tracing::debug;
use uuid::Uuid;

pub async fn rec_upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 1024 /* 250mb */
        },
    >,
) -> Result<Vec<u8>> {
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

    let file_path = save_file(&format!("{}.{}", id, extension), &file)?;

    let sub_command = format!(
        "{}/run_rec_upload.sh {} {} {} {} {} {}",
        SCRIPT_PREFIX, id, asr_kind, output_type, file_path, lmwt, min_word_gap
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    let result_path = get_command_output(&mut command).await?;
    let result_path = Path::new(&result_path);

    let result = read(result_path).map_err(|e| HttpError::NotFound(e.to_string()))?;
    remove_file(file_path)?;
    remove_dir_all(result_path.parent().unwrap().parent().unwrap())?;

    Ok(result)
}

pub async fn align_upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 1024 /* 250mb */
        },
    >,
) -> Result<Vec<u8>> {
    let MultipartContent {
        asr_kind,
        extension,
        file,
        ..
    } = parse_multipart(&mut multipart)
        .await
        .map_err(|e| HttpError::MultipartError(e.to_string()))?;

    let id = Uuid::new_v4().to_string();

    let file_path = save_file(&format!("{}.{}", id, extension), &file)?;

    let sub_command = format!(
        "{}/run_align_upload.sh {} {} {}",
        SCRIPT_PREFIX, id, asr_kind, file_path
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    let result_path = get_command_output(&mut command).await?;
    let result_path = Path::new(&result_path);

    let result = read(result_path).map_err(|e| HttpError::NotFound(e.to_string()))?;

    remove_file(file_path)?;
    remove_dir_all(result_path.parent().unwrap().parent().unwrap())?;

    Ok(result)
}

pub async fn app_upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 1024 /* 250mb */
        },
    >,
) -> Result<Vec<u8>> {
    let MultipartContent {
        extension, file, ..
    } = parse_multipart(&mut multipart)
        .await
        .map_err(|e| HttpError::MultipartError(e.to_string()))?;

    let id = Uuid::new_v4().to_string();

    let file_path = save_file(&format!("{}.{}", id, extension), &file)?;

    let sub_command = format!(
        "{}/run_app_upload.sh sa_me_2.0+vgh {}",
        SCRIPT_PREFIX, file_path
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    let result_path = get_command_output(&mut command).await?;
    let result_path = Path::new(&result_path);

    let result = read(result_path).map_err(|e| HttpError::NotFound(e.to_string()))?;

    remove_file(file_path)?;
    remove_dir_all(result_path.parent().unwrap().parent().unwrap())?;

    Ok(result)
}

pub async fn rec_youtube(form: Form<RecYoutubeRequest>) -> Result<Vec<u8>> {
    let request: RecYoutubeRequest = form.0;

    let id = Uuid::new_v4().to_string();

    let sub_command = format!(
        "{}/run_rec_youtube.sh {} {} {} {} {}",
        SCRIPT_PREFIX,
        id,
        request.asr_kind.unwrap_or("sa_me_2.0".to_string()),
        request.output_type.unwrap_or("srt".to_string()),
        request.vid,
        "10",
    );

    debug!("Run command: {}", sub_command);

    let mut command = Command::new("bash");
    command.arg("-c").arg(sub_command);

    let result_path = get_command_output(&mut command).await?;
    let result_path = Path::new(result_path.split(" ").next().unwrap());

    let result = read(result_path).map_err(|e| HttpError::NotFound(e.to_string()))?;

    if result_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .ends_with("decode")
    {
        fs::remove_dir_all(
            result_path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )?;
    } else {
        fs::remove_dir_all(result_path.parent().unwrap().parent().unwrap())?;
    }

    Ok(result)
}
