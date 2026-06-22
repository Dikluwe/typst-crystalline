# Passo 405 — relatório: activação de `Duration` — constructor + operações básicas

**Tipo:** Activação de tipo modelado (L1 — stdlib constructor + eval ops; zero I/O; zero tipo novo; reusa `Value::Duration` P400).  
**Data:** 2026-06-22. **HEAD:** pós-`583288435`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Activou-se `Value::Duration` para uso user-facing: constructor stdlib alinhado com vanilla + operações básicas eval.

- L0:
  - `00_nucleo/prompts/rules/stdlib/primitives-constructors.md` — expandido para cobrir a forma named args de `duration()` e as operações eval básicas (P405). O ficheiro `duration-stdlib.md` separado foi eliminado para evitar drift de hashes com múltiplos `@prompt` no mesmo ficheiro Rust.
- `01_core/src/rules/stdlib/primitives_constructors.rs`:
  - `native_duration` estendido para aceitar:
    - Forma vanilla: named args `days`, `hours`, `minutes`, `seconds`, `milliseconds`, `microseconds`, `nanoseconds` (todos `Int` ≥ 0).
    - Forma legada P403: string posicional `duration("1h30m")`.
  - Validação de negativos, tipos errados, overflow `u64`, e mistura posicional+named.
  - 8 novos testes unitários para o constructor named args.
- `01_core/src/rules/eval/operators.rs`:
  - `+` e `-` homogéneos `Duration` (com overflow/underflow checks).
  - `*` `Duration × Int/Float` e simétricos (rejeita negativos; trunca sub-nano).
  - `/` `Duration ÷ Int/Float` e `Duration ÷ Duration → Float` (rejeita div/0 e negativos).
  - Comparações `==`, `!=`, `<`, `<=`, `>`, `>=` homogéneas `Duration`.
- `01_core/src/rules/eval/tests.rs`:
  - 14 novos testes para operações eval de `Duration`.

`cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` → 2964 passed; `crystalline-lint .` → `✓ No violations found`; hash propagado (`d910ed09`).

## Protocolo de Nucleação cumprido

1. L0 (`primitives-constructors.md`) actualizado e hash propagado.
2. Sonda do substrato: `Value::Duration` OK; `Duration` L1 OK; eval binário infra OK; `rust_decimal` não aplicável.
3. Testes escritos nos módulos respectivos.
4. Zero tipo novo; zero variant novo; zero I/O.

## Decisão de engenharia

O constructor `duration()` suporta duas formas: named args (paridade vanilla) e string posicional (compatibilidade P403). A forma string é mantida para não quebrar o passo anterior; a forma vanilla é a preferida.

As operações `Duration` usam `u128` para intermédios e rejeitam overflow/underflow silencioso. `Duration / Duration` retorna `Float` (razão), seguindo vanilla.

## Paridade

| Expressão | Resultado | Estado |
|-----------|-----------|--------|
| `duration(seconds: 90)` | 90s | ✓ |
| `duration(days: 1, hours: 2, minutes: 3)` | 1d2h3m | ✓ |
| `duration(seconds: -1)` | erro | ✓ |
| `duration("1h30m")` | 5400s (compat P403) | ✓ |
| `duration(seconds: 90) + duration(seconds: 30)` | 120s | ✓ |
| `duration(minutes: 2) > duration(seconds: 119)` | true | ✓ |
| `duration(seconds: 120) / duration(seconds: 60)` | 2.0 | ✓ |
| `duration(seconds: 60) * 2` | 120s | ✓ |
| `duration(seconds: 60) * 1.5` | 90s | ✓ |
| `duration(seconds: 120) / 2` | 60s | ✓ |
| `duration(seconds: 30) - duration(seconds: 120)` | erro (underflow) | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `duration(seconds: 90)` retorna `Value::Duration` | ✓ |
| 2 | `duration(seconds: 90) + duration(seconds: 30)` → 120s | ✓ |
| 3 | `duration(minutes: 2) > duration(seconds: 119)` → true | ✓ |
| 4 | Overflow/underflow/div-by-zero rejeitados com `Err` | ✓ |
| 5 | Zero tipo novo; zero I/O; zero variant novo | ✓ |
| 6 | Testes verdes; lint zero; hashes propagados | ✓ 22 testes novos |
| 7 | Inventário 148: `duration()` stdlib implementado | ✓ |
| 8 | L0 salvo e hashado; sonda documentada | ✓ |

## Artefactos

- Código:
  - `01_core/src/rules/stdlib/primitives_constructors.rs`
  - `01_core/src/rules/eval/operators.rs`
  - `01_core/src/rules/eval/tests.rs`
- L0:
  - `00_nucleo/prompts/rules/stdlib/primitives-constructors.md`
- Plano: `00_nucleo/materialization/typst-passo-405.md`.
- Este relatório.

## Nota sobre o Tekt

P405 é activação de tipo S modelado, paralelo a P404 (Decimal). O ritmo foi S-M: o tipo já existia e as operações são mecânicas. A única complexidade extra foi reconciliar o constructor string do P403 com a forma vanilla named args — resolvido mantendo ambas.

O próximo passo da série é P406 (Version), que seguirá o mesmo padrão: constructor multi-arg (com prerelease/build) + comparações básicas.
