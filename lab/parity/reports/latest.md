# Paridade — Passo 480 (2026-06-27)

**Última actualização:** P480 (2026-06-27).
**Vanilla CLI:** typst 0.14.2.
**Corpus:** 46 ficheiros.

---

## Matriz de paridade estrutural

| Indicador | Valor |
|-----------|------:|
| Total ficheiros corpus | 46 |
| Includes (testados) | 28 |
| Skips | 18 |
| Errors | 0 |
| Comparações | 73 |
| — Matches | **73** |
| — Diffs | **0** |

---

## Diffs restantes (0)

Nenhum diff activo. Todos os diffs históricos resolvidos:

| Ficheiro | Selector | Resolução | Passo |
|----------|----------|-----------|-------|
| `visual/outline-toc.typ` | `heading` | Registo sintético em `kind_index[Heading]` no walk arm `Content::Outline`. | P480 |
| `visual/cite-bibliography.typ` | `heading` | `native_bibliography` define título padrão `Content::heading(1, "Bibliography")`. | P479 |

---

## Errors (0)

Nenhum error activo.

P480 resolveu os 22 errors do selector `equation`: vanilla usava `math.equation` como namespace; cristalino passou a aceitar ambas as formas (`math.equation` → alias, `equation` → path interno). Selector corpus actualizado para `"math.equation"`.

---

## Histórico de cobertura

| Passo | Data | Corpus | INCLUDE | Matches | Diffs | Fix materializado |
|-------|------|-------:|--------:|--------:|------:|-------------------|
| P150 | 2026-04-25 | 25 | N/A | N/A | N/A | Baseline cristalino-only |
| P206D | 2026-05-08 | 36 | 23 | ~20 | 3 | Vanilla integration infra |
| P479 | 2026-06-27 | 46 | 28 | 50 | 1 | `bibliography` título padrão S1 |
| **P480** | **2026-06-27** | **46** | **28** | **73** | **0** | Outline heading sintético + `math.equation` alias |

---

## Relatório versionado

`lab/parity/reports/2026-06-27-passo-480.md`
