# Diagnóstico — Fase A do Passo 283 (`P-stdlib-calc-trig`)

**Data**: 2026-05-18
**Spec mãe**: `00_nucleo/materialization/typst-passo-283.md`
**Inventário-fonte**: `lab/typst-original/crates/typst-library/src/foundations/calc.rs`

---

## A.1 — Scope vanilla vs cristalino

Vanilla expõe 38 entradas no `Scope` do módulo `calc` (`module()` em
`calc.rs:15-63`): 34 funções + 4 constantes. A separação em buckets segue
a regra do passo §A.1 (puro `f64` no bucket 1; dependências de tipos
tipográficos ou scope distinto no bucket 2; já existente no bucket 3).

| Nome | Bucket | Justificação |
|------|:---:|---|
| `abs` | 3 | Já presente. Cristalino aceita Int/Float; vanilla estende a `Length/Angle/Ratio/Fraction/Decimal` — divergência registada (não regride). |
| `pow` | 3 | Já presente. Vanilla usa `libm::exp/exp2/pow` com fast-path para base = e/2 e cast a `Scalar` para expoente inteiro; cristalino usa `f64::powf` com `#[allow(clippy::disallowed_methods)]` mais `DEBT (ADR-0018)`. |
| `exp` | **1** | XS: `f64::exp(x)` + `guard_float`. Validações vanilla extra (expoente `is_normal()`) — omitir nesta sessão (pratica vanilla é checagem específica que não muda o resultado para inputs típicos). |
| `sqrt` | 3 | Já presente. Domínio negativo → `Err` (paridade vanilla). |
| `root` | **2** | Vanilla aceita raiz n-ésima com tratamento especial para negativo + n ímpar. Decisão lógica não-trivial; passo dedicado. |
| `sin/cos/tan` | **1** | XS. Vanilla aceita `AngleLike` (Int|Float|Angle); cristalino aceita só `Int|Float` em radianos (paridade vanilla `f64`). Conversão deg→rad fica adiada (sem tipo `Angle`); registado em §Não-objectivos do passo. |
| `asin/acos` | **1** | XS. Domínio `[-1, 1]`; fora → `Err` (paridade vanilla). Vanilla devolve `Angle::asin(x)` (radianos com tipo); cristalino devolve `Value::Float` em radianos directamente — sem `Angle`. |
| `atan` | **1** | XS. Domínio R; sem `Err` de domínio. Vanilla devolve `Angle::atan(x)`; cristalino devolve `Value::Float` em radianos. |
| `atan2` | **1** | XS. Vanilla: ordem `(x, y)` com `Angle::atan2(y, x)`. **Manter ordem `(x, y)`** (paridade) e chamar `f64::atan2(y, x)` internamente — divergência apenas no tipo de retorno (Float radianos vs Angle). |
| `sinh/cosh/tanh/asinh/acosh/atanh` | **1** | XS. Domínios: `sinh/cosh/tanh/asinh` em R; `acosh` em `[1, +∞)`; `atanh` em `(-1, 1)` (estrito). Erros explícitos para fora-de-domínio (paridade vanilla). |
| `log` | **1** | Logarítmo de base arbitrária. Vanilla usa argumento nomeado `base:` (default 10) e fast-paths para `e/2/10`. Cristalino vai aceitar **2 formas posicionais**: `log(x)` (base 10) ou `log(x, base)` — divergência justificada por simplicidade do parser de args actual (`expect_no_named`); registado em §Divergências abaixo. |
| `ln` | **1** | XS. Domínio `(0, +∞)`; fora → `Err`. |
| `erf` | **2** | Função especial. `f64::erf` não existe; só `libm::erf` ou crate dedicada. Adiar até decisão libm formal. |
| `fact/perm/binom` | **2** | Combinatória sobre inteiros não-negativos com checagem de overflow `u64`. Scope distinto (não-trig); passo dedicado. |
| `gcd/lcm` | **2** | Teoria de números sobre inteiros. Scope distinto. |
| `floor/ceil/round/clamp/min/max` | 3 | Já presentes. Vanilla `round` aceita `digits: i64` named arg + `Decimal/Float/Int`; cristalino só Int/Float sem `digits`. Divergência registada em §Divergências. |
| `trunc/fract` | **2** | Adjacentes a `floor/ceil`. Adiar para sub-passo de paridade `floor/ceil` para tratar `digits/Decimal` em conjunto. |
| `even/odd` | **2** | Predicados `i64`. Scope distinto. |
| `rem/div_euclid/rem_euclid/quo` | **2** | Divisão modular. Scope distinto; depende de decisão sobre `Decimal`. |
| `norm` | **2** | p-norm vetorial. Variádico + named arg. Scope dedicado. |
| `pi/tau/e/inf` (constantes) | **1** | Inclusão trivial via `dict.insert("pi".into(), Value::Float(std::f64::consts::PI))`. Bom complemento ergonómico ao bloco trig. |

### Resumo numérico

- **Bucket 1 (este passo)**: 16 funções + 4 constantes.
- **Bucket 2 (adiar)**: 16 funções (4 categorias temáticas: especiais,
  combinatória, int theory, divisão/scope-Decimal).
- **Bucket 3 (já existem)**: 9 funções com paridade f64; extensões a
  `Length/Angle/Decimal/digits` são adiadas (não regridem).

Cobertura `calc` projectada: 22% → 9+16=25 funções de 34 ≈ **74%**
(passo §1 anunciava ~70%; alvo cumprido).

