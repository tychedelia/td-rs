use std::fs::File;
use std::io::Read;
use std::path::Path;
use cargo_metadata::{Metadata, MetadataCommand};

pub enum PluginType {
    Chop,
    Sop,
    Dat,
    Top,
}

pub fn plugin_type(plugin: &str) -> PluginType {
    let package_name = plugin;
    let metadata = fetch_cargo_metadata();
    let package = metadata
        .packages
        .into_iter()
        .find(|package| package.name == package_name);

    if let Some(package) = package {
        let plugin_type = package
            .metadata
            .get("td-rs")
            .expect("Didn't find td-rs metadata in Cargo.toml. Please add [package.metadata.td-rs] to your cargo manifest to specify the type of plugin.")
            .get("type")
            .expect("Could not find type in td-rs metadata. Please add [package.metadata.td-rs.type] to your cargo manifest to specify the type of plugin.")
            .as_str()
            .expect("Could not parse type in td-rs metadata. Please add [package.metadata.td-rs.type] as a string to your cargo manifest to specify the type of plugin.");
        match plugin_type {
            "chop" => PluginType::Chop,
            "sop" => PluginType::Sop,
            "dat" => PluginType::Dat,
            "top" => PluginType::Top,
            _ => panic!("Unknown plugin type: {}", plugin_type),
        }
    } else {
        panic!("Package not found: {}", package_name);
    }
}

fn fetch_cargo_metadata() -> Metadata {
    MetadataCommand::new()
        .exec()
        .expect("Failed to fetch cargo metadata")
}

fn fetch_cargo_metadata_for_package(package: &str) -> Metadata {
    MetadataCommand::new()
        .manifest_path(package)
        .no_deps()
        .exec()
        .expect("Failed to fetch cargo metadata")
}
