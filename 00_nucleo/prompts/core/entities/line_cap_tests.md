# L0 — core/entities/line_cap_tests
Hash do Código: 9221836c

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/line_cap_tests.rs`

## Observation contract

Observe lines produced by public routes through full layout, without manually
constructing the new representation or reading the candidate implementation.
Automatic matrix augment reaches the frame as Square; custom strokes preserve
Butt/Round/Square; legacy producers such as fraction lines remain Butt.
Missing lines, multiple unattributable lines or truncated cap observation yield
`UNKNOWN_STROKE_OBSERVATION`, with acceptance and mutation credit false.

## Evidence before exhaustive-pattern maintenance

F052 is line_debugs at `01_core/src/entities/line_cap_tests.rs:18`.
AST spans/patterns/guards/bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree,
UTC, source/L0/configuration/hashes/status/diffstat. The adjacent
classification.json records 70 cases in 26 owners with the CI-pinned parser.
Source and relevant L0 were read together. Rust exhaustiveness is mechanics;
product semantics/morphology, geometry, effects and diagnostics must remain.
No new architecture or upstream-intent inference follows from this measurement.

## Owned exhaustive pattern

In F052 replace only `_` with the FrameItem-prefixed nominal alternatives
TextReplacement/TextSeparator/Text/TextShaped/Glyph/Image/Shape. Preserve the
entire fallback body, previous arms, guards, transforms, calls, returns and order.
Keep callbacks/effects/metadata/clipping/fonts/positions/paint/width as defined
by their owners. Do not broaden traversal, treat wrappers as leaves or replace
geometry with text/ActualText. Preserve fixtures, expectations, tolerances,
counts and Unknown failure; absent observations cannot pass. V19 alternative
notes remain visible: one shared arm's coverage does not cover all alternatives.

## Acceptance

ADR-0127 internal continuous maintenance: no new enum/field, trait method,
public signature, default, dependency, compatibility or phase. Do not enact
pending historical proposals. L0 first, restricted forward/reverse resealing;
no suppression, threshold change or match-to-if-let workaround hiding V16.

Before code run the new FrameVisitor control and workspace suite with unchanged
expectations, confirming the family's 70 AST warnings are RED. Afterward require
zero family V16 at covered locations and no new warnings; external V16 remains
pending. In a disposable copy add a FrameItem sentinel for production-core
matches actually reached by cargo check -p typst-core: E0004 must be absent
there before and present afterward. Do not extrapolate to cfg(test) helpers or
unreachable L3 sites. Replay patterns/guards/bodies across the whole batch;
verify unchanged enum/signatures, real build/tests and no stubbed bodies.

Require full workspace tests, cargo build, Clippy workspace/all-targets
-D warnings -D unfulfilled_lint_expectations, formatting, Rustdoc -D warnings
with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and
diff --check. Freeze source/L0/configuration during commands. Heavy jobs run
sequentially with MemoryMax=12 GiB, MemorySwapMax=0 and CARGO_BUILD_JOBS=1.
Structural cleanup proves neither global parity, memory gains nor AI benefits.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/line_cap_tests.md`. Historical observations are not a new parity certification.
