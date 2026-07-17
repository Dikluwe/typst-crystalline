# Prompt L0 — stdlib tipo `color` (operadores de cor)
Hash do Código: 2c858da2

## Módulo
`01_core/src/engine/stdlib/color.rs`

## Camada
L1 (puro; sem I/O; sem estado global).

## Propósito

**P736 — estado vigente:** `color` é exposto no scope de eval como
`Value::Type(Type::Color)` (paridade vanilla — medido: `type(color)` →
`type`; `color("#f00")` → erro "type color does not have a constructor").
Os fields do tipo (`color.rgb`, `color.lighten`, …) resolvem-se por field
access em `Value::Type` via `color_type_field(field) -> Option<Value>`
(padrão P685, mesmo mecanismo de `int.min`/`str.from-unicode`).

Histórico: até P736, `color` era `Value::Dict` (P476, módulo de 4
operadores; P477 aumentou para 6).

Fields do tipo (20): constructors `rgb`, `linear-rgb`, `luma`, `cmyk`,
`hsl`, `hsv`, `oklab`, `oklch` (as mesmas funções nativas registadas
globalmente) + operadores `lighten`, `darken`, `mix`, `negate`,
`saturate`, `desaturate`, `rotate`, `components`, `space` (P742) +
`to-hex`, `transparentize`, `opacify` (P744).

Medições vanilla que fundamentam (P736):
- `type(color)` → `type`; `type(red) == color` → `true`; `repr(color)` → `color`.
- `type(color.rgb)` … `type(color.oklch)` → `function` (8 constructors).
- `type(color.lighten)` … `type(color.desaturate)` → `function`; o vanilla
  tem ainda `rotate`, `components`, `space` — **ausentes no cristalino**
  (scope-out; também ausentes como métodos de instância).
- `color("#ff0000")` → erro "type color does not have a constructor"
  (sem constructor — o despacho P685 já emite esta mensagem).
- `color.foo` → erro "type color does not contain field `foo`".

P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out:
4 dos 6 operadores implementados (`saturate`/`desaturate` scope-out futuro P477).

## Função de despacho de fields (P736)

```rust
/// Devolve o valor associado a `field` no tipo `color`, ou `None` se o
/// campo não existir (o chamador emite o erro "does not contain field").
pub fn color_type_field(field: &str) -> Option<Value> {
    // 20 entradas: 8 constructors + 12 operadores.
}
```

Registado em `eval/mod.rs` como `scope.define("color", Value::Type(Type::Color))`.
O braço `(Type::Color, _)` de `eval_field_access` (`eval/bindings.rs`) delega
em `color_type_field` e emite "type color does not contain field `<f>`"
(mensagem verbatim do vanilla) para campo inexistente.

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

### `color.negate(col, space: auto)` → Color

- `col`: `Value::Color`.
- `space`: `Value::Func` cujo nome é um dos 8 constructors de cor
  (`rgb`, `linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`);
  default `auto` = Oklab. Qualquer outro valor produz erro de tipo.
- Delega para `Color::negate(Some(space))` ou `Color::negate(None)`.

### `color.rotate(col, angle, space: auto)` → Color

- `col`: `Value::Color`.
- `angle`: `Value::Angle`.
- `space`: constructor de cor; default `auto` = Oklch. Só espaços com hue
  (Oklch, Hsl, Hsv) são válidos — os restantes produzem "this color space
  does not support hue rotation".
- Delega para `Color::rotate(angle, Some(space))`.

### `color.mix(col1, col2, weight: 0.5, space: auto)` → Color

- `col1`, `col2`: `Value::Color` (posicionais).
- `weight`: named opcional; default `0.5`; tipo igual a `amount`.
- `space`: constructor de cor; default `auto` = Oklab. O resultado fica no
  espaço indicado.
- Delega para `col1.mix(col2, weight, Some(space))`.

### `color.to-hex(col)` → Str

- `col`: `Value::Color`.
- Delega para `Color::to_hex()`.

### `color.transparentize(col, factor)` → Color

- `col`: `Value::Color`.
- `factor`: percentagem [0.0, 1.0] (mesmo tipo que `amount`).
- Delega para `Color::transparentize(factor)`.

### `color.opacify(col, factor)` → Color

- Análogo a `transparentize`; delega para `Color::opacify(factor)`.

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
- ~~`make_color_module()` retorna `Value::Dict` com 4 entradas.~~
  **P736** — substituído por: `color_type_field` devolve `Some(Value::Func(_))`
  para cada um dos 14 fields e `None` para campo inexistente; o scope global
  define `color` como `Value::Type(Type::Color)`.

## P477 — `saturate` e `desaturate`

Adicionados ao módulo e a `make_color_module()` (6 entradas total) —
**P736**: passaram a fields do tipo via `color_type_field`:

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

## P742 — métodos de instância + fields `rotate`/`components`/`space`

