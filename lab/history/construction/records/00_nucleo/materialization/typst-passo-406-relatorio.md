# Passo 406 — relatório: activação de `Version` — constructor + comparações stdlib

**Tipo:** Activação de tipo modelado (L1 — stdlib constructor + eval ops; zero I/O; zero tipo novo; reusa `Value::Version` P401).  
**Data:** 2026-06-22. **HEAD:** pós-`44e93e11f`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Activou-se `Value::Version` para uso user-facing: constructor stdlib alinhado com vanilla + comparações stdlib.

- L0:
  - `00_nucleo/prompts/engine/stdlib/primitives-constructors.md` — expandido para cobrir a forma vanilla de `version()` e as comparações eval (P406).
- `01_core/src/engine/stdlib/primitives_constructors.rs`:
  - `native_version` estendido para aceitar:
    - Forma vanilla: `version(1, 2, 3)`, `version(1, 2, 3, "alpha.1")`, `version(1, 2, 3, pre: "alpha.1", build: "build.2")`.
    - Forma legada P403: string posicional `version("1.2.3-alpha")`.
  - Validação de negativos, tipos errados, aridade, e mistura de formas.
  - 12 novos testes unitários para o constructor vanilla.
- `01_core/src/engine/eval/operators.rs`:
  - Comparações `==`, `!=`, `<`, `>`, `<=`, `>=` homogéneas `Version`.
  - `==`/`!=` ignoram build metadata (paridade vanilla semver); ordenação usa `Ord` de `Version` (já ignora build).
- `01_core/src/engine/eval/tests.rs`:
  - 4 novos testes para comparações de Version (incluindo prerelease vs release e build ignorado).

`cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` → 2980 passed; `crystalline-lint .` → `✓ No violations found`; hash propagado (`499f511d`).

## Protocolo de Nucleação cumprido

1. L0 (`primitives-constructors.md`) actualizado e hash propagado.
2. Sonda do substrato: `Value::Version` OK; `Version` L1 OK; eval comparações infra OK.
3. Testes escritos nos módulos respectivos.
4. Zero tipo novo; zero variant novo; zero I/O.

## Decisão de engenharia

O constructor `version()` suporta duas formas: string posicional (compatibilidade P403) e vanilla positional+named. A forma vanilla aceita `pre` como 4º posicional ou como named arg.

Em comparações, `==`/`!=` ignoram build metadata para paridade vanilla; ordenação usa o `Ord` já implementado em P401 (também ignora build). Isso evita tocar em `entities/version.rs`.

## Paridade

| Expressão | Resultado | Estado |
|-----------|-----------|--------|
| `version(1, 2, 3)` | `Value::Version(1,2,3)` | ✓ |
| `version(1, 2, 3, "alpha.1")` | pre=["alpha","1"] | ✓ |
| `version(1, 2, 3, pre: "alpha.1", build: "build.2")` | pre+build | ✓ |
| `version(-1, 2, 3)` | erro | ✓ |
| `version("1.2.3")` | 1.2.3 (compat P403) | ✓ |
| `version(1, 2, 3) == version(1, 2, 3)` | true | ✓ |
| `version(1, 2, 3, build: "a") == version(1, 2, 3, build: "b")` | true | ✓ |
| `version(1, 2, 3, "alpha") < version(1, 2, 3)` | true | ✓ |
| `version(1, 2, 3) < version(1, 2, 4)` | true | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `version(1, 2, 3)` retorna `Value::Version` | ✓ |
| 2 | `version(1, 2, 3, "alpha.1")` → pre=["alpha","1"] | ✓ |
| 3 | `version(1, 2, 3, build: "build.2")` → build=["build","2"] | ✓ |
| 4 | `version(1, 2, 3) == version(1, 2, 3)` → true | ✓ |
| 5 | `version(1, 2, 3, "alpha") < version(1, 2, 3)` → true | ✓ |
| 6 | build metadata ignorado em `==` | ✓ |
| 7 | Negativo em major/minor/patch → Err | ✓ |
| 8 | Zero tipo novo; zero I/O; zero variant novo | ✓ |
| 9 | Testes verdes; lint zero; hashes propagados | ✓ 16 testes novos |
| 10 | Inventário 148: `version()` stdlib implementado | ✓ |
| 11 | L0 salvo e hashado; sonda documentada | ✓ |

## Artefactos

- Código:
  - `01_core/src/engine/stdlib/primitives_constructors.rs`
  - `01_core/src/engine/eval/operators.rs`
  - `01_core/src/engine/eval/tests.rs`
- L0:
  - `00_nucleo/prompts/engine/stdlib/primitives-constructors.md`
- Plano: `00_nucleo/materialization/typst-passo-406.md`.
- Este relatório.

## Nota sobre o Tekt

P406 fecha a série de activação de tipos S modelados: P404 Decimal → P405 Duration → P406 Version. Todos os tipos primitivos pendentes da sonda 389 estão agora activos (tipo + constructor + ops básicas).

A complexidade de P406 foi comparável à de P405, com a nuance extra de reconciliar a forma string do P403 com a forma vanilla multi-arg. A decisão de manter ambas preserva compatibilidade sem criar divergência de API.
