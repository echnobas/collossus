use reqwest::Error;

#[derive(Debug)]
pub enum RbxError<'a> {
    UnexpectedStatusCode(u16),
    HttpError(reqwest::Error),
    Unauthorized(&'a str),
}

impl From<reqwest::Error> for RbxError<'_> {
    fn from(e: Error) -> Self {
        Self::HttpError(e)
    }
}
