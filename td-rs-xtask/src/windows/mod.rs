use crate::metadata::PluginType;
use crate::{build, PLUGIN_HOME};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Read;
use fs_extra::dir::CopyOptions;
use anyhow::Context;

pub(crate) fn build_plugin(plugin: &str, plugin_type: PluginType) -> anyhow::Result<()> {
    let target = "x86_64-pc-windows-msvc";
    build(
        &[plugin, plugin_type.to_plugin_name()],
        &["--release", &format!("--target={target}")],
    )?;

    let solution_name = format!("Rust{}", plugin_type.to_short_name().to_uppercase());
    let files = [
        format!("{solution_name}.sln"),
        format!("{solution_name}.vcxproj"),
        format!("{solution_name}.vcxproj.user")
    ];


    println!("Write solution");
    let to_copy: Vec<String> = files
        .iter()
        .map(|x| format!("./td-rs-xtask/msvc/{}/{}", plugin_type.to_short_name(), x))
        .collect();
    fs_extra::copy_items(&to_copy, ".", &CopyOptions::new().overwrite(true))?;
    run_msbuild(&target, &plugin)?;
    fs_extra::remove_items(&files)?;

    // move_plugin(&plugin)?;

    Ok(())
}

fn move_plugin(plugin: &str) -> anyhow::Result<()> {
    let plugin_build_path = format!("./Release/{plugin}.plugin");
    let plugin_target_path = Path::new(PLUGIN_HOME).join(plugin);
    std::fs::create_dir_all(&plugin_target_path).context("Could not create plugin directory")?;
    fs_extra::dir::remove(&plugin_target_path.join(format!("{plugin}.plugin")))
        .context("Could not remove plugin directory")?;
    fs_extra::dir::move_dir(&plugin_build_path, &plugin_target_path, &CopyOptions::new())
        .context("Could not move plugin to target directory")?;
    Ok(())
}

fn run_msbuild(target: &str, plugin: &str) -> anyhow::Result<()> {
    let msbuild = find_msbuild()?;
    let msbuild = msbuild.to_str().expect("Could not find msbuild");
    let lib = format!("{}.lib", plugin.replace("-", "_"));
    let cmd = format!("&'{msbuild}' /p:Configuration=Release /p:Platform=x64 /p:Plugin=.\\target\\{target}\\release\\{lib}");
    println!("Running {cmd}");
    let mut cmd = Command::new("powershell.exe")
        .arg(cmd)
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

fn write_solution(target: &str, plugin: &str, plugin_type: &PluginType, path: &PathBuf) -> anyhow::Result<()> {

    Ok(())
}