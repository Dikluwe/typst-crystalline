# L0 — compiler/stdlib/tests/figure_image
Hash do Código: d94182f5

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/figure_image.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the figure_image regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Approved Figure/Grid typed-kind and occurrence-phase migration

At baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`, this test
consumer asserted constructor-time inferred String kinds. The owner approved
moving inference to occurrence preview/preparation and preserving lexical
omission versus explicit auto, with nominal FigureKind Element/Name identities.
The native Image/Table/Raw and nested Sequence inference observations remain
required, now asserted through the pure occurrence preview with actual native
function identities. Separately assert constructor lexical kind/resolved fields
are absent, explicit auto is Some(None), and explicit strings are Name rather
than element identity. Preserve existing inputs and all independent image tests,
errors and source spans. This migration changes the measured phase and carrier
only as explicitly approved; it does not weaken inferred language outcomes.
