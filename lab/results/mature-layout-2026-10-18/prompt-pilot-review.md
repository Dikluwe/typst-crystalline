> Initial pilot draft, preserved as review evidence. The pending math/table labels
> in this draft were corrected after inspecting their approval receipts. Final
> review: `final-pilot-review.md`; final state: the active owner prompts.

# Mature-layout realization prompt pilot

Date: 2026-10-18. Read-only architecture and feasibility review preceded edits; parent marked the construction checkpoint ready at commit/tag `8ec827fd975bf14cd849bebb9ede245e49033337` / `tekt-construction-2026-10-18`. This report covers only `00_nucleo/prompts/compiler/realize/mod.md` and `00_nucleo/prompts/compiler/realize/context_occurrences.md`.

## Result and comparison limits

Both owners now use English prose, retain the existing machine header tokens and hashes, and have concise revision history. The realization owner retains its existing public API, behavior, phase, ownership and nucleus pins. The Context occurrence owner retains its ratified additive API and acceptance contract. No hash was resealed; the parent must seal centrally. No product source, tests, build, or git operation was changed/run by this pilot.

The representative focal unit is Context occurrence identity. At checkpoint commit `8ec827fd975bf14cd849bebb9ede245e49033337`, its source prompts were 580 lines (`mod.md`) and 79 lines (`context_occurrences.md`). At 2026-10-18T23:23:00Z, the edited working tree versions are 298 and 49 lines. The same checkpoint-to-working-tree command measured only these two prompt files, with `git diff HEAD --stat`: 188 insertions and 500 deletions. For a focal reading, the detailed Location-arena contract is now in the dedicated 49-line owner; the 298-line realization prompt retains only the coordinating boundary for it. These counts measure document extent, not cognitive cost or adequacy.

This is not an independent semantic comparison. No second executor was given an equivalent change task, no read-set or task-completion time was collected, and no independent clause-by-clause semantic reviewer has certified the English wording. The fact that text was translated and shortened does not prove equivalent interpretation. The parent should review the ledger and obtain whatever review it requires before treating the pilot as a validated context-cost improvement.

## Old-to-revised clause map

Original locations below refer to the checkpoint version at `tekt-construction-2026-10-18`, not the post-edit working tree. New locations are headings in the two edited prompts. Evidence paths are deliberately preserved as authored, including Portuguese directory names and construction paths; relocation remains the parent's task.

| Original clause(s) | Revised clause(s) | Status/disposition |
|---|---|---|
| `mod.md` lines 18–29: measurement, vanilla reference, domain decision | `Measured basis and domain decision` | Retained. Historical pin and source paths remain verbatim. |
| lines 30–61: facade, canonical types, prior snapshot, entity/eval ownership and transport | `Public boundary and ownership` | Retained; public signatures unchanged. |
| lines 62–77: identity, realization, indexing and layer coordination | `Identity, realization, and coordination` | Retained. |
| lines 78–89: language/execution acceptance and paired performance evidence | `Observable acceptance and performance` | Retained. |
| lines 90–104: insertion identity, textual recipe split, visual normalization, partial replay, Context IDs | `Occurrence normalization and index construction` | Retained. |
| lines 105–112: page wrapping and RuleId reservation | `Reuse the realized index — 2026-10-05`; `Captured rule IDs — 2026-10-06` | Retained with delegation/phase limits. |
| lines 113–199: D06 automatic math insertion, evidence, facade, gate, contextual snapshot, detached errors, no-math guard | `Proposal: automatic math insertion (ADR-0127 gate pending)` and subsections | Still explicitly pending; no proposal promoted to accepted behavior. All cited evidence paths and API text retained. |
| lines 200–263: D08B3-A table proposal, evidence, proposed facade, split scope and measured refinement | `Proposal: D08B3-A table resolution at insertion (ADR-0127 gate pending)` and subsections | Still explicitly pending; incomplete Cell/Table scope remains explicit. No new approval implied. |
| lines 264–271: D08B3-C2 ratification and private predicate helper | `Ratified D08B3-C2 responsibility` | Retained as a separate ratified scope. |
| lines 272–277: borrowed preflight decision later superseded | `Internal content traversal and allocation decisions` → `Finalizing occurrences without a change — 2026-10-09, superseded below` | Retained and explicitly marked superseded, including its limits and acceptance. |
| lines 278–285: Sequence reuse/no-reconstruction decision | same section → `Reuse during insertion normalization — 2026-10-10` | Retained. |
| lines 286–300: single traversal decision superseding preflights, measurements and acceptance | same section → `One traversal after change-proof propagation — 2026-10-10` | Retained; precedence and supersession explicit. |
| lines 301–353: Clippy arity measurement, C020 exception prescription, limits, materialization boundary | `Boundary arity — clippy::too_many_arguments` | Retained, with C020-only exception scope and pending fixes preserved. |
| lines 354–366: materialized named exceptions | same section → `Materialized named exceptions — 2026-10-16` | Retained, including counts/gates and exclusions. |
| lines 367–412: R2-S measurement, approval, private Sequence owner, acceptance and activation shortcut | `R2-S — nominal Sequence realization, ratified 2026-10-17` | Retained as ratified; public-contract and phase limits preserved. |
| lines 413–506: R3-T1 measurements, approval/manifest, causal realization, acceptance and three refinements | `R3-T1 — causal realization, ratified 2026-10-18` and `R3-T1 refinements` | Retained as ratified; internal refinements remain scoped, including raw contribution handling and V17. |
| lines 507–550: Context textual inheritance measurement, internal parity decision and snapshot capture | `Context — lexical inheritance, 2026-10-18` | Retained under existing R3-T1 scope; no API/phase change. |
| lines 551–580: Context occurrence measurement, approval, exclusive responsibility | `Context — occurrence identity, ratified 2026-10-18` in `mod.md`; full contract remains in `context_occurrences.md` | Retained as ratified, with coordination in `mod.md` and the complete API/arena/acceptance in its exclusive owner. |
| `context_occurrences.md` lines 1–29: owner identity, evidence, approval and gate | `context_occurrences.md` → `Context — ratified occurrence identity, 2026-10-18` / `Measurement before decision and gate` | Retained. |
| lines 30–71: arena API, identity reservation, slots, producer/frame lifetime, restoration/error/unwind rules | `Exclusive responsibility` | Retained; API and all explicit exclusions preserved. |
| lines 72–79: acceptance gates | `Acceptance` | Retained. |