---

## A.2 — Decisão libm vs f64::*

**Decidido**: opção **(a) com helper trig_op centralizado** (variante
prática da opção (c) do passo §A.2).

### Estado factual

- `clippy.toml:3-55` proíbe `f64::sin/cos/tan/sinh/cosh/asin/acos/atan/atan2/asinh/acosh/atanh/powf/exp/exp2/ln/log/log2/log10/cbrt` com a mensagem `"use libm::*"` (e simétrico para `f32::*`).
- `crystalline.toml:64-84` (`[l1_allowed_external]`) **não** inclui `libm`.
- Existe **divergência interna** entre os dois linters: clippy assume libm como destino; crystalline-lint não autoriza ainda.
- Precedente: `calc_pow` e `calc_sqrt` usam `#[allow(clippy::disallowed_methods)]` para `f64::powf` / `f64::sqrt`, com DEBT registado em ADR-0018.

### Razão da decisão

Promover `libm` a `[l1_allowed_external]` é decisão arquitectural que
requer ADR (per spec §7). Mitigação prescrita: sub-passo P283.1 dedicado
+ adiamento de P283. Foi descartada porque (i) o utilizador escolheu
materializar agora; (ii) o helper `trig_op` reduz o custo de migração
futura a 1 sítio (compromisso da opção (c) do passo); (iii) o helper
encaixa naturalmente em torno de `guard_float`, sem duplicar lógica.

### Concretização

```rust
/// Aplica uma função `f64 -> f64` da família matemática vanilla e
/// envolve o resultado em `guard_float`. Centraliza o
/// `#[allow(clippy::disallowed_methods)]` num único ponto para que a
/// migração futura a `libm` toque uma só assinatura (ADR-0018 §DEBT).
#[allow(clippy::disallowed_methods)]
fn trig_op(op: fn(f64) -> f64, x: f64) -> SourceResult<Value> {
    guard_float(op(x))
}
```

Cada `calc_<fn>` chama `trig_op(f64::<fn>, x)`. `atan2` (2-arg) e `log`
(2-arg) são casos especiais que não passam pelo helper — invocam
`f64::atan2` / `f64::ln` com `#[allow]` local. Mantemos a assimetria
honesta em vez de inflar `trig_op` para variadicidade.

### Follow-up

DEBT mantida em ADR-0018 §DEBT. Sub-passo P283.1 (promover `libm` +
migrar todos os sítios) continua disponível como follow-up natural —
agora migra ~3 sítios (`trig_op`, `f64::atan2`, `f64::ln`) em vez de
17. Volta-se a `calc_pow` e `calc_sqrt` no mesmo movimento.

---

## A.3 — Tratamento de domínio

**Decidido**: `Err` explícito para violação de domínio (paridade
vanilla); `guard_float` para NaN/Inf não previsto.

### Tabela

| Função | Domínio | Acção fora-de-domínio | Vanilla |
|--------|---------|----------------------|---------|
| `sin/cos/sinh/asinh/atan/erf` | R | nada (guard_float captura Inf raro) | idem |
| `tan` | R | guard_float (Inf perto de π/2+kπ) | sem guard explícito |
| `tanh` | R | nada | idem |
| `cosh/exp` | R | guard_float captura overflow → Inf | `bail!` se NaN |
| `asin/acos` | [-1, 1] | `Err "valor deve estar entre -1 e 1"` | `bail!` idem |
| `atan2` | R² | nada | idem |
| `acosh` | [1, +∞) | `Err "valor deve ser >= 1"` | `bail!` idem |
| `atanh` | (-1, 1) estrito | `Err "valor deve estar em (-1, 1)"` | `bail!` idem |
| `ln` | (0, +∞) | `Err "valor deve ser estritamente positivo"` | `bail!` idem |
| `log(x, base)` | x>0, base>0∧base≠1 | `Err` em cada constituinte | `bail!` |

### Divergência aceite vs vanilla

Cristalino usa `guard_float` (NaN/Inf → `Err`) onde vanilla por vezes
deixa Inf passar (`f64::tan(π/2)` em vanilla devolve um valor enorme
mas finito). Isto é **conservador** — paridade exacta no domínio válido,
endurecimento opcional na fronteira. Não introduz regressão visível
porque os inputs em causa são patológicos.

---

## §Divergências consolidadas (registar em L0)

1. **Sem tipo `Angle`**: trig recebe radianos `f64`; conversão deg/rad
   adiada (registado em §Não-objectivos do passo + bucket 2 desta tabela).
2. **`log(x, base)` posicional** em vez de `log(x, base: ...)` named.
   Justificado por simplicidade do parser; sub-passo futuro pode
   estender para named arg quando outros stdlib justificarem o esforço.
3. **`atan2(x, y)`**: ordem dos parâmetros preservada (paridade vanilla);
   chamada interna a `f64::atan2(y, x)` (ordem stdlib Rust).
4. **NaN/Inf → Err** via `guard_float` em todas as funções `f64`,
   ligeiramente mais conservador que vanilla.
5. **Constantes `pi/tau/e/inf`** adicionadas ao Dict; vanilla também
   tem mas o passo torna-as explícitas como complemento ergonómico.

---

## §Fecho da Fase A

Inventário completo + decisão arquitectural registada + tratamento de
domínio especificado. Material suficiente para materialização sem
ambiguidades. Procede-se a §3 do passo.
