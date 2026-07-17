# Passo 403 — relatório: constructors stdlib `decimal()`, `duration()`, `version()`

**Tipo:** Materialização (L1 stdlib + enum `Value`; zero tipo novo; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`06f628b87`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Ativaram-se os três tipos L1 modelados em P399–P401 com constructors stdlib puros `Str → T fallible`, consumindo o portão ADR-0017.

- L0:
  - `00_nucleo/prompts/engine/stdlib/primitives-constructors.md` — novo prompt dedicado aos três constructors.
- `01_core/src/engine/stdlib/primitives_constructors.rs`:
  - `native_decimal(s)` → parse via `Decimal::from_str` → `Value::Decimal`.
  - `native_duration(s)` → parser canónico `NdNhNmNs` (ordem fixa, fração decimal apenas em segundos) → `Value::Duration`.
  - `native_version(s)` → parse semver 2.0.0 via `Version::from_str` → `Value::Version(Arc::new(...))`.
  - 20 testes unitários cobrindo: parse válido/inválido, tipo errado, aridade errada, argumentos nomeados rejeitados, ordem de sufixos inválida.
- `01_core/src/engine/stdlib/mod.rs`:
  - Declaração do submódulo `primitives_constructors`.
  - Re-exportação de `native_decimal`, `native_duration`, `native_version`.
- `01_core/src/engine/eval/mod.rs`:
  - Registo dos três constructors em `make_stdlib` (`scope.define("decimal", ...)`, etc.).

`cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` → 2933 passed; `crystalline-lint .` → `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`primitives-constructors.md`) redigido e hash propagado (`e88a8a83`).
2. Sonda A.0 executada antes do código — substrato verificado (3 variants + 3 structs + padrão `make_stdlib` OK; zero natives pré-existentes).
3. Testes escritos no próprio módulo (20 novos).
4. Zero tipo novo; zero variant novo; zero I/O.

## Decisão de engenharia

Os três constructors são morfologicamente idênticos (`Str → T fallible`). Reutilizam os tipos L1 e variants de `Value` existentes; não expandem o enum. O parser de duração é hand-rolled minimal (regex não necessária) e aceita o subconjunto canónico `NdNhNmNs`; refins de formato (espaços, sinal negativo, unidades compostas) ficam scope-out para passos futuros.

> **Nota retroativa:** este passo entregou apenas a forma string de `duration()`. A forma named args (`duration(seconds:, hours:, minutes:, days:)`) foi adicionada em P405. Entre P403 e P405 o L0 `primitives-constructors.md` descreveu apenas a forma string; um leitor do L0 nesse intervalo tinha uma descrição incompleta do constructor. Regra para o futuro: quando um constructor é dividido entre passos, o L0 do passo inicial declara explicitamente que está incompleto e nomeia o passo que completa.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `decimal("1.23")` | `Value::Decimal(1.23)` | ✓ |
| `decimal("abc")` | erro eval | ✓ |
| `duration("1h30m")` | 5400s | ✓ |
| `duration("2h30m")` | 9000s | ✓ |
| `duration("3d2h30m15.5s")` | 3d + 2h + 30m + 15.5s | ✓ |
| `duration("abc")` | erro eval | ✓ |
| `duration("30m1h")` | erro (ordem invertida) | ✓ |
| `version("1.2.3")` | `Value::Version(1,2,3)` | ✓ |
| `version("1.2.3-alpha.1")` | pre=["alpha","1"] | ✓ |
| `version("1.2.3+build.2")` | build=["build","2"] | ✓ |
| `version("invalid")` | erro eval | ✓ |
| argumento nomeado rejeitado | erro eval | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `decimal("1.5")`, `duration("1h")`, `version("1.0.0")` produzem `Value` correto | ✓ |
| 2 | Zero tipo novo; zero variant novo; zero I/O | ✓ |
| 3 | Testes verdes (≥ 12 novos); lint zero; hashes propagados | ✓ 20 testes |
| 4 | Inventário 148: `decimal`/`duration`/`version` transitem `ausente` → `implementado` | ✓ (documentado em cobertura; código ativa os constructors) |
| 5 | L0 salvo e hashado antes do código | ✓ |
| 6 | Sonda A.0 executada e documentada no commit | ✓ |

## Artefactos

- Código:
  - `01_core/src/engine/stdlib/primitives_constructors.rs`
  - `01_core/src/engine/stdlib/mod.rs`
  - `01_core/src/engine/eval/mod.rs`
- L0:
  - `00_nucleo/prompts/engine/stdlib/primitives-constructors.md`
- Plano: `00_nucleo/materialization/typst-passo-403.md`.
- Este relatório.

## Nota sobre o Tekt

P403 é o consumo do portão ADR-0017: tipos modelados ganham vida via constructors. O ritmo foi M conforme esperado, com baixo risco porque os tipos já estavam validados. O parser de duração é o único componente com lógica nova; se crescer, merece módulo dedicado em passo futuro.
