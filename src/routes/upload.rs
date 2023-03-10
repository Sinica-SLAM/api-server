use anyhow::{anyhow, Result};
use axum::extract::Multipart;
use bytes::Bytes;
use std::{fmt, fs::File, io::Write, str::FromStr};

const UPLOAD_PATH: &str = "/mnt/md0/api-server/.temp";

#[derive(Debug)]
pub enum OutputType {
    Srt,
    Json,
    Trs,
}

impl FromStr for OutputType {
    type Err = ();

    fn from_str(input: &str) -> Result<OutputType, ()> {
        match input {
            "srt" => Ok(OutputType::Srt),
            "json" => Ok(OutputType::Json),
            "trs" => Ok(OutputType::Trs),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputType::Srt => write!(f, "srt"),
            OutputType::Json => write!(f, "json"),
            OutputType::Trs => write!(f, "trs"),
        }
    }
}

pub struct MultipartContent {
    pub asr_kind: String,
    pub file: Bytes,
    pub extension: String,
    pub output_type: OutputType,
    pub lmwt: i64,
    pub min_word_gap: f64,
}

impl Default for MultipartContent {
    fn default() -> Self {
        Self {
            asr_kind: String::from("sa_me_2.0"),
            file: Bytes::new(),
            extension: String::new(),
            output_type: OutputType::Srt,
            lmwt: 10,
            min_word_gap: 0.3,
        }
    }
}

pub async fn parse_multipart(multipart: &mut Multipart) -> Result<MultipartContent> {
    let mut content = MultipartContent::default();

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("source") => {
                let source = std::path::Path::new(field.file_name().unwrap());
                content.extension = source.extension().unwrap().to_str().unwrap().to_owned();
                content.file = field.bytes().await?;
            }
            Some("asr_kind") => content.asr_kind = field.text().await?,
            Some("output_type") => {
                content.output_type = field
                    .text()
                    .await?
                    .parse()
                    .map_err(|_| anyhow!("Invalid output type"))?;
            }
            Some("lmwt") => {
                content.lmwt = field
                    .text()
                    .await?
                    .parse()
                    .map_err(|_| anyhow!("Invalid lmwt"))?
            }
            Some("min_word_gap") => {
                content.min_word_gap = field
                    .text()
                    .await?
                    .parse()
                    .map_err(|_| anyhow!("Invalid min_word_gap"))?
            }
            unknown => {
                return Err(anyhow!(
                    "invalid parameter {}",
                    unknown.unwrap_or("not str")
                ))
            }
        }
    }

    Ok(content)
}

pub fn save_file(filename: &String, bytes: &Bytes) -> Result<String> {
    let file_path = format!("{}/{}", UPLOAD_PATH, filename);
    let mut file = File::create(&file_path)?;
    file.write_all(bytes)?;

    Ok(file_path)
}
