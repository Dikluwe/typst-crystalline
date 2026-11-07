# Watch startup: neutral control

Quarantine experiment; no productive source or L0 is changed. The patchers only
accept separate prepared children of `/dev/shm/tekt-watch-startup-20261106`.
Frozen/baseline/R paths and symlinks/escapes are refused. They do not copy, build,
run tests, change Cargo configuration, write receipts or choose compiler flags.
Keep the complete received Cargo configuration in all three workspaces.

`python3 apply.py WORKSPACE --variant bas|neu|laz` installs an unprobed variant:

- `bas`: original code, no modifications.
- `neu`: the exact helper, EvalContext slot type/position and None initializer
  of `laz`; nothing reads or populates the slot. Root and imports stay original.
- `laz`: the previous nominal lazy candidate, with constant policy 3. Roots
  retain their literal original builder/owned Scope; first import captures a
  private template, subsequent imports instantiate fresh identities after the
  ordered-inputs/full-Features guard. No evaluated Module memoization occurs.

The two modified variants use identical copied private helper source. No World
method, public portal, runtime selector, env, clock or global enters L1. Unused
private helper branches may be removed by release codegen; this is a measurement
question. Size and allocations are to be measured, not assumed from the source.
The expected causal distinction is extra context storage versus actual reuse.

`--mutation neutral-allocate` requires neu and initializes its slot with a dummy
Arc snapshot. `--mutation session-always-reuse` skips all freshness checks for an
already populated session; use a separate mutated workspace. These deliberate
faults must be rejected by executable tests, not merely source inspection.

After retaining each uninstrumented ELF, `python3 apply_probes.py WORKSPACE`
installs the same opt-in overlay on bas, neu and laz. Set `TEKT_WATCH_PHASE_FILE`
to a file outside every discovered font root. Unset means no clocks, counters,
file creation or output; opt-in branch/OnceLock checks remain in the probed ELF.
Calibrate on/off using that same ELF. Only normal CLI/watch completion flushes,
after publication/diagnostics and outside all measured scopes. Early errors
that return before the publication tail need not produce a cycle. A file error
prints a diagnostic and does not change compilation results. One process should
own each output file. `--mutation probe-invalid-label` emits an unknown cycle
label for the independent checker to reject.

TSV has no header: `pid<TAB>cycle<TAB>label<TAB>calls<TAB>ns`. Rows are unique and
sorted by label within one flush; pid is positive, cycles begin at zero and
increase per process, calls are positive. Event rows have ns=0. Missing labels
mean zero calls. `event.cycle` has one call for every completed flush.

## Inclusive scope hierarchy

All clocks aggregate entire operations, never individual paths/faces. Parent
names below mean containment at a call site, not disjoint partitions:

```text
catalog.access
  catalog.initialize
    session.total OR catalog.fresh_build
session.total
  session.lock_wait
  session.initial_fast_verify -> fast.verify
  session.initial_key_check -> generation.unchanged
  session.probe_before -> fast.key
  session.prepare -> prepare.total
  session.prepared_check -> generation.unchanged
  session.reuse_verify -> fast.verify
  session.build -> catalog.prepared_build
  session.final_key_check -> generation.unchanged
  session.final_fast_verify -> fast.verify
fast.key
  fast.config
  fast.walk_roots -> fast.roots_recheck
  fast.config_bytes_recheck
fast.verify
  fast.config
  fast.roots_recheck
prepare.total
  discovery.fontdb_load_system
  prepare.from_database
    project.discovery
    prepare.collect_stamps
    generation.unchanged
catalog.prepared_build
  catalog.embedded
  catalog.acquire_selective_coverage
  catalog.compose -> catalog.project_book
catalog.fresh_build
  catalog.embedded
  discovery.fontdb_load_system
  catalog.acquire_selective_coverage
  project.discovery
  catalog.compose -> catalog.project_book
pipeline.paged -> pipeline.eval
pipeline.pdf_export
source.access
```

