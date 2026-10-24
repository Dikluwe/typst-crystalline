# PDF text-box precision — batch F, 2026-10-23

The original strict PDF text-box failures are corrected in the finite controls
below. Font metrics, TJ compensation and compact origins retain fractional
precision. No layout coordinates were changed to compensate for an observer.
Table border painting remains a separate measured limitation.

## Exact state and provenance

All measurements used HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`
with an **uncommitted working tree**, not HEAD alone. The
[baseline](../../lab/results/pdf-bbox-precision-2026-10-23/baseline.json)
was frozen at 20:54:38 UTC and matched every input of the validated E state.
The [final state](../../lab/results/pdf-bbox-precision-2026-10-23/final-state.json)
contains the exact UTC, `git diff HEAD --stat` changed-file list and all input
hashes; the [source archive](../../lab/results/pdf-bbox-precision-2026-10-23/final-source.json.gz)
and [patch relative to the dirty baseline](../../lab/results/pdf-bbox-precision-2026-10-23/final-change-vs-dirty-baseline.patch)
make this batch independently recoverable. The
[validated state](../../lab/results/pdf-bbox-precision-2026-10-23/validated-state.json)
adds only the controlled snapshot references and operational MANIFEST; productive
Rust and L0 inputs remain identical to the measured final state. Only five L3 consumers, their five
owning L0 prompts and one shared Nucleus changed. Existing L1/L2/L4 and other
L3 changes were preserved. No commit, reset, resync or publication was performed.

Final product: `lab/results/pdf-bbox-precision-2026-10-23/tekt-final-debug`,
SHA-256 `1a927eb7ab98ca11c784387cc333b81c860a7c1a3398436c4b4840228465e078`.
The earlier `tekt-debug` remains the first candidate, not the final binary.
Oracle: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
[Completion receipt](../../lab/results/pdf-bbox-precision-2026-10-23/completion.json)
records observers, fonts, commands, input invariance and final validation.

## Measurement before decision

The [before receipt](../../lab/results/pdf-bbox-precision-2026-10-23/before.json)
reproduces Hello xMax error 0.003652 pt and Table xMax error 0.007417 pt.
Baseline `03_infra/src/export/fonts.rs:223` rounds /W to integers; `03_infra/src/export/stream/text.rs:223–245`
rounds TJ advance differences, while `03_infra/src/export/stream/text/runs.rs:207–248` rounds item boundaries
and the reconstructed cursor. The retained Hello PDF has local /W 752 versus
oracle 751.9531, and local TJ 59 versus oracle 58.59375. Those bytes locate a loss;
they are not the acceptance criterion or evidence of upstream intention.

The [width-only diagnostic mutation](../../lab/results/pdf-bbox-precision-2026-10-23/causal-width-only.json)
still fails: correcting /W alone does not correct the paired spacing model.
These modified PDFs are diagnostic copies, never qualified product outputs.
The inference that the combined export quantization explains text selection is
supported by final independent word/page boxes and SVG positions; residual
strict box errors would have refuted completeness. The original benchmark and
all failed receipts are retained with their original thresholds.

An additional [compact comparison](../../lab/results/pdf-bbox-precision-2026-10-23/compact-candidate1.json)
found math xMin 0.0076 pt and yMin 0.0346 pt differences from the default mode.
Compact direct Glyph origins used one-decimal Td, shaped origins three decimals,
and plain Text one decimal; verbose placement already used five decimals.

## L0, correction and RED→GREEN

ADR-0127 continuous flow applies: internal export precision correction, no new
public contract, default, flag, pipeline phase or compatibility surface.
Owning L0 prompts were updated before each implementation. The existing
`pdf-shaped-advances` Nucleus now requires consistent fractional widths and TJ
positions, with valid pins in its three consumers' owning prompts.

The production changes remove integer quantization from effective-face /W,
single/coalesced TJ advances, offsets, tracking residue and run boundaries.
The run cursor uses the serialized fractional advance. Compact text/shaped/Glyph
Td uses five decimal places, preserving its operator structure. Face selection,
subsetting, Unicode, supplied layout positions, bitmap handling, font-size policy,
Tc=0, graphic and ActualText scopes remain under their existing owners.

[RED](../../lab/results/pdf-bbox-precision-2026-10-23/red.log): three new regressions
fail for UPEM other than 1000 width, remapped glyph advance/offset and absolute coalesced
boundary. [Compact RED](../../lab/results/pdf-bbox-precision-2026-10-23/compact-red.log):
a fourth regression fails on `12.3 23.5 Td` for the requested
`12.34567 23.45678` origin. [Final focused GREEN](../../lab/results/pdf-bbox-precision-2026-10-23/complete-green.log):
all 604 exporter regressions pass.

The legacy P979 test claimed an exact 110 pt origin while requiring integer -233,
which places it 0.004 pt short. It now independently interprets TJ and checks 110 pt
within 1e-7 pt. Two compact envelope/link tests retain their coordinates and
operators with updated decimal spellings. No original geometric tolerance was
raised and no test was disabled.

## Finite output acceptance

The original 0.001 pt tolerance is unchanged; measured coordinates are not rounded
or normalized to qualify outputs.

| Control | Before | Final |
|---|---|---|
| Hello | xMax 0.003652 pt, failed | maximum box delta 0.000001 pt, pass |
| Table, both pages | xMax 0.007417 pt, failed | maximum box delta 0.000010 pt, pass |
| 16 prose controls | earlier full-box failures retained | all full word/page boxes pass |
| 16 math controls | prefix-text xMax 0.002191 pt failed | all full word/page boxes pass; prefix xMax delta0 |
| Compact vs default: Hello/Table/prose/math | math failed | all boxes pass; maximum 0.000001 pt |

[Final primary receipt](../../lab/results/pdf-bbox-precision-2026-10-23/final.json),
[32-control summary](../../lab/results/pdf-bbox-precision-2026-10-23/preservation-summary-final.json),
[34 page-bound controls](../../lab/results/pdf-bbox-precision-2026-10-23/page-bounds-final.json)
and [compact receipt](../../lab/results/pdf-bbox-precision-2026-10-23/compact-final.json)
retain complete coordinates, text/page membership and exact binary identities.
The maximum across the 32 prose/math word-box controls is approximately 0.000020 pt.
Eight weak-pagebreak/context/metadata controls preserve complete per-page text.

[Independent SVG and painting observations](../../lab/results/pdf-bbox-precision-2026-10-23/geometry-paint-final.json)
cover all Hello/Table SVG pages: glyph origins agree to approximately 1e-9 pt.
Hello first-page painting is identical at 300/600 dpi. Table first-page painting
is identical at 300 dpi but retains 1548 differing pixels at 600 dpi, the same before
and after F. [Compact painting](../../lab/results/pdf-bbox-precision-2026-10-23/compact-paint-final.json)
is identical to default at 600 dpi on the first page of all four controls.
These finite observations do not prove arbitrary-resolution equality.

## Remaining border painting boundary

Table has repeated horizontal stroke segments locally, with 42/26 SVG segments
versus 24/16 in the oracle; its glyph origins agree. In a
[diagnostic copy of each PDF](../../lab/results/pdf-bbox-precision-2026-10-23/table-boundary.json),
disabling stroke painting produces identical text-only first-page images at 600 dpi.
All 1548 original differing pixels lie near the outer borders. The boundary is
therefore stroke painting rather than font/text-box precision. Duplicate segments,
paint order, coordinate serialization and opposite path transforms are investigation
anchors; this experiment does not isolate which mechanism causes every edge pixel.

This painting failure remains a failure. It is not silently qualified by the
successful boxes, SVG text or diagnostic stroke removal. Border rendering needs
its own L0-backed investigation. The seven additional symbol-render limits and
unrelated reflection/field limits recorded in [batch E](symbol-identity-2026-10-23.md) remain outside this batch;
F makes no general language, PDF/A, PDF/UA or performance claim.

## Controlled snapshot maintenance

The first full run exposed five binary snapshots expecting the old Compact
serialization. The current owning L0, `00_nucleo/prompts/infra/pdf_snapshot_tests.md`,
already authorizes intentional reference maintenance through its controlled
harness. No harness code, assertion, source fixture or font was changed.
Only 01/02/03/07/09 were regenerated, using Compact/Disabled/epoch 0, twice with
identical hashes. All nine tests then passed without UPDATE; the other four
reference PDFs remained byte-identical. The operational MANIFEST was refreshed.

[Regeneration](../../lab/results/pdf-bbox-precision-2026-10-23/snapshots/regeneration.json)
and [independent controlled comparison](../../lab/results/pdf-bbox-precision-2026-10-23/snapshots/comparison.json)
retain old/new bytes, exact inputs, commands and coordinates. A quarantined
probe compiles the same World/source/fonts in both modes; its Compact outputs
match the new references exactly. All five new references have zero word-box
delta from Verbose and identical first-page 600 dpi painting. Old references
retain their measured painting differences against that same default. This
is justified local snapshot maintenance, not byte parity with vanilla.

## Complete validation

All 12 final jobs pass in one frozen input state, with 2,162 inputs unchanged:
Python CI tests, layout, strict/reverse lineage, workspace build, domain/infra and
wiring tests, doctests, all-target build, Clippy with warnings/expectations denied,
fmt and rustdoc with warnings denied. Rust totals: **9,849 passed, 0 failed, 7
pre-existing ignored** (domain/infra 9,589; wiring 255; doctests 5).

[Final command receipts](../../lab/results/pdf-bbox-precision-2026-10-23/validation-complete/validation-commands.json)
and [freeze](../../lab/results/pdf-bbox-precision-2026-10-23/validation-complete/validation-freeze.json)
retain exact argv, environment, UTC, source and each result. The complete
`crystalline-lint .` finishes with zero errors/warnings (existing informational
notes remain), and `git diff --check` passes. All heavy builds/tests use one job,
12 GiB maximum, 8 GiB high and zero swap. Validation after reference maintenance
uses the same productive Rust/L0 state as every final output measurement.

The first candidate validation was intentionally stopped during wiring compilation
after six successful gates when the compact precision failure was discovered.
Its logs, source, binary and [stop receipt](../../lab/results/pdf-bbox-precision-2026-10-23/candidate1-validation-stop.json)
remain separate; it is not credited as complete validation. Intermediate stale
integer/decimal-spelling test failures are also retained.

The subsequent full run retained five snapshot failures. Their exact old/new
bytes and intended positional correction were audited before regeneration;
that failed run is not credited as final validation either.
