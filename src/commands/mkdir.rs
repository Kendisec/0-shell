use std::fs;

pub fn create_directory(dir_name: &String) -> Result<(), String> {
    match fs::create_dir(dir_name) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create directory '{}': {}", dir_name, e)),
    }
}