pub fn help() {
    println!("Liste des commandes disponibles:");
    println!("  echo [args]        - Affiche les arguments fournis");
    println!("  cd [dir]           - Change le répertoire courant");
    println!("  ls [options]       - Liste les fichiers du répertoire courant");
    println!("        -l        Affiche les détails des fichiers");
    println!("        -a        Affiche tous les fichiers, y compris les fichiers cachés");
    println!("        -F        Ajoute un '/' à la fin des répertoires");
    println!("  pwd                - Affiche le répertoire courant");
    println!("  cat [file]         - Affiche le contenu du fichier");
    println!("  cp [src] [dest]    - Copie le fichier source vers la destination");
    println!("  rm [file]          - Supprime le fichier");
    println!("  touch [file]       - Crée un nouveau fichier vide");
    println!("  mv [src] [dest]    - Déplace ou renomme le fichier source");
    println!("  mkdir [dir]        - Crée un nouveau répertoire");
    println!("  clear              - Efface l'écran");
    println!("  exit               - Quitte le shell");
    println!("  help               - Affiche cette aide");
}