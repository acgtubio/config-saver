use clap::{Parser, Subcommand};
use std::{fs, io::Error, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, default_value = "")]
    mode: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    Config,
    Init,
    TrackFile {
        #[arg(short, long)]
        filename: String,
    },
    Reset,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Config => {
            print!("showing config...\n");
        }
        Command::Init => {
            init_handler(args.mode);
        }
        Command::TrackFile { filename } => {
            println!("Tracking: {}", filename);
        }
        Command::Reset => {
            reset_handler();
        }
    }
}

fn base_config_dir_exists() -> bool {
    Path::new("~/.config/config-swapper/.base").exists()
}

fn create_base_config_dir() {
    let home_path = std::env::var("HOME").unwrap();

    let config_dir_path = home_path.clone() + "/.config/config-swapper/";
    if let Err(..) = fs::create_dir_all(config_dir_path) {
        let result = "Error creating config folder in ~/.config/config-swapper/";
        println!("{}", result);
        return;
    }

    let base_file_path = home_path.clone() + "/.config/config-swapper/.base";
    if let Err(..) = fs::write(Path::new(&base_file_path), "") {
        let result = "Error creating config file in ~/.config/config-swapper/.base";
        println!("{}", result);
        return;
    }

    println!("Successfully created base config files.");
}

fn is_config_initialized() -> bool {
    false
}

fn proj_config_file_exists() -> bool {
    Path::new("./.cfs").exists()
}

fn create_config_location() {
    println!("Creating base config...\n");
}

fn create_proj_config_file() -> Result<(), std::io::Error> {
    println!("Creating project config file...\n");

    fs::write(Path::new("./.cfs"), "")?;

    Ok(())
}

fn init_handler(mode: String) {
    if !is_config_initialized() {
        create_config_location();
    }

    if !base_config_dir_exists() {
        create_base_config_dir();
    }

    if proj_config_file_exists() {
        println!("Config already exists...");

        // Temporarily set as comment due to testing code below.
        // return;

        // We remove config files when using it as testing.
        if mode == "debug" {
            reset_handler();
        } else if mode != "debug" && mode != "" {
            println!("unknown mode...");
            return;
        } else {
            return;
        }
    }

    if let Err(..) = create_proj_config_file() {
        println!("Error creating config file.");
    }
}

fn reset_handler() {
    println!("Removing config file...");
    if !proj_config_file_exists() {
        return;
    }

    if let Err(..) = fs::remove_file(Path::new("./.cfs")) {
        println!("Error removing config file...");
    }
}
