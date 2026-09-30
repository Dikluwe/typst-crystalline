---
Hash do Código: ddf76185
prompt: infra/export/svg/navigation
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1

# Prompt L0 — infra/export/svg/navigation

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:160-227, 1889-1941`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/navigation.rs`. Possuir SvgDestinationContext, SvgDestination, normalização por label, lookup e emissão de nós, e render_link. Preservar links URL e destino da página corrente, hit-area, filhos e fallback destino ausente. Não inventar routing cross-page/bundle. O tipo público permanece reexportado na fachada.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

## Destinos internos SVG na página corrente

### Medição anterior à decisão

`FrameItem::Link` já transporta `LinkTarget::Destination(Label)` até L3, e
`PagedDocument` já conserva `extracted_label_pages` e
`extracted_label_positions`. A API page-only do exporter não recebe esses
mapas; por isso o consumer atual conhece a aresta, mas não consegue materializar
o nó de destino nem fechar a referência.

### Contrato do owner SVG

- Preservar `export_svg` e `export_svg_with_fonts` como wrappers compatíveis que
  usam contexto vazio. Adicionar variantes explícitas que recebem
  `&SvgDestinationContext`; não alterar silenciosamente callers unitários.
- `SvgDestinationContext` é dado L3 imutável e page-local: associa `Label` a
  posição e identidade SVG local. Não executa layout, não consulta filesystem e
  não conhece nomes de ficheiro.
- Para um destino presente, emitir exatamente um nó local identificável na
  posição medida e ligar `<a>` a ele. IDs podem ser renomeados, desde que o grafo
  permaneça fechado, determinístico e sem colisões.
- Para destino ausente ou pertencente a outra página sem rota fornecida, manter
  os filhos visuais e classificar o link como `Unknown`; não emitir fragmento
  pendente nem fabricar posição.
- URL externa permanece preservada e não depende do contexto interno.
- Escaping, nesting e transform preservam o conteúdo e a área clicável. Bytes,
  uso de `href` versus `xlink:href` e ortografia do ID são mecânica quando o
  grafo semântico é equivalente.

Cross-page e bundle permanecem fora deste owner até o caller fornecer uma rota
explícita. A expansão pública de `link()` para label, location ou page/x/y é
outro contrato e não é autorizada por P1247.

