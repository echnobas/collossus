use reqwest::Error;
#[derive(Debug)]
pub enum RbxError {
    UnexpectedStatusCode(u16),
    HttpError(reqwest::Error),
    Unauthorized(&'static str),
}

impl From<reqwest::Error> for RbxError {
    fn from(e: Error) -> Self {
        Self::HttpError(e)
    }
}
