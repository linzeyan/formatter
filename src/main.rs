mod formats;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use clap::{ArgAction, Parser};
use ignore::{DirEntry, WalkBuilder};
use rayon::prelude::*;

use crate::formats::{FormatKind, detect_kind, detect_kind_from_label, format_dispatch};

#[derive(Parser, Debug)]
#[command(author, version, about = "Multi-language formatter (pure Rust)")]
struct Cli {
    /// Files or directories to format
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Mirror output under this directory instead of overwriting
    #[arg(long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Do not write anything; just show what would change
    #[arg(long, action = ArgAction::SetTrue)]
    dry_run: bool,

    /// Check if files are formatted; exit with error if changes needed
    #[arg(long, action = ArgAction::SetTrue)]
    check: bool,

    /// Number of worker threads (default: CPU cores)
    #[arg(long, value_name = "N")]
    jobs: Option<usize>,

    /// Extra glob patterns to ignore (added on top of .gitignore/.dockerignore and defaults)
    #[arg(long, value_name = "GLOB")]
    ignore: Vec<String>,

    /// Only run for these kinds (comma separated, e.g. json,ts,go)
    #[arg(long, value_delimiter = ',', value_name = "KINDS")]
    only: Vec<String>,

    /// Skip these kinds (comma separated)
    #[arg(long, value_delimiter = ',', value_name = "KINDS")]
    skip: Vec<String>,

    /// Verbose logging
    #[arg(long, short, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug)]
enum Outcome {
    Formatted,
    Unchanged,
    SkippedUnsupported,
    Error,
}

#[derive(Debug)]
struct FileJob {
    path: PathBuf,
    root: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let thread_count = cli.jobs.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    });

    let wanted_only = to_kind_set(&cli.only);
    let skip = to_kind_set(&cli.skip);

    let files = collect_files(&cli, &wanted_only, &skip).context("collecting files to format")?;

    if files.is_empty() {
        println!("No files matched.");
        return Ok(());
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()?;

    let total = files.len();
    let output_root = cli
        .output
        .as_ref()
        .map(|p| fs::canonicalize(p).unwrap_or(p.clone()));

    let (formatted, unchanged, skipped, failed, to_fix) = pool.install(|| {
        files
            .par_iter()
            .map(|job| match process_file(job, &output_root, &cli) {
                Outcome::Formatted => (1, 0, 0, 0, 1),
                Outcome::Unchanged => (0, 1, 0, 0, 0),
                Outcome::SkippedUnsupported => (0, 0, 1, 0, 0),
                Outcome::Error => (0, 0, 0, 1, 0),
            })
            .reduce(
                || (0, 0, 0, 0, 0),
                |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4),
            )
    });

    println!(
        "Processed {} file(s): formatted {}, unchanged {}, skipped {}, errors {}",
        total, formatted, unchanged, skipped, failed
    );

    if cli.check && to_fix > 0 {
        return Err(anyhow!("{} file(s) would be reformatted", to_fix));
    }
    if failed > 0 {
        return Err(anyhow!("one or more files failed to format"));
    }
    Ok(())
}

fn to_kind_set(list: &[String]) -> HashSet<FormatKind> {
    list.iter()
        .filter_map(|s| detect_kind_from_label(s))
        .collect()
}

fn collect_files(
    cli: &Cli,
    only: &HashSet<FormatKind>,
    skip: &HashSet<FormatKind>,
) -> Result<Vec<FileJob>> {
    let mut jobs = Vec::new();
    let default_ignores = vec![
        ".git",
        "node_modules",
        "vendor",
        "target",
        "dist",
        ".cache",
        ".idea",
        ".vscode",
        ".DS_Store",
    ];

    for input in &cli.paths {
        let canonical_root = fs::canonicalize(input).unwrap_or(input.clone());
        if canonical_root.is_file() {
            if should_take(&canonical_root, only, skip) {
                let root = canonical_root
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."));
                jobs.push(FileJob {
                    path: canonical_root,
                    root,
                });
            }
            continue;
        }

        let mut builder = WalkBuilder::new(&canonical_root);
        builder
            .git_ignore(true)
            .git_global(false)
            .git_exclude(false)
            .ignore(false) // do not use .ignore files
            .follow_links(false)
            .parents(true)
            .hidden(false)
            .add_custom_ignore_filename(".dockerignore");

        let mut overrides = ignore::overrides::OverrideBuilder::new(&canonical_root);
        for pat in &default_ignores {
            let _ = overrides.add(&format!("**/{}", pat));
        }
        for pat in &cli.ignore {
            let _ = overrides.add(pat);
        }
        let overrides = overrides.build()?;
        builder.overrides(overrides);

        for entry in builder.build() {
            let entry: DirEntry = match entry {
                Ok(e) => e,
                Err(err) => {
                    if cli.verbose {
                        eprintln!("walk error: {err}");
                    }
                    continue;
                }
            };
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                continue;
            }
            let path = entry.into_path();
            if !should_take(&path, only, skip) {
                continue;
            }
            jobs.push(FileJob {
                path,
                root: canonical_root.clone(),
            });
        }
    }
    Ok(jobs)
}

