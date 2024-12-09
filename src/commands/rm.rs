use std::fs;
use std::io;
use std::path::Path;

pub struct RmOptions {
    pub recursive: bool,
}

pub fn rm(args: Vec<String>) -> io::Result<()> {
    // Analyse des flags
    let options = parse_rm_flags(&args);

    // Récupérer les chemins à supprimer (tout ce qui n'est pas un flag)
    let targets: Vec<&String> = args.iter().filter(|arg| !arg.starts_with('-')).collect();

    if targets.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Aucun fichier ou répertoire spécifié",
        ));
    }

    // Supprimer chaque cible
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

// Fonction pour analyser les flags de la commande rm
fn parse_rm_flags(args: &[String]) -> RmOptions {
    let recursive = args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string());

    RmOptions { recursive }
}

// Supprimer un fichier
fn remove_file(path: &Path) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Fichier supprimé : {}", path.display());
    Ok(())
}

// Supprimer un répertoire de manière récursive
fn remove_directory_recursive(path: &Path) -> io::Result<()> {
    fs::remove_dir_all(path)?;
    println!("Répertoire supprimé : {}", path.display());
    Ok(())
}
