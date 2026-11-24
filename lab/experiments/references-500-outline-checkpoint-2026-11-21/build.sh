#!/usr/bin/env bash
set -euo pipefail
d="$(cd -- "$(dirname -- "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
ram="$(mktemp -d /dev/shm/tekt-outline-checkpoint.XXXXXXXX)"
trap 'rm -rf -- "$ram"' EXIT
findmnt -T "$ram" > "$d/candidate-ram-mount.txt"
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
mkdir "$ram/tmp" "$ram/source"
cd "$root"
date -Iseconds > "$d/candidate-start.txt"
rsync -a .cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring "$ram/source/"
mkdir -p "$ram/source/lab/tools"
rsync -a lab/benches "$ram/source/lab/"
product=(.cargo Cargo.toml Cargo.lock rust-toolchain.toml 01_core 02_shell 03_infra 04_wiring)
(cd "$ram/source" && find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum) > "$d/control-input-manifest.sha256"
python3 "$d/prototype.py" "$ram/source"
(cd "$ram/source" && find "${product[@]}" -type f -print0 | sort -z | xargs -0 sha256sum) > "$d/candidate-input-manifest.sha256"
cp -a lab/experiments/references-500-integration-2026-11-19/target-release "$ram/target"
cd "$ram/source"
export TMPDIR="$ram/tmp" CARGO_TARGET_DIR="$ram/target" CARGO_INCREMENTAL=0 TYPST_COMMIT_SHA=b9914ad09a078f1c71c2068cf1fad97255de8a9b
export CARGO_ENCODED_RUSTFLAGS=$'-C\x1fllvm-args=-align-all-functions=3\x1f-C\x1fllvm-args=-align-all-functions=3'
rustc -Vv > "$d/rustc.txt"
taskset -c 9,11,13,15 cargo build --locked --offline -j 2 -p typst-wiring --bin typst --release
sha256sum "$ram/target/release/typst" | cut -d ' ' -f1 > "$d/candidate.sha256"
cp "$ram/target/release/typst" "$d/candidate.elf"
date -Iseconds > "$d/candidate-end.txt"
tar -czf "$d/lab-consumers.tar.gz" 03_infra/src/pipeline/{context_stabilization.rs,context_stabilization/page_running.rs} 01_core/src/compiler/layout/{convergence.rs,mod.rs,physical_memo.rs,line_commit.rs,checkpoint.rs,page_running_requests.rs,outline.rs,sequence.rs,lab_retain.rs} 01_core/src/entities/sealed_positions.rs
