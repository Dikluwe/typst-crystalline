# L0 — compiler/stdlib/registry/layout
Hash do Código: 8ed5e24f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/layout.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register grid with short-named cell/header/footer/hline/vline children, transforms, align and place.flush; register pad, hide, h, v, linebreak, parbreak, pagebreak, page, block, box, stack, repeat, columns, colbreak, measure, layout and stroke. Page uses Engine ABI and stroke remains Type. Place.flush uses the short name flush. Do not implement layout or validation.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
