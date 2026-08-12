# Prompt L0 — `stdlib/calc` — subset trig/hiperbólicas/log/exp/constantes
Hash do Código: 11f331b6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/calc.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção de
assinatura e helpers partilhados: ver `stdlib/_comum.md`.
**Passo de origem do subset**: P283 (trig/log/exp + constantes).
**ADRs**: ADR-0018 (libm DEBT agregada), ADR-0101 (IEEE 754 — `guard_float`),
ADR-0054 (perfil graded).

---

## Subset `calc` — 21 entradas (trig + hiperbólicas + exp/log + constantes)

Este prompt cobre o subset fechável do módulo `calc` identificado no P433:
as 7 funções trigonométricas, as 6 hiperbólicas, `exp`, `ln`, `log` (incluindo
a variante com base explícita), e as 4 constantes `pi`/`tau`/`e`/`inf`.

Todas as funções do subset partilham a assinatura padrão de `calc_*`:

```rust
fn calc_X(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>
```

As funções trig/hiperbólicas/exp/ln usam o helper `unary_f64` (1 argumento
posicional, sem named args, coerção `Int|Float` → `f64`, resultado via
`guard_float`). `sin`/`cos`/`tan` usam `unary_angle` (P817-A — aceitam
`Angle` além de `Int|Float`, paridade vanilla `AngleLike`); `asin`/`acos`/
`atan` usam `angle_op` (P817-A — devolvem `Angle`). `atan2` e `log` têm
aridade própria. Ver `diagnostico-calc-passo-283.md` para a decisão libm vs
`f64::*`.

---

### `calc.sin(x)`

