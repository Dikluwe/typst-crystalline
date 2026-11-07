# Joint font witness, capture v2

Quarantine only. Native fontdb methods remain the oracle; copied vendor version
0.23/dependencies/features are unchanged. The productive read_at/extractor,
coverage, slot ordering and fallback are untouched. No productive source, L0,
Cargo configuration or older experiment is changed by these source files.

## Interfaces

The crate is `joint_font_witness`. All previous APIs/types are retained:
`capture_dirs`, `capture_system`, `capture_config_file`, `validate`, `Limits`,
`Stamp`, `ExpectedConfig`, `Evidence`, `Attempt`, `Stats`, `Event`.

`CaptureMode::{Legacy, SingleRegular}` selects capture mechanics. The added
`capture_dirs_with_mode(roots, limits, hook, mode)`,
`capture_system_with_mode(expected, limits, hook, mode)` and
`capture_config_file_with_mode(path, expected, limits, hook, mode)` have the
same previous arguments plus mode last. Old functions default to SingleRegular.

Legacy reproduces v1 observations. SingleRegular uses the lstat metadata for
both link and target of a non-symlink, including directories and roots. Links
still query their target independently. Only capture changes; validate still
queries link and target separately, including negative witnesses. New counters
`Stats.capture_lstat` and `capture_stat` count real attempted std::fs operations
during capture only, including failed attempts. They exclude validation,
fontdb's own metadata operations and kernel syscall interpretation.

The native observed traversal still provides canonical and lexical witnesses;
no second read_dir is added and no further observation is dropped for dedup.
Original retention rules, negative/invalid candidates, metadata identity,
ctime/inode/mode/ns, context, bounds (16384 descriptors / 1 MiB), errors and
selfcheck remain. This is one discovery traversal, not one byte read/extractor.

`establish<F,P,K,C>(discover, check_prepared, key_of, build, check_key,
check_policy) -> Option<(F,K,C)>` is the actual cold-session controller. Its
order is discover (includes G1), G2, key extraction, build, G3, policy. Every
None/false short-circuits. The session wires the real prepared/key unchanged
predicates and original fast_key_unchanged policy reparse; it publishes only
Some after all gates. Warm/poison/refusal releases the attempted joint lock and
uses the original route. Callback/config evidence alone is not proof of the
complete Fontconfig policy. CLI direct discovery stays original.

## Applying isolated copies

C prepares a full pipeline copy under `/dev/shm/tekt-joint-witness-20261106`,
including the complete received `.cargo/config.toml`. Keep its bytes and flags
identical in all pipeline copies, including the active LLVM alignment option;
set only appropriate RAM build/cache paths externally. Frozen/baseline/R paths,
symlinks and escapes are refused. Do not reuse older experiment scripts.

```text
python3 apply_overlay.py PIPELINE_RAM [ENGINE_RAM]
```

If ENGINE_RAM does not exist, the patcher copies this crate there. Existing
prepared copies are preserved. Pipeline and engine must be separate RAM
children. The patcher installs only two L3 source overlays, the lab dependency
and local fontdb patch. It requires a config file but never rewrites it, runs
Cargo or changes the lockfile. C reconciles only paths/the added lab package
and rejects dependency upgrades. The permitted lock delta is the new
joint_font_witness 0.1.0 path package, the infra dependency edge to it, and the
existing fontdb 0.23.0 source/checksum replaced by the RAM path. Its dependency
versions and every unrelated locked package must remain identical. Reconcile
offline against the received lock and inspect this delta before any build;
a broad unlocked update is outside this experiment. Pipeline mode is one character in L3:
`TEKT_JOINT_MODE=0` original, `1` joint Legacy, `2` joint SingleRegular. Set 0
on the independent original binary too for equal environment shape. The
linked mode0 control separates diagnostic/vendor codegen from capture changes.

Directed mutations require a fresh private ENGINE_RAM:
`--mutation ignore-ctime|omit-witness|omit-selfcheck|omit-g2|omit-g3|omit-policy`.
Mutants exist only in that RAM engine. They must be rejected by executable
behavioral tests; compile failures do not count as detection.

## Separate aggregate probes

Retain the unprobed ELF first, then apply to a fresh/shared engine exactly once:

```text
python3 apply_probes.py PIPELINE_RAM ENGINE_RAM
```

Do not probe an engine still serving an unprobed rebuild. Set
`TEKT_JOINT_PHASE_FILE` outside every font root to enable. Off reads no clocks,
writes no output and takes no aggregate-counter lock, but opt-in checks remain.
Calibrate on/off using the same ELF. Probes preserve expression/short-circuit
order, lock release, all G1/G2/G3/policy checks and fallback. Clocks can affect
scheduling, so concurrent races can legitimately change refusal timing.

One shared accumulator in the lab crate receives engine and infra records.
TSV has no header: `pid cycle label calls ns`, separated by tabs. Each normal
publication tail flushes after measured scopes, with event.cycle=1, ns0; cycle
starts at0 per pid. Missing labels mean0; labels are unique per pid/cycle.
Only one process should own a file. Early publication-tail bypass/errors need
not flush. `--mutation probe-invalid-label` emits an unknown cycle label for
independent validation. No per-face/path clocks exist.

The standard route reuses labels from watch-startup-neutral: catalog.access /
initialize; session.total/lock_wait/probe_before/initial_fast_verify /
initial_key_check/prepare/prepared_check/reuse_verify/build/final_key_check /
final_fast_verify; fast.key/verify/config/walk_roots/roots_recheck /
config_bytes_recheck; prepare.total/from_database/collect_stamps;
discovery.fontdb_load_system; project.discovery; generation.unchanged;
catalog.prepared_build/fresh_build/acquire_selective_coverage/embedded /
compose/project_book; source.access; pipeline.paged/eval/pdf_export. The same
standard branch event.session.* labels and count constraints apply whenever
that route is entered. Mode0 and CLI context can be profiled without selecting
--timings-json or an oracle PDF path.

Added inclusive hierarchy:

```text
joint.session
  joint.lock_wait
  joint.establish
    joint.establish_discover
      fast.config
      joint.capture
        joint.config_context
        joint.discovery
        joint.evidence_selfcheck
      joint.g1_prepare -> prepare.from_database (including G1 unchanged)
    joint.g2 -> generation.unchanged
    joint.build -> catalog.prepared_build
    joint.g3 -> generation.unchanged
    joint.policy -> fast.verify (original policy reparse + root rechecks)
  session.total (only after warm/decline/poison fallback)
```

`joint.discovery` includes native discovery and capture callbacks. Capture
therefore includes config-context setup, discovery and final selfcheck. These
clocks overlap and are not a partition. Acquisition remains the aggregate
catalog_from_database/selective+coverage timer; no per-face probe is inserted.
Shared child labels can have multiple parents; do not compare them to only one
parent or sum inclusive totals into exclusive time.

Events `event.joint.cold`, `.warm`, `.poisoned`, `.established`, `.decline`,
`.fallback_standard` have ns0. `joint.session=cold+warm+poisoned`,
`cold=established+decline`, `fallback_standard=warm+poisoned+decline`.
`joint.establish=joint.establish_discover=cold`, with
`g2<=cold`, `build<=g2`, `g3=build`, `policy<=g3` (nominal, completed calls).
`event.joint.capture_lstat` / `.capture_stat` aggregate their Stats attempt
counters as calls, ns0 (absent if zero), not outcome counts. Their timings and
kernel syscall counts must be measured independently. No gain or causal
attribution is claimed by the source; keep Unknown if paired runs do not
separate causes or show a net gain.
