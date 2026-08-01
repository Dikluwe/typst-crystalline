# Passo 944 — Relatório (Fase A completa; Fase B aguarda hash do L0)

**Data**: 2026-07-31/2026-08-01
**Estado da árvore na medição**: commit `8c8fb3c97` (P943, HEAD de `Tekt`). Working tree:
untracked `00_nucleo/materialization/typst-passo-944.md`, `test_crystalline.pdf`,
`test_vanilla.pdf`; `git diff` limpo (edições DIAG de `eprintln!` em
`attach.rs`/`stretchy.rs` **revertidas** após as medições).
**Artefactos**: `temp/p944/` (PDFs, PNGs, bboxes, worktrees `wt-p924`/`wt-p917`/`wt-pre914`).

---

## 1. Resumo executivo

Os dois defeitos confirmados visualmente pelo dono (segundo `lim` da secção 4 com colisão
`x→∞`; delimitadores de matriz curtos nas secções 5/21) **não são uma regressão da frente
de performance de fontes (P925-943)**. A reescrita de `Coverage`/`covering()` (P937/938/942)
está **exonerada por identidade de pixels**: todos os binários de P927 a P943 produzem
render **pixel-idêntico** nas secções 4, 5 e 21, e o binário compilado do commit de fim da
frente de geometria (`da18ea9f3`, fim de P924, anterior a P925) também.

A causa raiz é **mais antiga e única para os dois defeitos**: o cristalino **nunca
implementou o `EquationElem::show_set` do vanilla** que fixa a fonte das equações a
`New Computer Modern Math` (`lab/typst-original/crates/typst-library/src/math/equation.rs:197-201`).
Em modo math, o cristalino usa a fonte de texto do documento ("New Computer Modern") como
primária — e essa fonte (`NewCM10-Regular.otf`) **tem uma tabela MATH stub**
(`LowerLimitGapMin = 0`, sem `MathVariants`), que "ganha" tanto o primeiro passe de
`covering` (P912) como a resolução de `math_constants` (P893).

## 2. Fase A — bisseção (proveniência registada por medição)

Método: compilação de `.typ/typst-math-comprehensive-test.typ` (30 secções, o mesmo desde
P894) com cada binário; `mutool draw -r 72`; crops das secções 4/5/21; comparação por
`PIL.ImageChops.difference` (`None` = idêntico).

| Binário / commit | Estado | Sec 4 | Sec 5 | Sec 21 |
|---|---|---|---|---|
| `typst-p927` (28 jul) | perf front, pré-P937 | defeito presente | defeito presente | defeito presente |
| `typst-p933-fixed` | idem | idêntico a p927 | idêntico | idêntico |
| `typst-p937` | coverage exata Fase B | idêntico | idêntico | idêntico |
| `typst-p938` | coverage lazy | idêntico | idêntico | idêntico |
| `typst-p942` | sem re-verificação glyph_index | idêntico | idêntico | idêntico |
| `typst` (P943, `8c8fb3c97`) | HEAD | (referência) | (referência) | (referência) |
| worktree `wt-p924` @ `da18ea9f3` | **fim da frente de geometria, pré-P925** | **idêntico a P943** | **idêntico** | — |
| worktree `wt-p917` @ `0e98008a2` | pré-P918 | defeitos relacionados **piores** (linhas das matrizes sobrepostas; subscrito do `lim` colide com a fração) | idem | — |
| worktree `wt-pre914` @ `ccbe6816c` | pré-P914 | incomparável (render A4; `page(width/height: auto)` ainda não suportado) | — | — |

**Conclusão da bisseção**: o defeito, na sua forma atual, está presente desde pelo menos
`da18ea9f3` (fim de P924) e permaneceu **byte-a-byte estável** através de toda a frente de
performance (P925→P943). Em P917 a mesma zona já estava defeituosa (forma anterior, pior).
Não é regressão nova — é defeito antigo nunca fechado (Fase A, item 3 do passo).

**Suspeita inicial do passo (reescrita `Coverage`/`covering()` P937-942)**: **refutada**
pela identidade de pixels P927↔P943. `covering()` mudou de implementação mas não de
resultado neste documento.

## 3. Fase A — causa exata (instrumentação do código atual)

Instrumentação temporária (`eprintln!`, revertida) em `stretchy.rs` e `attach.rs`, sobre
`temp/p944/min.typ` (2 matrizes + 2 `lim`, `#set text(font: "New Computer Modern")`):

### 3.1 Matrizes (secções 5/21)

