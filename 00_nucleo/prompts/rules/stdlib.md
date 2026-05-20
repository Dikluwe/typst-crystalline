# Prompt L0 — `rules/stdlib` — Biblioteca Padrão Intrínseca
Hash do Código: 5cfe11d2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib.rs`
**Passo de origem**: Passo 17 (funções nativas base), Passo 25 (rgb/luma),
                     Passo 27 (str/int/float/calc), Passo 283 (trig/log/exp +
                     constantes), Passo 306 (15 funções aritmética inteira),
                     Passo 308 (`erf` — paridade calc 41/41 = 100%)
**ADRs relevantes**: ADR-0016 (spread adiado), ADR-0024 (EcoString/Value::Str),
                     ADR-0018 (libm DEBT agregada), ADR-0054 (perfil
                     observacional graded — tolera aproximação Caminho A
                     `erf`)

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

Constrói `Value::Dict` com **41 funções + 4 constantes** (divergência do original
que usa `Value::Module` — Cristalino usa Dict pois não há stdlib Module sem
world). Acesso via `calc.abs`, `calc.sin`, `calc.pi`, etc. funciona via
`eval_field_access` sobre Dict.

**Marco P308**: paridade categórica `calc` atinge **41/41 = 100%** funções
vanilla (primeira categoria stdlib cristalina a fechar). Cobertura
condicionada ao perfil ADR-0054 graded (sem bit-exact em `erf` — ver §"Função
erro" abaixo).

#### Funções base (P27)

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

#### Trigonometria — radianos (P283)

Aceitam `Int|Float`. **Sem tipo `Angle`** — radianos directos, paridade
`f64::sin`. Conversão deg→rad adiada para passo dedicado ao tipo `Angle`.

| Função | Domínio | Retorno |
|--------|---------|---------|
| `calc_sin` / `calc_cos` / `calc_tan` | R (radianos) | `Float`; `guard_float` captura Inf raro (e.g. `tan(π/2)`) |
| `calc_asin` / `calc_acos` | `[-1, 1]` (inclusivo) | `Float` (radianos); fora → `Err` |
| `calc_atan` | R | `Float` (radianos) |
| `calc_atan2` | `(x, y)` ∈ R² | `Float` (radianos). **Ordem `(x, y)` na chamada** preservada de vanilla; internamente chama `f64::atan2(y, x)` |

#### Hiperbólicas (P283)

| Função | Domínio | Retorno |
|--------|---------|---------|
| `calc_sinh` / `calc_cosh` / `calc_tanh` / `calc_asinh` | R | `Float`; `cosh` grande → Inf → `Err` via `guard_float` |
| `calc_acosh` | `[1, +∞)` | `Float`; <1 → `Err` |
| `calc_atanh` | `(-1, 1)` **estrito** | `Float`; ±1 ou fora → `Err` |

#### Exponencial / logaritmos (P283)

| Função | Domínio | Notas |
|--------|---------|-------|
| `calc_exp` | R | overflow → `Err` via `guard_float` |
| `calc_ln` | `(0, +∞)` | ≤0 → `Err` |
| `calc_log(x[, base])` | x>0; base∈(0,+∞)∖{1}, finita | base default = 10. **Divergência vanilla**: posicional em vez de named arg `base:` — registado em `diagnostico-calc-passo-283.md` §A.1 |

#### Constantes (P283)

| Chave | Valor |
|-------|-------|
| `calc.pi`  | `std::f64::consts::PI`  |
| `calc.tau` | `std::f64::consts::TAU` |
| `calc.e`   | `std::f64::consts::E`   |
| `calc.inf` | `f64::INFINITY`         |

**DEBT (ADR-0018)**: `calc_pow`, `trig_op` (cobre `sin/cos/tan/asin/acos/atan/
sinh/cosh/tanh/asinh/acosh/atanh/exp`), `calc_atan2`, `calc_ln`/`calc_log`,
`calc_root`/`calc_norm` (via `f64::powf`) e `calc_erf` (via `f64::exp` no helper
de aproximação P308) usam `f64::*` directamente com
`#[allow(clippy::disallowed_methods)]`. Centralização em `trig_op` reduz a
migração futura para `libm::*` a ≤7 sítios (ver
`diagnostico-calc-passo-283.md` §A.2 para racional da opção (c) escolhida em
P283; P308 adiciona o sétimo sítio sem reabrir a decisão).

