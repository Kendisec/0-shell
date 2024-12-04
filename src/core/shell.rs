use std::collections::HashMap;

use super::{error, executor, interpreter, prompt};
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
}