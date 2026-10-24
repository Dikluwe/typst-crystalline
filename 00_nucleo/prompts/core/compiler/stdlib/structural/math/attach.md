# L0 — compiler/stdlib/structural/math/attach
Hash do Código: 39d19efb

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/attach.rs`
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba

**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.attach(base, t:, b:, tl:, bl:, tr:, br:)` and its bare bridge. Require one positional base castable through the shared `p1293_math_content` cast in `00_nucleo/prompts/core/compiler/stdlib/structural/math/casts.md`. Six named slots use the closed MathAttachSlot carrier: omitted, explicit None, or present Content including empty content. String and Symbol produce MathText; LocatedContent preserves its content. Preserve exact named order, positional hints, diagnostics, spans and `math_attach_content` construction. No sentinel, layout or callback. The pinned nucleus below is authoritative for shared tri-state invariants; this prompt owns only this productive constructor.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.


## R4 supplementary measurement and exclusive correction

`lab/results/math-fields-r4-2026-10-23/before-expanded/receipt.json` retains
UTC/source-map/HEAD/diff stat and binary hashes against pinned bd5ebec2.
Binom lower None/integer/boolean and raw Symbol slots of attach differ;
upstream math/frac.rs:134–150 explicitly uses Value::display for lower.
These are language semantics/morphology, not Rust equality certification.

Keep base conversion and slot-state validation separate. For a present Symbol slot preserve Content::Symbol(value), rather than MathText, in all six slots. Omitted, explicit None and present empty remain distinct. Keep String slots MathText and all prior slot flags/diagnostics. Add co-located tests covering all six Symbol slots and their tri-state controls.

This supersedes conflicting historical clauses only. Existing public fields,
trait declaration/signatures, defaults and pipeline phases stay unchanged;
ADR-0127 continuous parity flow, L0 before behavioral RED→GREEN, full validation
and lineage/full lint zero, preserving all received changes.
