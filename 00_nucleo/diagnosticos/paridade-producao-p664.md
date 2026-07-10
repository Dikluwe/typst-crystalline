# Relatório de Paridade — P664

**Passo:** 664  
**Data:** 2026-07-09  
**Foco:** Testar directamente, sem depender de auto-rotulagem nos relatórios, se argumentos nomeados aceites pelo cristalino também o são pelo vanilla de referência.  
**Dependências:** P663 (auditoria por palavras-chave), P662 (padrão de reversão de extensões de linguagem por erro).  
**Hash do commit com as alterações:** `PENDING`

---

## 1. Método

Criou-se um script (`temp_p664/auditoria_p664.py`) que gera documentos mínimos e compila-os com os dois binários:

- **Cristalino:** `target/release/typst`
- **Vanilla:** `lab/typst-original/target/release/typst` (Typst 0.15.0)

Foram testadas duas categorias:

1. **Argumentos que o cristalino reconhece** (extraídos de `01_core/src/rules/eval/rules.rs`):
   - `heading.numbering`
   - `math.equation.numbering`
   - `document.title`, `author`, `keywords`
   - `page.width`, `height`, `margin`, `numbering`, `columns`
   - `figure.numbering`
   - `table.numbering`
   - `par.leading`
   - `text.bold`, `italic`, `size`, `fill`, `weight`, `tracking`, `lang`, `font`, `dir`

2. **Argumentos do vanilla ausentes ou não suportados no cristalino** (selecção de argumentos comuns):
   - `text`: `stretch`, `style`, `smallcaps`, `underline`, `overline`, `strike`, ...
   - `page`: `paper`, `flipped`, `binding`, ...
   - `par`: `justify`, `first-line-indent`, `outline`, ...
   - `heading`: `level`, `outlined`, `bookmarked`, ...
   - `figure`: `caption`, `placement`, `bookmarked`, ...
   - `table`: `columns`, `rows`, `fill`, `align`, `stroke`, `repeat-header`, ...
   - `grid`: `columns`, `rows`, `fill`, `align`, `stroke`, `gutter`, ...
   - `list`: `marker`, `indent`, `tight`, `spacing`
   - `enum`: `numbering`, `start`, `full`, `indent`, `tight`, `spacing`
   - `raw`: `lang`, `theme`, `block`, `tab-size`

Total de testes: **92**.

---

## 2. Resultados

### 2.1 Argumentos reconhecidos pelo cristalino — divergências

| Argumento | Vanilla | Cristalino | Nota |
|---|---|---|---|
| `table.numbering` | rejeita (`unexpected argument: numbering`) | aceita | Extensão consciente P459; já documentada. |
| `text.bold` | rejeita (`unexpected argument: bold`) | aceita | **Diferença de linguagem não confirmada.** |
| `text.italic` | rejeita (`unexpected argument: italic`) | aceita | **Diferença de linguagem não confirmada.** |

Todos os outros argumentos reconhecidos pelo cristalino (`heading.numbering`, `math.equation.numbering`, `document.title/author/keywords`, `page.width/height/margin/numbering/columns`, `figure.numbering`, `par.leading`, `text.size/fill/weight/tracking/lang/font/dir`) são aceites por ambos os compiladores.

### 2.2 Argumentos do vanilla ausentes no cristalino — divergências

Nenhum argumento do vanilla foi encontrado como **aceite pelo vanilla e rejeitado pelo cristalino** de forma inesperada. O padrão observado é:

- Quando o cristalino não suporta um target (`enum`, `raw`, `grid`, etc.), emite **warning** e continua a compilação.
- O vanilla **rejeita** argumentos inválidos para esses targets.
- Para argumentos de `text` não suportados (`stretch`, `smallcaps`, `underline`, etc.), o cristalino emite warning de "propriedade 'X' ainda não suportada" e continua; o vanilla rejeita.

Estas divergências são **diferenças de tratamento de erro** (warn vs. reject), não extensões de linguagem intencionais. O utilizador é avisado de que o argumento não funciona.

### 2.3 Validação cruzada de valores

Confirmou-se manualmente que o vanilla aceita os argumentos canónicos alternativos:

- `#set text(weight: "bold")` → aceite
- `#set text(style: "italic")` → aceite
- `#set heading(numbering: "1")` → aceite
- `#array.sorted(key: x => -x)` → aceite por ambos

---

## 3. Descoberta principal: `text.bold` e `text.italic`

O cristalino aceita `#set text(bold: true)` e `#set text(italic: true)`. O vanilla 0.15.0 rejeita ambos, exigindo `#set text(weight: "bold")` / `#set text(style: "italic")`.

Esta diferença existe pelo menos desde P525, onde o relatório refere `#set text(italic: true)` como forma de obter itálico no cristalino, dado que `text.style` ainda não estava implementado. Não foi registada como decisão consciente de extensão de linguagem.

### 3.1 Impacto

- Documentos que usem `#set text(bold: true)` / `#set text(italic: true)` no cristalino **falham no Typst real**.
- Internamente, o cristalino usa as flags `bold`/`italic` no `TextStyle` e no shaper; `*...*` e `_..._` também dependem delas.

### 3.2 Classificação

**Diferença de linguagem introduzida sem confirmação.** Segue a mesma categoria de `variant: (eixo: valor)` (P660/P662), embora `bold`/`italic` sejam atalhos semanticamente equivalentes a `weight`/`style`.

---

## 4. Decisão e próximos passos

### 4.1 `table.numbering`

- **Classificação:** extensão de linguagem consciente.
- **Acção:** nenhuma. Já documentada em P639/P661.

### 4.2 `text.bold` / `text.italic`

- **Classificação:** diferença de linguagem por erro / atalho não confirmado.
- **Acção:** propor um passo próprio (P665 ou posterior) para decidir entre:
  1. **Reverter:** remover `#set text(bold: ...)` / `#set text(italic: ...)` do parser e guiar utilizadores para `weight`/`style`. Manter internamente `bold`/`italic` apenas para `*...*` / `_..._` se necessário, ou mapeá-los para `weight`/`style`.
  2. **Manter como extensão consciente:** documentar claramente que `bold`/`italic` são atalhos cristalinos não portáveis.

Recomendação preliminar: **reverter**, para manter a linguagem alinhada com o vanilla. A complexidade é maior do que `variant` porque `bold`/`italic` são usados por `*...*` / `_..._` e pelo shaper, mas a equivalência semântica com `weight`/`style` facilita a transição.

### 4.3 Tratamento de erros (warn vs. reject)

- **Classificação:** diferença de implementação / UX.
- **Acção:** não é prioridade para esta auditoria. O utilizador é avisado de que o argumento não é suportado.

---

## 5. Validação

O script de auditoria e os resultados brutos encontram-se em:

- `temp_p664/auditoria_p664.py`
- `temp_p664/p664-resultados.json`

Não houve alterações de código neste passo.

---

## 6. Estado de fecho

- [x] Argumentos nomeados reconhecidos pelo cristalino listados.
- [x] Cada argumento testado directamente contra o vanilla.
- [x] Caso inverso (argumentos do vanilla ausentes no cristalino) testado.
- [x] Divergências novas classificadas.
- [x] Próximo passo proposto para `text.bold` / `text.italic`.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p664.md`.
