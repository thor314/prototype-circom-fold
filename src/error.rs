//! prototype-circom-fold error types
// https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    // Some other error type
    #[allow(dead_code)]
    #[error("an unhandled error")]
    Unhandled,
}
