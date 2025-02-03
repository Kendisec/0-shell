use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use super::{error, executor, interpreter::{self}, prompt};
pub use prompt::Prompt;
pub use interpreter::Interpreter;
pub use executor::Executor;
pub use error::ShellError;


pub struct Shell {
    prompt: Prompt,
    interpreter: Interpreter,
    executor: Executor,
    variables: HashMap<String, String>
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prompt: Prompt::new(),
            interpreter: Interpreter::new(),
            executor: Executor::new(),
            variables: HashMap::new(),
        }
    }

    fn run_shell_command(&self, command: &str) -> anyhow::Result<()> {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    pub fn run_script(&mut self, script_path: &str) -> anyhow::Result<()> {
        let file = File::open(script_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let input = line?;
            if input.trim().is_empty() {
                continue;
            }

            if input.trim() == "exit" {
                break;
            }

            match self.interpreter.parse(&input).and_then(|cmd| self.executor.execute(cmd, &mut self.variables)) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.prompt.display()?;
            let input = self.prompt.read_input()?;

            if input.trim().starts_with("0sh ") {
                let script_path = input.trim().strip_prefix("0sh ").unwrap();
                if let Err(e) = self.run_script(script_path) {
                    eprintln!("Erreur lors de l'exécution du script: {}", e);
                }
                continue;
            }

            if input.trim() == "exit" {
                break;
            }

            if input.trim().starts_with("create-dir") {
                if let Err(e) = self.run_shell_command("src/commands/create_dir.sh") {
                    eprintln!("Erreur lors de l'exécution de create-dir: {}", e);
                }
                continue;
            }

            match self.interpreter.parse(&input).and_then(|cmd| self.executor.execute(cmd, &mut self.variables)) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
        Ok(())
    }
}