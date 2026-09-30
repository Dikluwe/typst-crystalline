# Prompt L0 — infra/export/stream/text — texto, glifos e agrupamento de runs
Hash do Código: 70b95db2

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream/text.rs`
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

- Possui seleção de fonte por FontList/FontVariant/FontVariations e emissores
  de Text, TextShaped e Glyph nos modos Compact e Verbose.
- Type1 conserva WinAnsi/fallback histórico. CIDFont/Multifont preservam o
  remap old->new do subset; glifos matemáticos selecionam a fonte pelo mapa
  reverso. Glifos bitmap usam os XObjects preparados pelo builder.
- TJ usa nominal - x_advance, convertido por units_per_em, e compensa x_offset.
  As regras de precisão e emissão atuais permanecem; y_offset continua no escopo
  vigente, sem introduzir operadores novos.
- Verbose emite q/cm/cor/BT/Tr/Tf/Tc/Tm/TJ ou Tj/ET/Q. Faux-bold, tracking,
  variações e cor spot preservam seus operadores.
- Runs só agrupam TextShaped de prosa consecutivos com mesma baseline e
  envelope: fonte, tamanho, fill, bold/italic, tracking, direção e upem.
  Texto math, bitmap e folhas UA-1 conservam fronteiras individuais.
  A fronteira TJ vem de pos.x contra o cursor efetivamente arredondado.
- Os helpers são compartilhados pelos dispatchers top-level e local.
  A seleção Type1/CIDFont/Multifont continua por enum estático.

## Verificação

Regressões P485/P486/P520 verificam TJ, offset e kerning; P906 verifica remap
e seleção multifonte; P956/P979 verificam envelopes e agrupamento. O teste local
exercita remap e delta nominal no construtor TJ compartilhado.