#### Aritmética inteira, divisão e partes (P306)

| Função | Domínio | Retorno | Semântica |
|--------|---------|---------|-----------|
| `calc_trunc` | `Int` ou `Float` | `Int` | identidade para `Int`; `f.trunc() as i64` para `Float` |
| `calc_fract` | `Int` ou `Float` | `Float` | `0.0` para `Int`; `f.fract()` para `Float` |
| `calc_rem` | `(Num, Num)` | `Int` se ambos `Int`, senão `Float` | semântica truncada (`%` Rust): sinal acompanha o dividendo. Divisor zero → `Err` |
| `calc_rem_euclid` | `(Num, Num)` | `Int`/`Float` | semântica Euclidiana via `i64::rem_euclid` / `f64::rem_euclid`. Resultado sempre ≥ 0 para divisor > 0. Divisor zero → `Err` |
| `calc_div_euclid` | `(Num, Num)` | `Int`/`Float` | quociente Euclidiano. Divisor zero → `Err` |
| `calc_quo` | `(Num, Num)` | `Int` | quociente truncado: `Int/Int → Int` directo; `Float → trunc as i64`. Paridade vanilla `calc.quo(-7, 2) = -3` |

#### Predicados inteiros (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_even` | `Int` apenas | `Bool` (`n % 2 == 0`) |
| `calc_odd`  | `Int` apenas | `Bool` (`n % 2 != 0`) |

Float é rejeitado (`Err("…requer Int…")`) — semântica vanilla.

#### Teoria dos números (P306)

| Função | Args | Algoritmo |
|--------|------|-----------|
| `calc_gcd` | `(Int, Int)` | Euclides iterativo sobre `a.abs()`, `b.abs()`. `gcd(0, 0) = 0` por convenção |
| `calc_lcm` | `(Int, Int)` | `a.abs() / gcd(a, b) * b.abs()` com `checked_div`/`checked_mul` (dividir antes para evitar overflow intermédio). `lcm(0, x) = 0` |

#### Combinatória (P306)

| Função | Args | Algoritmo | Erros |
|--------|------|-----------|-------|
| `calc_fact` | `Int n ≥ 0` | loop `1..=n` com `checked_mul` | `n < 0` → `Err("factorial de negativo")`; overflow (`fact(21)` para `i64`) → `Err("número demasiado grande")` |
| `calc_perm` | `(Int n ≥ 0, Int k ≥ 0)` | `n * (n-1) * … * (n-k+1)` iterativo com `checked_mul`. `k > n` retorna `0` | argumentos negativos → `Err`; overflow → `Err` |
| `calc_binom` | `(Int n ≥ 0, Int k ≥ 0)` | iterativo com `k = k.min(n-k)` (simetria); a cada passo multiplica por `(n-i)` (`checked_mul`) e divide exactamente por `(i+1)`. Garante ausência de overflow intermédio sempre que o resultado caiba em `i64`. `k > n` retorna `0` | idem `perm` |

A divisão exacta a cada iteração de `binom` é matematicamente garantida pela
propriedade dos coeficientes binomiais (`C(n, k)` é sempre inteiro e o
produto parcial após `i` passos divide-se exactamente por `(i+1)`).

#### Norma vectorial (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_norm` | `..values: Num` posicionais + `p: Float` named (default `2.0`) | `Float` (`(Σ \|x_i\|^p)^(1/p)`) |

Convenções:
- Sem argumentos posicionais → retorna `Float(0.0)` (norma do vector vazio).
- `p` aceita named arg ou — quando ausente — assume `2.0` (norma Euclidiana).
- Resultado passa por `guard_float` (captura `NaN`/`Inf` para inputs
  degenerados como `p = 0` com algum valor nulo).
- Usa `f64::powf` — DEBT-libm partilhado com `calc_pow` e `calc_root`.