`session.total` includes lock acquisition, proof, builds and any fresh fallback;
`session.lock_wait` isolates acquisition, not the entire lock-holding period.
`fast.config` includes config parsing, bytes and environment capture/sort.
`fast.roots_recheck` is aggregate metadata proof, not a content hash.
`prepare.collect_stamps` includes descriptor collection and its existing bounds.
`catalog.acquire_selective_coverage` covers the entire catalog_from_database loop,
including selective acquisition, fallbacks, coverage and insertion; no per-face
clock was added. `catalog.project_book` is the project pairing within composition.
`pipeline.eval` includes imports, not just root scope construction. PDF dispatch
includes subsetting/serialization, and is separate from the paged scope.
Source/catalog calls may precede the pipeline or nest in eval/layout/PDF.

Several labels have multiple parents and aggregate their calls. Never compare
such a label to only one parent or sum overlapping clocks into a false total.
The real default path remains selected; the probes do not request
`--timings-json` or oracle emission. Wrappers call original expressions once,
retaining `&&`/`||` short-circuit order, returns, locks, postchecks and fallback.
Clocks can affect scheduling, so proof outcomes under concurrent mutation can
still differ without weakening the proof itself.

## Branch events and count constraints

Every session call emits one of cold/warm/fallback_poisoned, and exactly one
terminal event from reuse_fast/reuse_prepared/published_new/the four fallbacks.
The full labels are `event.session.` followed by:

```text
cold warm reuse_fast reuse_prepared published_new
fallback_poisoned fallback_prepare fallback_prepared_changed fallback_final_changed
```

Let missing counts be zero. `session.total = cold + warm + fallback_poisoned`;
`lock_wait = total`; `initial_fast_verify = warm`; `initial_key_check <= warm`;
`prepare = probe_before = total - reuse_fast - fallback_poisoned`;
`prepared_check = prepare - fallback_prepare`;
`build = final_key_check = published_new + fallback_final_changed`;
`final_fast_verify <= build`. Initial verify can decline before any fast-key
work; do not assume a particular environment is eligible. A non-Linux build
legitimately lacks Linux fast scopes. Timer nanoseconds are nonnegative and
inclusive; bounds for shared labels require all possible parents.

`catalog_harness.rs` optionally compares public APIs with two fresh Worlds per
round: direct discovery versus one shared FontCatalogSession (first sample cold,
later warm). Arguments are INPUT [ROUNDS=25]; mode order alternates. It prints
config/Source/first-book TSV after measurement. This is a catalog microbench,
not the whole watch. `--cfg watch_phase_probe` optionally flushes probes outside
its measured scopes when linking the instrumented infra library.

## Build-recipe control

The received `.cargo/config.toml` selects
`-C llvm-args=-align-all-functions=3` on x86-64. Copy that configuration for the
primary recipe, overriding only `CARGO_TARGET_DIR` into RAM. A separate historical
recipe deliberately omits the config to reproduce the preceding experiment's
RAM recipe. These are different measurements; never pool their baselines or
attribute their difference solely to the stdlib slot. The alignment option is
an existing product setting, not a new optimization proposed by this experiment.

`measure.py` retains the preceding CLI/watch publication and process-clock
method. It takes named `--variant NAME=BINARY:-`, self-contained fixtures and a
new task-RAM output directory. `--probe-variant NAME` sets
`TEKT_WATCH_PHASE_FILE` for only that process; other variants remain off.
Use names and paths of matching lengths, rotate variant order, validate outputs
and epochs outside clocks, and keep all builds/counters/extractors outside the
timed batches. `--timings-json` is not used.

The flush hook is the tail of `run_compile_observed_with_sessions`. For CLI its
file has been published; for watch that file is still the staging destination,
and the watch loop publishes the final destination afterwards. Therefore probe
flush I/O is outside the font/pipeline scopes but inside the observed watch
publication wall interval. On/off calibration includes that cost. Do not claim
that the TSV write is outside the watch wall measurement.
