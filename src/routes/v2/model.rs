use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct RecYoutubeRequest {
    pub asr_kind: Option<String>,
    pub output_type: Option<String>,
    pub vid: String,
    pub lmwt: Option<String>,
    pub n_jobs: Option<String> // none docker version don't have this option
}
