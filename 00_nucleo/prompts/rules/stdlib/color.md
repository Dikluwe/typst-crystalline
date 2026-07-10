# Prompt L0 — stdlib módulo `color` (operadores de cor)
Hash do Código: c634083a

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

## Cores predefinidas (P492/P497/P687)

`color.rs` exporta `predefined_color_bindings()` — vector de pares `(EcoString, Value)`
para injeção no scope global de eval (`eval/mod.rs`, `eval/modules.rs`).

**P687 — paridade vanilla 0.15.0 (969087ec).** A lista oficial de cores nomeadas
globais do vanilla é **exactamente** o conjunto de 18 abaixo, definido em
`lab/.../crates/typst-library/src/lib.rs:359-376` com os valores em
`crates/typst-library/src/visualize/color.rs:291-322`. Os bytes sRGB foram
confirmados por `#repr(<cor>)` no vanilla (ex.: `gray`→`luma(66.67%)`≡`#aaaaaa`,
`navy`→`rgb("#001f3f")`, `green`→`rgb("#2ecc40")`). `ostrich` e `pink` (citados no
passo) **não** são globais vanilla (`unknown variable`) e foram excluídos.

| Nome | sRGB | `Color::rgb` |
|------|------|--------------|
| `black` | `#000000` | `Color::rgb(0x00, 0x00, 0x00)` |
| `gray` | `#AAAAAA` | `Color::rgb(0xAA, 0xAA, 0xAA)` |
| `silver` | `#DDDDDD` | `Color::rgb(0xDD, 0xDD, 0xDD)` |
| `white` | `#FFFFFF` | `Color::rgb(0xFF, 0xFF, 0xFF)` |
| `navy` | `#001F3F` | `Color::rgb(0x00, 0x1F, 0x3F)` |
| `blue` | `#0074D9` | `Color::rgb(0x00, 0x74, 0xD9)` |
| `aqua` | `#7FDBFF` | `Color::rgb(0x7F, 0xDB, 0xFF)` |
| `teal` | `#39CCCC` | `Color::rgb(0x39, 0xCC, 0xCC)` |
| `eastern` | `#239DAD` | `Color::rgb(0x23, 0x9D, 0xAD)` |
| `purple` | `#B10DC9` | `Color::rgb(0xB1, 0x0D, 0xC9)` |
| `fuchsia` | `#F012BE` | `Color::rgb(0xF0, 0x12, 0xBE)` |
| `maroon` | `#85144B` | `Color::rgb(0x85, 0x14, 0x4B)` |
| `red` | `#FF4136` | `Color::rgb(0xFF, 0x41, 0x36)` |
| `orange` | `#FF851B` | `Color::rgb(0xFF, 0x85, 0x1B)` |
| `yellow` | `#FFDC00` | `Color::rgb(0xFF, 0xDC, 0x00)` |
| `olive` | `#3D9970` | `Color::rgb(0x3D, 0x99, 0x70)` |
| `green` | `#2ECC40` | `Color::rgb(0x2E, 0xCC, 0x40)` |
| `lime` | `#01FF70` | `Color::rgb(0x01, 0xFF, 0x70)` |

**Nota (língua vs mecânica — ADR-0107):** no vanilla, `black/gray/silver/white` são
cores `Luma` e imprimem como `luma(..%)`; no cristalino são `Color::rgb(..)` (Srgb) e
imprimem via `Debug`. A **cor observável** (bytes sRGB → PDF) é idêntica; a diferença
de espaço de cor e de formatação de `repr` é **mecânica** e diverge de propósito
(P329). A aceitação mede-se pelos bytes sRGB, nunca pela string de `repr`.

**Extras não-vanilla (sem regressão):** `cyan` (`rgb(0x00,0xB3,0xB3)`),
`magenta` (`rgb(0xE5,0x00,0xE5)`) e `none` (`Value::None`) já existiam antes de P687 e
são mantidos para não regredir documentos/testes que os usam. Não fazem parte do
conjunto oficial vanilla de 18 (falso-aceite pré-existente → débito, fora de escopo).
O parser de cores por *string* (`fill: "gray"` → `rgb(128,128,128)`, em `shapes.rs`) é
uma via **separada** (nomes CSS) e **não** é alterado por este passo.

A função `text(...)` é registada separadamente no scope global (P492) para permitir
`#show regex("\\d+"): it => text(red, it)`.

## Scope-out

- `color.mix` com N > 2 cores — scope-out (cristalino aceita 2 + weight).
- `color.mix` com `space:` arg — scope-out.
- `color.saturate/desaturate` com `space:` arg — scope-out.
