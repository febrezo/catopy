# Packaging plan for catopy

This document describes the release and packaging route for catopy.

## Route

1. GitHub Releases
2. crates.io
3. Fedora COPR
4. Debian `.deb` in GitHub Releases
5. openSUSE OBS
6. Fedora official repositories
7. Debian official repositories

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

COPR is commonly understood as "Cool Other Package Repo".

In practice, COPR is a hosted RPM build and repository service:

- You provide sources and a `.spec` file.
- COPR builds RPMs for selected Fedora targets.
- Users install through `dnf` after explicitly enabling your COPR repo.

It is ideal for fast distribution, but it is not the same as Fedora official repositories.

Primary RPM spec is `catopy.spec`.
Repository copy is at `packaging/rpm/catopy.spec`.

Validation tools:

```bash
sudo dnf install -y copr-cli rpmdevtools rpmlint mock fedora-packager rust-packaging
rpmlint catopy.spec
mock -r fedora-rawhide-x86_64 --buildsrpm --spec catopy.spec --sources .
mock -r fedora-rawhide-x86_64 --rebuild *.src.rpm
```

Notes:

- For COPR this spec is a good starting point.
- For Fedora official packaging, use the Rust packaging workflow (`rust2rpm`) and proper crate dependency handling.
- If vendoring is required for offline or reproducible builds, vendor dependencies with `cargo vendor` and update spec accordingly.

### COPR quickstart

1. Create a Fedora account (FAS) and a COPR project (for example `febrezo/catopy`).
2. Install tooling:

```bash
sudo dnf install copr-cli rpmdevtools rpmlint mock fedpkg
```

3. Configure `copr-cli` using your API token (`~/.config/copr`).
4. Validate the spec:

```bash
rpmlint catopy.spec
```

5. Trigger a build from GitHub/tag:

```bash
copr-cli buildscm febrezo/catopy \
  --clone-url https://github.com/febrezo/catopy.git \
  --commit v0.2.0 \
  --spec catopy.spec
```

### User install via DNF

```bash
sudo dnf copr enable febrezo/catopy
sudo dnf install catopy
```

Enabling a COPR repository is an explicit trust decision by the user.

## openSUSE OBS

Use OBS after COPR when targeting openSUSE users.

Quickstart:

```bash
sudo dnf install osc
osc checkout home:febrezo catopy
osc build openSUSE_Tumbleweed x86_64 catopy.spec
osc commit
```

OBS helper files live in `packaging/obs/`.

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

For Debian/Ubuntu long-term packaging, migrate to `dh-cargo`/`debcargo` conventions.
