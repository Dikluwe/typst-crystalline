# Prompt L0 — `rules/eval/operators`
Hash do Código: 3450b13f

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
