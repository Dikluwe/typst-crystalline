# Passo 959 — Relatório (shifts de limites de operadores grandes: 4 termos MATH + extents de tinta da variante)

**Data**: 2026-08-04
**Estado da árvore**: Fase A (L0s) commitada em `e945d6193` após gate
confirmado pelo dono; Fases B/C por cima.

---

## 1. Fase A — causa (confirmada com valores reais)

A fórmula do vanilla (`compute_limit_shifts`,
`typst-layout/src/math/scripts.rs:290-313`):

```text
t_shift = base.ascent + max(UpperLimitBaselineRiseMin, UpperLimitGapMin + t.descent)
b_shift = base.descent + max(LowerLimitBaselineDropMin, LowerLimitGapMin + b.ascent)
```

O cristalino usava só os gaps (sem os `max()` — scope-out de P944 §4).
Valores reais medidos (NewCMMath-Book, upem 1000): `UpperLimitGapMin=200`,
`UpperLimitBaselineRiseMin=111`, `LowerLimitGapMin=167`,
`LowerLimitBaselineDropMin=600`. **Gate de contrato**: 2 campos novos em
`MathConstants` — confirmado pelo dono antes da Fase B.

## 2. Fase B — implementação (dois agentes + revisão do orquestrador)

- **Agente A**: 7 testes `p959_*` (braço gap domina / rise domina / drop
  domina / gap domina / sup+sub juntos / caixa final deriva dos shifts /
  guarda de integral), stub com as constantes reais; RED limpo (E0609 nos
  campos novos). Inventário: nenhum teste existente codifica a fórmula
  gap-only.
- **Agente B**: campos + leitura L3 (`math_constants_from_face`) + fórmula
  em `attach.rs`. 7/7 verdes; suite verde.
- **Revisão do orquestrador (Fase B.3) — achado da segunda causa**: o smoke
  do Agente B mostrou os gaps inalterados no caso comum. Investigação
  (A/B com stash, shifts medidos nos streams): a fórmula nova estava certa,
  mas a caixa da base chegava com `ascent = descent = advance/2` (split
  simétrico de P952 em `layout_large_operator_display`), enquanto o vanilla
  usa os extents de tinta reais da variante (`update_glyph`,
  `glyph.rs:215-231`). É a resposta à pergunta Fase A.4 (variação
  6.3-24pt = multi-causa): **(1) termos max() ausentes + (2) extents
  simétricos da base**. Correcção adicional (orquestrador, TDD):
  `p959_variante_display_caixa_usa_extents_de_tinta` (RED: 8.406/8.406
  simétrico) → `layout_large_operator_display` passa a usar
  `glyph_ink_bounds(variante)` — a mesma primitiva de P952b. Teste
  `p952op_sum_display_usa_variante_v1` actualizado (asseverava altura =
  advance; agora = tinta injectada, com nota).

## 3. Fase C — revalidação

- **Caso mínimo** (`$ sum_(k=1)^n $`, pdftotext -bbox): gap superior de
  tinta **2.38pt** (vanilla 2.36) e inferior **3.79pt** (vanilla 3.79) —
  antes: −0.37 (sobreposição!) e 6.55.
- **38 ocorrências** (doc de 30 secções, `tools/geometry/gaps959.py`,
  mesmo método da auditoria): cristalino min 2.39 / med **3.77** / core
  2.4-4.2pt vs vanilla min 1.86 / med 3.55 / core 1.9-4.2pt — **na banda**.
  A cauda (11-32pt) existe nos dois lados em forma idêntica (vanilla chega
  a 33.3) — é conteúdo (limites altos/multi-elemento), não bug.
- **`compare.py`** (secções 4/15/25/29): semanas laterais inalteradas —
  med|dy| ≈ 0 já era o estado; a correcção actua no gap relativo
  operador↔limite (medido acima), não no posicionamento absoluto.
- **Benchmark**: ver tabela (hyperfine, "antes" = release pós-P961; JSONs
  em `tools/perf/results/p959-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 88.24 | 88.82 | 1.007 |
| 02-lorem | 107.38 | 107.73 | 1.003 |
| 03-images | 94.55 | 95.78 | 1.013 |
| 04-math | 120.17 | 121.46 | 1.011 |
| 05-tables | 92.26 | 92.80 | 1.006 |
| 06-long | 296.56 | 300.48 | 1.013 |
| 07-context | 128.88 | 129.10 | 1.002 |

Ratio médio **1.008** — spread apertado e reproduzível, sem regressão.

## 4. Notas

- `layout_assembly` (peças de delimitadores, P957) e este passo partilham a
  lição: extents simétricos ou deslocados quebram tudo a jusante — a caixa
  da base tem de carregar a tinta real (vanilla `update_glyph`).
- Scripts de medição da frente vivem agora em `tools/geometry/`
  (`stream_diff.py` de P956 movido + `gaps959.py` novo) — o linter fatal
  V8/V1 sobre `.py` em `temp/` fica resolvido pela mudança.
