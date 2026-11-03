use colored::Colorize;
use indexmap::IndexMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

fn settings_dir() -> PathBuf {
    let home = env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".config").join("lazy-up")
}

pub fn check_settings() -> bool {
    settings_dir().join("pkgs.yml").exists()
}

pub fn create_settings_file() -> std::io::Result<()> {
    create_settings_file_in(&settings_dir())
}

fn create_settings_file_in(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(dir.join("pkgs.yml"), "rust: rustup update\n")
}

/// Only macOS and Linux are supported.
pub fn is_supported_os(os: &str) -> bool {
    matches!(os, "macos" | "linux")
}

pub fn read_settings() -> Result<IndexMap<String, String>, Box<dyn Error>> {
    read_settings_from(&settings_dir().join("pkgs.yml"))
}

fn read_settings_from(path: &Path) -> Result<IndexMap<String, String>, Box<dyn Error>> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("could not read {}: {e}", path.display()))?;
    serde_yaml_ng::from_str(&content)
        .map_err(|e| format!("invalid config {}: {e}", path.display()).into())
}

/// Runs the package's update command; a spawn failure or non-zero exit is an error.
pub fn update_pkg(pkg_name: &str, command: &str) -> std::io::Result<()> {
    println!(
        "{}{}{}",
        "\n*** Updating: ".green(),
        pkg_name.blue(),
        " ***".green()
    );
    let status = run_command(command)
        .map_err(|e| std::io::Error::new(e.kind(), format!("'{command}' could not run: {e}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "'{command}' failed ({status})"
        )))
    }
}

fn run_command(command: &str) -> std::io::Result<ExitStatus> {
    let mut parts = command.split_whitespace();
    let program = parts
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty command"))?;
    Command::new(program)
        .args(parts)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("lazy-up-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn creates_default_settings_file() {
        let dir = temp_dir("create");
        create_settings_file_in(&dir.join("nested")).unwrap();
        let content = fs::read_to_string(dir.join("nested/pkgs.yml")).unwrap();
        assert_eq!(content, "rust: rustup update\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reads_settings_in_file_order() {
        let dir = temp_dir("read");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "zeta: echo z\nalpha: echo a\n").unwrap();

        let settings = read_settings_from(&path).unwrap();
        let keys: Vec<_> = settings.keys().cloned().collect();
        assert_eq!(keys, ["zeta", "alpha"]);
        assert_eq!(settings["alpha"], "echo a");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn rejects_non_string_values() {
        let dir = temp_dir("badtype");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "nested:\n  a: b\n").unwrap();

        assert!(read_settings_from(&path).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn malformed_yaml_returns_error() {
        let dir = temp_dir("malformed");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "rust: [unclosed\n").unwrap();

        assert!(read_settings_from(&path).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn update_pkg_runs_command() {
        assert!(update_pkg("ok", "true now").is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn update_pkg_errors_when_program_missing() {
        assert!(update_pkg("bad", "definitely-not-a-program-xyz arg").is_err());
    }

    #[test]
    #[cfg(unix)]
    fn update_pkg_accepts_command_without_args() {
        assert!(update_pkg("ok", "true").is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn run_command_passes_multiple_args() {
        assert!(run_command("test a = a").unwrap().success());
        assert!(!run_command("test a = b").unwrap().success());
    }

    #[test]
    fn reads_dotted_keys() {
        let dir = temp_dir("dotted");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "node.js: echo n\n").unwrap();

        assert_eq!(read_settings_from(&path).unwrap()["node.js"], "echo n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn update_pkg_errors_on_nonzero_exit() {
        assert!(update_pkg("bad", "false").is_err());
    }

    #[test]
    fn supported_os() {
        assert!(is_supported_os("macos"));
        assert!(is_supported_os("linux"));
        assert!(!is_supported_os("windows"));
    }

    #[test]
    #[cfg(unix)]
    fn update_pkg_errors_on_empty_command() {
        let err = update_pkg("bad", "   ").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }
}
