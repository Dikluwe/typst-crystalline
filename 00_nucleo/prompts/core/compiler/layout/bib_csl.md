# L0 — bib csl
Hash do Código: 91e99dd9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:5c68aee638a285b125399ae9198883a90ae27a0c3a89f68e0765b86fa84b8197

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bib_csl.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bib_csl.rs:39`, `01_core/src/compiler/layout/bib_csl.rs:47`, `01_core/src/compiler/layout/bib_csl.rs:58`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/before.json.gz` records 12 pairs, sources/argv/UTC/HEAD/status/diff HEAD --stat, text/boxes/binaries; before-sources.json.gz preserves prior sources/L0s. HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree; Tekt SHA-256 `c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899`; oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. IEEE12 places A. at x25.124 versus 29.924 and second entry y89.936 versus 112.232. IEEE18 uses x33.686 versus 40.886. In prefix-free CSL, second entry starts at y184.424 versus 191.024. par.spacing0/2em probes separate margins from within-line spacing. Adjacent-citation grouping also differs (MLA Silva/Doe), outside list composition: preserve that RED explicitly and do not claim general parity from list equality alone.

`bib_csl.rs:342–355` concatenates first_field, literal space, content and Linebreak, losing entry boundaries/hanging_indent. `convergence.rs:58–82` carries only public BibRenderCache; `bibliography.rs:107–151` emits one Block. Pinned `typst-layout/src/rules.rs:463–531` composes two Auto columns with 0.65em gutter and par.spacing row-gutter, or one Block per prefix-free entry; authored Rust form is not required. Observables are position, order, content/style and scope. Hypothesis: real boundaries/prefix transport correct columns/separation without reformatting; font, order, empty fields, no-prefix, zero/2em, show-grid and style controls can refute it.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Construct pure citation/bibliography caches from supplied BibEntry values, built-in styles or a supplied parsed IndependentStyle, locale and optional citation order. Preserve the Normal/Prose/Author/Year mappings and the existing unsupported-form behavior. Built-in style lookup uses the archive and the cache builder's `ieee` default; the hub decides whether a cache is requested at all. Parse supplied custom CSL XML without filesystem or URL access. Convert rendered CSL nodes into Content while preserving supported links, strong/emphasis, small caps, underline and block breaks. Serialize textual YAML scalars through total JSON-string escaping so controls, Unicode, empty and YAML-looking strings remain text. Normalize a maximal run of at least two ASCII hyphens in the pages field to one hyphen; single hyphens, Unicode dashes, and other fields are unchanged. Preserve diagnostics for malformed styles/entry conversion and the existing locale fallback.

Declare private submodule plan. A private builder prepares BibRenderCache and Option<Arc<plan::BibliographyPlan>> together using the same existing CSL execution. Both public facades preserve signature, diagnostics, citations and legacy Content, extracting only cache from the private result. RenderedBibliography→typed-plan conversion belongs exclusively to `00_nucleo/prompts/core/compiler/layout/bib_csl/plan.md`; delegate existing ElemChild/ElemChildren converters through descent. Do not render CSL again or infer boundaries from strings.

This internal parity correction follows ADR-0127 continuous flow within layout, without a new public API/type, flag/default, compatibility or phase. L0 precedes tests/code. Structural plans preserve hanging in their discriminant; physical hanging, per-occurrence overrides and simple/optimized/RTL composition belong to the separately specified transport/paragraph-indent work. Do not force greedy fallback to bypass missing initial-advance preparation. Title margins, complete CSL, multiple-bibliography attribution/cache and adjacent-citation grouping are not certified by structural-list work.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require functional column RED with relative gutter/interentry margin before implementation. Controls cover public cache (prior Content/signatures), prefix-free CSL, hanging/plain mode preservation, empty input, order/citations and rollback. Bilateral PDF compares list boxes/text while preserving indent/citation-grouping divergences in the report. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26 and reverse lineage; SARIF has no new error/warning. Reseal only after valid ownership/graph. Use MemoryMax12GiB/High8GiB/swap0, jobs1/debug0/incremental0, sequential heavy commands; no commit/push/release.
