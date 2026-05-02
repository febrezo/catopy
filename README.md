# clipcat

clipcat is a small Rust CLI that copies file contents to the system clipboard with safety limits.

## Features

- Copy full file content to clipboard.
- Copy only first N lines (`--head`) or last N lines (`--tail`).
- Size guard with override (`--max-bytes`, `--force`).
- Optional ANSI color output, with `--no-color` and `NO_COLOR` support.
- Native clipboard backends via `arboard`.

## Install

### From crates.io

```bash
cargo install clipcat
```

### From GitHub Release binaries

1. Open the latest Release in GitHub.
2. Download the archive for your target.
3. Verify checksum:

```bash
sha256sum -c clipcat-<version>-<target>.tar.gz.sha256
```

4. Extract and place `clipcat` in your `PATH`.

## Quick Start

```bash
clipcat file.txt
clipcat --head 50 logs.txt
clipcat --tail 100 logs.txt
clipcat --max-bytes 5M notes.md
clipcat --force big_dump.txt
```

## Usage

```text
clipcat [OPTIONS] <FILE>
```

Main options:

- `--head <N>`: copy only first N lines.
- `--tail <N>`: copy only last N lines.
- `--max-bytes <SIZE>`: set size guard (for example `1M`, `500K`).
- `--force`: ignore size guard and copy anyway.
- `--no-color`: disable ANSI colors.

Common real-world examples:

```bash
clipcat README.md
clipcat --head 10 /var/log/messages
clipcat --tail 20 app.log
```

## Configuration file

Optional user config path:

- `~/.clipcat.rc`

Example:

```ini
behave_as_cat=true
warning_size_limit=1000000
default_head=0
default_tail=0
```

Precedence:

1. CLI arguments
2. `~/.clipcat.rc`
3. Internal defaults

## Man page and shell completions

Static assets included in repository:

- `man/clipcat.1`
- `completions/clipcat.bash`
- `completions/clipcat.zsh`
- `completions/clipcat.fish`

## Development

### Build locally

```bash
cargo build
cargo build --release
```

### Test and quality checks

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features
```

## Packaging status

### GitHub Releases

- Workflow: `.github/workflows/release.yml`
- Trigger: tags like `v0.1.0`
- Builds release archives and SHA256 checksums.

### crates.io

```bash
cargo login
cargo package
cargo publish --dry-run
cargo publish
```

### Fedora COPR (planned)

- RPM spec skeleton: `packaging/rpm/clipcat.spec`
- Initial validation:

```bash
sudo dnf install rpmdevtools rpmlint mock fedpkg
rpmlint packaging/rpm/clipcat.spec
```

### Debian/Ubuntu `.deb`

- Simple `.deb` via cargo-deb:

```bash
cargo install cargo-deb
cargo deb
```

- Debian packaging skeleton:
  - `packaging/debian/control`
  - `packaging/debian/rules`
  - `packaging/debian/changelog`
  - `packaging/debian/copyright`

## Cross-compilation targets

Validated release builds with `cross`:

- x86_64-unknown-linux-gnu
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu
- aarch64-unknown-linux-musl
- armv7-unknown-linux-gnueabihf
- x86_64-pc-windows-gnu

## Packaging docs

See `PACKAGING.md` for the end-to-end distribution route and repository workflow.
