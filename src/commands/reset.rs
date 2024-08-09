use super::utils::dirs::*;
use std::{fs, path::Path};

// reset_handler() removes all existing .cfs file in the current directory
pub fn reset_handler() {
    println!("Removing config file...");
    if !proj_config_file_exists() {
        return;
    }

    if let Err(..) = fs::remove_file(Path::new("./.cfs")) {
        println!("Error removing config file...");
    }
}
