# Outline warm residual

Laboratory only. Reuses joint-font-witness by explicit Python imports; only the
in-process RAM constant changes. Old files, product, L0, guards, Cargo recipes
and compiler flags stay untouched. Every prepared pipeline must include the
complete received `.cargo/config.toml`; no lock reconciliation or build is run
by these scripts. C retains unprobed ELFs before installing optional probes.

```text
python3 apply.py PIPELINE_RAM --variant base
python3 apply.py PIPELINE_RAM --variant joint --engine ENGINE_RAM
# Later, in the same unprobed pipeline/engine, after saving its ELF:
python3 apply.py PIPELINE_RAM --variant base --probes
python3 apply.py PIPELINE_RAM --variant joint --engine ENGINE_RAM --probes
```

Paths must be separate non-reference children of
`/dev/shm/tekt-outline-warm-20261106`, without symlinks/escapes. The joint path
is the previous nominal overlay/engine, unchanged when probes are absent.
Already-installed joint is accepted only for its separate probe step, after
checking the exact nominal sources and matching dependency/vendor paths.
Reapplying probes or a joint overlay is refused. Base without probes is a source
no-op. Base probes use a standalone infra collector, with no lab dependency or
vendor patch; joint probes share the previous engine collector. Both get the
same new stage-attribution changes from the definitive phase_probe.rs source.
That standalone source can be imported by independent Rust tests; the previous
collector is never changed. Imports suppress bytecode writes in the old area. C resolves
only the previous minimal path/lab-package lock delta offline, without upgrades.

Set `TEKT_JOINT_MODE=0` for baseline/control and `2` for joint. The unprobed joint
also supports previous mode0, separating linked-code effects from the candidate.
Set `TEKT_JOINT_PHASE_FILE` outside font roots to enable probes. Unset means no
clock, output, aggregate lock or TLS-stage access. Calibrate the same probed ELF
on/off; opt-in branches remain. The original publication-tail flush is retained.

TSV remains `pid<TAB>cycle<TAB>label<TAB>calls<TAB>ns`, no header. All previous
labels/branch constraints remain, including fast-key and GenerationKey detail.
New inclusive labels, adjacent to the original t1/t2/t3/t4 boundaries:

- `pipeline.introspect`: t1→t2, including heading supplement preparation.
- `pipeline.context`: t2→t3, including contextual preparation.
- `pipeline.layout`: t3→t4, including its original realization/fixpoint work.
- `pipeline.finish`: t4→return, including bidi/shaping/validation.
- `layout.round`: each real invocation of the callback-layout helper.
- `layout.pass.1`…`.8`: each actual call from that helper into L1, including
  argument preparation; the timer ends before handling Complete/Pending.
- `event.layout.complete` / `.pending`: actual outcomes, ns0. Their combined
  calls equal pass calls on completed executions.
- `context.attempt.1`…`.5`: each actual iteration of the selective stabilization
  loop, including early error returns. No attempt count is inferred.
- `source.construct`: actual Source::new/Source::new_with_parser expressions
  in L3 world/cache.rs (synthetic/get/source_for_expression) and
  world/source_session.rs (get_or_parse). Includes construction and parsing,
  not filesystem loading/error conversion/cache hits. It can precede paged or
  occur without a registered parent; it is not a pure-parse measurement.

RAII closes scopes on early returns. The five stage timers and new phase timers
are children of paged; layout helpers can run in context or layout, and may be
invoked repeatedly. Per-pass/attempt labels aggregate across those invocations;
the numbers are local loop indices, not one global budget per compilation.
No clock is placed in L1 or per glyph/path. Policy and argument order remain.

