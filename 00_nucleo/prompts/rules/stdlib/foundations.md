# Prompt L0 — `stdlib/foundations` — utilitários, cores, conversões e introspeção
Hash do Código: 9f4165fc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/foundations.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com
reforços pontuais P13–P25, P99–P102, P171–P179, P208–P210, P236–P241,
P421 (`repr`) e P465 (`repr()` completo). P685: `native_type` devolve
`Value::Type`; `int`/`float`/`str`/`type` são valores-tipo chamáveis no scope.
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded),
ADR-0081 (state/counter display two-pass), ADR-0083 (color spaces),
ADR-0107 (paridade linguagem).
**Convenções partilhadas**: ver `00_nucleo/prompts/rules/stdlib/_comum.md`.

---

## 1. Visão geral

`foundations.rs` implementa funções nativas fundamentais do Typst que não
pertencem a domínios específicos (layout, formas, transforms, calc, etc.).
Inclui:

- **Utilitários gerais**: `type`, `repr`, `len`.
- **Conversões de tipo**: `range`, `str`, `int`, `float`.
- **Construtores de cor**: `rgb`, `luma`, `oklab`, `oklch`, `linear_rgb`,
  `cmyk`, `hsl`, `hsv`.
- **Metadados**: `metadata`.
- **Estado runtime**: `state` (objeto com métodos — ver
  `rules/stdlib/state.md`), `state_update`, `state_update_with`,
  `state_display`, `state_final`, `state_at`.
- **Contadores**: `counter` (objeto com métodos — ver
  `rules/stdlib/counter.md`), `counter_display`, `counter_at`,
  `counter_final`, `counter_step`.
- **Query / localização**: `query`, `locate`, `here`.

A assinatura padrão das funções nativas é:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

A maioria das funções deste módulo não aceita argumentos nomeados e chama
`expect_no_named(&args.named)?` no início.

---

## 2. Utilitários gerais

### `native_type` — `type(v)`

**Assinatura**: `type(v: any) -> type`  (P685: devolve `Value::Type`, não `str`)

**Argumentos**:
- `v`: um valor posicional obrigatório.

**Semântica (P685)**: Devolve o **valor-tipo** do argumento — `Value::Type(v.type_of())`.
Antes de P685 devolvia `Value::Str(v.type_name())`; a mudança é necessária para
`type(x) == length` funcionar por comparação directa de valores de tipo
(paridade vanilla, medida na sonda P685). O nome textual do tipo continua
disponível via `repr(type(x))` (ex.: `repr(type(1)) == "int"`).

**Paridade vanilla**: `#type(1)` → valor `int`; `#(type(1) == int)` → `true`;
`#(type(int) == type)` → `true`; `#(type(rgb) == function)` → `true`.

**Testes canônicos**:
```
type(1)             -> Value::Type(Type::Int)
type("abc")         -> Value::Type(Type::Str)
type(none)          -> Value::Type(Type::None)
type(true)          -> Value::Type(Type::Bool)
type(1pt)           -> Value::Type(Type::Length)
type(50% + 1pt)     -> Value::Type(Type::Length)   // relative length == length
type(rgb(0,0,0))    -> Value::Type(Type::Color)
type(int)           -> Value::Type(Type::Type)     // tipo de um tipo é `type`
type()              -> Err "type() requer 1 argumento"
type(1, 2)          -> Err "type() requer 1 argumento"
repr(type(1))       -> "int"
```

---

### Nota P685 — `int`, `float`, `str`, `type` como valores-tipo chamáveis

Os construtores `int`, `float`, `str` (§3) e a função `type` (§2) estão
registados no scope global como `Value::Type(Type::Int | Float | Str | Type)`,
não como `Value::Func`. A chamada (`int("5")`, `str(5)`, `float("3.5")`,
`type(1)`) é despachada em `eval_func_call` (`rules/eval/closures.rs`) que, ao
ver `Value::Type` chamável, invoca o construtor nativo correspondente. A
semântica de `native_int`/`native_float`/`native_str`/`native_type` (abaixo) é
inalterada — só muda o valor no scope (de função para tipo chamável).

