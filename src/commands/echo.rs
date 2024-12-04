use std::collections::HashMap;
use crate::core::variables::handle_variables;


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
