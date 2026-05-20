# Diagnóstico — Passo 309 — Conformidade IEEE 754 em L1

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-309.md`
**Tipo**: diagnóstico-primeiro per ADR-0065 (sem código tocado, sem
ADR nova, sem L0 alterado, sem testes tocados)
**Escopo escolhido**: **α** — L1 completo (stdlib + eval + layout +
math + entities)
**Comparação vanilla escolhida**: **β** — obrigatória para todas as
categorias (sítio-a-sítio)
**Output**: este ficheiro
**Saída pós-P309**: 0 ficheiros alterados; 0 hashes propagados; 0
testes alterados

---

## §1 — Contexto e escopo

P308 (`calc.erf`) implementou três short-circuits, um dos quais foi
documentado como "divergência consciente":

- `erf(NaN) → Err` (cristalino) vs `libm::erf(NaN) = NaN` (vanilla).

A inspecção do código revelou que **a divergência não é local a `erf`**:
é política transversal cristalina expressa via helper central
`guard_float` em `stdlib/calc.rs:803-806`. Simultaneamente, o L0
`prompts/rules/eval.md` documenta política oposta:

> **Float → IEEE 754**: NaN e Inf propagados silenciosamente (sem
> guarda)

Cristalino tem portanto **duas políticas IEEE 754 contraditórias em
sítios diferentes do código**. P309 não decide; cataloga.

**Pré-requisitos para qualquer decisão futura**:
1. Tabela posicional completa dos sítios que rejeitam vs propagam.
2. Confronto sítio-a-sítio com vanilla typst.
3. Implicações por opção de política (cobertura, paridade, testes,
   migração DEBT-libm).

P309 produz exactamente isso. **Decisão fica adiada** para passo
subsequente.

---

## §2 — Política IEEE 754 declarada (textual)

### §2.1 — Em `eval` (propaga silenciosamente)

`00_nucleo/prompts/rules/eval.md` — sem citação directa de IEEE 754,
mas o comportamento documentado por `eval/operators.rs:14-20`:

```rust
//! Semântica confirmada com `lab/typst-original/.../foundations/ops.rs`:
//! - Int/Int → Float (não truncamento): `5/2 = 2.5`
//! - Int overflow → Err (checked_add/sub/mul/neg, como no original)
//! - Float: IEEE 754 propagado silenciosamente (sem guarda NaN/Inf)
//! - Divisão por zero → Err explícito
```

**Política declarada eval**: IEEE 754 puro para Float; divisão por
zero é caso especial **mas só para divisor literal `0`/`0.0`, não para
divisor que se torna zero por subnormal**.

### §2.2 — Em `stdlib` (rejeita via `guard_float`)

`00_nucleo/prompts/rules/stdlib.md` §"Helpers Internos":

> `guard_float(f)` — NaN → Err "não é um número", Inf → Err "infinito"

Implementação em `01_core/src/rules/stdlib/calc.rs:803-806`:

```rust
fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan()           { err("resultado não é um número (NaN)") }
    else if f.is_infinite() { err("resultado é infinito") }
    else                    { Ok(Value::Float(f)) }
}
```

**Política declarada stdlib**: rejeitar NaN E Inf no **resultado** de
funções matemáticas. Helper invocado em 9 sítios de `calc.rs` (ver
§3.1).

### §2.3 — Contradição declarativa

Em consequência:

- `5.0 / 0.0` no markup Typst (`#(5.0 / 0.0)`) → **Err** (`eval/
  operators.rs:28`).
- `5.0 / 0.5e-200` no markup Typst → **Inf propagado** (sem guarda
  em `operators.rs:60`).
- `calc.sqrt(5.0 / 0.5e-200)` → **Err via `guard_float`** porque
  passa por `sqrt` que invoca `guard_float`.
- `calc.exp(1000)` → **Err via `guard_float`** (overflow para Inf).
- `5e200 * 5e200` em markup → **Inf propagado** silenciosamente.

A política depende **do caminho sintáctico** que o utilizador escolheu
para chegar ao mesmo valor — não da identidade do valor. **Esta
inconsistência é o ponto central que P309 expõe**.

---

## §3 — Inventário factual L1 (grep posicional)

### §3.1 — Categoria A — Rejeita NaN/Inf (divergência IEEE 754)

Todos os sítios L1 que rejeitam NaN e/ou Inf no resultado ou input.

| # | Ficheiro:linha | Função | Mecanismo | Rejeita |
|---:|---|---|---|---|
| 1 | `01_core/src/rules/stdlib/calc.rs:132` | `calc_pow` (resultado Float) | `guard_float(b.powf(e))` | NaN+Inf |
| 2 | `calc.rs:146` | `calc_sqrt` | `guard_float(f.sqrt())` | NaN+Inf |
| 3 | `calc.rs:252` (helper `trig_op`) | `calc_sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`sinh`/`cosh`/`tanh`/`asinh`/`acosh`/`atanh`/`exp` (13 funções) | `guard_float(op(x))` | NaN+Inf |
| 4 | `calc.rs:327` | `calc_atan2` | `guard_float(r)` | NaN+Inf |
| 5 | `calc.rs:391` | `calc_ln` | `guard_float(r)` (após `x>0` validado) | NaN+Inf |
| 6 | `calc.rs:418` | `calc_log` | `guard_float(r)` (após `x>0` + base validados) | NaN+Inf |
| 7 | `calc.rs:695` | `calc_norm` | `guard_float(r)` | NaN+Inf |
| 8 | `calc.rs:719` | `calc_root` | `guard_float(r)` (após `index≠0` + ramo) | NaN+Inf |
| 9 | `calc.rs:747` | `calc_erf` (P308) | `x.is_nan() → err(...)` no input | NaN no input |
| 10 | `calc.rs:752` | `calc_erf` (P308) | `x.is_infinite() → short-circuit ±1.0` | Inf input convertido |
| 11 | `calc.rs:413` | `calc_log` (base) | `!base.is_finite() → err(...)` (base) | NaN+Inf input base |

