use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::IsTerminal;

/// Exit command.
const EXIT_COMMANDS: &[&str] = &["exit", "quit", "q"];

pub struct Cli {
    rl: DefaultEditor,
    printer: Printer,
    prompt: String,
}

#[derive(Clone)]
pub struct Printer {
    is_terminal: bool,
}

pub struct ExtPrinter {
    is_terminal: bool,
    printer: Box<dyn rustyline::ExternalPrinter + Send>,
}

#[allow(dead_code)]
impl Cli {
    pub fn new(prompt: impl ToString) -> Result<Self, Box<dyn std::error::Error>> {
        let rl = DefaultEditor::new()?;
        let printer = Printer {
            is_terminal: std::io::stdout().is_terminal(),
        };

        Ok(Self {
            rl,
            printer,
            prompt: prompt.to_string(),
        })
    }

    pub fn readline(&mut self) -> Option<(String, Vec<String>)> {
        loop {
            let readline = self.rl.readline(&self.prompt);

            let mut args = match readline {
                Ok(ref line) => match shell_words::split(line) {
                    Ok(x) => x,
                    Err(e) => {
                        self.printer.errln(format!("[Input Error] {e}"));
                        continue;
                    }
                },
                Err(ReadlineError::Eof) => return None,
                Err(ReadlineError::Interrupted) => return None,
                Err(e) => {
                    self.printer.errln(format!("[Input Error] {e}"));
                    continue;
                }
            };

            if args.is_empty() {
                continue;
            }

            let cmd = args.remove(0);

            // Exit commands
            if EXIT_COMMANDS.contains(&cmd.as_str()) {
                return None;
            }

            return Some((cmd, args));
        }
    }

    pub fn get_printer(&self) -> Printer {
        self.printer.clone()
    }

    pub fn create_external_printer(&mut self) -> Result<ExtPrinter, ReadlineError> {
        let printer = Box::new(self.rl.create_external_printer()?);
        Ok(ExtPrinter {
            is_terminal: self.printer.is_terminal,
            printer,
        })
    }
}

#[allow(dead_code)]
impl Printer {
    #[inline]
    pub fn println(&self, msg: impl AsRef<str>) {
        println!("{}", msg.as_ref());
    }

    #[inline]
    pub fn print(&self, msg: impl AsRef<str>) {
        print!("{}", msg.as_ref());
    }

    #[inline]
    pub fn errln(&self, msg: impl AsRef<str>) {
        if self.is_terminal {
            eprintln!("\x1b[31m{}\x1b[0m", msg.as_ref());
        } else {
            eprintln!("{}", msg.as_ref());
        }
    }

    #[inline]
    pub fn err(&self, msg: impl AsRef<str>) {
        if self.is_terminal {
            eprint!("\x1b[31m{}\x1b[0m", msg.as_ref());
        } else {
            eprint!("{}", msg.as_ref());
        }
    }

    #[inline]
    pub fn warnln(&self, msg: impl AsRef<str>) {
        if self.is_terminal {
            eprintln!("\x1b[33m{}\x1b[0m", msg.as_ref());
        } else {
            eprintln!("{}", msg.as_ref());
        }
    }

    #[inline]
    pub fn warn(&self, msg: impl AsRef<str>) {
        if self.is_terminal {
            eprint!("\x1b[33m{}\x1b[0m", msg.as_ref());
        } else {
            eprint!("{}", msg.as_ref());
        }
    }
}

#[allow(dead_code)]
impl ExtPrinter {
    #[inline]
    pub fn println(&mut self, msg: impl AsRef<str>) {
        let _ = self.printer.print(format!("{}\n", msg.as_ref()));
    }

    #[inline]
    pub fn print(&mut self, msg: impl AsRef<str>) {
        let _ = self.printer.print(msg.as_ref().to_string());
    }

    #[inline]
    pub fn errln(&mut self, msg: impl AsRef<str>) {
        if self.is_terminal {
            let _ = self.printer.print(format!("\x1b[31m{}\x1b[0m\n", msg.as_ref()));
        } else {
            let _ = self.printer.print(format!("{}\n", msg.as_ref()));
        }
    }

    #[inline]
    pub fn err(&mut self, msg: impl AsRef<str>) {
        if self.is_terminal {
            let _ = self.printer.print(format!("\x1b[31m{}\x1b[0m", msg.as_ref()));
        } else {
            let _ = self.printer.print(format!("{}", msg.as_ref()));
        }
    }

    #[inline]
    pub fn warnln(&mut self, msg: impl AsRef<str>) {
        if self.is_terminal {
            let _ = self.printer.print(format!("\x1b[33m{}\x1b[0m\n", msg.as_ref()));
        } else {
            let _ = self.printer.print(format!("{}\n", msg.as_ref()));
        }
    }

    #[inline]
    pub fn warn(&mut self, msg: impl AsRef<str>) {
        if self.is_terminal {
            let _ = self.printer.print(format!("\x1b[33m{}\x1b[0m", msg.as_ref()));
        } else {
            let _ = self.printer.print(format!("{}", msg.as_ref()));
        }
    }
}
