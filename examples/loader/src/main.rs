#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use abi_stable::std_types::RResult::RErr;
use context::ExtensionContext;
use eloader::ExtensionLoader;
use extension::ExtensionRef;
use log::{debug, error, info};
use std::{path::PathBuf, str::FromStr};
use walkdir::WalkDir;

mod context;
mod error;
#[allow(unused)]
mod extension;

fn main() {
    env_logger::init();

    let loader = |path: &str| {
        let extension_directory = PathBuf::from_str(path).unwrap().canonicalize().unwrap();

        info!(
            "Try loading extensions from directory: {}",
            extension_directory.to_str().unwrap()
        );
        ExtensionLoader::<ExtensionRef>::new(WalkDir::new(extension_directory).max_depth(1))
    };

    let loader = loader("./target/debug").into_iter();

    let mut containers = Vec::new();
    for loaded in loader.into_iter() {
        match loaded {
            Err(err) => debug!("{err}"),
            Ok(container) => {
                info!(
                    "Loaded extension into memory: {}",
                    container.path().to_str().unwrap()
                );
                containers.push(container);
            }
        }
    }

    for extension in containers {
        let meta = match extension.module().meta() {
            Some(invoker) => invoker(),
            _ => {
                info!(
                    "Cannot find invoker `meta()` in extension {}",
                    extension.path().to_string_lossy()
                );
                continue;
            }
        };

        match extension.module().load() {
            Some(invoker) => {
                info!(
                    "Loaded extension {} in {}",
                    meta.name,
                    extension.path().to_string_lossy()
                );
                if let RErr(err) = invoker(&mut ExtensionContext {}) {
                    error!("While loading extension {}: {}", meta.name, err);
                    error!("Extension {} will dropped.", meta.name);

                    if let Some(invoker) = extension.module().drop() {
                        invoker()
                    }
                }
            }
            _ => {
                error!(
                    "Cannot find invoker `load()` in extension {}",
                    extension.path().to_string_lossy()
                );
                continue;
            }
        };
    }
}
