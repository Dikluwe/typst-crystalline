# L0 — compiler/stdlib/structural/math/accent
Hash do Código: 2aa01635

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/selected-symbol-accent-call.toml sha256:c6ef1be7fbad9f9513c172b144b3dbe314ba24a6d4c8c09a26a9bca6c6f8b48b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/accent.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-accent-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:393-550` contains the alias table for combining marks, normalization, base/accent/size/dotless casts, semantic constructor and native. Static registration occurs later in the parent. The `p1429` filter passed 9 tests before movement. This is a structural regression baseline, not timing or a new bilateral vanilla measurement.

## Language contract

Own `math.accent(base, accent, size: 100%, dotless: true)`. Base and accent are required positionals. Base accepts Content, LocatedContent, String and None with the existing semantic conversion; other types preserve the content-cast diagnostic. Accent accepts a one-codepoint String or Symbol, or MathText containing one symbol. Normalize direct combining marks and the existing fixed alias table to canonical combining characters; preserve the mapping for hat, arrows, overline and other entries without environmental lookup. Invalid or multi-codepoint inputs keep their distinct current diagnostics.

Named `size` accepts Relative, Ratio and Length, preserving the full `Rel<Length>` and marking explicit presence even at the default. Named `dotless` accepts boolean and marks its explicit bit. Unknown, duplicate, positional-as-named, missing and excessive arguments retain their current validation order, spans and exact diagnostic wording. Construct the complete semantic `Content::MathAccent` carrier; do not execute layout or add a second table in eval or entities.

The parent `math.rs` owns only static `math.accent` registration. The structural hub reexports `math_accent_content` for eval and `native_accent` for global/module registries directly from this child, preserving visibility and public identity. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

The P1429 regression filter and full core suite remain green. A local test checks alias normalization and direct combining-codepoint preservation. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No speedup is claimed without separate measurement.

## Type and Symbol function-cast classification

### Evidence before decision

type-kind-probe.json and more-cast-probe.json under
`lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17` record a
candidate passing 5/6 tests but failing kind:int. Historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`, foundations/value.rs:647–658 accepts
Type.constructor or Symbol.func as Func; content/element.rs:210–214 requires
to_element; symbol.rs:133–137, math/accent.rs:189–214 and math/lr.rs:144–198
classify symbolic conversion. CLI exercises 37 types; Location without a
constructor was checked in pinned source. type-kind-classes.json preserves all
38 classifications. A Symbol body/caption becomes SymbolElem upstream, while
local Content lacks that variant and current markup retains Text. That carrier/
morphology requires public evolution SY1; preserving text does not close it.

### Exclusive responsibility

Add only pub(crate) is_known_math_accent(&str)->bool, consulting the one existing ACCENT_ALIASES table for a canonical codepoint or exact alias. Unlike normalize_math_accent, it must not accept arbitrary characters through the direct fallback. This pure Function-cast classifier invokes no function and changes neither normalize_math_accent, Accent construction nor layout.

### Acceptance and limits

L0 first; functional Type/Symbol/textual-value RED before code, GREEN and full
batch revalidation under 12 GiB/zero swap/jobs1. ADR-0127 continuous flow adds no
public Rust contract, phase or product default. Reseal only affected owners after
V15/V26; preserve math tests and original classifiers.


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

Delegate math_accent_base conversion to casts::math_lr_content_value. Accent-character normalization and other arguments are unchanged.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.


### R4 supplementary qualified-math accent carrier

`render-candidate/receipt.json` under the R4 evidence root records exact
source/binary hashes and UTC: code-mode eval succeeds but the same qualified
accent inside a formula rejects the accent with `expected a single-codepoint
symbol`; vanilla renders it. Local accent.rs:65–82 admits only MathText content.
This is a carrier conversion omission, not a layout change.

Accept native Content::Symbol alongside the existing MathText accent carrier,
including Source wrappers through the existing source_body projection. Normalize
with the same accent table and keep distinct invalid-character diagnostics. Do
not admit arbitrary styled content, arbitrary text or change accent geometry.
Add co-located RED→GREEN for native and Source-wrapped Symbol, invalid
multicodepoint content and unchanged String/Symbol alias conversion. Repeat full
validation on the final frozen source; the interrupted validation remains evidence.


## Selected-symbol accent callability — readiness 2026-10-29

### Measurement before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/accent-callability-findings6.json`
binds actual release6 source/archive, HEAD 212aed0449150dd296ceea13fc2be665cc5db01a,
uncommitted exact dirty-file stat, UTC and pinned oracle64c5e8c0. The original
14-case receipt's single-letter a is notation in BOTH engines; only the corrected
long-name before6b cohort proves local alias callability. Bare grave/arrow render
symbol+parentheses in Tekt, native accents; qualified and Code Symbol calls reject
Func in Tekt. Long local symbols lose accents/options/arity; beta/string notation
and noncallable Code rejection remain controls. accent-effects-before6/receipt.json
measures one seen.push effect in valid math/native Code controls, with Code Tekt
blocked at callee. No source/font/PDF-byte equality is language acceptance.

Pinned typst-library/src/foundations/symbol.rs:133-137 dispatches selected Symbol
through its accent function; math/accent.rs:212-290 owns strict aliases and the
single-base wrapper. Existing Func::native/with and call_dispatch's With merger
already transport bound arguments, final call span and occurrence order.
This is an internal semantic/morphological parity repair under ADR-0127 continuous
flow, not a public API/default/phase change.

### Exclusive amendment and acceptance

Add a pure pub(in crate::compiler) symbol_accent_func(selected: &str, call_span: Span) -> Option<Func>. Consult strict is_known_math_accent and existing normalization only. For a recognized value, return an existing native Func with the canonical accent prebound through existing With/Args; retain actual call_span in the private bound occurrence. This function is used immediately as a callee, not published as a new language value or registry. A private native wrapper removes only its own bound accent occurrence, validates exactly one actual positional base (missing argument: base / unexpected argument), inserts that accent immediately after base, retains all user occurrences/options, rebuilds Args at the retained actual call span and delegates math_accent_content. Propagate ordinary diagnostics; no unwrap/expect or fabricated source. Native/With/application machinery stays unchanged; no new Func representation or alias table. Existing four bare special wrappers may remain unchanged. Preserve all old constructors/classifiers and geometry. General Symbol metadata/field-method/LR callability and existing unrelated diagnostic wording are outside certification.

L0 precedes actual functional RED, then implementation/GREEN; preserve old tests,
source-bound bilateral PDF/SVG/error/effect controls, full core/workspace, build,
strict lint/Clippy/rustdoc/fmt and both lineage directions. Validate V15/V26 before
resealing. No completion/performance/JPEG qualification from a finite call fix.
