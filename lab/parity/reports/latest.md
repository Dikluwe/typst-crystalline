# Paridade — Passo 479 (2026-06-27)

**Última actualização:** P479 (2026-06-27).
**Vanilla CLI:** typst 0.14.2.
**Corpus:** 46 ficheiros.

---

## Matriz de paridade estrutural

| Indicador | Valor |
|-----------|------:|
| Total ficheiros corpus | 46 |
| Includes (testados) | 28 |
| Skips | 18 |
| Errors | 22 |
| Comparações | 73 |
| — Matches | **50** |
| — Diffs | **1** |

---

## Diffs restantes (1)

| Ficheiro | Selector | Cristalino | Vanilla | Causa | Magnitude fix |
|----------|----------|:----------:|:-------:|-------|:-------------:|
| `visual/outline-toc.typ` | `heading` | 5 | 6 | Outline title heading criado em layout-time; invisível a pré-layout query (walk arm `Content::Outline` vazio P189B). | M |

---

## Errors (22)

Todos do selector `equation` standalone: vanilla rejeita ("unknown variable: equation"); cristalino aceita via `ElementKind::Equation`. Arquitectónico; pré-existente.

---

## Histórico de cobertura

| Passo | Data | Corpus | INCLUDE | Matches | Diffs | Fix materializado |
|-------|------|-------:|--------:|--------:|------:|-------------------|
| P150 | 2026-04-25 | 25 | N/A | N/A | N/A | Baseline cristalino-only |
| P206D | 2026-05-08 | 36 | 23 | ~20 | 3 | Vanilla integration infra |
| P479 | 2026-06-27 | 46 | 28 | 50 | 1 | `bibliography` título padrão S1 |

---

## Relatório versionado

`lab/parity/reports/2026-06-27-passo-479.md`
