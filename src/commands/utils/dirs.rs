use std::path::Path;

// base_config_dir_exists() checks if the .base file exists
pub fn base_config_dir_exists() -> bool {
    let home_path = std::env::var("HOME").unwrap();
    let base_file_path = home_path.clone() + "/.config/config-swapper/.base";

    Path::new(&base_file_path).exists()
}
//
// proj_config_file_exists() checks if the .cfs file exists in the current directory.
pub fn proj_config_file_exists() -> bool {
    Path::new("./.cfs").exists()
}

pub fn is_config_initialized() -> bool {
    false
}
