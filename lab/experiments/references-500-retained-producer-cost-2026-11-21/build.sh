#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
parent="$results/references-500-pre-bidi-page-reuse-2026-11-21"
old="$results/references-500-context-substitution-2026-11-21"
links="$results/references-500-link-redundant-vectors-2026-11-21"
# Same absolute path as the recorded ELF, in this sandbox's tmpfs.
ram=/dev/shm/tekt-context-substitution.SUvxx9kr
mkdir -- "$ram"
finish() {
    local status=$?
    date -Iseconds > "$out/$1-end.txt"
    printf '%s\n' "$status" > "$out/$1-exit.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    for file in measurements.jsonl attribution.json native.json; do
        if test -f "$ram/$file"; then cp "$ram/$file" "$out/"; fi
    done
    if test "$status" != 0 && test -f "$ram/candidate"; then
        recovery=$(mktemp -d /tmp/tekt-retained-producer-recovery.XXXXXXXX)
        cp "$ram/candidate" "$recovery/"
        if test -f "$ram/reuse"; then cp "$ram/reuse" "$recovery/"; fi
        printf '%s\n' "$recovery" > "$out/recovery.txt"
    fi
    rm -rf -- "$ram"
}
trap 'finish "$1"' EXIT
git -C "$root" rev-parse HEAD > "$out/head.txt"
git -C "$root" diff HEAD --stat > "$out/diffstat-before.txt"
git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
date -Iseconds > "$out/$1-build-start.txt"
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
tar -xzf "$parent/candidate-source.tar.gz"
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum | cmp "$parent/candidate-input.sha256" -
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
rustc -Vv > "$out/rustc.txt"
case "$1" in
    retained)
        experiment=references-500-context-index-retention-2026-11-21
        arm=reuse
        ;;
    no-producers)
        experiment=references-500-heading-reference-proof-2026-11-21
        arm=proof
        ;;
    *)
        exit 2
        ;;
esac
python3 -B "$root/lab/experiments/$experiment/prototype.py" "$ram/src"
cargo build --locked --offline -j 8 -p typst-wiring --bin typst --release
expected=$(awk -v arm="$arm" '$2==arm {print $1}' "$results/$experiment/binaries.sha256")
printf '%s  %s\n' "$expected" "$ram/target/release/typst" | sha256sum --check
find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/$1-input.sha256"
date -Iseconds > "$out/$1-build-end.txt"
cp "$ram/target/release/typst" "$2/$arm"
printf '%s  %s\n' "$expected" "$arm" > "$out/$1-elf.sha256"
