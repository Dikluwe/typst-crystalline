# L0 — compiler/stdlib/structural/math/vector
Hash do Código: 012f0dd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/vector.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9

## Measurement before decision

At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-vector-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:724-893` owns the vector options, delimiter conversion, named casts, content constructor and native function. The local `p1292_c_vec_tests` at `:2401-2422` checks pure ratio conversion. The `math_vec` test filter passed 3 tests before movement. This shows a coherent behavior boundary, not a speedup or fresh bilateral vanilla result.

## Contract

Own the complete `math.vec(..children, delim: "(", align: center, gap: 0.2em)` constructor family. It accepts variadic children, including zero children, converting Content, LocatedContent, String and Symbol to the same semantic content as before and diagnosing other types with the existing span and wording. Named `delim`, horizontal `align` and relative-length `gap` retain current cast order, default values and explicit-presence bits. A present named marks its bit even when equal to the default. Unknown named remains an error. Preserve the full `Rel<Length>` in the carrier, including a pure percentage converted to a relative component with zero absolute component.

The measured error vocabulary is retained: integer `gap` gives `expected relative length, found integer`; integer `align` gives `expected alignment, found integer`; integer `delim` gives `expected array, none, symbol, or string, found integer`. Delimiter strings, symbols, array pairs and none preserve their current mapping. No truncation, layout, region-dependent resolution, callback or dynamic registry is added. The percentage base and `height:auto` belong to `00_nucleo/prompts/core/compiler/math/layout/vec.md`; this owner constructs semantic `Content::MathVec` only.

The parent `math.rs` owns static binding registration and calls the native function in this module. The local pure-ratio test moves with the constructor. No public signature, pipeline phase, default or compatibility change is authorized.

## Acceptance

The local ratio test and existing math/vector tests remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. Each Prompt L0 has one productive consumer. No performance gain is claimed without a separate measurement.

## Nominal Value delimiter projection

### Evidence before decision

M3 is compiler/stdlib/structural/math/vector.rs:47. Current L0 and the uncommitted
tree inventory at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` were read before
prescription. `lab/results/construction/diagnostics/tekt-v16-math-value-2026-10-16/before.json.gz`
retains UTC, status, git diff HEAD --stat and full hashes. Value has 39 variants;
measured fallback arms are internal and uniform. These are Rust evolution
warnings, not functional-failure or improved-AI-reading evidence. Infer no
upstream intent.

### Exclusive responsibility and acceptance

M3 vec_delim_char accepts None/Str/Symbol. Name all other 36 variants with the
same None result. Preserve casts/diagnostics/aliases and math_vec_delim array
fallback, including invalid first and absent/invalid second. Do not change
invalid-type error boundaries or option flags.

Replace only each selected wildcard with the nominal complement of current
variants and retain one shared result body. Informational V19 is legitimate for
identical results/effects and remains visible. Do not duplicate bodies, create
a facade helper, use a catchall binding/exemption or change configuration to
hide V16. Unused payloads may keep internal `_` patterns within named variants.
Public signatures, carriers, phase, defaults and diagnostics remain unchanged.

Before implementation write local accepted/rejected and specified morphology/
presence controls. Preservation controls pass before/after; RED is current V16
and evolution in a real workspace copy: a sentinel added to copied Value must
be rejected at each selected decision (the current fallback absorbs it).
GREEN requires E0004 at those points, zero selected V16, only equivalent new
V19 and no other new diagnostic. Controls do not claim execution coverage of
all 39 variants or global vanilla parity.

Reseal direct/reverse lineage only for this owner/consumer; validate build, full
workspace, all-targets Clippy with warnings/unfulfilled expectations as errors,
fmt, strict Rustdoc, V1/V5/V15/V26 and reverse lineage. Heavy commands run
sequentially under 12 GiB, zero swap and one Cargo job. Preserve commands,
source/hashes and UTC, and unrelated source.

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

Delegate each positional child conversion to casts::math_lr_content_value. Named option parsing and explicit bits remain unchanged.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.
