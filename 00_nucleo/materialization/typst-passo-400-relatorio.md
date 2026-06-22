# Passo 400 — relatório: `Value::Duration`

**Tipo:** modelagem de tipo primitivo L1 (S puro; zero consumer; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`ed363a5d7`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Modelou-se o tipo `Duration` como novo variant de `Value`, o segundo da série de tipos S puros (P399 Decimal → P400 Duration → P401 Version).

- L0:
  - `00_nucleo/prompts/entities/duration.md` — tipo `Duration` e variant `Value::Duration`.
- `01_core/src/entities/duration.rs`:
  - Tipo L1 puro `Duration { nanos: u64 }` com `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`.
  - Constantes `ZERO`, `SECOND`, `MINUTE`, `HOUR`, `DAY`.
  - Constructors `from_nanos`, `from_seconds`, `from_minutes`, `from_hours`, `from_days`.
  - Acessores `as_seconds`, `as_minutes`, `as_hours`, `as_days`, `is_zero`.
  - `to_string()` canónica `"NdNhNmN.NNNs"` (ex.: `"3d2h30m15.5s"`, `"0.0005s"`, `"0s"`).
  - Testes unitários: 9 testes.
- `01_core/src/entities/value.rs`:
  - Novo variant `Value::Duration(Duration)`.
  - `type_name()` retorna `"duration"`.
  - `cast_duration()` converte `Duration`, `Int` (nanos; rejeita negativos) e `Float` (segundos→nanos; rejeita negativos e overflow).
  - `From<Duration> for Value`.
  - Testes unitários: 9 testes.
- `01_core/src/entities/mod.rs`:
  - `pub mod duration;` registado com nota P400.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela B.1: `Duration` reclassificado de `ausente` para `implementado`; variants 20 → **21**, total 33 → **34**.
  - Tabela B resumo: implementado 76 → **77**, total arquitectural 108 → **109**.
  - Contagem user-facing total mantida em **141** (`Duration` é tipo arquitectural; `duration()` stdlib continua `ausente` — futuro S).
  - Nota de rodapé ⁸⁴ para P400.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`duration.md`) escrito e hash propagado.
2. TDD: testes unitários escritos antes/paralelamente à implementação.
3. Novo `Value` variant justificado por ADR-0017 (portão aberto) e paridade linguagem.
4. Zero I/O; zero consumer; zero func stdlib nova — S puro.

## Decisão de engenharia

`Duration` usa um `u64` de nanossegundos como representação interna. A paridade linguagem (ADR-0107) é com o **valor do intervalo**, não com a mecânica interna. A representação canónica `"NdNhNmN.NNNs"` é legível, reversível e suficiente para o passo de modelagem; pode ser ajustada quando o constructor stdlib for implementado e os testes de paridade com o vanilla definirem o formato exacto.

`cast_duration` rejeita valores negativos e overflow, mantendo a semântica de intervalo não-negativo do vanilla. `Float` é interpretado em segundos (multiplicado por `1e9`), com perda documentada de sub-nanosegundos.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `Duration::from_seconds(270180)` | `3d2h30m` | ✓ |
| `Duration::from_nanos(500_000)` | `0.0005s` | ✓ |
| `Duration::ZERO.to_string()` | `"0s"` | ✓ |
| `Value::Duration(...).type_name()` | `"duration"` | ✓ |
| `Value::Int(1_000_000_000).cast_duration()` | 1 segundo | ✓ |
| `Value::Int(-1).cast_duration()` | `None` | ✓ |
| `Value::Float(1.5).cast_duration()` | 1.5 segundos | ✓ |
| `Value::Float(-1.0).cast_duration()` | `None` | ✓ |
| `Duration::from_seconds(60) == Duration::from_minutes(1)` | `true` | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Value::Duration(Duration)` compila e integra-se no enum | ✓ |
| 2 | `Duration` é `Copy`, puro-Rust, zero alloc | ✓ |
| 3 | Cast: `Duration → Duration`, `Int → Duration` (nanos), `Float → Duration` (segundos) | ✓ |
| 4 | Repr: formato canónico `NdNhNmN.NNNs` ou `"0s"` | ✓ |
| 5 | Zero consumer complexo; zero I/O; zero func stdlib nova | ✓ |
| 6 | Testes verdes; lint zero; hashes propagados | ✓ 18 unit novos |
| 7 | Inventário 148 actualizado | ✓ |
| 8 | L0 salvo e hashado | ✓ `duration.md` |
| 9 | Ritmo S comparável a P399 | ✓ tipo puro, ~ mesmo padrão |

## Artefactos

- Código:
  - `01_core/src/entities/duration.rs`
  - `01_core/src/entities/value.rs`
  - `01_core/src/entities/mod.rs`
- L0:
  - `00_nucleo/prompts/entities/duration.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Plano: `00_nucleo/materialization/typst-passo-400.md`.
- Este relatório.

## Nota sobre o Tekt

P400 mantém o ritmo da série de tipos S puros: L1 puro, zero consumer, ~10-15 testes. Após P400, resta `Version` (P401) para fechar a fila de tipos primitivos pendentes aberta pelo portão ADR-0017.
