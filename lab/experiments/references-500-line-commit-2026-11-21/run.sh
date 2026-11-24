#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
out="$root/lab/results/construction/diagnostics/$(basename "$here")"
mkdir -p "$out"
ram=$(mktemp -d /dev/shm/tekt-line-commit.XXXXXXXX)
trap 'rm -rf -- "$ram"' EXIT
export TEKT_LAB_WORK="$ram" TEKT_CONTEXT_OUT="$out" TMPDIR="$ram/tmp"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
mkdir -p "$ram/src/lab" "$TMPDIR"
cd "$root"
git rev-parse HEAD > "$out/head.txt"
git diff HEAD --stat > "$out/diffstat-before.txt"
git diff HEAD -- 00_nucleo 01_core 02_shell 03_infra 04_wiring .cargo Cargo.toml Cargo.lock | sha256sum > "$out/product-before.sha256"
if [[ ${1:-} != resume ]]; then date -Iseconds > "$out/start.txt"; fi
findmnt -T "$ram" > "$out/mount.txt"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
cp lab/typst-original/target/release/typst "$ram/vanilla"
cd "$ram/src"
find .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring -type f -print0 | sort -z | xargs -0 sha256sum > "$out/control-source.sha256"
build() {
    date -Iseconds >> "$out/$1${suffix:-}-build.log"
    cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release >> "$out/$1${suffix:-}-build.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/typst" "$ram/$1"
    date -Iseconds >> "$out/$1${suffix:-}-build.log"
}
if [[ ${1:-} == resume ]]; then
    cp "$out/binaries.sha256" "$out/first-binaries.sha256"
    date -Iseconds > "$out/resume-start.txt"
    suffix=-resume
    export CARGO_PROFILE_RELEASE_LTO=false
fi
build integrated
python3 "$here/inject.py" "$ram/src"
python3 "$here/prototype.py" "$ram/src"
build proof
ln -s proof "$ram/census"
if [[ ${1:-} != resume ]]; then
    python3 "$here/bench.py" census
fi
(cd "$ram" && sha256sum integrated census proof vanilla) > "$out/binaries${suffix:-}.sha256"
if [[ ${1:-} == resume ]]; then
    python3 "$here/bench.py" resume
else
    python3 "$here/bench.py" verify
fi
git -C "$root" diff HEAD -- 00_nucleo 01_core 02_shell 03_infra 04_wiring .cargo Cargo.toml Cargo.lock | sha256sum > "$out/product-after.sha256"
python3 "$here/summarize.py" "$out"
date -Iseconds > "$out/end.txt"
