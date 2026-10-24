# L0 — compiler/stdlib/structural/math/root
Hash do Código: 69179a45

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/root.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-root-2026-09-30-before.json.gz` places `sqrt_content` at `math.rs:46-48` and both native `math.sqrt`/`math.root` constructors at `math.rs:216-289`. The structural hub reexports `sqrt_content` via the parent for eval math mode; the parent registers both native bindings. The P1426 filter passed 12 tests before movement. This is a structural and regression baseline, not new bilateral vanilla evidence or a timing measurement. Historical language evidence remains in parent L0 P1140.3-A and P1426.

## Language contract

Own the shared pure `sqrt_content` constructor and both native casts. `math.sqrt(radicand)` accepts exactly one Content, rejects named arguments through the existing `expect_no_named` path, and constructs a MathRoot carrier with absent index and `index_explicit: false`. Preserve its present diagnostics, including the current arity error for zero or excess positionals.

`math.root(index, radicand)` accepts exactly two positional arguments. An index Content becomes `Some(content)`, while None becomes `None`; `index_explicit` is true in both cases. Radicand accepts Content or None converted to empty content. Named `radicand` reports its positional status before unknown named validation. Preserve the current order of named, arity and type checks, exact diagnostics, spans and `vanilla_type_name_class` mapping. Construct the same semantic carrier without layout or dynamic dispatch.

The parent `math.rs` retains static `math.sqrt` and `math.root` registrations. The structural hub directly reexports `sqrt_content` from this child with the same visibility and public `compiler::stdlib` bridge. No public signature, default, phase or compatibility change is authorized.

## Acceptance

P1426 regressions and a local tri-state carrier test remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.


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

Use casts::math_lr_content_value for sqrt radicand and both root operands. Root index None remains absent; its explicit bit remains true. Keep arity/named validation. Co-located tests exercise actual module-bound constructors for Symbol, String, None and rejected integer/boolean, comparing accepted carriers against explicit content conversion; cover both root operands.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.

R4 supplemental co-located tests cover the registered binom lower display
(None/integer/boolean), required upper rejection, and Symbol root index.
Evidence is before-expanded/receipt.json above, and tests precede code.

R4 preflight found V16 at the test fixture's catchall returning Content::Empty.
Use an explicit table of input/general/attach expected content instead; do not
add linter exemptions or change productive conversion. The complete final-source
suite must recheck the five focused tests. This is test construction only.
