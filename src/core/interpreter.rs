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
        let mut parts = input.trim().split_whitespace();
        let name = parts.next().unwrap_or("").to_string();
        let args = parts.map(String::from).collect();

        Ok(Command { name, args })
    }
}