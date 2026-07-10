# Prompt L0 — rules/style/font-dict
Hash do Código: b307d85a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/rules.rs` (arm `"font"` de `#set text`)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem),
ADR-0054bis condicional (variant selection), DEBT-52 (fecho)

## Contexto

DEBT-52 bloqueava a forma dict de `#set text(font: (...))` porque
`Value::Regex` ainda não era tipo L1. P402 activou `Regex`; P407
consumiu essa dependência, fechando DEBT-52 com o formato legado de dict
(cujas chaves são nomes de família). P414 acrescenta a forma named
fields do vanilla.

No vanilla há duas formas de dict aceites:

```typ
// Forma legado / chave-valor (P407)
#set text(font: ("Name": ("Regular", "Bold")))
#set text(font: (regex("Name.*"): ("Regular", "Bold")))

// Forma named fields (P414)
#set text(font: (family: "Linux Libertine", variant: "bold", weight: 700, style: "italic", fallback: true))
```

## Interface pública

O arm `"font"` em `eval_set_rule` aceita:
- `Expr::Str(node)` → lista com uma família literal, variants vazio.
- `Expr::Array(arr_node)` de `Expr::Str` → lista com N famílias literais,
  variants vazio.
- `Expr::Dict(dict_node)`:
  - Se existir chave identificador `family` → **named fields** (P414).
  - Caso contrário → **formato legado** (P407):
    - Key `Expr::Str` → `FontNamePattern::Literal`.
    - Key `Expr::FuncCall(regex(...))` → `FontNamePattern::Regex`.
    - Value `Value::Str` → variants `[s]`.
    - Value `Value::Array` de `Value::Str` → variants array.

### Named fields (P414 + P660)

Campos suportados:
- `family` (Str|Regex) — obrigatório.
- `variant` (Str | Dict) — opcional.
  - Str: nome de variante canónica (ex.: `"bold"`, `"italic"`).
  - Dict: eixos OpenType explícitos `tag → valor` (ex.: `(wdth: 62.5)`).
    Keys devem ser strings de 4 caracteres; valores `Int` ou `Float`.
- `weight` (Int|Str) — opcional.
- `style` (Str) — opcional.
- `fallback` (Bool, default `true`) — opcional.

Campos rejeitados (erro hard): `stretch` e quaisquer outros identificadores.

## Representação na chain custom

`#set text(font: ...)` persiste no canal `"text.font"` da `StyleChain`
como `Value::Array`. Cada item é um `Value::Dict` com:
- `"name"` → `Value::Str` ou `Value::Regex`.
- `"variants"` → `Value::Array` de `Value::Str`.
- `"variant"` → `Value::Str` (named fields, opcional).
- `"weight"` → `Value::Str` (named fields, opcional).
- `"style"` → `Value::Str` (named fields, opcional).
- `"axes"` → `Value::Dict` de `Value::Str(tag) → Value::Float(valor)`
  (P660, quando `variant` for dict de eixos).

Zero tipo novo em `Value`: reusa `Array`, `Dict`, `Str`, `Regex`, `Int`, `Bool`, `Float`.

## Erros (erro hard com span)

### Formato legado (P407)
- Dict key não-Str/não-Regex → "font dict key must be string or regex".
- Dict value não-Str/não-Array → "font dict value must be string or array of strings".
- Array value com item não-Str → "font variants must be strings".
- Dict vazio → "font dict must not be empty".

### Named fields (P414)
- Campo desconhecido → "unknown font dict field: <name>".
- `family` ausente → "font dict missing field 'family'".
- Tipo errado → "font dict field '<name>' expects <expected>, got <type>".

## Scope-out

- Selecção variant-aware (peso/estilo a partir dos nomes de variant / campos
  named fields) permanece scope-out (ADR-0054bis condicional; requer
  `FontVariant` tipo + shaping XL).
- Optimização O(1) para literais em dict com regex — scope-out performance
  futuro.
- Fallback chain para múltiplos regex matches — primeiro match wins.
- `stretch` no dict named fields — scope-out futuro.
- Dict spread (`..dict`) — scope-out P407, mantido.

## Critérios de Verificação

```
Dado #set text(font: ("Name": ("Regular", "Bold")))
Quando eval
Então "text.font" na chain é Array[Dict{name: Str("Name"), variants: Array[Str("Regular"), Str("Bold")]}]

Dado #set text(font: (regex("Name.*"): ("Regular")))
Quando eval
Então "text.font" é Array[Dict{name: Regex("Name.*"), variants: Array[Str("Regular")]}]

Dado #set text(font: (family: "Arial"))
Quando eval
Então "text.font" é Array[Dict{name: Str("Arial"), variants: [], variant: None, weight: None, style: None}]

Dado #set text(font: (family: regex("Ar.*"), variant: "bold"))
Quando eval
Então name é Regex("Ar.*"), variant é "bold"

Dado #set text(font: (family: "Arial", weight: 700, style: "italic", fallback: false))
Quando eval
Então weight="700", style="italic"

Dado #set text(font: (family: "Arial", stretch: "expanded"))
Quando eval
Então erro hard
```
