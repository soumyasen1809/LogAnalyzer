use crate::log_line::LogLine;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("IO Error: {0:#?}")]
    IO(#[from] std::io::Error),

    #[error("Tokio Error: {0:#?}")]
    TokioSendError(#[from] SendError<LogLine>),
}
