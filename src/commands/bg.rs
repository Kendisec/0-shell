use std::process::Command;
use std::io::{self};

pub fn send_to_background(pid: &str) -> io::Result<()> {
    // Envoyer le signal SIGCONT au processus pour le reprendre
    let output = Command::new("kill")
        .arg("-CONT")
        .arg(pid)
        .output()?;

    if output.status.success() {
        println!("Process {} sent to background.", pid);
    } else {
        eprintln!("Failed to send process {} to background.", pid);
        std::process::exit(1);
    }

    Ok(())
}


pub fn bring_to_foreground(pid: &str) -> io::Result<()> {
    // Envoyer le signal SIGCONT au processus pour le reprendre
    let output = Command::new("kill")
        .arg("-CONT")
        .arg(pid)
        .output()?;

    if output.status.success() {
        println!("Process {} brought to foreground.", pid);
    } else {
        eprintln!("Failed to bring process {} to foreground.", pid);
        std::process::exit(1);
    }

    Ok(())
}