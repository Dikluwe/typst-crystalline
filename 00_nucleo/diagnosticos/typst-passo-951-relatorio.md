# Passo 951 — Relatório (`compare.py`: variância e classificação de padrão por secção)

**Data**: 2026-08-01
**Estado da árvore**: commit `8dccb0636` (inclui este passo, `580aea6d0`).

---

## 1. O que foi pedido e o que foi feito

O dono identificou uma limitação metodológica: a **mediana** de deltas, sozinha, não
distingue uma **regra de layout diferente** (padrão repetido: linhas simples ~0,
multi-linha +7 a +17pt consistentes) de um **bug pontual** (um outlier grande
arrastando a estatística). Este passo estendeu `tools/geometry/compare.py` para
reportar, por secção:

- **`padrao`**: classificação `sistemático` / `pontual` / `indeterminado`;
- **`cv`**: coeficiente de variação dos |delta| (contexto numérico adicional).

A mediana e o máximo existentes **mantêm-se** — a classificação é adicional, não
substitui (como o passo pede).

## 2. A heurística (triagem, não veredicto)

Implementada em `classify_section(dxs, dys, limiar)`
(`tools/geometry/compare.py`):

- **`sistemático`**: existe um **cluster apertado** (bins de 1pt, desvio interno
  < 1pt) de deltas com centro ≥ `max(2pt, 4×limiar)`, cobrindo ≥ 8% dos glifos da
  secção, em qualquer eixo (x ou y) — assinatura de regra diferente entre os
  motores (ex.: espaçamento vertical de equações multi-linha; conteúdo extra
  deslocando a secção inteira).
- **`pontual`**: mediana < limiar e um número pequeno de outliers (mediana +
  6×MAD, ≤ 5% dos glifos) — assinatura de bug localizado.
- **`indeterminado`**: limpa, ou misto sem assinatura clara.

**Agrupamento automático por tipo de construção** (equação de uma linha vs
multi-linha): avaliado na Fase A e **não implementado** (over-engineering para
triagem — a distinção cluster/variância já captura o efeito que o dono separou
manualmente; fica registada como extensão possível se a triagem precisar dela).

## 3. Validação

Casos de referência (todos correctos):

| Caso | Resultado esperado | Obtido |
|---|---|---|
| Distribuição sintética: 50 deltas ~0 + 1 outlier de 12pt | `pontual` | `pontual` |
| Distribuição sintética: 40 ~0 + cluster apertado em ~10.5pt | `sistemático` | `sistemático` |
| Distribuição dispersa | `indeterminado` | `indeterminado` |
| Auto-comparação (`test_crystalline.pdf` vs si próprio) | tudo `indeterminado` | 30/30 `indeterminado` |
| Par actual (espaçamento multi-linha, motivador de P952) | `sistemático` nas secções afectadas | ✓ |
| Par P943 (defeitos de então) | padrão forte nas secções defeituosas | ✓ |

Duas calibragens durante a validação, registadas para quem mantiver a heurística:

1. A primeira versão (CV simples sobre a secção inteira) **não disparava** nos
   casos reais — a assinatura sistemática é **bimodal** (zeros + cluster
   não-zero), que CV global não captura. Daí a detecção por cluster.
2. Clusters com centro < 2pt são ruído de triagem (disparavam em secções sãs) —
   gate de magnitude no centro do cluster.

**Registo** (pedido do passo): o emparelhamento por posição desambiguou `(1)` do
rótulo de equação vs `(1)` no texto corrido — o desenho da ferramenta já cobria;
funcionou como pretendido, sem alteração.

## 4. Spec actualizada

`tools/geometry/README.md` — nova secção "P951 — variância e classificação de
padrão por secção", com a definição das três classes, o carácter de triagem (não
veredicto) e os casos de validação de referência.

## 5. Estado

- Código: `tools/geometry/compare.py` (coluna `padrão` na tabela, campos
  `padrao`/`cv` no JSON, contagem agregada no fim).
- Commit: `580aea6d0` ("feat(tools): compare.py classifica padrão da secção
  (sistemático/pontual/indeterminado) — Passo 951").
- Este passo não tocou o compilador — sem suite/benchmark aplicáveis (ferramenta
  de tooling, mesmo padrão de `tools/perf/`).
