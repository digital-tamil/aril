use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use crate::cli::Cli;
use crate::diff::DiffViewer;
use crate::pipeline::Pipeline;
use crate::safety::SafetyEngine;

#[derive(Serialize, Debug)]
pub(crate) struct BatchSummary {
    pub files_scanned: usize,
    pub files_converted: usize,
    pub files_skipped_binary: usize,
    pub bytes_processed: usize,
    pub duration_ms: u128,
    pub throughput_mb_per_sec: f64,
}

#[derive(Debug)]
pub(crate) struct BatchProcessor;

impl BatchProcessor {
    pub(crate) fn run(cli: &Cli) -> Result<()> {
        let start = Instant::now();
        let files = collect_files(&cli.files, cli.recursive, &cli.extensions);

        if files.is_empty() {
            bail!("No matching files found to process.");
        }

        // Single file to stdout / output file
        if files.len() == 1 && !cli.in_place {
            return process_single_file(&files[0], cli);
        }

        if !cli.in_place && !cli.dry_run {
            bail!(
                "Multiple files detected. Specify `--in-place` (`-i`) to modify or `--dry-run` to preview."
            );
        }

        let scanned_count = files.len();
        let converted_count = AtomicUsize::new(0);
        let skipped_count = AtomicUsize::new(0);
        let bytes_processed = AtomicUsize::new(0);

        files.par_iter().try_for_each(|file_path| -> Result<()> {
            if SafetyEngine::is_binary(file_path)? {
                skipped_count.fetch_add(1, Ordering::Relaxed);
                if cli.verbose {
                    eprintln!("\x1b[90m[Skip Binary]\x1b[0m {}", file_path.display());
                }
                return Ok(());
            }

            let content = fs::read_to_string(file_path)
                .with_context(|| format!("Reading failed for {}", file_path.display()))?;

            bytes_processed.fetch_add(content.len(), Ordering::Relaxed);
            let result = Pipeline::execute(&content, cli.from, cli.to)?;

            if cli.diff {
                DiffViewer::render_preview(
                    &content,
                    &result.output,
                    Some(&file_path.to_string_lossy()),
                );
            }

            if !cli.dry_run {
                SafetyEngine::atomic_write(file_path, &result.output, cli.backup)?;
            }

            converted_count.fetch_add(1, Ordering::Relaxed);
            if cli.verbose {
                eprintln!("\x1b[32m[Converted]\x1b[0m {}", file_path.display());
            }

            Ok(())
        })?;

        let duration = start.elapsed();
        let total_bytes = bytes_processed.load(Ordering::Relaxed);
        let total_converted = converted_count.load(Ordering::Relaxed);
        let total_skipped = skipped_count.load(Ordering::Relaxed);

        let throughput = if duration.as_secs_f64() > 0.0 {
            (total_bytes as f64 / 1_048_576.0) / duration.as_secs_f64()
        } else {
            0.0
        };

        let summary = BatchSummary {
            files_scanned: scanned_count,
            files_converted: total_converted,
            files_skipped_binary: total_skipped,
            bytes_processed: total_bytes,
            duration_ms: duration.as_millis(),
            throughput_mb_per_sec: throughput,
        };

        if cli.json {
            println!("{}", serde_json::to_string_pretty(&summary)?);
        } else if cli.stats || cli.verbose {
            println!("\n\x1b[1;36m=== Batch Conversion Summary ===\x1b[0m");
            println!(
                "  Files converted:       \x1b[1;32m{}\x1b[0m",
                summary.files_converted
            );
            println!(
                "  Files skipped binary:  \x1b[1;33m{}\x1b[0m",
                summary.files_skipped_binary
            );
            println!("  Total bytes processed: {} bytes", summary.bytes_processed);
            println!("  Execution time:        {duration:.2?}");
            println!(
                "  Throughput:            \x1b[1m{:.2} MB/s\x1b[0m",
                summary.throughput_mb_per_sec
            );
        }

        Ok(())
    }
}

fn process_single_file(path: &Path, cli: &Cli) -> Result<()> {
    if SafetyEngine::is_binary(path)? {
        bail!("File appears to be binary: {}", path.display());
    }

    let content = fs::read_to_string(path)?;
    let result = Pipeline::execute(&content, cli.from, cli.to)?;

    if cli.diff {
        DiffViewer::render_preview(&content, &result.output, Some(&path.to_string_lossy()));
        return Ok(());
    }

    if let Some(out_path) = &cli.output {
        if !cli.dry_run {
            fs::write(out_path, result.output)?;
        }
    } else {
        print!("{}", result.output);
    }
    Ok(())
}

fn collect_files(paths: &[PathBuf], recursive: bool, extensions: &str) -> Vec<PathBuf> {
    let ext_set: HashSet<String> = extensions
        .split(',')
        .map(|s| s.trim().trim_start_matches('.').to_lowercase())
        .collect();

    let mut collected = Vec::new();
    for path in paths {
        if path.is_file() {
            collected.push(path.clone());
        } else if path.is_dir() && recursive {
            walk_dir(path, &ext_set, &mut collected);
        }
    }
    collected
}

fn walk_dir(dir: &Path, extensions: &HashSet<String>, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, extensions, out);
            } else if path.is_file()
                && let Some(ext) = path.extension().and_then(|s| s.to_str())
                && extensions.contains(&ext.to_lowercase())
            {
                out.push(path);
            }
        }
    }
}
