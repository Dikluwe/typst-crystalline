# Análise de Paridade — Lente — Passo 530 (2026-07-02)

## Resultado da lente (parity-runner)

Corpus principal (46 ficheiros, excluindo corpora especializados p490/p500/p520/p523/rtl):

```
46/46 ✓ — paridade total
```

Nenhuma divergência. O parser cristalino está em paridade sintáctica completa com
`typst-syntax` em todos os 46 ficheiros.

---

## Diff SyntaxKind: vanilla (137) vs cristalino (134)

**No vanilla mas não no cristalino — 3 variantes:**

| Variante vanilla     | O cristalino usa                         |
|----------------------|------------------------------------------|
| `MathCall`           | `FuncCall` (mesmo nó, contexto math)     |
| `MathArgs`           | `Args` (mesmo nó, contexto math)         |
| `MathFieldAccess`    | `FieldAccess` (mesmo nó, contexto math)  |

**No cristalino mas não no vanilla — 0.**

Não é lacuna de linguagem. É divergência arquitectónica deliberada: o cristalino
reutiliza os nós de code (`FuncCall`, `Args`, `FieldAccess`) dentro de math em vez de
criar variantes math-específicas. A lente (`compact.rs`) normaliza isto com os aliases
`"math function call"` / `"math call arguments"`, por isso os 46/46 passam.

---

## HTML / SVG

Não existem `SyntaxKind::Html*` ou `SyntaxKind::Svg*` em nenhum dos dois parsers.
O vanilla tem export HTML (pós-compilação, target separado) mas não é sintaxe —
não há nós a cobrir no corpus de parsing.

---

## Lacunas de corpus (corpora especializados separados)

| Corpus              | Estado            |
|---------------------|-------------------|
| `corpus/rtl/`       | SKIP-feature (RTL/bidi) |
| `corpus/p490/`      | Corpus especializado P490 |
| `corpus/p500/`      | Corpus especializado P500 |
| `corpus/p520/`      | Sentinela kerning/ligatures |
| `corpus/p523/`      | Corpus especializado P523 |

A paridade de linguagem está completa no corpus principal.

---

## Suites de teste — estado em P527

| Suite                | Resultado  | Detalhe                                      |
|----------------------|------------|----------------------------------------------|
| `parse_parity`       | ✓ 50/50    | markup, math, code                           |
| `eval_parity`        | ✓ 1/1      | 15 fixtures semânticas                       |
| `layout_parity`      | ✓ 1/1      | 46 ficheiros compilam                        |
| `structural_parity`  | ✓ todos    | P479/P480/P482–P488/P500/P501 ok             |
| `consolidado_p206d`  | ✓ 11/11    | visual 15/15 struct+text; markup 6/7 (error.typ intencional) |
| `vanilla_cli_smoke`  | ✓ 2/2      | CLI 0.14.x disponível                        |

Paridade P480 confirmada: **73 comparações, 0 diffs, 0 errors.**