#### Raiz n-ésima (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_root` | `(Int index, Num x)` | `Float` |

Semântica vanilla:
- `index == 0` → `Err("índice de raiz zero")`.
- `x < 0` com `index` par → `Err("raiz par de número negativo")`.
- `x < 0` com `index` ímpar → ramo simétrico negativo: `-(-x).powf(1.0 / index as f64)` (preserva sinal: `root(3, -8) = -2.0`).
- `x ≥ 0` → `x.powf(1.0 / index as f64)`; passa por `guard_float`.

#### Função erro (P308)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_erf` | `Int` ou `Float` | `Float` ∈ `[-1, 1]` |

Implementa a função erro de Gauss `erf(x) = (2/√π) ∫₀ˣ exp(-t²) dt`.

**Diagnóstico inline P308a — escolha de aproximação**:

- **Vanilla** (`lab/typst-original/.../foundations/calc.rs`): delega a
  `libm::erf(value)` — implementação Chebyshev rational (Caminho C, erro
  < 2⁻⁵³, precisão IEEE 754 dupla).
- **Cristalino**: `libm` não está em `[l1_allowed_external]` (ADR-0018
  DEBT compartilhado com `f64::powf`, `f64::sin`, etc.). Adoptar `libm`
  apenas para `erf` violaria o princípio de migração agregada.
- **Escolha**: **Caminho A — Abramowitz & Stegun 7.1.26**, aproximação
  polinomial pura em Rust com 5 coeficientes + constante:
  ```
  t = 1 / (1 + p * |x|)
  poly = t·(a₁ + t·(a₂ + t·(a₃ + t·(a₄ + t·a₅))))         (Horner)
  erf(x) ≈ sign(x) · (1 - poly · exp(-x²))
  com p = 0.3275911,
      a₁ = 0.254829592, a₂ = -0.284496736, a₃ = 1.421413741,
      a₄ = -1.453152027, a₅ = 1.061405429.
  ```
- **Precisão**: erro máximo absoluto **1,5 × 10⁻⁷** — dentro da
  tolerância ADR-0054 graded (paridade observável; não bit-exact). Para
  consumidores que requerem bit-exact face vanilla, futura migração
  agregada a `libm` (ADR-0018) resolve simultaneamente `erf`, `pow`,
  trig, etc.
- **Determinismo**: produto e soma `f64` IEEE 754; `f64::exp` é
  determinístico nas plataformas suportadas.

**Domínio e casos de borda**:
- `x ∈ R` finito, `x ≠ 0` → fórmula directa.
- `x = +∞` → short-circuit retorna `Float(1.0)` (limite analítico).
  Sem short-circuit a fórmula produziria `0 · ∞ = NaN` em
  `poly · exp(-∞)`.
- `x = -∞` → short-circuit retorna `Float(-1.0)`.
- `x = NaN` → `Err("calc.erf() valor é NaN")` (divergência consciente
  vs vanilla `libm::erf(NaN) = NaN`; paridade com convenção cristalina
  `guard_float`/`coerce_to_f64` em todas as outras unárias).
- `x = ±0.0` → **short-circuit** retorna `Float(x)` (preserva sinal de
  zero). Sem este desvio, A&S 7.1.26 produziria `~1e-9` em vez de
  `0` exacto: a soma analítica `a₁+a₂+a₃+a₄+a₅ = 1.0` recebe ruído
  de arredondamento `f64` (≈ 1e-9) que se torna o resultado quando
  `x = 0`. Paridade vanilla `libm::erf(0) = 0` mantida bit-exact.

**Tipos aceites**:
- `Int` (coerce via `coerce_to_f64`) e `Float` posicional único.
- Sem argumentos posicionais → `Err`.
- Argumentos named → `Err` (paridade `expect_no_named`).

**DEBT-libm (ADR-0018)** compartilhado: usa `f64::exp` directo com
`#[allow(clippy::disallowed_methods)]`. Migração futura agregada para
`libm::erf` (juntamente com `libm::exp`, `libm::pow`, etc.) toca também
este sítio.

