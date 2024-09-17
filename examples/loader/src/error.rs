use abi_stable::StableAbi;
use thiserror::Error;

#[allow(unused)]
#[repr(C)]
#[derive(Debug, Error, StableAbi)]
pub enum Error {
    #[error("unknow error")]
    Unknown,
}
