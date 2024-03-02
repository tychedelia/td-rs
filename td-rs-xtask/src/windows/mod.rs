use std::fmt::format;
use crate::config::Config;
use crate::metadata::PluginType;
use crate::{build, PLUGIN_HOME};
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::util::ToTitleCase;

pub(crate) fn install_plugin(
    config: &Config,
    plugin: &str,
    plugin_type: PluginType,
) -> anyhow::Result<()> {
    let plugin_target_path = plugin_target_path(plugin);
    let td_plugin_folder = &config.windows.plugin_folder;
    println!(
        "Installing plugin {:?} to {}",
        plugin_target_path, td_plugin_folder
    );
    fs_extra::copy_items(
        &[&plugin_target_path],
        td_plugin_folder,
        &CopyOptions::new().overwrite(true),
    )?;
    Ok(())
}

pub(crate) fn build_plugin(
    config: &Config,
    plugin: &str,
    plugin_type: PluginType,
) -> anyhow::Result<()> {
    let target = "x86_64-pc-windows-msvc";
    build(
        &[plugin, plugin_type.to_plugin_name()],
        &["--release", &format!("--target={target}")],
    )?;

    let solution_name = "RustOp";
    let files = [
        format!("{solution_name}.sln"),
        format!("{solution_name}.vcxproj"),
        format!("{solution_name}.vcxproj.user"),
    ];

    println!("Write solution");
    let to_copy: Vec<String> = files
        .iter()
        .map(|x| format!("./td-rs-xtask/msvc/{x}"))
        .collect();

    println!("Run msbuild");
    fs_extra::copy_items(&to_copy, ".", &CopyOptions::new().overwrite(true))?;

    let short_title = plugin_type.to_short_name().to_title_case();
    let short_upper = plugin_type.to_short_name().to_uppercase();
    let op_path = plugin_type.to_plugin_name();
    let vcxproj = std::fs::read_to_string(format!("./{solution_name}.vcxproj"))?
        .replace(
            "{{ OP_LIB_NAME }}",
            &format!("{}.lib", op_path.replace("-", "_")),
        )
        .replace(
            "{{ TD_OP_H_PATH }}",
            &format!("{}/src/{}_CPlusPlusBase.h", op_path, short_upper),
        )
        .replace(
            "{{ PLUGIN_H_PATH }}",
            &format!("{}/src/Rust{}Plugin.h", op_path, short_title),
        )
        .replace(
            "{{ PLUGIN_CPP_PATH }}",
            &format!("{}/src/Rust{}Plugin.cpp", op_path, short_title),
        );
    std::fs::write(format!("./{solution_name}.vcxproj"), vcxproj)?;

    let is_python_enabled = crate::metadata::is_feature_enabled(plugin, "python", &plugin_type);
    let is_cuda_enabled = crate::metadata::is_feature_enabled(plugin, "cuda", &plugin_type);
    run_msbuild(config, &target, &plugin, is_python_enabled, is_cuda_enabled)?;
    fs_extra::remove_items(&files)?;

    println!("Move plugin to target");
    move_plugin(&plugin, &plugin_type)?;

    Ok(())
}

fn move_plugin(plugin: &str, plugin_type: &PluginType) -> anyhow::Result<()> {
    let dll_name = match plugin_type {
        PluginType::Chop => "RustCHOP",
        PluginType::Sop => "RustSOP",
        PluginType::Dat => "RustDAT",
        PluginType::Top => "RustTOP",
    };

    let plugin_build_path = format!("./Release/{dll_name}.dll");
    let plugin_target_path = plugin_target_path(plugin);
    std::fs::create_dir_all(&plugin_target_path.parent().unwrap())
        .context("Could not create plugin directory")?;
    std::fs::copy(&plugin_build_path, &plugin_target_path)
        .context("Could not move plugin to target directory")?;
    Ok(())
}

fn plugin_target_path(plugin: &str) -> PathBuf {
    let plugin_target_path = Path::new(PLUGIN_HOME)
        .join(plugin)
        .join(format!("{plugin}.dll"));
    plugin_target_path
}

fn run_msbuild(
    config: &Config,
    target: &str,
    plugin: &str,
    is_python_enabled: bool,
    is_cuda_enabled: bool,
) -> anyhow::Result<()> {
    let msbuild = find_msbuild()?;
    println!("Found msbuild at {:?}", msbuild);
    let msbuild = msbuild.to_str().expect("Could not find msbuild");
    let lib = format!("{}.lib", plugin.replace("-", "_"));

    let mut additional_include_dirs = vec![];
    let mut additional_library_dirs = vec![];
    let mut additional_dependencies = vec![];

    if is_python_enabled {
        additional_include_dirs.push(config.windows.python_include_dir.as_str());
        additional_library_dirs.push(config.windows.python_lib_dir.as_str());
        additional_dependencies.push(config.windows.python_libs.as_str());
    }
    if is_cuda_enabled {
        additional_library_dirs.push(config.windows.cuda_lib_dir.as_str());
        additional_dependencies.push(config.windows.cuda_libs.as_str());
    }

    let additional_include_dirs = additional_include_dirs.join("\";\"");
    let additional_library_dirs = additional_library_dirs.join("\";\"");
    let additional_dependencies = additional_dependencies.join(";");

    println!("is_python_enabled: {}", is_python_enabled);
    println!("is_cuda_enabled: {}", is_cuda_enabled);

    let mut cmd = Command::new("powershell.exe")
        .arg(format!("&'{msbuild}'"))
        .arg("--%")
        .arg(format!("/p:AdditionalIncludeDirectories={additional_include_dirs}"))
        .arg(format!("/p:AdditionalLibraryDirectories={additional_library_dirs}"))
        .arg(format!("/p:AdditionalDependencies={additional_dependencies}"))
        .arg(if is_python_enabled {
            "/p:PreprocessorDefinitions=PYTHON_ENABLED"
        } else {
            ""
        })
        .arg("/p:Configuration=Release")
        .arg("/t:Rebuild")
        .arg("/p:Platform=x64")
        .arg(format!("/p:Plugin=.\\target\\{target}\\release\\{lib}"))
        .spawn()?;

    let status = cmd.wait()?;
    if !status.success() {
        anyhow::bail!("Couldn't run msbuild");
    }
    Ok(())
}

fn find_msbuild() -> anyhow::Result<PathBuf> {
    let cmd = r#"&"${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -prerelease -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe"#;
    let mut cmd = Command::new("powershell.exe")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()?;
    let status = cmd.wait()?;
    if !status.success() {
        anyhow::bail!("Could not find msbuild");
    } else {
        let mut stdout = cmd.stdout.take().expect("Couldn't read stdout");
        let mut path = String::new();
        stdout.read_to_string(&mut path)?;
        let path = PathBuf::from(&path.trim_end());

        if !path.exists() {
            anyhow::bail!("Unknown msbuild path {:?}", path);
        }

        Ok(path)
    }
}
