use std::collections::HashMap;

pub fn echo(args: Vec<String>, variables: &HashMap<String, String>) {
    if args.is_empty() {
        println!();
        return;
    }

    let mut expanded_args = Vec::new();
    for arg in args {
        expanded_args.push(handle_variables(&arg, variables));
    }

    println!("{}", expanded_args.join(" "));
}

pub fn handle_variables(arg: &str, variables: &HashMap<String, String>) -> String {
    let arg = arg.trim_matches('"');

    if arg.starts_with("$") {
        let var_name = &arg[1..];
        
        match variables.get(var_name) {
            Some(value) => value.clone(),  
            None => {
                eprintln!("Erreur: La variable d'environnement '{}' n'est pas définie.", var_name);
                String::new()
            }
        }
    } else {
        arg.to_string()
    }
}