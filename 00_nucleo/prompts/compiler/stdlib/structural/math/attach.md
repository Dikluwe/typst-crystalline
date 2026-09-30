# L0 — compiler/stdlib/structural/math/attach
Hash do Código: ab4b79a1

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/attach.rs`
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.attach(base, t:, b:, tl:, bl:, tr:, br:)` and its bare bridge. Require one positional base castable through the shared P1293 content cast. Six named slots use the closed MathAttachSlot carrier: omitted, explicit None, or present Content including empty content. String and Symbol produce MathText; LocatedContent preserves its content. Preserve exact named order, positional hints, diagnostics, spans and `math_attach_content` construction. No sentinel, layout or callback. The pinned nucleus below is authoritative for shared tri-state invariants; this prompt owns only this productive constructor.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.
