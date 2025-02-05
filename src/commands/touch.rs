use std::fs::File;
use std::io;

pub fn touch(args: Vec<String>) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("Erreur: Aucun fichier spécifié.");
        return Ok(());
    }

    for file_name in args {
        match File::create(&file_name) {
            Ok(_) => println!("Fichier créé : {}", file_name),
            Err(e) => eprintln!("Erreur: Impossible de créer '{}': {}", file_name, e),
        }
    }
    Ok(())
}