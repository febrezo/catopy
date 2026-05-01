use std::io::Write;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn help_includes_no_color_flag() {
    Command::cargo_bin("clipcat")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--no-color"));
}

#[test]
fn large_file_prints_two_line_warning() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "line1").expect("write");
    writeln!(file, "line2").expect("write");

    Command::cargo_bin("clipcat")
        .expect("binary exists")
        .arg(file.path())
        .arg("--max-bytes")
        .arg("1")
        .arg("--no-color")
        .assert()
        .failure()
        .stderr(predicate::str::contains("warning: file is"))
        .stderr(predicate::str::contains(
            "use --head N, --tail N, or --force",
        ));
}

#[test]
fn head_and_tail_conflict_returns_error() {
    Command::cargo_bin("clipcat")
        .expect("binary exists")
        .arg("file.txt")
        .arg("--head")
        .arg("1")
        .arg("--tail")
        .arg("1")
        .arg("--no-color")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: --head and --tail cannot be used together",
        ));
}

#[test]
fn missing_file_returns_read_error() {
    Command::cargo_bin("clipcat")
        .expect("binary exists")
        .arg("definitely_missing_file.txt")
        .arg("--no-color")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: cannot read"));
}
