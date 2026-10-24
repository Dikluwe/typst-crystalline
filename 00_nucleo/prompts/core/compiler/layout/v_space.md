# L0 — v space
Hash do Código: eb5465b8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/v_space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/v_space.rs:17`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, diffstat/UTC/sources/binaries/argv and historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. All 24 root/Box pairs after 12 prefixes differ in marker: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, but Tekt uses 15pt. Rotate also has independent visual overlap. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` sees only still-pending line. Pinned `flow/distribute.rs:703–715` uses physical flow offset for axis-unspecified Place. This is language geometry, not Rust structure. Weak margin, overlap, new page, rollback or subframe leakage can refute the hypothesis.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/shape-terminal-complete/results.json.gz` records the same HEAD, uncommitted tree, status/diffstat/UTC/sources/binaries and historical vanilla pin 9f2b6e871. On width100/height Auto/margin 10 with rect20×10, Tekt height is 43.2 versus pin30; second of two shapes y25.962 versus 33.2; text baseline after Shape 20.438 versus 40.438; Shape→Rotate top13.2 versus 33.2; Rotate→Shape height 30 versus 53.2. `shape.rs:196–266` subtracts cap-height from geometric top, while `shape.rs:289–304` publishes neither prev_line_baseline nor last_block_descent_y. `cursor.rs:90–110` and `transform.rs:352–356` consume these fields. `v_space.rs:22–31` publishes only Place anchor rather than confirmed terminal bottom. Pinned `flow/collect.rs:253–285` and `flow/distribute.rs:302–318` distinguish frame from terminal weak margin. Geometry is observable; Rust/buffer strategy is not a parity requirement.

The extension `shape-flow-before/results.json.gz` measures strong VSpace between Shape and Shape/text and after Rotate: pin adds 7pt/30pt to the end while Tekt loses it in baseline/bottom consumers. Weak-between-item controls show collapse priority is still absent.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve the element's amount against the active font size. Close a horizontally pending line using the ordinary hook, then advance cursor_y by that amount. This adapter does not add a separate collapse algorithm, float policy or page transition.

After existing flush, preserve the certified boundary (or cursor if absent), add amount and record with the post-advance cursor. Do not include hypothetical closed-line leading in physical end.

After computing physical end and recording the existing anchor, update an already-confirmed terminal bottom to that end only for strong spacing. Do not add weak block margin to explicit advance or create a bottom where the prior path had none. Weak spacing does not replace confirmed bottom. While block_chain_active remains true, a strong VSpace also shifts prev_line_baseline by the same amount; weak gap remains separate. Do not apply a generic max rule.

The internal ADR-0127 correction authorizes only private state required by this contract, superseding no-new-state restrictions from earlier extractions only to that extent. LIFO save/restore is mandatory. No new public field/signature, product mode/flag/default or phase.

The certified-anchor slice (historical B1) covers line, shape, Block, Align, Grid, Raw and VSpace producers; it does not complete the whole system. Block/inset region completion (historical B2), late settling of a pending inline Box/Transform line and audits of Heading/List/Stack/Equation/Columns (historical B3) remain completion boundaries before universal Place certification. Invalid witnesses retain the legacy route and count as pending work rather than parity credit. The public tiling gate remains pending.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

L0 precedes RED→GREEN and restricted resealing. Revalidate complete suite, HTML/context, PDF/SVG, build/doc/fmt/check, lineage and warning inventory, with the bilateral corpus. This correction does not certify general VSpace collapse, negative/fraction acceptance, shape defaults, RTL alignment or Block Auto. Weak-collapse gaps and the broader vertical-flow work remain open; this contract does not close those axes.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Record actual explicit vertical-spacing admission as unpainted page material outside isolated subframes, including zero and weak spacing. The page admission witness does not implement or modify geometric spacing collapse.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.
