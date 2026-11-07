# CLI residual: unchanged delayed stdlib

Quarantine only. This directory does not alter any product, L0, previous helper,
build configuration, lockfile or dependency. C prepares exclusive RAM
`/dev/shm/tekt-stdlib-cli-residual-20261106` and frozen source copies including the
complete received `.cargo/config.toml`, then applies the original nominal overlay:

```text
python3 -B apply.py RAM_COPY --variant bas|neu|laz
```

The wrapper changes only the imported patcher's process-local RAM constant. It
retains the old workspace/symlink/reference/reapplication/config/helper guards.
`bas` remains a source no-op. No new runtime mode or policy enters L1.

## CLI-only measurement

```text
python3 -B measure.py --variant bas=BASE_ELF --variant neu=NEUTRAL_ELF \
  --variant laz=LAZY_ELF --fixtures FIXTURE_TREE --fixture no_import.typ \
  --input-file INPUT_PAIRS_JSON --input-profile large \
  --package-cache RAM_CACHE_ROOT --ram-dir NEW_OUTPUT_RAM \
  --rounds 36 --repeats 1 --cpu 0
```

The wrapper imports `stdlib-qualification/measure.py` privately without writing
bytecode to the earlier directory. It calls the unchanged `run_process` for
**CLI only**, retaining wait4 CPU/user/system/RSS, wall clock, PID/case/epoch and
stable complete PDF publication. CSV schema is the previous 16 fields. CLI rows
have mode=cli/cycle=0; no watch warmup or clock is run. `--cycles` remains accepted
by the shared parser but is irrelevant here. The source marker is unchanged.

Two to four alphanumeric labels are permitted. Multiple labels may point to the
same ELF for the A/A control; this is not a distinct build. Every permutation is
used once per factorial-sized block of observations per fixture. With three
variants use a multiple of six total round/repeat observations; with four labels,
a multiple of 24. Position balance does not itself remove thermal, filesystem,
ASLR or time-dependent noise. Warmups are one per fixture/label and not exported.

Stock data-home fonts are always inherited, regardless of the old parser's
optional flag. The old read-only refusal of a nonempty/unverifiable data-home
package directory remains enforced outside each process clock. The prepared RAM
cache can be empty for no-import cases; it must contain `typst/packages`.
The compile CLI does not expose `--ignore-system-fonts`; no such flag is used.
The no-font phase control is the separate public-API evaluator below. Inputs
must use the explicit JSON option; no new product input/file flag is introduced.

### Optional shared absolute input paths

`--shared-input-root NEW_RAM_DIR` creates one self-contained fixture copy before
warmups, appending the ordinary CLI `EPOCH_ZERO` marker to every requested main
fixture once. All nominal/A-A labels then receive the same absolute source path,
`--root` and working directory. Each process still has its own output/temp/case
directory; `run_process` and its clock are unmodified. Its usual disposable
private input copies are still prepared but are not passed to the compiler.

The directory must be a fresh, non-symlink child of this task's RAM, disjoint from
the output tree, cache and source fixtures; reference/frozen names are refused.
No shared source is written during warmups or measured processes. Redirection is
through the privately imported Child constructor boundary. The inherited package
guard checks relative data-home paths against the actual shared working directory
outside the process clock. Without the option the previous path flow is retained.
Keep this as a distinct diagnostic cohort: sharing paths reduces path-dependent
hash/ID variation but does not establish a causal explanation for a timing shift.

## Public-API no-import evaluation control

`eval_control.rs` is an external standalone executable, not a product test/cfg or
portal. Link against the **actual compiler-artifact paths** from each nominal
build. Rust extern dependencies: `typst_core`, `comemo`, `serde_json`; search its
release dependency directory with `-L dependency=...`. No Cargo dependency edit.
Build the same harness with the same flags for bas/neu/laz, writing only RAM:

```text
rustc --edition=2021 -C opt-level=s -C lto=thin -C codegen-units=1 \
  -C llvm-args=-align-all-functions=3 eval_control.rs -o RAM_API_ELF \
  -L dependency=RAM_TARGET/release/deps \
  --extern typst_core=ACTUAL_CORE_RLIB --extern comemo=ACTUAL_COMEMO_RLIB \
  --extern serde_json=ACTUAL_SERDE_JSON_RLIB
RAM_API_ELF --source no_import.typ --input-file large.json \
  --iterations 100 --samples 10 --warmup 10
```

C controls affinity/resource limits and interleaves nominal processes. Source
parsing, reading JSON, map construction and ordered full-pair assertions occur
outside the timed loop. The exact same Source, World, Routines and element
registry are reused; each iteration invokes the ordinary full public evaluator,
creates its own sink/context and drops its result. `black_box` keeps evaluation
observable. There is no font acquisition, layout, export or filesystem in the
loop. Success-path sink/result checks are identical in every harness; unexpected
diagnostics abort rather than becoming a sample. Each sample verifies exactly
one World.inputs per iteration. This isolates evaluation, not complete CLI
startup; its magnitudes cannot replace CLI clocks or attribute a difference to a
particular evaluator function.

CSV to stdout: `sample,iterations,input_pairs,elapsed_ns,world_input_calls`.
Samples inside one process share state and are not independent process replicas.
No gain, practical regression budget or integration verdict is encoded by these
scripts; C interprets independent tests, A/A and baseline/neutral/lazy results.