**Funções vanilla adiadas** (pós-P308):
- ~~`erf`~~ — **fechado P308** (Caminho A; Abramowitz & Stegun 7.1.26).
- Extensões `Length`/`Angle`/`Decimal`/`digits` em funções existentes
  (escopo separado; sem `Angle` type ainda).

---

## Helpers Internos

| Função | Uso |
|--------|-----|
| `coerce_to_f64(v, ctx)` | `Int`→`f64`, `Float`→`f64`, outros → Err com contexto |
| `guard_float(f)` | NaN → Err "não é um número", Inf → Err "infinito" |
| `format_float(f)` | compacto sem trailing zeros; garante ponto decimal (`"3.0"`) |
| `format_length(l)` | `Length` → `"12pt"`, `"1.5em"`, `"6pt + 1em"` |

---

## Política IEEE 754 — `guard_float` (ADR-0101 EM VIGOR)

`guard_float(f)` rejeita NaN e Inf no **resultado** de funções
matemáticas escalares (`calc.pow`, `calc.sqrt`, trig, hiperbólicas,
log, exp, root, norm, atan2). `calc.erf` (P308) tem política dedicada:
NaN no input → Err; ±∞ → short-circuit ±1.0; ±0 → short-circuit ±0.0.

**Política transversal cristalina** (ADR-0101):
- `eval`/layout/operators: **IEEE 754 puro** (paridade vanilla;
  ver `eval.md` §"Política IEEE 754 — propagação silenciosa").
- `stdlib` funções matemáticas: **rejeita NaN+Inf** (divergência
  consciente vanilla; ADR-0101).

Esta divergência é **categórica e consciente**, não acidental. Vanilla
`sin/cos/tan/.../erf` retornam `f64` transparente; cristalino encerra
em `SourceResult<Value>` via `guard_float`. Racional completo em
ADR-0101 §"Racional".

**Sítios afectados** (11 directos em `calc.rs`):
- `calc_pow`/`calc_sqrt` via `guard_float`.
- `calc_sin/cos/tan/asin/acos/atan/sinh/cosh/tanh/asinh/acosh/atanh/
  exp` (13 funções via wrapper `trig_op` partilhado).
- `calc_atan2`/`calc_ln`/`calc_log`/`calc_norm`/`calc_root` directos.
- `calc_erf` input + short-circuits dedicados.
- `calc_log` base (`!is_finite() → Err`).

**Excepções não cobertas por ADR-0101** (adiadas para reforços
pontuais futuros, per P309 §10.5/§10.6):
- Cat D Float→Int saturating (`floor`/`ceil`/`round`/`trunc`/`quo`).
- Cat D color constructors (`oklab/oklch/cmyk/hsl/hsv` sem range
  check `[0.0, 1.0]`).
- Backdoor `native_float("NaN")` aceita string literal.

Catálogo completo de 67 sítios L1 em
`00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md`. Comparação
sítio-a-sítio vanilla em P309 §4.3.

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

// P283 — trig (tolerância 1e-10 implícita)
calc_sin([Float(0.0)])            → Ok(Float(0.0))
calc_sin([Float(PI)])             → Ok(Float(~0.0))
calc_cos([Float(0.0)])            → Ok(Float(1.0))
calc_cos([Float(PI)])             → Ok(Float(-1.0))
calc_tan([Float(FRAC_PI_4)])      → Ok(Float(~1.0))
calc_asin([Float(1.0)])           → Ok(Float(FRAC_PI_2))
calc_asin([Float(1.5)])           → Err (fora de [-1, 1])
calc_acos([Float(-1.0)])          → Ok(Float(PI))
calc_atan([Float(1.0)])           → Ok(Float(FRAC_PI_4))
calc_atan2([Float(1.0), Float(1.0)])  → Ok(Float(FRAC_PI_4))   // ordem (x, y)
calc_atan2([Float(1.0)])              → Err (arity)

