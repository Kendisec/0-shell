### STRUTURE GLOBALE DU PROJET
```
my_shell_project/
├── src/
│   ├── core/
│   │   ├── prompt.rs               # Personne A : Affichage du prompt
│   │   ├── parser.rs               # Personne A : Parsing des commandes
│   │   ├── interpreter.rs          # Personne A : Interprétation des commandes
│   │   ├── executor.rs             # Personne A : Exécution des commandes
│   │   ├── error.rs                # Personne A : Gestion des erreurs
│   │   ├── utils.rs                # Personne A : Fonctions utilitaires globales
│   ├── commands/
│   │   ├── echo.rs                 # Personne B : Implémentation de la commande "echo"
│   │   ├── cd.rs                   # Personne B : Implémentation de la commande "cd"
│   │   ├── ls.rs                   # Personne C : Implémentation de la commande "ls"
│   │   ├── pwd.rs                  # Personne B : Implémentation de la commande "pwd"
│   │   ├── cat.rs                  # Personne C : Implémentation de la commande "cat"
│   │   ├── cp.rs                   # Personne C : Implémentation de la commande "cp"
│   │   ├── rm.rs                   # Personne C : Implémentation de la commande "rm"
│   │   ├── mv.rs                   # Personne C : Implémentation de la commande "mv"
│   │   ├── mkdir.rs                # Personne B : Implémentation de la commande "mkdir"
│   │   ├── exit.rs                 # Personne B : Implémentation de la commande "exit"
│   ├── scripting/
│   │   ├── script_runner.rs        # Personne A : Exécution des scripts
│   │   ├── parser.rs               # Personne A : Parsing des scripts
│   │   ├── executor.rs             # Personne A : Exécution des commandes dans les scripts
│   │   ├── utils.rs                # Personne A : Fonctions utilitaires pour les scripts
│   │   ├── builtin.rs              # Personne A : Gestion des commandes intégrées des scripts
│   ├── job_control/
│   │   ├── jobs.rs                 # Personne C : Commande "jobs" - Liste des jobs
│   │   ├── bg.rs                   # Personne C : Commande "bg" - Mettre un job en arrière-plan
│   │   ├── fg.rs                   # Personne C : Commande "fg" - Ramener un job au premier plan
│   │   ├── kill.rs                 # Personne C : Commande "kill" - Tuer un job
│   │   ├── signal_handler.rs       # Personne C : Gestion des signaux, y compris Ctrl+Z pour suspendre
│   │   ├── job_manager.rs          # Personne C : Gestionnaire de jobs, création, suspension, et reprise des jobs
│   ├── shell.rs                    # Personne A : Module principal du shell
│   ├── main.rs                     # Personne A : Entrée principale du programme
├── Cargo.toml                      # Personne A : Configuration du projet
├── README.md                       # Personne A : Documentation du projet

```