**Medição ADR-0108 (sonda contra vanilla 0.15.0 969087ec):** os métodos de
instância `red.lighten(20%)`, `red.darken(20%)`, `red.negate()`,
`red.rotate(90deg)`, `red.mix(blue)`, `red.components()`, `red.space()`,
`red.saturate(20%)`, `red.desaturate(20%)` existem no vanilla e estavam
**ausentes** no cristalino (pré-existente desde P476 — só havia estáticas).
A sonda mediu também que a semântica dos operadores P476/P477 divergia do
vanilla (lighten/darken/saturate via Oklch, negate em sRGB) — **corrigida no
domínio** (`entities/color.md` §"Operadores de cor (P476/P477, semântica
corrigida em P742)"); a correcção beneficia estáticas e instâncias.

Descoberta-chave da sonda: o `red` vanilla é `rgb(1.0, 0.254902, 0.211765)`
(`#ff4136`, `visualize/color.rs:311`), não `#ff0000`; as fórmulas do
`palette 0.7.6` (crate usada pelo vanilla) reproduzem byte a byte os valores
medidos (verificado em scratch dedicado).

### Despacho de métodos de instância (padrão P506)

Braço `Value::Color` no bloco P506 de `eval_func_call` (`closures.rs`),
interceptando **apenas** os 12 métodos conhecidos — método desconhecido cai
no caminho genérico (erro de field, comportamento pré-P742 preservado):

```rust
Value::Color(ref color) => {
    if crate::engine::stdlib::color::is_color_instance_method(method) {
        return super::bindings::eval_color_method(
            color, method, call.args(), scopes, ctx, engine,
        );
    }
}
```

`eval_color_method` (`eval/bindings.rs`) faz `eval_args` uma vez e despacha:

- `lighten` / `darken` / `saturate` / `desaturate` / `negate` / `mix` /
  `rotate` / `to-hex` / `transparentize` / `opacify` — sintetiza `Args` com
  a cor como primeiro posicional e **delega nas nativas estáticas**
  (validação e mensagens idênticas aos dois caminhos).
- `components` — named `alpha: bool = true`; delega em
  `native_color_components`.
- `space` — sem argumentos; devolve `Func::native(<nome do constructor>, <nativa>)`
  fresco (nomes vanilla medidos: `rgb`, `luma`, `linear-rgb`, `oklab`,
  `oklch`, `cmyk`, `hsl`, `hsv`). `red.space() == rgb` é `true` via a
  igualdade por nome de nativas introduzida em `entities/func.md` (P742 —
  medido no vanilla; `Func::eq` era só identidade de Arc).

### Seis fields novos do tipo (20 total)

`color_type_field` passa a 20 entradas: as 14 anteriores + `rotate`,
`components`, `space` (P742) + `to-hex`, `transparentize`, `opacify`
(P744).

- `color.rotate(col, angle, space: auto)` → `Color` — default espaço Oklch.
- `color.components(col, alpha: true)` → `Array` de `Ratio`/`Float`/`Angle`.
- `color.space(col)` → `Func` do constructor do espaço.
- `color.to-hex(col)` → `Str` — hex sRGB com alpha quando aplicável.
- `color.transparentize(col, factor)` → `Color` — reduz opacidade.
- `color.opacify(col, factor)` → `Color` — aumenta opacidade.
- **Nomes plain nas funcs do tipo** (medido P742): `repr(color.rgb)` →
  `rgb`, `repr(color.lighten)` → `lighten` no vanilla (não `color.rgb`) —
  as entradas de `color_type_field` passam a registar os constructors e
  operadores com o nome simples do vanilla.
- **Repr de Func sem `#`** (medido P742): `repr(rgb)` → `rgb`,
  `#red.space()` em markup → `rgb`. `repr.rs` passa a formatar Func
  nomeada como `name` (era `#name`); o ramo sem nome mantém-se.

### Scope-outs P742/P744 (medidos, com erro explícito)

- `mix` variádico com pesos `(cor, peso)` — mantido de P476.
- Repr de closure anónima (`#function(...)`) — o vanilla imprime `(..) => ..`
  (P744); fora do caminho da sonda → achado adiado.

## Scope-out

- `color.mix` com N > 2 cores — scope-out (cristalino aceita 2 + weight).
- `color.saturate/desaturate` com `space:` arg — scope-out (não suportado no vanilla).
- ~~`color.rotate`, `color.components`, `color.space`~~ — **P742: implementados**
  (estáticas + instância).
- ~~Named `space:` em `negate`/`rotate`/`mix`~~ — **P744: implementados**.
- ~~`to-hex`/`transparentize`/`opacify`~~ — **P744: implementados**
  (estáticas + instância).
- ~~Métodos de instância de cor (`red.lighten(20%)`, …)~~ — **P742/P744: implementados**
  (12 métodos; despacho P506).
- `mix` variádico com pesos — scope-out (acima).
