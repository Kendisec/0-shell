use std::collections::HashMap;
use crate::core::variables::handle_variables;


pub fn echo(args: Vec<String>, variables: &HashMap<String, String>) {
    if args.is_empty() {
        println!();
        return;
    }

    let mut filtre_arg = Vec::new();
    for arg in args {
        let handled_arg = handle_variables(&arg, variables);
        let filtered_arg = handled_arg.replace("\"", "")
                                      .replace("\\n", "\n")
                                      .replace("\\t", "\t")
                                      .replace("\\r", "\r");
        filtre_arg.push(filtered_arg);
    }

    println!("{}", filtre_arg.join(" "));
}
 