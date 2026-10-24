# L0 — compiler/stdlib/structural/math/equation
Hash do Código: 58dc3260

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/equation-constructor-origin.toml sha256:947456f05619642c93f67721453e4ad5577557679dbc422a20a48f397ae97ded

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/equation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree recorded in `lab/results/construction/diagnostics/atomizacao-stdlib-math-equation-2026-09-30-before.json.gz` places `equation_content`, option casts and `native_math_equation` at `math.rs:46-210`. The structural hub reexports four helpers via the parent for eval and set rules, while the parent registers the static native. The P1140 filter passed 85 tests before movement. This measures structure and regression, not a new bilateral vanilla measurement or timing. Historical language measurements and snapshot qualifications remain in the parent L0 P1140 sections.

## Language contract

Own the pure `equation_content(body, block)` semantic constructor, the casts `equation_number_align`, `equation_supplement`, `equation_alt`, and the native `math.equation(body, block: false, numbering:, number-align:, supplement:, alt:)`. Preserve exactly one required Content body and its existing missing, excess and invalid diagnostics and spans. Named `block` accepts Bool; `numbering` accepts String, Func or None; `number-align` accepts Align2D except horizontal Center; `supplement` accepts String converted to Content, Content, Func, None or Auto; `alt` accepts String or None. Unknown named arguments fail. Preserve current cast order and exact diagnostic wording.

Keep the existing style-chain transport: only present named options add entries, under `equation.numbering`, `equation.number-align`, `equation.supplement`, and `equation.alt`; omit them without adding a delta. `block` controls the semantic Equation carrier and defaults to false. The complete Content is wrapped in local Styles only when the custom delta is nonempty. Preserve morphology and the same native signature. Do not execute callbacks, resolve layout defaults or move behavior into eval or entities.

The parent `math.rs` retains static binding registration. The structural hub reexports the four helpers directly from this child with the same visibility and `compiler::stdlib` paths. No public signature, default, pipeline phase or compatibility change is authorized.

## Constructor provenance and reflection

At HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`, native construction at
`01_core/src/compiler/stdlib/structural/math/equation.rs:175-195` transports
explicit options in Styled without recording their constructor origin.
`entities/content/identity.rs:13-31` consequently names that object styled.
Current-pin evidence in
`lab/results/v16-corrections-2026-10-19/batch-e/agents/language/` distinguishes
the equation constructor from ordinary external Styled. Upstream
`lab/typst-original/crates/typst-library/src/math/equation.rs:62-131` declares
the options and required body on Equation; the wrapper is a local transport
mechanism, not a new language element.

Only when the already-existing explicit-option custom delta is nonempty, append
the private `equation.constructor` Bool(true) marker before wrapping. Omitted
options, including block-only calls, still produce the bare Equation. Preserve
the original validated option Values, explicit none/auto and callbacks without
executing them. Do not infer constructor origin from an equation.* property key
on an unrelated Styled. The marker is internal metadata, never a public field;
its consumers own their private bounded classifications. No new public helper,
signature, carrier field, phase or default is authorized. This is an internal
language-parity correction under continuous ADR-0127 with L0 before code.

Co-located tests invoke the actual native and an alias: explicit options retain
equation identity, omission/block-only stay bare, option callbacks never execute,
and invalid arguments retain their diagnostics. Identity/reflection/serialization
consumers must distinguish valid marked transport from external/unmarked Styled.
Record real baseline RED and corrected GREEN, then the existing suites/lineage.

## Regression acceptance

P1140 regressions and a local test of supplement/alt cast distinction remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.


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

Delegate the one positional body conversion to casts::math_lr_content_value. Preserve explicit-option transport, constructor marker and all other validation.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.
