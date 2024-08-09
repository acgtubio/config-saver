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

// base_config_dir_exists() checks if the .base file exists
fn base_config_dir_exists() -> bool {
    let home_path = std::env::var("HOME").unwrap();
    let base_file_path = home_path.clone() + "/.config/config-swapper/.base";

    Path::new(&base_file_path).exists()
}

// create_base_config_dir() creates a directory in $HOME/.config/config-swapper/
// and the .base file that contains something TODO: here
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

// proj_config_file_exists() checks if the .cfs file exists in the current directory.
fn proj_config_file_exists() -> bool {
    Path::new("./.cfs").exists()
}

fn create_config_location() {
    println!("Creating base config...\n");
    todo!()
}

fn create_proj_config_file() -> Result<(), std::io::Error> {
    println!("Creating project config file...\n");

    fs::write(Path::new("./.cfs"), "")?;

    Ok(())
}

// init_handler() initializes the directory to create the .cfs file which contains information about
// the current configuration. TODO: Add proper docs
//
// If the base configuration directory in the $HOME/.config/config-swapper/ directory does not exist, create it first.
//
// Once the base directory is created, create a directory specific to the current project which
// will contain all the config files for the project.
fn init_handler(mode: String) {
    if !base_config_dir_exists() {
        create_base_config_dir();
    }

    if !is_config_initialized() {
        create_config_location();
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

// reset_handler() removes all existing .cfs file in the current directory
fn reset_handler() {
    println!("Removing config file...");
    if !proj_config_file_exists() {
        return;
    }

    if let Err(..) = fs::remove_file(Path::new("./.cfs")) {
        println!("Error removing config file...");
    }
}
