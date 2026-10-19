# L0 — infra/export/tests/pdfua_structure_tests
Hash do Código: c477be31

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdfua_structure_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdfua_structure_tests.rs:10` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF/UA structure regression coverage

Own tests through the existing public PDF API with Ua1 and Enabled tags for catalog/
XMP language, real text with structural node/MCID, local-language preservation and
the ordinary-PDF negative control. Preserve the multi-fragment heading/paragraph
case: one structural H1, one P, Strong as its child and no empty text node.

Productive owners are
`00_nucleo/prompts/infra/export/builder/metadata.md`,
`00_nucleo/prompts/infra/export/stream.md` and
`00_nucleo/prompts/infra/export/stream/draw.md`.
Image-alt and profile compatibility diagnostics belong to the upstream pipeline.

## Verification and limits

Check structural relationships and actual marked content, not IDs/compression/
whole-file byte equality. Existing string assertions do not replace bilateral
corpora or independent PDF/UA validation and do not establish full accessibility.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.
