use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackagesSiteError {
    #[error("Failed to fetch from packages site: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("Unexpected response status code: {0}")]
    UnexpectedStatus(reqwest::StatusCode),
}

pub type PResult<T> = Result<T, PackagesSiteError>;
