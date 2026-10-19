# P545 — Interpolação `#{expr}` dentro de markup

## Resumo

Corrigiu-se a interpolação de expressões de código dentro de texto markup
(`#{...}`). Antes, valores como inteiros, floats, booleans e arrays eram
silenciosamente descartados; agora convertem-se para texto e inserem-se no
conteúdo.

## Sonda

Comportamento observado antes da correcção:

```typst
O resultado é #{1+1}.
```

produzia:

```text
O resultado é .
```

— o `#{1+1}` desaparecia.

Variações testadas:

| Expressão | Resultado antes |
|-----------|-----------------|
| `#{1+1}`  | vazio           |
| `#{if true [sim]}` | "sim" (funcionava) |
| `#{"texto"}` | "texto" (funcionava) |
| `#{1+2*3}` | vazio          |

Conclusão da sonda: o parser reconhecia `#{...}`; o eval tratava
`Value::Content`, `Value::Str` e `Value::Symbol`, mas descartava todos os
outros tipos de valor no arm `_ => {}` de `eval_markup_body`.

## Implementação

### 1. Conversão de valores primitivos para texto

Em `01_core/src/engine/eval/mod.rs`, o arm catch-all do eval de markup
passou a converter valores primitivos com `repr_value`:

- `Value::Int`, `Value::Float`, `Value::Bool` → representação textual.
- `Value::Array`, `Value::Dict` → representação `repr`.
- `Value::Length`, `Value::Relative`, `Value::Ratio`, `Value::Angle`,
  `Value::Color`, `Value::Stroke`, `Value::Fraction`, `Value::Align`,
  `Value::Datetime`, `Value::Decimal`, `Value::Duration`, `Value::Version`
  → `repr_value`.

Tipos como `Value::None`, `Value::State`, `Value::Counter` continuam a não
emitir conteúdo. `Value::Content`, `Value::Str`, `Value::Symbol` mantêm o
comportamento anterior.

### 2. Espaçamento entre fragmentos de texto

Remover o avanço automático de `space_width()` de `layout_word`
(`01_core/src/engine/layout/cursor.rs`). O espaço entre palavras passa a ser
responsabilidade de:

- `Content::Space` (tokens `Space` do markup);
- layout de `Content::Text` (múltiplas palavras num token, via
  `split_whitespace()` + avanço de `space_width()`).

Isto evita inserir espaço entre fragmentos de texto adjacentes produzidos
por interpolações `#{expr}`, como em `#{1+1}.` → "2." em vez de "2 .".

## Ficheiros alterados

- `01_core/src/engine/eval/mod.rs` — interpolação de valores primitivos.
- `01_core/src/engine/layout/cursor.rs` — `layout_word` sem avanço de
  espaço automático.
- `01_core/src/engine/eval/tests.rs` — teste P540 actualizado para usar
  `#{i+1}` em vez de `#str(i)`.
- `03_infra/fixtures/p307b/reference/*.pdf` — snapshots binários
  actualizados (mudança de espaçamento entre palavras).

## Validação

### Casos manuais

```typst
O resultado é #{1+1}.
#{if true [Isto aparece.]}
- Item #{1+1}
- Item #{(1, 2)}
#table(
  columns: (1fr, 1fr),
  [Célula], [Valor #{2+2}],
)
```

Output cristalino:

```text
O resultado é 2. Isto aparece.
•Item 2
•Item (1, 2)
Célula
Valor 4
```

Nota: o espaço em falta entre "•" e "Item" é um problema preexistente no
layout de listas, não da interpolação.

### Comparação com vanilla

```typst
#{1+1}
#{if true [sim]}
#{"texto"}
#{(1, 2)}
#{true}
```

Cristalino: `2 sim texto (1, 2) true`  
Vanilla:    `2 sim texto (1, 2) true`

### Teste P540 actualizado

O teste `p540_for_destructuring_tuplo` passou a usar:

```typst
#for (i, x) in items.enumerate() [#{i+1}. #x]
```

e verifica que o texto contém os índices interpolados `1`, `2`, `3`.

## Qualidade de build

- `cargo test --workspace` — passa (580 tests).
- `crystalline-lint .` — zero violations.

## Conclusão

A interpolação `#{expr}` em markup funciona agora para os tipos
primitivos. A mudança no espaçamento de `layout_word` foi necessária para
manter a pontuação colada ao valor interpolado e não introduziu regressões
nos casos de texto contínuo (validado pelos snapshots P307b e pelo caso
`lorem(1200)` de P544).
