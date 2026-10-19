# L0 — compiler/stdlib/structural/math/root
Hash do Código: f7114b9f

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
