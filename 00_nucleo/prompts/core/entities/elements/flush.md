# L0 — FlushElem
Hash do Código: 4ac2ff85

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/flush.rs`

## Evidence before specification

Historical upstream `a51e02804` returned `flush()` for `repr(place.flush())`;
the earlier float-order probe established that preceding floats are forced
before subsequent content, without anticipating floats created after the marker.
That probe is historical evidence, not a current-pin remeasurement.

## Owned contract

`FlushElem` is a fieldless public singleton with
Debug/Clone/PartialEq/Eq/Hash/Default. It is distinct from Empty and non-locatable.
Text is empty, `is_empty` remains false so normalization preserves the effect,
and both maps are terminal. Equality and hashing reflect singleton identity.
It produces no visual item or introspection identity; it is not a text-empty
substitute because its flow effect is observable.

Content construction belongs to `00_nucleo/prompts/core/entities/content/construction.md`,
zero-argument validation to `00_nucleo/prompts/core/compiler/stdlib/layout.md`,
namespace exposure to `00_nucleo/prompts/core/compiler/eval/mod.md`, repr to
`00_nucleo/prompts/core/compiler/eval/repr.md`, and the effect to
`00_nucleo/prompts/core/compiler/layout/flush.md`. Their existing errors and repr are
not re-owned here. Verify nonempty terminal identity without text or own payload.
