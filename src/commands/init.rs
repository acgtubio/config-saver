use std::{fs, path::Path};

use super::reset::reset_handler;
use super::utils::dirs::*;

// create_base_config_dir() creates a directory in $HOME/.config/config-swapper/
// and the .base file that contains something TODO: here
pub fn create_base_config_dir() {
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

pub fn create_proj_config_file() -> Result<(), std::io::Error> {
    println!("Creating project config file...\n");

    fs::write(Path::new("./.cfs"), "")?;

    Ok(())
}

pub fn create_config_location() {
    println!("Creating base config...\n");
    todo!()
}

// init_handler() initializes the directory to create the .cfs file which contains information about
// the current configuration. TODO: Add proper docs
//
// If the base configuration directory in the $HOME/.config/config-swapper/ directory does not exist, create it first.
//
// Once the base directory is created, create a directory specific to the current project which
// will contain all the config files for the project.
pub fn init_handler(mode: String) {
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
