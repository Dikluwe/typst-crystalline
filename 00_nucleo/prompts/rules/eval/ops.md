# Prompt L0 — `rules/eval/operators`
Hash do Código: 05178d5e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/operators.rs`
**Criado em**: 2026-06-25 (P469)
**Atualizado em**: 2026-06-25
**ADRs relevantes**: ADR-0029 (`Length`), ADR-0117 Cláusula 4

---

## Contexto

`operators.rs` implementa `eval_binary_op` e `eval_unary_op`, o dispatcher de
operadores do eval Typst. P469 adiciona semântica para comprimentos relativos
(`Rel<Length>` / `Value::Relative`).

---

## Decisão — `%` como relativo, operações aritméticas com `Relative`

### Literal percentual

No eval de `Expr::Numeric` com `Unit::Percent`, o valor produzido é
`Value::Relative(Rel::from_percent(value))` em vez de `Value::Ratio`.

```rust
Unit::Percent => Ok(Value::Relative(Rel::from_percent(value)))
```

Isto reflete que, no subset actual, percentuais são comprimentos relativos.
`Value::Ratio` continua existindo como tipo L1 mas deixa de ser produzido pelo
literal `50%`.

### Operações binárias

```rust
// Relative + Relative, Relative - Relative
(Value::Relative(a), Value::Relative(b)) => Value::Relative(a + b) // / - b

// Relative + Length / Length + Relative
(Value::Relative(r), Value::Length(l)) |
(Value::Length(l), Value::Relative(r)) => Value::Relative(r + l)

// Relative - Length
(Value::Relative(r), Value::Length(l)) => Value::Relative(r - l)

// Relative * Int / Int * Relative
(Value::Relative(r), Value::Int(n)) |
(Value::Int(n), Value::Relative(r)) => Value::Relative(r * n as f64)

// Relative * Float / Float * Relative
(Value::Relative(r), Value::Float(f)) |
(Value::Float(f), Value::Relative(r)) => Value::Relative(r * f)

