use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn cat(args: Vec<String>) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("Erreur: Aucun fichier spécifié.");
        return Ok(());
    }

    for file_name in args {
        match File::open(&file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("{}", line?);
                }
            }
            Err(e) => {
                eprintln!("Erreur: Impossible d'ouvrir '{}': {}", file_name, e);
            }
        }
    }
    Ok(())
}