---

### `native_repr` — `repr(v)`

**Assinatura**: `repr(v: any) -> str`

**Argumentos**:
- `v`: um valor posicional obrigatório.

**Semântica**: Devolve uma representação textual reconhecível do valor
(sem round-trip garantido). Implementado em `rules/eval/repr.rs`.

**Tabela de tipos e formatos (P465)**:

| Tipo | Saída exemplo |
|------|---------------|
| `none` | `"none"` |
| `auto` | `"auto"` |
| `bool` | `"true"` / `"false"` |
| `int` | `"1"` |
| `float` | `"1.0"`, `"1.5"` |
| `str` | `"\"hello\""` |
| `array` | `"(1, 2)"` |
| `dict` | `"(a: 1)"` |
| `content` | `"heading(level: 1)[\"Title\"]"` |
| `function` | `"#repr"` ou `"#function(...)"` |
| `module` | `"module(mylib)"` |
| `datetime` | `"datetime(2026-06-25)"` ou `"datetime(2026-06-25T14:30:00)"` |
| `duration` | `"duration(1s)"` |
| `version` | `"version(0, 11, 0)"` |
| `bytes` | `"bytes(10)"` |
| `regex` | `"regex(\"a+\")"` |
| `gradient` | `"gradient(...)"` (placeholder) |
| `tiling` | `"tiling(...)"` (placeholder) |
| `location` | `"location(...)"` (placeholder) |

**Scope-outs**:
- Round-trip perfeito (`eval(repr(x)) == x`).
- Representação completa de closures, módulos e valores dinâmicos opacos.
- `Symbol` e `Type` como valores de primeira classe (ainda não existem em L1).
- `Color` avançado (CMYK/Oklab) — usa `Debug` existente; refinamento é Trilha 4.

**Testes canônicos**:
```
repr(1)             -> "1"
repr(1.5)           -> "1.5"
repr(3.0)           -> "3.0"
repr("abc")         -> "\"abc\""
repr(none)          -> "none"
repr(auto)          -> "auto"
repr(version(0,11,0)) -> "version(0, 11, 0)"
repr(bytes(10))     -> "bytes(10)"
repr(regex("a+"))   -> "regex(\"a+\")"
repr((1, "a", none)) -> "(1, \"a\", none)"
repr()              -> Err "repr() requer 1 argumento"
```

---

### `native_len` — `len(v)`

**Assinatura**: `len(v: str | array | dict) -> int`

**Argumentos**:
- `v`: um valor posicional obrigatório.

**Semântica**:
- `Str` → número de caracteres Unicode (`chars().count()`).
- `Array` → número de elementos.
- `Dict` → número de entradas.

**Paridade vanilla**: Equivalente a `#len("abc")` → `3`.

**Testes canônicos**:
```
len("abc")           -> 3
len("é")             -> 1
len((1, 2, 3))       -> 3
len((:))             -> 0
len(1)               -> Err "len() não suporta int"
len()                -> Err "len() requer 1 argumento"
```

---

## 3. Conversões de tipo

### `native_range` — `range(n)` / `range(start, end)`

**Assinatura**: `range(n: int) -> array` / `range(start: int, end: int) -> array`

**Argumentos**:
- 1 arg: `n` — limite superior exclusivo; gera `[0, 1, ..., n-1]`.
- 2 args: `start`, `end` — gera `[start, start+1, ..., end-1]`.

**Semântica**: Devolve um `Value::Array` de `Value::Int`. `start > end` →
array vazio. `n < 0` → erro.

**Testes canônicos**:
```
range(3)        -> (0, 1, 2)
range(0)        -> ()
range(2, 5)     -> (2, 3, 4)
range(3, 3)     -> ()
range(5, 2)     -> ()
range(-1)       -> Err "range() requer argumento não-negativo"
range(1.5)      -> Err "range() requer 1 ou 2 Int"
```

