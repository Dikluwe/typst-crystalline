# L0 — compiler/eval/flow
Hash do Código: 9f12e895

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/flow.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/flow.rs:17` — `pub enum FlowEvent {`.

## Contract

Own FlowEvent and forbidden-event diagnostics. Break and Continue carry their
causal Span; Return carries Span, optional Value and the conditional-return flag.
forbidden emits exactly `cannot break outside of loop`,
`cannot continue outside of loop` or `cannot return outside of function` at that
Span. Preserve payload and flag without consuming events here; their consumers
are loops, closures and the coordinating entrypoint.
