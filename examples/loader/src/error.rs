use abi_stable::StableAbi;
use thiserror::Error;

#[repr(C)]
#[derive(Debug, Error, StableAbi)]
pub enum Error {
    #[error("unknow error")]
    Unknown,
}
