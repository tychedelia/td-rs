#[cfg(target_os = "windows")]
#[derive(serde::Deserialize, Debug)]
pub struct WindowsConfig {
    pub(crate) plugin_folder: String,
    pub(crate) python_include_dir: String,
    pub(crate) python_lib_dir: String,
}

#[cfg(target_os = "macos")]
#[derive(serde::Deserialize, Debug)]
pub struct MacOsConfig {
    pub(crate) python_include_dir: String,
    pub(crate) plugin_folder: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    #[cfg(target_os = "windows")]
    pub(crate) windows: WindowsConfig,
    #[cfg(target_os = "macos")]
    pub(crate) macos: MacOsConfig,
}

pub(crate) fn read_config() -> Config {
    let config_path = std::path::Path::new("td-rs.toml");
    let config_file = std::fs::read_to_string(config_path).expect("Could not read td-rs.toml");
    let config = toml::from_str(&config_file).expect("Could not parse td-rs.toml");
    process_config(config)
}


fn process_config(mut config: Config) -> Config {
    // special handling for $HOME in path
    #[cfg(target_os = "windows")]
    if config.windows.plugin_folder.contains("$HOME") {
        let home_dir = homedir::get_my_home().expect("Could not get home directory").expect("Could not get home directory");
        let plugin_folder = config.windows.plugin_folder.replace("$HOME", home_dir.to_str().expect("Could not convert home directory to string"));
        config.windows.plugin_folder = plugin_folder;
    }
    #[cfg(target_os = "macos")]
    if config.macos.plugin_folder.contains("$HOME") {
        let home_dir = homedir::get_my_home().expect("Could not get home directory").expect("Could not get home directory");
        let plugin_folder = config.macos.plugin_folder.replace("$HOME", home_dir.to_str().expect("Could not convert home directory to string"));
        config.macos.plugin_folder = plugin_folder;
    }

    config
}