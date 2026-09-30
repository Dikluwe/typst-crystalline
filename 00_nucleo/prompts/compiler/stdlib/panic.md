# L0 — stdlib/panic
Hash do Código: 277f949d

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/panic.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/panic.rs:21` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_panic accepts zero or more positional values of any type, rejects named arguments and always returns an error through SourceDiagnostic, aborting evaluation. With no values the exact message is `panicked`; otherwise `panicked with: ` followed by values separated by ", ". Str enters raw; other values use language repr. Preserve the empty string's trailing message space. Do not introduce a Value/Content variant, layout work, I/O, localization or an error(...) alias.

The final abort uses the aggregate whole-call Args.span supplied by call_dispatch, including alias/With; synthetic applications retain their supplied aggregate. Never recover origin from displayed values, AST or World, or invent traces. Preserve the preexisting named-validation message and primary span: its documented divergence is not fixed or claimed paritary here. Source overlay may add natural trace_call information without reanchoring/translating the primary diagnostic.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
