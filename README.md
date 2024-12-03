### STRUTURE GLOBALE DU PROJET
```
my_shell_project/
├── src/
│   ├── core/
│   │   ├── prompt.rs               # Babs: Affichage du prompt
│   │   ├── parser.rs               # Babs: Parsing des commandes
│   │   ├── interpreter.rs          # Babs: Interprétation des commandes
│   │   ├── executor.rs             # Babs: Exécution des commandes
│   │   ├── error.rs                # Babs: Gestion des erreurs
│   │   ├── utils.rs                # Babs: Fonctions utilitaires globales
│   ├── commands/
│   │   ├── echo.rs                 # Kendi : Implémentation de la commande "echo"
│   │   ├── cd.rs                   # Kendi : Implémentation de la commande "cd"
│   │   ├── ls.rs                   # Gaby : Implémentation de la commande "ls"
│   │   ├── pwd.rs                  # Kendi : Implémentation de la commande "pwd"
│   │   ├── cat.rs                  # Gaby : Implémentation de la commande "cat"
│   │   ├── cp.rs                   # Gaby : Implémentation de la commande "cp"
│   │   ├── rm.rs                   # Gaby : Implémentation de la commande "rm"
│   │   ├── mv.rs                   # Gaby : Implémentation de la commande "mv"
│   │   ├── mkdir.rs                # Kendi : Implémentation de la commande "mkdir"
│   │   ├── exit.rs                 # Kendi : Implémentation de la commande "exit"
│   ├── scripting/
│   │   ├── script_runner.rs        # Babs: Exécution des scripts
│   │   ├── parser.rs               # Babs: Parsing des scripts
│   │   ├── executor.rs             # Babs: Exécution des commandes dans les scripts
│   │   ├── utils.rs                # Babs: Fonctions utilitaires pour les scripts
│   │   ├── builtin.rs              # Babs: Gestion des commandes intégrées des scripts
│   ├── job_control/
│   │   ├── jobs.rs                 # Gaby : Commande "jobs" - Liste des jobs
│   │   ├── bg.rs                   # Gaby : Commande "bg" - Mettre un job en arrière-plan
│   │   ├── fg.rs                   # Gaby : Commande "fg" - Ramener un job au premier plan
│   │   ├── kill.rs                 # Gaby : Commande "kill" - Tuer un job
│   │   ├── signal_handler.rs       # Gaby : Gestion des signaux, y compris Ctrl+Z pour suspendre
│   │   ├── job_manager.rs          # Gaby : Gestionnaire de jobs, création, suspension, et reprise des jobs
│   ├── shell.rs                    # Babs: Module principal du shell
│   ├── main.rs                     # Babs: Entrée principale du programme
├── Cargo.toml                      # Babs: Configuration du projet
├── README.md                       # Babs: Documentation du projet

```