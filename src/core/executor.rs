use std::collections::HashMap;

use crate::commands::{cat::*, echo::echo};

use super::interpreter::Command;
use anyhow::Result;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: Command, variables: &mut HashMap<String, String>) -> Result<()> {
        match command.name.as_str() {
            "" => Ok(()), // Empty command
            "exit" => Ok(()),
            "echo" => Ok(echo(command.args, variables)),
            "cat" => cat(command.args).map_err(|e| anyhow::anyhow!(e.to_string())),
            "set_variable" => {
                if command.args.len() == 2 {
                    let var_name = &command.args[0];
                    let var_value = &command.args[1];
                    variables.insert(var_name.to_string(), var_value.to_string());
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Invalid variable assignment"))
                }
            }
            _ => Err(anyhow::anyhow!("Command '{}' not found", command.name)),
        }
    }
}