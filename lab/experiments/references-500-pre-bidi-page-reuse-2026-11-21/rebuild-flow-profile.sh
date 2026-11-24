#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
old="$results/references-500-context-substitution-2026-11-21"
links="$results/references-500-link-redundant-vectors-2026-11-21"
# Same absolute path as the recorded ELF, in this sandbox's tmpfs.
ram=/dev/shm/tekt-context-substitution.SUvxx9kr
mkdir -- "$ram"
trap 'rm -rf -- "$ram"' EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
analysis=${1:-flow-profile}
case "$analysis" in flow-profile|eval-work-split) ;; *) exit 2 ;; esac
findmnt -T "$ram" > "$out/$analysis-mount.txt"
date -Iseconds > "$out/$analysis-start.txt"
mkdir -p "$ram/src/lab" "$ram/tmp"
cd "$root"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/src/"
rsync -a lab/benches "$ram/src/lab/"
python3 -B "$root/lab/experiments/references-500-visible-textual-style-collapse-2026-11-21/prototype.py" "$ram/src"
cd "$ram/src"
product=(.cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring)
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$old/source-base.sha256" -
tar -xzf "$old/retained-source.tar.gz"
cp "$links/link_capture.candidate.rs" 01_core/src/compiler/layout/link_capture.rs
tar -xzf "$out/candidate-source.tar.gz"
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$out/candidate-input.sha256" -
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
candidate="$ram/target/release/typst"
vanilla="$root/lab/typst-original/target/release/typst"
(cd "$out" && awk '$2=="candidate" {print $1}' binaries.sha256) | {
    read -r expected
    printf '%s  %s\n' "$expected" "$candidate" | sha256sum --check
}
sha256sum "$candidate" "$vanilla" > "$out/$analysis-binaries.sha256"
awk '$2=="vanilla" {print $1}' "$out/binaries.sha256" | {
    read -r expected
    printf '%s  %s\n' "$expected" "$vanilla" | sha256sum --check
}
nm -n -S -C --defined-only "$candidate" > "$ram/candidate.symbols"
python3 -B "$d/$analysis.py" "$out" "$ram/candidate.symbols"
date -Iseconds > "$out/$analysis-end.txt"