**Totais**:
- **9 funções `calc.*`** usam `guard_float` no resultado (cobrem 24
  pontos de invocação somando trig + hiperbólicas + exp).
- **`calc_erf` explícito** no input (NaN+Inf + ±0).
- **`calc_log` explícito** na base (NaN+Inf/0/1).
- **0 sítios fora de `calc.rs`** rejeitam NaN/Inf em L1 produção.

Outras ocorrências de `is_nan`/`is_infinite`/`is_finite` em L1
inspeccionadas:
- `01_core/src/rules/layout/tests.rs:11125` — teste de layout
  (`min_body_y.is_finite()`); **não é produção** — filtrado.

**Conclusão Cat A**: o "guard NaN/Inf" cristalino está concentrado em
`stdlib/calc.rs`. Nenhum outro sítio L1 valida IEEE 754
explicitamente.

### §3.2 — Categoria B — Propaga IEEE 754 naturalmente

Operações que aceitam/produzem `f64` sem qualquer guarda. Cataloga
sítios estruturais (não cada chamada).

#### B.1 — `eval/operators.rs` (binárias e unárias sobre Float)

| # | Linha | Operação | Tipos | Guarda |
|---:|---:|---|---|---|
| 1 | `eval/operators.rs:37` | `Add` Float+Float | `Float(a + b)` | nenhuma |
| 2 | `eval/operators.rs:38-39` | `Add` Float↔Int | `Float(a + b as f64)` | nenhuma |
| 3 | `eval/operators.rs:47-49` | `Sub` Float (3 variantes) | `Float(a - b)` | nenhuma |
| 4 | `eval/operators.rs:54-56` | `Mul` Float (3 variantes) | `Float(a * b)` | nenhuma |
| 5 | `eval/operators.rs:59-62` | `Div` (4 variantes) | `Float(a / b)` | só `rhs==0` literal |
| 6 | `eval/operators.rs:75-90` | `Lt`/`Leq`/`Gt`/`Geq` Float (12 ramos) | `a < b` etc. | nenhuma — NaN comparisons sempre `false` |
| 7 | `eval/operators.rs:68-71` | `Eq`/`Neq` Float↔Int | coerção `as f64` | nenhuma — `NaN == NaN` é `false` |
| 8 | `eval/operators.rs:144` | `Neg` Float | `Float(-f)` | nenhuma (preserva sinal NaN) |
| 9 | `eval/operators.rs:147` | `Neg` Length | `Abs(-l.abs.to_pt())` | nenhuma |
| 10 | `eval/operators.rs:151` | `Pos` Float | `Float(f)` | nenhuma |
| 11 | `eval/operators.rs:101-104` | `Mul` Ratio↔Int | `r.get() * n as f64` | nenhuma |

**Confirmação**: `5.0 * f64::INFINITY` em markup compila e produz
`Float(Inf)` sem erro. `0.0 * f64::INFINITY` produz `Float(NaN)`. O
`Eq` posterior compara `NaN == NaN` que é `false` (semântica IEEE
754).

#### B.2 — `entities/layout_types.rs` (impls `std::ops::*`)

| # | Linha | Tipo | Operação | Guarda |
|---:|---:|---|---|---|
| 1 | `layout_types.rs:43-46` | `Pt` | `Add` | nenhuma |
| 2 | `layout_types.rs:48-51` | `Pt` | `Sub` | nenhuma |
| 3 | `layout_types.rs:53-56` | `Pt` | `Mul<f64>` | nenhuma |
| 4 | `layout_types.rs:58-60` | `Pt` | `AddAssign` | nenhuma |
| 5 | `layout_types.rs:583+` | `Abs` | `Add` (+ Sub/Mul/Div implícitos) | nenhuma |
| 6 | `layout_types.rs:622+` | `Length` | `Add` | nenhuma |

**Confirmação**: `Pt(1.0) + Pt(f64::NAN) = Pt(NaN)`. Coordenadas
podem ficar NaN/Inf silenciosamente e fluir para `FrameItem` →
`export.rs` → PDF.

#### B.3 — `rules/layout/**` (matemática de coordenadas)

Layout faz aritmética sobre `Pt`/`f64` extensiva em ~30 ficheiros
(grid, columns, boxes, alignment, cursor, etc.). **Nenhum sítio L1
produção** invoca `guard_float` ou validações IEEE 754. Todos
dependem da semântica de B.2 (propaga).

Sítios identificados por padrão (grep `\.to_pt()` + aritmética):
- `layout/grid.rs`, `layout/columns.rs`, `layout/cursor.rs`,
  `layout/figure.rs`, `layout/outline.rs`, `layout/boxed.rs`,
  `layout/block.rs`, `layout/cell.rs`, `layout/inline.rs`, etc.