fn should_take(path: &Path, only: &HashSet<FormatKind>, skip: &HashSet<FormatKind>) -> bool {
    match detect_kind(path) {
        Some(kind) => {
            if skip.contains(&kind) {
                return false;
            }
            if !only.is_empty() && !only.contains(&kind) {
                return false;
            }
            true
        }
        None => true, // we'll mark as unsupported later
    }
}

fn process_file(job: &FileJob, output_root: &Option<PathBuf>, cli: &Cli) -> Outcome {
    let kind = match detect_kind(&job.path) {
        Some(k) => k,
        None => {
            if cli.verbose {
                eprintln!("Skip unsupported: {}", job.path.display());
            }
            return Outcome::SkippedUnsupported;
        }
    };

    let content = match fs::read_to_string(&job.path) {
        Ok(c) => c,
        Err(_err) => return Outcome::Error,
    };

    let relative = job
        .path
        .strip_prefix(&job.root)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            job.path
                .file_name()
                .map(PathBuf::from)
                .unwrap_or_else(|| job.path.clone())
        });

    let target_path = output_root
        .as_ref()
        .map(|root| root.join(&relative))
        .unwrap_or_else(|| job.path.clone());

    let formatted = match format_dispatch(kind, &job.path, &content) {
        Ok(Some(new_text)) => new_text,
        Ok(None) => {
            if output_root.is_some() {
                if cli.verbose {
                    println!("Copy unchanged {}", job.path.display());
                }
                if !cli.check && !cli.dry_run {
                    if let Some(parent) = target_path.parent() {
                        if let Err(_err) = fs::create_dir_all(parent) {
                            return Outcome::Error;
                        }
                    }
                    if let Err(_err) = fs::write(&target_path, content) {
                        return Outcome::Error;
                    }
                }
            }
            return Outcome::Unchanged;
        }
        Err(_err) => return Outcome::Error,
    };

    if cli.check || cli.dry_run {
        if cli.verbose {
            println!("Would format {}", job.path.display());
        }
        return Outcome::Formatted;
    }

    if let Some(parent) = target_path.parent() {
        if let Err(_err) = fs::create_dir_all(parent) {
            return Outcome::Error;
        }
    }
    match fs::write(&target_path, formatted) {
        Ok(_) => Outcome::Formatted,
        Err(_) => Outcome::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn mirror_output_path() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("src");
        let file = root.join("a/b/test.json");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "{ \"a\":1 }").unwrap();
        assert!(file.exists());

        let cli = Cli {
            paths: vec![root.clone()],
            output: Some(tmp.path().join("out")),
            dry_run: false,
            check: false,
            jobs: Some(1),
            ignore: vec![],
            only: vec![],
            skip: vec![],
            verbose: false,
        };
        let out_root = cli.output.clone();
        let job = FileJob {
            path: file.clone(),
            root: root.clone(),
        };
        let outcome = process_file(&job, &out_root, &cli);
        assert!(
            matches!(outcome, Outcome::Formatted | Outcome::Unchanged),
            "{outcome:?}"
        );

        let mirrored = tmp.path().join("out").join("a/b/test.json");
        assert!(mirrored.exists());
        let content = fs::read_to_string(mirrored).unwrap();
        assert!(content.contains("\"a\": 1"));
    }
}