## Evidence and approval artifacts that must remain directly recoverable

The L0 keeps its original references as requested. When the parent maps the construction tree, these items need real retained files at the new locations so a reviewer can inspect the approval or supporting contract, rather than only an archive index:

- Context occurrence approval: `00_nucleo/materialization/context-occurrences-2026-10-18/review.md`, `approval-receipt.json`, and its `.gz` copy. The exact receipt is the gate evidence for the additive API in `context_occurrences.md` and `mod.md`.
- Math proposal gate (still pending): `00_nucleo/materialization/math-bare-2026-10-06.md`, its `manifest.json`, candidate L0 snapshots under `l0/`, and the pinned nucleus snapshot under `nuclei/math/bare-insertion.toml`. The retained diagnostics alone do not satisfy the human-confirmation gate.
- Table D08B3-A gate (still pending): `00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md` and its `manifest.json`, including candidate L0/nucleus material. Do not label this as approved based on the separate D08B3-C2 approval.
- Table D08B3-C2 ratification: `00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md` and `manifest.json`, alongside the referenced baseline. The prompt records the human `confirmo`; this contract is the inspectable scope artifact.
- Sequence R2-S approval: `00_nucleo/materialization/sequence-realization-2026-10-17/review.md`, `manifest.sha256`, and `sources.json.gz`, plus the prompt's referenced diagnostics.
- Textual realization R3-T1 approval: `00_nucleo/materialization/textual-realization-2026-10-18/review.md`, `approval-receipt.json`, `manifest.json`/`manifest.sha256`, `acceptance.json`, and source snapshots. The prompt retains manifest hash `47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`.
- Context diagnostics referenced by both Context sections: retain the full `00_nucleo/diagnosticos/context-paridade-2026-10-18` evidence set, including `before.json.gz`, `before-sources.json.gz`, and `baseline-build-resources.json`, and its receipt/approval evidence as applicable.

Many other evidence records are referenced directly in the table/traversal/Clippy sections. Preserve the old paths in the path manifest and keep every referenced record accessible; this pilot did not relocate them or claim their archive is complete.

## Worktree scope and validation

Checkpoint HEAD is `8ec827fd975bf14cd849bebb9ede245e49033337`. At 2026-10-18T23:23:00Z the two owned prompts were modified and no other files were included in this pilot's diff. The parent had other concurrent branch changes. No build, test, lint, hash reseal, product source edit, commit, or publication was performed here. The next appropriate review is semantic and lineage review after central resealing, followed by the parent's chosen validation; this report makes no test or conformance claim.
