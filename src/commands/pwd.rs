use std::env;

pub fn get_current_directory() -> Result<String, String> {
    env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to get current directory: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_get_current_directory() {
        // Ensure the function returns a valid directory
        let result = get_current_directory();
        assert!(result.is_ok());

        // Ensure the returned directory is the same as the current directory
        let current_dir = env::current_dir().unwrap();
        let result_dir = result.unwrap();
        assert_eq!(result_dir, current_dir.to_string_lossy().to_string());
    }
}