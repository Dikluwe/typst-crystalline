# Paridade de Produção — Passo 553

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst` (após P552)
- Vanilla 0.14.2: `/usr/local/bin/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `pdfinfo`, `pdftotext` (poppler), `mutool` 1.23.10

---

## 1. Objetivo

Re-medir a paginação de `#set page(columns: 2)` depois de P544 (correcção de largura de palavra) e P552 (correcção de footnotes em colunas). O inventário registava o documento como produzindo **5 páginas no cristalino contra 2 no vanilla** para `#lorem(1200)`. Esta medição datava de antes de P544; era necessário confirmar se a diferença persistia e qual a causa actual.

---

## 2. Método

Documento de teste:

```typst
#set page(columns: 2)
#lorem(1200)
```

Comandos:

```bash
./target/release/typst /tmp/p553-cols-long.typ /tmp/p553-cristalino.pdf
pdfinfo /tmp/p553-cristalino.pdf | grep Pages
pdftotext /tmp/p553-cristalino.pdf - | wc -w

/usr/local/bin/typst compile /tmp/p553-cols-long.typ /tmp/p553-vanilla-014.pdf
pdfinfo /tmp/p553-vanilla-014.pdf | grep Pages
pdftotext /tmp/p553-vanilla-014.pdf - | wc -w

lab/typst-original/target/release/typst compile /tmp/p553-cols-long.typ /tmp/p553-vanilla-015.pdf
pdfinfo /tmp/p553-vanilla-015.pdf | grep Pages
pdftotext /tmp/p553-vanilla-015.pdf - | wc -w
```

Para isolar o factor fonte, repetiu-se o teste no vanilla 0.14.2/0.15.0 forçando a mesma família sans-serif usada pelo cristalino:

```typst
#set page(columns: 2)
#set text(font: "Liberation Sans")
#lorem(1200)
```

---

## 3. Resultados

| Versão | Fonte | Páginas | Palavras extraídas |
|---|---|---|---|
| Cristalino | CrystallineFont (sans-serif) | **5** | 1200 |
| Vanilla 0.14.2 | LibertinusSerif-Regular | **2** | 1200 |
| Vanilla 0.15.0 | LibertinusSerif-Regular | **2** | 1213 |
| Vanilla 0.14.2 | Liberation Sans | **3** | — |
| Vanilla 0.15.0 | Liberation Sans | **3** | — |

**Interpretação imediata:**

- A contagem de palavras é praticamente idêntica; não há perda de conteúdo.
- A diferença de 5 → 2 páginas não é explicada pela correcção de largura de palavra de P544 nem pela correcção de footnotes de P552.
- A fonte padrão do cristalino (sans-serif) ocupa mais avanço horizontal que a fonte padrão do vanilla (serif). Mesma fonte no vanilla: **3 páginas**. Logo, o bug geométrico de colunas é responsável por aproximadamente **2 páginas extra** (5 → 3); a diferença restante (3 → 2) é da escolha de fonte padrão, que é outro eixo de paridade.

---

## 4. Diagnóstico

### 4.1 Onde está o erro geométrico

`01_core/src/rules/layout/columns.rs:75` calcula a largura base das colunas a partir da largura **total** da página:

```rust
let full_width = layouter.regions.current.width;   // 595.28 pt
```

Com duas colunas e gutter default (`0.04 × 595.28 = 23.81 pt`):

```text
column_width = (595.28 - 23.81) / 2 = 285.73 pt
```

No entanto, o layout interno de cada coluna (`layout_word` / `layout_chunk` em `cursor.rs`) usa:

```rust
let right_margin = self.regions.current.width - self.page_config.margin;
```

e o cursor começa em `page_config.margin` (`70.87 pt`). A área útil efectiva da coluna torna-se assim:

```text
column_width_efetiva = 285.73 - 2 × 70.87 ≈ 144 pt
```

Na prática, a primeira linha da coluna esquerda no cristalino termina em `xMax ≈ 193.9 pt`, o que dá uma largura ocupada de apenas `~123 pt` (palavras maiores e espaçamento justificam a diferença para o limite teórico).

### 4.2 O que o vanilla faz