// P283 — hiperbólicas
calc_sinh([Float(0.0)])           → Ok(Float(0.0))
calc_cosh([Float(0.0)])           → Ok(Float(1.0))
calc_tanh([Float(0.0)])           → Ok(Float(0.0))
calc_acosh([Float(1.0)])          → Ok(Float(0.0))
calc_acosh([Float(0.5)])          → Err (< 1)
calc_atanh([Float(0.0)])          → Ok(Float(0.0))
calc_atanh([Float(1.0)])          → Err (fronteira ±1 exclusiva)

// P283 — exp/log
calc_exp([Float(0.0)])            → Ok(Float(1.0))
calc_exp([Int(1)])                → Ok(Float(E))
calc_exp([Float(1e10)])           → Err (overflow → Inf via guard_float)
calc_ln([Float(E)])               → Ok(Float(1.0))
calc_ln([Float(0.0)])             → Err
calc_log([Float(100.0)])                  → Ok(Float(2.0))     // base default 10
calc_log([Float(8.0), Float(2.0)])        → Ok(Float(3.0))
calc_log([Float(10.0), Float(1.0)])       → Err (base = 1)
calc_log([Float(10.0), Float(f64::INFINITY)]) → Err (base não-finita)

// P283 — constantes via make_calc_module
make_calc_module().pi  ≡ Float(std::f64::consts::PI)
make_calc_module().tau ≡ Float(std::f64::consts::TAU)
make_calc_module().e   ≡ Float(std::f64::consts::E)
make_calc_module().inf ≡ Float(f64::INFINITY)

// P306 — trunc / fract
calc_trunc([Int(5)])         → Ok(Int(5))             // identidade
calc_trunc([Float(3.7)])     → Ok(Int(3))
calc_trunc([Float(-3.7)])    → Ok(Int(-3))            // truncamento para zero
calc_trunc([Str("x")])       → Err
calc_fract([Int(5)])         → Ok(Float(0.0))
calc_fract([Float(3.7)])     → Ok(Float(~0.7))
calc_fract([Float(-3.7)])    → Ok(Float(~-0.7))

// P306 — even / odd (Int apenas)
calc_even([Int(4)])          → Ok(Bool(true))
calc_even([Int(-3)])         → Ok(Bool(false))
calc_even([Int(0)])          → Ok(Bool(true))
calc_even([Float(2.0)])      → Err   // Float rejeitado
calc_odd([Int(3)])           → Ok(Bool(true))
calc_odd([Int(0)])           → Ok(Bool(false))

// P306 — rem (semântica truncada %)
calc_rem([Int(7), Int(3)])         → Ok(Int(1))
calc_rem([Int(-7), Int(3)])        → Ok(Int(-1))     // sinal do dividendo
calc_rem([Float(7.5), Float(2.0)]) → Ok(Float(~1.5))
calc_rem([Int(1), Int(0)])         → Err              // divisão por zero
calc_rem([Int(5)])                 → Err              // arity

// P306 — rem_euclid (sempre ≥ 0 para divisor > 0)
calc_rem_euclid([Int(-7), Int(3)]) → Ok(Int(2))
calc_rem_euclid([Float(-7.5), Float(2.0)]) → Ok(Float(~0.5))
calc_rem_euclid([Int(1), Int(0)])  → Err

// P306 — div_euclid
calc_div_euclid([Int(-7), Int(3)]) → Ok(Int(-3))     // i64::div_euclid
calc_div_euclid([Int(7), Int(3)])  → Ok(Int(2))
calc_div_euclid([Int(1), Int(0)])  → Err

// P306 — quo (quociente truncado)
calc_quo([Int(7), Int(2)])         → Ok(Int(3))
calc_quo([Int(-7), Int(2)])        → Ok(Int(-3))     // truncado, não Euclidiano
calc_quo([Float(7.5), Float(2.0)]) → Ok(Int(3))
calc_quo([Int(1), Int(0)])         → Err

// P306 — gcd / lcm
calc_gcd([Int(12), Int(18)])       → Ok(Int(6))
calc_gcd([Int(0), Int(5)])         → Ok(Int(5))
calc_gcd([Int(0), Int(0)])         → Ok(Int(0))      // convenção
calc_gcd([Int(-12), Int(18)])      → Ok(Int(6))      // abs nos dois
calc_lcm([Int(4), Int(6)])         → Ok(Int(12))
calc_lcm([Int(0), Int(5)])         → Ok(Int(0))
calc_lcm([Int(i64::MAX), Int(2)])  → Err              // overflow

