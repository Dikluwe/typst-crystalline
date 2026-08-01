# Passo 945 — Relatório (Fase A completa; Fase B aguarda hash dos L0)

**Data**: 2026-08-01
**Estado da árvore na medição**: commit `49ca7a629` (P944 commitado, HEAD de `Tekt`).
Working tree: untracked `test_crystalline.pdf` (regenerado pós-P944),
`test_vanilla.pdf`; instrumentação DIAG (`eprintln!` em `assembly.rs`) **revertida**
após as medições; edições restantes são só os 12 prompts L0 deste passo.
**Artefactos**: `temp/p945/` (`m33.typ`, `m33-crys.pdf`, `m33-vanilla.pdf`, PNGs).

---

## 1. Resumo executivo

O sintoma (delimitadores de assembly "duplicados/desencontrados" em matrizes/casos de 3+
linhas) **não é** duplo desenho nem excesso de peças. A causa dominante é a grelha estar
**~30% mais curta que o vanilla**: as células de `mat`/`cases` são compostas a
`size × script_percent_scale_down` (7.7pt) mesmo em equações de bloco (`Display`), quando o
vanilla as compõe a **tamanho cheio** (`style_for_denominator`: `Display→Text`, factor 1.0 —
`lab/typst-original/crates/typst-library/src/math/style.rs:343-363`). Grelha curta → alvo do
delimitador curto → assembly com menos extensores (1 vs 2) → peças "desencontradas" face às
linhas. Há duas causas secundárias confirmadas: acumulação duplicada de `total_descent` em
`layout_grid_boxes` (+~4-6pt) e ausência de `minConnectorOverlap` no algoritmo de assembly
(P913 simplificou-o para 0).

## 2. Fase A — reprodução e contagem de traços

`temp/p945/m33.typ` = `$ mat(1,2,3;4,5,6;7,8,9) $` com
`#set text(font: "New Computer Modern", size: 11pt)` (a configuração do documento de teste).

`mutool trace` — glifos desenhados para os delimitadores:

| | cristalino | vanilla |
|---|---|---|
| peças por lado | 3 (gancho topo, **1** extensor, gancho fundo) | 4 (gancho, **2** extensores, gancho) |
| altura total do delimitador | ≈34.3pt | ≈41pt |

O cristalino desenha **menos** peças, não mais — a "duplicação" no texto extraído são os
hooks/extensores contados como caracteres separados (`(`+`|`+`(`), inerente a qualquer
assembly (o vanilla também extrai 4 peças). O defeito geométrico real: delimitador curto e
peças posicionadas para um alvo errado.

### Instrumentação do código actual (`eprintln!` temporário, revertido)

```
assembly c='(' target_du=3122.68 target_pt=34.349 ratio=0.266
  parts=[(1388,1495,..),(1387,498,ext),(1386,1495,..)]   ← 1 extensor
```

## 3. Medições da grelha (a causa dominante)

`pdftotext -bbox` do mesmo `.typ` nos dois compiladores:

| | vanilla | cristalino |
|---|---|---|
| tamanho dos dígitos das células | **11.0pt** (trm 11) | **7.7pt** (trm 7.7) |
| passo entre linhas | 13.16pt | 9.87pt |
| altura da grelha (ascent+descent) | ≈36.3pt | 31.23pt |

Leitura do vanilla que ancora a correcção:
`lab/typst-original/crates/typst-library/src/math/style.rs:343-363` —
`style_for_denominator = style_for_numerator + cramped`, e `style_for_numerator` desce um
nível **discreto**: `Display→Text` (factor 1.0), `Text→Script` (×0.7),
`Script|ScriptScript→ScriptScript`. O P923 implementou a descida como ×0.7 incondicional —
medido e correcto em **inline** (`Text→Script`), errado em **bloco** (`Display→Text`).

## 4. Causas secundárias confirmadas

1. **`layout_grid_boxes` — `total_descent` acumulado a mais**
   (`01_core/src/engine/math/layout/mod.rs:811`): a transição soma
   `row_descent + gap + next_row_ascent + next_row_descent` — o `next_row_descent` de cada
   linha intermédia é contado duas vezes (para N linhas: +`d_1+…+d_{N-1}`, ≈4-6pt). Vanilla
   (`lab/typst-original/crates/typst-layout/src/math/table.rs:103-106`):
   `total_height = Σ(a_r + d_r) + gap×(nrows-1)`.
2. **Assembly sem `minConnectorOverlap`** (`01_core/src/engine/math/layout/assembly.rs`):
   vanilla (`glyph.rs:610,639-640`): `growable += max(0, max_overlap − min_overlap)` e
   `advance += ratio × (max_overlap − min_overlap)`; cristalino equivale a `min_overlap=0`.
   NewCMMath: `minConnectorOverlap = 20du` (fontTools). Efeito: juntas mais abertas que o
   vanilla quando `ratio > 0`.

