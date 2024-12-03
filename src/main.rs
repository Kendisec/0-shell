use shell::core::shell::Shell;


fn main() -> anyhow::Result<()> {
    let mut shell = Shell::new();
    shell.run()
}