// P306 — fact
calc_fact([Int(0)])                → Ok(Int(1))
calc_fact([Int(5)])                → Ok(Int(120))
calc_fact([Int(-1)])               → Err
calc_fact([Int(21)])               → Err              // overflow i64
calc_fact([Float(5.0)])            → Err              // Int only

// P306 — perm (arranjos)
calc_perm([Int(5), Int(2)])        → Ok(Int(20))      // 5*4
calc_perm([Int(5), Int(0)])        → Ok(Int(1))       // n!/(n)!
calc_perm([Int(5), Int(8)])        → Ok(Int(0))       // k > n
calc_perm([Int(-1), Int(2)])       → Err

// P306 — binom (combinações)
calc_binom([Int(5), Int(2)])       → Ok(Int(10))
calc_binom([Int(5), Int(0)])       → Ok(Int(1))
calc_binom([Int(5), Int(5)])       → Ok(Int(1))
calc_binom([Int(5), Int(8)])       → Ok(Int(0))       // k > n
calc_binom([Int(-1), Int(2)])      → Err
calc_binom([Int(67), Int(33)])     → Ok(Int(_))       // grande mas cabe em i64

// P306 — norm (vector p-norm; p named, default 2.0)
calc_norm([])                                  → Ok(Float(0.0))     // vector vazio
calc_norm([Int(3), Int(4)])                    → Ok(Float(5.0))     // p=2 default
calc_norm([Int(3), Int(4)], p: Float(2.0))     → Ok(Float(5.0))
calc_norm([Int(1), Int(1), Int(1)], p: Float(1.0)) → Ok(Float(3.0)) // taxicab
calc_norm([Float(-3.0), Float(4.0)])           → Ok(Float(5.0))     // abs por dentro
calc_norm([Int(3)], p: Int(0))                 → Err                  // NaN/Inf via guard_float

// P306 — root (raiz n-ésima)
calc_root([Int(2), Int(9)])        → Ok(Float(3.0))
calc_root([Int(3), Int(8)])        → Ok(Float(2.0))
calc_root([Int(3), Int(-8)])       → Ok(Float(-2.0))  // ramo ímpar simétrico
calc_root([Int(2), Int(-1)])       → Err              // raiz par de negativo
calc_root([Int(0), Int(5)])        → Err              // índice zero
calc_root([Int(2), Float(0.0)])    → Ok(Float(0.0))

