# L0 — page geometry
Hash do Código: c6e66e62

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_geometry.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_geometry.rs:103`, `01_core/src/compiler/layout/page_geometry.rs:137`, `01_core/src/compiler/layout/page_geometry.rs:9`, `01_core/src/compiler/layout/page_geometry.rs:56`, `01_core/src/compiler/layout/page_geometry.rs:72`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/correcao-auto-espacos-2026-09-29/before/results.json`, provenance.json and initial.json measure 30 sources/60 PDFs, font-path, commands, UTC, HEAD, working tree and binaries against historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. Literal spaces only (`#text("   ")`) with tracking 0/±0.2 produce Tekt `/MediaBox [0 0 0 0]` (Poppler displays 612×792), versus vanilla 9.36/9.76/8.96×8.568pt. Final literal separators after text lose width on auto pages; initial separators followed by text already match. A18pt-size scope ends at 14.44×12.852pt in vanilla for+0.2pt. Empty text and inkless markup expose a separate baseline/minimum gap; terminal newline has a separate vertical divergence, not closed here.

Source causes: `space.rs:22-35` does not transport initial Literal without tracking; `page_geometry.rs:104-135` uses `helpers::line_content_right`, correctly filtering all TextSeparator for RTL alignment, while a carrier-only line lacks typographic top; `builder.rs:291` and `builder/fonts.rs:633,1167` write zero MediaBox. Vanilla `crates/typst-pdf/src/convert.rs:108-118` enforces physical 3×3 minimum at PDF output. This supports separate auto geometry from RTL alignment and a shared exporter minimum; do not infer historical intent from older scope.

The later baseline refuter `lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/after-legacy33/results.json` shows `#text("  ")` at 12pt with 17.136pt Tekt versus 8.568pt vanilla after closing a Literal-only line: `page_geometry.rs:180-201` adds text top to pos.y although `text_baseline::admit` has already moved the carrier to its admitted baseline.

Arity evidence uses HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, captured 2026-10-15T16:39:18.873477+00:00. Full sources/prior L0/signatures/hashes/UTC/status/git diff HEAD --stat are in `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`; individual classification and predecision owner clauses in classification.json. Clippy spans were reconfirmed against inventory sources. C011 at `01_core/src/compiler/layout/page_geometry.rs:9`, apply, has 8 counted arguments against threshold 7. This measures Rust structure, not parity/functional failure; maintenance judgment is supported by the stated data, not arity alone.

The validated `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` preserves 79 warnings. Current source/span/reason/signature/hash reconfirmation is in `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, with HEAD, uncommitted tree, UTC and git diff HEAD --stat. Functions/concrete data were read before decision. Arity/type heuristics are mechanics, without new parity or measured cognitive benefit.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Apply typed page geometry to PageConfig: explicit width/height override paper dimensions, omitted axes retain prior base dimensions, and flip swaps those bases. Preserve binding, direction and folded margin precedence. The apply helper initially resolves margins at physical page one; subsequent physical-page resolution remains in the existing regional transition path. Install no page, evaluate no callbacks and do not own a regional transition.

For auto width, derive rightmost material extent from emitted and pending items using `00_nucleo/prompts/core/compiler/layout/helpers.md`, include deferred equation logical widths and the current numbered-equation reservation, then add right margin with a minimum of total horizontal margins. The numbered reservation uses the equation width plus both number/gutter sides, with the existing 0.5em gutter. For auto height, use the causal last_block_descent_y or current cursor fallback plus bottom margin, with a minimum of vertical margins. These are final geometry projections, not arbitrary shrink-to-ink promises. `00_nucleo/prompts/core/compiler/layout/page_commit.md` and the hub call them at their existing composition points.

For width:auto, combine existing visual extent with causal extent of emitted/pending Literal TextSeparator (pos.x + advance), preserving equation reservations/margins. Do not alter helpers::line_content_right used by RTL alignment or use a global cursor counting omitted spaces from other lines.

For a page containing only Literal carriers without visual material, use the admitted baseline pos.y as causal height extent, preserving the greatest confirmed bottom and margins. This supersedes the earlier proposal to add the carrier style's FontMetrics.text_edges top after baseline admission: do not add top twice. Material pages retain prior height. Do not change RTL alignment or auto width. The physical PDF minimum belongs to exporter owners, never this geometry unit.

Preserve apply signature/order/semantics. This pure geometry applier receives only six geometric overrides, direction and mutable PageConfig; receiving an entire PageRun would bring unrelated body/numbering, while None presence and flip/binding must stay explicit. The nominal C011 exception is authorized and materialized only on this function: `#[expect(clippy::too_many_arguments, reason="C011: Aplicador puro de geometria recebe apenas os seis overrides geométricos, direção e PageConfig mutável")]`. Preserve body/calls/cfg/unrelated attributes. Do not annotate module/impl/crate/global config, raise threshold or cover another lint. If arity stops exceeding threshold, remove/review the expectation; never silence unfulfilled_lint_expectations.

The nominal exception supersedes generic suppression prohibitions only for this identified item/lint. Original audit-only materialization restrictions are superseded only by the later authorized direct annotation. Entries classified for correction, including conditional C047/C066 in the wider package, remain pending and unannotated; a residual exception where dead-argument removal is required depends on that correction. Other correction obligations remain.

Metadata-only continuous flow under ADR-0127 changes no public contracts, executable body, cfg, defaults, phase, dependencies or architecture. Private-signature corrections require caller/L0 migration before code. A newly required public contract/external compatibility change requires concrete ADR-0127 confirmation rather than anticipatory authorization here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

For auto geometry require RED→GREEN on spaces 0/± and PDF minimum, causal bilateral cases, prior corpus, build/lint without errors and resealing strictly within the manifest. Explicit empty text and terminal newline remain separate pending work.

For C011, confirm warning in workspace/all-targets and no unfulfilled_lint_expectations after annotation; preserve calls/functional controls. L0 precedes code, restricted resealing, and removing added attributes plus lineage header must recover prior rustfmt-normalized source. Measure RED first; afterwards check all-targets inventory: exactly 46 arities and five test warnings satisfied by 49 expectations, no new warning, unfulfilled_lint_expectations as error. The 11 arities requiring correction stay visible. Validate build, formatting, strict/reverse lineage, Tekt without new errors/warnings, existing Counter tests and coverage under 12GiB, zero swap and one job. The prior full suite remains evidence for unchanged executable code; do not create tests mirroring lint attributes or demand new performance comparison for metadata. Corrections requiring code retain owner language/integration and call before/after controls without relaxed fixtures/tolerances. Preserve layers, phase, defaults, static dispatch and no mutable global state.
