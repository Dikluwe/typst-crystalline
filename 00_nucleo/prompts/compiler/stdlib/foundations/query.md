# L0 — stdlib/foundations/query
Hash do Código: 80ed15e4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/query.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/query.rs:26`, `01_core/src/compiler/stdlib/foundations/query.rs:45`, `01_core/src/compiler/stdlib/foundations/query.rs:74` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own metadata/query/locate/here/target native validation and projection. metadata embeds an opaque value in Content for introspection. Delegate selector parsing statically to `00_nucleo/prompts/compiler/stdlib/foundations/selector.md`, preserving established kind strings, label strings, Location/Selector/Label and element functions.

query preserves selected order/cardinality and each exact Location. Clone the complete stored entry into Value::LocatedContent(entry,loc), not only content or the caller's style chain; absent synthetic entry falls back to Value::Location. Present snapshots must not fall back field-by-field. Identical contents can remain language-equal with distinct locations; never search location by equality/hash/first occurrence. locate returns the first match Location or none, not all matches.

Preserve the entire Element selector including empty filter group/function identity/order. Before selection validate new Element leaves through the typed static native-recognition owner in `00_nucleo/prompts/compiler/eval/bindings/value_methods.md`; do not call constructors, compare names or infer invalid type from zero matches. text fails `text is not locatable`; Strong/Emph remain valid without requiring ElementKind. Compositions validate their new leaves while preserving legacy rules. This restriction does not apply to selector construction or show use.

After existing argument validation, record the effective query/locate request and observed result through EvalContext's private observation API. Empty query is an observation; equal locations with altered content/fields are not equality of results. locate records only its observed first occurrence/absence, not all later matches. Here records current_location or its absence; its existing contextual error remains. Revalidation selects/projects against the candidate without executing contextual bodies or constructors; use the observation comparator, not Debug/repr/location-only equality, and report incompleteness rather than success. Current-location identity is a retained-attempt precondition, not a candidate snapshot fact; a changed block requires fresh pipeline evaluation. Logging a read does not select a block for reexecution; Element-counter demand owns that selection. metadata/selector creation remain without reads.

target requires known context, rejects arguments and preserves `can only be used when context is known` with the existing hints. It returns the propagated target paged/html/bundle, with stable target/features as attempt inputs, not query results. The old paged-only statement contradicts current code at `01_core/src/compiler/stdlib/foundations/query.rs:159` and the separate-target architecture of ADR-0128; it is archived explicitly, not used to revert current public target observations. Do not change propagation, callbacks or pipeline phases.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
