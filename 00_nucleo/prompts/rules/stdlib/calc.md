# Prompt L0 — `stdlib/calc` — módulo `calc`
Hash do Código: a9e879a5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/calc.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção de
assinatura e helpers partilhados: ver `stdlib/_comum.md`.
**Passo de origem**: P27 (base), P283 (trig/log/exp + constantes), P306 (15
funções aritmética inteira), P308 (`erf`).
**ADRs**: ADR-0018 (libm DEBT agregada), ADR-0101 (IEEE 754 — guard_float),
ADR-0054 (perfil graded — `erf`).

---

## Módulo `calc` — `make_calc_module() -> Value`

Constrói `Value::Dict` com **41 funções + 4 constantes** (divergência do original
que usa `Value::Module` — Cristalino usa Dict pois não há stdlib Module sem
world). Acesso via `calc.abs`, `calc.sin`, `calc.pi`, etc. via
`eval_field_access` sobre Dict.

**Marco P308**: paridade categórica `calc` atinge **41/41 = 100%** funções
vanilla (primeira categoria stdlib cristalina a fechar). Cobertura condicionada
ao perfil ADR-0054 graded (sem bit-exact em `erf`).

### Funções base (P27)

| Função | Tipos | Semântica |
|--------|-------|-----------|
| `calc_abs` | `Int` ou `Float` | `Int.saturating_abs()` / `Float.abs()` |
| `calc_pow` | `(Int,Int)` ou `(Num,Num)` | exp negativo em Int → Err; Float usa `powf` |
| `calc_sqrt` | `Int` ou `Float` | argumento negativo → Err |
| `calc_floor` | `Int` ou `Float` | `Int`→`Int` (identidade); `Float`→`Int` |
| `calc_ceil` | `Int` ou `Float` | idem `floor` com `ceil` |
| `calc_round` | `Int` ou `Float` | arredondamento half-up |
| `calc_min` | `≥1 Num` (mistos Int/Float) | coerção Int→f64 quando misturado |
| `calc_max` | `≥1 Num` | idem `min` |
| `calc_clamp` | `(value, min, max)` | min > max → Err |

### Trigonometria — radianos (P283)

Aceitam `Int|Float`. **Sem tipo `Angle`** — radianos directos, paridade
`f64::sin`. Conversão deg→rad adiada para passo dedicado ao tipo `Angle`.

| Função | Domínio | Retorno |
|--------|---------|---------|
| `calc_sin`/`calc_cos`/`calc_tan` | R (radianos) | `Float`; `guard_float` captura Inf raro (`tan(π/2)`) |
| `calc_asin`/`calc_acos` | `[-1, 1]` inclusivo | `Float` (radianos); fora → `Err` |
| `calc_atan` | R | `Float` (radianos) |
| `calc_atan2` | `(x, y)` ∈ R² | `Float` (radianos). **Ordem `(x, y)`** na chamada; internamente `f64::atan2(y, x)` |

### Hiperbólicas (P283)

| Função | Domínio | Retorno |
|--------|---------|---------|
| `calc_sinh`/`calc_cosh`/`calc_tanh`/`calc_asinh` | R | `Float`; `cosh` grande → Inf → `Err` |
| `calc_acosh` | `[1, +∞)` | `Float`; <1 → `Err` |
| `calc_atanh` | `(-1, 1)` **estrito** | `Float`; ±1 ou fora → `Err` |

### Exponencial / logaritmos (P283)

| Função | Domínio | Notas |
|--------|---------|-------|
| `calc_exp` | R | overflow → `Err` via `guard_float` |
| `calc_ln` | `(0, +∞)` | ≤0 → `Err` |
| `calc_log(x[, base])` | x>0; base∈(0,+∞)∖{1}, finita | base default = 10. **Divergência vanilla**: posicional em vez de named `base:` (`diagnostico-calc-passo-283.md` §A.1) |

### Constantes (P283)

| Chave | Valor |
|-------|-------|
| `calc.pi` | `std::f64::consts::PI` |
| `calc.tau` | `std::f64::consts::TAU` |
| `calc.e` | `std::f64::consts::E` |
| `calc.inf` | `f64::INFINITY` |

