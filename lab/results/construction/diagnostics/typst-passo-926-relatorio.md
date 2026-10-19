# Relatório P926 — Opção 5 (thread de fundo para coverage) não resolve o outlier UTF-8

**Precede este passo:** `typst-passo-925-relatorio.md` — diagnóstico que recomendou
Opção 1 (pré-computar coverage no arranque), mas não mediu o custo no caso comum.
**Objetivo deste passo:** medir o custo no caso comum da Opção 1 e testar a
Opção 5 (thread de fundo) como alternativa híbrida.

**Data:** 2026-07-28.
**Commit base:** `da18ea9f3` (P922–P924 integrados).

---

## Resumo executivo

A Opção 5 (thread de fundo que pré-computa a cobertura Unicode, com
`candidates_for_char` a bloquear só se for chamado antes da thread terminar)
**não resolve o outlier `05-utf8`**.

- **Caso comum:** não há regressão significativa (rácios proto/original de
  0.98× a 1.06× nos 7 cenários canônicos). Isto é muito melhor do que a
  Opção 1, que regredia ~1.12×–1.48×.
- **UTF-8 / CJK / emoji:** nenhuma melhoria mensurável (rácios ~1.00×).
  O motivo é que o primeiro caractere que exige fallback aparece antes de a
  thread de fundo ter terminado a pré-computação, pelo que `candidates_for_char`
  bloqueia pelo tempo total do scan — exactamente o mesmo comportamento do
  caminho lazy original.

**Decisão:** reverti o protótipo e a actualização do L0 para não deixar
complexidade de threading sem ganho comprovado.

---

## Metodologia

- **Binário original:** `target-original/release/typst`, SHA-256
  `87c535f3772e0cf2c8038a5851b8c589ba98707c76d4cd399e2b5742a753a1d5`
  (compilado a partir do worktree `typst-crystalline-original` no commit
  `da18ea9f3`).
- **Binário protótipo:** `target/release/typst` com a Opção 5 implementada em
  `03_infra/src/world.rs`.
- **Cenários canônicos:** os 7 documentos da frente (P872–P921), comparados
  `depois/antes`.
