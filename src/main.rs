use shell::core::shell::Shell;

use anyhow::Result;

fn main() -> Result<()> {
    let mut shell = Shell::new();
 
    shell.run()?;

    Ok(())
}