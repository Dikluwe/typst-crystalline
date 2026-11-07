# Nominal stdlib qualification

Laboratory only; no product/L0 integration or new optimization. snapshot.rs is
byte-identical to watch-startup-neutral/snapshot.rs. apply.py explicitly imports
that experiment's nominal patcher, changing only its in-process RAM constant.
Imports suppress old-area bytecode writes. The previous source remains fixed.

```text
python3 apply.py WORKSPACE_RAM --variant bas|neu|laz
```

WORKSPACE_RAM must be a prepared, separate non-reference child of
`/dev/shm/tekt-stdlib-qualification-20261106`, with the complete received
`.cargo/config.toml`. Symlinks/escapes/frozen/baseline/reference/R are refused.
No Cargo/lock/dependency/configuration or build operation is performed. bas is
a source no-op; neu gets the same private helper and initially None context
slot without using it. laz is exactly the prior static candidate: original root
builder and owned Scope, lazy template on first import, ordered-input/full-
Features guard, new per-file identities with local aliases retained. Evaluated
Modules are never cached. No World method, public portal, selector, environment,
clock or global is added to L1. An existing helper refuses reapplication.

`--mutation share-identities|ignore-inputs|ignore-features` requires laz in a
fresh workspace. It flips only the corresponding copied-helper constant in
RAM after the unchanged nominal patch. Source helper bytes remain original.
New failures must be classified by C; this experiment does not repair the
received helper or reinterpret package failures as candidate regressions.

## Package and input route

Local source audit: 02_shell/src/cli.rs CompileArgs exposes repeated --input;
WatchArgs flattens the same args. There is no package-cache-path or @file input
loader in the current route. 03_infra/src/world/packages.rs package_candidate_dirs
uses XDG_DATA_HOME/typst/packages before XDG_CACHE_HOME/typst/packages.
Consequently `--package-cache` in the **driver** means a RAM XDG cache root
already prepared by C with this structure:

```text
CACHE_RAM/typst/packages/preview/NAME/VERSION/typst.toml
```

The driver sets XDG_CACHE_HOME to that root and XDG_DATA_HOME to an empty shared
RAM data-home under its output directory, preventing host data packages from
shadowing the corpus. It leaves package source unmodified and does not download
or prepare packages. C must materialize the complete pinned corpus/transitive
requirements before timing. Missing packages or unsupported language features
must remain visible; the successful benchmark fails on nonzero CLI status or
missing watch publication, retaining RAM diagnostics. There is no failure-to-
success smoke conversion. C may classify unsupported cases separately first.

## Driver

Derived from outline-warm-residual/measure.py, with no probe/runtime selector.

```text
python3 measure.py --variant bas=BASE_ELF --variant laz=LAZ_ELF \
  --fixtures CORPUS_DIR --fixture main.typ --input-file INPUTS.json \
  --package-cache CACHE_RAM --ram-dir NEW_OUTPUT_RAM \
  --rounds 10 --repeats 2 --cycles 2 --cpu 0
```

Repeat --fixture and --variant; all variants receive the same ordered input
arguments/environment shape. --input-file is optional for the no-input control.
Preferred JSON is an array of string pairs, e.g. `[["a","x"],["a","y"]]`;
ordered objects are also accepted, preserving duplicate pairs. No strings are
coerced, sorted or truncated. Keys must be nonempty without `=`; values may
contain `=`/empty strings. NUL is rejected. The file is parsed before timing and
becomes repeated `--input=key=value` arguments in exactly its pair order.

Default `--input-profile normal` permits at most64 pairs and4096 UTF-8 bytes per
key/value. `--input-profile large` permits512 pairs and65536 bytes per string.
Both additionally check per-argument limits and a conservative ARG_MAX budget
including environment, pointer table and safety space before spawning. Not all
maximal combinations fit; refusal is explicit. Larger cases, including1MiB,
belong in the independent APIWorld tests rather than oversized CLI arguments.
`--extra-arg=VALUE` is repeatable for existing product flags; inputs must come
only from the JSON option to preserve one explicit ordering. No new productive
flag is introduced.

--cycles defaults2, meaning two same_bytes/main_edit pairs per watch process.
CLI/startup are cycle0. Warm cycles1..2N alternate odd same_bytes / even main_edit;
TEKT_EPOCH_((cycle//2)%2) is appended to each case's main fixture. Same-byte
writes preserve the current marker and edits alternate0↔1. Package source files
are not changed. Publications are saved as cli.pdf, startup.pdf,
cycle-001-same_bytes.pdf, cycle-002-main_edit.pdf, etc. Validate text/semantics
offline; no PDF extraction runs inside the clocks.

CPU0 is explicit by default; an unavailable CPU is rejected without fallback.
The driver retains stack64MiB, RAYON1, poll1ms, settle30ms, wall/process CPU clocks,
wait4 user/system/peak RSS, balanced order rotation, stable new PDF identity and
%%EOF validation. Every child is terminated and reaped on error. Fixtures,
package cache and output stay separate, with generated files only under the
exclusive task RAM area. CSV is flushed per completed process, with columns:

```text
round repeat variant fixture mode cycle process_id case sample_epoch
wall_ns cpu_ns process_cpu_ns process_user_ns process_system_ns
process_peak_rss_bytes exit_code
```

Process totals/RSS repeat across that process's cycle rows, not per-cycle
quantities; watch cpu_ns is the measured cycle delta. sample_epoch is wall-clock
provenance. Primary timing includes normal process/watch publication latency.
No gain, safety qualification or attribution follows merely from these sources;
C performs corpus classification, independent tests and paired measurements.

## Optional inherited data-home environment

`--inherit-data-home` preserves the original XDG_DATA_HOME value. If it was
absent, the child environment leaves it absent, using the normal HOME/.local/share
route. This retains the original data-home font discovery environment. Before
each process and outside its clocks, the driver checks the effective
`data-home/typst/packages`: it must be absent or empty. Any entry, including a
hidden entry, is refused explicitly to prevent host packages from shadowing the
prepared RAM corpus. Unreadable/unverifiable directories are also refused.
Relative explicit data-home values are checked relative to the child working
directory, matching the actual package route. The check never writes host data.

Without this option the previous empty-RAM-data-home behavior remains. Earlier
batches using that behavior describe an isolated environment; they must not be
relabeled as stock-font measurements. C qualifies equality of the stock font
inventory and reruns comparisons using the same ELFs. This option changes only
the measurement environment before the input matrix; candidate/helper, package
cache policy, input ordering and the CSV schema remain unchanged.
