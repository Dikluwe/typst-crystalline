# Prompt L0 — `rules/stdlib` — Biblioteca Padrão Intrínseca
Hash do Código: 6e6c49e4

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
