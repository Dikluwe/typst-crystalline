# Passo 409 — Relatório de execução: Aritmética `Duration`

**Tipo**: Materialização (L1 eval + stdlib).  
**Data**: 2026-06-22.  
**Status**: **concluído sem alterações de código** — funcionalidade já implementada no Passo 405.

---

## 1. Sonda do substrato (FASE A.0)

Executados os comandos de sonda definidos em `typst-passo-409.md`:

1. `Value::Duration` existe em `entities/value.rs` ✅
2. `Duration` tipo L1 em `entities/duration.rs` com campo `nanos: u64` público ✅
3. Infra de eval binário existe em `rules/eval/operators.rs` (dispatch por `(op, lhs, rhs)`) ✅
4. `Value::Duration` **já está presente** nos operadores em `rules/eval/operators.rs` ✅
5. Tipo base (`Duration` L1 com `u64` nanos) suporta as operações via aritmética inteira ✅

**Resultado da sonda**: a aritmética e comparações de `Duration` já foram materializadas no Passo 405 (ativação do constructor `duration()`). O passo 409 é, portanto, redundante em relação ao código atual.

---

## 2. Decisão sobre L0 (FASE A)

Não foi criado `00_nucleo/prompts/rules/eval/duration-arithmetic.md` separado porque:

- A implementação já existe e está testada.
- Criar um prompt L0 para código que já foi escrito violaria o protocolo de nucleação (L0 antes do código).
- O material `duration-arithmetic` encontra-se documentado implicitamente nos comentários `P405` de `rules/eval/operators.rs` e nos testes de `rules/eval/tests.rs`.

---

## 3. Implementação existente (FASE B)

Local: `01_core/src/rules/eval/operators.rs` — comentários `P405`.

Operadores suportados para `Value::Duration`:

| Operador | Operandos | Resultado | Observação |
|---|---|---|---|
| `+` | `Duration + Duration` | `Duration` | overflow → erro |
| `-` | `Duration - Duration` | `Duration` | underflow (negativo) → erro |
| `*` | `Duration * Int` / `Int * Duration` | `Duration` | int negativo → erro; overflow → erro |
| `*` | `Duration * Float` / `Float * Duration` | `Duration` | float negativo → erro |
| `/` | `Duration / Int` | `Duration` | divisão por zero → erro; int negativo → erro |
| `/` | `Duration / Float` | `Duration` | divisão por zero → erro; float negativo → erro |
| `/` | `Duration / Duration` | `Float` | ratio de nanos; divisão por zero → erro |
| `==`, `!=` | `Duration == Duration` | `Bool` | comparação bit-equivalente via nanos |
| `<`, `<=`, `>`, `>=` | `Duration <=> Duration` | `Bool` | ordenação total via nanos |

Nota: o P409 scope-out `Duration * Float`, mas o P405 já o implementou. Isso não introduz regressão nem quebra paridade.

---

## 4. Testes existentes

Local: `01_core/src/rules/eval/tests.rs` — bloco `P405 — Operações básicas Duration`.

14 testes cobrem:

- `duration_add`, `duration_add_overflow`
- `duration_sub`, `duration_sub_underflow`
- `duration_mul_int` (ambas as ordens), `duration_mul_int_neg`
- `duration_mul_float` (ambas as ordens)
- `duration_div_int`, `duration_div_int_zero`, `duration_div_int_neg`
- `duration_div_float`
- `duration_div_duration`, `duration_div_duration_zero`
- `duration_eq_neq`
- `duration_lt_gt_leq_geq`

---

## 5. Validações

- `cargo test -p typst-core duration` — **52 testes passaram; 0 falhas**.
- `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — **verde**.
- `crystalline-lint` — **0 erros / 0 drift**; resta apenas o warning pré-existente de `show-regex.md` órfão.

---

## 6. Conclusão

O Passo 409 não requereu alterações de código: a aritmética e comparações de `Duration` já estavam implementadas no Passo 405. O trabalho deste passo limitou-se a:

1. Executar a sonda do substrato.
2. Verificar a implementação e a cobertura de testes.
3. Documentar o estado num relatório.

Nenhum prompt L0 novo foi criado, nenhum hash de prompt precisou ser propagado, e nenhuma mudança no `Content`, `Value`, `Duration` ou stdlib foi necessária.