**Risco real avaliado**: input do utilizador normalmente não introduz
NaN/Inf em layout porque tipos `Length`/`Ratio` são parsed com
unidades concretas. Mas via stdlib pode chegar lá se o utilizador
contornar guard_float (ex: `f64::INFINITY` literal não é parseable em
Typst — protege na entrada).

#### B.4 — `rules/math/**` (matemática de equações)

`math/layout/frac.rs`, `math/layout/root.rs`, `math/layout/matrix.rs`
etc. fazem aritmética sobre `Pt`/`Em` para posicionar
sub/super-scripts. Mesmo padrão de B.3: propaga IEEE 754, sem
guardas.

#### B.5 — `entities/color.rs` (componentes f32)

`Color::Srgb(f32, f32, f32, f32)`, `Color::Oklab(...)` etc. armazenam
componentes f32 sem validação de domínio. **Mas** os construtores em
foundations.rs ((P)257) só fazem `as f32` sem range check (ver §3.3).

**Total Categoria B**: estimadamente >200 sítios estruturais (impls
Add/Sub/Mul/Div + cada uso em layout). Distribuição contrasta com
**~11 sítios Cat A** concentrados em `calc.rs`. **Cat B é a norma; Cat
A é a excepção**.

### §3.3 — Categoria C — Validação por intervalo (não IEEE 754)

Sítios que validam domínio antes da operação, retornando Err em range
violation. **Não detecta NaN/Inf** explicitamente (mas algumas
comparações filtram-nos por efeito IEEE 754, ex: `x < 0.0` com `x =
NaN` é `false`).

| # | Ficheiro:linha | Função | Validação | Cobre NaN? |
|---:|---|---|---|---|
| 1 | `calc.rs:123` | `calc_pow` | `Int exp < 0 → Err` | n/a (Int) |
| 2 | `calc.rs:144` | `calc_sqrt` | `f < 0.0 → Err` | **não** — NaN→branch normal→`guard_float` apanha |
| 3 | `calc.rs:227` | `calc_clamp` | `lo > hi → Err` | **não** — NaN→branch normal |
| 4 | `calc.rs:290` | `calc_asin` | `x ∉ [-1,1] → Err` | **não** — NaN→branch normal→`guard_float` |
| 5 | `calc.rs:304` | `calc_acos` | `x ∉ [-1,1] → Err` | idem |
| 6 | `calc.rs:355` | `calc_acosh` | `x < 1.0 → Err` | idem |
| 7 | `calc.rs:369` | `calc_atanh` | `x ≤ -1.0 ∨ x ≥ 1.0 → Err` | idem |
| 8 | `calc.rs:387` | `calc_ln` | `x ≤ 0.0 → Err` | **não** — `NaN ≤ 0.0` é `false`; passa para `f64::ln`→NaN→`guard_float` apanha |
| 9 | `calc.rs:411` | `calc_log` (value) | `x ≤ 0.0 → Err` | idem |
| 10 | `calc.rs:413` | `calc_log` (base) | `!base.is_finite() ∨ base ≤ 0 ∨ base == 1 → Err` | **sim** — `!is_finite()` apanha NaN e Inf |
| 11 | `calc.rs:470/481` | `calc_rem` (divisor) | `b == 0 → Err` | apenas zero literal |
| 12 | `calc.rs:493/503` | `calc_rem_euclid` | idem | idem |
| 13 | `calc.rs:515/525` | `calc_div_euclid` | idem | idem |
| 14 | `calc.rs:537/547` | `calc_quo` | idem | idem |
| 15 | `calc.rs:603` | `calc_fact` | `n < 0 → Err` | n/a (Int) |
| 16 | `calc.rs:622` | `calc_perm` | `n < 0 ∨ k < 0 → Err` | n/a (Int) |
| 17 | `calc.rs:646` | `calc_binom` | idem | n/a (Int) |
| 18 | `calc.rs:670` | `calc_norm` (p named) | `Some(other) → Err` tipo | n/a (tipo) |
| 19 | `calc.rs:703` | `calc_root` | `index == 0 → Err` | n/a (Int) |
| 20 | `calc.rs:708` | `calc_root` | `xf < 0.0 ∧ index par → Err` | **não** |
| 21 | `foundations.rs:50-58` (helper `check`) | `native_rgb` | `(0..=255).contains(&v)` | n/a (Int) |
| 22 | `foundations.rs:83` | `native_luma` | `(0..=255).contains(l)` | n/a (Int) |
| 23 | `foundations.rs:253` | `native_range` | `n < 0 → Err` | n/a (Int) |
| 24 | `eval/operators.rs:25-30` | `eval_binary_op` (Div) | `rhs == 0 ∨ 0.0 → Err` | **não** — NaN→branch normal→Float NaN |

**Total Cat C**: 24 sítios. Quase todos em `calc.rs` (20). 3 em
`foundations.rs`. 1 em `eval/operators.rs`.

**Achado importante**: 8 dos 24 sítios Cat C delegam implicitamente o
caso NaN ao `guard_float` posterior (Cat A composta). Mas `calc_log
base` (linha 413) é único sítio Cat C que **trata NaN/Inf
explicitamente no input** via `is_finite()`.

### §3.4 — Categoria D — Conversões/coerções

Conversões entre tipos numéricos que **preservam ou alteram** o status
IEEE 754 do valor.

