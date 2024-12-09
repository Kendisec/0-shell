use std::env;
use std::fs;
use std::io;
use std::path::*;

pub struct LsOptions {
    pub show_details: bool,
    pub show_hidden: bool,
    pub append_slash: bool,
}

pub fn ls(args: Vec<String>) -> io::Result<()> {
    let options = parse_flags(&args);

    let binding = ".".to_string();
    let dir = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .unwrap_or(&binding)
        .to_string();

    let file_names = get_file_names(&dir, options.show_hidden)?;

    display_files(&dir, &file_names, &options)?;

    Ok(())
}

fn parse_flags(args: &[String]) -> LsOptions {
    let show_details = args.contains(&"-l".to_string());
    let show_hidden = args.contains(&"-a".to_string());
    let append_slash = args.contains(&"-F".to_string());

    LsOptions {
        show_details,
        show_hidden,
        append_slash,
    }
}

fn get_file_names(dir: &str, show_hidden: bool) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(Path::new(dir))?;

    let mut file_names: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            if !show_hidden && file_name.starts_with('.') {
                None
            } else {
                Some(file_name)
            }
        })
        .collect();

    if show_hidden {
        file_names.push(".".to_string());
        file_names.push("..".to_string());
    }

    file_names.sort_by(|a, b| {
        let a_trimmed = a.trim_start_matches('.');
        let b_trimmed = b.trim_start_matches('.');
        a_trimmed.to_lowercase().cmp(&b_trimmed.to_lowercase())
    });
    

    Ok(file_names)
}

fn get_terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(80)
}

fn display_files(dir: &str, file_names: &[String], options: &LsOptions) -> io::Result<()> {
    let terminal_width = get_terminal_width();
    let mut line_length = 0;

    for file_name in file_names {
        if options.show_details {
            display_details(file_name)?;
        }

        print_file_name(dir, file_name, options.append_slash)?;

        line_length += file_name.len() + if options.append_slash { 1 } else { 0 };
        if line_length > terminal_width {
            println!();
            line_length = file_name.len() + if options.append_slash { 1 } else { 0 };
        } else {
            print!(" ");
        }
    }

    println!();
    Ok(())
}

fn display_details(_file_name: &str) -> io::Result<()> {
    todo!();
    // Ok(())
}

fn print_file_name(dir: &str, file_name: &str, append_slash: bool) -> io::Result<()> {
    let path = Path::new(dir).join(file_name);

    if let Ok(metadata) = fs::metadata(&path) {
        if metadata.is_dir() {
            print!("\x1b[34m{}\x1b[0m", file_name);
        } else {
            print!("{}", file_name);
        }

        if append_slash && metadata.is_dir() {
            print!("/");
        }
    }

    Ok(())
}