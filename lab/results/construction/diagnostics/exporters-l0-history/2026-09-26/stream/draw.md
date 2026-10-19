# Prompt L0 — infra/export/stream/draw — dispatch e desenho recursivo de FrameItem
Hash do Código: 5ebc84c1

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream/draw.rs`
**Criado em**: 2026-09-26
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0129

## Medição anterior à decisão

Working tree não commitada sobre HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`:
`03_infra/src/export/stream.rs:359-970` contém texto e glifos;
`:971-1070,1830-1990` contém pintura e geometria;
`:1104-1182` contém montagem de página; `:1370-1829,1991-2329`
contém dispatch, marcação e desenho recursivo. O diagnóstico de atomização
registra horário e estado da árvore dessa observação antes da extração.

## Decisão e contrato proprietário

Extração interna estática para esta unidade, preservando a interface do hub.

- Possui draw_item_top e draw_item_local e seus matches estáticos de FrameItem.
  Preserva ordem, recursão e tratamento visual de Text/TextShaped/Glyph,
  Line/Image/Shape/Group/Link/Semantic.
- Top-level converte Y por page_height - pos.y; filhos de Link top-level
  seguem o mesmo caminho. Group preserva matriz, clip mask e desenho local.
- Texto local recebe o envelope de reflexão P1120 e coordenada -pos.y;
  shape, linha e imagem conservam seus referenciais próprios.
- Imagem compõe escala/posição com orientação EXIF 1..8 e precisão vigente,
  conservando os bytes originais preparados em resources.
- Sem tags, Semantic é transparente. Com tags, Formula/Lbl/TH/TD usam a
  tabela de MCIDs pré-computada; Artifact não consome MCID e conserva BMC/BDC.
- Em UA-1, folhas reais usam Span/Figure e decorações geométricas usam
  Artifact Layout. ParbreakBoundary e ExplicitLinebreakBoundary não pintam
  conteúdo vazio nem consomem MCID. Aberturas/fechamentos permanecem balanceados.
- Delega texto a text e geometria/pintura a shape; recebe PageContext e não
  decide IDs, recursos, ParentTree, fases de layout ou política de exportação.

## Verificação

Testes P788/P1120 verificam referenciais top-level e local; P1140/P1286/P1288/
P1405/P1481 verificam conteúdo marcado. O teste local de EXIF verifica a matriz
de uma imagem rotacionada com origem deslocada.
