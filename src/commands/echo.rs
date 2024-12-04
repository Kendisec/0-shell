use std::collections::HashMap;
use crate::core::variables::handle_variables;


pub fn echo(args: Vec<String>, variables: &HashMap<String, String>) {
    if args.is_empty() {
        println!();
        return;
    }

    let mut filtre_arg = Vec::new();
    for arg in args {
        filtre_arg.push(handle_variables(&arg, variables));
    }

    println!("{}", filtre_arg.join(" "));
}
