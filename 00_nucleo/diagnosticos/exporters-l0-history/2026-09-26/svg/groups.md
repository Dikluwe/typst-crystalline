---
Hash do Código: dfc5d2fb
prompt: infra/export/svg/groups
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/export/svg/groups

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:1726-1781, 1805-1887, 1984-1986`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/groups.rs`. Possuir render_group e clips geométricos locais, validação conservadora de ShapeKind e teste de identidade da matriz. Recursar através do dispatcher da fachada; preservar transform uma vez, base espacial, interseção ancestral/local, IDs compartilhados com imagens e fallback. Não transformar Unknown em clip ausente equivalente nem inventar carrier fill-rule.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

## Clip geométrico local

### Medição que precede a proposta

`FrameItem::Group` transporta `pos`, `matrix`, `clip_mask: Option<ShapeKind<Pt>>`,
`inner_width`, `inner_height` e filhos. O exporter SVG recebe `clip_mask` em
`render_group`, mas atualmente o ignora. A referência ratificada cria uma
definição `clipPath`, referencia-a no grupo e aplica a geometria no espaço do
grupo. A obrigação é o recorte observável, não a ortografia do ID, a ordem de
`defs` ou a estrutura Rust da deduplicação.

### Contrato vigente

- Quando `clip_mask` é `None`, o grupo mantém exatamente a semântica atual sem
  recorte adicional.
- Para `Rect`, `RoundedRect`, `Ellipse` e `Path` com geometria finita e
  representável, emitir um `clipPath` local e uma referência resolvida no grupo
  dono. `Rect`/`Ellipse` usam `inner_width` e `inner_height`; `RoundedRect`
  conserva os quatro raios; `Path` conserva subpaths e curvas no espaço local.
- A transformação e a posição do grupo são aplicadas uma única vez. O clip e
  os filhos compartilham a mesma base local; não assar novamente a transformação
  na geometria do clip.
- Grupos aninhados compõem clips por interseção sem perder o clip ancestral.
- IDs são locais, resolvidos e determinísticos. Renomear IDs ou reordenar
  definições sem alterar o grafo é mecânica. Deduplicação só é permitida quando
  geometria e base espacial observável forem equivalentes.
- O contrato atual não transporta fill-rule no `clip_mask`; paths cuja região
  dependa de even-odd permanecem `Unknown` até existir carrier legítimo. Uma
  linha aberta sem área não é promovida como clip preservado.
- Máscara alpha, `mask`, `filter`, `opacity` ou rasterização não substituem
  clip geométrico e permanecem fora desta alegação.
- Falha de representação, geometria não finita ou informação ausente conserva
  `Unknown`/`CONTRACT-GAP`; nunca remove silenciosamente o conteúdo nem promove
  ausência de clip a equivalência.

### Verificação

Os testes cobrem rect, rounded rect, ellipse, path, transform, nesting,
referência pendente, chave de deduplicação e casos opacos.

