#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use eloader::ExtensionLoader;
use extension::ExtensionRef;
use log::{debug, info};
use std::{path::PathBuf, str::FromStr};
use walkdir::WalkDir;

mod context;
mod error;
#[allow(unused)]
mod extension;

fn main() {
    env_logger::init();

    let f_loader = |path: &str| {
        let extension_directory = PathBuf::from_str(path).unwrap().canonicalize().unwrap();

        info!(
            "Try loading extensions from directory: {}",
            extension_directory.to_str().unwrap()
        );
        ExtensionLoader::<ExtensionRef>::new(WalkDir::new(extension_directory))
    };

    let loader = f_loader("./target/debug").into_iter();

    let mut containers = Vec::new();
    for loaded in loader.into_iter() {
        match loaded {
            Err(err) => debug!("{err}"),
            Ok(container) => {
                info!("Loaded extension: {}", container.path().to_str().unwrap());
                containers.push(container);
            }
        }
    }
}
