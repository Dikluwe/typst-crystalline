# Passo 413 — Relatório de execução: Aritmética `Decimal`

**Tipo**: Materialização (L1 eval + stdlib).  
**Data**: 2026-06-22.  
**Status**: **concluído sem alterações de código** — funcionalidade já implementada no Passo 404.

---

## 1. Sonda do substrato (FASE A.0)

Executados os comandos de sonda definidos em `typst-passo-413.md`:

1. `Value::Decimal` existe em `entities/value.rs` ✅
2. `Decimal` tipo L1 em `entities/decimal.rs` é wrapper sobre `rust_decimal::Decimal` ✅
3. Infra de eval binário existe em `rules/eval/operators.rs` (dispatch por `(op, lhs, rhs)`) ✅
4. `Value::Decimal` **já está presente** nos operadores em `rules/eval/operators.rs` ✅
5. `rust_decimal` é dependência do workspace ✅

**Resultado da sonda**: a aritmética e comparações de `Decimal` já foram materializadas no Passo 404. O passo 413 é, portanto, redundante em relação ao código atual.

---

## 2. Decisão sobre L0 (FASE A)

Não foi criado `00_nucleo/prompts/engine/eval/decimal-arithmetic.md` separado porque:

- A implementação já existe e está testada.
- Criar um prompt L0 para código que já foi escrito violaria o protocolo de nucleação.
- O material `decimal-arithmetic` encontra-se documentado nos comentários `P404` de `rules/eval/operators.rs` e nos testes de `rules/eval/tests.rs`.

---

## 3. Implementação existente (FASE B)

Local: `01_core/src/engine/eval/operators.rs` — comentários `P404`.

Operadores suportados para `Value::Decimal`:

| Operador | Operandos | Resultado | Observação |
|---|---|---|---|
| `+` | `Decimal + Decimal` | `Decimal` | via `a.0 + b.0` |
| `-` | `Decimal - Decimal` | `Decimal` | via `a.0 - b.0` |
| `*` | `Decimal * Decimal` | `Decimal` | via `a.0 * b.0` |
| `/` | `Decimal / Decimal` | `Decimal` | divisão por zero verificada previamente |
| `==`, `!=` | `Decimal == Decimal` | `Bool` | bit-equivalente via `rust_decimal` |
| `<`, `<=`, `>`, `>=` | `Decimal <=> Decimal` | `Bool` | ordenação total via `rust_decimal` |

Divisão por zero é verificada no início de `eval_binary_op` para `Value::Decimal` com `d.0.is_zero()`.

---

## 4. Testes existentes

Local: `01_core/src/engine/eval/tests.rs` — bloco `P404 — Aritmética e comparações Decimal`.

9 testes cobrem:

- `decimal_add`
- `decimal_sub`
- `decimal_mul`
- `decimal_div`
- `decimal_eq_neq`
- `decimal_lt_gt_leq_geq`
- `decimal_no_coercion_with_int_float`
- `decimal_neg_unary`

---

## 5. Validações

- `cargo test -p typst-core decimal` — **30 testes passaram; 0 falhas**.
- `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — **verde**.
- `crystalline-lint` — **0 erros / 0 drift**; resta apenas warning pré-existente de `show-regex.md` órfão.

---

## 6. Conclusão

O Passo 413 não requereu alterações de código: a aritmética e comparações de `Decimal` já estavam implementadas no Passo 404. O trabalho deste passo limitou-se a:

1. Executar a sonda do substrato.
2. Verificar a implementação e a cobertura de testes.
3. Documentar o estado num relatório.

Nenhum prompt L0 novo foi criado, nenhum hash de prompt precisou ser propagado, e nenhuma mudança no `Value`, `Decimal` ou stdlib foi necessária.

## Nota metodológica

Esta spec foi originalmente redigida como materialização antes da sonda A.0. A sonda revelou
que o trabalho já tinha sido feito no Passo 404. A correção de deriva converteu a spec em
documento de verificação retroativa e acrescentou este relatório. Este caso é um dos cinco
(P388, P409, P413, P416, P421) que motivam o gate "sonda A.0 antes da spec" formalizado na
ADR metodológica deste plano de correção.
