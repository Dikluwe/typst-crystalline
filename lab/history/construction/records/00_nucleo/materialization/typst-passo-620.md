---
# P620 — Bissecção da regressão de `macro-10x` entre P518 e P543

> **Passo:** 620
> **Data:** 2026-07-05
> **Foco:** P546 encontrou que o cristalino era 3,3 vezes **mais rápido** que o vanilla no `macro-10x` em P518 (rácio 0,30×), e 11,52 vezes **mais lento** já em P543 — uma inversão nunca investigada, explicitamente registada como "fica para bissecção futura dedicada". Isso nunca aconteceu. P619 confirmou que `shape_ms` domina hoje (90,66%), mas não respondeu porque é que o custo de shaping (ou outra coisa) mudou tanto entre P518 e P543. Este passo faz a bissecção que ficou por fazer.
> **Tipo:** Sonda directa, com bissecção de commits.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Uma pergunta registada como "para investigação futura" há muitos passos não desaparece por os relatórios seguintes não a repetirem.

---

## Contexto

| Momento | Rácio `macro-10x` | Nota |
|---|---|---|
| P518 | **0,30×** (cristalino mais rápido) | Baseline histórico, medido com `--warmup 0 --runs 1`, a frio. |
| P543 | 11,52× (cristalino mais lento) | Primeira medição onde a inversão já estava presente. |
| P546 | Achado registado, bissecção nunca feita | "fica registada para bisecção futura dedicada" |
| P619 (agora) | ~6,78× | `shape_ms` confirmado como 90,66% do tempo, mas sem explicação para a mudança desde P518. |

A diferença entre P518 e P543 é uma janela de tempo grande — muitos passos aconteceram nesse intervalo (Trilha 5 completa, shaping RTL inicial, subsetting, CFF, Variation Fonts). A causa pode ser um único passo, ou uma soma de vários.

---

## Sonda

### Localizar os commits de fronteira

```bash
git log --oneline | grep -i "P518\|P543" | tail -10
```

### Bissecção manual, com o mesmo documento e método

```bash
# Lista de commits entre P518 e P543, em ordem cronológica
git log --oneline <hash-P518>..<hash-P543-ou-antes> -- 03_infra/src/shaper.rs 03_infra/src/font_metrics.rs 01_core/src/engine/layout/ | tac
```

Para cada commit candidato (ou usando `git bisect` de forma mais sistemática, se houver um critério automatizável — por exemplo, um script que mede `shape_ms` para `macro-10x` e falha se exceder um limiar):

```bash
git bisect start
git bisect bad <hash-P543>
git bisect good <hash-P518>
# em cada passo do bisect:
cargo build --release --bin typst
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p620-bisect.pdf --timings-json /tmp/p620-timings.json
python3 -c "import json; d = json.load(open('/tmp/p620-timings.json')); print(d['shape_ms'])"
# decidir good/bad conforme o valor, com um limiar razoável (por exemplo, 2x o valor de P518)
```

### Critério de fecho da sonda

- [ ] Commit (ou commits) exacto identificado, onde `shape_ms` para `macro-10x` cresce significativamente.
- [ ] Confirmado se é um único passo, ou uma soma de vários com contribuições menores.
- [ ] Razão da mudança compreendida — por exemplo, se P518 media com um caminho de código que evitava algum trabalho que passou a ser feito sempre a partir de determinado passo.

---

## Decisão

Se a causa for uma regressão real, não intencional: decidir se vale a pena corrigir agora, seguindo o mesmo método de sonda-causa-correcção já estabelecido, ou se o custo actual (confirmado por P619 como estável desde então, sem continuar a piorar) é aceitável dado o trabalho que se ganhou entretanto (shaping correcto para árabe, subsetting, etc.).

Se a causa for uma mudança de metodologia de medição (por exemplo, P518 media a frio, com caches de disco/SO ainda não quentes, e isso inflacionava artificialmente a vantagem do cristalino) — já foi sugerido isto em P546 como possível explicação parcial, mas nunca confirmado directamente. Testar essa hipótese também.

---

## Critério de fecho do passo

- [ ] Bissecção feita, causa localizada ou hipótese de metodologia confirmada/refutada.
- [ ] Decisão registada — corrigir, aceitar com razão, ou reclassificar a medição de P518 como não comparável por metodologia diferente.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p620.md`, com hash do commit.
- [ ] A pergunta registada em P546 finalmente fechada, não deixada para mais um passo "futuro" que nunca vem.
