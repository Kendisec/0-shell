use super::{error, executor, interpreter, prompt};
pub use prompt::Prompt;
pub use interpreter::Interpreter;
pub use executor::Executor;
pub use error::ShellError;


pub struct Shell {
    prompt: Prompt,
    interpreter: Interpreter,
    executor: Executor,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prompt: Prompt::new(),
            interpreter: Interpreter::new(),
            executor: Executor::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.prompt.display()?;
            let input = self.prompt.read_input()?;
            
            if input.trim() == "exit" {
                break;
            }
    
            match self.interpreter.parse(&input).and_then(|cmd| self.executor.execute(cmd)) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e), 
            }
        }
        Ok(())
    }
}