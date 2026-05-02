mod config;
mod file_ops;
mod output;

use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail};
use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use clap::Parser;
use config::{Config, parse_size};
use file_ops::file_size;
use file_ops::read_with_slice;
use output::Output;

#[cfg(target_os = "linux")]
const LINUX_CLIPBOARD_WAIT_MS: u64 = 200;

#[derive(Debug, Parser)]
#[command(
    name = "catopy",
    version,
    about = "Copy file contents to clipboard safely"
)]
struct Cli {
    /// File to read.
    file: PathBuf,

    /// Copy only first N lines.
    #[arg(long)]
    head: Option<usize>,

    /// Copy only last N lines.
    #[arg(long)]
    tail: Option<usize>,

    /// Allow copying even if file exceeds max-bytes.
    #[arg(long)]
    force: bool,

    /// Max file size before refusing copy (supports 1K, 5M, 2G).
    #[arg(long, value_parser = parse_size_cli)]
    max_bytes: Option<u64>,

    /// Disable colored output.
    #[arg(long)]
    no_color: bool,
}

fn parse_size_cli(value: &str) -> Result<u64, String> {
    parse_size(value).ok_or_else(|| "invalid size; use N, NK, NM, or NG".to_string())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let output = Output::new(cli.no_color);
    let cfg = Config::load().context("error: could not load config")?;

    let head = cli.head.unwrap_or(cfg.default_head);
    let tail = cli.tail.unwrap_or(cfg.default_tail);
    if head > 0 && tail > 0 {
        bail!("error: --head and --tail cannot be used together");
    }

    let max_bytes = cli.max_bytes.unwrap_or(cfg.warning_size_limit);
    let size = file_size(&cli.file)
        .map_err(|err| anyhow!("error: cannot read {}: {}", cli.file.display(), err))?;

    if should_block_for_size(size, max_bytes, head, tail, cli.force) {
        bail!("{}", output.warning_size_limit(size, max_bytes));
    }

    let content = read_with_slice(&cli.file, head, tail)
        .map_err(|err| anyhow!("error: cannot read {}: {}", cli.file.display(), err))?;

    let behave_as_cat = cfg.behave_as_cat;
    if behave_as_cat {
        print!("{}", content);
    }

    match copy_to_clipboard(&content) {
        Ok(()) => {
            println!("{}", output.success_copy(&cli.file, head, tail, &content));
            Ok(())
        }
        Err(copy_err) => {
            eprintln!(
                "{}",
                output.warning_clipboard_unavailable(&copy_err.to_string())
            );
            if !behave_as_cat {
                // Fallback path when clipboard is unavailable in this environment.
                print!("{}", content);
                eprintln!("{}", output.warning_stdout_fallback());
            }
            Ok(())
        }
    }
}

fn should_block_for_size(size: u64, max_bytes: u64, head: usize, tail: usize, force: bool) -> bool {
    !force && head == 0 && tail == 0 && size > max_bytes
}

fn copy_to_clipboard(content: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("clipboard backend unavailable")?;

    #[cfg(target_os = "linux")]
    {
        let deadline = Instant::now() + Duration::from_millis(LINUX_CLIPBOARD_WAIT_MS);
        clipboard
            .set()
            .wait_until(deadline)
            .text(content.to_string())
            .context("failed to set clipboard text")
    }

    #[cfg(not(target_os = "linux"))]
    clipboard
        .set_text(content.to_string())
        .context("failed to set clipboard text")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_size_suffix_parser() {
        assert_eq!(parse_size_cli("1K").expect("1K parses"), 1_000);
        assert_eq!(parse_size_cli("5M").expect("5M parses"), 5_000_000);
    }

    #[test]
    fn limit_check_triggered() {
        assert!(should_block_for_size(4_800_000, 1_000_000, 0, 0, false));
        assert!(!should_block_for_size(4_800_000, 1_000_000, 10, 0, false));
        assert!(!should_block_for_size(4_800_000, 1_000_000, 0, 10, false));
        assert!(!should_block_for_size(4_800_000, 1_000_000, 0, 0, true));
    }
}
