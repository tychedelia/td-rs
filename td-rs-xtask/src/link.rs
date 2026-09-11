use anyhow::Context;
use std::process::{Command, Stdio};

pub fn native_static_libs(plugin: &str, target: &str) -> anyhow::Result<Vec<String>> {
    let out = Command::new("cargo")
        .arg("rustc")
        .arg("--color")
        .arg("never")
        .args(crate::cargo_workspace_args())
        .args(["-p", plugin, "--release", &format!("--target={target}"), "--", "--print=native-static-libs"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output()
        .context("could not run cargo rustc to list native static libs")?;
    let stderr = strip_ansi(&String::from_utf8_lossy(&out.stderr));
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
fn is_python_lib(tok: &str) -> bool {
    let t = tok.to_ascii_lowercase();
    t.starts_with("-lpython") || (t.starts_with("python") && t.ends_with(".lib"))
}

pub fn ldflags(libs: &[String]) -> String {
    libs.iter().filter(|l| !is_python_lib(l)).cloned().collect::<Vec<_>>().join(" ")
}

pub fn msbuild_libs(libs: &[String]) -> String {
    libs.iter()
        .filter(|l| l.to_ascii_lowercase().ends_with(".lib") && !is_python_lib(l))
        .cloned()
        .collect::<Vec<_>>()
        .join(";")
}
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            while let Some(&n) = chars.peek() {
                chars.next();
                if ('\x40'..='\x7e').contains(&n) {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pyo3s_interpreter_is_not_the_plugins() {
        let libs: Vec<String> = ["-lc++", "-lpython3.14", "-framework Python", "-lm"].iter().map(|s| s.to_string()).collect();
        assert_eq!(ldflags(&libs), "-lc++ -framework Python -lm");
        let libs: Vec<String> = ["kernel32.lib", "python314.lib", "python3.lib", "/defaultlib:msvcrt"].iter().map(|s| s.to_string()).collect();
        assert_eq!(msbuild_libs(&libs), "kernel32.lib");
    }

    #[test]
    fn coloured_notes_parse_to_clean_tokens() {
        let s = "\x1b[1m\x1b[36mnote\x1b[0m: native-static-libs: -lc++ -framework CoreAudio -lm\x1b[0m\n";
        assert_eq!(strip_ansi(s), "note: native-static-libs: -lc++ -framework CoreAudio -lm\n");
    }
}
