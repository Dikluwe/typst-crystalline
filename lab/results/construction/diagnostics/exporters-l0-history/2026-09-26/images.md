# Prompt L0 — infra/export/images — composição de imagens PDF
Hash do Código: 5933a208

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: images.rs:32-38 contém cache por ponteiro; :40-131 perfis ICC;
:137-184 lê SOF JPEG; :188-198 define payload; :200-204 comprime Zlib;
:221-261 decodifica raster; :280-320 valida; :323-342 define refs/objetos;
:354-539 varre/deduplica/coleta recursos; :546-625 serializa streams PDF.
A organização física não é observável pela língua. A extração preserva os
contratos vigentes; medições históricas contra pins anteriores não são nova
prova de paridade. Snapshot e resultados pertencem ao diagnóstico.

## Decisão e contrato proprietário

Hub com módulos privados icc, jpeg, png, scan e xobjects. Cada consumer tem
owner próprio sob infra/export/images/. O hub conserva PdfImagePayload,
ImageRef e ImageXObject com campos e visibilidade existentes, e compress_zlib:
ZlibEncoder/Compression::default, erro por String sem nova política.

Preserva os caminhos, assinaturas e visibilidade dos helpers por reexports.
PdfImagePayload/process_png_for_pdf mantêm pub; ImageRef mantém pub(crate);
ImageXObject e helpers de export mantêm alcance crate::export;
validate_document_images mantém pub(crate). Tipos produtivos não apontam a
Núcleos e a detecção continua propriedade de entities/image-format.md (L1),
importada sem redefinição. Nenhum novo API, formato, default ou fase.

O hub não possui algoritmos de ICC, JPEG, decode raster ou scan. Esses
pertencem exclusivamente a infra/export/images/icc.md,
infra/export/images/jpeg.md, infra/export/images/png.md,
infra/export/images/scan.md e infra/export/images/xobjects.md.

## Aceitação

Consumidores de bitmap glyphs, resources, fonts, metadata e pipeline mantêm
compilação e comportamento. Preservar suíte E2E de exportação e acrescentar
testes locais de pixel/alpha, perfis ICC, cabeçalhos JPEG, dedup em Groups e
streams binários. cargo build e crystalline-lint . fecham sem erros.
Histórico anterior preservado em
00_nucleo/diagnosticos/images-l0-before-atomization.md.
