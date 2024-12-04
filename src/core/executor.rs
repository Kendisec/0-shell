use crate::commands::{cd::{change_directory, get_home_dir}, pwd::get_current_directory};

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
            "exit" => Ok(()), // Exit command
            "pwd" => {
                match get_current_directory() {
                    Ok(dir) => {
                        println!("{}", dir);
                        Ok(())
                    }
                    Err(err) => Err(anyhow::anyhow!("Error executing pwd: {}", err)),
                }
            }
            "cd" => {
                // Ensure a path is provided as an argument
                if let Some(path) = command.args.get(0) {
                    match change_directory(path) {
                        Ok(()) => Ok(()),
                        Err(_err) => Err(anyhow::anyhow!("cd: no such file or directory: {}", path)),
                    }
                } else {
                    let home = get_home_dir();
                    if let Some(path) = home{
                        match change_directory(&path) {
                            Ok(()) => Ok(()),
                            Err(_err) => Err(anyhow::anyhow!("cd: no such file or directory {}", path)),
                        }
                    }else{
                        Err(anyhow::anyhow!("cd: no such file or directory "))
                    }
                }
            }
            cmd => Err(anyhow::anyhow!("Command '{}' not found", cmd)),
        }
    }
}