```
stretchy c='(' min_height_du=2244 target_du=2144 variants=[] style.math=true
stretchy c='(' sem variante; assembly.parts=0
```

`vertical_glyph_variants('(')` e `vertical_glyph_assembly('(')` devolvem **vazio** →
`layout_stretchy_delimiter` (`01_core/src/engine/math/layout/stretchy.rs`) cai no fallback
de glifo base → parêntese de 1 linha, não centrado no eixo, colado à última linha da
matriz. Medição de posições no PDF (pdftotext -bbox): na matriz 3×3, as linhas ficam em
yMin 1304.9/1316.1/1327.3 e o `(` ocupa yMin 1323.9–1334.9 (~11pt = 1 linha, na baseline
da última linha).

**Porquê vazio**: `covering('(')` (P912, `03_infra/src/font_metrics.rs:950-957`) escolhe a
primeira primária com tabela MATH que cobre `(`. Com `style.font = "New Computer Modern"`,
a primeira primária é `NewCM10-Regular.otf` — **que tem tabela MATH, mas stub** (medido via
fontTools: `MathVariants` ausente → extração devolve vazio). A fonte MATH real
(`NewCMMath-*.otf`) tem 8 variantes verticais para `(` (997–2991du) e assembly de 3 partes.

### 3.2 `lim` (secção 4, segundo caso)

```
attach-limits base_descent=0 lower_gap=0 sb.ascent=4.4088 y_sub=4.4088
              consts.upem=1000 consts.lower_limit_gap_min=0 style.math=true
```

`math_constants()` (`03_infra/src/font_metrics.rs:1358-1369`, P893) devolve as constantes
da **primeira primária com tabela MATH** — outra vez o stub `NewCM10-Regular.otf`:
`LowerLimitGapMin = 0` (fontTools; o real em NewCMMath é **167du** e
`LowerLimitBaselineDropMin = 600du`). Com `lower_gap = 0`, `y_sub = base_descent + 0 +
sb.ascent`, e como `sb.ascent` é de **tinta** (P921), `x→∞` (4.41pt) fica 1.45pt mais alto
que `x→0` (5.86pt). Medição contra o vanilla (bbox): vanilla coloca o topo do subscrito
1.39/1.74pt acima da baseline do `lim` nos dois casos; o cristalino coloca 3.38/4.83pt —
daí a colisão `x→∞` × `lim`.

### 3.3 Causa raiz única (fonte do vanilla)

`lab/typst-original/crates/typst-library/src/math/equation.rs:197-201` — o
`EquationElem::show_set` do vanilla fixa, para **toda** a equação (inline e bloco):

```rust
out.set(TextElem::weight, FontWeight::from_number(450));
out.set(TextElem::font, FontList(vec![FontFamily::new("New Computer Modern Math")]));
```

O `#set text(font:)` do documento **não se aplica** dentro de equações no vanilla. O
cristalino (`01_core/src/engine/layout/equation.rs:60-61`) constrói
`math_style = TextStyle { math: true, ..self.style.clone() }` — herdando a fonte do
documento. O mecanismo de fallback por glifo (P890) mascara isto para glifos que a fonte de
texto não cobre (itálicos matemáticos U+1D4xx, gregos), mas `(` é coberto pela fonte de
texto e as constantes MATH são lidas do stub.

### 3.4 Sondagem end-to-end (sem código novo)

`temp/p944/min-mathfont.typ` = o mesmo caso com
`#set text(font: "New Computer Modern Math")`:

- `variants` povoado (8 variantes com `advance` 997–2991du); matriz 3×3 →
  `assembly.parts = 3` (extensores);
- `lower_gap = 1.837pt` (= 167du × 11pt), `y_sub = 6.97/5.76pt`;
- render visual (`temp/p944/min-mathfont.png`): delimitadores esticam sobre todas as
  linhas; os dois `lim` limpos, subscritos abaixo da base.

**Causa confirmada** (não presumida): os dois defeitos partilham a raiz "equações não
forçam New Computer Modern Math".

## 4. Achados secundários (scope-out proposto no L0)

1. `attach.rs` (braço `is_limits`) não implementa o termo
   `max(lower_limit_baseline_drop_min, …)` da fórmula vanilla
   (`lab/typst-original/crates/typst-layout/src/math/scripts.rs:306-310`); `MathConstants`
   não tem `lower_limit_baseline_drop_min`/`upper_limit_baseline_rise_min`. Residual
   sub-ponto após a correção raiz.
