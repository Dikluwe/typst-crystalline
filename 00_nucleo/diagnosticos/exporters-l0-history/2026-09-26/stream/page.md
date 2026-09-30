# Prompt L0 — infra/export/stream/page — montagem e ordenação do content stream de página
Hash do Código: fc7f7e39

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream/page.rs`
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

- Possui build_page_stream e emit_page_items: preenchimento do canvas, bleed,
  background, corpo e foreground nessa ordem. Bleed usa q/cm/Q e dimensões do canvas.
- Recebe PageContext emprestado; devolve Vec<u8> com os operadores da página.
  Compressão e envelope físico pertencem a builder/streams.rs.
- Agrupamento verbose de prosa delega a text; folhas UA-1 preservam MCIDs
  individuais. Dispatch de cada item delega a draw::draw_item_top.
- Não atribui recursos, IDs de objetos, ParentTree ou fontes; consome o contexto
  já preparado pelo builder. Não altera o padrão Verbose nem o modo Compact.

## Verificação

Testes de flip Y P788, envelopes e posições P956, grupos P1120, snapshots P307b
e exportação PDF com background/foreground/bleed exercitam o caminho público.
O teste local verifica que uma página vazia não emite operadores.
