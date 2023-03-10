use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RecYoutubeRequest {
    pub asr_kind: Option<String>,
    pub output_type: Option<String>,
    pub vid: String,
}