2. `sb.ascent` de tinta (P921) vs ascent de frame por métricas da fonte do vanilla —
   diferença residual na mesma casa.
3. `base_char = None` para bases `MathOp` (ex.: `lim`) — `compute_math_kern` não corre
   para limites de operadores (sem impacto visual medido neste documento).

## 5. Lição de processo (registo exigido pelo passo)

O `.typ` de 30 secções deveria ter sido revalidado após **qualquer** passo que mexesse em
`font_metrics`/`covering()`/`Coverage` — mesmo em passos classificados como "performance".
A frente P925-943 nunca rodou esse documento (só benchmarks canónicos e casos UTF-8
isolados). Ironia medida: desta vez a frente de performance era inocente — mas só a
revalidação do documento permitiu prová-lo (identidade de pixels P924↔P943) em vez de o
presumir. Proposta: incluir a compilação + diff de render do documento de 30 secções no
checklist de qualquer passo que toque `03_infra/src/font_metrics.rs`,
`01_core/src/engine/math/`, ou o pipeline de fontes.

## 6. Estado do protocolo

- Fase A: **completa** (bisseção + causa exata + sondagem end-to-end).
- L0: secção **P944** redigida em `00_nucleo/prompts/engine/layout/equation.md` —
  **aguarda** o dono guardar e calcular o hash (Regra de Ouro).
- Fase B (dois agentes, TDD): só após confirmação do hash. Escopo: override
  `font = [New Computer Modern Math]` + `weight = 450` em `layout_equation`
  (`01_core/src/engine/layout/equation.rs`), sem tocar em L3.
- Fase C: revalidação visual do documento de 30 secções inteiro após a correção.

---

## 7. Fase B — correção implementada (dois agentes, TDD)

L0 confirmado pelo dono ("continue"). Protocolo de dois agentes:

**Agente 1 (implementação, TDD)** — 3 testes novos em
`01_core/src/engine/layout/tests.rs` (`p944_equacao_bloco_usa_new_computer_modern_math`,
`p944_equacao_inline_usa_new_computer_modern_math`,
`p944_fonte_do_documento_nao_se_aplica_a_equacao`), verificados a FALHAR antes da
implementação. Implementação em `01_core/src/engine/layout/equation.rs` (ponto único,
junto ao P784): `math_style` fixa `font = FontList::single("New Computer Modern Math")` e
`weight = Some(450)`. Zero alterações em L3.

**Agente 2 (revisão cética)** — aprovado com 3 achados, todos resolvidos nesta emenda:

1. **(médio)** O número de equações numeradas era emitido com `self.style` (fonte do
   documento) — no vanilla é composto com a chain que inclui o show-set
   (`lab/typst-original/crates/typst-layout/src/math/mod.rs:217`,
   `layout_frame(engine, &counter, …, styles)`). **Corrigido** (TDD: teste
   `p944_numero_de_equacao_numerada_usa_new_computer_modern_math`, red → green): os dois
   pontos de emissão (directa e adiada por `width: auto`) e a medição da largura usam
   agora `math_style`. L0 actualizado com esta emenda.
2. **(doc)** Comentário sobre `MathStyled` impreciso — corrigido (código e L0):
   `mono`/`serif`/`sans`/`upright`/`bold` não tocam `style.font`, actuam por codepoints.
3. **(doc)** `@updated` do header de `equation.rs` actualizado.

Risco teórico registado pelo revisor e verificado inalcançável: `faux_bold_stroke_pt > 0`
para weight 450 (~0.073pt) só é consumido no cenário `FontScenario::Type1` (Helvetica
legado); math embutida vai por Cidfont/Multifont. Flake de 2 testes em `typst-infra` visto
uma vez pelo revisor; não reproduzível em 11 runs subsequentes — registado como
pré-existente e não relacionado (a alteração é só L1).

**Validação Fase B** (commit base `8c8fb3c97` + alterações P944, working tree não
commitado — `git diff HEAD --stat`: `01_core/src/engine/layout/equation.rs`,
`01_core/src/engine/layout/tests.rs`, `00_nucleo/prompts/engine/layout/equation.md`,
este relatório):

- `cargo test --workspace`: **5640 passed, 0 failed** (4810 core incl. 4 testes P944).
- `crystalline-lint .`: **zero violations** (`@prompt-hash` de `equation.rs` → `41244bcc`
  via `--fix-hashes`); resta só o warning V7 pré-existente e alheio ao passo
  (`package_version_resolution.md` órfão).
