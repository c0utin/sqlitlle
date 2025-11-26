use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlittleError {
    #[error("Internal error: {0}")]
    InternalError(String),
}
