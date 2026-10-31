# L0 — compiler/stdlib/structural/math/class
Hash do Código: 18f1dd36

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/class.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9

## Measurement before decision

The uncommitted working tree in `lab/results/construction/diagnostics/atomizacao-stdlib-math-class-2026-09-30-before.json.gz` places `native_math_class` at `math.rs:1069-1152`, with static registration in the parent and a public reexport through `structural/mod.rs`. `vanilla_type_name_class` at `math.rs:1060-1067` is shared by other natives. Baseline filters passed 29 `math_class` tests and 15 P825 tests. This is structural and regression evidence, not new bilateral vanilla evidence or a timing claim. The parent L0 retains the historical P772y/P825 language measurements.

## Language contract

Own the native `math.class(class, body)` cast and semantic `Content::MathClassOverride` construction. The first positional must be a String in the exact ten-name public cast domain: normal, punctuation, opening, closing, fence, large, relation, unary, binary or vary. Reject internal enum names such as alphabetic with the current exact cast message. For a nonstring class, retain the same message with long vanilla type names via the shared parent helper. The second positional accepts Content, String converted to text, or Symbol preserving its full value. Keep the present order of class validation, body validation and named rejection, the current detached spans, and the current behavior for extra positional items. Preserve all current diagnostic wording and the same native signature.

The shared `vanilla_type_name_class` remains owned by the parent because other natives use it. The parent retains static `math.class` registration; the structural hub directly reexports the native from this child with the same public `compiler::stdlib` path. No layout, dynamic dispatch, public signature, default, phase or compatibility change is authorized.

## Acceptance

Existing `math_class` and P825 regressions plus a local invalid-internal-class test remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Preserve the selected raw Symbol as Content::Symbol when casting a content argument; preserve quoted/numeric/manual legacy carriers and argument diagnostics. Matrix comma syntax recognizes the native Symbol comma without altering the payload.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Convert the second positional through casts::math_lr_content_value at the existing detached span. Preserve class-domain and argument-order validation. Replace only invalid body type diagnostics with the measured common content-cast diagnostic.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.


## Current-pin class cast domain

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/math-class-domain-before/receipt.json retains twenty
actual source cases, UTC, HEAD/dirty stat, source archive and both immutable
binaries. The pinned64c5e8c0 oracle accepts diacritic and the prior ten names;
alphabetic/glyph-part/space/special remain rejected. foundations/cast.rs:528
explicitly includes Diacritic. Local class.rs:34 has only ten names. The
class dispatch/domain/style receipts and source identity are pinned in
math-class-findings-before-decision.json. Domain and error text are language
observables; no intent is inferred from Rust representation.

### Exclusive correction and acceptance

Extend only the existing cast domain with diacritic and update its exact error
message to the measured eleven-name order, ending vary, or diacritic. This
supersedes the historical ten-name/domain-message clauses and the stale local
message expectation, preserving its alphabetic negative and all other inputs.
Require an accepted Diacritic carrier test plus all four internal-name negatives
and the exact eleven-name diagnostic. Existing body conversion/argument order/
detached fallback/native signature are unchanged. Default/explicit glyph
recursion belongs exclusively to math/layout/style_transform.md.

This mapping/diagnostic parity correction follows continuous ADR-0127, with L0
first and functional RED before code. No public Rust contract/default/phase is
added. Preserve all prior tests except the measured stale message literal, whose
exact pre-change source is archived in math-private-l0-before.json.gz. Full
validation/historical/JPEG criteria remain binding; no global class parity claim.
