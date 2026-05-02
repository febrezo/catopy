# Changelog

## Unreleased

### Added

- Project renamed from `clipcat` to `catopy` to avoid collisions with an existing crate and repository name.
- Professional CLI output style with short success, warning, and error messages.
- New output color controls: --no-color and NO_COLOR.
- Integration test suite in tests/ with CLI-focused behavior checks.
- Packaging and distribution infrastructure:
  - Man page at `man/catopy.1`
  - Static shell completions for bash, zsh, and fish
  - RPM spec skeleton at `packaging/rpm/catopy.spec`
  - Debian packaging skeleton at `packaging/debian/`
  - Packaging process documentation in `PACKAGING.md`
  - Maintenance `Makefile` with release/package-check targets
- GitHub release workflow now publishes release assets and SHA256 checksums to tagged releases.
- Cargo package metadata expanded for crates.io readiness (`authors`, `repository`, `homepage`, `include`).
- Local and CI cross-build matrix aligned for:
  - x86_64-unknown-linux-gnu
  - x86_64-unknown-linux-musl
  - aarch64-unknown-linux-gnu
  - aarch64-unknown-linux-musl
  - armv7-unknown-linux-gnueabihf
  - x86_64-pc-windows-gnu
- Cross-compilation and packaging setup:
  - GitHub Actions CI and release workflows
  - Cross.toml for cross builds
  - cargo-deb and cargo-generate-rpm metadata
  - tarball + SHA256 generation script
- README distribution section for AUR, COPR, .deb, Homebrew, crates.io, and manual install.

### Fixed

- Linux clipboard handoff now uses `arboard`'s `SetExtLinux::wait_until(...)` to reduce "Clipboard was dropped very quickly" warnings for short-lived runs.
