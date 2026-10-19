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

---

## 6. Adenda (2026-08-03) — revisão cética retroativa e correcção do item 4

### 6.1 Desvio de processo (admitido)

Os itens 1 (espaçamento equação→equação), 4 (ancoragem da grelha) e 5
(centragem da tinta) foram implementados **sem o protocolo de dois agentes**
que os itens 2 e 3 tiveram, e as duas mudanças de contrato do item 3/5 —
campo `display_operator_min_height` em `MathConstants` e método
`glyph_ink_bounds` no trait `FontMetrics` — entraram **sem a confirmação
prévia do dono** que o protocolo de P893/896/906/909/915/918/922/937 exige
para mudanças de contrato. A pedido do dono, foi corrida uma **revisão
cética retroativa** (agente read-only) sobre os cinco itens e os dois
contratos.

### 6.2 Veredicto da revisão retroativa

- **Item 1 (espaçamento)**: APROVADO — gap medido ≈1.2em aresta-a-aresta.
- **Item 5 (centragem da tinta)**: APROVADO — desvio medido 0.07pt a 300dpi.
- **Contrato `display_operator_min_height`**: APROVADO — 1300du em
  NewCMMath confirmado via fontTools.
- **Contrato `glyph_ink_bounds`**: APROVADO — default `cap_height`/`0`
  seguro. Nota latente registada: `FallbackFontMetrics::glyph_ink_bounds`
  resolve a 1ª face MATH em vez de `covering()` — coincide no setup actual;
  endereçar num passo futuro nesse ficheiro.
- **Item 4 (ancoragem da grelha)**: **REPROVADO — achado crítico**. A
  fórmula `dy = baseline_offset + row_ascent − cell_box.ascent` quebrava o
  alinhamento intra-linha de células com ascents diferentes (medido pela
  revisão: 2.77pt em `mat(a,b;c,d)`; pitch não-uniforme em `mat(a;b;c)`).

### 6.3 Correcção do item 4

A causa: o vanilla (`run.rs:137`) é **top-anchored** (frames ancorados no
topo), mas os items de `MathBox` são **baseline-relativos** (convenção de
facto: `layout_text_node` emite em `pos.y = 0` = baseline; `attach` põe
sup/sup offsets relativos à baseline). A tradução correcta da fórmula do
vanilla para esta convenção é simplesmente:

```rust
let dy = baseline_offset; // mod.rs:921 — a baseline da linha, sem termo por célula
```

O termo `− cell.ascent` só faria sentido para items top-anchored.

**Validação** (estado: working tree não commitado sobre `ec55046ce`;
ficheiros alterados no momento da medição: `_comum.md`, `font_metrics.md`,
`font_metrics.rs` (resselo), `math/layout/mod.rs`, `math/layout/tests.rs`
— `git diff HEAD --stat`: 5 ficheiros de código/L0, +119/−31):

- Teste novo `p952_grid_celulas_ascents_diferentes_partilham_baseline`
  (TDD: RED confirmado com a fórmula antiga — falha; GREEN com a
  correcção): células com ascents 4/14 partilham a baseline da linha e o
  pitch é uniforme nas duas colunas.
- **Render real** (`temp/p952/mat-ink.typ`, fonte NewCMMath do sistema,
  binário `typst-wiring` fresco): `mat(a, b; c, d)` com baselines da linha
  **exactamente iguais** (a e b com yMax 73.8388; c e d com 83.7078,
  pdftotext -bbox) e pitch uniforme **9.869pt** ≈ vanilla **9.8692pt**
  (typst 0.15.1, mesmo ficheiro). `mat(a; b; c)` também com pitch uniforme
  9.869/9.870. Antes da correcção: a e b diferiam de 1.94pt neste ficheiro
  (2.77pt no documento da revisão).
- **Armadilha registada**: uma primeira revalidação mediu o desalinhamento
  antigo porque o binário `target/debug/typst` estava **stale** — foi
  compilado `-p typst-shell` (biblioteca, sem bin target) em vez de
  `-p typst-wiring` (dono do binário `typst`). Sonda por estágio
  (L1 → bidi → shape → fix_line_positions) confirmou y preservado e igual
  em todos os estágios, o que expôs o binário velho. Lição: em
  revalidações end-to-end, confirmar que o binário medido é mais recente
  que a fonte (`stat`), ou compilar sempre `-p typst-wiring`.
- `cargo test --workspace`: 5669 testes, 0 falhas. `crystalline-lint .`:
  zero violations (3 ficheiros resselados; resta só o V7 órfão
  pré-existente alheio).

### 6.4 Pendência explícita — medianas horizontais do compare.py (sec 4/25/28)

A pedido do dono, os pares flagged do `temp/p952/report-v2.json` foram
decompostos: **~70% são pares com texto divergente** (ruído de
emparelhamento da ferramenta — limitação documentada em P948 §4); com
texto igual, a mediana de |dx| cai para **3.33pt (sec 4), 1.83pt (sec 25),
2.02pt (sec 28)**. O remanescente vem de deslocamentos de origem de
cluster causados por conteúdo já catalogado (`lr` literal, gregos
literais — P944 §8.3 itens 1-2). **Não é deslocamento novo** — mas fica
registado como **pendência explícita, não como resolvido**: a prova é por
decomposição estatística + um ponto de referência visual, abaixo do padrão
de prova que esta frente estabeleceu para si.

### 6.5 Lição de P949 incorporada à ferramenta

A armadilha "`compare.py` mede distância entre bounding boxes das peças,
não continuidade de tinta" foi incorporada a `tools/geometry/README.md`
(secção própria) — commit `ec55046ce`.