| # | Ficheiro:linha | Conversão | Comportamento NaN | Comportamento Inf |
|---:|---|---|---|---|
| 1 | `calc.rs:786-794` (helper `coerce_to_f64`) | `Int → f64`, `Float → f64` | preserva NaN | preserva Inf |
| 2 | `calc.rs:430` | `calc_trunc` (Float→Int via `as i64`) | **UB**: `NaN as i64 = 0` (Rust 1.45+ saturating) | **UB**: `Inf as i64 = i64::MIN` ou `i64::MAX` (saturating) |
| 3 | `calc.rs:165` (genérico) | `calc_floor` (Float→Int) | idem trunc | idem |
| 4 | `calc.rs:175` | `calc_ceil` (Float→Int) | idem | idem |
| 5 | `calc.rs:185` | `calc_round` (Float→Int) | idem | idem |
| 6 | `calc.rs:485` | `calc_quo` (Float→Int via `trunc as i64`) | idem | idem |
| 7 | `foundations.rs:99-100` (helper `as_f32` em `native_oklab`) | `Float→f32`, `Int→f32` | preserva NaN | preserva Inf |
| 8 | `foundations.rs:123-126` (helper `as_f32` em `native_oklch`) | idem | idem | idem |
| 9 | `foundations.rs:149-152` (`native_linear_rgb`) | idem | idem | idem |
| 10 | `foundations.rs:178-181` (`native_cmyk`) | idem | idem | idem |
| 11 | `foundations.rs:201-204` (`native_hsl`) | idem | idem | idem |
| 12 | `foundations.rs:227-230` (`native_hsv`) | idem | idem | idem |
| 13 | `foundations.rs:319` | `native_int(Float) → Err` | n/a — Float é rejeitado totalmente | n/a |
| 14 | `foundations.rs:340` | `native_float(Int) → Float` | n/a (Int não é NaN) | n/a |
| 15 | `foundations.rs:341-346` | `native_float(Str) → Float` | parsing `"NaN"`/`"nan"` produz NaN preservado | `"inf"`/`"infinity"` produz Inf preservado |
| 16 | `calc.rs:586` (interno `calc_norm`) | `*i as f64` posicional | n/a (Int) | n/a |

**Total Cat D**: 16 sítios.

**Achado crítico**: **`Float → Int as i64` em `calc_floor`/`calc_ceil`/
`calc_round`/`calc_trunc`/`calc_quo` (5 sítios)** silenciosamente
produz `0` para NaN ou `i64::MIN/MAX` para Inf. **Cristalino actual
não emite erro** nestes casos — depende da saturação implícita de
Rust 1.45+ (`f64 as i64` saturating semantics RFC 2484).

**Achado adicional**: `native_float("NaN")` aceita strings literais
"NaN"/"inf" e retorna `Value::Float(NaN/Inf)`. **Backdoor** para
introduzir NaN/Inf no eval pipeline.

---

## §4 — Comparação vanilla typst (`lab/typst-original/`)

### §4.1 — Política vanilla declarada

Vanilla **não tem documentação centralizada de política IEEE 754**.
Política é deduzida sítio-a-sítio de `crates/typst-library/src/
foundations/calc.rs` e `foundations/ops.rs`.

**Política implícita vanilla** (deduzida):
- **Operações aritméticas** (`ops.rs`): IEEE 754 puro (paridade com
  cristalino Cat B). Divisão por zero é caso especial via
  `is_zero(rhs)` (paridade).
- **`calc.pow`/`calc.exp`**: rejeita expoente **não-`is_normal()`** no
  input (NaN/Inf/subnormal) **+ rejeita NaN no resultado**. Inf no
  resultado é **deixado passar**.
- **`calc.sqrt`**: só valida `x ≥ 0` no input. Resultado IEEE 754
  puro (Inf/NaN propagam).
- **`calc.sin`/`cos`/`tan`**: **transparente IEEE 754** — retorna
  `f64` directo, sem guarda, sem `SourceResult`.
- **`calc.asin`/`acos`**: range no input, resultado não guarded.
- **`calc.atan`/`atan2`**: nenhuma guarda; transparente.
- **`calc.sinh`/`cosh`/`tanh`/`asinh`**: transparente (sem
  `SourceResult`).
- **`calc.acosh`/`atanh`**: range no input; resultado não guarded.
- **`calc.ln`**: `x > 0` + Inf result → Err. **NaN result não
  verificado**.
- **`calc.log`**: `x > 0` + base `!is_normal()` → Err + Inf||NaN
  result → Err.
- **`calc.erf`**: **`libm::erf` directo**, transparente
  (NaN/Inf passam).
- **`calc.root`**: índice 0 + raiz par negativa → Err; resultado não
  guarded.

### §4.2 — Sítios equivalentes (`lab/typst-original/.../calc.rs`)