- Sondagem end-to-end `temp/p944/min.typ` (com a fonte do documento original): matrizes
  2×2/3×3 esticam (variante + assembly de 3 partes), ambos os `lim` limpos —
  `temp/p944/min-fixed.png`.

## 8. Fase C — revalidação do documento de 30 secções inteiro

Método: `out-fixed.pdf` (HEAD+correção) comparado (a) com `out-p943.pdf` (pré-correção) —
detecção de regressões introduzidas pelo override — e (b) com `test_vanilla.pdf` —
paridade. 30/30 secções revistas visualmente lado a lado (`temp/p944/oldnew-batch*.png`,
`temp/p944/cmp-batch*.png`).

### 8.1 Os dois defeitos confirmados — prova geométrica

- **Secção 4, `lim_(x→∞) 1/x`**: antes, subscrito com topo a 4.83pt acima da baseline do
  `lim` (colisão; vanilla: 1.74pt). Depois: 2.60pt — sem colisão
  (`temp/p944/min-fixed.png`, zoom). Residual sub-ponto vs vanilla registado no scope-out
  (ascent de tinta P921 vs ascent de frame do vanilla). O primeiro `lim` ficou em 1.40pt
  (vanilla: 1.39pt).
- **Secções 5/21, matrizes**: antes, `(` único de 11pt colado à baseline da última linha.
  Depois: assembly cobre yMin 1309.4→1338.3 (~29pt) abraçando as 3 linhas da matriz 3×3;
  2×2 usa variante pré-fabricada. `{` da secção 21 e matriz com `augment` idem.

### 8.2 Regressões introduzidas pelo override: **nenhuma detectada**

Comparação pré/pós nas 30 secções: as únicas diferenças são as correcções pretendidas e
melhorias directas do mesmo mecanismo (delimitadores/constantes da fonte MATH real):
secções 7 (`binom`), 9 (chave de `cases`), 12 (`sqrt`), 13 (radical sobre somatório),
17 (limites de `sum` duplo e de `⋃`/`⋂`), 18 (radicais aninhados), 22 (grupos
delimitados esticam), 23/25/28/29/30 (limites abaixo de `lim`/`max`/`min`/`sum` em
display — confirmado no vanilla `op.rs:44-101` que `max`/`min`/`lim`/`det`/`gcd` têm
`limits: true` e `log`/`ln`/`exp` não — `log_a` verificado com subscrito lateral),
27 (radical sobre fracção). Secção 16 (`bb`/`cal`/`frak`/`mono`/`serif`/`sans`/`upright`/
`bold`/`italic`) **idêntica** — confirma que `MathStyled` não é afectado pelo override.

### 8.3 Achados pré-existentes (presentes no P943, inalterados por este passo)

Registados para triagem futura — nenhum introduzido ou agravado por P944:

1. Secção 22: `lr(...)` renderiza como texto literal (função `lr` não implementada);
   os delimitadores do grupo já esticam, o prefixo `lr` é que permanece.
2. Secções 25/26/28: `zeta`/`Gamma`/`Psi`/`psi`/`chi`/`omega`/`alpha`/`tau` renderizam
   como palavras literais em vez dos símbolos gregos correspondentes.
3. Secção 8: quebras de linha `\\` em equações multilinha aparecem como texto literal.
4. Secções 9/24: texto citado (`"se"`, `"is natural"`) em itálico matemático em vez de
   upright do vanilla.
5. Secção 10: etiquetas de `underbrace`/`overbrace` sobrepostas ao corpo.
6. Secções 12/15/18: fracções display ligeiramente mais pequenas que o vanilla
   (sizing de fracções aninhadas).
7. Secção 20: `@ref` a equações formata "(1)" vs "Equation 1" do vanilla.

### 8.4 Artefacto para o dono

`test_crystalline.pdf` (raiz) **regenerado** com o binário corrigido para comparação
directa com `test_vanilla.pdf`. O render de referência desta revisão está também em
`temp/p944/out-fixed.pdf`.

## 9. Fecho

- Bisseção: regressão **não** é da frente P925-943 — `Coverage`/`covering()` exonerada
  por identidade de pixels (P924↔P943).
- Causa exacta: ausência do `EquationElem::show_set` de fonte do vanilla — confirmada por
  instrumentação, fontTools e sondagem end-to-end.
- Correção: L1 apenas (`equation.rs`), 4 testes novos, emenda do número numerado via
  revisão cética. Suite 5640 verde, lint zero.
- Revalidação: 30/30 secções revistas; zero regressões; 7 achados pré-existentes
  registados (§8.3); lição de processo em §5.
