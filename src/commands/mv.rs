use std::{fs, path::{Path, PathBuf}};

pub fn mv(source: &str, destination: &str) -> Result<(), std::io::Error> {
      // Check if the source exists

      let sr = Path::new(source);
      let dst = Path::new(destination);

      if !sr.exists() {
        eprintln!("Error: Source '{}' does not exist.", sr.display());
        std::process::exit(1);
    }

    let file_name = match sr.file_name() {
        Some(name) => name,
        None => {
            eprintln!(
                "Error: Source '{}' does not have a valid file name.",
                sr.display()
            );
            std::process::exit(1);
        }
    };

    // Determine the target path
    let target = if dst.is_dir() {
        // If destination is a directory, move the source into it
        dst.join(file_name)
    } else {
        // Otherwise, use the destination directly
        PathBuf::from(destination)
    };

    // Try renaming (moving) the file or directory
    match fs::rename(&source, &target) {
        Ok(_) => println!("Moved '{}' to '{}'.", sr.display(), target.display()),
        Err(e) => {
            eprintln!("Error moving '{}': {}", sr.display(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}
