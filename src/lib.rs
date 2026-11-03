use colored::Colorize;
use config::{Config, ConfigError, Map};
use std::env;
use std::fs::{self, File};
use std::io::Write;
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
    if !dir.exists() {
        fs::create_dir_all(dir)?
    }

    let mut file = File::create(dir.join("pkgs.yml"))?;
    file.write_all(b"rust: rustup update\n")?;

    Ok(())
}

pub fn read_settings() -> Result<Map<String, String>, ConfigError> {
    read_settings_from(settings_dir().join("pkgs.yml"))
}

fn read_settings_from(path: PathBuf) -> Result<Map<String, String>, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    settings.try_deserialize::<Map<String, String>>()
}

pub fn update_pkg(pkg_name: String, command: String) -> std::io::Result<()> {
    println!(
        "{}{}{}",
        "\n*** Updating: ".green(),
        pkg_name.blue(),
        " ***".green()
    );
    run_command(&command)?;
    Ok(())
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
        assert_eq!(content, "rust: rust up\n");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reads_settings_in_file_order() {
        let dir = temp_dir("read");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "zeta: echo z\nalpha: echo a\n").unwrap();

        let settings = read_settings_from(path).unwrap();
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

        assert!(read_settings_from(path).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn malformed_yaml_returns_error() {
        let dir = temp_dir("malformed");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pkgs.yml");
        fs::write(&path, "rust: [unclosed\n").unwrap();

        assert!(read_settings_from(path).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn update_pkg_runs_command() {
        assert!(update_pkg("ok".into(), "true now".into()).is_ok());
    }

    #[test]
    fn update_pkg_errors_when_program_missing() {
        assert!(update_pkg("bad".into(), "definitely-not-a-program-xyz arg".into()).is_err());
    }

    #[test]
    fn update_pkg_accepts_command_without_args() {
        assert!(update_pkg("ok".into(), "true".into()).is_ok());
    }

    #[test]
    fn run_command_passes_multiple_args() {
        assert!(run_command("test a = a").unwrap().success());
        assert!(!run_command("test a = b").unwrap().success());
    }

    #[test]
    fn update_pkg_errors_on_empty_command() {
        let err = update_pkg("bad".into(), "   ".into()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }
}