| Vanilla:linha | Função | Política vanilla |
|---|---|---|
| `calc.rs:103-156` | `pow` | input is_normal; result NaN → Err; Inf passa |
| `calc.rs:164-185` | `exp` | input is_normal; result NaN → Err; Inf passa |
| `calc.rs:194-202` | `sqrt` | x ≥ 0; result IEEE 754 puro |
| `calc.rs:213-233` | `root` | index ≠ 0; raiz par; result IEEE 754 puro |
| `calc.rs:245-296` | `sin`/`cos`/`tan` | **transparente** (return `f64`, não `SourceResult`) |
| `calc.rs:305-332` | `asin`/`acos` | range; result não guarded |
| `calc.rs:341-364` | `atan`/`atan2` | transparente |
| `calc.rs:373-406` | `sinh`/`cosh`/`tanh` | transparente |
| `calc.rs:415-458` | `asinh`/`acosh`/`atanh` | acosh+atanh range; asinh transparente |
| `calc.rs:468-501` | `log` | x > 0 + base is_normal + result Inf+NaN → Err |
| `calc.rs:509-525` | `ln` | x > 0 + result Inf → Err (**NaN não verificado**) |
| `calc.rs:533-538` | `erf` | **`libm::erf` directo, transparente** |

### §4.3 — Divergências sítio-a-sítio (vanilla vs cristalino)

| Função | Cristalino | Vanilla | Divergência |
|---|---|---|---|
| `pow` | `guard_float` resultado (NaN+Inf→Err) | NaN→Err, Inf passa | **Cristalino mais restritivo** (rejeita Inf) |
| `sqrt` | `guard_float` resultado | resultado transparente | **Cristalino mais restritivo** (rejeita Inf+NaN) |
| `sin`/`cos`/`tan` | `guard_float` resultado | transparente | **Divergência total**: cristalino bloqueia, vanilla passa |
| `asin`/`acos` | range + `guard_float` resultado | range apenas | **Cristalino mais restritivo** |
| `atan` | `guard_float` resultado | transparente | **Divergência total** |
| `atan2` | `guard_float` resultado | transparente | **Divergência total** |
| `sinh`/`cosh`/`tanh` | `guard_float` resultado | transparente | **Divergência total** |
| `asinh` | `guard_float` resultado | transparente | **Divergência total** |
| `acosh`/`atanh` | range + `guard_float` | range apenas | **Cristalino mais restritivo** |
| `exp` | `guard_float` resultado | NaN→Err, Inf passa | **Cristalino mais restritivo** |
| `ln` | x>0 + `guard_float` | x>0 + Inf→Err (NaN não) | **Cristalino mais restritivo** (NaN tb) |
| `log` | x>0 + base + `guard_float` | x>0 + base + Inf+NaN→Err | **Paridade ≈ total** |
| `root` | índice + ramo + `guard_float` | índice + ramo apenas | **Cristalino mais restritivo** |
| `norm` | `guard_float` | (não existe em vanilla — específico cristalino P306) | n/a |
| `erf` | NaN→Err + ±∞→±1 + ±0→±0 + A&S poly | `libm::erf` directo | **Divergência total** (input + algoritmo) |

**Padrão vanilla**: rejeita NaN no resultado em **3 funções**
(`pow`/`exp`/`log`) — todas que envolvem expoentes ou logaritmos
onde NaN indica condição matemática indefinida. Trig e hiperbólicas
são deixadas transparentes.

**Padrão cristalino**: rejeita NaN+Inf no resultado em **9 funções**
+ NaN no input em 1 (erf). Bloqueia tudo o que vanilla deixa passar
em trig/hiperbólicas/atan/atan2.

**Operações binárias** (`eval/operators.rs` vs `ops.rs`): **paridade
total** — ambos propagam IEEE 754 com divisão por zero como caso
especial.

---

## §5 — Política DEBT-libm (ADR-0018)

### §5.1 — 7 sítios actuais `f64::*` (`#[allow(clippy::disallowed_methods)]`)

`grep -c "disallowed_methods" calc.rs` = **11 directivas** (algumas
em helpers reutilizados).

Sítios físicos:

| # | Linha | Função `f64::*` | Usado em |
|---:|---:|---|---|
| 1 | `calc.rs:131` | `f64::powf` | `calc_pow` |
| 2 | `calc.rs:250` (`trig_op`) | parâmetro `op: fn(f64) -> f64` | `calc_sin/cos/tan/asin/acos/atan/sinh/cosh/tanh/asinh/acosh/atanh/exp` (13 funções via `unary_f64`) |
| 3 | `calc.rs:325` | `f64::atan2` | `calc_atan2` |
| 4 | `calc.rs:389` | `f64::ln` | `calc_ln` |
| 5 | `calc.rs:416` | `f64::ln` (dupla) | `calc_log` (divisão de logs) |
| 6 | `calc.rs:689` | `f64::powf` | `calc_norm` |
| 7 | `calc.rs:711/715` | `f64::powf` (dois ramos) | `calc_root` |
| 8 | `calc.rs:771` | `f64::exp` | `calc_erf` helper `erf_approx_as` (P308) |

**Total funcional**: 7 sítios físicos com 8 marcas
`#[allow]`. **Sítios efectivos**: `calc_pow`/`trig+hyperbólicas+exp via
trig_op`/`atan2`/`ln`/`log`/`norm`/`root`/`erf-approx`.

Migração agregada futura para `libm::*` substitui:
- `f64::powf` → `libm::pow` (3 callsites).
- `f64::ln` → `libm::log` (2 callsites em ln+log).
- `f64::sin/cos/tan/asin/acos/atan/sinh/cosh/tanh/asinh/acosh/atanh/
  exp` → `libm::*` (13 funções via `trig_op`).
- `f64::atan2` → `libm::atan2`.
- `f64::exp` em `erf_approx_as` → eliminado se migrar para `libm::erf`
  directo.

### §5.2 — Como `libm` afecta a decisão IEEE 754

