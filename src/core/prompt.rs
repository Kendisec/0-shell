use anyhow::Result;
use std::io::{self, Write};

pub struct Prompt;

impl Prompt {
    pub fn new() -> Self {
        Self
    }

    pub fn display(&self) -> Result<()> {
        print!("$ ");
        io::stdout().flush()?;
        Ok(())
    }

    pub fn read_input(&self) -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }
}