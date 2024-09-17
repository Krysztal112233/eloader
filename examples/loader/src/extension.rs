use crate::{context::ExtensionContext, error::Error};
use abi_stable::{
    library::RootModule,
    sabi_types::VersionStrings,
    std_types::{RResult, RString},
    StableAbi,
};

#[repr(C)]
#[derive(Debug, StableAbi)]
pub struct ExtensionMeta {
    pub author: RString,
    pub name: RString,
}

/// Define basic layout of extension

#[derive(Debug, StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ExtensionRef, prefix_fields = ExtensionPrefix)))]
#[repr(C)]
pub struct Extension {
    pub meta: extern "C" fn() -> ExtensionMeta,
    pub load: extern "C" fn(&mut ExtensionContext) -> RResult<(), Error>,
    pub drop: extern "C" fn(),
}

impl RootModule for ExtensionRef {
    const BASE_NAME: &'static str = "example";
    const NAME: &'static str = "dynamic_extension";
    const VERSION_STRINGS: VersionStrings = abi_stable::package_version_strings!();

    abi_stable::declare_root_module_statics! {ExtensionRef}
}