**Assinatura**: `sin(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos (`Int` coagido para `f64`, `Float`),
  ou valor `Angle` (convertido para radianos — P817-A).
- Sem argumentos nomeados.

**Semântica**: `f64::sin(x)`, com `guard_float` no resultado.

**Domínio**: ℝ (radianos).

**Paridade vanilla**: Equivalente a `calc.sin(x)` vanilla (`AngleLike`:
aceita `angle` — P817-A, medido `calc.sin(90deg) = 1.0` nos dois binários).

**Testes canónicos**:
```
sin(0) -> 0.0
sin(pi) -> ~0.0
sin(pi/2) -> ~1.0
sin("x") -> Err "esperava Int ou Float"
sin() -> Err "requer 1 argumento"
```

---

### `calc.cos(x)`

**Assinatura**: `cos(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos.

**Semântica**: `f64::cos(x)`; `guard_float`.

**Domínio**: ℝ (radianos).

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
cos(0) -> 1.0
cos(pi) -> -1.0
cos(pi/2) -> ~0.0
```

---

### `calc.tan(x)`

**Assinatura**: `tan(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos.

**Semântica**: `f64::tan(x)`; `guard_float`. Em `x = π/2` devolve um valor
muito grande que `guard_float` pode rejeitar como Inf.

**Domínio**: ℝ \ {π/2 + kπ}; fora do domínio o resultado tende para ±∞ → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
tan(0) -> 0.0
tan(pi/4) -> ~1.0
tan(pi/2) -> Err "resultado é infinito"
```

---

### `calc.asin(x)`

**Assinatura**: `asin(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ [-1, 1]`; em caso contrário `Err`. De seguida
`f64::asin(x)`; resultado em `Value::Angle` (radianos internos).

**Domínio**: `[-1, 1]`.

**Paridade vanilla**: Equivalente a `calc.asin(x)`; devolve `angle`
(P817-A — medido `#repr(calc.asin(0.5))` → `30deg` nos dois binários).

**Testes canónicos**:
```
asin(0) -> 0deg
asin(1) -> 90deg
asin(-1) -> -90deg
asin(1.5) -> Err "valor deve estar entre -1 e 1"
```

---

### `calc.acos(x)`

**Assinatura**: `acos(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ [-1, 1]`; depois `f64::acos(x)`; `Value::Angle`.

**Domínio**: `[-1, 1]`.

**Paridade vanilla / limitações**: Idênticas a `asin` (devolve `angle`).

**Testes canónicos**:
```
acos(0) -> 90deg
acos(1) -> 0deg
acos(-1) -> 180deg
acos(2) -> Err
```

---

### `calc.atan(x)`

**Assinatura**: `atan(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::atan(x)`; `Value::Angle`.

**Domínio**: ℝ.

**Paridade vanilla**: Devolve `angle` (P817-A).

**Testes canónicos**:
```
atan(0) -> 0deg
atan(1) -> 45deg
```

---

### `calc.atan2(x, y)`

**Assinatura**: `atan2(x: Int | Float, y: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.
- 2º posicional `y`.

**Semântica**: Paridade vanilla na **ordem dos parâmetros** (`x` antes de `y`).
Internamente chama `f64::atan2(y, x)`. Resultado em `Value::Angle`.

**Domínio**: `(x, y) ∈ ℝ²`.

**Paridade vanilla**: Equivalente a `calc.atan2(x, y)`; devolve `angle`
(P817-A — medido `#repr(calc.atan2(1, 2))` → `63.43deg` nos dois binários).

**Testes canónicos**:
```
atan2(1, 1) -> 45deg
atan2(0, 1) -> 0deg
atan2(1) -> Err "requer 2 argumentos"
```

---

### `calc.sinh(x)`

**Assinatura**: `sinh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::sinh(x)`; `guard_float`.

**Domínio**: ℝ; valores muito grandes produzem Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
sinh(0) -> 0.0
sinh(1) -> ~1.1752
sinh(1e10) -> Err "resultado é infinito"
```

---

### `calc.cosh(x)`

**Assinatura**: `cosh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::cosh(x)`; `guard_float`.

**Domínio**: ℝ; valores grandes → Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
cosh(0) -> 1.0
cosh(1) -> ~1.5431
cosh(1e10) -> Err "resultado é infinito"
```

---

### `calc.tanh(x)`

**Assinatura**: `tanh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::tanh(x)`; `guard_float`.

**Domínio**: ℝ.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
tanh(0) -> 0.0
tanh(10) -> ~1.0
```

---

### `calc.asinh(x)`

**Assinatura**: `asinh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::asinh(x)`; `guard_float`.

**Domínio**: ℝ.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
asinh(0) -> 0.0
asinh(1) -> ~0.8814
```

---

### `calc.acosh(x)`

**Assinatura**: `acosh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x >= 1`; depois `f64::acosh(x)`; `guard_float`.

**Domínio**: `[1, +∞)`.

**Paridade vanilla / limitações**: Idênticas a `asin`.

**Testes canónicos**:
```
acosh(1) -> 0.0
acosh(2) -> ~1.31696
acosh(0.5) -> Err "valor deve ser >= 1"
```

---

### `calc.atanh(x)`

**Assinatura**: `atanh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ (-1, 1)` estrito; depois `f64::atanh(x)`;
`guard_float`.

**Domínio**: `(-1, 1)`.

**Paridade vanilla / limitações**: Idênticas a `asin`.

**Testes canónicos**:
```
atanh(0) -> 0.0
atanh(0.5) -> ~0.5493
atanh(1) -> Err "valor deve estar em (-1, 1)"
atanh(-1) -> Err
```

---

### `calc.exp(x)`

**Assinatura**: `exp(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::exp(x)`; `guard_float`.

**Domínio**: ℝ; overflow → Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
exp(0) -> 1.0
exp(1) -> ~2.71828
exp(1e10) -> Err "resultado é infinito"
```

---

### `calc.ln(x)`

**Assinatura**: `ln(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x > 0`; depois `f64::ln(x)`; `guard_float`.

**Domínio**: `(0, +∞)`.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
ln(1) -> 0.0
ln(e) -> 1.0
ln(0) -> Err "valor deve ser estritamente positivo"
ln(-1) -> Err
```

---

### `calc.log(x)` / `calc.log(x, base:)` / `calc.log(x, base)`

**Assinatura**: `log(x: Int | Float, base: Int | Float?) -> Float`

**Argumentos**:
- 1º posicional `x`.
- `base`: named opcional (`Int | Float`). Default `10`.
- 2º posicional `base`: forma legada, mutuamente exclusiva com `base:`.

**Semântica**: Despacho por base exacta (P817-F, paridade vanilla
`calc.rs:506-515`): base `e` → `ln(x)`; base `2` → `log2(x)`; base `10` →
`log10(x)`; outras → `ln(x) / ln(base)`. Valida `x > 0`, `base` finita,
`base > 0`, `base != 1`; `guard_float`.

**Domínio**: `x > 0`; `base ∈ (0, +∞) \ {1}`.

**Paridade vanilla**: Equivalente a `calc.log(x)` (base default 10) e a
`calc.log(x, base: b)`.

**Testes canónicos**:
```
log(1) -> 0.0
log(100) -> 2.0
log(100, base: 10) -> 2.0
log(8, 2) -> 3.0
log(8, base: 2) -> 3.0
log(0) -> Err "valor deve ser estritamente positivo"
log(10, 1) -> Err "base inválida"
log(10, base: 1) -> Err "base inválida"
log(10, 0) -> Err
log(10, inf) -> Err
log(100, 10, base: 10) -> Err (ambos especificados)
```

---

### `calc.round(x, digits:)`

**Assinatura**: `round(x: Int | Float, digits: Int?) -> Int | Float`

**Argumentos**:
- 1º posicional `x`.
- `digits`: named opcional (`Int`). Default `0`.

**Semântica**: Arredondamento half-up de `x` com `digits` casas decimais.
`digits` pode ser negativo (arredonda para dezenas, centenas, etc.).
- Se `digits == 0`, `Float` → `Int` (paridade vanilla `round(3.5) == 4`).
- Se `digits != 0`, retorna `Float`.
- `Int` com `digits == 0` → identidade; com `digits != 0` → `Float` arredondado.

**Domínio**: ℝ.

**Paridade vanilla**: Equivalente a `calc.round(x)` e `calc.round(x, digits: n)`.

**Testes canónicos**:
```
round(3.567) -> 4
round(3.567, digits: 2) -> 3.57
round(1234.0, digits: -2) -> 1200.0
round(255, digits: 2) -> 255.0
round(3.5, digits: "x") -> Err
round(3.5, foo: 1) -> Err
```

---

### `calc.pi` (constante)

**Assinatura**: `pi -> Float`

**Semântica**: `Value::Float(std::f64::consts::PI)`.

**Paridade vanilla**: Equivalente a `calc.pi`.

**Testes canónicos**:
```
calc.pi -> 3.141592653589793
```

---

### `calc.tau` (constante)

**Assinatura**: `tau -> Float`

**Semântica**: `Value::Float(std::f64::consts::TAU)`.

**Paridade vanilla**: Equivalente a `calc.tau`.

**Testes canónicos**:
```
calc.tau -> 6.283185307179586
```

---

### `calc.e` (constante)

**Assinatura**: `e -> Float`

**Semântica**: `Value::Float(std::f64::consts::E)`.

**Paridade vanilla**: Equivalente a `calc.e`.

**Testes canónicos**:
```
calc.e -> 2.718281828459045
```

---

### `calc.inf` (constante)

**Assinatura**: `inf -> Float`

**Semântica**: `Value::Float(f64::INFINITY)`.

**Paridade vanilla**: Equivalente a `calc.inf`.

**Testes canónicos**:
```
calc.inf -> inf
```

---

## Restante do módulo `calc` (fora do subset P433)

As funções abaixo continuam documentadas no módulo; o P433 não as reestrutura
em secções individuais porque são scope-out ou já cobertas por passos
anteriores. Mantêm-se as tabelas do estado actual de `calc.md`.

### Funções base (P27)

| Função | Tipos | Semântica |
|--------|-------|-----------|
| `calc_abs` | `Int`, `Float` ou `Decimal` (P817-D) | `Int.saturating_abs()` / `Float.abs()` / `Decimal.abs()` |
| `calc_pow` | `(Int,Int)`, `(Num,Num)` ou `(Decimal,Int)` (P817-C/D) | `0^0` → Err; exp Int não-i32 → Err; exp Float não-normal → Err; `(Int,Int≥0)` → `Int` (`checked_pow`, overflow → Err); `(Int,Int<0)` → `Float` (`powi`); `(Decimal,Int)` → `Decimal` (`checked_powi`); `(Decimal,Float)` → erro dedicado + hint; resto → `powf` |
| `calc_sqrt` | `Int` ou `Float` | argumento negativo → Err |
| `calc_floor` | `Int`, `Float` ou `Decimal` (P817-D) | `Int`→`Int` (identidade); `Float`→`Int`; `Decimal`→`Int` (overflow → Err) |
| `calc_ceil` | `Int`, `Float` ou `Decimal` (P817-D) | idem `floor` com `ceil` |
| `calc_round` | `Int`, `Float` ou `Decimal` (P817-D) | paridade vanilla de tipos: `Int`→`Int` (digits>0 no-op; digits<0 `round_int_com_precisao` away-from-zero); `Float`→`Float` (half away); `Decimal`→`Decimal` (`MidpointAwayFromZero`); `digits:` named opcional (default 0) |
| `calc_min` | `≥1 Num` (mistos Int/Float) | coerção Int→f64 quando misturado |
| `calc_max` | `≥1 Num` | idem `min` |
| `calc_clamp` | `(value, min, max)` | min > max → Err |

### Aritmética inteira, divisão e partes (P306)

| Função | Domínio | Retorno | Semântica |
|--------|---------|---------|-----------|
| `calc_trunc` | `Int`/`Float`/`Decimal` (P817-D) | `Int` | identidade `Int`; `f.trunc() as i64`; `Decimal`→`Int` (overflow → Err) |
| `calc_fract` | `Int`/`Float`/`Decimal` (P817-D) | `Int`/`Float`/`Decimal` | paridade vanilla: `Int` → `Int(0)`; `Float` → `f.fract()`; `Decimal` → `Decimal.fract()` |
| `calc_rem` | `(Num,Num)` | `Int` se ambos `Int`, senão `Float` | truncada (`%` Rust): sinal do dividendo. Zero → `Err` |
| `calc_rem_euclid` | `(Num,Num)` | `Int`/`Float` | Euclidiana; ≥0 para divisor>0. Zero → `Err` |
| `calc_div_euclid` | `(Num,Num)` | `Int`/`Float` | quociente Euclidiano. Zero → `Err` |
| `calc_quo` | `(Num,Num)` | `Int` | quociente **floored** (P817-B). Paridade vanilla `calc.quo(-7,2) = -4`; caminho float faz `floor` e falha fora do alcance i64 |
| `calc_even` | `Int` apenas | `Bool` | `n % 2 == 0`; `Float` → Err |
| `calc_odd` | `Int` apenas | `Bool` | `n % 2 != 0`; `Float` → Err |
| `calc_gcd` | `(Int,Int)` | `Int` | Euclides iterativo sobre `abs`. `gcd(0,0)=0` |
| `calc_lcm` | `(Int,Int)` | `Int` | `a.abs()/gcd(a,b)*b.abs()` com `checked_*`. `lcm(0,x)=0` |
| `calc_fact` | `Int n ≥ 0` | `Int` | loop `1..=n` `checked_mul`; `n<0` → Err; overflow → Err |
| `calc_perm` | `(n≥0, k≥0)` | `Int` | `n·(n-1)·…·(n-k+1)`; `k>n`→0; neg/overflow → Err |
| `calc_binom` | `(n≥0, k≥0)` | `Int` | iterativo divisão exacta; `k>n`→0; neg/overflow → Err |
| `calc_norm` | `..values: Num` + `p: Float` named | `Float` | `(Σ|x_i|^p)^(1/p)`; default `p=2.0` |
| `calc_root` | `(Num radicand, Int index)` — ordem vanilla (P817) | `Float` | `index==0` → Err; `radicand<0` index par → Err; ímpar → raiz real negativa; index negativo → `x^(1/index)` |
| `calc_erf` | `Int`/`Float` | `Float` | Aproximação Abramowitz & Stegun 7.1.26; erro máx 1.5e-7 |

---

## Política IEEE 754 — `guard_float` (ADR-0101 EM VIGOR)

`guard_float(f)` rejeita NaN e Inf no **resultado** de funções matemáticas
escalares (`pow`, `sqrt`, trig, hiperbólicas, log, exp, root, norm, atan2).
`calc.erf` (P308): NaN input → Err; ±∞ → short-circuit ±1.0; ±0 → ±0.0.

**Política transversal cristalina** (ADR-0101):
- `eval`/layout/operators: **IEEE 754 puro** (paridade vanilla).
- `stdlib` funções matemáticas: **rejeita NaN+Inf** (divergência consciente).

---

## Critérios de Verificação (subset P433)

```
calc_sin([Float(0.0)]) -> Ok(Float(0.0))
calc_sin([Float(PI)]) -> Ok(Float(~0.0))
calc_cos([Float(0.0)]) -> Ok(Float(1.0))
calc_cos([Float(PI)]) -> Ok(Float(-1.0))
calc_tan([Float(FRAC_PI_4)]) -> Ok(Float(~1.0))
calc_asin([Float(1.0)]) -> Ok(Float(FRAC_PI_2))
calc_asin([Float(1.5)]) -> Err
calc_acos([Float(-1.0)]) -> Ok(Float(PI))
calc_atan([Float(1.0)]) -> Ok(Float(FRAC_PI_4))
calc_atan2([Float(1.0), Float(1.0)]) -> Ok(Float(FRAC_PI_4))
calc_atan2([Float(1.0)]) -> Err

calc_sinh([Float(0.0)]) -> Ok(Float(0.0))
calc_cosh([Float(0.0)]) -> Ok(Float(1.0))
calc_tanh([Float(0.0)]) -> Ok(Float(0.0))
calc_acosh([Float(1.0)]) -> Ok(Float(0.0))
calc_acosh([Float(0.5)]) -> Err
calc_atanh([Float(0.0)]) -> Ok(Float(0.0))
calc_atanh([Float(1.0)]) -> Err

calc_exp([Float(0.0)]) -> Ok(Float(1.0))
calc_exp([Int(1)]) -> Ok(Float(E))
calc_exp([Float(1e10)]) -> Err
calc_ln([Float(E)]) -> Ok(Float(1.0))
calc_ln([Float(0.0)]) -> Err
calc_log([Float(100.0)]) -> Ok(Float(2.0))
calc_log([Float(8.0), Float(2.0)]) -> Ok(Float(3.0))
calc_log([Float(100.0)], named={base: Int(10)}) -> Ok(Float(2.0))
calc_log([Float(10.0), Float(1.0)]) -> Err
calc_round([Float(3.567)], named={digits: Int(2)}) -> Ok(Float(3.57))
calc_round([Float(1234.0)], named={digits: Int(-2)}) -> Ok(Float(1200.0))

make_calc_module().pi  -> Float(PI)
make_calc_module().tau -> Float(TAU)
make_calc_module().e   -> Float(E)
make_calc_module().inf -> Float(INF)
```
