use std::io;

#[derive(thiserror::Error, Debug)]
pub enum ChainError {
    #[error("Cannot save to {path}: {source}")]
    SavingError {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Cannot load chain from {path}: {source}")]
    LoadingError {
        path: String,
        #[source]
        source: io::Error,
    },
}
