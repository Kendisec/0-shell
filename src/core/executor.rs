use std::collections::HashMap;

use crate::commands::{cat::cat, cd::{change_directory, get_home_dir}, clear::clear, cp::cp, echo::echo, help, ls::ls, mkdir::create_directory, mv::mv, pwd::get_current_directory, rm::rm, touch::touch};

use super::interpreter::Command;
use anyhow::Result;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: Command, variables: &mut HashMap<String, String>) -> Result<()> {
        match command.name.as_str() {
            "" => Ok(()),     // Empty command
            "exit" => Ok(()), // Exit command
            "echo" => Ok(echo(command.args, variables)),
            "cat" => cat(command.args).map_err(anyhow::Error::from),
            "ls" => ls(command.args).map_err(anyhow::Error::from),
            "touch" => touch(command.args).map_err(anyhow::Error::from),
            "rm" => rm(command.args).map_err(anyhow::Error::from),
            "cp" => cp(command.args).map_err(anyhow::Error::from),
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
            "clear" => {
                clear();
                Ok(())
            },
            "pwd" => {
                match get_current_directory() {
                    Ok(dir) => {
                        println!("{}", dir);
                        Ok(())
                    }
                    Err(err) => Err(anyhow::anyhow!("Error executing pwd: {}", err)),
                }
            },
            "cd" => {
                if let Some(path) = command.args.get(0) {
                    match change_directory(path) {
                        Ok(()) => Ok(()),
                        Err(_err) => {
                            Err(anyhow::anyhow!("cd: no such file or directory: {}", path))
                        }
                    }
                } else {
                    let home = get_home_dir();
                    if let Some(path) = home {
                        match change_directory(&path) {
                            Ok(()) => Ok(()),
                            Err(_err) => {
                                Err(anyhow::anyhow!("cd: no such file or directory {}", path))
                            }
                        }
                    } else {
                        Err(anyhow::anyhow!("cd: no such file or directory "))
                    }
                }
            }
            "mkdir" => {
                if let Some(paths) = command.args.get(..) {
                    for path in paths {
                        if let Err(err) = create_directory(path) {
                            return Err(anyhow::anyhow!(
                                "mkdir: failed to create directory '{}': {}",
                                path,
                                err
                            ));
                        }
                    }
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("mkdir: missing operand"))
                }
            }
            "mv" => {
                if let Some(paths) = command.args.get(..) {
                    // let command = Command {args : paths.to_vec(), name: "test".to_string() };
                    let file_destination = paths[paths.len()-1].clone();
                    for (i,path) in paths.iter().enumerate(){
                        if i < paths.len()-1{
                            match mv(path, &file_destination) {
                                Ok(_) => println!("Files moved successfully."),
                                Err(err) => eprintln!("Error: {}", err),
                            }
                        }
                    }
                        Ok(())
                } else {
                    Err(anyhow::anyhow!("mv: missing operand"))
                }
            }
            "help" => {
                help::help();
                Ok(())
            }
            cmd => Err(anyhow::anyhow!("Command '{}' not found", cmd)),
        }
    }
}

