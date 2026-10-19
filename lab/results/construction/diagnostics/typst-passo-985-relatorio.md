# Relatório — Passo 985: gaps de `underbrace`/`overbrace` invertidos

**Estado do código das medições**: HEAD `5c5c56ca3` (P984) + alterações deste
passo. Commit final no fim.
**Gate ADR-0127**: não aplicável — correcção de paridade interna (fluxo
contínuo: L0 primeiro + resselo).

## Fase A — causa (medição + leitura do vanilla + dados da fonte)

Medição inicial (bandas de tinta a 600dpi, doc canónico secção 10,
`temp/p984/ubc-1.png` vs `ubv-1.png`):

| gap | vanilla | cristalino (antes) |
|-----|---------|--------------------|
| under: conteúdo↔chave | 0.72pt | 8.9pt |
| under: chave↔legenda | 3.24pt | sobrepostos (ápice toca o "m") |
| over: legenda↔chave | 2.28pt | sobrepostos |
| over: chave↔conteúdo | 0.48pt | 5.4pt |

Três causas encadeadas, todas confirmadas por leitura do vanilla + fontTools:

1. **Caixa da peça com métricas da fonte, não da tinta** —
   `emit_horizontal_variant` (`stretchy.rs`) usava `vertical_metrics`
   (ascent 0.8em, descent 0). NewCMMath-Book: `uni23DF` (⏟) tem tinta y
   −353..−109du (toda ABAIXO da baseline), `uni23DE` (⏞) +539..+783du (toda
   ACIMA). Com ascent inflado o ⏟ descia ~9.6pt e a caixa (descent 0) deixava
   a tinta sair por baixo → a legenda tight caía em cima da curva.
2. **Peça de cima não pode usar tight stacking** — a tinta do ⏞ flutua 539du
   acima da baseline. O vanilla trata o spreader como `AccentItem`
   (`resolve.rs:1416-1472`): aplica-se a fórmula P922 do acento,
   `over_y = −base.ascent + min(base.ascent, accent_base_height)`.
3. **Legenda é anexo de LIMITE, não tight** — hipótese inicial ("tight com
   caixas honestas chega") **refutada pela medição** (ápice do ⏟ a tocar o
   "m" de "soma" a 1200dpi — em produção a caixa da legenda vem da tinta
   real, não da caixa de linha como no stub). Leitura seguinte: a anotação é
   `ScriptsItem` top/bottom (`resolve.rs:1438-1460`), posicionada por
   `compute_limit_shifts` (`scripts.rs:295-311`):
   `base.descent + max(LowerLimitBaselineDropMin, LowerLimitGapMin +
   legenda.ascent)` / o espelho superior. Valores NewCMMath-Book:
   LowerLimitGapMin=167, LowerLimitBaselineDropMin=600, UpperLimitGapMin=200,
   UpperLimitBaselineRiseMin=111, AccentBaseHeight=450.

**Achado bónus (documentado no trait)**: `glyph_ink_bounds` devolve valores
**com sinal** na L3 (`up = y_max·scale`, negativo para ⏟; `down = −y_min·
scale`, negativo para ⏞) — o doc do trait dizia "ambos ≥ 0" (errado). O
sinal é necessário para `layout_accent` (`height() = y_max − y_min`); a peça
de baixo em `underover.rs` faz `max(0, ink_up)` no próprio site. Doc do
trait em `metrics.rs` corrigido.

## Fase B — TDD

L0 primeiro: `underover.md` §P985 (com a correcção pós-revalidação registada,
ADR-0108), `stretchy.md` §P985 (tinta com sinal), `_comum.md` §P918
(`stack_tight_above` removida — último consumidor deixou de a usar).

RED confirmado em 2 rondas: T1–T3 (tinta real + P922), depois T4–T5 (limit
shifts: obtido 5.88/−8.40 vs esperado 7.20/−9.73) e T1 reforçado com
`ink_up` com sinal (−109du: obtido −1.308 vs esperado 0.0). Testes em
`tests.rs` (`p906_tests`, stub ganhou `with_ink_bounds`).

Implementação: `emit_horizontal_variant` usa `glyph_ink_bounds`;
`underover.rs` com as 4 fórmulas (peça cima P922, peça baixo
`max(0, ink_up)`, legendas limit shifts) e ascent/descent da caixa por
`max(...)`; `stack_tight_above` e os 2 imports removidos (sem consumidores).

GREEN: **5779 testes, 0 falhas**. `crystalline-lint .`: 0 violations (só o V7
órfão pré-existente).

## Fase C — Revalidação

Documento canónico (`temp/p984/ubc4-1.png`, faixa central, 600dpi):

| gap | vanilla | cristalino (depois) |
|-----|---------|---------------------|
| under: conteúdo↔chave | 0.72pt | **0.72pt** |
| under: chave↔legenda | 3.24pt | **3.24pt** |
| over: legenda↔chave | 2.28pt | **2.40pt** (1px a 600dpi) |
| over: chave↔conteúdo | 0.48pt | **0.48pt** |

Confirmação visual: chaves coladas ao conteúdo, legendas com gap real, sem
toque na curva — distribuição idêntica ao vanilla (não só o mesmo total).

Benchmark canónico (`benchmark-p985-canonical.py`, antes = release P984):
ratios 0.987–1.017, **média 1.003 — sem regressão**.

## Notas

- `underbracket`/`overbracket` beneficiam da mesma correcção (mesmo caminho).
- O desalinhamento vertical acumulado entre secções (~5pt na secção 10)
  continua fora de âmbito (deriva de espaçamento entre equações, não destes
  gaps).
