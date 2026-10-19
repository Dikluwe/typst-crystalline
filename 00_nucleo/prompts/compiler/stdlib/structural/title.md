# L0 — stdlib/structural/title
Hash do Código: dfc59335

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/title.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/title.rs:27` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

title resolves body from named body: Content|Str, then first positional Content|Str, then ctx.document_info.title converted to text. Named and positional together fail `title(): não pode usar body posicional e named 'body' simultaneamente`; without any source fail `title() exige body ou metadado document.title`. Wrong named body preserves `title(body:): espera content ou string, recebeu {type}`; wrong positional `title(): body espera content ou string, recebeu {type}`. Emit Content::Title(TitleElem::new(body)).

The precedence is observable; metadata must not override an explicit argument. Reading already resolved document_info is genuine context use, but remains pure L1 with no I/O or state mutation. Do not classify ctx as pass-through or add helpers/render logic.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