**DEBT (ADR-0018)**: `calc_pow`, `trig_op` (cobre `sin/cos/tan/asin/acos/atan/
sinh/cosh/tanh/asinh/acosh/atanh/exp`), `calc_atan2`, `calc_ln`/`calc_log`,
`calc_root`/`calc_norm` (via `f64::powf`) e `calc_erf` (via `f64::exp`) usam
`f64::*` directamente com `#[allow(clippy::disallowed_methods)]`. Centralização
em `trig_op` reduz a migração futura para `libm::*` a ≤7 sítios
(`diagnostico-calc-passo-283.md` §A.2; P308 adiciona o sétimo sem reabrir).

### Aritmética inteira, divisão e partes (P306)

| Função | Domínio | Retorno | Semântica |
|--------|---------|---------|-----------|
| `calc_trunc` | `Int`/`Float` | `Int` | identidade `Int`; `f.trunc() as i64` |
| `calc_fract` | `Int`/`Float` | `Float` | `0.0` para `Int`; `f.fract()` |
| `calc_rem` | `(Num,Num)` | `Int` se ambos `Int`, senão `Float` | truncada (`%` Rust): sinal do dividendo. Zero → `Err` |
| `calc_rem_euclid` | `(Num,Num)` | `Int`/`Float` | Euclidiana; ≥0 para divisor>0. Zero → `Err` |
| `calc_div_euclid` | `(Num,Num)` | `Int`/`Float` | quociente Euclidiano. Zero → `Err` |
| `calc_quo` | `(Num,Num)` | `Int` | quociente truncado. Paridade `calc.quo(-7,2) = -3` |

### Predicados inteiros (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_even` | `Int` apenas | `Bool` (`n % 2 == 0`) |
| `calc_odd` | `Int` apenas | `Bool` (`n % 2 != 0`) |

Float é rejeitado (`Err`) — semântica vanilla.

### Teoria dos números (P306)

| Função | Args | Algoritmo |
|--------|------|-----------|
| `calc_gcd` | `(Int,Int)` | Euclides iterativo sobre `abs`. `gcd(0,0)=0` |
| `calc_lcm` | `(Int,Int)` | `a.abs()/gcd(a,b)*b.abs()` com `checked_*` (dividir antes). `lcm(0,x)=0` |

### Combinatória (P306)

| Função | Args | Algoritmo | Erros |
|--------|------|-----------|-------|
| `calc_fact` | `Int n ≥ 0` | loop `1..=n` `checked_mul` | `n<0` → Err; overflow (`fact(21)`) → Err |
| `calc_perm` | `(n≥0, k≥0)` | `n·(n-1)·…·(n-k+1)` `checked_mul`; `k>n`→0 | negativos → Err; overflow → Err |
| `calc_binom` | `(n≥0, k≥0)` | iterativo `k=k.min(n-k)`; multiplica `(n-i)` e divide exacto `(i+1)`; `k>n`→0 | idem `perm` |

A divisão exacta a cada iteração de `binom` é garantida pela propriedade dos
coeficientes binomiais (o produto parcial após `i` passos divide-se exactamente
por `(i+1)`).

### Norma vectorial (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_norm` | `..values: Num` + `p: Float` named (default `2.0`) | `Float` (`(Σ\|x_i\|^p)^(1/p)`) |

- Sem posicionais → `Float(0.0)`. `p` named ou default `2.0` (Euclidiana).
- Resultado passa por `guard_float`. Usa `f64::powf` (DEBT-libm).

### Raiz n-ésima (P306)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_root` | `(Int index, Num x)` | `Float` |

- `index == 0` → Err. `x<0` index par → Err. `x<0` index ímpar → `-(-x).powf(1/index)` (`root(3,-8)=-2.0`). `x≥0` → `x.powf(1/index)`; `guard_float`.

### Função erro (P308)

| Função | Args | Retorno |
|--------|------|---------|
| `calc_erf` | `Int`/`Float` | `Float` ∈ `[-1, 1]` |

