use std::process;

use clap::Parser;
use colored::Colorize;

use lazy_up::{check_settings, create_settings_file, read_settings, update_pkg};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "all")]
    pkg: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if !check_settings() {
        create_settings_file()?;
        println!(
            "{}{}\n{}",
            "Created default config file ".red(),
            "'~/.config/lazy-up/pkgs.yml'".yellow(),
            "Please update and re-run.".red(),
        );
        process::exit(0);
    }
    let settings = match read_settings() {
        Ok(s) => s,
        Err(ce) => {
            eprintln!("{}", ce);
            process::exit(1);
        }
    };

    println!(
        "Running: {}, {}",
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    if args.pkg == "all" {
        for setting in settings.into_iter() {
            update_pkg(setting.0, setting.1)?;
        }
    } else {
        let command = settings.get(&args.pkg);
        match command {
            Some(c) => update_pkg(args.pkg, c.to_owned())?,
            None => println!("Could not find package."),
        };
    }
    Ok(())
}