// Relative / Int, Relative / Float
(Value::Relative(r), Value::Int(n)) => Value::Relative(r / n as f64)
(Value::Relative(r), Value::Float(f)) => Value::Relative(r / f)
```

### Operação unária

```rust
(UnOp::Neg, Value::Relative(r)) => Value::Relative(-r)
```

---

## Fronteiras

- `%` como operador binário de remainder (`5 % 2`) não existe no parser
  actual; se for introduzido, deve continuar a produzir `Int` quando ambos os
  operandos forem `Int`.
- Comparações de `Relative` (`<`, `>`) permanecem sem suporte — requerem
  contexto de layout.
- Cast implícito `Relative → Length` em consumers deve usar `cast_length` e
  propagar `CastError::NeedsContext` quando não houver contexto.

---

## P706 — Operador `in` / `not in`

`BinOp::In`/`BinOp::NotIn` **já existiam no parser** (`entities/ast/expr.rs`)
e chegavam a `eval_binary_op`, mas caíam sempre no braço de fronteira
genérico (`"cannot apply {op:?} to {a} e {b}"`) — nenhuma combinação de
valores estava implementada. Isolado por P705/P706 via `cetz`
(`mark.typ:75,174,200` — `"chave" in dict`; `mark.typ:81,118`,
`draw/projection.typ:187` — `str in (tuple de strings)`).

### Combinações confirmadas contra o vanilla (`file:line` = medição directa)

| `lhs in rhs` | Resultado | Notas |
|---|---|---|
| `Str` in `Dict` | `Bool` — testa se a chave existe | `"a" in (a:1,b:2)` → `true` |
| `Str` in `Str` | `Bool` — testa substring | `"ell" in "hello"` → `true` |
| `any` in `Array` | `Bool` — testa igualdade de elemento (`PartialEq` de `Value`, recursivo — cobre array-de-arrays, `(1,2) in ((1,2),(3,4))` → `true`, e tipos mistos, `none in (none, 0%)` → `true`) | `matrix.typ:252` (`out in _ident`, array de matrizes) |
| `Int` in `Str` | **Erro**: `"cannot apply 'in' to integer and string"` (vanilla) | tipos incompatíveis — medido, não assumido |
| `not in` | Negação lógica das combinações acima | `1 not in (1,2,3)` → `false` |

### Semântica de implementação

- `Str in Dict` → `dict.contains_key(s.as_str())` (`IndexMap`, já com
  `Borrow<str>` via `EcoString`).
- `Str in Str` → `haystack.as_str().contains(needle.as_str())`.
- `any in Array` → `arr.contains(&needle)` — usa `PartialEq` já derivado em
  `Value`, cobre recursivamente arrays aninhados sem código extra.
- Combinação sem braço específico → cai no fronteira genérico já existente
  (mensagem `"cannot apply {op:?} to {a} e {b}"`) — **divergência de
  mecânica aceite** (mensagem, não o "é erro"): o texto exacto diverge do
  vanilla (`"cannot apply 'in' to integer and string"`), mas a
  observável "isto é um erro de tipo" é preservada. Mesmo padrão já
  aceite noutros operadores deste ficheiro (ADR-0107, mecânica diverge de
  propósito).
- `NotIn` implementado via o mesmo cálculo de `In`, negado — evita
  duplicar as 3 combinações.

---

## P713 — `Length / Length` (e `Length / Int|Float`)

`(BinOp::Div, Value::Length(_), Value::Length(_))` não tinha braço —
caía no fronteira genérico (`"cannot apply Div to length and length"`).
Isolado por P710/P711/P712 via `cetz` (`canvas.typ:37-38`:
`assert(length / 1cm != 0, ...)`, e `resolve-number`: `x / length`,
ambos `Length / Length` — `length` já resolvido por `.to-absolute()`,
`em: 0.0`).

### Mecanismo do vanilla (`layout/length.rs:64-72`, `foundations/ops.rs:289-342`)

```rust
pub fn try_div(self, other: Self) -> Option<f64> {
    if self.abs.is_zero() && other.abs.is_zero() {
        Some(self.em / other.em)
    } else if self.em.is_zero() && other.em.is_zero() {
        Some(self.abs / other.abs)
    } else {
        None  // incomensurável — erro no caller, não silencioso
    }
}
```

`div()` (vanilla) verifica `is_zero(&rhs)` **antes** do match, para
**todos** os tipos numéricos, incluindo `Length` — `x / 0cm` erra
"cannot divide by zero" antes de chegar a `try_div`. `Length(a) /
Int(b)` e `Length(a) / Float(b)` são o mesmo agrupamento no vanilla
(`ops.rs:312-314`) — escala uniforme, já implementado no cristalino
via `Length: Div<f64>` (`entities/layout_types.rs:808-813`), só faltava
o braço em `eval_binary_op` para os alcançar a partir do eval.

### Scope-out medido, não assumido — `Value::Ratio`/`Value::Relative` mistos

Vanilla também define `Length/Relative`, `Relative/Length`,
`Ratio/Relative`, `Relative/Ratio` (`ops.rs:315,324,328-330`). **Não
implementados aqui** — `Value::Ratio` não é actualmente produzível por
sintaxe de utilizador no cristalino (`50%` produz `Value::Relative`,
não `Value::Ratio`; `grep` confirma que os únicos construtores de
`Value::Ratio` em `eval`/`stdlib` são `Ratio * Int`/`Int * Ratio`, que
exigem já ter um `Ratio` — circular, sem ponto de entrada). Sem
consumidor medido em `cetz` para estas combinações; um bug por passo
(mesma disciplina de P711/P712).

### Semântica de implementação

```rust
// Div por zero — Length(l) se l.is_zero(), mesmo gate genérico já
// usado por Int(0)/Float(0.0)/Decimal(0).
Value::Length(l) if l.is_zero() => Err("cannot divide by zero")

