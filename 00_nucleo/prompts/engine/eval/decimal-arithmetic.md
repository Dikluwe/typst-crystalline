# Prompt L0 — `rules/eval/decimal-arithmetic` — operadores `Decimal`
Hash do Código: 1e867e48

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/eval/operators.rs`
**Origem**: Passo 404 (`typst-passo-404.md`) — ativar aritmética e comparações para `Value::Decimal`.
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (medir-antes-de-decidir), ADR-0025 (dois sistemas de igualdade).
**Prompt pai**: `00_nucleo/prompts/engine/eval.md`

---

## 1. Contexto

P399 modelou `Decimal` em L1; P403 ativou o constructor `decimal("1.23")`. Este passo adiciona `+`, `-`, `*`, `/` e comparações (`==`, `!=`, `<`, `<=`, `>`, `>=`) entre dois `Value::Decimal`, homogeneamente.

---

## 2. Escopo

- **Aritmética homogénea**: `Decimal op Decimal → Decimal` para `+`, `-`, `*`.
- **Divisão**: `Decimal / Decimal → Decimal`, fallible por divisão por zero.
- **Comparações**: `Decimal cmp Decimal → Bool` para `==`, `!=`, `<`, `<=`, `>`, `>=`.
- **Sem coerção cruzada**: misturas `Decimal + Int`, `Decimal + Float`, etc. continuam a cair no braço de erro existente.
- **Sem tipo/variant novo**: reutiliza `Value::Decimal` e `entities::decimal::Decimal`.

---

## 3. Implementação

No dispatch centralizado `eval_binary_op` em `01_core/src/engine/eval/operators.rs`, adicionar braços `Value::Decimal` junto aos braços `Int`/`Float` existentes.

### Aritmética

```rust
(BinOp::Add, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a + b)),
(BinOp::Sub, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a - b)),
(BinOp::Mul, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(a * b)),
```

`Decimal` já implementa `Add`/`Sub`/`Mul` via o `InnerDecimal` da crate `rust_decimal`. O wrapper `entities::decimal::Decimal` precisa expor as operações (acesso ao `.0` interno ou via métodos). Se o wrapper não implementar os traits de operadores, aceder a `a.0 op b.0` e rewrap em `Decimal(...)`.

### Divisão

```rust
(BinOp::Div, Value::Decimal(a), Value::Decimal(b)) => {
    if b.is_zero() {
        Err("cannot divide by zero".into())
    } else {
        Ok(Value::Decimal(a / b))
    }
}
```

A verificação de divisor zero pode unir-se à verificação prévia já existente para `Int`/`Float` (linha ~25 do ficheiro) ou ficar num braço dedicado. A mensagem deve ser `"cannot divide by zero"` para paridade.

### Comparações

Os braços `(BinOp::Eq, a, b)` e `(BinOp::Neq, a, b)` genéricos no fim do bloco já resolvem `==`/`!=` para qualquer `Value` via `derive(PartialEq)`. Como `Decimal` implementa `Eq`/`PartialEq` e `Value::Decimal` usa o wrapper, isso funciona automaticamente **desde que** não haja braço mais específico antes que capture `Value::Decimal` (não há). Confirmar que `decimal("1.0") == decimal("1.00")` retorna `true` (trailing zeros ignorados pela igualdade de `rust_decimal`).

Para ordenação, adicionar braços antes do fallback de erro:

```rust
(BinOp::Lt,  Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a < b)),
(BinOp::Leq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a <= b)),
(BinOp::Gt,  Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a > b)),
(BinOp::Geq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a >= b)),
```

### Unário negativo (opcional mas desejável)

Se `Decimal` suportar `-a` (a crate suporta), adicionar:

```rust
(UnOp::Neg, Value::Decimal(d)) => Ok(Value::Decimal(-d)),
```

Isto melhora a paridade sem aumentar o escopo do passo.

---

## 4. Paridade vanilla

| Expressão | Resultado |
|-----------|-----------|
| `decimal("1.5") + decimal("2.5")` | `4.0` (Decimal) |
| `decimal("10") - decimal("3")` | `7` (Decimal) |
| `decimal("2.5") * decimal("4")` | `10.0` (Decimal) |
| `decimal("10") / decimal("3")` | `3.333...` (Decimal, 28 dígitos) |
| `decimal("1") / decimal("0")` | erro "cannot divide by zero" |
| `decimal("1.0") == decimal("1.00")` | `true` |
| `decimal("1.0") < decimal("2.0")` | `true` |
| `decimal("1") + 2` | erro de tipo (sem coerção) |

---

## 5. Testes

Adicionar em `01_core/src/engine/eval/tests.rs` (ou módulo de testes apropriado):

```
eval_binary_op(Add, Decimal(1.5), Decimal(2.5)) → Decimal(4.0)
eval_binary_op(Sub, Decimal(10), Decimal(3)) → Decimal(7)
eval_binary_op(Mul, Decimal(2.5), Decimal(4)) → Decimal(10.0)
eval_binary_op(Div, Decimal(10), Decimal(3)) → Decimal(3.333...)
eval_binary_op(Div, Decimal(1), Decimal(0)) → Err("cannot divide by zero")
eval_binary_op(Eq, Decimal(1.0), Decimal(1.00)) → Bool(true)
eval_binary_op(Neq, Decimal(1.0), Decimal(2.0)) → Bool(true)
eval_binary_op(Lt, Decimal(1.0), Decimal(2.0)) → Bool(true)
eval_binary_op(Leq, Decimal(2.0), Decimal(2.0)) → Bool(true)
eval_binary_op(Gt, Decimal(3.0), Decimal(2.0)) → Bool(true)
eval_binary_op(Geq, Decimal(3.0), Decimal(3.0)) → Bool(true)
eval_binary_op(Add, Decimal(1), Int(2)) → Err(...)
eval_unary_op(Neg, Decimal(1.5)) → Decimal(-1.5)
```

---

## 6. Scope-out

- Não implementar coerção `Decimal ↔ Int/Float`.
- Não implementar `//` (floor division) nem `%` (módulo) para `Decimal`.
- Não tocar em `entities/decimal.rs` além de confirmar que o wrapper permite as operações.
- Não adicionar funções stdlib `decimal_add`, etc. — operações são infixas.
