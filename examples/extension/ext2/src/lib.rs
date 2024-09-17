#![allow(non_upper_case_globals, non_camel_case_types)]

use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    std_types::RResult::{self, RErr},
};
use loader::{
    context::ExtensionContext,
    error::Error,
    extension::{Extension, ExtensionMeta, ExtensionRef},
};

#[export_root_module]
fn initializer() -> ExtensionRef {
    let ext = Extension { meta, load, drop };
    ext.leak_into_prefix()
}

#[no_mangle]
extern "C" fn meta() -> ExtensionMeta {
    ExtensionMeta {
        author: "KrysztalHuang".into(),
        name: "ExampleExtension2".into(),
    }
}

#[no_mangle]
extern "C" fn load(_ctx: &mut ExtensionContext) -> RResult<(), Error> {
    RErr(Error::Unknown)
}

#[no_mangle]
extern "C" fn drop() {}
