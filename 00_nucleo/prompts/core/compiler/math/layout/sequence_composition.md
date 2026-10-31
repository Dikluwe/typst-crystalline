# L0 — compiler/math/layout/sequence_composition
Hash do Código: 08e2438d


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/math/layout/sequence_composition.rs`. The original behavior was in `01_core/src/compiler/math/layout/mod.rs:1729-1832,1834-1906` at HEAD `6fc546566`, clean tree, 2026-10-01 01:04:22 UTC. This is internal extraction, not a new parity or performance measurement.

## Contract

Prepare linear math sequences, resolve weak spaces/class gaps and concatenate MathBox horizontally. Preserve grid-branch selection, child order, baseline-relative item translation and text adjustment. Preserve the public contract, pipeline phase, defaults, compatibility and L1 purity.

## Acceptance

The L1 suite, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.


## Nominal source-transparent sequence projection

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz`
retains UTC, sources/L0/hashes/configuration, status and exact git diff HEAD --stat;
initial-ten-owners.json.gz preserves the first snapshot. contexts.json,
all-contexts.json and wildcard-inventory.json retain AST/spans from the pinned
CI parser. The previous suite passed on the same input code, reconfirmed by hash.

source-probe-before.json measures inline limits at width 7.2pt versus 12.912pt
with Source. source-family-before.json preserves the first 12 pairs (11
differences). source-family-supplement.json and analysis preserve 23 attributable
pairs (19 differences, 4 equal controls): limits, kerns, Op.text, accents/dotless,
supports, matrix alignment, spacing, sequences, stretch and class.
source-lines-before.json measures the same equation at two lines versus one and
last baseline 16.2 versus 0 under Source despite equal extents. Geometry,
extractable identity and equation metadata are observables; bytes/pointers/Rust
equality are not. The content-source-origin nucleus already requires transparency;
inspection finds helpers classifying wrappers instead of their bodies. Infer no
historical intent or global parity. The supplemental analyzer's 24-versus-23
count error was corrected from valid original stdout without rerunning or
counting the analyzer error as functional RED.

### Exclusive prescription

Normalize Source in existing reads of AlignPoint/Linebreak, weak space, next
material sibling and punctuation MathText. Reapply proven origin in the derived
punctuation view through the existing helper. The spacing view also recognizes
Source(Text) and carries its origin when producing transient MathText. Do not
modify author nodes, remove explicit spaces or use extractable text as geometry.
Preserve terminal whitespace without following material versus weak spaces
between material.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

L27: sequence_composition.rs:57, layout_sequence, Content. Exclude only Empty;
HSpace is guarded and remains in the complement when its condition fails.
Retain the previous body for variants outside the sought property and broaden
classification only by the specified Source transparency.

### Acceptance and limits

L0 before code. This ADR-0127 internal parity correction adds no public field/
signature/trait, dependency, product flag/default, compatibility break or phase.
V19 groups share their previous body with per-case justification; notes remain
visible and do not prove execution coverage per alternative. No allow/expect,
threshold increase, @ capture or hidden match in if-let/matches!. Only named
transparency/shallow restructuring changes functional code; do not change
formulas/constants or relax expectations. Require functional Source RED and
structural warning/probe RED before repair; afterwards regression GREEN, zero
emitted-site V16 and future sentinel review of all 36 sites (including new
Content-under-Option projection) using copied Content/Value/MathStyleKind/FrameItem
in a disposable real workspace compiling cfg(test). Current scrutinees/functions/
composition change only as specified; revalidate AST/replay, signatures and other
owners.

Gates: full workspace suite, build, fmt, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, Rustdoc -D warnings with four indexes,
strict/reverse V1/V5/V15/V26, full differential SARIF and diff-check. Expected V16
170→152; new V19 only from prescribed groups/new observer; no V20 growth.
Other violations remain pending. Freeze sources during gates; MemoryMax 12 GiB,
zero swap, one Cargo job, heavy commands sequentially. No commit/push/release or
CI pin change. Lint closure does not prove RAM gain, AI-reading benefit or
complete parity.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## R3 native Symbol grapheme composition

Evidence precedes this decision in `00_nucleo/prompts/core/compiler/math/layout/mod.md` and `lab/results/math-geometry-r3-2026-10-23/before/receipt.json`; actual RED in `red-tests/receipt.json` proves omitted independent advances and unsplit payload. Pinned `math/ir/resolve.rs:326–358` creates one glyph per grapheme. Compose the native Symbol's transient glyph nodes through existing sequence spacing and hconcat; keep selectors/combining marks attached with Unicode grapheme segmentation, preserve the authored public Symbol, and measure each cluster once. The owning dispatch/flatten projection delegates here, retaining source spans and neighboring boundary classes. No entity/public API, phase, callback or exporter change. Existing MathIdent/Text lexical runs remain unchanged. RED→GREEN and complete PDF/SVG comparisons at 0.001 pt are required under continuous ADR-0127.


## Issue 8973 — measured lr sizing and linebreak correction (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8973-2026-10-31/before-state.json`,
before-sources.json, before/receipt.json and before-relative/receipt.json
bind dirty HEAD, exact diffstat, UTC, sources and freshly built product bytes.
The oracle bytes are verified against SHA-256
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34,
commit 64c5e8c02fe0394d913ead36e42ef8c1334d2107. The Tekt baseline warns
that set lr.size is ignored; even without set, its two fences enclose the
whole two-row stack rather than their own first/last segments. The pinned
oracle itself wrongly uses inherited size over explicit size in multiline;
that defect is not an acceptance oracle. Unset explicit-size controls,
unequal-height rows and percentage/mixed probes distinguish the healthy
sizing contract from that defect. Source: rules.rs:200-216 omits lr.size;
math_set.rs:170-195 drops it; delimited.rs:193-205 pairs across Linebreak.
Pinned math/lr.rs:25-28 defines size relative to wrapped content;
math/ir/multiline.rs:58-111 splits rows/cells; fenced.rs:63-100 takes the
maximum natural fragment extent, not the complete stacked height. Explicit
precedence is already ratified in the owning delimited L0 and MathLrElem
approval of 2026-09-17. Geometry, row semantics, casts, diagnostics and
morphological presence are language obligations; upstream IR/cache/storage
and output bytes are not. New measurements that lose a row, leak size,
resize non-boundary delimiters or change authored content refute closure.

### Classification

This is an internal parity correction under ADR-0127 continuous flow. Keep
all existing public entities, fields, signatures, defaults, phases and
compatibility. In particular preserve MathLrElem's original body, size and
size_explicit; no eval-to-layout migration or new public contract is needed.
L0 and seals precede code; preserve meaningful RED→GREEN and valid V15/V26.

### Exclusive correction

Before the existing block sequence/grid decision, consume only the private
optional lr multiline-node view delegated to
`00_nucleo/prompts/core/compiler/math/layout/delimited.md`. Once expanded,
use the existing grid/spacing/concatenation paths and preserve markers and
neighbor order. Unchanged sequences and inline composition keep their
previous route. Do not implement fence geometry or sizing in this owner.
This bounded lr expansion supersedes only the earlier unchanged grid-choice
clause for hidden lr markers; all other source/class/spacing rules remain.
