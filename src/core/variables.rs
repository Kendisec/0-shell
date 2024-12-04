use std::collections::HashMap;

pub fn handle_variables(arg: &str, variables: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < arg.len() {
        let c = arg[i..].chars().next().unwrap();

        if c == '$' {
            if let Some((output, new_index)) = handle_dollar_expression(arg, i, variables) {
                result.push_str(&output);
                i = new_index;
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }

        i += 1;
    }

    result
}

fn handle_dollar_expression(arg: &str, start: usize, variables: &HashMap<String, String>) -> Option<(String, usize)> {
    if start + 1 < arg.len() && &arg[start + 1..start + 2] == "(" {
        handle_subcommand(arg, start + 2).map(|(output, end)| (output, end + 1))
    } else {
        handle_variable(arg, start + 1, variables)
    }
}

fn handle_subcommand(arg: &str, start: usize) -> Option<(String, usize)> {
    let end = find_closing_parenthesis(arg, start)?;
    let subcmd = &arg[start..end];
    let output = match subcmd {
        "pwd" => "appel de la fonction pwd @Kendi".to_string(),
        _ => format!("commande inconnue: {}", subcmd),
    };
    Some((output, end))
}

fn handle_variable(arg: &str, start: usize, variables: &HashMap<String, String>) -> Option<(String, usize)> {
    let mut end = start;
    while end < arg.len() &&
    (arg[end..].chars().next().unwrap().is_alphanumeric() ||
    arg[end..].chars().next().unwrap() == '_') {
        end += 1;
    }

    let var_name = &arg[start..end];

    if let Some(value) = variables.get(var_name) {
        Some((value.to_string(), end - 1))
    } else {
        eprintln!("Erreur: La variable d'environnement '{}' n'est pas définie.", var_name);
        None
    }
}

//ça vous rappel pas bracket :):)
fn find_closing_parenthesis(arg: &str, start: usize) -> Option<usize> {
    let mut depth = 1;

    for (i, c) in arg[start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(start + i);
                }
            }
            _ => {}
        }
    }

    None
}