- **Cenários UTF-8:** `05-utf8.typ` e 4 inputs de bloco Unicode isolado
  (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`).
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 para canônicos; warmup 2 /
  min-runs 10 para UTF-8 (porque cada run demora vários segundos).
- **Atestação:** `tools/perf/results/p926-canonical/attestation.json` e
  `tools/perf/results/p926-utf8/attestation.json`.

---

## Fase A — impacto no caso comum (7 cenários canônicos)

| Cenário | original (ms) | proto Opção 5 (ms) | rácio |
|---|---:|---:|---:|
| 01-hello | 90.68 | 88.67 | 0.98× |
| 02-lorem | 111.09 | 114.22 | 1.03× |
| 03-images | 96.53 | 95.84 | 0.99× |
| 04-math | 149.90 | 156.67 | 1.05× |
| 05-tables | 92.52 | 92.70 | 1.00× |
| 06-long | 295.23 | 304.06 | 1.03× |
| 07-context | 130.64 | 138.20 | 1.06× |

**Conclusão:** o caso comum não regrediu de forma prática. A variação está na
banda de ruído (~0.98–1.06×), muito abaixo da regressão de ~1.12–1.48× medida
para a Opção 1 na primeira fase deste passo.

---

## Fase B — impacto nos casos UTF-8

| Cenário | original (ms) | proto Opção 5 (ms) | rácio |
|---|---:|---:|---:|
| 05-utf8 | 7624.60 | 7686.27 | 1.01× |
| utf8-latin | 88.94 | 88.65 | 1.00× |
| utf8-greek | 88.89 | 90.14 | 1.01× |
| utf8-cjk | 6959.74 | 6958.99 | 1.00× |
| utf8-emoji | 7572.11 | 7575.87 | 1.00× |

Os timings de `05-utf8` confirmam que o tempo continua todo em `layout_ms`:

| | original | proto Opção 5 |
|---|---:|---:|
| `layout_ms` | 6082.95 ms | 6132.71 ms |
| `shape_ms` | 480.24 ms | 481.48 ms |
| `total_ms` | 6841.85 ms | 6904.63 ms |

A Opção 5 não deslocou o custo do scan para fora do caminho crítico.

---

## Fase C — por que a Opção 5 não ajudou

No input `05-utf8`, o primeiro caractere que exige fallback (CJK no primeiro
parágrafo) é encontrado durante o layout, pouco depois de a `SystemWorld` ser
construída. A thread de fundo precisa de ~1.6 s para percorrer as ~1112 faces
do sistema; quando `candidates_for_char('你')` é chamado, a thread ainda não
terminou, pelo que o método faz `join()` e o caminho principal espera pelo
tempo total do scan.

Resultado: o tempo total é essencialmente igual ao original — a diferença é
apenas a pequena sobrecarga de criar e gerir a thread.

A Opção 5 só ajudaria em documentos onde o primeiro fallback ocorresse **depois**
de a thread já ter terminado (ex.: um documento longo em latim com um caractere
CJK só no fim). O outlier em questão não é esse caso.

---

## Fase D — opções seguintes

| Opção | Custo no caso comum | Benefício no UTF-8 | Notas |
|---|---|---|---|
| **Opção 1** (pré-computar no arranque) | ~+1.6 s por documento | ~8.6 s → ~0.2 s | Resolve o outlier, mas regredia 1.12–1.48× nos canônicos. |
| **Opção 5+ paralelização** | pequeno / nenhum | reduz scan de ~1.6 s para fração | Usar várias threads para parsear as fontes em paralelo; não elimina o bloqueio, mas pode reduzi-lo. |
| **Opção 6** (precompute só se necessário) | 0 para latim puro | ~0.2 s + scan quando há CJK/emoji | Escanear o source antes do layout; só disparar scan se encontrar blocos não cobertos pela fonte primária. |
| **Opção 7** (cache em disco) | 0 após primeira execução | ~0.2 s após cache quente | Serializar `coverage_cache` entre processos; primeira execução ainda lenta. |

A Opção 6 parece a mais promissora para o caso concreto: evita o custo nos
documentos sem fallback e resolve o outlier nos documentos com fallback,
pagando o scan uma única vez por compilação.

---

## Fase E — estado da árvore

- Protótipo Opção 5 implementado e medido, depois **revertido**:
  - `03_infra/src/world.rs` voltou ao estado de `da18ea9f3`.
  - Actualização ao L0 `00_nucleo/prompts/infra/system-world.md` revertida.
- Artefactos de medição mantidos:
  - `tools/perf/benchmark-p926-canonical.py`
  - `tools/perf/benchmark-p926-utf8.py`
  - `tools/perf/corpus/p923/utf8-{latin,greek,cjk,emoji}.typ`
  - `tools/perf/results/p926-canonical/attestation.json`
  - `tools/perf/results/p926-utf8/attestation.json`
- `target-original/` e worktree `typst-crystalline-original` removidos.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente, `package_version_resolution.md`).
- `cargo check -p typst-infra`: ok (apenas warnings pré-existentes).

---

## Proveniência

- Commit base: `da18ea9f3`.
- Binário original: `target-original/release/typst`, SHA-256
  `87c535f3772e0cf2c8038a5851b8c589ba98707c76d4cd399e2b5742a753a1d5`.
- Binário protótipo: `target/release/typst`, SHA-256
  `a04a66639b898e31ca2213365579972d4d094ec538b1ded4e08e2552245aa4bc`
  (após reversão, o SHA deixa de ser relevante).
- Scripts: `tools/perf/benchmark-p926-canonical.py`,
  `tools/perf/benchmark-p926-utf8.py`.
- Atestações: `tools/perf/results/p926-canonical/attestation.json`,
  `tools/perf/results/p926-utf8/attestation.json`.
