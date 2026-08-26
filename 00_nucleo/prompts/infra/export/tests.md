# Prompt L0 — `infra/export/tests` — Testes agregadores E2E
Hash do Código: 0c915794

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0037 Regra 5 Ajuste C (testes E2E cross-cutting), ADR-0100 (coesão L3)

---

## Contexto

Ficheiro agregador de **230 testes** unitários e end-to-end do
exporter PDF. Migrado em P307b.1 do bloco inline `#[cfg(test)] mod tests`
de `export.rs` para ficheiro dedicado, preservando bit-exact a
estrutura e a cobertura.

Distribuição por feature (counts P307a):

| Prefixo | Tests | Cluster |
|---|---:|---|
| `p270_*` | 69 | CMYK gradient |
| `p273_*` | 56 | Gradient relative + clip |
| `p272_*` | 15 | Conic Coons |
| `p274_*` | 14 | Adaptive N |
| `p269_*` | 13 | Gradient focal |
| `p281_*` | 10 | PageContext unificado |
| `p263_*` | 8 | Gradient linear |
| Outros | 45 | distribuídos |
| **Total** | **230** | |

## Restrições estruturais

- Categoria Regra 6 (ADR-0037): "infraestrutura de testes E2E".
  Excede limite 800 LOC por design — testes cruzam features
  (gradients dentro de Groups, multifont com images, etc.).
- `use super::*` traz toda a API interna de `export/mod.rs`
  (re-exports dos submódulos).
- Não decompôr por cluster em P307b.1 — perda de cobertura cross-feature
  esperada. Decomposição futura só se ratio LOC/coverage justificar.

## Interface

Nenhuma — é módulo `#[cfg(test)]` com `#[test]` functions.
Não consumido por código de produção.

## Critérios de verificação

`cargo test -p typst-infra --lib export::tests` deve passar 230+
testes. Falhas indicam regressão em algum cluster L3.

## Não-objectivos

- **Não** testa paridade vanilla (vive em `lab/parity/`).
- **Não** testa binário cristalino vs cristalino (vive em `p307b_snapshot_tests.rs`).
- **Não** valida performance.