`erf(x) = (2/√π) ∫₀ˣ exp(-t²) dt`. **Caminho A — Abramowitz & Stegun 7.1.26**
(vanilla usa `libm::erf`, fora de `[l1_allowed_external]` por ADR-0018):

```
t = 1 / (1 + p·|x|)
poly = t·(a₁ + t·(a₂ + t·(a₃ + t·(a₄ + t·a₅))))         (Horner)
erf(x) ≈ sign(x)·(1 - poly·exp(-x²))
com p = 0.3275911,
    a₁=0.254829592, a₂=-0.284496736, a₃=1.421413741,
    a₄=-1.453152027, a₅=1.061405429.
```

- **Precisão**: erro máximo **1,5×10⁻⁷** (perfil ADR-0054 graded).
- **Casos de borda** (short-circuits dedicados): `x=+∞`→`1.0`; `x=-∞`→`-1.0`;
  `x=NaN`→`Err` (divergência consciente vs `libm::erf(NaN)=NaN`); `x=±0.0`→`x`
  (preserva sinal; sem isto A&S produziria `~1e-9` em vez de 0 exacto).
- **Tipos**: `Int`(coerce)/`Float` posicional único; 0 args → Err; named → Err.
- **DEBT-libm (ADR-0018)**: `f64::exp` directo; migração agregada futura.

**Funções vanilla adiadas** (pós-P308): extensões `Length`/`Angle`/`Decimal`/
`digits` em funções existentes (sem tipo `Angle` ainda).

---

## Política IEEE 754 — `guard_float` (ADR-0101 EM VIGOR)

`guard_float(f)` rejeita NaN e Inf no **resultado** de funções matemáticas
escalares (`pow`, `sqrt`, trig, hiperbólicas, log, exp, root, norm, atan2).
`calc.erf` (P308): NaN input → Err; ±∞ → short-circuit ±1.0; ±0 → ±0.0.

**Política transversal cristalina** (ADR-0101):
- `eval`/layout/operators: **IEEE 754 puro** (paridade vanilla; ver `eval.md`).
- `stdlib` funções matemáticas: **rejeita NaN+Inf** (divergência consciente; ADR-0101).

**Sítios afectados** (11 directos em `calc.rs`): `calc_pow`/`calc_sqrt` via
`guard_float`; 13 funções via `trig_op`; `calc_atan2`/`calc_ln`/`calc_log`/
`calc_norm`/`calc_root` directos; `calc_erf` input + short-circuits; `calc_log`
base (`!is_finite()→Err`).

**Excepções não cobertas por ADR-0101** (adiadas, P309 §10.5/§10.6): Cat D
Float→Int saturating (`floor`/`ceil`/`round`/`trunc`/`quo`); Cat D color
constructors sem range check; backdoor `native_float("NaN")`. Catálogo completo:
`diagnostico-ieee754-passo-309.md`.

---

## Critérios de Verificação (calc)

