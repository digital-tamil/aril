use crate::encoding::CliEncoding;
use crate::pipeline::Pipeline;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
pub struct Repl;

impl Repl {
    pub(crate) fn run() -> anyhow::Result<()> {
        println!("\x1b[1;36m┌───────────────────────────────────────────────┐\x1b[0m");
        println!("\x1b[1;36m│       aril interactive conversion REPL        │\x1b[0m");
        println!("\x1b[1;36m│  Paste legacy text. Type 'exit' to quit.      │\x1b[0m");
        println!("\x1b[1;36m└───────────────────────────────────────────────┘\x1b[0m\n");

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut handle = stdin.lock();

        loop {
            print!("\x1b[1;33maril > \x1b[0m");
            stdout.flush()?;

            let mut line = String::new();
            if handle.read_line(&mut line)? == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
                println!("Goodbye!");
                break;
            }

            match Pipeline::execute(trimmed, CliEncoding::Auto, CliEncoding::Unicode) {
                Ok(result) => {
                    let enc = result
                        .source_detected.map_or_else(|| "Unknown".to_string(), |e| format!("{e:?}"));
                    println!("\x1b[90m[Detected: {enc}]\x1b[0m");
                    println!("\x1b[1;32m=> {}\x1b[0m\n", result.output);
                }
                Err(err) => {
                    println!("\x1b[1;31mError: {err}\x1b[0m\n");
                }
            }
        }

        Ok(())
    }
}
