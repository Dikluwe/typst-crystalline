# L0 — CSL bibliography plan
Hash do Código: bb42d4da

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:5c68aee638a285b125399ae9198883a90ae27a0c3a89f68e0765b86fa84b8197

Layer: L1. Exclusive consumer: `01_core/src/compiler/layout/bib_csl/plan.rs`.
Architecture: ADR-0107/0108/0109/0127/0129/0130.

### Evidence before specification

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/before.json.gz` records 12 pairs, sources/argv/UTC/HEAD/status/diff HEAD --stat, text/boxes/binaries; before-sources.json.gz preserves prior sources/L0s. HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree; Tekt SHA-256 `c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899`; oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. IEEE12 places A. at x25.124 versus 29.924 and second entry y89.936 versus 112.232. IEEE18 uses x33.686 versus 40.886. In prefix-free CSL, second entry starts at y184.424 versus 191.024. par.spacing0/2em probes separate margins from within-line spacing. Adjacent-citation grouping also differs (MLA Silva/Doe), outside list composition: preserve that RED explicitly and do not claim general parity from list equality alone.

`bib_csl.rs:342–355` concatenates first_field, literal space, content and Linebreak, losing entry boundaries/hanging_indent. `convergence.rs:58–82` carries only public BibRenderCache; `bibliography.rs:107–151` emits one Block. Pinned `typst-layout/src/rules.rs:463–531` composes two Auto columns with 0.65em gutter and par.spacing row-gutter, or one Block per prefix-free entry; authored Rust form is not required. Observables are position, order, content/style and scope. Hypothesis: real boundaries/prefix transport correct columns/separation without reformatting; font, order, empty fields, no-prefix, zero/2em, show-grid and style controls can refute it.

`lint-before-fixture-fix.json.gz` measures a new V19 note on the Columns|Hanging|Plain projection match. This is legitimate public adaptation: legacy projection keeps one format across modes while private physical composition consumes their distinction.

## Ownership and contract

Define layout-private entries {prefix: Option<Content>, body: Content} and discriminated Columns/Hanging/Plain plans, each containing ordered entries. Columns takes precedence if any prefix exists, even empty; otherwise use CSL's hanging_indent flag. Convert nodes using parent helpers, retaining wrappers/links without reformatting. Expose conversion and legacy_content projection to the parent: for every entry, prefix if Some, literal space if Some, body, Linebreak, exactly matching previous public rendering, including absent prefix and empty list. The plan is a layout artifact, never a public field/language entity; it contains no I/O, clock, environment, callbacks or synthesized Location. The shared pin governs transport.

This internal parity correction follows ADR-0127 continuous flow within layout, without a new public API/type, flag/default, compatibility or phase. L0 precedes tests/code. Structural plans preserve hanging in their discriminant; physical hanging, per-occurrence overrides and simple/optimized/RTL composition belong to the separately specified transport/paragraph-indent work. Do not force greedy fallback to bypass missing initial-advance preparation. Title margins, complete CSL, multiple-bibliography attribution/cache and adjacent-citation grouping are not certified by structural-list work.

## Acceptance and limits

Require functional column RED with relative gutter/interentry margin before implementation. Controls cover public cache (prior Content/signatures), prefix-free CSL, hanging/plain mode preservation, empty input, order/citations and rollback. Bilateral PDF compares list boxes/text while preserving indent/citation-grouping divergences in the report. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26 and reverse lineage; SARIF has no new error/warning. Reseal only after valid ownership/graph. Use MemoryMax12GiB/High8GiB/swap0, jobs1/debug0/incremental0, sequential heavy commands; no commit/push/release.

Test projection with Columns and nonempty Plain/Hanging plus empty input; one passing mode does not prove all three. Tests may parameterize hanging false/true without duplicating assert bodies. Preserve informative V19 rather than suppressing it or artificially splitting identical projection to hide grouping.
