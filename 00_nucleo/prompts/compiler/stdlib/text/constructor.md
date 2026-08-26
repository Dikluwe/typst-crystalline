# Prompt L0 — `compiler/stdlib/text/constructor` — `text(...)` e validação de argumentos
Hash do Código: c7807d84

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/constructor.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história acumulada destas nativas. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/mod.rs::TextElem` — o nó cristalino cobre a
chamada de função (`#text(...)`); a set rule homónima vive em `compiler/eval/rules.rs`
e partilha a lista `VANILLA_TEXT_SET_PROPS`.

---

## Contexto

`text(...)` é o único constructor deste módulo com superfície de argumentos larga: os
seis validadores privados (`require_length`, `parse_text_weight`, `parse_text_style`,
`parse_text_lang`, `parse_text_font`, `parse_text_edge`) existem só para ele e nunca
foram tocados por outro passo — a co-mudança move-os sempre e apenas com `native_text`.

**Princípio de paridade**: qualquer argumento nomeado válido em `#set text(...)` é
também válido em `#text(...)`, com a mesma validação de tipo e o mesmo transporte.

## Instrução

**Posicionais**: `body: Content | Str` (obrigatório); `fill: Color` opcional como
primeiro posicional — se o primeiro item for `Value::Color` e não houver `fill:`
nomeado, é consumido como cor de preenchimento e o body passa a ser o segundo item.
Mais posicionais do que `fill + body` → erro.

**Nomeados** — todos os campos settable do `TextElem` vanilla
(`VANILLA_TEXT_SET_PROPS`) são aceites. Os implementados convertem-se para o mesmo
transporte que a set rule:

| Nome | Tipo | Transporte | Validador |
|------|------|------------|-----------|
| `size` | `Length` | `Style::Size(Pt)` | `require_length` |
| `fill` | `Color \| none` | `Style::Fill` (ou remove) | inline |
| `weight` | `Int` (raw) \| `Str` (nome simbólico) | `Style::Weight(u16)` | `parse_text_weight` |
| `style` | `"normal" \| "italic" \| "oblique"` | custom `"text.style"` | `parse_text_style` |
| `tracking` | `Length` | `Style::Tracking` | `require_length` |
| `lang` | `Str` (BCP-47) | `Style::Lang` | `parse_text_lang` |
| `font` | `Str \| Array[Str] \| Dict` | `Style::Font(FontList)` | `parse_text_font` |
| `top-edge` / `bottom-edge` | métrica `Str` ou `Length` | custom `"text.top-edge"` / `"text.bottom-edge"` | `parse_text_edge` |
| `dir` | `Dir` horizontal | custom `"text.dir"` | inline |
| `variations` | `Dict` | custom `"text.variations"` (fold) | `FontVariations::from_value` |

- `bold` / `italic` → `unexpected argument: {name}` (não são campos do vanilla; o
  vanilla usa `weight:` / `style:`).
- Nome dentro de `VANILLA_TEXT_SET_PROPS` mas ainda não capturado → aceite sem efeito
  (scope-out silencioso).
- Nome fora da lista → erro hard `unexpected argument: {name}`.
- `dir` vertical (`ttb`/`btt`) → `text direction must be horizontal`.

**Emissão**: acumula os estilos numa `Styles`; devolve `Content::Styled(body, styles)`,
ou o body directo quando a colecção fica vazia.

**`parse_text_font`** aceita três formas: string única; array de strings não vazio;
dict em forma **nomeada** (`family` obrigatório + `variant`/`weight`/`style` opcionais)
ou em forma **legada** (chaves = nomes de família, valores = variante ou array de
variantes). Lista vazia → `font fallback list must not be empty`.

**`parse_text_edge`** aceita `Length` ou uma métrica nomeada — `top-edge`:
`ascender`/`cap-height`/`x-height`/`baseline`/`bounds`; `bottom-edge`:
`baseline`/`descender`/`bounds`. Fora da lista → `edge_cast_error`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through: recebidos pela assinatura
  uniforme das nativas e **nunca usados** — nenhum efeito, nenhuma leitura de contexto.
- As mensagens de erro partilhadas (`type_mismatch`, `expected_length_error`,
  `edge_cast_error`) vêm de `compiler/eval/rules.rs` — ponto único de verdade com a
  set rule; não duplicar texto de erro aqui.
- Os seis validadores são privados ao nó; só `native_text` é reexportado pelo hub.

## Critérios de Verificação

```
#text(red)[a]                    → Styled([Fill(red)], "a")
#text(fill: red)[a]              → Styled([Fill(red)], "a")
#text(fill: none)[a]             → "a" sem Fill
#text("a")                       → Content::text("a")
#text(size: 12pt)[a]             → Styled([Size(12pt)])
#text(weight: "bold")[a]         → Styled([Weight(700)])
#text(weight: 700)[a]            → Styled([Weight(700)])
#text(weight: "xpto")[a]         → Err "unknown font weight name: xpto"
#text(style: "oblique")[a]       → custom "text.style"
#text(style: "xpto")[a]          → Err "unknown font style name: xpto"
#text(font: ("A", "B"))[a]       → Style::Font com 2 famílias
#text(font: ())[a]               → Err "font fallback list must not be empty"
#text(font: (family: "A"))[a]    → família nomeada
#text(top-edge: "cap-height")[a] → custom "text.top-edge"
#text(top-edge: "descender")[a]  → Err (métrica só válida em bottom-edge)
#text(dir: rtl)[a]               → custom "text.dir"
#text(dir: ttb)[a]               → Err "text direction must be horizontal"
#text(bold: true)[a]             → Err "unexpected argument: bold"
#text(xpto: 1)[a]                → Err "unexpected argument: xpto"
#text[a][b]                      → Err (2 posicionais sem fill)
#text()                          → Err "text() requer body como argumento"
```
