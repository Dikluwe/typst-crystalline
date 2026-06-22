# Prompt L0 — rules/style/font-dict
Hash do Código: 1d8c1a1a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/rules.rs` (arm `"font"` de `#set text`)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem),
ADR-0054 graded scope-out (variant selection), DEBT-52 (fecho)

## Contexto

DEBT-52 bloqueava a forma dict de `#set text(font: (...))` porque
`Value::Regex` ainda não era tipo L1. P402 activou `Regex`; P407
consome essa dependência, fechando DEBT-52.

No vanilla:
```typ
#set text(font: ("Name": ("Regular", "Bold")))
#set text(font: (regex("Name.*"): ("Regular", "Bold")))
```

O dict mapeia **font family name** (string literal ou regex) para
**array de variant names** (strings). O layout resolve cada entrada
contra `FontBook`; a primeira que matcha vence.

## Interface pública

O arm `"font"` em `eval_set_rule` aceita:
- `Value::Str(s)` → lista com uma família literal, variants vazio.
- `Value::Array(arr)` de `Value::Str` → lista com N famílias literais,
  variants vazio.
- `Value::Dict(dict)` → lista com N famílias:
  - Key `Value::Str(s)` → `FontNamePattern::Literal(s)`.
  - Key `Value::Regex(re)` → `FontNamePattern::Regex(re)`.
  - Value `Value::Str(s)` → variants `[s]`.
  - Value `Value::Array(arr)` de `Value::Str` → variants array.

## Representação na chain custom

`#set text(font: ...)` persiste no canal `"text.font"` da `StyleChain`
como `Value::Array`. Cada item pode ser:
- `Value::Str(name)` — família literal, variants vazio (compatível
  retroativo com P292/P373).
- `Value::Dict` com keys `"name"` e `"variants"`:
  - `"name"` → `Value::Str` ou `Value::Regex`.
  - `"variants"` → `Value::Array` de `Value::Str`.

Zero tipo novo em `Value`: reusa `Array`, `Dict`, `Str`, `Regex`.

## Erros (erro hard com span)

- Dict key não-Str/não-Regex → "font dict key must be string or regex".
- Dict value não-Str/não-Array → "font dict value must be string or array of strings".
- Array value com item não-Str → "font variants must be strings".
- Array de fontes vazio → "font array must not be empty".

## Scope-out

- Selecção variant-aware (peso/estilo a partir dos nomes de variant)
  permanece scope-out (ADR-0054bis condicional; requer `FontVariant`
  tipo + shaping XL).
- Optimização O(1) para literais em dict com regex — scope-out
  performance futuro.
- Fallback chain para múltiplos regex matches — primeiro match wins.

## Critérios de Verificação

```
Dado #set text(font: ("Name": ("Regular", "Bold")))
Quando eval
Então "text.font" na chain é Array[Dict{name: Str("Name"), variants: Array[Str("Regular"), Str("Bold")]}]

Dado #set text(font: (regex("Name.*"): ("Regular")))
Quando eval
Então "text.font" é Array[Dict{name: Regex("Name.*"), variants: Array[Str("Regular")]}]

Dado #set text(font: ("Name": "Regular"))
Quando eval
Então variants = ["Regular"]

Dado #set text(font: (123: ("Regular")))
Quando eval
Então erro hard
```
