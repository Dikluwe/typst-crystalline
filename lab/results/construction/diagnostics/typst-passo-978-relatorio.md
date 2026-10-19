# Relatório — Passo 978 (escala de tamanho de headings)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `25a94f1fc` (P977). Benchmark:
`temp/p978/typst-antes` = release de P977.

## Fase A — as duas divergências confirmadas com file:line

1. **Factores errados**: `heading_scale` (`layout/helpers.rs`) usava
   2.0/1.667/1.333/1.167. Vanilla
   (`lab/typst-original/crates/typst-library/src/model/heading.rs:281-285`):
   `level 1 → 1.4em, level 2 → 1.2em, level ≥3 → 1.0em`. Medido no
   vanilla: 15.4/13.2/11.0pt sobre corpo de 11pt — bate com a fonte.
2. **Escala perdida com `#set text(size:)`**: mecanismo confirmado por
   instrumentação — `heading.rs` calcula o tamanho certo
   (`layouter.style.size × scale`, ex.: 9×1.667=15.0), mas o merge de
   `layout/text.rs` (`ns_size.unwrap_or(layouter.style.size)`) deixava o
   canal custom do `#set` sobrepor-se — a regra documentada do merge ("a
   chain tipada vence") estava invertida para `size`. Verificado que
   `layouter.style.size` já reflecte o `#set` (9.00 no debug), pelo que
   preferi-lo não perde o set.
3. **Testes que travavam o comportamento errado**: dois snapshots
   binários P307b (`02-markup-heading`, `07-multi-feature`) — regenerados
   via `UPDATE_P307B_SNAPSHOTS=1` (mecanismo previsto para mudança
   intencional de output); nenhum teste de unidade trava os factores
   antigos.

## Fase B — correcção (fluxo contínuo ADR-0127: paridade, sem contratos)

- `heading_scale`: 1.4 / 1.2 / 1.0 (nível 1 / 2 / 3+).
- `text.rs`: `size: layouter.style.size` (o tamanho deliberado de
  features de layout — heading, super/subscrito de texto — vence o canal
  ns, como a regra documentada manda).

**Testes** (L1, `layout_typst` — RED confirmado nos dois primeiros):
- `p978_factores_de_nivel_do_vanilla` — L1/L2/L3 = 15.4/13.2/11.0.
- `p978_escala_compoe_com_set_text` — `#set text(size: 9pt)` → L2 a
  10.8pt (antes: 9.0).
- `p978_corpo_respeita_set_text` — guarda: o corpo fica a 9pt.

Suite completa: **5753 testes, 0 falhas** (+3; os 2 snapshots P307b
regenerados por mudança intencional de output).

## Fase C — Revalidação

- **Documento de 30 secções**: Tf sizes do cristalino passam a incluir
  **13.2 e 15.4 — os tamanhos de heading do vanilla** (antes: headings a
  11.0). Casos mínimos (com/sem `#set text`, níveis 1-5): iguais ao
  vanilla.
- **compare.py sec 4/25/28**: flagged 174 → **149** (sec 28: 33 → 25,
  max|dx| 16.163 → 7.304 — o gradiente das letras de heading desapareceu;
  sec 4: 53 → 45; sec 25: 88 → 79).
- **Benchmark** (`benchmark-p978-canonical.py`, 7 cenários,
  `tools/perf/results/p978-canonical/`): 01-hello 1.003 · 02-lorem 0.996 ·
  03-images 1.009 · 04-math 1.007 · 05-tables 1.000 · 06-long 0.999 ·
  07-context 1.005 — rácio médio **1.003**, zero regressão.
- **Linter**: resselo dos ficheiros tocados; `crystalline-lint .` →
  0 violations (só o V7 órfão pré-existente).

## Resultado

- Headings com a escala do vanilla (1.4em/1.2em/1.0em **relativa ao
  tamanho corrente**) em todos os níveis, com e sem `#set text`.
- Efeito colateral benéfico já verificado na suite: super/subscritos de
  texto (que também escrevem `layouter.style.size` directamente) ficam
  igualmente protegidos do stomp pelo `#set text(size:)`.
- Residual registado no L0: espaçamento acima/abaixo do heading do
  vanilla (1.8em/1.44em ÷ escala acima, 0.75em ÷ escala abaixo —
  `heading.rs:288-289`) não é tratado neste passo.
- Benchmark sem regressão; suíte verde; linter limpo.
