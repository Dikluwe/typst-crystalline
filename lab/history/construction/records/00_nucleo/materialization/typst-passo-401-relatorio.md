# Passo 401 — relatório: `Value::Version`

**Tipo:** modelagem de tipo primitivo L1 (S puro; zero consumer; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`84be3351b`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Modelou-se o tipo `Version` como novo variant de `Value`, fechando a série de tipos S puros (P399 Decimal → P400 Duration → P401 Version).

- L0:
  - `00_nucleo/prompts/entities/version.md` — tipo `Version` e variant `Value::Version`.
- `01_core/src/entities/version.rs`:
  - Tipo L1 puro `Version { major, minor, patch, pre, build }` com `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`.
  - Sem prerelease/build metadata por omissão; builders `with_pre` / `with_build`.
  - `to_string()` canónica `"major.minor.patch[-pre][+build]"`.
  - `from_str()` parse semver (build depois de `+`, prerelease depois de `-`).
  - Comparação semver 2.0.0: major→minor→patch, prerelease < release, build metadata ignorado.
  - Testes unitários: 18 testes (parse, repr, comparação).
- `01_core/src/entities/value.rs`:
  - Novo variant `Value::Version(Arc<Version>)` (Arc para cheap clone, paridade pattern P395).
  - `type_name()` retorna `"version"`.
  - `cast_version()` extrai `Arc<Version>` ou converte `Str` (parse semver).
  - `From<Version> for Value`.
  - Testes unitários: 8 testes.
- `01_core/src/entities/mod.rs`:
  - `pub mod version;` registado com nota P401.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela B.1: `Version` reclassificado de `ausente` para `implementado`; variants 21 → **22**, total 34 → **35**.
  - Tabela B resumo: implementado 77 → **78**, total arquitectural 109 → **110**.
  - Contagem user-facing total mantida em **141** (`Version` é tipo arquitectural; `version()` stdlib continua `ausente` — futuro S).
  - Nota de rodapé ⁸⁵ para P401.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`version.md`) escrito e hash propagado.
2. TDD: testes unitários escritos antes/paralelamente à implementação.
3. Novo `Value` variant justificado por ADR-0017 (portão aberto) e paridade linguagem.
4. Zero I/O; zero consumer; zero func stdlib nova — S puro.

## Decisão de engenharia

`Version` usa tipo próprio em vez do crate `semver`, evitando uma dependência externa adicional. A representação com `Vec<EcoString>` para prerelease/build é suficiente para a paridade linguagem (ADR-0107). Como o tipo não é `Copy`, `Value::Version` usa `Arc<Version>` para cheap clone — o mesmo padrão de `Value::Tiling` (P395).

A comparação semver implementa as regras essenciais: major→minor→patch, prerelease < release, identificadores numéricos comparados numericamente, não-numéricos lexicograficamente, e build metadata ignorado. Edge cases raros ficam scope-out para quando o constructor stdlib e as operações de comparação forem implementados.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `Version::new(1, 2, 3).to_string()` | `"1.2.3"` | ✓ |
| `Version::from_str("1.2.3-alpha.1+build.2")` | parse correto | ✓ |
| `Version::from_str("abc")` | `None` | ✓ |
| `1.0.0-alpha < 1.0.0` | `true` | ✓ |
| `1.0.0-1 < 1.0.0-alpha` | `true` (numeric < non-numeric) | ✓ |
| `1.0.0+build1 == 1.0.0+build2` | `true` (build ignorado) | ✓ |
| `Value::Version(...).type_name()` | `"version"` | ✓ |
| `Value::Str("1.2.3").cast_version()` | `Arc<Version>` correto | ✓ |
| `Value::Str("abc").cast_version()` | `None` | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Value::Version(Arc<Version>)` compila e integra-se no enum | ✓ |
| 2 | `Version` é `Clone`, puro-Rust, alloc mínimo | ✓ |
| 3 | Parse semver: `Str → Version` | ✓ |
| 4 | Comparação semver 2.0.0 (`PartialOrd` + `Ord`) | ✓ |
| 5 | Repr canónico `"major.minor.patch[-pre][+build]"` | ✓ |
| 6 | Zero consumer complexo; zero I/O; zero func stdlib nova | ✓ |
| 7 | Testes verdes; lint zero; hashes propagados | ✓ 26 unit novos |
| 8 | Inventário 148 actualizado | ✓ |
| 9 | L0 salvo e hashado | ✓ `version.md` |
| 10 | Ritmo S comparável a P399/P400 | ✓ tipo puro, mesmo padrão |

## Artefactos

- Código:
  - `01_core/src/entities/version.rs`
  - `01_core/src/entities/value.rs`
  - `01_core/src/entities/mod.rs`
- L0:
  - `00_nucleo/prompts/entities/version.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Plano: `00_nucleo/materialization/typst-passo-401.md`.
- Este relatório.

## Nota sobre o Tekt

P401 fecha a **série de tipos S puros** aberta pelo portão ADR-0017. Após este passo:
- Todos os tipos primitivos pendentes da sonda 389 estão modelados (`Decimal`, `Duration`, `Version`).
- O portão ADR-0017 permanece aberto como regra de processo para features futuras.
- O projecto pode avançar para constructors stdlib (`native_decimal`, `native_duration`, `native_version`) ou para features com dependências reais (bibliography CSL, shaping, etc.).

O ritmo uniforme dos três passos (P399 → P400 → P401) valida que o portão funciona para modelagem pura de tipos.
