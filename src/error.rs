

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request Error")]
    RequestError(#[from] reqwest::Error),

    #[error("Auth Error")]
    AuthError,

    #[error("URL Error")]
    InvalidURL,

    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("General Error")]
    GeneralError,

    #[error("Internal Error")]
    InternalError,

    #[error("Critical Error")]
    CriticalError,
}