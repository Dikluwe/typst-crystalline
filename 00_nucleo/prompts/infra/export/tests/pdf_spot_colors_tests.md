# L0 — infra/export/tests/pdf_spot_colors_tests
Hash do Código: 7d581839

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdf_spot_colors_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdf_spot_colors_tests.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Spot PDF regression coverage

Own the existing in-memory public-API tests for fill/stroke/text Separation, named/
None/All colorants, tint endpoints/midpoint, resource reuse and process controls.
Preserve same-name distinct-fallback identities, PDF-name escaping, Compact versus
Verbose resource controls and axial/radial/conic one-component gradients.

Exercise gradient fills in background as well as body, shape-local axial/conic
placement and reuse at distinct shape locations. Check that un-emitted reserved
object IDs are free xref entries, not in-use zero offsets. SVG/raster controls
observe process fallback color rather than Separation or exact PDF bytes.

The productive owners are
`00_nucleo/prompts/infra/export/builder/resources.md`,
`00_nucleo/prompts/infra/export/stream/shape.md`,
`00_nucleo/prompts/infra/export/gradients/mod.md` and their specialized children.
Do not promote test assertions into duplicate emission algorithms.

## Verification

Preserve colorant/tint and graph/reference checks. Independent qpdf/pdftoppm
evidence is external to these Rust tests; a historic pinned run is not proof
against a newer upstream pin.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
