# Prompt L0 — `infra/export/tests/p1481_pdfua_structure_tests`
Hash do Código: 04ab0e49

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1481_pdfua_structure_tests.rs`
**Criado em**: 2026-09-23 (P1481)
**ADRs**: ADR-0107, ADR-0108, ADR-0129

## Medição anterior à decisão

Após P1480, a fixture com título falhava 4 regras e 10 checks em
`veraPDF 1.30.2 -f ua1`; o vanilla pinado passava 106/0.
O corpus P1481 de duas páginas, com H1/H2, idiomas locais, Strong/Em,
Link, Figure descrita e Artifact, passava 106/0 regras no vanilla.
O exporter cristalino só criava nós para Formula/Lbl/tabela e
mantinha o conteúdo textual comum fora do ParentTree.

## Contrato test-only

`03_infra/src/export/tests/p1481_pdfua_structure_tests.rs` exercita
a fronteira pública de exportação com `PdfStandard::Ua1` e `PdfTags::Enabled`:
idioma no catálogo e XMP, texto real com nó e MCID, e um documento
cujo catálogo é francês mas cujo trecho padrão precisa conservar `en`.
O controle de PDF comum confirma ausência do `/Lang` específico de UA1.
O diagnóstico de imagem sem alt é testado no owner `infra/pipeline.md`.

Os testes não comparam bytes, IDs de objeto, compressão nem whitespace;
inspecionam relações e observáveis da linguagem. O corpus bilateral e
`veraPDF` continuam gates de aceitação do passo, não são substituídos
pelos testes de string aqui.

## Aceitação

`cargo test -p typst-infra --lib p1481_` passa os testes deste módulo
e o teste de gate no pipeline. `crystalline-lint .` não reporta V5/V15/V26.
O teste de morfologia constrói H1 e P em múltiplos fragments com
`ParbreakBoundary`: cada um resulta em um único nó estrutural,
preservando Strong como filho do P e sem nó para texto vazio.
