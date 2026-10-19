# Paridade Produção — P713 — `Length / Length` (e `Length / Int|Float`)

**Data:** 2026-07-11
**Passo:** sem materialização prévia — instrução directa do utilizador
("avança para o Length / Length e corrija o bug"), sonda feita
directamente contra `lab/typst-original` per protocolo (ADR-0108).
**Hash do commit (implementação):** `9c47d6cb3`.
**HEAD base:** `757fa6623` (fim de P712).
**Estado:** FECHADO — `Length / Length`, `Length / Int`, `Length /
Float` implementados com paridade exacta ao `Length::try_div` do
vanilla, incluindo o gate de divisão por zero.

---

## 1. Sonda

### 1.1 Localização exacta do bug

`(BinOp::Div, Value::Length(_), Value::Length(_))` **não tinha braço**
em `eval_binary_op` (`01_core/src/engine/eval/operators.rs`) — caía no
fronteira genérico, produzindo `"cannot apply Div to length and
length"`. Isolado por P710/P711/P712 via `cetz`: `canvas.typ:37-38`
(`assert(length / 1cm != 0, ...)`, logo após `.to-absolute()`) e
`resolve-number` (`x / length`) — ambos `Length / Length`.

### 1.2 Mecanismo vanilla confirmado (`file:line`, não assumido)

`layout/length.rs:64-72` (`Length::try_div`):

```rust
pub fn try_div(self, other: Self) -> Option<f64> {
    if self.abs.is_zero() && other.abs.is_zero() {
        Some(self.em / other.em)
    } else if self.em.is_zero() && other.em.is_zero() {
        Some(self.abs / other.abs)
    } else {
        None  // incomensurável
    }
}
```

`foundations/ops.rs:289-342` (`div()`):
- Gate genérico `is_zero(&rhs)` **antes** do match, cobrindo **todos**
  os tipos numéricos, incluindo `Length` (`is_zero()`,
  `ops.rs:344-359`) — `x / 0cm` erra `"cannot divide by zero"` antes
  de chegar a `try_div`.
- `Length(a) / Int(b) => Length(a / b as f64)`, `Length(a) / Float(b)
  => Length(a / b)` — mesmo agrupamento de `Length / Length` no
  vanilla (linhas 312-314), não uma feature separada.
- `try_div_length` (linha 362-364): `a.try_div(b).ok_or_else(|| "cannot
  divide these two lengths".into())` — mensagem exacta para o caso
  incomensurável (`None`).

### 1.3 Scope-out medido — combinações `Ratio`/`Relative` mistas

Vanilla também define `Length/Relative` (`b.rel.is_zero()`),
`Relative/Length` (`a.rel.is_zero()`), `Ratio/Relative`,
`Relative/Ratio` (`ops.rs:315,324,328-330`). **Não implementadas
neste passo** — confirmado por grep exaustivo (`Value::Ratio(` em
`eval/`+`stdlib/`) que `Value::Ratio` **não é produzível por sintaxe
de utilizador** no cristalino actual: `50%` produz `Value::Relative`
(P705), e os únicos construtores de `Ratio` existentes
(`Ratio * Int`/`Int * Ratio`, `operators.rs:215-218`) exigem já ter um
`Ratio` — sem ponto de entrada a partir de código de utilizador. Sem
consumidor medido em `cetz` para estas combinações. Um bug por passo
(mesma disciplina de P711/P712) — se um passo futuro tornar `Ratio`
produzível, estas combinações devem ser revisitadas.

### Critério de fecho da sonda

- [x] Mecanismo do vanilla confirmado (`try_div`, gate de zero
      genérico, agrupamento com `Int`/`Float`).
- [x] Localização exacta do braço em falta, `file:line`.
- [x] Scope-out de `Ratio`/`Relative` mistos confirmado como
      não-alcançável, não assumido.

---

## 2. Implementação

**`01_core/src/engine/eval/operators.rs`**:

