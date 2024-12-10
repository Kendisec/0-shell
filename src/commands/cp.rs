use std::fs;
use std::io;
use std::path::Path;

pub struct CpOptions {
    pub recursive: bool,
}

pub fn cp(args: Vec<String>) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Usage: cp [options] <source>... <destination>",
        ));
    }

    let options = parse_cp_flags(&args);

    let destination = &args[args.len() - 1];
    let destination_path = Path::new(destination);

    let sources = if options.recursive{
        &args[1..args.len() - 1]
    } else {
        &args[..args.len() - 1]
    };

    if !destination_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("'{}' n'existe pas.", destination),
        ));
    }

    if !destination_path.is_dir() && args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalide destination '{}' n'est pas un répertoire.", destination),
        ));
    }

    for source in sources {
        let source_path = Path::new(source);

        if !source_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("'{}' n'existe pas.", source),
            ));
        }

        let destination_file_path = if destination_path.is_dir() {
            destination_path.join(source_path.file_name().unwrap())
        } else {
            destination_path.to_path_buf()
        };

        if source_path.is_file() {
            copy_file(source_path, &destination_file_path)?;
        } else if source_path.is_dir() {
            if options.recursive {
                copy_directory_recursive(source_path, &destination_file_path)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!(
                        "La source '{}' est un répertoire. Utilisez '-r' pour copier récursivement.",
                        source
                    ),
                ));
            }
        }
    }

    println!("Copie terminée vers '{}'", destination);
    Ok(())
}

fn parse_cp_flags(args: &[String]) -> CpOptions {
    let recursive = args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string());

    CpOptions { recursive }
}

fn copy_file(source: &Path, destination: &Path) -> io::Result<()> {
    let dest = if destination.is_dir() {
        destination.join(source.file_name().unwrap())
    } else {
        destination.to_path_buf()
    };
    fs::copy(source, &dest)?;
    println!("Fichier copié : {} -> {}", source.display(), dest.display());
    Ok(())
}

fn copy_directory_recursive(source: &Path, destination: &Path) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory_recursive(&source_path, &dest_path)?;
        } else {
            copy_file(&source_path, &dest_path)?;
        }
    }

    Ok(())
}

