#!/usr/bin/env bash
set -euo pipefail

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
output="$root/dist"
staging="$(mktemp -d "$root/.web-build.XXXXXX")"

cleanup() {
  rm -rf -- "$staging"
}
trap cleanup EXIT

mkdir -- "$staging/pkg"
wasm-pack build \
  --target web \
  --release \
  --out-dir "$staging/pkg" \
  "$root/crates/encoding_toolbox_web" \
  -- --locked
rm -f -- "$staging/pkg/.gitignore"
cp -- "$root/crates/encoding_toolbox_web/static/index.html" "$staging/index.html"

if [[ -e "$output" && ! -d "$output" ]]; then
  echo "dist exists but is not a directory" >&2
  exit 1
fi
rm -rf -- "$output"
mv -- "$staging" "$output"
trap - EXIT

test -f "$output/index.html"
test -f "$output/pkg/encoding_toolbox_web.js"
test -f "$output/pkg/encoding_toolbox_web_bg.wasm"
echo "Web application assembled in $output"