**Crucial**: `libm::*` tem **a mesma semântica IEEE 754 que `f64::*`**
— ambos produzem NaN/Inf nas mesmas condições. **Adoptar `libm` não
muda o problema NaN/Inf**.

A decisão IEEE 754 (Opções §7) é **ortogonal** à decisão libm
(ADR-0018). Podem ser tomadas independentemente.

**Mas** existe **uma intersecção significativa**: se a política IEEE
754 final for Opção A (cristalino mantém status quo restritivo), então
**`calc.erf` cristalino pode ser substituído por `libm::erf` directo**
sem perda funcional — porque `guard_float` cristalino apanharia o
NaN/Inf que `libm::erf` propaga. Cristalino não precisa do refino
A&S 7.1.26 que P308 implementou. **Possível regressão P308**.

---

## §6 — Inventário de mensagens de erro emitidas

Mensagens que sinalizam guarda IEEE 754 cristalino:

| Mensagem | Sítio | Contexto |
|---|---|---|
| `"resultado não é um número (NaN)"` | `calc.rs:804` (`guard_float`) | resultado de pow/sqrt/trig/log/exp/norm/root |
| `"resultado é infinito"` | `calc.rs:805` (`guard_float`) | idem |
| `"calc.erf() valor é NaN"` | `calc.rs:748` | input directo de `calc_erf` |
| `"calc.log() base inválida: {base}"` | `calc.rs:414` | base NaN/Inf/0/1 |
| `"cannot divide by zero"` | `eval/operators.rs:27,28` | divisor literal 0 ou 0.0 |

**Outras mensagens relacionadas (não IEEE 754 mas relevantes)**:
- `"calc.sqrt() argumento negativo"` (linha 144)
- `"calc.asin() valor deve estar entre -1 e 1, recebeu {x}"`
- `"calc.ln() valor deve ser estritamente positivo, recebeu {x}"`
- `"calc.fact() factorial de número negativo"`
- ...

**Consistência**: mensagens guard_float são genéricas
("resultado não é um número") sem indicar **qual operação** o
produziu — leitor humano precisa olhar a stack para descobrir. Refino
possível em sub-passo: `guard_float_at(f, op_name)`.

