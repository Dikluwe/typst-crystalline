# L0 — core/compiler/math/layout/math_accent_layout_tests
Hash do Código: fbd7c308


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_accent_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_accent_layout_tests.rs:29` locates `struct NullWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent accent tests

Own only `01_core/src/compiler/math/layout/math_accent_layout_tests.rs`.
Observe attributable FrameItems, not serialized bytes.
Upper accents stay above cramped bases and preserve attachment/baseline
regressions; Unicode Below and the four explicit lower support symbols stay
below non-cramped bases. Only the upper path performs dotless realization.

Effective Rel<Length> size resolves against base width with shortfall 0.5em.
Synthetic identifiable variants distinguish 100% from 150%. Size and dotless
independently obey explicit > cascade > default, without manufacturing carrier
presence. Both transformations preserve every causal field. Dotless morphology
covers i and j with both polarities.

Preserve the exact-threshold witness: base=1200du, first variant=750du.
Shortfall 0.5em gives target=700du and selects that variant; 0.4em gives
800du and selects the larger variant; 0.6em gives 600du and retains the base.
This discriminates the actual threshold, not merely percentage differences.

Unattributable font, variant, glyph, frame, cascade or geometry is Unknown.
Unknown fails closed in mandatory cases and never kills a mutant.

## Occurrence identity observation

After accents acquire TextReplacement, position/size still observes original
Text/TextShaped/Glyph children. The helper traverses the envelope transparently,
without replacing geometry assertions with ActualText or changing unwrapped
cases. Integrated RED: orientation/dotless and size-precedence tests no longer
found direct leaves. Base i/j without dotless becomes 𝑖/𝑗 under existing default
math italics; compare realized morphology, not input ASCII. Dotless stays ı/ȷ.
Lower accents preserve base italics and use child positions for orientation.

## Nominal FrameItem observer projection

### Evidence before decision

F041 is compiler/math/layout/math_accent_layout_tests.rs:94 (visit). AST spans/patterns/guards/
bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners using the pinned CI parser. Pertinent
L0/source were read together, without new architecture or inferred upstream
intent. Rust exhaustiveness is mechanics; preserve product semantics/morphology/
geometry/effects/diagnostics.

### Exclusive prescription

For F041 visit, replace only `_` with nominal TextSeparator/Semantic/Line/Glyph/
Image/Shape/Group/Link alternatives using the in-scope FrameItem
prefix. Preserve the complete fallback body, prior arms, guards, transformations,
calls, returns and order.

Preserve callbacks/effects/metadata/clipping/fonts/positions/ink/width under their
current owners. Do not expand traversals, turn wrappers into leaves or substitute
text/ActualText for geometry. Test helpers retain fixtures/expected results/
tolerances/counts/Unknown failures; missing observation is not acceptance.
Nominal groups share the previous body; keep V19 notes without equating arm
coverage with every alternative.

### Acceptance and limits

ADR-0127 internal flow adds no enum/field/public trait method/signature/default/
dependency/compatibility/phase. Do not materialize unrelated pending proposals.
L0 first and restricted bilateral resealing. No suppressions, threshold changes
or match-to-if-let rewrites to hide V16.

Before code run the new FrameVisitor control and workspace suite on unchanged
expectations and confirm 70-family-warning RED by AST inspection. Afterwards
require zero selected-family V16 and no new warning; external V16 stays open.
For productive-core matches reached by cargo check -p typst-core, a disposable
copy with sentinel FrameItem measures absent E0004 before/present after. Claim
coverage only for compiled sites, not inaccessible cfg(test)/L3. Replay checks
all patterns/guards/bodies, unchanged enum/signatures and real builds/tests;
no stubbed bodies.

Validate full workspace/build, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes,
strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/
configuration during heavy sequential commands under MemoryMax 12 GiB,
MemorySwapMax0/CARGO_BUILD_JOBS1. Structural sanitation proves neither global
parity nor RAM/AI-reading benefit.
