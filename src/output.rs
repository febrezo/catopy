use std::env;
use std::io;
use std::io::IsTerminal;
use std::path::Path;

use crate::file_ops::format_size;

const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone)]
pub struct Output {
    use_color: bool,
}

impl Output {
    pub fn new(no_color_flag: bool) -> Self {
        let no_color_env = env::var_os("NO_COLOR").is_some();
        let stdout_is_tty = io::stdout().is_terminal();
        Self {
            use_color: !no_color_flag && !no_color_env && stdout_is_tty,
        }
    }

    pub fn success_copy(&self, path: &Path, head: usize, tail: usize, content: &str) -> String {
        let mark = self.success_marker();
        if head > 0 {
            return format!(
                "{} copied first {} lines from {}",
                mark,
                head,
                path.display()
            );
        }
        if tail > 0 {
            return format!(
                "{} copied last {} lines from {}",
                mark,
                tail,
                path.display()
            );
        }
        format!(
            "{} copied {} to clipboard ({} lines, {})",
            mark,
            path.display(),
            line_count(content),
            format_size(content.len() as u64)
        )
    }

    pub fn warning_size_limit(&self, size: u64, limit: u64) -> String {
        format!(
            "{} file is {}; limit is {}\nuse --head N, --tail N, or --force",
            self.warning_prefix(),
            format_size(size),
            format_size(limit)
        )
    }

    pub fn warning_clipboard_unavailable(&self, reason: &str) -> String {
        format!(
            "{} clipboard unavailable: {}",
            self.warning_prefix(),
            reason
        )
    }

    pub fn warning_stdout_fallback(&self) -> String {
        format!(
            "{} clipboard unavailable; content printed to stdout",
            self.warning_prefix()
        )
    }

    fn success_marker(&self) -> String {
        if self.use_color {
            format!("{}✓{}", ANSI_GREEN, ANSI_RESET)
        } else {
            "✓".to_string()
        }
    }

    fn warning_prefix(&self) -> String {
        if self.use_color {
            format!("{}warning:{}", ANSI_YELLOW, ANSI_RESET)
        } else {
            "warning:".to_string()
        }
    }
}

fn line_count(content: &str) -> usize {
    if content.is_empty() {
        return 0;
    }
    content.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn success_format_for_full_copy() {
        let output = Output::new(true);
        let msg = output.success_copy(Path::new("file.txt"), 0, 0, "a\nb\n");
        assert_eq!(msg, "✓ copied file.txt to clipboard (2 lines, 4 B)");
    }

    #[test]
    fn success_format_for_head_copy() {
        let output = Output::new(true);
        let msg = output.success_copy(Path::new("file.txt"), 10, 0, "");
        assert_eq!(msg, "✓ copied first 10 lines from file.txt");
    }

    #[test]
    fn warning_limit_format() {
        let output = Output::new(true);
        let msg = output.warning_size_limit(4_800_000, 1_000_000);
        assert_eq!(
            msg,
            "warning: file is 4.8 MB; limit is 1 MB\nuse --head N, --tail N, or --force"
        );
    }
}
