# L0 — stdlib/sys
Hash do Código: ffe9ed74

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/sys.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/sys.rs:34` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own pure make_sys_module(inputs:&SysInputs) composition. Return Value::Module named sys with exactly version and inputs. Version is built from the shared PARITY_VERSION in `00_nucleo/prompts/entities/version.md`, not a duplicated literal or crystalline binary version. The preserved language value is version(0,15,1); changing it is outside this sanitation.

Convert supplied SysInputs entries in insertion order into a Dict of Str keys/Str values, empty by default; n="42" remains a string, not an integer. Inputs are already resolved; this unit must not read World, CLI, environment or files. World injection, eval orchestration and CLI input collection belong to their respective owners. Preserve module type/repr and field access; the old Dict pseudocode and accepted-Dict-repr statements are superseded history, not current normative alternatives.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
