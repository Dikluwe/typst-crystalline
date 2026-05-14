# Prompt L0 — `rules/stdlib` — Biblioteca Padrão Intrínseca
Hash do Código: 5cfe11d2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib.rs`
**Passo de origem**: Passo 17 (funções nativas base), Passo 25 (rgb/luma),
                     Passo 27 (str/int/float/calc)
**ADRs relevantes**: ADR-0016 (spread adiado), ADR-0024 (EcoString/Value::Str),
                     ADR-0018 (libm futuro)

---

## Contexto e Objetivo

Enquanto `eval.rs` é o motor que **caminha pela AST**, este módulo contém as
**ferramentas nativas** que Typst expõe no seu escopo global — funções
implementadas directamente em Rust e registadas como `Value::Func` durante a
inicialização do compilador.

**Separação de responsabilidades crítica:**
- `eval.rs`: sabe como avaliar `Expr::LetBinding`, loops, condicionais → produz `Value`
- `stdlib.rs`: sabe o que `abs(-5)` retorna → implementa as funções que `eval` *chama*

A convenção de assinatura de todas as funções nativas é (Passo 71 — DEBT-24):
```rust
fn native_X(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>
```
Funções sem I/O usam `_ctx` (prefixo underscore suprime o warning).
`native_image` usa `ctx.world.read_bytes(path)` para aceder ao ficheiro.
Aceita positional (`args.items`) e named args (`args.named`).
Funções que não aceitam named args chamam `expect_no_named(&args.named)?` no início.

### Imagens (Passo 71)

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_image` | `image(path, width?, height?)` | lê bytes via `ctx.world.read_bytes(path)`, cria `Content::Image` |

---

## Funções Nativas Registadas

### Utilitários gerais

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_type` | `type(v)` | nome do tipo como `Value::Str` |
| `native_len` | `len(v)` | `Str` (chars), `Array` (items), `Dict` (entries) |
| `native_range` | `range(n)` ou `range(start, end)` | `Array` de `Int` |
| `native_str` | `str(v)` | conversão para string (`None`→`"none"`, `Bool`→`"true"/"false"`) |
| `native_int` | `int(v)` | `Int`, `Bool`, `Str(decimal)` → `Int`; `Float` → **Err** (ADR Typst) |
| `native_float` | `float(v)` | `Float`, `Int`, `Str` → `Float` |

**Nota** `native_int(Float)` retorna `Err` — esta é a semântica vanilla do
Typst. Para converter float a inteiro, o utilizador deve usar
`int(calc.round(x))`.

### Cores

| Função | Args | Retorno |
|--------|------|---------|
| `native_rgb` | `(r, g, b)` ou `(r, g, b, a)` — Int 0–255 | `Value::Color` |
| `native_luma` | `(l)` — Int 0–255 | `Value::Color(Color::rgb(l, l, l))` |

Fora de 0–255 → `Err`.

### Módulo `calc` — `make_calc_module() -> Value`

Constrói `Value::Dict` com 9 funções (divergência do original que usa
`Value::Module` — Cristalino usa Dict pois não há stdlib Module sem world).
Acesso via `calc.abs`, `calc.pow`, etc. funciona via `eval_field_access` sobre Dict.

| Função | Tipos | Semântica |
|--------|-------|-----------|
| `calc_abs` | `Int` ou `Float` | `Int.saturating_abs()` / `Float.abs()` |
| `calc_pow` | `(Int,Int)` ou `(Num,Num)` | exp negativo em Int → Err; Float usa `powf` |
| `calc_sqrt` | `Int` ou `Float` | argumento negativo → Err |
| `calc_floor` | `Int` ou `Float` | `Int`→`Int` (identidade); `Float`→`Int` |
| `calc_ceil` | `Int` ou `Float` | idem `floor` com `ceil` |
| `calc_round` | `Int` ou `Float` | arredondamento half-up |
| `calc_min` | `≥1 Num` (mistos Int/Float permitidos) | coerção Int→f64 quando misturado |
| `calc_max` | `≥1 Num` | idem `min` |
| `calc_clamp` | `(value, min, max)` | min > max → Err |

**DEBT (ADR-0018)**: `calc_pow` usa `f64::powf` directamente em vez de
`libm::pow`. Quando `libm` for adicionado como dependência do workspace, migrar.

---

## Helpers Internos

| Função | Uso |
|--------|-----|
| `coerce_to_f64(v, ctx)` | `Int`→`f64`, `Float`→`f64`, outros → Err com contexto |
| `guard_float(f)` | NaN → Err "não é um número", Inf → Err "infinito" |
| `format_float(f)` | compacto sem trailing zeros; garante ponto decimal (`"3.0"`) |
| `format_length(l)` | `Length` → `"12pt"`, `"1.5em"`, `"6pt + 1em"` |

---

## Sistema de Tipos — Regras de Promoção

```
Int + Int   → Int       (sem promoção)
Int + Float → Float     (coerce_to_f64)
Float pow Float → guarda NaN/Inf via guard_float
Int/Int divisão → Float (semântica eval.rs, não stdlib)
```

---

## Critérios de Verificação

```
// native_type
native_type([Int(1)])    → Ok(Str("int"))
native_type([Bool(true)])→ Ok(Str("bool"))
native_type([None])      → Ok(Str("none"))
native_type([])          → Err
native_type([Int, Int])  → Err

// native_len
native_len([Str("abc")])          → Ok(Int(3))
native_len([Array([Int(1), Int(2)])])→ Ok(Int(2))
native_len([Int(1)])              → Err

// native_rgb
native_rgb([Int(255), Int(0), Int(128)]) → Ok(Color::rgb(255,0,128))
native_rgb([Int(300), Int(0), Int(0)])   → Err (fora de 0-255)
native_rgb([Int(255), Int(0), Int(0), Int(200)]) → Ok(Color::rgba(255,0,0,200))

// native_luma
native_luma([Int(128)]) → Ok(Color::rgb(128,128,128))
native_luma([Int(256)]) → Err

// native_str
native_str([Int(42)])     → Ok(Str("42"))
native_str([Float(3.14)]) → Ok(Str("3.14"))
native_str([Bool(true)])  → Ok(Str("true"))
native_str([None])        → Ok(Str("none"))
native_str([Str("hi")])   → Ok(Str("hi"))  // identidade

// native_int
native_int([Int(42)])       → Ok(Int(42))
native_int([Bool(true)])    → Ok(Int(1))
native_int([Str("42")])     → Ok(Int(42))
native_int([Str("abc")])    → Err
native_int([Float(3.7)])    → Err  // semântica Typst
native_int([])              → Err

// native_range
native_range([Int(3)])        → Ok(Array([0,1,2]))
native_range([Int(2), Int(5)])→ Ok(Array([2,3,4]))
native_range([Int(3), Int(3)])→ Ok(Array([]))
native_range([Int(-1)])       → Err

// calc_abs
calc_abs([Int(-5)])   → Ok(Int(5))
calc_abs([Float(-3.14)]) → Ok(Float(3.14))
calc_abs([])          → Err

// calc_pow
calc_pow([Int(2), Int(10)])  → Ok(Int(1024))
calc_pow([Int(2), Int(-1)])  → Err (expoente negativo Int)
calc_pow([Float(4.0), Float(0.5)]) → Ok(Float(2.0))

// calc_sqrt
calc_sqrt([Float(4.0)]) → Ok(Float(2.0))
calc_sqrt([Int(4)])     → Ok(Float(2.0))
calc_sqrt([Float(-1.0)])→ Err

// calc_floor / ceil / round
calc_floor([Float(3.7)]) → Ok(Int(3))
calc_ceil([Float(3.2)])  → Ok(Int(4))
calc_round([Float(3.5)]) → Ok(Int(4))
calc_round([Float(3.4)]) → Ok(Int(3))

// calc_min / max
calc_min([Int(3), Int(1), Int(2)]) → Ok(Int(1))
calc_max([Int(3), Int(1), Int(2)]) → Ok(Int(3))
calc_min([]) → Err
calc_max([]) → Err

// calc_clamp
calc_clamp([Int(5), Int(0), Int(10)])   → Ok(Int(5))
calc_clamp([Int(-5), Int(0), Int(10)])  → Ok(Int(0))
calc_clamp([Int(15), Int(0), Int(10)])  → Ok(Int(10))
calc_clamp([Float(5.0), Float(10.0), Float(0.0)]) → Err (min > max)
```

## `state_display(key, [callback])` — Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ)

Render-mediated state display real walk-time. Constroi
`Content::StateDisplay { key, callback }` que walk emite como
`Tag::Start(loc, ElementInfo::new(ElementPayload::StateDisplay
{ key, callback }))` via `extract_payload`. Pós-fixpoint,
`apply_state_displays` (em `rules/introspect/from_tags.rs`)
chama `apply_func(callback, [state.value_at(key, loc)], ctx,
engine)` com Engine+ctx disponíveis e armazena Content
resultado em `intr.state_displays[(key, loc)]`. Layout arm
consome via `Introspector::state_display_value(key, loc)` —
Layouter permanece puro.

```rust
pub fn native_state_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value>
```

**Formas aceites**:
- **1-arg `state_display(key: Str)`** — callback ausente; pós-fixpoint
  renderiza value directo (`Value::Content(c)` passa-through;
  `Value::Str(s)` via `Content::text`; outros tipos
  `Content::Empty`).
- **2-arg `state_display(key: Str, callback: Func)`** — callback
  aplicada ao value; resultado convertido para Content pela
  mesma regra.

**Casos canónicos**:
```
state_display([Str("k")])                                → Ok(Value::Content(Content::StateDisplay { key: "k", callback: None }))
state_display([Str("k"), Func(fn)])                      → Ok(Value::Content(Content::StateDisplay { key: "k", callback: Some(fn) }))
state_display([Int(1)])                                  → Err (key não-string)
state_display([Str("k"), Int(1)])                        → Err (callback não-Func)
state_display([])                                        → Err (0 args)
state_display([Str("k"), Func(f), Int(1)])               → Err (3 args)
```

**Pipeline two-pass real**: identicamente ao `state_final`
(P236 — `state.final_value` retorna `history.last()` que reflete
valor pós-`apply_state_funcs` Funcs cumulativos), `state_display`
beneficia da convergência fixpoint: callback recebe state value
cumulativo correcto no ponto de location-monotónica do walk.
Paridade vanilla `state.display(fn)`.

## `counter_display(key, [callback])` — Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto P240)

Render-mediated counter display real walk-time. **Paralelo
absoluto** a `state_display(key, [callback])` P240. Constroi
`Content::CounterDisplayCallback { key, callback }` que walk
emite como `Tag::Start(loc, ElementInfo::new(ElementPayload::
CounterDisplay { key, callback }))` via `extract_payload`.
Pós-fixpoint, `apply_counter_displays` (em
`rules/introspect/from_tags.rs`) converte
`intr.counters.value_at(key, loc)` para
`Value::Array(Vec<Value::Int>)` e chama
`apply_func(callback, [array], ctx, engine)` com Engine+ctx
disponíveis. Resultado Content armazenado em
`intr.counter_displays[(key, loc)]`. Layout arm consome via
`Introspector::counter_display_value(key, loc)` — Layouter
permanece puro.

```rust
pub fn native_counter_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value>
```

**Formas aceites**:
- **1-arg `counter_display(key: Str)`** — callback ausente;
  pós-fixpoint formata snapshot default "1.2.3" via join "."
  (paridade `formatted_counter_at` P177). Counter inexistente:
  `Content::Empty`.
- **2-arg `counter_display(key: Str, callback: Func)`** —
  callback recebe `Value::Array(Vec<Value::Int>)` (counter
  state actual); resultado convertido para Content paridade
  `state_display` P240.

**Casos canónicos**:
```
counter_display([Str("heading")])                          → Ok(Value::Content(Content::CounterDisplayCallback { key: "heading", callback: None }))
counter_display([Str("figure"), Func(fn)])                 → Ok(Value::Content(Content::CounterDisplayCallback { key: "figure", callback: Some(fn) }))
counter_display([Int(1)])                                  → Err (key não-string)
counter_display([Str("k"), Int(1)])                        → Err (callback não-Func)
counter_display([])                                        → Err (0 args)
counter_display([Str("k"), Func(f), Int(1)])               → Err (3 args)
```

**Pipeline two-pass real**: identicamente a `state_display` P240,
`counter_display` beneficia da convergência fixpoint via
`apply_counter_displays` pós-walk com Engine+ctx. Callback
recebe snapshot counter state cumulativo location-monotónica.
Paridade vanilla `counter("heading").display(fn)`.

**Distinto de `Content::CounterDisplay { kind }` legacy
single-pass** — variant nova paralela; legacy preservada para
display simples sem callback no Layouter directo.
