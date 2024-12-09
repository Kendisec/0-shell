use std::fs;
use std::path::Path;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;

#[derive(Default)]
pub struct ShellCompleter;

impl ShellCompleter {
    fn new() -> Self {
        Self
    }
}

#[derive(Default)]
pub struct ShellHelper {
    pub completer: ShellCompleter,
}

impl ShellHelper {
    pub fn new() -> Self {
        Self {
            completer: ShellCompleter::new(),
        }
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<String>)> {
        let word_start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &line[word_start..pos];

        let completions = get_completions(word);
        Ok((word_start, completions))
    }
}

fn get_completions(partial: &str) -> Vec<String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return vec![],
    };

    let search_path = if partial.contains('/') {
        Path::new(partial)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    } else {
        current_dir
    };

    let prefix = Path::new(partial)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    match fs::read_dir(search_path) {
        Ok(entries) => {
            let mut completions: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let name = e.file_name().to_string_lossy().to_string();
                        if name.starts_with(&prefix) {
                            if e.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                Some(format!("{}/", name))
                            } else {
                                Some(name)
                            }
                        } else {
                            None
                        }
                    })
                })
                .collect();
            completions.sort();
            completions
        }
        Err(_) => vec![],
    }
}

impl Helper for ShellHelper {}

impl Validator for ShellHelper {}

impl Highlighter for ShellHelper {}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}