use std::env;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use std::time::UNIX_EPOCH;

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

fn calculate_total_blocks(dir: &str, file_names: &[String]) -> io::Result<u64> {
    let mut total_blocks = 0;

    for file_name in file_names {
        let path = Path::new(dir).join(file_name);

        if let Ok(metadata) = fs::metadata(&path) {
            total_blocks += metadata.blocks();
        }
    }

    Ok(total_blocks)
}

fn display_files(dir: &str, file_names: &[String], options: &LsOptions) -> io::Result<()> {
    let terminal_width = get_terminal_width();
    let mut line_length = 0;

    if options.show_details {
        let total_blocks = calculate_total_blocks(dir, file_names)?;
        println!("total {}", total_blocks / 2);
    }

    for file_name in file_names {
        if options.show_details {
            display_details(dir, file_name)?;
        }else{
            print_file_name(dir, file_name, options.append_slash)?;
        }


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

fn display_details(dir: &str, file_name: &str) -> io::Result<()> {
    let path = Path::new(dir).join(file_name);

    if let Ok(metadata) = fs::metadata(&path) {
        let permissions = metadata.permissions();
        let nlink = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();
        let size = metadata.len();
        // let blocks = metadata.blocks();
        let modified = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let datetime: chrono::DateTime<chrono::Local> = (UNIX_EPOCH + modified).into();

        let permissions_str = display_permissions(permissions, metadata.is_dir());
        let user = users::get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or(uid.to_string());
        let group = users::get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or(gid.to_string());

        println!(
            "{} {:>2} {:<8} {:<8} {:>6} {} {}",
            permissions_str,
            nlink,
            user,
            group,
            size,
            datetime.format("%b %d %H:%M"),
            file_name
        );
    }

    Ok(())
}

fn display_permissions(permissions: fs::Permissions, is_dir: bool) -> String {
    let mode = permissions.mode();
    let mut result = String::new();

    result.push(if is_dir { 'd' } else { '-' });
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    result  
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


// =========================================================================

// ls -l
// total 48
// -rw-r--r-- 1 student student 19298 Dec  9 14:49 Cargo.lock
// -rw-r--r-- 1 student student   211 Dec  9 14:49 Cargo.toml
// drwxrwxr-x 5 student student  4096 Dec  9 14:40 Docs
// -rw-rw-r-- 1 student student   263 Dec  9 14:40 makefile
// -rw-rw-r-- 1 student student  2927 Dec  9 14:40 README.md
// drwxrwxr-x 6 student student  4096 Dec  9 14:49 src
// drwxr-xr-x 3 student student  4096 Dec  9 14:47 target
// drwxrwxr-x 2 student student  4096 Dec  9 14:49 Task

