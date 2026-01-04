mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{IsTerminal, stdout};
    let prompt = if stdout().is_terminal() {
        "\x1b[33m>\x1b[0m "
    } else {
        "> "
    };

    let mut cli = cli::Cli::new(prompt)?;
    let printer = cli.get_printer();

    // --------------  Example: Simple command processing ----------------------------------------------
    while let Some((cmd, args)) = cli.readline() {
        match cmd.as_str() {
            "echo" => {
                printer.println(args.join(" "));
            }
            "upper" => {
                printer.println(args.join(" ").to_uppercase());
            }
            "lower" => {
                printer.println(args.join(" ").to_lowercase());
            }
            "reverse" => {
                let reversed: String = args.join(" ").chars().rev().collect();
                printer.println(reversed);
            }
            _ => {
                printer.warnln(format!("Unknown command: {}", cmd));
            }
        }
    }

    Ok(())
}
