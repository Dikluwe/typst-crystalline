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

## 5. Conclusão

- A diferença de paginação **persiste** após P544 e P552: cristalino **5 páginas**, vanilla **2 páginas** para `#lorem(1200)` em `#set page(columns: 2)`.
- A medição antiga de P538c/g não estava desactualizada no número; estava desactualizada na **causa**. A causa não é largura de palavra nem footnotes, mas o cálculo geométrico de `columns::layout`.
- A correção requer mudanças em `01_core/src/rules/layout/columns.rs` (e interacção com `cursor.rs`):
  1. Calcular `column_width` a partir da largura útil da página (`page_width - 2 × margin`).
  2. Ajustar a largura da região de trabalho dentro de cada coluna para que o layout preencha toda a largura útil da coluna (actualmente perde margem à direita).
- O item no inventário de decisões pendentes foi actualizado com a medição nova e a causa. Continua **aberto** até a correção geométrica ser implementada e validada.

---

## 6. Ficheiros de verificação

- `/tmp/p553-cols-long.typ`
- `/tmp/p553-cristalino.pdf`
- `/tmp/p553-vanilla-014.pdf`
- `/tmp/p553-vanilla-015.pdf`
- `/tmp/p553-cols-liberation.typ`
- `/tmp/p553-vanilla-014-liberation.pdf`
- `/tmp/p553-vanilla-015-liberation.pdf`

Temporários, não commitados.
