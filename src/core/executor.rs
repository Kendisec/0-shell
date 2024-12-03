use super::interpreter::Command;
use anyhow::Result;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: Command) -> Result<()> {
        match command.name.as_str() {
            "" => Ok(()), // Empty command
            "exit" => Ok(()),
            cmd => Err(anyhow::anyhow!("Command '{}' not found", cmd)),
        }
    }
}