---

### `native_str` — `str(v)` / `str(int, base:)`

**Assinatura**: `str(v: any) -> str` / `str(int: Int, base: Int) -> str`

**Argumentos**:
- `v`: um valor posicional obrigatório.
- `base`: named opcional (`Int`), apenas quando `v` é `Int`. Default `10`.
  Deve estar em `[2, 36]`.

**Semântica**: Converte o valor para string. Suporta:
- `None` → `"none"`
- `Bool` → `"true"` / `"false"`
- `Int` → decimal (ou representação na `base` indicada)
- `Float` → compacto com ponto decimal (`format_float`)
- `Str` → pass-through
- `Auto` → `"auto"`
- `Length` → `"12pt"`, `"1.5em"`, `"6pt + 1em"`
- `Ratio` → `"50%"`
- `Angle` → `"45deg"`
- `Color` → erro (não suportado)

**Scope-out**: Conversão de tipos complexos (`Func`, `Module`, `Gradient`,
`Content`, etc.).

**Testes canônicos**:
```
str(42)           -> "42"
str(3.0)          -> "3.0"
str(true)         -> "true"
str(none)         -> "none"
str("hi")         -> "hi"
str(auto)         -> "auto"
str(12pt + 1em)   -> "12pt + 1em"
str(50%)          -> "50%"
str(45deg)        -> "45deg"
str(255, base: 16) -> "ff"
str(42, base: 2)  -> "101010"
str(-255, base: 16) -> "-ff"
str(255, base: 37) -> Err "base deve estar entre 2 e 36"
str(3.14, base: 16) -> Err "base só se aplica a Int"
str(red)          -> Err "str() não suporta color"
```

---

### `native_int` — `int(v)`

**Assinatura**: `int(v: int | bool | str) -> int`

**Argumentos**:
- `v`: um valor posicional obrigatório.

**Semântica**: Converte para inteiro. Aceita:
- `Int` → identidade.
- `Bool` → `1` / `0`.
- `Str` → parse decimal (`parse::<i64>()`).
- `Float` → **erro** (semântica vanilla).

**Testes canônicos**:
```
int(42)           -> 42
int(true)         -> 1
int("-7")         -> -7
int("abc")        -> Err "int() não consegue parsear"
int(3.7)          -> Err "int() não converte float"
int()             -> Err "int() requer 1 argumento"
```

---

### `native_float` — `float(v)`

**Assinatura**: `float(v: float | int | str) -> float`

**Argumentos**:
- `v`: um valor posicional obrigatório.

**Semântica**: Converte para float. Aceita `Float`, `Int` (coerção) e `Str`
(parse `f64`).

**Testes canônicos**:
```
float(3.14)       -> 3.14
float(7)          -> 7.0
float("2.5")      -> 2.5
float("abc")      -> Err "float() não consegue parsear"
float()           -> Err "float() requer 1 argumento"
```

---

## 4. Construtores de cor

### `native_rgb` — `rgb(r, g, b)` / `rgb(r, g, b, a)`

**Assinatura**: `rgb(r: int, g: int, b: int, a: int?) -> color`

**Argumentos**: componentes inteiros 0–255. Alpha opcional.

**Semântica**: Constrói `Value::Color(Color::rgb(...))` ou
`Color::rgba(...)`.

**Testes canônicos**:
```
rgb(255, 0, 128)          -> Color::rgb(255,0,128)
rgb(255, 0, 0, 200)       -> Color::rgba(255,0,0,200)
rgb(300, 0, 0)            -> Err "componente fora de 0–255"
rgb(0, 0)                 -> Err "rgb() requer 3 ou 4 Int"
```

---

### `native_luma` — `luma(l)`

**Assinatura**: `luma(l: int) -> color`

**Argumentos**: `l` em `[0, 255]`.

