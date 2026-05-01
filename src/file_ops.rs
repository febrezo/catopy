use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Result, bail};

pub fn file_size(path: &Path) -> Result<u64> {
    let meta = fs::metadata(path)?;
    Ok(meta.len())
}

pub fn read_full(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn read_head(path: &Path, lines: usize) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut out = Vec::new();
    let mut buf = Vec::new();

    for _ in 0..lines {
        buf.clear();
        let read = reader.read_until(b'\n', &mut buf)?;
        if read == 0 {
            break;
        }
        out.extend_from_slice(&buf);
    }

    Ok(String::from_utf8_lossy(&out).into_owned())
}

pub fn read_tail(path: &Path, lines: usize) -> Result<String> {
    if lines == 0 {
        return Ok(String::new());
    }

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut ring: VecDeque<Vec<u8>> = VecDeque::with_capacity(lines);
    let mut buf = Vec::new();

    loop {
        buf.clear();
        let read = reader.read_until(b'\n', &mut buf)?;
        if read == 0 {
            break;
        }
        if ring.len() == lines {
            ring.pop_front();
        }
        ring.push_back(buf.clone());
    }

    let mut out = Vec::new();
    for chunk in ring {
        out.extend_from_slice(&chunk);
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

pub fn read_with_slice(path: &Path, head: usize, tail: usize) -> Result<String> {
    match (head, tail) {
        (0, 0) => read_full(path),
        (h, 0) => read_head(path, h),
        (0, t) => read_tail(path, t),
        _ => bail!("--head and --tail cannot be used together"),
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        return format_decimal(bytes as f64 / 1_000_000_000.0, "GB");
    }
    if bytes >= 1_000_000 {
        return format_decimal(bytes as f64 / 1_000_000.0, "MB");
    }
    if bytes >= 1_000 {
        return format_decimal(bytes as f64 / 1_000.0, "KB");
    }
    format!("{} B", bytes)
}

fn format_decimal(value: f64, unit: &str) -> String {
    let fixed = format!("{:.1}", value);
    if let Some(stripped) = fixed.strip_suffix(".0") {
        return format!("{} {}", stripped, unit);
    }
    format!("{} {}", fixed, unit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn head_reads_first_lines() {
        let mut file = NamedTempFile::new().expect("tempfile");
        writeln!(file, "line1").expect("write");
        writeln!(file, "line2").expect("write");
        writeln!(file, "line3").expect("write");

        let got = read_head(file.path(), 2).expect("head works");
        assert_eq!(got, "line1\nline2\n");
    }

    #[test]
    fn tail_reads_last_lines() {
        let mut file = NamedTempFile::new().expect("tempfile");
        writeln!(file, "line1").expect("write");
        writeln!(file, "line2").expect("write");
        writeln!(file, "line3").expect("write");

        let got = read_tail(file.path(), 2).expect("tail works");
        assert_eq!(got, "line2\nline3\n");
    }

    #[test]
    fn format_size_human_readable() {
        assert_eq!(format_size(1_500), "1.5 KB");
        assert_eq!(format_size(4_800_000), "4.8 MB");
        assert_eq!(format_size(1_000_000), "1 MB");
    }
}