(BinOp::Div, Value::Length(a), Value::Int(b))    => Ok(Value::Length(a / b as f64))
(BinOp::Div, Value::Length(a), Value::Float(b))  => Ok(Value::Length(a / b))
(BinOp::Div, Value::Length(a), Value::Length(b)) => {
    if a.abs.is_zero() && b.abs.is_zero() { Ok(Value::Float(a.em / b.em)) }
    else if a.em == 0.0 && b.em == 0.0 { Ok(Value::Float(a.abs.to_pt() / b.abs.to_pt())) }
    else { Err("cannot divide these two lengths") }
}
```

Mensagem de erro (`"cannot divide these two lengths"`) segue a
convenção já usada em `try_div_length` do vanilla (mesmo texto),
disponível porque o vanilla o expõe como string literal simples (não
há `hint`/formatação especial a replicar aqui, ao contrário de outros
casos já divergentes por design neste ficheiro).

---

## Critérios de Verificação

```rust
eval("#let x = 50%")        → Value::Relative(Rel::from_percent(50.0))
eval("#let x = 100% - 1em") → Value::Relative(Rel::from_percent(100.0) - Length::em(1.0))
eval("#let x = 50% + 2cm")  → Value::Relative(Rel::from_percent(50.0) + Length::cm(2.0))
eval("#let x = 50% * 2")    → Value::Relative(Rel::from_percent(100.0))

eval_binary_op(Add, Relative(50%), Length(2cm))
    == Relative(50% + 2cm)

eval_unary_op(Neg, Relative(50%)) == Relative(-50%)

// P706 — in / not in
eval_binary_op(In, Str("a"), Dict{a:1,b:2})     == Bool(true)
eval_binary_op(In, Str("z"), Dict{a:1,b:2})     == Bool(false)
eval_binary_op(In, Str("ell"), Str("hello"))    == Bool(true)
eval_binary_op(In, Int(1), Array[1,2,3])        == Bool(true)
eval_binary_op(In, Array[1,2], Array[Array[1,2],Array[3,4]]) == Bool(true)
eval_binary_op(NotIn, Int(5), Array[1,2,3])     == Bool(true)
eval_binary_op(In, Int(1), Str("hello"))        == Err (tipos incompatíveis)

// P713 — Length / Length, Length / Int|Float
eval_binary_op(Div, Length(2cm), Length(1cm))   == Float(2.0)
eval_binary_op(Div, Length(0em+4em-part), Length(0em+2em-part)) == Float(2.0)  // rácio de em
eval_binary_op(Div, Length(10pt+1em), Length(5pt)) == Err (incomensurável)
eval_binary_op(Div, Length(10pt), Length(0pt))  == Err ("cannot divide by zero")
eval_binary_op(Div, Length(10pt+4em), Int(2))   == Length(5pt+2em)
eval_binary_op(Div, Length(10pt), Float(4.0))   == Length(2.5pt)
```

---

## Resultado Esperado

- `operators.rs` com braços para `Value::Relative` em `eval_binary_op` e
  `eval_unary_op`.
- `eval/mod.rs` mapeando `Unit::Percent` para `Value::Relative`.
- Testes E2E e unitários cobrindo as expressões acima.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | Criação — P469 operadores para `Rel<Length>` | `operators.rs`, `eval/mod.rs`, `tests.rs` |
| 2026-07-11 | P706 — `in`/`not in` para `Str`/`Dict`/`Array` (isolado via `cetz`) | `operators.rs`, `tests.rs` |
| 2026-07-11 | P713 — `Length / Length` (paridade `try_div`), `Length / Int\|Float`; scope-out `Ratio`/`Relative` mistos (não produzíveis por sintaxe de utilizador) | `operators.rs`, `tests.rs` |