```
// calc_abs / pow / sqrt
calc_abs([Int(-5)]) → Ok(Int(5));  calc_abs([Float(-3.14)]) → Ok(Float(3.14));  calc_abs([]) → Err
calc_pow([Int(2),Int(10)]) → Ok(Int(1024));  calc_pow([Int(2),Int(-1)]) → Err;  calc_pow([Float(4.0),Float(0.5)]) → Ok(Float(2.0))
calc_sqrt([Float(4.0)]) → Ok(Float(2.0));  calc_sqrt([Int(4)]) → Ok(Float(2.0));  calc_sqrt([Float(-1.0)]) → Err
// floor / ceil / round / min / max / clamp
calc_floor([Float(3.7)]) → Ok(Int(3));  calc_ceil([Float(3.2)]) → Ok(Int(4));  calc_round([Float(3.5)]) → Ok(Int(4));  calc_round([Float(3.4)]) → Ok(Int(3))
calc_min([Int(3),Int(1),Int(2)]) → Ok(Int(1));  calc_max([Int(3),Int(1),Int(2)]) → Ok(Int(3));  calc_min([]) → Err;  calc_max([]) → Err
calc_clamp([Int(5),Int(0),Int(10)]) → Ok(Int(5));  calc_clamp([Int(-5),Int(0),Int(10)]) → Ok(Int(0));  calc_clamp([Int(15),Int(0),Int(10)]) → Ok(Int(10));  calc_clamp([Float(5.0),Float(10.0),Float(0.0)]) → Err
// P283 trig (tolerância 1e-10)
calc_sin([Float(0.0)]) → Ok(Float(0.0));  calc_sin([Float(PI)]) → Ok(Float(~0.0));  calc_cos([Float(0.0)]) → Ok(Float(1.0));  calc_cos([Float(PI)]) → Ok(Float(-1.0))
calc_tan([Float(FRAC_PI_4)]) → Ok(Float(~1.0));  calc_asin([Float(1.0)]) → Ok(Float(FRAC_PI_2));  calc_asin([Float(1.5)]) → Err;  calc_acos([Float(-1.0)]) → Ok(Float(PI))
calc_atan([Float(1.0)]) → Ok(Float(FRAC_PI_4));  calc_atan2([Float(1.0),Float(1.0)]) → Ok(Float(FRAC_PI_4));  calc_atan2([Float(1.0)]) → Err
// P283 hiperbólicas / exp / log
calc_sinh([Float(0.0)]) → Ok(Float(0.0));  calc_cosh([Float(0.0)]) → Ok(Float(1.0));  calc_tanh([Float(0.0)]) → Ok(Float(0.0));  calc_acosh([Float(1.0)]) → Ok(Float(0.0));  calc_acosh([Float(0.5)]) → Err;  calc_atanh([Float(0.0)]) → Ok(Float(0.0));  calc_atanh([Float(1.0)]) → Err
calc_exp([Float(0.0)]) → Ok(Float(1.0));  calc_exp([Int(1)]) → Ok(Float(E));  calc_exp([Float(1e10)]) → Err;  calc_ln([Float(E)]) → Ok(Float(1.0));  calc_ln([Float(0.0)]) → Err
calc_log([Float(100.0)]) → Ok(Float(2.0));  calc_log([Float(8.0),Float(2.0)]) → Ok(Float(3.0));  calc_log([Float(10.0),Float(1.0)]) → Err;  calc_log([Float(10.0),Float(INF)]) → Err
make_calc_module().{pi,tau,e,inf} ≡ Float(consts.*)
// P306 trunc/fract/even/odd/rem/rem_euclid/div_euclid/quo
calc_trunc([Int(5)]) → Ok(Int(5));  calc_trunc([Float(3.7)]) → Ok(Int(3));  calc_trunc([Float(-3.7)]) → Ok(Int(-3));  calc_trunc([Str("x")]) → Err
calc_fract([Int(5)]) → Ok(Float(0.0));  calc_fract([Float(3.7)]) → Ok(Float(~0.7));  calc_fract([Float(-3.7)]) → Ok(Float(~-0.7))
calc_even([Int(4)]) → Ok(Bool(true));  calc_even([Int(-3)]) → Ok(Bool(false));  calc_even([Int(0)]) → Ok(Bool(true));  calc_even([Float(2.0)]) → Err;  calc_odd([Int(3)]) → Ok(Bool(true));  calc_odd([Int(0)]) → Ok(Bool(false))
calc_rem([Int(7),Int(3)]) → Ok(Int(1));  calc_rem([Int(-7),Int(3)]) → Ok(Int(-1));  calc_rem([Float(7.5),Float(2.0)]) → Ok(Float(~1.5));  calc_rem([Int(1),Int(0)]) → Err;  calc_rem([Int(5)]) → Err
calc_rem_euclid([Int(-7),Int(3)]) → Ok(Int(2));  calc_rem_euclid([Float(-7.5),Float(2.0)]) → Ok(Float(~0.5));  calc_rem_euclid([Int(1),Int(0)]) → Err
calc_div_euclid([Int(-7),Int(3)]) → Ok(Int(-3));  calc_div_euclid([Int(7),Int(3)]) → Ok(Int(2));  calc_div_euclid([Int(1),Int(0)]) → Err
calc_quo([Int(7),Int(2)]) → Ok(Int(3));  calc_quo([Int(-7),Int(2)]) → Ok(Int(-3));  calc_quo([Float(7.5),Float(2.0)]) → Ok(Int(3));  calc_quo([Int(1),Int(0)]) → Err
// P306 gcd/lcm/fact/perm/binom/norm/root
calc_gcd([Int(12),Int(18)]) → Ok(Int(6));  calc_gcd([Int(0),Int(5)]) → Ok(Int(5));  calc_gcd([Int(0),Int(0)]) → Ok(Int(0));  calc_gcd([Int(-12),Int(18)]) → Ok(Int(6))
calc_lcm([Int(4),Int(6)]) → Ok(Int(12));  calc_lcm([Int(0),Int(5)]) → Ok(Int(0));  calc_lcm([Int(i64::MAX),Int(2)]) → Err
calc_fact([Int(0)]) → Ok(Int(1));  calc_fact([Int(5)]) → Ok(Int(120));  calc_fact([Int(-1)]) → Err;  calc_fact([Int(21)]) → Err;  calc_fact([Float(5.0)]) → Err
calc_perm([Int(5),Int(2)]) → Ok(Int(20));  calc_perm([Int(5),Int(0)]) → Ok(Int(1));  calc_perm([Int(5),Int(8)]) → Ok(Int(0));  calc_perm([Int(-1),Int(2)]) → Err
calc_binom([Int(5),Int(2)]) → Ok(Int(10));  calc_binom([Int(5),Int(0)]) → Ok(Int(1));  calc_binom([Int(5),Int(5)]) → Ok(Int(1));  calc_binom([Int(5),Int(8)]) → Ok(Int(0));  calc_binom([Int(-1),Int(2)]) → Err;  calc_binom([Int(67),Int(33)]) → Ok(Int(_))
calc_norm([]) → Ok(Float(0.0));  calc_norm([Int(3),Int(4)]) → Ok(Float(5.0));  calc_norm([Int(3),Int(4)],p:Float(2.0)) → Ok(Float(5.0));  calc_norm([Int(1),Int(1),Int(1)],p:Float(1.0)) → Ok(Float(3.0));  calc_norm([Float(-3.0),Float(4.0)]) → Ok(Float(5.0));  calc_norm([Int(3)],p:Int(0)) → Err
calc_root([Int(2),Int(9)]) → Ok(Float(3.0));  calc_root([Int(3),Int(8)]) → Ok(Float(2.0));  calc_root([Int(3),Int(-8)]) → Ok(Float(-2.0));  calc_root([Int(2),Int(-1)]) → Err;  calc_root([Int(0),Int(5)]) → Err;  calc_root([Int(2),Float(0.0)]) → Ok(Float(0.0))
// P308 erf (tolerância 1.5e-7 perfil graded)
calc_erf([Float(0.0)]) → Ok(Float(0.0));  calc_erf([Int(0)]) → Ok(Float(0.0));  calc_erf([Float(1.0)]) → Ok(Float(~0.84270079));  calc_erf([Float(-1.0)]) → Ok(Float(~-0.84270079))
calc_erf([Float(0.5)]) → Ok(Float(~0.52049988));  calc_erf([Float(2.0)]) → Ok(Float(~0.99532227));  calc_erf([Float(5.0)]) → Ok(Float(~1.0));  calc_erf([Float(-5.0)]) → Ok(Float(~-1.0))
calc_erf([Float(INF)]) → Ok(Float(1.0));  calc_erf([Float(NEG_INF)]) → Ok(Float(-1.0));  calc_erf([Float(NAN)]) → Err;  calc_erf([]) → Err;  calc_erf([Float(1.0),Float(2.0)]) → Err;  calc_erf([Str("x")]) → Err;  calc_erf([Float(1.0)],named:{p:Float(0.5)}) → Err
```
