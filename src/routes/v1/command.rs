use std::{process::Command, str};

use anyhow::Result;
use tracing::debug;

pub const SCRIPT_PREFIX: &str = "source ~/miniconda3/etc/profile.d/conda.sh;conda activate pkasr;bash /mnt/md0/nfs_share/PKASR/sinica_asr/api";

pub async fn get_command_output(command: &mut Command) -> Result<String> {
    let output = command.output()?;

    let result = str::from_utf8(&output.stdout)?;
    let result = result.trim();
    let error = str::from_utf8(&output.stderr)?;

    debug!("child status was: {}", output.status);
    debug!("child output was: {}", result);
    debug!("child error was: {}", error);

    Ok(result.to_string())
}
