use anyhow::Result;
use rustyline::Editor;
use std::io::{self, Write};

use crate::commands::autocomplete::*;
use crate::commands::cd::get_home_dir;

pub struct Prompt;

impl Prompt {
    pub fn new() -> Self {
        Self
    }

    pub fn display(&self) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let dir_str = current_dir.to_str().unwrap_or("");
        let home = get_home_dir();

        let dir = if let Some(home_str) = home.as_ref() {
            dir_str.replace(home_str, "~")
        } else {
            dir_str.to_string()
        };

        let icon = if Some(dir_str) == home.as_deref() {
            ""
        } else {
            ""
        };

        let blue = "\x1b[34m";
        let green = "\x1b[32m";
        let cyan = "\x1b[36m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";

        println!(
            "{}╭─{}  │ {}{} {}{} {}{}",
            cyan, reset, icon, blue, dir, reset, bold, reset
        );
        print!("{}╰─{}${} ", cyan, green, reset);

        io::stdout().flush()?;
        Ok(())
    }

    pub fn read_input(&self) -> Result<String> {
        let mut editor = Editor::with_config(
            rustyline::Config::builder()
                .auto_add_history(true)
                .completion_type(rustyline::CompletionType::List)
                .build(),
        )?;

        editor.set_helper(Some(ShellHelper::new()));

        match editor.readline(&format!("{}╰─{}${} ", "\x1b[36m", "\x1b[32m", "\x1b[0m")) {
            Ok(line) => Ok(line),
            Err(err) => Err(anyhow::anyhow!("Erreur de lecture: {}", err)),
        }
    }
}