`font.eval`, `font.introspect`, `font.context`, `font.layout`, `font.finish`,
`font.outside` duplicate each catalog.initialize interval into exactly one label,
with identical calls/ns. A thread-local coarse stage is active only inside the
five matching pipeline scopes. RAII saves/restores the previous stage, including
nested scopes/errors. Guard uses PhantomData<Rc<()>> so it cannot cross threads;
the phantom allocates nothing. A worker without a local stage is outside; no
attribution is guessed from another thread. Off never accesses this TLS.
These labels are inclusive duplicates: do not add them to catalog.initialize.
Their sums must equal initialize; verify each stage bound before deriving
stage_nonfont=stage-font.stage. A negative/unexplained bound stays Unknown.

## Measurement driver

`measure.py` derives from the previous driver, with `--cycles N` (default6)
meaning N pairs of same_bytes/main_edit after each watch startup. Variant syntax
is `--variant NAME=ELF:0|2`; baseline receives env0 too. CPU defaults to0
(`--cpu` can select an explicitly available CPU; no silent fallback). Stack64MiB,
RAYON1, SOURCE_DATE_EPOCH0, poll1ms and settle30ms are retained. Repeat
`--extra-arg=VALUE` to pass existing product configuration arguments unchanged.
No new productive flag is created. Every subprocess is terminated/reaped on
errors. Publication must have a new stable identity and complete %%EOF before
ending the measured interval. PDF extraction/validation is offline.

`--ram-dir` must be new, non-reference and under the exclusive RAM root. All
fixtures/copies/logs/CSVs/PDFs remain there. CSV is incremental per completed
process. Columns:

```text
round repeat variant fixture mode cycle process_id case sample_epoch
wall_ns cpu_ns process_cpu_ns process_user_ns process_system_ns
process_peak_rss_bytes exit_code
```

CLI/startup have cycle0; warm cycles are1..2N, odd same_bytes/even main_edit.
The rendered marker is TEKT_EPOCH_((cycle//2)%2), including cycle0=0. Each edit
alternates0↔1; the following same-byte write preserves it. sample_epoch retains
wall-clock provenance, not the rendered marker number. Saved publications are
`cli.pdf`, `startup.pdf`, `cycle-001-same_bytes.pdf`,
`cycle-002-main_edit.pdf`, etc., unique per process case directory.

Watch cpu_ns is a process-clock delta; process CPU/RSS fields repeat final wait4
totals for that process and are not per-cycle quantities. Wall includes
watch/event/debounce latency. No gain or cause follows from source inspection;
C supplies paired timings, calibration and outside-clock profiles/syscalls.

## Incremental physical-channel complement

After the general probes are installed, this separate step adds only the real
page-running layout scopes; it does not reinstall the prior overlays, alter the
collector or change any budget/policy. Apply to each already-probed copy:

```text
python3 apply.py BASE_RAM --variant base --physical-probes
python3 apply.py JOINT_RAM --variant joint --engine ENGINE_RAM --physical-probes
```

Do not combine with `--probes`. The step requires the existing tested collector
and refuses reapplication. It changes only the L3 page_running.rs source. The
same environment/flush/TSV schema and inclusive semantics apply.

`physical.candidate` surrounds Session::physical_candidate (the method beginning
at original page_running.rs:1390), including discovery, replay, indexing and
outline preparation. `physical.step.discovery` / `.replay` surround physical_step
according to its existing replay argument. `physical.layout.discovery` / `.replay`
surround each actual layout_with_page_running(...).into_result() call within
that step (original line1598), including unchanged argument preparation.
`event.physical.complete` / `.pending` count the actual borrowed Ok/Err outcome
before the original match, with ns0. These are observations of that outcome,
not an inferred global attempt or convergence count.

The physical layout timers nest in their matching step, and steps reached via
physical_candidate nest in candidate. They can occur under context.attempt /
pipeline.context, so they overlap font.context and the font acquisition scopes.
No font interval is subtracted blindly from a physical role. Actual call counts
are separate from callbacks.passes: replay may retain the same callback counter.
The legacy layout.round/pass labels remain independent; an outline using only
the physical channel can legitimately emit none of them. On completed calls,
complete+pending equals discovery-layout+replay-layout calls. RAII includes early
returns in candidate/step while each call guard closes before outcome handling.
