use anyhow::Result;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Result<Command> {
        let input = input.trim();

        if let Some(eq_pos) = input.find('=') {
            let (var_name, var_value) = input.split_at(eq_pos);
            let var_name = var_name.trim();
            let var_value = var_value[1..].trim().trim_matches('"');

            if !var_value.is_empty() {
                return Ok(Command {
                    name: "set_variable".to_string(),
                    args: vec![var_name.to_string(), var_value.to_string()],
                });
            }
        }

        let mut parts = input.split_whitespace();
        let name = parts.next().unwrap_or("").to_string();
         let args = parts
        .map(|arg| arg.trim_matches('"').to_string())
        .collect::<Vec<String>>();

        Ok(Command { name, args })
    }
}
