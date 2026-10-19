# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/font_metrics/basic.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `60cc3baafe4c2b92035de74f3d9be07ec41eaabe4381123c8778d1b172335511`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — infra/font_metrics/basic
Hash do Código: 6ede788c

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/font_metrics/basic.rs`
**ADRs:** ADR-0019, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: a família basic está concentrada no consumer
03_infra/src/font_metrics.rs, nas regiões registradas em infra/font_metrics.md.
Corpos e fixtures são preservados pela movimentação interna; a escolha da
organização não é observable da língua. Nenhuma medição nova de paridade é
inferida dos registros históricos contra pins anteriores.

## Decisão e contrato

Owner dos helpers typo_metrics e edge_offset_pt e do impl FontMetrics para
FontBookMetrics. Mantém advance proporcional em Pt, proteção upem, fallback
de glifo ausente, preferência OS/2 seguida de hhea, cap-height e edges.
Edges Length resolvem a partir de size; métricas desconhecidas usam default.
Ink bounds não negativos usam max(0, y_max) e max(0, -y_min); bounds assinados
preservam bottom negativo para tinta acima da baseline. Mede o codepoint
recebido; não reaplica map_glyph. Glyph-id usa a bbox da face única.
IC de glifo-base singular só integra advance em math e fora de TextItem;
variation selectors não criam um segundo glifo-base. GSUB ssty e extrações
MATH delegam a infra/font_metrics/math.md. glyph_to_char usa o mapa construído
pelo hub. Não altera fórmulas, unidades, fallbacks ou assinaturas.
Aceitação: proporcionalidade, escala vertical, edges explícitos, tinta real,
bottom assinado, attachment e distinção TextItem conservam os testes reais.

## Linhagem e validação

Este é o único owner deste consumer. O módulo é privado ao hub; helpers
compartilhados têm visibilidade limitada ao pai, exceto reexports crate-internos
preexistentes. Os testes locais não alteram API nem defaults. Executar a suíte
font_metrics, cargo build e crystalline-lint .; preservar o nome e o corpo dos
testes deslocados, sem transformar estrutura Rust em critério de paridade.
