# Prompt L0 — stdlib módulo `color` (operadores de cor)
Hash do Código: bcea91be

## Módulo
`01_core/src/rules/stdlib/color.rs`

## Camada
L1 (puro; sem I/O; sem estado global).

## Propósito

Módulo `color` exposto no scope de eval como `Value::Dict` com 4
operadores de cor: `lighten`, `darken`, `mix`, `negate`.

P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out:
4 dos 6 operadores implementados (`saturate`/`desaturate` scope-out futuro P477).

## Função de construção

```rust
pub fn make_color_module() -> Value {
    // Retorna Value::Dict com 4 entradas.
}
```

Registado em `eval/mod.rs` como `scope.define("color", make_color_module())`.

## Funções nativas

### `color.lighten(col, amount)` → Color

- `col`: `Value::Color` — cor base.
- `amount`: `Value::Float` | `Value::Relative` (percentagem, rel-only) — [0.0, 1.0].
- Delega para `Color::lighten(amount)` (P476 L1 método).
- Erros: argumento count ≠ 2, tipo errado, named inesperado.

### `color.darken(col, amount)` → Color

- Análogo a `lighten`; delega para `Color::darken(amount)`.

### `color.mix(col1, col2, weight: 0.5)` → Color

- `col1`, `col2`: `Value::Color` (posicionais).
- `weight`: named opcional; default `0.5`; tipo igual a `amount`.
- Delega para `col1.mix(col2, weight)`.
- Erro: named desconhecido (além de `weight:`).

### `color.negate(col)` → Color

- `col`: `Value::Color`.
- Delega para `Color::negate()`.
- Sem named args.

## Extração de ratio

```rust
fn extract_ratio_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<f32> {
    Value::Float(f)   => f as f32,
    Value::Int(i)     => i as f32 / 100.0,
    Value::Relative(r) if r.abs.is_zero() => r.rel as f32,
    _ => Err(...)
}
```

## Critérios de verificação (P476)

- `color.lighten(Color::srgb_f32(1,0,0,1), 0.2)` → `Ok(Value::Color(_))`.
- `color.darken(Color::srgb_f32(0,0,1,1), 0.2)` → `Ok(Value::Color(_))`.
- `color.negate(Color::srgb_f32(1,0,0,1))` → ciano `(0.0, 1.0, 1.0, 1.0)`.
- `color.mix(red, blue)` sem `weight:` → Ok (default 0.5).
- `color.mix(red, blue, weight: 0.25)` → Ok.
- `make_color_module()` retorna `Value::Dict` com 4 entradas.

## P477 — `saturate` e `desaturate`

Adicionados ao módulo e a `make_color_module()` (6 entradas total):

```rust
dict.insert("saturate",   Value::Func(Func::native("color.saturate",   native_color_saturate)));
dict.insert("desaturate", Value::Func(Func::native("color.desaturate", native_color_desaturate)));
```

`color.saturate(col, amount)` — aumenta chroma Oklch; clamp mínimo 0.0.
`color.desaturate(col, amount)` — diminui chroma Oklch; clamp mínimo 0.0.

ADR-0083 §"Operadores cor": **TOTALMENTE FECHADO** (6/6) pós-P477.

## Scope-out

- `color.mix` com N > 2 cores — scope-out (cristalino aceita 2 + weight).
- `color.mix` com `space:` arg — scope-out.
- `color.saturate/desaturate` com `space:` arg — scope-out.
