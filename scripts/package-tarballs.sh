#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <target-triple>"
  exit 1
fi

target="$1"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n1)"
name="clipcat"
dist_dir="dist"
bundle_dir="${dist_dir}/${name}-${version}-${target}"

bin_path="target/${target}/release/${name}"
if [[ ! -f "${bin_path}" && -f "${bin_path}.exe" ]]; then
  bin_path="${bin_path}.exe"
fi

if [[ ! -f "${bin_path}" ]]; then
  echo "error: binary not found at ${bin_path}"
  exit 1
fi

rm -rf "${bundle_dir}"
mkdir -p "${bundle_dir}"

cp "${bin_path}" "${bundle_dir}/"
cp README.md "${bundle_dir}/"
cp CHANGELOG.md "${bundle_dir}/"

archive="${dist_dir}/${name}-${version}-${target}.tar.gz"
tar -C "${dist_dir}" -czf "${archive}" "${name}-${version}-${target}"

if command -v sha256sum >/dev/null 2>&1; then
  sum="$(sha256sum "${archive}" | awk '{ print $1 }')"
else
  sum="$(shasum -a 256 "${archive}" | awk '{ print $1 }')"
fi
printf "%s  %s\n" "${sum}" "$(basename "${archive}")" > "${archive}.sha256"
