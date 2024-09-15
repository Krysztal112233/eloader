use abi_stable::library::RootModule;
use std::path::{Path, PathBuf};

use crate::error::Error;

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

impl<M> TryFrom<&Path> for ExtensionContainer<M>
where
    M: RootModule,
{
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let module = M::load_from_file(value)?;

        Ok(ExtensionContainer {
            path: value.to_path_buf(),
            module,
        })
    }
}

impl<M> TryFrom<PathBuf> for ExtensionContainer<M>
where
    M: RootModule,
{
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}
