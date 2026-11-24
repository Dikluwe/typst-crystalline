#!/usr/bin/env bash
set -euo pipefail
d="$(cd -- "$(dirname -- "$0")" && pwd)"
cd "$(git -C "$d" rev-parse --show-toplevel)"
ram="$(mktemp -d /dev/shm/tekt-watch-control.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
findmnt -T "$ram" > "$d/control-ram-mount.txt"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
mkdir "$ram/tmp"
date -Iseconds > "$d/control-start.txt"
git rev-parse HEAD > "$d/head.txt"
git diff HEAD --stat > "$d/control-diffstat.txt"
product=(.cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring)
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum > "$d/control-before.sha256"
cp -a lab/experiments/references-500-integration-2026-11-19/target-release "$ram/target"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
taskset -c 1,3,5,7 cargo build --locked --offline -j 2 -p typst-wiring --bin typst --release
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum > "$d/control-after.sha256"
cmp "$d/control-before.sha256" "$d/control-after.sha256"
sha256sum "$ram/target/release/typst" | cut -d ' ' -f1 > "$d/control.sha256"
cp "$ram/target/release/typst" "$d/control.elf"
date -Iseconds > "$d/control-end.txt"
