# Packaging plan for clipcat

This document describes the release and packaging route for clipcat.

## Route

1. GitHub Releases
2. crates.io
3. Fedora COPR
4. AUR/Homebrew later
5. Fedora official repositories
6. Debian official repositories

## GitHub Releases

- Tag format: `vX.Y.Z`
- Workflow: `.github/workflows/release.yml`
- Builds with `cross` for:
  - `x86_64-unknown-linux-gnu`
  - `x86_64-unknown-linux-musl`
  - `aarch64-unknown-linux-gnu`
  - `aarch64-unknown-linux-musl`
  - `x86_64-pc-windows-gnu`
- Produces tarballs and `.sha256` checksum files.
- Uploads assets to the GitHub Release attached to the tag.

## crates.io

Local flow:

```bash
cargo login
cargo package
cargo publish --dry-run
cargo publish
```

Do not run `cargo publish` from CI.

## Fedora COPR and RPM

RPM spec template is at `packaging/rpm/clipcat.spec`.

Validation tools:

```bash
sudo dnf install rpmdevtools rpmlint mock fedpkg
rpmlint packaging/rpm/clipcat.spec
```

Notes:

- For COPR this spec is a good starting point.
- For Fedora official packaging, use the Rust packaging workflow (`rust2rpm`) and proper crate dependency handling.
- If vendoring is required for offline or reproducible builds, vendor dependencies with `cargo vendor` and update spec accordingly.

## Debian/Ubuntu

Debian skeleton is at `packaging/debian/` with:

- `control`
- `rules`
- `changelog`
- `copyright`

For local `.deb` builds use cargo-deb:

```bash
cargo install cargo-deb
cargo deb
```

For Debian official packaging, migrate to `debcargo`-aligned source package conventions.
