# Prompt L0 — `compiler/layout/raw_highlight` — Highlighting em Blocos Raw
Hash do Código: 4b333868

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/raw.rs`
**Origem**: P785a (restauração de syntax highlighting em blocos `raw`).
**ADRs Aplicadas**: ADR-0107 (paridade de render em PDF com a linguagem), ADR-0109 (atomização no arquivo da feature).

---

## 1. Contexto e Objetivo

O elemento `raw` (`` `...` `` e ` ```lang ... ``` `) em Typst Vanilla renderiza código-fonte com syntax highlighting colorido via gramáticas `syntect` e o tema padrão `RAW_THEME` (`two-face`). No Crystalline, o layout de `raw` emitia o texto de forma monocromática.

Este módulo L1 implementa a tokenização por linha do texto `raw` e o mapeamento de cores/estilos por token para que os caracteres emitidos contenham as cores RGB correspondentes (`TextElem::fill` / `TextStyle::fill`).

---

## 2. Paleta de Cores e Temas

O tema padrão (`RAW_THEME`) define as seguintes cores base por escopo `syntect`:

| Escopo Syntect | Cor Hex | Uso |
|---|---|---|
| `keyword`, `storage.type` | `#d73948` | Palavras-chave (`fn`, `let`, `if`, `return`) |
| `string` | `#198810` | Literais de texto |
| `entity.name`, `variable.function` | `#4b69c6` | Identificadores de funções/tipos |
| `comment` | `#74747c` | Comentários de linha/bloco |
| `constant.character.escape` | `#1d6c76` | Sequências de escape |
| `entity.other`, `meta.interpolation` | `#8b41b1` | Interpolação / referências |

---

## 3. Comportamento de Layout

1. Para cada linha do texto `raw`:
   - Se uma linguagem `lang` for especificada (ou `txt` por omissão), localizar a gramática correspondente em `RAW_SYNTAXES` (`two-face`).
   - Processar a linha com `syntect::easy::HighlightLines`.
   - Para cada fragmento `(Style, &str)`, instanciar `TextStyle` ajustando `fill: Some(Color::Srgb { .. })` e estilos `bold`/`italic` conforme a cor de primeiro plano do `Style`.
   - Emitir cada palavra/segmento no layouter preservando espaços e quebras de linha em bloco.
2. Em blocos (`block: true`), aplicar margem/recuo e quebra de linha inicial e final.
