# Passo 952 — Relatório (espaçamento sistemático equação↔equação + sizing de fracções + operadores grandes + ancoragem vertical)

**Data**: 2026-08-01
**Estado da árvore**: commit base `5986691ea` (P950); alterações deste passo por cima.

---

## 1. Fase A — a premissa refinada pela medição

O achado do dono (deltas de +7 a +17pt entre as 44 equações numeradas, padrão
sistemático ligado a equações multi-linha) confirmou-se por medição
etiqueta-a-etiqueta. A investigação decompôs o padrão em **várias causas
distintas**, não numa regra única:

1. **A regra de espaçamento entre blocos JÁ batia com o vanilla** (1.2em,
   `flow/collect.rs:253-278` com colapso para `max(above, below)` em
   `flow/distribute.rs:185-199`) — mas **o cristalino aplicava-a errada**:
   baseline seguinte = `baseline_anterior + 1.2em + ascent_ink`, **sem a
   `descent_ink` da equação anterior** (o vanilla posiciona aresta-a-aresta:
   fundo do frame anterior + 1.2em + topo do seguinte). Défice = `descent_ink`
   da anterior — o padrão exacto do dono (multi-linha +7 a +17, linha simples ~0).
2. **Fracção display a ×0.7** (scope-out de P944 §8.3.6, aprovado corrigir aqui):
   `Display→Text` do vanilla é factor 1.0 (`style.rs:343-363`) — frações ~8.5pt
   mais baixas que o vanilla por equação.
3. **Operadores grandes não esticados em Display**: o vanilla estica glifos de
   classe `Large` (∑, ∏, ∫, ⋃, …) para `DisplayOperatorMinHeight` (1300du,
   `resolve.rs:350-354` + `fragment/glyph.rs:445-451`) — ∑ do vanilla a
   1.444em vs 1.056em do cristalino; ∫ a 0.999em vs 0.665em (medido nos PDFs).
4. **Ancoragem vertical das grelhas** (`layout_grid_boxes`): `dy = baseline_offset
   − row_ascent` — fórmula errada face ao vanilla (`run.rs:137`: `pos.y = size.y
   + row_ascent − sub.ascent`) — a grelha flutuava ~1 altura-de-linha acima da
   baseline da equação, tornando a `ascent`/`descent` declarada inconsistente
   com os items (extent de P813 errado → espaçamento errado).
5. **Centragem da tinta de variantes/assembly**: `shift_y` simétrico
   (`axis − half`) não centra a tinta real (assimétrica) no eixo — exposto
   quando (4) foi corrigido (teste de guarda P945 2×2 falhou).

## 2. Fase B — correcções implementadas (dois agentes nas frentes de geometria)

1. **Espaçamento equação→equação** (`equation.rs`): nova
   `Layouter::prev_block_equation_descent` (set no epílogo do bloco, reset nos
   mesmos pontos de `last_flush_advance` — `cursor.rs`, `sub_frame.rs`),
   somada na recuperação de baseline. Caso texto→equação inalterado (guardado
   pelos testes P813).
2. **Fracção com descida por nível** (dois agentes — Agente A: 4 testes red/1
   green; Agente B: implementação): `num_style`/`den_style` via
   `numerator_style` (novo helper, sem cramped forçado) e `denominator_style`
   (P945) — `Display→Text` ×1.0. Suite sem nenhum teste pré-existente ajustado.
3. **Operadores grandes em Display** (dois agentes): MathConstants ganha
   `display_operator_min_height` (lido da tabela MATH em L3); braços
   `MathIdent`/`MathText` de 1 carácter `is_large_operator` em Display usam a
   primeira variante vertical ≥ alvo (sem short_fall — `StretchInfo::default()`
   do vanilla). **Bug de subsetting apanhado na revalidação end-to-end**: as
   variantes ficavam fora do subset embutido (somas invisíveis no PDF!) —
   corrigido registando os caracteres de operadores grandes em
   `STRETCHY_BASES` (`build_math_glyph_reverse_map`, que alimenta o subset).
   Resultado: 22 somatórios a 1.444em, exactamente como o vanilla.
