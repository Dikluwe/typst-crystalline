# Prompt L0 — P1483 testes de export PDF spot
Hash do Código: 1f02d6e7

**Camada:** L3 (testes)
**Ficheiro alvo:** `03_infra/src/export/tests/p1483_pdf_spot_tests.rs`
**Criado em:** 2026-09-23
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

Vanilla pinado `586e1bd43`, `typst-pdf/src/paint.rs:115-163`,
PDFs de sonda P1483 às 18:43-18:50 de 2026-09-23:
fill/stroke/texto spot selecionam `/Separation` e tint
0/50/100 preserva identidade do colorant. `none` vira
`/None`, `"all"` vira `/All`; fallback CMYK mantém
`/DeviceCMYK`. Gradientes spot mantêm separação no shading.
O cristalino anterior a P1483 projeta RGB e não pinta fill
gradiente (também com cores de processo). SVG/PNG processuais
mostram o fallback tingido; HTML está indisponível nos dois
binários sem `--features html`.

## Contrato de teste

Testes RED→GREEN usam documentos `PagedDocument` construídos
em memória, sem I/O no código produtivo. Verificar separação e
colorant em recursos PDF, operador de um tint em fill/stroke,
extremos e meio, reuso do mesmo colorant, gradiente axial/radial/
cônico em separação, axial spot com extensão medida no vanilla,
e ausência de separação para controle de
processo. Modo compacto mantém a separação sem o recurso
ICC de texto verbose; nomes PDF escapam bytes reservados e
colorantes homónimos com fallbacks distintos não colapsam.
Gradientes spot no background da página também recebem
recurso Pattern e conservam a separação. O eixo axial e a matriz
cónica são posicionados pelo bbox do shape, inclusive quando
a página é maior do que a área pintada. O mesmo gradiente
reutilizado em dois shapes produz dois patterns geométricos,
mas conserva uma identidade de colorante.
IDs reservados não emitidos ficam livres (`f`) na
xref; `qpdf --check` não pode reportar offset zero em uso.
Testes de SVG/PNG conferem cor fallback sem impor
mecânica ou bytes iguais. Ferramenta independente (qpdf/pdftoppm)
fica no relatório do passo, não dentro dos testes Rust.
