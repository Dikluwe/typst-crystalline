# Prompt L0 — `rules/eval/operators`
Hash do Código: adccec9b

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

## Critérios de Verificação

```rust
eval("#let x = 50%")        → Value::Relative(Rel::from_percent(50.0))
eval("#let x = 100% - 1em") → Value::Relative(Rel::from_percent(100.0) - Length::em(1.0))
eval("#let x = 50% + 2cm")  → Value::Relative(Rel::from_percent(50.0) + Length::cm(2.0))
eval("#let x = 50% * 2")    → Value::Relative(Rel::from_percent(100.0))

eval_binary_op(Add, Relative(50%), Length(2cm))
    == Relative(50% + 2cm)

eval_unary_op(Neg, Relative(50%)) == Relative(-50%)
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
