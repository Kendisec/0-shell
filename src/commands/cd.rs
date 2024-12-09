use std::{env, path::PathBuf};
use dirs::home_dir;

fn path_to_string(path: PathBuf) -> String {
    // Convert PathBuf to String
    path.into_os_string().into_string().unwrap_or_else(|_| {
        eprintln!("Failed to convert path to string");
        std::process::exit(1);
    })
}

pub fn get_home_dir() -> Option<String>{
    match home_dir() {
        Some(test) => {
            Some(path_to_string(test))
        }
        None => None,
    }
}

pub fn change_directory(path: &str) -> Result<(), String> {
    // Use the Rust standard library to change the current directory.
    env::set_current_dir(path)
        .map_err(|e| format!("Failed to change directory: {}", e))
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_change_to_valid_directory() {
        // Change to a valid directory, like the system's temporary directory.
        let result = change_directory("/tmp");
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    }

    #[test]
    fn test_change_to_invalid_directory() {
        // Attempt to change to an invalid directory.
        let result = change_directory("/this/directory/does/not/exist");
        assert!(result.is_err(), "Expected Err, got Ok");
    }

    #[test]
    fn test_current_directory_change() {
        use std::env;

        // Save the current directory.
        let original_dir = env::current_dir().expect("Failed to get current directory");

        // Change to a valid directory.
        change_directory("/tmp").expect("Failed to change to /tmp");

        // Verify the current directory is now /tmp.
        let current_dir = env::current_dir().expect("Failed to get current directory");
        assert_eq!(current_dir.to_string_lossy(), "/tmp", "Expected /tmp, got {:?}", current_dir);

        // Revert to the original directory.
        change_directory(original_dir.to_str().unwrap()).expect("Failed to revert to original directory");
    }
}