# L0 — compiler/stdlib/registry/structural
Hash do Código: 7c166924

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/structural.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register heading/title/outline/par/raw, terms/divider/quote/document/footnote and bibliography/cite/link/label/ref/list/enum in their established order. Preserve outline.entry body/prefix/indented/inner/page children and ABIs. Par.line uses line; raw.line uses Func::element. List/enum/terms have item children; footnote has entry.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
