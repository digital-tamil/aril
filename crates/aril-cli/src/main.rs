#![allow(clippy::redundant_pub_crate)]
mod batch;
mod cli;
mod diff;
mod encoding;
mod pipeline;
mod repl;
mod safety;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use serde_json::json;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};

use batch::BatchProcessor;
use cli::{Cli, Commands};
use diff::DiffViewer;
use encoding::print_encodings_catalog;
use pipeline::Pipeline;
use repl::Repl;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(subcmd) = &cli.command {
        match subcmd {
            Commands::Repl => return Repl::run(),
            Commands::List => {
                print_encodings_catalog();
                return Ok(());
            }
            Commands::Completions { shell } => {
                let mut cmd = Cli::command();
                clap_complete::generate(*shell, &mut cmd, "aril", &mut io::stdout());
                return Ok(());
            }
            Commands::Detect { files, json } => {
                return run_detect_subcommand(files, *json);
            }
        }
    }

    if let Some(text) = &cli.text {
        let result = Pipeline::execute(text, cli.from, cli.to)?;
        if cli.diff {
            DiffViewer::render_preview(text, &result.output, None);
            return Ok(());
        }
        if let Some(out_path) = &cli.output {
            if !cli.dry_run {
                fs::write(out_path, result.output)?;
            }
        } else {
            print!("{}", result.output);
        }
        return Ok(());
    }

    if cli.files.is_empty() && !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Reading stdin failed")?;
        let result = Pipeline::execute(&buffer, cli.from, cli.to)?;

        if cli.diff {
            DiffViewer::render_preview(&buffer, &result.output, None);
        } else if let Some(out_path) = &cli.output {
            if !cli.dry_run {
                fs::write(out_path, result.output)?;
            }
        } else {
            io::stdout().write_all(result.output.as_bytes())?;
        }
        return Ok(());
    }

    BatchProcessor::run(&cli)
}

fn run_detect_subcommand(files: &[std::path::PathBuf], json_out: bool) -> Result<()> {
    if files.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let detected = Pipeline::detect(&buffer);
        if json_out {
            println!(
                "{}",
                json!({ "source": "stdin", "detected": detected.map(|e| format!("{e:?}")) })
            );
        } else {
            println!(
                "stdin: {}",
                detected.map_or_else(|| "Unknown".into(), |e| format!("{e:?}"))
            );
        }
        return Ok(());
    }

    for file in files {
        let content = fs::read_to_string(file)?;
        let detected = Pipeline::detect(&content);
        if json_out {
            println!(
                "{}",
                json!({ "file": file, "detected": detected.map(|e| format!("{e:?}") ) })
            );
        } else {
            println!(
                "{}: {}",
                file.display(),
                detected.map_or_else(|| "Unknown".into(), |e| format!("{e:?}"))
            );
        }
    }
    Ok(())
}
