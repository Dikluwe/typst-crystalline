# L0 — infra/export/tests/pdfua_structure_tests
Hash do Código: 34c91036

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

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

## Ratified bidi carrier adaptation

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
identifies this unit's references and 1:1 owners before the decision. Manifesto
1ae7513e was confirmed. Uncertified constructors use None and read patterns use
`..`, retaining existing language observations. Transformations obey the pinned
origin nucleus without phase/API expansion.

Under the approved PDF warning carrier contract, the local export helper returns
`.output` for the existing byte assertions. The fixture remains warning-free;
do not filter exporter warnings or relax structure assertions.
