use anyhow::{anyhow};
use entity::results;
use std::{fs, path::Path, str};

use sea_orm::DatabaseConnection;
use tokio::process::Command;
use tracing::{debug, info_span, Instrument};

use crate::db::{set_result_file_path, set_result_status};

pub const SCRIPT_PREFIX: &str = "source ~/miniconda3/etc/profile.d/conda.sh;conda activate pkasr;bash /mnt/md0/nfs_share/PKASR/sinica_asr/api";
pub const RESULT_PATH: &str = "/mnt/md0/api-server/.result";

pub fn get_command_output(
    mut command: Command,
    id: &String,
    delete_file_path: Option<String>,
    conn: &DatabaseConnection,
    result_model: results::Model,
) {
    let id = id.clone();
    let conn = conn.clone();

    tokio::spawn(
        async move {
            match async {
                let output = command.output().await?;

                let result = str::from_utf8(&output.stdout)?;
                let result = result.trim();
                let error = str::from_utf8(&output.stderr)?;

                debug!("child status was: {}", output.status);
                if !output.status.success() {
                    debug!("child error was: {}", error);

                    return Err(anyhow!("run command error"));
                }

                debug!("child output was: {}", result);

                let result_path = Path::new(result.split(" ").next().unwrap());
                let save_path = format!("{}/{}", RESULT_PATH, id);

                fs::copy(result_path, save_path.clone())?;
                // if result_path
                //     .parent()
                //     .unwrap()
                //     .parent()
                //     .unwrap()
                //     .ends_with("/decode")
                // {
                //     fs::remove_dir_all(
                //         result_path
                //             .parent()
                //             .unwrap()
                //             .parent()
                //             .unwrap()
                //             .parent()
                //             .unwrap(),
                //     )?;
                // } else {
                //     fs::remove_dir_all(result_path.parent().unwrap().parent().unwrap())
                //         .expect("remove result dir error");
                // }

                if let Some(delete_file_path) = delete_file_path {
                    fs::remove_file(delete_file_path)?;
                }

                set_result_status(&conn, results::Status::Complete, result_model.clone()).await?;

                set_result_file_path(&conn, save_path, result_model.clone()).await?;
                Ok(())
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
        .instrument(info_span!("get_command_output async")),
    );
}
