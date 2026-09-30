# L0 — compiler/stdlib/structural/math/accent
Hash do Código: 7634bb21

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/accent.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-accent-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:393-550` contains the alias table for combining marks, normalization, base/accent/size/dotless casts, semantic constructor and native. Static registration occurs later in the parent. The `p1429` filter passed 9 tests before movement. This is a structural regression baseline, not timing or a new bilateral vanilla measurement.

## Language contract

Own `math.accent(base, accent, size: 100%, dotless: true)`. Base and accent are required positionals. Base accepts Content, LocatedContent, String and None with the existing semantic conversion; other types preserve the content-cast diagnostic. Accent accepts a one-codepoint String or Symbol, or MathText containing one symbol. Normalize direct combining marks and the existing fixed alias table to canonical combining characters; preserve the mapping for hat, arrows, overline and other entries without environmental lookup. Invalid or multi-codepoint inputs keep their distinct current diagnostics.

Named `size` accepts Relative, Ratio and Length, preserving the full `Rel<Length>` and marking explicit presence even at the default. Named `dotless` accepts boolean and marks its explicit bit. Unknown, duplicate, positional-as-named, missing and excessive arguments retain their current validation order, spans and exact diagnostic wording. Construct the complete semantic `Content::MathAccent` carrier; do not execute layout or add a second table in eval or entities.

The parent `math.rs` owns only static `math.accent` registration. The structural hub reexports `math_accent_content` for eval and `native_accent` for global/module registries directly from this child, preserving visibility and public identity. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

The P1429 regression filter and full core suite remain green. A local test checks alias normalization and direct combining-codepoint preservation. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No speedup is claimed without separate measurement.
