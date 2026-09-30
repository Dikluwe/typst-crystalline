# L0 — compiler/stdlib/structural/math/vector
Hash do Código: 8faf22ad

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/vector.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-vector-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:724-893` owns the vector options, delimiter conversion, named casts, content constructor and native function. The local `p1292_c_vec_tests` at `:2401-2422` checks pure ratio conversion. The `math_vec` test filter passed 3 tests before movement. This shows a coherent behavior boundary, not a speedup or fresh bilateral vanilla result.

## Contract

Own the complete `math.vec(..children, delim: "(", align: center, gap: 0.2em)` constructor family. It accepts variadic children, including zero children, converting Content, LocatedContent, String and Symbol to the same semantic content as before and diagnosing other types with the existing span and wording. Named `delim`, horizontal `align` and relative-length `gap` retain current cast order, default values and explicit-presence bits. A present named marks its bit even when equal to the default. Unknown named remains an error. Preserve the full `Rel<Length>` in the carrier, including a pure percentage converted to a relative component with zero absolute component.

The measured error vocabulary is retained: integer `gap` gives `expected relative length, found integer`; integer `align` gives `expected alignment, found integer`; integer `delim` gives `expected array, none, symbol, or string, found integer`. Delimiter strings, symbols, array pairs and none preserve their current mapping. No truncation, layout, region-dependent resolution, callback or dynamic registry is added. The percentage base and `height:auto` belong to `compiler/math/layout/vec.md`; this owner constructs semantic `Content::MathVec` only.

The parent `math.rs` owns static binding registration and calls the native function in this module. The local pure-ratio test moves with the constructor. No public signature, pipeline phase, default or compatibility change is authorized.

## Acceptance

The local ratio test and existing math/vector tests remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. Each Prompt L0 has one productive consumer. No performance gain is claimed without a separate measurement.
