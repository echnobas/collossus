#[derive(Debug)]
pub enum RbxError {
    UnexpectedStatusCode(u16),
    HttpError(reqwest::Error),
    SerdeError(serde_json::Error),
    Unauthorized(&'static str),
    FieldMissing(&'static str),
}

impl From<reqwest::Error> for RbxError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<serde_json::Error> for RbxError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}