**Semântica**: Constrói `Value::Color(Color::luma(l / 255.0))`.

**Testes canônicos**:
```
luma(128)         -> Color::Luma { l: 0.5, a: 1.0 }
luma(0)           -> preto
luma(255)         -> branco
luma(256)         -> Err "componente fora de 0–255"
```

---

### `native_oklab` — `oklab(l, a, b)` / `oklab(l, a, b, alpha)`

**Assinatura**: `oklab(l: float|int, a: float|int, b: float|int, alpha: float|int?) -> color`

**Semântica**: Constrói `Color::Oklab`. Componentes convertidos para `f32`.

**Testes canônicos**:
```
oklab(0.5, 0.2, -0.1)          -> Color::Oklab
oklab(0.5, 0.2, -0.1, 0.8)     -> Color::Oklab com alpha 0.8
oklab(1, 0, 0)                 -> Color::Oklab (Int promovido)
oklab(0.5, 0.2)                -> Err "oklab() requer 3 ou 4"
```

---

### `native_oklch` — `oklch(l, c, h)` / `oklch(l, c, h, alpha)`

**Assinatura**: `oklch(l: float|int, c: float|int, h: float|int, alpha: float|int?) -> color`

**Semântica**: Constrói `Color::Oklch`. `h` em graus.

**Testes canônicos**:
```
oklch(0.5, 0.2, 180)           -> Color::Oklch
oklch(0.5, 0.2, 180, 0.9)      -> Color::Oklch com alpha 0.9
oklch(0.5, 0.2)                -> Err "oklch() requer 3 ou 4"
```

---

### `native_linear_rgb` — `linear_rgb(r, g, b)` / `linear_rgb(r, g, b, a)`

**Assinatura**: `linear_rgb(r: float|int, g: float|int, b: float|int, a: float|int?) -> color`

**Semântica**: Constrói `Color::LinearRgb`. Componentes em `[0.0, 1.0]`.

**Testes canônicos**:
```
linear_rgb(1.0, 0.0, 0.5)          -> Color::LinearRgb
linear_rgb(1, 0, 0, 0.5)           -> Color::LinearRgb com alpha 0.5
linear_rgb(1.0, 0.0)               -> Err "linear_rgb() requer 3 ou 4"
```

---

### `native_cmyk` — `cmyk(c, m, y, k)`

**Assinatura**: `cmyk(c: float|int, m: float|int, y: float|int, k: float|int) -> color`

**Semântica**: Constrói `Color::Cmyk`. Componentes em `[0.0, 1.0]`.

**Scope-out**: PDF nativo `/DeviceCMYK` — o exporter converte para sRGB via
`to_srgb()` (ADR-0083).

**Testes canônicos**:
```
cmyk(0.0, 1.0, 0.0, 0.0)       -> Color::Cmyk (magenta)
cmyk(0, 0, 0, 1)               -> preto
cmyk(0.0, 1.0, 0.0)            -> Err "cmyk() requer 4"
```

---

### `native_hsl` — `hsl(h, s, l)` / `hsl(h, s, l, alpha)`

**Assinatura**: `hsl(h: float|int, s: float|int, l: float|int, alpha: float|int?) -> color`

**Semântica**: Constrói `Color::Hsl`. `h` em graus; `s`, `l` em `[0.0, 1.0]`.

**Testes canônicos**:
```
hsl(0, 1.0, 0.5)               -> vermelho
hsl(120, 1.0, 0.5, 0.8)        -> verde com alpha 0.8
hsl(0, 1.0)                    -> Err "hsl() requer 3 ou 4"
```

---

### `native_hsv` — `hsv(h, s, v)` / `hsv(h, s, v, alpha)`

**Assinatura**: `hsv(h: float|int, s: float|int, v: float|int, alpha: float|int?) -> color`

**Semântica**: Constrói `Color::Hsv`. `h` em graus; `s`, `v` em `[0.0, 1.0]`.

