# L0 — text_tracking
Hash do Código: 56666dcc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_tracking.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Inspection of `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` and `space.rs:162` found textual-boundary, baseline-admission and initial-space integration-test responsibilities in other owners. Exact state, sources and prior prompts are in `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` and `before/`; the measured correction is in `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`. Separation preserves those results and limits. Positions/text/styles are language observables; function location is mechanics, proving neither intent nor new bilateral parity.

`lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` and `summary.json` record 30 sources/90 PDFs, commands/sources and exact state in initial.json and before-pdf/provenance.json against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Repeated equal scopes accumulate about 0.4pt, nested scopes 0.8pt; positive wrap changes the line. `text_tracking.rs:50` compares only resolved TextStyle. Vanilla `inline/collect.rs:287-295` retains distinct-chain segments; `inline/shaping.rs:1269-1295` adds tracking only within each run. These are observable boundaries, without requiring copied PartialEq/chain representation. Hide/Box also form boundaries: `hide.rs:21-24` and `boxed.rs:157-208` replace/restore buffers without preserving that distinction.

`lab/results/construction/diagnostics/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` and `initial.json` record 66 inputs/198 PDFs against the same historical pin, with complete sources/commands/binaries/HEAD/working tree. In dir:rtl, three Latin terms lack four intervals: at ±0.2pt the first term differs by 0.8pt, while 0pt matches. Explicit Arabic differs by 0.4pt in the two interword intervals at ±0.2pt. `text_tracking.rs:65` rejects RTL unconditionally; `word_layout.rs:34,72` and `space.rs:25` already project/add the signed boundary; `cursor.rs:150-186` anchors by real right edge. Vanilla `inline/shaping.rs:1269-1295` applies tracking before each next distinct cluster also in negative direction. This measures positions/line choice rather than intent. Hebrew dir:rtl and initial literal space differ even at 0pt and do not demonstrate a boundary defect. Ligatures, mixed direction and global pagination need separate probes.

The same diagnostic's `after-pdf/results.json` and `summary.json` measure 66 sources/198 PDFs, with source/pin/state in provenance.json and exact pre-second-correction state in wrap-initial.json. In latin-wrap ±0.2/0.333/1.2pt, first/second Tekt lines shift by exactly one ±t interval versus vanilla; final line, text and line choices match. `line_commit.rs:241-248` knows continuation, but `:477` calls `cursor.rs:150` without it; `cursor.rs:164-177` anchors by actual right edge. Wrap/hyphenation controls showing another shift would refute the pending-terminal-interval inference. It establishes no universal mixed-bidi or initial-space rule.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

V17 structural evidence T192 is at `01_core/src/compiler/layout/text_tracking.rs:98`. Existing sources/L0 were read first; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, status and git diff HEAD --stat are in `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. The hash-reconfirmed inventory has 28 guards in 21 owners; pinned CI requires splitting or moving the condition into the body. This is Rust structure, not functional-defect, improved-AI-readability, upstream-intent, parity or performance evidence.

## Ownership and current contract

Own only pure `boundary(items, style)`, private to the parent. Inspect the last item, ignoring only direct HTML markers. Nonempty Text/TextShaped and TextSeparator supply style; TextReplacement recursively inspects only its last child. Empty text, glyphs, objects, groups, links, lines and semantic wrappers interrupt the boundary. Keep exhaustive match.

Return resolved signed tracking, including negative values, only for fully equal nonmath styles and eligible direction. Originally limited to absent/LTR, eligibility is superseded only by the measured incremental explicit-RTL slice: admit explicit RTL with the same lexical-style/math/object/empty-line/last-item filters. Direction never changes interval sign. An empty/refused boundary returns zero. Do not inspect Layouter, emit, move cursor or choose breaks. Word and Space apply the result through their own operations, without space multipliers. Later RTL anchoring by actual line width belongs to `00_nucleo/prompts/core/compiler/layout/cursor.md`. Do not add tracking inside another unit's clusters, copy independent Hebrew differences, enable global selection or claim general RTL parity.

Own a parent-private Clone State with opaque active-scope and last-text-producer identities. enter creates a new identity and returns the prior one; leave restores only active identity, retaining last-emitted-item provenance. Empty scopes without emission do not interrupt continuity. record associates the last producer with the current scope; interrupt creates an object barrier until the next emission. State boundary returns zero across distinct identities and delegates to the existing pure calculation within continuous identity. Local Arc<()> identities are allowed, without global counter, serialized pointer or public contract change. Preserve signed values, TextStyle/math/direction filters, object barriers and selection. This private continuity state refines the earlier state-free separation; no unrelated state is authorized.

Own a private terminal-interval helper for a continued RTL line. Inspect the last material text unit, ignoring trailing TextSeparator carrier and direct HTML marker; respect object/barrier, math and direction. Return that unit's own resolved signed tracking; TextReplacement consults its last child. Do not adjust Hebrew deviations already present at 0pt or initial literal spaces.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, retaining language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Merge Text/TextShaped arms in continued_rtl_end and decide nonempty/nonmath/RTL eligibility in the body. Preserve sign/resolution, zero outside domain, recursive TextReplacement and other-item barriers; match remains exhaustive. Do not create a helper merely to hide conjunction, add suppression or change linter configuration.

## Acceptance, scope and remaining limits

Preserve causal tests, text/positions/styles/pages/diagnostics from pre-separation controls, 1:1 ownership, valid pins and error-free build/lint. Internal ADR-0109/0127 movement uses static descendant functions. Mixed-style tracking, global selection and cluster-counting limits remain as recorded in the correction diagnostic; RTL authorization is confined to the measured slices above.

Require RED→GREEN and bilateral text/position/line controls, preserved uniform tracking and LTR, with restricted resealing. Distinguish repeated scopes, entry/exit, empty scopes, restoration, barrier, clone/checkpoint and wrap. RTL controls cover both directions/signed values; continued-line controls preserve terminals/LTR. All other open limits remain open.

Maintenance follows continuous flow under ADR-0127 without a new public contract/default/compatibility/phase. Run existing functional controls before code and preserve expected outputs; actual V17 RED is 28 warnings, GREEN zero V17. Check success/fallback/error, short-circuit and effects. Revalidate build, full workspace suite, Clippy all-targets with warnings/unfulfilled expectations as errors, formatting, Rustdoc warnings as errors, strict/reverse V1/V5/V15/V26 lineage and SARIF without new warnings, under 12 GiB, zero swap and one job. Do not claim global memory/performance gain or parity without a separate measurement.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

Provide a private State::source_run_boundary only for adjacent validated source ranges within one actual superscript Text producer. Same lexical producer is mandatory; return the preceding resolved script style's signed interval across a generated face/size change, with zero for equal styles (the normal word boundary already owns it), nontext/empty/math/object/different-producer cases. Share the existing exhaustive previous-style projection without changing ordinary State::boundary semantics. This is a source partition, never an arbitrary source/Styled boundary.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.
