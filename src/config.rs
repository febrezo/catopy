use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub behave_as_cat: bool,
    pub warning_size_limit: u64,
    pub default_head: usize,
    pub default_tail: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            behave_as_cat: false,
            warning_size_limit: 1_000_000,
            default_head: 0,
            default_tail: 0,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let Some(path) = rc_paths().into_iter().find(|path| path.exists()) else {
            return Ok(Self::default());
        };

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

        parse_config(&raw)
    }
}

fn parse_config(raw: &str) -> Result<Config> {
    let mut cfg = Config::default();
    for (idx, line) in raw.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let (key, value) = trimmed
            .split_once('=')
            .ok_or_else(|| anyhow!("invalid config entry at line {}", line_no))?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "behave_as_cat" => {
                cfg.behave_as_cat = parse_bool(value)
                    .ok_or_else(|| anyhow!("invalid bool for behave_as_cat at line {}", line_no))?;
            }
            "warning_size_limit" => {
                cfg.warning_size_limit = parse_size(value).ok_or_else(|| {
                    anyhow!("invalid size for warning_size_limit at line {}", line_no)
                })?;
            }
            "default_head" => {
                cfg.default_head = value.parse::<usize>().with_context(|| {
                    format!("invalid usize for default_head at line {}", line_no)
                })?;
            }
            "default_tail" => {
                cfg.default_tail = value.parse::<usize>().with_context(|| {
                    format!("invalid usize for default_tail at line {}", line_no)
                })?;
            }
            _ => {
                return Err(anyhow!("unknown config key '{}' at line {}", key, line_no));
            }
        }
    }

    Ok(cfg)
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

pub fn parse_size(value: &str) -> Option<u64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let upper = trimmed.to_ascii_uppercase();
    let (digits, mult) = match upper.chars().last() {
        Some('K') => (&upper[..upper.len() - 1], 1_000_u64),
        Some('M') => (&upper[..upper.len() - 1], 1_000_000_u64),
        Some('G') => (&upper[..upper.len() - 1], 1_000_000_000_u64),
        _ => (upper.as_str(), 1_u64),
    };

    if digits.is_empty() {
        return None;
    }

    let base = digits.parse::<u64>().ok()?;
    base.checked_mul(mult)
}

fn rc_paths() -> Vec<PathBuf> {
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    vec![home.join(".catopy.rc"), home.join(".clipcat.rc")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_size_plain() {
        assert_eq!(parse_size("1000"), Some(1000));
    }

    #[test]
    fn parse_size_with_suffix() {
        assert_eq!(parse_size("1K"), Some(1_000));
        assert_eq!(parse_size("5m"), Some(5_000_000));
        assert_eq!(parse_size("2G"), Some(2_000_000_000));
    }

    #[test]
    fn parse_size_invalid() {
        assert_eq!(parse_size(""), None);
        assert_eq!(parse_size("abc"), None);
        assert_eq!(parse_size("K"), None);
    }

    #[test]
    fn parse_config_from_text() {
        let text = "behave_as_cat=true\nwarning_size_limit=2M\ndefault_head=5\ndefault_tail=0\n";
        let cfg = parse_config(text).expect("config parses");
        assert_eq!(
            cfg,
            Config {
                behave_as_cat: true,
                warning_size_limit: 2_000_000,
                default_head: 5,
                default_tail: 0,
            }
        );
    }
}
