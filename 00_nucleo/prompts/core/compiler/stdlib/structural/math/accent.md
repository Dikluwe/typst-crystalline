# L0 — compiler/stdlib/structural/math/accent
Hash do Código: 0e1d0953

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