- Gate de divisão por zero (linha ~27-37): novo braço `Value::Length(l)
  if l.is_zero() => Err("cannot divide by zero")` — paridade com o
  gate genérico do vanilla, que já cobria `Int`/`Float`/`Decimal` no
  cristalino mas não `Length`.
- `(BinOp::Div, Value::Length(a), Value::Int(b))` /
  `(..., Value::Float(b))` → `Ok(Value::Length(a / b as f64))` / `Ok(Value::Length(a / b))`
  — reaproveita `impl Div<f64> for Length` já existente
  (`entities/layout_types.rs:808-813`), só faltava o braço de
  `eval_binary_op` para os alcançar a partir do eval.
- `(BinOp::Div, Value::Length(a), Value::Length(b))` → paridade exacta
  com `Length::try_div`: `abs` ambos zero → rácio de `em`; `em` ambos
  zero → rácio de `abs`; caso contrário `Err("cannot divide these two
  lengths")` (mesma mensagem do vanilla, sem hints — o vanilla expõe-na
  como string literal simples).

### Testes (7 novos, `eval/tests.rs`)

Mesma unidade abs (`2cm / 1cm`), rácio de `em` puro, caso
incomensurável (erro), divisor zero (erro "divide by zero"),
`Length / Int`, `Length / Float`, e reprodução exacta do cetz via
`eval_let` (`(2cm).to-absolute() / 1cm`).

---

## 3. Validação

Reprodução manual (release build):

```
#(2cm / 1cm)   → 2.0   (vanilla: 2 — divergência de formatação Float
                         pré-existente e não relacionada: #(4/2)
                         também dá "2.0" no cristalino, "2" no
                         vanilla; confirmado não ser regressão deste
                         passo)
#(5cm / 0cm)   → Err "cannot divide by zero"
```

- `cargo test --workspace` → **3809** (typst-core: 3802 + 7 novos) +
  **630** (typst-infra, inalterado) passed, 0 failed.
- `crystalline-lint .` → 0 violations; `--fix-hashes .` realinhou
  `operators.rs` (único ficheiro com `@prompt rules/eval/ops.md`).

### Reprodução `cetz`

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({ import cetz.draw: *; line((0,0),(2,1)); circle((0,0)) })
```

**Bloqueio anterior resolvido** — `Length / Length` deixou de ser a
causa. **Novo bloqueio**: `error: campo desconhecido em array: 'at'`
— `array.at(index, default: ...)` não implementado. Confirmado uso
real em `cetz` (`aabb.typ:43,45,75-77` — `bounds.high.at(2, default:
0)`, `padding.at("left", default: 0)`, etc.). Tempo de compilação
**~52.7s**, mesma ordem de grandeza dos passos anteriores, sem
regressão.

### Próximo passo sugerido (não iniciado)

`array.at(index, default: value)` — método de instância de `Array`
(paralelo a `.first()`/`.last()`/`.sum()` já implementados em P466,
`try_dispatch_collection_method`), com suporte a `default:` nomeado
para índice fora de alcance (vanilla: sem `default`, índice inválido
é erro; com `default`, devolve o valor por defeito).

---

## 4. Critério de fecho do passo

- [x] Sonda completa, mecanismo e causa exacta confirmados
      (`try_div`, gate de zero, agrupamento com Int/Float).
- [x] Corrigido, testado com múltiplos casos (mesma unidade, rácio de
      em, incomensurável, zero, Int, Float).
- [x] Sem regressão em `cargo test --workspace` (3809 vs 3802 antes).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — bloqueio de `Length / Length` resolvido,
      novo bloqueio identificado (`array.at()`), sem regressão de
      tempo.
- [x] Relatório com resultado exacto (este ficheiro), incluindo
      scope-out medido (`Ratio`/`Relative` mistos, não alcançáveis) e
      a divergência de formatação Float pré-existente (não desta
      correcção).
