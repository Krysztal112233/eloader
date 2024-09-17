#![doc = include_str!("../../README.md")]

use crate::error::Error;
use abi_stable::library::RootModule;
use log::debug;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ExtensionContainer<T> {
    path: PathBuf,
    module: T,
}

impl<T> ExtensionContainer<T>
where
    T: RootModule,
{
    /// Module path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loaded trait object implemented [abi_stable::StableAbi]
    pub fn module(&self) -> &T {
        &self.module
    }
}

impl<T> TryFrom<&Path> for ExtensionContainer<T>
where
    T: RootModule,
{
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        debug!("Try to load file {}", value.to_str().unwrap());
        let module = T::load_from_file(value)?;

        Ok(ExtensionContainer {
            path: value.to_path_buf(),
            module,
        })
    }
}

impl<T> TryFrom<PathBuf> for ExtensionContainer<T>
where
    T: RootModule,
{
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}