No vanilla 0.15.0 (`LibertinusSerif-Regular`), a primeira linha da coluna esquerda termina em `xMax ≈ 252.0 pt` e a coluna direita começa em `xMin ≈ 306.7 pt`. Isso corresponde a:

- largura útil de cada coluna ≈ `211 pt`
- gutter ≈ `24 pt`
- largura útil total ≈ `595.28 - 2 × 70.87 = 453.5 pt`

Ou seja, o vanilla calcula a largura das colunas a partir da **área útil** da página (`page_width - 2 × margin`), não da largura total.

### 4.3 Causa nova vs factores conhecidos

| Factor | Estado |
|---|---|
| Largura de palavra / quebra de linha (P544) | **Descartado** — contagem de palavras mantém-se; linhas quebram cedo por falta de largura, não por medição errada de palavra. |
| Footnotes em colunas (P552) | **Descartado** — o documento de teste não tem footnotes. |
| Cálculo de `column_width` a partir da largura total | **Causa nova confirmada** — explica por que as colunas cristalinas são substancialmente mais estreitas. |
| Fonte padrão sans-serif vs serif | **Factor separado** — com mesma fonte no vanilla, ainda há 3 vs 5 páginas. |

---

## 5. Alterações implementadas

### 5.1 `columns::layout`

- `page_width` passou a ser distinguido de `usable_width = page_width - 2 × margin`.
- `column_width` passou a ser calculado a partir de `usable_width`:
  ```text
  column_width = (usable_width - (count - 1) × gutter) / count
  ```
- Gutter default mantém-se proporcional à largura total da página (`0.04 × page_width`), para paridade com a medição vanilla.
- Adicionada `column_region_width = column_width + 2 × margin`, a largura da mini-página usada durante o layout de cada coluna.

### 5.2 `columns::layout_segmented` e `columns::layout_flow`

- A região de trabalho de cada coluna passou a ter largura `column_region_width` (em vez de `column_width`).
- O cursor continua a iniciar em `margin`, mas agora o `right_margin` interno (`width - margin`) coincide com o fim da área útil da coluna, preenchendo toda a largura disponível.
- A translação horizontal dos items mantém-se: `dx = column_x_offsets[idx] - margin`.

---

## 6. Validação final

### 6.1 Paginação de `#lorem(1200)`

Após a correção:

| Versão | Fonte | Páginas | Palavras |
|---|---|---|---|
| Cristalino | CrystallineFont (sans-serif) | **3** | 1200 |
| Vanilla 0.15.0 | LibertinusSerif-Regular | **2** | 1213 |
| Vanilla 0.15.0 (Liberation Sans) | Liberation Sans | **3** | — |

A correção geométrica eliminou 2 das 3 páginas extra. A diferença restante (3 vs 2) é da fonte padrão sans-serif do cristalino vs serif do vanilla.

### 6.2 Regressões em colunas

- `#set page(columns: 2)` com footnotes em ambas as colunas continua a numerar `[1]`, `[2]` e a colocar notas no fundo de cada coluna (P552).
- `#columns(2)[...]` com footnotes continua a empilhar as notas na primeira coluna (P552).

### 6.3 Comandos de validação

```bash
cargo build --release        # ok
cargo test --workspace       # 3568 + 573 + 24 + 2 + 21 + 2 passed; 0 failed
crystalline-lint .           # 0 violations
```

---

## 7. Conclusão

- A diferença de paginação foi **reduzida** de 5 vs 2 para **3 vs 2** páginas.
- A causa geométrica identificada em P553 foi **corrigida** em `01_core/src/rules/layout/columns.rs`.
- O item do inventário foi actualizado: a geometria de colunas está corrigida; a diferença restante para 2 páginas do vanilla é atribuída à fonte padrão do cristalino, que é um eixo de paridade separado.

---

## 8. Ficheiros de verificação

- `/tmp/p553-cols-long.typ`
- `/tmp/p553-cristalino.pdf`
- `/tmp/p553-vanilla-014.pdf`
- `/tmp/p553-vanilla-015.pdf`
- `/tmp/p553-cols-liberation.typ`
- `/tmp/p553-vanilla-014-liberation.pdf`
- `/tmp/p553-vanilla-015-liberation.pdf`

Temporários, não commitados.
