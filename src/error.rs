use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to acquire semaphore")]
    SemaphoreError,

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, HashError>;
