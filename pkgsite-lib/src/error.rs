use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackagesSiteError {
    #[cfg(feature = "reqwest")]
    #[error("Failed to fetch from packages site: {0}")]
    FetchError(#[from] reqwest::Error),
    #[cfg(feature = "reqwest")]
    #[error("Unexpected response status code: {0}")]
    UnexpectedStatus(reqwest::StatusCode),
    #[cfg(feature = "nyquest")]
    #[error("Failed to fetch from packages site: {0}")]
    FetchError(#[from] nyquest::Error),
    #[cfg(feature = "nyquest")]
    #[error("Unexpected response status code: {0}")]
    UnexpectedStatus(u16),
    #[cfg(feature = "nyquest")]
    #[error("Failed to build client: {0}")]
    BuildClientError(#[from] nyquest::client::BuildClientError),
}

pub type PResult<T> = Result<T, PackagesSiteError>;
