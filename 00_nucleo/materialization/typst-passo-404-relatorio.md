# Passo 404 — relatório: aritmética `Decimal` (`+`, `-`, `*`, `/`) e comparações

**Tipo:** Materialização (L1 eval + stdlib; zero tipo novo; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`f5b2e9db1`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Ativaram-se os operadores aritméticos e comparativos para `Value::Decimal`, homogeneamente (`Decimal op Decimal`).

- L0:
  - `00_nucleo/prompts/rules/eval/decimal-arithmetic.md` — novo prompt dedicado.
- `01_core/src/rules/eval/operators.rs`:
  - Verificação de divisão por zero estendida a `Value::Decimal`.
  - Braços `BinOp::Add/Sub/Mul/Div` para `Value::Decimal` → `Value::Decimal` via `rust_decimal::Decimal`.
  - Braços de ordenação `Lt/Leq/Gt/Geq` para `Value::Decimal` → `Value::Bool`.
  - `UnOp::Neg` para `Value::Decimal`.
  - `Eq`/`Neq` já funcionam via `derive(PartialEq)` de `Value::Decimal` (trailing zeros ignorados).
- `01_core/src/rules/eval/tests.rs`:
  - 8 novos testes cobrindo `+`, `-`, `*`, `/`, comparações, divisão por zero, ausência de coerção com Int/Float, e negação unária.

`cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` → 2941 passed; `crystalline-lint .` → `✓ No violations found`; hashes propagados manualmente (o linter `--fix-hashes` atualizou o código; o prompt foi alinhado em seguida).

## Protocolo de Nucleação cumprido

1. L0 (`decimal-arithmetic.md`) redigido e hash propagado (`1e867e48`).
2. Sonda A.0 executada antes do código — Decimal variant OK; eval binário infra OK (Int/Float); Decimal ausente em ops; `rust_decimal` OK.
3. Testes escritos no módulo de testes do eval.
4. Zero tipo novo; zero variant novo; zero I/O.

## Decisão de engenharia

A aritmética Decimal é homogénea neste passo: não há coerção com `Int`/`Float`. As operações usam o `InnerDecimal` da crate `rust_decimal` diretamente, rewrap em `entities::decimal::Decimal`. A divisão por zero reutiliza a mensagem `"cannot divide by zero"` do motor.

A comparação `==`/`!=` não precisou de braços novos porque o `derive(PartialEq)` de `Value` (e de `Decimal`) já trata `decimal("1.0") == decimal("1.00")` corretamente.

## Paridade

| Expressão | Resultado | Estado |
|-----------|-----------|--------|
| `decimal("1.5") + decimal("2.5")` | `Decimal(4.0)` | ✓ |
| `decimal("10") - decimal("3")` | `Decimal(7)` | ✓ |
| `decimal("2.5") * decimal("4")` | `Decimal(10.0)` | ✓ |
| `decimal("10") / decimal("3")` | `Decimal(3.333...)` | ✓ |
| `decimal("1") / decimal("0")` | erro "cannot divide by zero" | ✓ |
| `decimal("1.0") == decimal("1.00")` | `true` | ✓ |
| `decimal("1.0") < decimal("2.0")` | `true` | ✓ |
| `decimal("1") + 2` | erro de tipo | ✓ |
| `-decimal("1.5")` | `Decimal(-1.5)` | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `+`, `-`, `*`, `/` entre `Value::Decimal` funcionam | ✓ |
| 2 | Comparações retornam `Value::Bool` correto | ✓ |
| 3 | Divisão por zero produz erro eval | ✓ |
| 4 | Mistura Decimal + Int/Float sem coerção → erro | ✓ |
| 5 | Zero tipo novo; zero variant novo; zero I/O | ✓ |
| 6 | Testes verdes (≥ 20); lint zero; hashes propagados | ✓ 8 testes novos + regressões |
| 7 | L0 salvo e hashado; sonda A.0 documentada | ✓ |

## Artefactos

- Código:
  - `01_core/src/rules/eval/operators.rs`
  - `01_core/src/rules/eval/tests.rs`
- L0:
  - `00_nucleo/prompts/rules/eval/decimal-arithmetic.md`
- Plano: `00_nucleo/materialization/typst-passo-404.md`.
- Este relatório.

## Nota sobre o Tekt

P404 é S/M: o tipo L1 já existia e o dispatch binário centralizado em `operators.rs` permitiu adicionar todos os operadores com ~20 linhas de produção. O maior trabalho foi o conjunto de testes. A coerção cruzada Decimal↔Int/Float permanece scope-out para passo futuro, quando a semântica de precisão for discutida explicitamente.
