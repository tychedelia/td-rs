use anyhow::Context;
use std::process::{Command, Stdio};

pub fn native_static_libs(plugin: &str, target: &str) -> anyhow::Result<Vec<String>> {
    let out = Command::new("cargo")
        .args(["rustc", "-p", plugin, "--release", &format!("--target={target}"), "--", "--print=native-static-libs"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output()
        .context("could not run cargo rustc to list native static libs")?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    let mut libs: Vec<String> = Vec::new();
    for line in stderr.lines() {
        if let Some(rest) = line.split("native-static-libs:").nth(1) {
            let mut it = rest.split_whitespace().peekable();
            while let Some(tok) = it.next() {
                let tok = if tok == "-framework" {
                    match it.next() {
                        Some(name) => format!("-framework {name}"),
                        None => break,
                    }
                } else {
                    tok.to_string()
                };
                if !libs.contains(&tok) {
                    libs.push(tok);
                }
            }
        }
    }
    if !out.status.success() {
        anyhow::bail!("cargo rustc failed while listing native static libs");
    }
    println!("native static libs for {plugin}: {}", libs.join(" "));
    Ok(libs)
}

pub fn ldflags(libs: &[String]) -> String {
    libs.join(" ")
}

pub fn msbuild_libs(libs: &[String]) -> String {
    libs.iter()
        .filter(|l| l.to_ascii_lowercase().ends_with(".lib"))
        .cloned()
        .collect::<Vec<_>>()
        .join(";")
}
