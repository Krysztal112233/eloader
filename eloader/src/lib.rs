pub mod container;
pub mod error;

use abi_stable::{library::RootModule, StableAbi};
use container::ExtensionContainer;
use error::Error;
use std::marker::PhantomData;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct ExtensionLoader<T> {
    walker: WalkDir,
    _mark: PhantomData<T>,
}

impl<T> ExtensionLoader<T>
where
    T: StableAbi,
{
    pub fn new(walker: WalkDir) -> Self {
        Self {
            walker,
            _mark: PhantomData,
        }
    }
}

impl<T> IntoIterator for ExtensionLoader<T>
where
    T: RootModule,
{
    type Item = Result<ExtensionContainer<T>, Error>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.walker
            .into_iter()
            .flatten()
            .map(DirEntry::into_path)
            .filter(|it| it.is_file())
            .map(ExtensionContainer::try_from)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
