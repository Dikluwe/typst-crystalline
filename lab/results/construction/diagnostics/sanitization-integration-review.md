# Documentary sanitization integration review

## Scope and provenance

This diagnostic integrates the six assigned documentary batches: evaluation,
layout, introspection/columns, shell, export, and remaining infrastructure/wiring.
It is not an L0, a new product contract or evidence of repository-wide translation
completion. The working tree is uncommitted at HEAD
`6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`. Final checks started at
`2026-09-27T01:18:12Z`. Each batch's source and prompt fingerprints and historical
qualifications remain in its original report.

## Integrated deliveries

- `00_nucleo/diagnosticos/sanitization-eval-history.md`
- `00_nucleo/diagnosticos/sanitization-layout-history.md`
- `00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`
- `00_nucleo/diagnosticos/sanitization-shell-history.md`
- `00_nucleo/diagnosticos/sanitization-export-history.md`
- `00_nucleo/diagnosticos/sanitization-infra-wiring-history.md`

The reviewed hubs specify current coordination and route child behavior to its
exclusive owner. Historical execution narrative is archived separately. Source
inspection establishes mechanical ownership and existing behavior; it does not
renew language parity, conformance or historical approval. Source-backed defects
remain open even when prose has been translated.

An automated scan of the assigned prompt families found no missing explicit
canonical prompt/nucleus file references and no matched Portuguese section
headings or historical file-size expressions. This is a bounded lexical check,
not an exhaustive semantic translation certificate. Machine-readable
`Hash do Código` and `Núcleos Tekt` labels, identifiers and observable literals
remain unchanged where required by the parser or product contract.

## Lineage integration

Ownership and nucleus integrity passed before hash repair. The official repair
initially refused the corpus because reverse hash metadata was absent or outside
the canonical preamble. Introspection/columns metadata was moved before the first
blank line; missing metadata on existing exclusive test owners was restored from
SHA-256 of the current source bytes with the leading `//! @prompt-hash` line
excluded, following the repairer's canonical algorithm.

This prerequisite repair includes existing test owners in math/stdlib/entities
outside the six textual batches. Their contract prose was not sanitized by this
integration. The source owner was checked before inserting each reverse seal;
unowned documents were not assigned fictitious consumers. A present seal records
the current source/prompt relationship and does not retroactively prove approval,
independent test authorship or RED/GREEN history.

`crystalline-lint --fix-hashes --quiet .` then completed successfully. No nucleus
content or product logic was changed by the integration.

## Verification

- `crystalline-lint --checks v15,v26 --quiet .`: passed before repair.
- `crystalline-lint --fix-hashes --quiet .`: passed after metadata repair.
- `crystalline-lint --quiet .`: passed after resealing.
- `cargo build --quiet`: passed; compiler warnings remain.
- `cargo fmt --all -- --check`: passed.
- `git diff --check`: passed.
- No `.orig` or `.rej` files were found in the inspected documentation, layer and
  benchmark directories.

Behavioral tests and benchmarks were not rerun for this documentary/header-only
integration. Prior execution receipts belong to their recorded source state and
are not represented here as new results. No new performance or parity measurement
was produced, and no line-count metric was used.

## Open follow-ups

The batch reports remain the detailed evidence register. The following require
focused investigation and a separate contract/test/implementation cycle:

- Layout: raw whitespace; logical versus physical page references; floating
  Place offsets/scope; weak breaks; table semantics; clipping preservation; link
  suffix safety and rich running-content width.
- Introspection/columns: direct OutlineEntry allocation/extraction alignment;
  causal replay limits; callback and fractional-region coverage; possible loss
  of deferred footnote bodies at the segmented-column boundary. The latter is an
  inference, not a reproduced defect.
- Export: decorative-layer tagging; raster clipping/support limits; SVG line
  caps; ICC compression-failure envelope; version-dependent artifact treatment.
- Infrastructure: the unowned package-version specification/cache proposal;
  escaped quotes and signed numbers in BibTeX; zero image density; losing-download
  temporary-directory cleanup; incomplete historical evidence artifacts.
- Historical approval and independent-authorship claims remain qualified as
  described in the batch reports.

The root `00_nucleo/prompts/wiring.md`, entities/contracts, parser, stdlib and
other compiler families were not included in the six documentary assignments.
Their complete English sanitization is not asserted by this review.