**Testes canônicos**:
```
hsv(0, 1.0, 1.0)               -> vermelho
hsv(240, 1.0, 1.0, 0.9)        -> azul com alpha 0.9
hsv(0, 1.0)                    -> Err "hsv() requer 3 ou 4"
```

---

## 5. Metadados

### `native_metadata` — `metadata(value)`

**Assinatura**: `metadata(value: any) -> content`

**Argumentos**:
- `value`: valor posicional obrigatório.

**Semântica**: Produz `Content::metadata(Box<Value>)`, um content zero-size
em layout que pode ser consultado via `Introspector::query_metadata`.

**Paridade vanilla**: Equivalente a `#metadata(value)`.

**Testes canônicos**:
```
metadata("x") -> Content::Metadata
metadata()    -> Err "metadata() requer 1 argumento"
```

---

## 6. Estado runtime

### `native_state` — `state(key, init)`

**Assinatura**: `state(key: str, init: any) -> state`

**Argumentos**:
- `key`: string identificadora.
- `init`: valor inicial.

**Semântica**: Produz `Value::State { key, init }` (valor de primeira classe
com métodos `.update`, `.get`, `.display`). Quando usado em posição de markup,
o eval converte-o para `Content::State { key, init }` para registo no
`StateRegistry`. Detalhes dos métodos em `rules/stdlib/state.md`.

**Testes canônicos**:
```
state("total", 0) -> Value::State
state(1, 0)       -> Err "state() requer string como primeiro argumento"
state("total")    -> Err "state() requer 2 argumentos"
```

---

### `native_state_update` — `state_update(key, value)`

**Assinatura**: `state_update(key: str, value: any) -> content`

**Semântica**: Produz `Content::StateUpdate` com `StateUpdate::Set(value)`.
Substitui o valor actual do estado.

**Testes canônicos**:
```
state_update("total", 10) -> Content::StateUpdate
state_update(1, 10)       -> Err "string como primeiro argumento"
```

---

### `native_state_update_with` — `state_update_with(key, fn)`

**Assinatura**: `state_update_with(key: str, fn: function) -> content`

**Semântica**: Produz `Content::StateUpdate` com `StateUpdate::Func(fn)`.
Após fixpoint, a callback é aplicada ao valor corrente do estado para
produzir o novo valor.

**Testes canônicos**:
```
state_update_with("total", x => x + 1) -> Content::StateUpdate
state_update_with("total", 1)          -> Err "função como segundo argumento"
```

---

### `native_state_display` — `state_display(key)` / `state_display(key, callback)`

**Assinatura**: `state_display(key: str, callback: function?) -> content`

**Semântica**: Produz `Content::StateDisplay`. Durante o layout walk emite
uma tag; pós-fixpoint, o valor do estado na localização é resolvido e, se
houver callback, aplicado. O resultado é armazenado no introspector para
consumo pelo layout.

**Testes canônicos**:
```
state_display("total")           -> Content::StateDisplay sem callback
state_display("total", v => v)   -> Content::StateDisplay com callback
state_display(1)                 -> Err "string como primeiro argumento"
```

---

### `native_state_final` — `state_final(key)`

**Assinatura**: `state_final(key: str) -> any`

**Semântica**: Consulta `ctx.introspector.state_final_value(key)`. Devolve o
valor final do estado após todas as updates (incluindo callbacks
pós-fixpoint). Se inexistente → `none`.

**Testes canônicos**:
```
state_final("total") -> valor final ou none
state_final(1)       -> Err "string como argumento"
```

---

### `native_state_at` — `state_at(key, label)`

**Assinatura**: `state_at(key: str, label: str) -> any`

**Semântica**: Resolve a `Location` associada a `label` e consulta o valor do
estado nessa localização. Se label ou estado inexistente → `none`.

**Testes canônicos**:
```
state_at("total", "intro") -> valor do estado em "intro" ou none
state_at("total", 1)       -> Err "string como segundo argumento"
```