// P308 — erf (função erro de Gauss; tolerância 1.5e-7 perfil graded)
calc_erf([Float(0.0)])             → Ok(Float(0.0))                  // identidade
calc_erf([Int(0)])                 → Ok(Float(0.0))                  // Int coerce
calc_erf([Float(1.0)])             → Ok(Float(~0.84270079))          // valor conhecido
calc_erf([Float(-1.0)])            → Ok(Float(~-0.84270079))         // simetria ímpar
calc_erf([Float(0.5)])             → Ok(Float(~0.52049988))
calc_erf([Float(2.0)])             → Ok(Float(~0.99532227))
calc_erf([Float(5.0)])             → Ok(Float(~1.0))                 // ≈1, dentro 1.5e-7
calc_erf([Float(-5.0)])            → Ok(Float(~-1.0))                // simetria
calc_erf([Float(f64::INFINITY)])   → Ok(Float(1.0))                  // short-circuit limite
calc_erf([Float(f64::NEG_INFINITY)]) → Ok(Float(-1.0))               // short-circuit limite
calc_erf([Float(f64::NAN)])        → Err                              // guard NaN
calc_erf([])                       → Err                              // arity 0
calc_erf([Float(1.0), Float(2.0)]) → Err                              // arity 2
calc_erf([Str("x")])               → Err                              // tipo inválido
// Named arg rejeitado (paridade expect_no_named):
calc_erf([Float(1.0)], named: { "p": Float(0.5) }) → Err
```

## `smartquote(double?, enabled?)` — Passo 287 (`P-smartquote`)

Função stdlib paralela ao markup `"foo"`/`'bar'` (P155). Emite
`Content::SmartQuote { double }` que o consumer Layouter resolve
lang-aware via `localize_quotes` + state per-document
(`Layouter.smartquote_*_open`). Diagnóstico completo em
`diagnostico-smartquote-passo-287.md`.

```rust
pub fn native_smartquote(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value>
```

**Argumentos**:
- `double: bool = true` — aspas duplas (default; vanilla paridade).
- `enabled: bool = true` — quando `false`, emite `Content::Text(glyph)`
  ASCII literal directo (não passa pelo variant). Paridade vanilla
  `set smartquote(enabled: false)`.
- **Não aceita argumentos posicionais** (vanilla usa só named).

**Scope-out per ADR-0054 graded** (erro educacional mencionando ADR):
- `alternative: bool` — alternância de quotes em DE/FR (vanilla
  feature); passo dedicado futuro condicional.
- `quotes: Smart<SmartQuoteDict>` — custom string/array/dict override.

**Construção de variant**:
```
native_smartquote()                              → Ok(Value::Content(Content::SmartQuote { double: true }))
native_smartquote(double: false)                 → Ok(Value::Content(Content::SmartQuote { double: false }))
native_smartquote(enabled: false)                → Ok(Value::Content(Content::text("\"")))    // ASCII directo
native_smartquote(enabled: false, double: false) → Ok(Value::Content(Content::text("'")))
native_smartquote(alternative: true)             → Err (scope-out P287 / ADR-0054)
native_smartquote(quotes: "()")                  → Err (scope-out P287 / ADR-0054)
native_smartquote(Bool(true))                    → Err (sem args posicionais)
```

**Layouter consumer** (resolve glyph lang-aware): ver `content.md`
secção "Variant Content::SmartQuote — Passo 287".

## `underline(body, stroke?, offset?, extent?)` / `strike(body, ...)` / `overline(body, ...)` — Passo 284 (ADR-0054 graded)

Decoração textual paralela vanilla `text/deco.rs`. Três funções com `body`
posicional obrigatório (Content ou Str) + cosméticos opcionais. Emit reusa
`FrameItem::Line` existente; offsets Y default por kind (constantes em em-units
no espaço Layouter):

| Função | offset_em default | Posição visual |
|--------|---:|---|
| `underline` | `+0.10` | logo abaixo do baseline |
| `strike`    | `-0.25` | atravessa o x-height |
| `overline`  | `-0.80` | acima do cap-height |

```rust
pub fn native_underline(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value>
// — assinaturas idênticas para native_strike e native_overline.
```

**Argumentos named aceites**:
- `stroke: Color | none` — paint da linha. **Apenas Color (paint puro)**;
  objecto `Stroke` rico vanilla (paint+thickness+cap+dash) é scope-out
  per diagnóstico §A.1 (Tabela A.7 linha 201 stroke parcial). `none`
  desactiva default.
- `offset: Length | none` — override do offset Y default (ver tabela
  acima). Resolução via `Length::resolve_pt(font_size_pt)`. `Int`/`Float`
  posicionais aceites como pt.
- `extent: Length | none` — extende a linha horizontalmente além do body
  (positiva ou negativa); `None` = 0pt.

**Scope-out vanilla (registar erro explícito que referencia ADR-0054)**:
- `evade: bool` — descender skipping; requer geometria glifo-a-glifo.
- `background: bool` — z-order da decoração.

**Construção de variant**:
```
native_underline([Content(c)])              → Ok(Value::Content(Content::Underline {
                                                  body: c, stroke: None, offset: None, extent: None }))
native_underline([Str("x")])                → Ok(Value::Content(Content::Underline { body: text("x"), ... }))
native_underline([], stroke: Color)         → Err (body obrigatório)
native_underline([Content(c)], evade: true) → Err (scope-out P284 / ADR-0054)
```

Idem para `native_strike` (sem `evade` em vanilla) e `native_overline`.

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