## 5. Suspeitas do passo investigadas e refutadas (com evidência)

- **Duplo desenho** (`delimited.rs` + `stretchy.rs` desenhando o mesmo delimitador):
  refutado — o trace mostra exactamente 3 peças por lado; `delimited.rs` delega uma única
  vez por delimitador (`delimited.rs:36-37`).
- **Peças a mais (extensores extra)**: refutado — há peças a **menos** (alvo curto).
- **Fórmula do alvo da grelha errada** (hipótese levantada a meio da investigação:
  `(a+d)×1.1` vs `2×max(a−axis, d+axis)`): **refutada por leitura do vanilla** —
  `resolve.rs:1168-1186` (`resolve_delimiters` para matrizes/casos): alvo
  `Rel::new(Ratio::new(1.1))`, `balanced=false` → `1.1 × (a+d)`, exactamente a fórmula
  cristalina. A fórmula balanceada (`2×max`, `fenced.rs:93-97`) só se aplica a grupos
  `MathDelimited` (`balanced=true`, `resolve.rs:976`) — já correcta em `delimited.rs`.
  Registado em `_comum.md` §P945 como anti-deriva.
- **`y_offset` por peça (bbox descent)**: irrelevante para NCM — todas as peças de assembly
  de NewCMMath têm `yMin = 0` (fontTools BoundsPen, `uni239B..uni23A0`).

## 6. L0s actualizados (aguardam hash do dono)

12 prompts: `entities/layout_types.md` (campo `math_size`), `engine/layout/equation.md`
(entrada Display/Text), `engine/math/layout/_comum.md` (`denominator_style` +
`total_descent` + anti-deriva do alvo), `matrix.md`/`cases.md` (descida por nível),
`attach.md`/`frac.md`/`root.md`/`underover.md` (só manter `math_size` honesto, sem mudar
factores), `assembly.md` + `entities/glyph_variants.md` + `infra/font_metrics.md`
(`min_overlap` ponta a ponta).

**Scope-out registado**: correcção dos factores de `frac.rs`/`root.rs` (fracção display a
×0.7 em vez de Text — P944 relatório §8.3 item 6) fica para passo dedicado que reutilize
`math_size`; `y_offset` de peças (no-op em NCM); `lr()` literal (P944 §8.3 item 1).

## 7. Plano da Fase B (após hash)

Dois agentes (protocolo P898): testes primeiro — contagem de glifos/peças do assembly
(3×3 não-duplica e cobre a grelha; 2×2 guarda de não-regressão; 4×4/reticências;
`cases()` de 3 ramos; `binom`), incluindo o caso `Display` (células a tamanho cheio) e
`inline` (células a ×0.7, comportamento P923 preservado). Implementação nos pontos do §6.
Fase C: revalidação visual das 30 secções, attestation `mutool trace`, benchmark dos 7
cenários canónicos.

---

## 8. Fase B — dois agentes, TDD

L0s confirmados pelo dono ("Continue"). Protocolo executado:

**Agente A (testes)** — 11 testes novos (`p945_*`) em
`01_core/src/engine/math/layout/tests.rs` e `01_core/src/engine/layout/tests.rs`:
**6 red** (células Display a tamanho cheio em mat e cases; assembly 3×3 com 4 peças;
`total_descent` sem dupla contagem; matriz em superscript desce Script→ScriptScript;
matriz em ScriptScript não desce mais) e **5 green** (inline ×0.7 preservado — P923;
nível Text; guarda anti-deriva de `grid_delim_target_du`; guarda 2×2). Suite: 4815
passed + 6 failed (só os red novos).

**Agente B (implementação)** — conforme os L0s:
- `entities/layout_types.rs`: enum `MathSize { Display, Text, Script, ScriptScript }`
  + campo `TextStyle::math_size` (default `Text`); sites não-spread
  (`style_chain.rs`, `engine/layout/text.rs`) no padrão P891/P915.
- `engine/layout/equation.rs`: entrada fixa `math_size: Display` (bloco) / `Text`
  (inline).
- `math/layout/mod.rs`: helper `denominator_style` (tabela de níveis do L0) +
  fix de `total_descent` em `layout_grid_boxes` (forma vanilla `table.rs:103-106`).
- `matrix.rs`/`cases.rs`: `cell_style` via `denominator_style`.
- `attach.rs`/`frac.rs`/`root.rs`/`underover.rs`: só o campo `math_size` actualizado
  (factores inalterados, scope-out respeitado).
- `entities/glyph_variants.rs` + `03_infra/src/font_metrics.rs` +
  `math/layout/assembly.rs`: `GlyphAssembly::min_overlap` ponta a ponta
  (`min_connector_overlap` da face; laço e posicionamento com a fórmula do vanilla
  `glyph.rs:610,639-640`).