---

## 7. Contadores

### `native_counter` — `counter(selector)`

**Assinatura**: `counter(selector: str | selector) -> counter`

**Argumentos**:
- `selector`: string de kind ou `Value::Selector` designando um kind.

**Semântica**: Produz `Value::Counter { key }` (valor de primeira classe com
métodos `.update`, `.step`, `.get`, `.display`, `.at`). Detalhes dos métodos em
`rules/stdlib/counter.md`.

**Testes canônicos**:
```
counter("heading") -> Value::Counter
counter(heading)   -> Value::Counter { key: "heading" }
counter(1)         -> Err "counter() requer string ou selector"
```

---

### `native_counter_display` — `counter_display(key)` / `counter_display(key, callback)`

**Assinatura**: `counter_display(key: str, callback: function?) -> content`

**Semântica**: Produz `Content::CounterDisplayCallback`. Sem callback, o
formato default é `"1.2.3"` (join por `.`). Com callback, recebe
`Value::Array(Vec<Int>)` com o estado do counter.

**Testes canônicos**:
```
counter_display("heading")            -> Content::CounterDisplayCallback
counter_display("figure", arr => ...) -> com callback
counter_display(1)                    -> Err "string como primeiro argumento"
```

---

### `native_counter_at` — `counter_at(key, label)`

**Assinatura**: `counter_at(key: str, label: str) -> str`

**Semântica**: Devolve o valor do counter `key` na `Location` do `label`,
formatado hierarquicamente. Se label/counter inexistente → `""`.

**Testes canônicos**:
```
counter_at("heading", "intro") -> "1.2.3" ou ""
counter_at("heading", 1)       -> Err "string como segundo argumento"
```

---

### `native_counter_final` — `counter_final(key)`

**Assinatura**: `counter_final(key: str) -> str`

**Semântica**: Devolve o valor final do counter `key` no introspector da
iteração de fixpoint anterior. Iteração 0 → `""`.

**Testes canônicos**:
```
counter_final("heading") -> "1.2.3" ou ""
counter_final(1)         -> Err "string como argumento"
```

---

### `native_counter_step` — `counter_step(key)`

**Assinatura**: `counter_step(key: str) -> content`

**Semântica**: Produz `Content::CounterUpdate { key, action: Step }`. Quando
inserido no documento, o Layouter incrementa o counter `key`.

**Testes canônicos**:
```
counter_step("heading") -> Content::CounterUpdate
counter_step(1)         -> Err "string como argumento"
```

---

## 8. Query e localização

### `native_query` — `query(selector)`

**Assinatura**: `query(selector: str | location | selector) -> array<location>`

**Argumentos**:
- `selector`:
  - `"<name>"` → label.
  - `"kind"` → kind de elemento (`heading`, `figure`, `citation`, `metadata`,
    `state`, `state_update`, `outline`).
  - `Value::Location` → localização específica.
  - `Value::Selector` → selector de primeira classe (P417).

**Semântica**: Consulta o `Introspector` e devolve um `Value::Array` de
`Value::Location` com todos os matches, em ordem de aparecimento. Iteração 0
→ array vazio.

**Scope-outs**: Combinadores `and`/`or` de selectors via string; regex
(P209D).

**Testes canônicos**:
```
query("heading")      -> array de Locations
counter_step("heading") -> Content::CounterUpdate
query("<intro>")      -> array com Location do label "intro"
query(here())         -> array com Location atual
query(unknown_kind)   -> Err "kind não reconhecido"
```

---

### `native_locate` — `locate(selector)`

**Assinatura**: `locate(selector: str | location | selector) -> location | none`

**Semântica**: Idêntico a `query` mas devolve apenas o **primeiro** match.
Se não houver matches → `none`.

**Testes canônicos**:
```
locate("heading")     -> Location ou none
locate("<intro>")     -> Location ou none
locate("unknown")     -> none (se kind válido sem matches) ou Err (kind inválido)
```

