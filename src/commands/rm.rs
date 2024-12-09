use std::fs;
use std::io;
use std::path::Path;

pub struct RmOptions {
    pub recursive: bool,
}

pub fn rm(args: Vec<String>) -> io::Result<()> {
    let options = parse_rm_flags(&args);

    let targets: Vec<&String> = args.iter().filter(|arg| !arg.starts_with('-')).collect();

    if targets.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Aucun fichier ou répertoire spécifié",
        ));
    }

    for target in targets {
        let path = Path::new(target);
        if path.is_dir() {
            if options.recursive {
                remove_directory_recursive(path)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("'{}' est un répertoire. Utilisez '-r' pour le supprimer.", target),
                ));
            }
        } else if path.is_file() {
            remove_file(path)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("'{}' n'existe pas.", target),
            ));
        }
    }

    Ok(())
}

fn parse_rm_flags(args: &[String]) -> RmOptions {
    let recursive = args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string());

    RmOptions { recursive }
}

fn remove_file(path: &Path) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Fichier supprimé : {}", path.display());
    Ok(())
}

fn remove_directory_recursive(path: &Path) -> io::Result<()> {
    fs::remove_dir_all(path)?;
    println!("Répertoire supprimé : {}", path.display());
    Ok(())
}
