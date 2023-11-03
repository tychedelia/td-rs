use cargo_metadata::{Metadata, MetadataCommand};
use std::path::Path;

pub enum PluginType {
    Chop,
    Sop,
    Dat,
    Top,
}

impl PluginType {
    pub(crate) fn to_plugin_name(&self) -> &str {
        match self {
            Self::Chop => "td-rs-chop",
            Self::Sop => "td-rs-sop",
            Self::Dat => "td-rs-dat",
            Self::Top => "td-rs-top",
        }
    }

    pub(crate) fn to_short_name(&self) -> &str {
        match self {
            Self::Chop => "chop",
            Self::Sop => "sop",
            Self::Dat => "dat",
            Self::Top => "top",
        }
    }
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

#[allow(dead_code)]
fn fetch_cargo_metadata_for_package(package: &str) -> Metadata {
    MetadataCommand::new()
        .manifest_path(package)
        .no_deps()
        .exec()
        .expect("Failed to fetch cargo metadata")
}

#[cfg(not(target_os = "windows"))]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[cfg(target_os = "windows")]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}

pub(crate) fn list_plugins() -> anyhow::Result<Vec<String>> {
    let meta = fetch_cargo_metadata();
    let plugin_dir = adjust_canonicalization(
        Path::new("./plugins")
            .canonicalize()
            .expect("Could not canonicalize plugin dir"),
    );
    println!("Plugin dir: {:?}\n", plugin_dir);
    let ws_members = meta
        .workspace_packages()
        .iter()
        .filter(|package| package.manifest_path.starts_with(&plugin_dir))
        .map(|package| package.name.clone())
        .collect::<Vec<String>>();
    Ok(ws_members)
}
