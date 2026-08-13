# Passo 1027 — `#text(size:)` ignorado em math bare

**Data**: 2026-08-13
**Estado**: **fechado**. Fases A, B, C e D feitas. Gate ADR-0127 categoria 2 aplicado e
aprovado via confirmação do dono; L0 actualizado antes do código.

---

## Resumo

O Passo 1026 encontrou, ao construir uma régua contínua para delimitadores, que
`#text(size: 40pt)[x]` dentro de `$...$` sem equação aninhada era renderizado a
~5,5 pt em vez de ~20 pt. A causa estava em `needs_external_layout`
(`01_core/src/compiler/math/layout/mod.rs:252-263`): a deny-list de P994 subia para
`layout_external` apenas conteúdo que continha `Equation`/`Boxed`/`Align`/`Pad`/`Block`.
`Content::Styled(Text, [Size])` não estava na lista, ficando no caminho de texto de math,
que achata o conteúdo para `plain_text()` e descarta o override de estilo.

A correcção alarga `needs_external_layout` para `Styled` cujo `StyleDelta` contém
propriedades tipadas de texto que o catch-all não consegue aplicar (`size`, `fill`, `weight`,
`font`, `tracking`, `leading`, `lang`, `bold`, `italic`). O corpo continua a ser verificado
recursivamente pela deny-list original.

---

## Proveniência

`HEAD = a5b23004c`, árvore de trabalho limpa (dois ficheiros de materialização não
acompanhados: `typst-passo-1027.md` e `typst-passo-1028.md`).

Binário cristalino reconstruído no HEAD actual (`cargo build --release -p typst-wiring`);
referência vanilla `/usr/local/bin/typst` e `lab/typst-original/target/release/typst`,
baseline ratificado `a51e02804`.

Documentos e scripts de medição em `temp/p1027/`:
- `medir.py` — medição de dimensões de ink via `pdftotext -bbox`.
- `medir_pos.py` — medição de posicionamento vertical.
- Vários `.typ` de verificação visual.

---

## Fase A — histórico da adenda 2 de P994

A adenda 2 de P994 estreitou `needs_external_layout` porque a delegação cega de markup
puro para `layout_external` causava regressão real: `"dado"` em `$ 9 & "dado" $` e
`$ sin(x) $` ficavam centrados no eixo (~4,5 pt abaixo da baseline vizinha). A regra final
foi a deny-list actual.

O custo dessa regra — perder overrides de `text()` em markup bare — não tinha sido medido.
A hipótese do Passo 1027 confirmou-se: o estreitamento foi deliberado e correcto para
markup puro, mas excluiu `Styled` com estilos de texto. A correcção tem de distinguir os
dois casos.

---

## Fase B — alcance das propriedades de `text()`

Medição via `pdftotext -bbox` e rasterização 300 dpi (mesmo documento nos dois binários):

| documento | vanilla | cristalino | conclusão |
|---|---|---|---|
| `$ #text(size: 40pt)[x] $` | ink 21,12 × 40,00 pt | ink 5,81 × 11,00 pt | **perdido** |
| `$ #text(fill: red)[x] $` | sRGB (cor), ink 5,81 × 11,00 pt | Grayscale (sem cor), ink 5,81 × 11,00 pt | **cor perdida** |
| `$ #text(weight: "bold")[x] $` | ink 6,68 × 11,00 pt | ink 5,81 × 11,00 pt | **peso perdido** |
| `$ #text(style: "italic")[x] $` | ink 5,81 × 11,00 pt | ink 5,81 × 11,00 pt | sem diferença (math já itálico) |
| `$ #text(style: "normal")[x] $` | ink 5,50 × 11,00 pt | ink 5,50 × 11,00 pt | sem diferença mensurável |

Fora de math todas as propriedades aplicam-se; dentro de math, `size`, `fill` e `weight`
são ignorados quando não há equação aninhada. `style` não evidencia perda porque o texto
matemático já é itálico por defeito.

Posicionamento vertical para `$ a + #text(size: 40pt)[x] + b $`:
- vanilla: `a`/`b` na baseline (`yMin=8,374`), `x` grande centrado no eixo (`yMin=-15,0`).
- cristalino (pré-fix): todo o conteúdo fundido na baseline (`yMin=-1,237`), ignorando o
tamanho.

O posicionamento centrado no eixo para `size` grande é o alvo de paridade do vanilla.

---

## Fase C — gate ADR-0127 e L0

Mudança de output visual em qualquer documento com `text()` bare dentro de math → gate
ADR-0127 categoria 2 (comportamento por defeito).

O Prompt L0 `00_nucleo/prompts/compiler/math/layout/_comum.md` §P994 adenda 4 foi
actualizado com as medições, as respostas às três perguntas pendentes e a decisão:
alargar `needs_external_layout` para `Styled` com delta tipado de texto. Hash resselado
antes do código.

---

## Fase D — implementação e validação

### Código alterado

- `01_core/src/compiler/math/layout/mod.rs:252-279` — `needs_external_layout` alargada.

### Testes adicionados

Em `01_core/src/compiler/layout/tests.rs` (secção `p994_tests`):

- `p1027_text_size_bare_em_math`
- `p1027_text_weight_bare_em_math`
- `p1027_text_fill_bare_em_math`
- `p1027_markup_puro_em_math_mantem_baseline`

### Resultados

- `cargo test --workspace`: **5846 passed, 0 failed**.
- `crystalline-lint .`: **0 violações** (apenas 3 warnings V7 pré-existentes de prompts
  órfãos).
- Validação visual (rasterização 300 dpi):
  - `$ #text(size: 40pt)[x] $` → imagem idêntica ao vanilla (RMSE 0).
  - `$ #text(fill: red)[x] $` → sRGB/vermelho aplicado, imagem idêntica ao vanilla.
  - `$ #text(weight: "bold")[x] $` → peso aplicado, imagem idêntica ao vanilla.
  - `$ 9 & "dado" $` → markup puro mantém alinhamento na baseline (não-regressão).
  - `$ a + #text(size: 40pt)[x] + b $` → posicionamento vertical bate com vanilla ao
    milésimo de ponto.

### Commits

- `612628498` — docs(L0): decisão de alargar `needs_external_layout` a `Styled` com
  estilos de texto.
- `b14cf867b` — fix(math): P1027 — `text()` bare em math aplica size/fill/weight.
