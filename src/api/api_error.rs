use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub(crate) enum ApiError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),

    #[error(transparent)]
    IOError(#[from] IoError),

    #[error("Invalid file name")]
    InvalidFileName,

    #[error(transparent)]
    SerdeError(#[from] SerdeError),

    #[error(transparent)]
    TokioJoinError(#[from] JoinError),

    #[error("Invalid file hash")]
    InvalidFileHash,

    #[error("No download URL found")]
    NoDownloadUrl,

    #[error("Mod version not found")]
    ModVersionNotFound,
}