**Validação**: `cargo test -p typst-core p945` → 11/11; `cargo test --workspace` →
**4821 + 748 + 41 + 2 + 37 + 2 = 5651 passed, 0 failed**; `crystalline-lint .` →
**zero violations** (hashes resselados; resta só o V7 pré-existente alheio). Nenhum
teste pré-existente precisou de ajuste de expectativa (os 4 literais `GlyphAssembly`
sintéticos ganharam `..Default::default()` só para compilar — semântica idêntica).

## 9. Revisão do orquestrador + attestation

`temp/p945/review.typ` (3×3, `cases` de 3 ramos, `binom`, matriz 4×4 de reticências,
matriz aumentada) compilado com o binário corrigido e com o vanilla real
(`temp/p945/review-crys.png` / `review-vanilla.png`): a causa raiz é confirmada comum
aos três consumidores de grelha — todos esticam agora com células a tamanho cheio e
delimitadores que abraçam todas as linhas, visualmente próximos do vanilla.

**Attestation `mutool trace`** (`temp/p945/m33-fixed.pdf`, matriz 3×3):

| | cristalino P944 (antes) | cristalino P945 (depois) | vanilla |
|---|---|---|---|
| peças do `(` esquerdo | 3 (1 extensor) | **4 (2 extensores)** | **4 (2 extensores)** |
| total de glifos na página | 15 | 17 | 17 |
| passo entre linhas | 9.87pt | **13.156pt** | 13.16pt |
| altura do delimitador | ≈34.3pt | ≈39.5pt | ≈41pt |

**Achado pré-existente novo** (não introduzido por este passo; presente já em P944):
a matriz aumentada (`augment: #3`, secção 21) **não desenha a linha vertical** de
aumento que o vanilla desenha. Registado para passo próprio.

## 10. Fase C — revalidação das 30 secções + benchmark

Recompilado `.typ/typst-math-comprehensive-test.typ` com o binário P945
(`temp/p945/out-p945.pdf`) e comparado com o render pós-P944
(`temp/p944/out-fixed.pdf`), 30/30 secções lado a lado
(`temp/p945/p944vsp945-batch*.png`):

- **Melhorias** (esperadas, a correção a actuar): secções 5, 7, 9, 21 — células a
  tamanho cheio, delimitadores com folga simétrica próxima do vanilla.
- **Regressões novas**: **nenhuma** — as restantes 26 secções estão visuaismente
  idênticas ao render pós-P944 (sujeitas à mesma comparação por pares).
- Os achados pré-existentes de P944 §8.3 mantêm-se inalterados (não agravados).

**Benchmark** (hyperfine, warmup 1, min 10 runs; "antes" = binário do commit P944
`49ca7a629` compilado em worktree — `temp/p945/typst-p944`; "depois" = working tree
P945; corpus `tools/perf/corpus/p922923-canonical`, JSONs em
`tools/perf/results/p945-*.json`):

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 86.51 | 87.30 | 1.009 |
| 02-lorem | 104.75 | 106.24 | 1.014 |
| 03-images | 92.56 | 93.96 | 1.015 |
| 04-math | 117.50 | 118.47 | 1.008 |
| 05-tables | 91.43 | 91.36 | 0.999 |
| 06-long | 283.29 | 285.86 | 1.009 |
| 07-context | 124.77 | 127.15 | 1.019 |

Ratio médio **1.011** (desvios-padrão sobrepostos nos 7 cenários; o cenário mais
afetado pela mudança, `04-math`, ficou em 1.008) — **zero regressão de performance**
dentro do ruído de medição.

## 11. Fecho

- Causa exacta confirmada por leitura + `mutool trace` + instrumentação (não era
  duplicação nem duplo desenho — era grelha curta por descida de nível errada,
  `total_descent` inflado e `min_overlap` ausente).
- Correção nos pontos certos, com matrizes/casos de 2 linhas guardados (5 testes
  green de não-regressão) e o sintoma corrigido com prova numérica (4 peças =
  vanilla; passo entre linhas 13.156pt ≈ 13.16pt do vanilla).
- 30/30 secções revalidadas sem regressões novas; benchmark 7 cenários sem
  regressão; suíte 5651 verde; lint zero.
- Achado novo registado (não tratado neste passo): matriz aumentada sem a linha
  vertical de `augment` (secção 21 — já presente em P944).
- Nota do passo (itens adiados pelo dono) mantida: gregas literais e termos de
  `attach.rs` seguem candidatos a passos próprios; "centralização em parênteses/
  linha de fração fora do centro" — **reconfirmado após este fecho**: com as
  células a tamanho cheio e o `total_descent` corrigido, a grelha e os
  delimitadores ficam centrados no eixo como o vanilla (ver crops das secções
  5/7/9/21) — não sobrou sintoma separado que justifique investigação própria;
  a linha de fração fora do centro remanescente é a classe de `frac.rs` (factor
  de tamanho, scope-out já registado em §6).