4. **Ancoragem da grelha** (`layout_grid_boxes`): `dy = baseline_offset +
   row_ascent − cell_box.ascent` (por célula, fórmula do vanilla `run.rs:137`).
5. **Centragem da tinta**: braço de variante de `stretchy.rs` usa a bbox real
   via novo `FontMetrics::glyph_ink_bounds` (L1 trait + L3); assembly usa
   `shift_y = −axis_pt − (total − adv_topo − adv_fundo)/2` — tinta centrada no
   eixo, caixa declarada consistente com a tinta.

**TDD**: teste `p952_equacao_equacao_spacing_inclui_descent_da_anterior`
(invariante aresta-a-aresta, red → green); 7 testes p952op (4 red → green);
5 testes frac (4 red → green); guarda P945 2×2 verde após (4)+(5).

## 3. Validação

- `cargo test --workspace`: **9 suites verdes, 0 falhas** (incluindo os testes
  novos e os snapshots P307b).
- `crystalline-lint .`: **zero violations** (hashes resselados; resta só o V7
  pré-existente alheio).
- **Gaps etiqueta-a-etiqueta** (as 44 equações): média de |Δ| de **5.77 →
  2.46pt**; gaps >5pt: **23 → 12**; os grandes deltas de sec 21 (+12.3/+12.9 →
  +5.2/+3.7) e sec 25 (+14.6 a +16.7 → +0.3 a +11.5) colapsaram. O remanescente
  concentra-se em sec 22 (`lr` literal — conteúdo extra, já catalogado P944
  §8.3.1) e pontos isolados dentro da margem.
- **Render end-to-end**: somatórios/integrais em tamanho display (22 ×
  1.444em = vanilla), frações a tamanho cheio, matrizes com delimitadores
  centrados a abraçar a grelha, espaçamento entre equações com ar (visual
  `temp/p952/`).
- `compare.py`: medianas verticais próximas de zero nas secções estáveis;
  medianas horizontais elevadas nas secções muito alteradas (4, 25, 28) são
  dominadas por mis-pairs de clusters (limitação documentada da ferramenta,
  §4 do relatório de P948) — a verificação visual directa confirma centragem
  correcta (lim a x=226 em todas as versões, página 511pt).

## 4. L0s actualizados

`equation.md` §P952 (espaçamento), `frac.md` §P952 (sizing), `_comum.md`
§P952 (numerator_style + large-op) + §P952b (ancoragem da grelha),
`math_constants.md` §P952 (campo), `font_metrics.md` §P952 (leitura do campo)
+ §P952b (glyph_ink_bounds), `stretchy.md` §P952b (centragem da variante),
`assembly.md` §P952b (shift da tinta), `layout_types.md`/`metrics.rs`
(glyph_ink_bounds).

## 5. Achados e lições registadas

- O padrão "sistemático" era **cinco bugs distintos a somar-se** — a medição
  por componentes (extents, gaps, trace de glifo) foi o que os separou.
- Testes com stubs não apanham subsetting/ToUnicode: as somas invisíveis só
  apareceram na revalidação end-to-end com PDF real — a disciplina de
  revalidação visual/`pdftotext` continua a ser o gate que apanha esta classe.
- Benchmark: ver tabela abaixo (hyperfine, "antes" = binário de `5986691ea` em
  worktree; corpus canónico; JSONs em `tools/perf/results/p952-*.json`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 91.07 | 92.55 | 1.016 |
| 02-lorem | 110.32 | 111.19 | 1.008 |
| 03-images | 98.01 | 98.19 | 1.002 |
| 04-math | 123.32 | 124.34 | 1.008 |
| 05-tables | 95.37 | 95.50 | 1.001 |
| 06-long | 298.65 | 302.14 | 1.012 |
| 07-context | 132.50 | 133.04 | 1.004 |

Ratio médio **1.007** — zero regressão de performance dentro do ruído de medição.
