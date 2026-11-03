use std::process;

use clap::Parser;
use colored::Colorize;

use lazy_up::{check_settings, create_settings_file, is_supported_os, read_settings, update_pkg};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Package to update, or "all" for every configured package
    #[arg(short, long, default_value = "all")]
    pkg: String,
}

fn main() -> std::io::Result<()> {
    if !is_supported_os(std::env::consts::OS) {
        eprintln!(
            "{}",
            "lazy-up only supports macOS and Linux. Please run it on one of those systems.".red()
        );
        process::exit(1);
    }

    let args = Args::parse();
    if !check_settings() {
        create_settings_file()?;
        eprintln!(
            "{}{}\n{}",
            "Created default config file ".red(),
            "'~/.config/lazy-up/pkgs.yml'".yellow(),
            "Please update and re-run.".red(),
        );
        process::exit(1);
    }
    let settings = match read_settings() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    println!(
        "Running: {}, {}",
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let selected: Vec<(&String, &String)> = if args.pkg == "all" {
        settings.iter().collect()
    } else {
        match settings.get_key_value(&args.pkg) {
            Some(entry) => vec![entry],
            None => {
                let names: Vec<&str> = settings.keys().map(String::as_str).collect();
                eprintln!(
                    "Could not find package '{}'. Available: {}",
                    args.pkg,
                    names.join(", ")
                );
                process::exit(1);
            }
        }
    };

    let mut failed = false;
    for (name, command) in selected {
        if let Err(e) = update_pkg(name, command) {
            eprintln!("{} {}: {e}", "Error updating".red(), name.yellow());
            failed = true;
        }
    }
    if failed {
        process::exit(1);
    }
    Ok(())
}