---

### `native_here` — `here()`

**Assinatura**: `here() -> location`

**Semântica**: Devolve `ctx.current_location` quando populado. Usado dentro
de contextos locatable (show-rules, etc.).

**Scope-out**: Captura automática de `current_location` durante o eval walk
ainda é deferred; caller sintético deve preencher `ctx.current_location`.

**Testes canônicos**:
```
here() (com current_location populado) -> Value::Location
here() (fora de contexto)              -> Err "here() chamado fora de contexto locatable"
here(1)                                -> Err "here() não aceita argumentos"
```

---

## 9. Notas de fecho

Todas as funções nativas de `foundations.rs` estão agora especificadas em L0.
O fecho deste subset **encerra DEBT-57 por completo** — todos os ficheiros
stdlib de L1 (`structural.rs`, `layout.rs`, `calc.rs`, `assert.rs`,
`shapes.rs`, `transforms.rs`, `gradients.rs`, `foundations.rs`) possuem prompt
L0 dedicado.

## 10. Métodos de coleção (P466)

Implementados em `rules/stdlib/collections.rs` e despachados em
`rules/eval/closures.rs::eval_func_call` para a sintaxe de método de
instância (ex.: `(1, 2, 3).first()`).

### `array`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `first()` | `array.first() -> any` | Primeiro elemento ou `none`. |
| `last()` | `array.last() -> any` | Último elemento ou `none`. |
| `rev()` | `array.rev() -> array` | Array invertido. |
| `sum()` | `array.sum() -> int \| float` | Soma numérica; mistura de int/float produz float. |
| `sorted()` | `array.sorted(key: function?) -> array` | Ordena; `key` opcional recebe elemento e devolve chave. |
| `filter()` | `array.filter(pred: function) -> array` | Filtra por predicado. |
| `map()` | `array.map(func: function) -> array` | Mapeia por função. |
| `find()` | `array.find(pred: function) -> any` | Primeiro elemento que satisfaz ou `none`. |
| `any()` | `array.any(pred: function) -> bool` | Algum satisfaz? |
| `all()` | `array.all(pred: function) -> bool` | Todos satisfazem? |
| `zip()` | `array.zip(other: array) -> array` | Pares `(a_i, b_i)`. |
| `enumerate()` | `array.enumerate() -> array` | Pares `(index, value)`. |

### `dict`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `pairs()` | `dict.pairs() -> array` | Array de pares `(key, value)`. |
| `remove()` | `dict.remove(key: str) -> any` | Valor removido ou `none`. **Nota**: no cristalino o dict original não é mutado porque o dispatch recebe o valor por valor. |
| `update()` | `dict.update(other: dict) -> dict` | Mescla com outro dict. |

### `str`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `contains()` | `str.contains(substr: str) -> bool` | Contém substring? |
| `starts-with()` | `str.starts-with(prefix: str) -> bool` | Começa com prefixo? |
| `ends-with()` | `str.ends-with(suffix: str) -> bool` | Termina com sufixo? |
| `find()` | `str.find(substr: str) -> int \| none` | Índice da primeira ocorrência. |
| `replace()` | `str.replace(old: str, new: str) -> str` | Substitui substring literal. |
| `trim()` | `str.trim() -> str` | Remove whitespace dos extremos. |
| `split()` | `str.split(sep: str) -> array` | Divide por separador. |
| `repeat()` | `str.repeat(n: int) -> str` | Repete `n` vezes. |

---

## 11. Notas de fecho

Todas as funções nativas de `foundations.rs` e os métodos de coleção de
`collections.rs` (P466) estão agora especificados em L0.

Scope-outs transversais que permanecem fora deste subset:
- Render PDF nativo para CMYK (`/DeviceCMYK`) e gradientes (`/Sh`).
- Métodos avançados de `str`, `array` e `dict` fora do subset crítico P466.
- Introspeção avançada de módulos (`module` como valor de primeira classe
  completo).