Inconsistência idiomática:
- `guard_float`: PT ("não é um número", "é infinito").
- `eval/operators.rs:27`: EN ("cannot divide by zero").
- Calc validations: PT misturado ("requer Int ou Float", "fora de
  0–255").
- `foundations.rs`: PT ("rgb(): componente {} fora de 0–255").

**Achado**: linguagem das mensagens **não é uniforme** — facto
ortogonal a IEEE 754 mas anotado por ser visível durante a auditoria.

---

## §7 — Opções de política IEEE 754 (sem decisão)

Quatro opções enunciadas. **P309 não escolhe**.

### §7.1 — Opção A — Conformidade IEEE 754 total (status quo divergente)

**Manter** `guard_float` em todos os sítios actuais Cat A. Documentar
politicamente que cristalino **rejeita** NaN/Inf em funções
matemáticas escalares como contrato consciente.

**Implicações**:
- Divergência vanilla **assumida** e documentada em ADR nova
  (`ADR-IEEE754-restricao-stdlib` ou similar).
- 12 testes pré-existentes verdes continuam verdes.
- Refino P308 (`erf` com A&S + short-circuit ±0) **mantém-se útil**:
  divergência consciente já reportada em P308 §10.3.
- Reverte zero código.
- Migração agregada libm (futuro passo M+) substitui `f64::*` por
  `libm::*` **mantendo** o wrapper `guard_float`.
- **Custo zero** de implementação. **Custo alto** de paridade.

### §7.2 — Opção B — Conformidade IEEE 754 total (alinhar com vanilla)

**Remover** `guard_float` dos sítios trig/hiperbólicas/atan2/exp/sqrt
(seguir vanilla transparente). **Manter** apenas em `pow`/`exp` (NaN
result) e `log` (Inf+NaN result) onde vanilla rejeita.

**Implicações**:
- Paridade vanilla restaurada para 9 funções (sin/cos/tan/atan/atan2/
  sinh/cosh/tanh/asinh).
- Reverter `erf`: remover NaN short-circuit; libm::erf substitui A&S
  inteiramente quando ADR-0018 fechar.
- Quebra dos testes que verificam `Err` para NaN/Inf result em trig.
  Reescrever ~5-8 testes.
- ADR nova obrigatória (`ADR-IEEE754-paridade-vanilla` ou anotação em
  ADR-0033 sobre paridade observable).
- Custo M+ de implementação (refactor sítio-a-sítio + revisão de
  testes).
- **Custo alto** de implementação. **Custo zero** de paridade.

### §7.3 — Opção C — Manter status quo (documentado)

Não mudar nada do código. **Apenas documentar** em ADR que cristalino
tem política dupla:
- `eval/operators.rs` propaga IEEE 754 (paridade vanilla).
- `stdlib/calc.rs` rejeita via `guard_float` (divergência consciente).

**Implicações**:
- Aceita a contradição como facto histórico (decisão tomada em P27/
  P283 sem ADR explícita; P306+P308 ampliaram sem questionar).
- Zero código tocado.
- Zero testes tocados.
- ADR nova **opcional** mas recomendada (codifica decisão tácita).
- **Bloqueia futuro debate** — reaberto só com ADR-revogação formal.
- **Custo mínimo**. **Compromisso epistémico**: documenta que
  "consistência cristalina total" não é objectivo.

### §7.4 — Opção D — Híbrido por categoria (granular)

Política diferenciada por **categoria de função**:
- **Aritmética eval** (Cat B operators): IEEE 754 puro (paridade
  vanilla).
- **Layout/math/coordenadas** (Cat B layout): IEEE 754 puro
  (operacional).
- **Funções matemáticas escalares** (Cat A stdlib): rejeita
  selectivamente:
  - `pow`/`exp`/`log` → NaN result Err (paridade vanilla);
  - `sqrt` → resultado transparente (paridade vanilla);
  - `sin`/`cos`/`tan`/`atan*`/`hyperbolicas` → transparente (paridade
    vanilla);
  - `erf` → libm directo (paridade vanilla).
- **Funções de conversão** (Cat D Float→Int): **adicionar guarda**
  para NaN/Inf produzir Err em vez de saturating-cast silencioso.
  Esta é a única **adição** (não remoção) de guarda.
- **Constructors de cor** (Cat D `as_f32`): **adicionar guarda** opcional
  ou range check `[0.0, 1.0]` no `oklab/oklch/cmyk/hsl/hsv`.

**Implicações**:
- Paridade vanilla onde vanilla decidiu (sin/cos/sqrt/...).
- Reforço onde vanilla é potencialmente unsafe (Float→Int saturating).
- Custo M+ implementação (mais cirúrgico que Opção B, mais ambicioso
  que Opção A).
- ADR nova obrigatória (clarifica política por categoria).
- Permite passo futuro fechar DEBT-libm com `libm::erf` directo (com
  ou sem NaN guard, consistente com vanilla).

---

## §8 — Implicações por opção (resumo)

| Critério | Opção A | Opção B | Opção C | Opção D |
|---|---|---|---|---|
| Paridade vanilla `sin/cos/tan/...` | ✗ divergente | ✓ paridade | ✗ divergente | ✓ paridade |
| Paridade vanilla `erf` | ✗ divergente | ✓ paridade | ✗ divergente | ✓ paridade |
| Cobertura cobre 41/41 calc | ✓ | ✓ | ✓ | ✓ |
| Testes pré-existentes (12) | ✓ inalterados | ✗ refactor 5-8 | ✓ inalterados | ✗ refactor 3-6 |
| Custo implementação | 0 | M+ | 0 | M+ |
| ADR nova | recomendada | obrigatória | recomendada | obrigatória |
| Reverte P308 (erf A&S) | ✗ | ✓ (libm::erf substitui) | ✗ | ✓ |
| Float→Int NaN/Inf seguro | ✗ silent saturating | ✗ silent | ✗ silent | ✓ Err explícito |
| Cor `oklab/...` range check | ✗ ausente | ✗ ausente | ✗ ausente | ✓ presente |
| DEBT-libm desbloqueio | ortogonal | ortogonal | ortogonal | facilita (`libm::erf` directo) |
| Coerência interna stdlib | uniforme (rejeita) | uniforme (passa) | dupla declarada | granular declarada |
| Risco descoberta tardia | baixo | médio (sítios escondidos) | baixo | médio (categorias mal classificadas) |

---

## §9 — Recomendação operacional (per ADR-0065)

P309 é diagnóstico-primeiro; §9 é **recomendação**, não decisão.

**Recomendação primária**: **Opção D — Híbrido por categoria**.

**Racional**:

1. **Paridade vanilla onde existe**: trig/hiperbólicas/erf transparentes
   alinham cristalino com vanilla bit-equivalente (no que `libm` permite),
   removendo divergências detectadas em §4.3. Reduz "cristalino é mais
   restritivo" para zero em 9 funções.

2. **Reforça onde vanilla é unsafe**: Float→Int em `floor/ceil/round/
   trunc/quo` actualmente saturam silenciosamente para `0` (NaN) ou
   `i64::MIN/MAX` (Inf). Adicionar guarda nestes 5 sítios é correcção
   **legítima** que vanilla **também deveria ter** mas tem por design
   (paridade `libm` directa).

3. **Adiciona range check em cores**: `oklab/oklch/cmyk/hsl/hsv`
   aceitam f32 sem range check `[0.0, 1.0]`. Vanilla usa
   `RatioComponent`/`ChromaComponent` tipados. Cristalino pode
   adicionar validação leve sem tipos novos.

4. **DEBT-libm desbloqueio**: com `erf` transparente, migração
   futura para `libm::erf` é trivial (remove `erf_approx_as` helper +
   short-circuits; chama `libm::erf` directo). P308 fica como **passo
   transitório** valioso até libm ser adicionado.

5. **ADR nova documenta política granular** — não é "consistência
   cristalina total" mas **classificação granular consciente** com
   racional sítio-a-sítio.

**Magnitude da implementação Opção D**: estimada **M+** (3-5 passos
agregados ou 1 passo grande):

| Sub-passo | Magnitude | Conteúdo |
|---|---|---|
| D.1 — Remover `guard_float` em trig/hyp/atan2/sqrt/exp_transparente | S | 8-10 sítios cirúrgicos |
| D.2 — Reverter `calc_erf` para libm-style (ou aguardar DEBT-libm) | XS | trivial pós-D.1 |
| D.3 — Adicionar guarda Float→Int em conversões | S | 5 sítios `as i64` |
| D.4 — Adicionar range check `[0.0, 1.0]` em `oklab/...` | S | 5 sítios `as f32` |
| D.5 — ADR-IEEE754-categorial nova | XS | documentação |

Granularidade aceitável para 1-2 passos M.

**Recomendação secundária**: se Opção D for considerada ambiciosa
demais, **Opção C — Status quo documentado** é a alternativa mais
barata. Codifica a decisão tácita em ADR e remove a contradição
declarativa entre `eval.md` e `stdlib.md` por **clarificação**, não por
mudança de código.

**Não recomendado**: Opção A (manter status quo sem ADR — perpetua
contradição). Opção B (refactor completo sem reforço Float→Int —
desperdício de oportunidade de reforço pontual).

---

## §10 — Pendências e adiamentos explícitos

Decisões adiadas para passos subsequentes (não objectivo de P309):

1. **Adoptar Opção A/B/C/D** (decisão política).
2. **Reverter ou manter `calc_erf` NaN→Err** (subsidiária de 1).
3. **Migração agregada libm** (ADR-0018) — ortogonal mas pode informar
   1.
4. **ADR-IEEE754-conformidade** ou **ADR-IEEE754-categorial** ou
   **anotação em ADR-0033** — depende de 1.
5. **Range check em `oklab/...`** — pode ser separado da decisão IEEE
   754 (sub-padrão "reforço pontual").
6. **Float→Int guard em `floor/ceil/round/trunc/quo`** — idem.
7. **Uniformização linguística mensagens de erro** (PT vs EN) —
   ortogonal a IEEE 754 mas anotado.

---

## §11 — Invariantes preservados

P309 é diagnóstico-primeiro; **zero código tocado**.

| Invariante | Estado |
|---|---|
| `entities/content.rs` hash | `82d3c47d` inalterado (**26º consecutivo**) |
| `export/*` snapshots | inalterados (**2º consecutivo pós-P307**) |
| Cobertura calc 41/41 | inalterada |
| ADRs meta novas | **0** (**16ª vez consecutiva** anti-padrão P273.17 §0) |
| `crystalline-lint .` | continua zero violations (sem alterações para o linter ver) |
| Testes | inalterados — 2 409 verdes pós-P308 |

---

## §12 — Sub-padrão observado

**"Auditoria de política transversal pós-divergência local isolada"** —
primeira ocorrência (per spec §11):

- P308 implementou divergência local (`erf(NaN) → Err`).
- Relatório P308 §10.3 reconheceu divergência consciente.
- Pergunta humana subsequente expôs que divergência é **sistémica**
  (`guard_float`), não local.
- P309 emerge como diagnóstico transversal.

**Candidato a observação cumulativa futura se padrão repetir (N≥3)**.

Sub-padrão potencial relacionado: **"Diagnóstico-primeiro factual
amplo após divergência declarativa detectada"** — paralelo a P156B
(Layout), P154A (Model), P307a (export). P309 é o **4º caso**
cristalino (N=4 cumulativo). Adiamento de promoção formal natural
(threshold tentativo N=3 atingido mas pattern muito específico —
"divergência declarativa" é critério estrito).

---

## §13 — Fecho

P309 fechado com:

- **1 ficheiro** publicado: este diagnóstico (`diagnostico-ieee754-
  passo-309.md`).
- **Tabelas §3.1-§3.4**: 11 + ~16 + 24 + 16 = **67 sítios catalogados
  posicionalmente** (ficheiro:linha:função).
- **§4** com comparação vanilla **obrigatória todas categorias**:
  política vanilla deduzida sítio-a-sítio em 12+ funções
  `calc.*`; binárias vanilla `ops.rs` confirmadas paridade total Cat
  B; `oklab/...` vanilla usa tipos validados (RatioComponent/
  ChromaComponent), cristalino `as f32` directo.
- **§5** com inventário DEBT-libm: 7 sítios físicos, 8 marcas
  `#[allow]`, intersecção crítica com decisão IEEE 754 identificada
  (cristalino `erf` poderia ser libm-directo se Opção D adoptada).
- **§6** com inventário de mensagens de erro: 5 mensagens guard_float
  + ~20 mensagens range validation + inconsistência linguística PT/EN
  anotada.
- **§7** com **4 opções** enunciadas (A/B/C/D) sem decisão.
- **§8** com tabela comparativa 12 critérios × 4 opções.
- **§9** com recomendação operacional **explícita** (Opção D primária,
  Opção C secundária; A/B não recomendadas).

**Conclusão substantiva**:

Cristalino tem política IEEE 754 **dupla e não-uniforme**: eval/layout
propaga (paridade vanilla), stdlib rejeita (divergência vanilla em 9
funções). A divergência foi introduzida em P27/P283 sem ADR explícita
e ampliada em P306/P308 sem questionar. P309 expõe o terreno
factualmente; **a decisão é do operador humano**.

**Recomendação operacional**: **Opção D — Híbrido por categoria** com
3-5 sub-passos S cada. Restaura paridade vanilla em 9 funções
escalares + reforça pontualmente Float→Int e cor range checks (onde
vanilla é igualmente unsafe mas por delegação a libm). Magnitude
total M+ aceitável.

**Próxima acção**: humano lê P309, decide A/B/C/D (ou variante), e
arranca passo material (P310+) que materializa a opção escolhida.
P309 não cria L0 nem ADR nem código — fica em standby até decisão.
