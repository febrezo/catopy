# catopy

catopy is a small Rust CLI that copies file contents to the system clipboard with safety limits.

## Features

- Copy full file content to clipboard.
- Copy only first N lines (`--head`) or last N lines (`--tail`).
- Size guard with override (`--max-bytes`, `--force`).
- Optional ANSI color output, with `--no-color` and `NO_COLOR` support.
- Native clipboard backends via `arboard`.

## Install

### From crates.io

```bash
cargo install catopy
```

### From GitHub Release binaries

1. Open the latest Release in GitHub.
2. Download the archive for your target.
3. Verify checksum:

```bash
sha256sum -c catopy-<version>-<target>.tar.gz.sha256
```

4. Extract and place `catopy` in your `PATH`.

## Quick Start

```bash
catopy file.txt
catopy --head 50 logs.txt
catopy --tail 100 logs.txt
catopy --max-bytes 5M notes.md
catopy --force big_dump.txt
```

## Usage

```text
catopy [OPTIONS] <FILE>
```

Main options:

- `--head <N>`: copy only first N lines.
- `--tail <N>`: copy only last N lines.
- `--max-bytes <SIZE>`: set size guard (for example `1M`, `500K`).
- `--force`: ignore size guard and copy anyway.
- `--no-color`: disable ANSI colors.

Common real-world examples:

```bash
catopy README.md
catopy --head 10 /var/log/messages
catopy --tail 20 app.log
```

## Configuration file

Optional user config path:

- `~/.catopy.rc`

Legacy fallback still supported:

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
2. `~/.catopy.rc`
3. `~/.clipcat.rc`
4. Internal defaults

## Man page and shell completions

Static assets included in repository:

- `man/catopy.1`
- `completions/catopy.bash`
- `completions/catopy.zsh`
- `completions/catopy.fish`

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
- Trigger: tags like `v0.2.0`
- Builds release archives and SHA256 checksums.

### crates.io

```bash
cargo login
cargo package
cargo publish --dry-run
cargo publish
```

### Fedora COPR

- RPM spec: `catopy.spec`
- Initial validation:

```bash
sudo dnf install -y copr-cli rpmdevtools rpmlint mock fedora-packager rust-packaging
rpmlint catopy.spec
mock -r fedora-rawhide-x86_64 --buildsrpm --spec catopy.spec --sources .
mock -r fedora-rawhide-x86_64 --rebuild *.src.rpm
```

- Build in COPR from Git tag:

```bash
copr-cli buildscm febrezo/catopy \
  --clone-url https://github.com/febrezo/catopy.git \
  --commit v0.2.0 \
  --spec catopy.spec
```

- User install (after COPR project is published):

```bash
sudo dnf copr enable febrezo/catopy
sudo dnf install catopy
```

### openSUSE (OBS)

Use OBS after COPR:

```bash
sudo dnf install osc
osc checkout home:febrezo catopy
osc build openSUSE_Tumbleweed x86_64 catopy.spec
osc commit
```

### Debian/Ubuntu `.deb`

- Simple `.deb` via cargo-deb:

```bash
cargo install cargo-deb
cargo deb
```

- For Debian/Ubuntu long-term packaging, use `dh-cargo`/`debcargo` workflows.

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
