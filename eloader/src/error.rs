use abi_stable::library::LibraryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Library(#[from] LibraryError),
}
