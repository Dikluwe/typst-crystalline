# P1138 — baseline de paridade profunda

## Proveniência

- Medição: `2026-08-23T21:29:41.436043+00:00`.
- HEAD: `781b207b4a5de9c2bfbe5819918a193d1d9293e5`.
- Working tree: não commitada; `git diff HEAD --stat` no instante da medição:
  `271 files changed, 12130 insertions(+), 4697 deletions(-)`, além de
  ficheiros não rastreados.
- Vanilla: `upstream/main a51e02804`, binário
  `lab/typst-original/target/release/typst`.
- Cristalino: `target/release/typst` construído da árvore acima.
- Reprodução: `python3 lab/parity/matrix/runner.py --output
  /tmp/p1138-results-final.json`.

## Resultado

Denominador explícito: **19 casos — 12 MATCH, 7 DIFF, 0 ABSENT, 0 PARTIAL,
0 ERROR e 0 UNMEASURED**.

Lista integral não-MATCH:

| ID | Estado | Classe | Medição |
|---|---|---|---|
| `P1138-S-001` | DIFF | PUBLIC_DIAGNOSTIC | sintaxe: vanilla usa bloco fonte/caret; cristalino usa linha gcc |
| `P1138-S-002` | DIFF | PUBLIC_DIAGNOSTIC | nome desconhecido: quoting e forma da mensagem divergem |
| `P1138-S-003` | DIFF | PUBLIC_DIAGNOSTIC | cristalino não emite o trace público `while including` |
| `P1138-X-001` | DIFF | LANGUAGE_MORPHOLOGY | árvore SVG diverge em agrupamento, fundo, precisão e paths |
| `P1138-L-001` | DIFF | LANGUAGE_MORPHOLOGY | largura PDF: 595,276 pt vs 595,280 pt; altura e páginas coincidem |
| `P1138-X-002` | DIFF | PUBLIC_FORMAT | 168/2.005.644 pixels diferem; delta máximo 1 por canal |
| `P1138-X-003` | DIFF | PUBLIC_FORMAT | texto/páginas coincidem; caixa e nome da fonte PDF divergem |

## Resultados positivos profundos

- `P1138-B-001`: DTO tipado composto coincide, incluindo `none`, bool, int,
  float, string, array e dict.
- `P1138-X-004`: árvore HTML de texto/parágrafo simples coincide evento a
  evento.
- Os dez casos P1137 permanecem MATCH.

## Tolerâncias e artefactos

Geometria é arredondada por campo a 0,001 pt; a diferença de 0,004 pt continua
visível e portanto não foi tolerada. Raster usa igualdade RGBA exata nesta
baseline e emite `/tmp/p1138-results-final-artifacts/P1138-X-002/pixel-diff.png`.
O relatório registra também `different_pixels=168`, `max_channel_delta=1` e
`mean_channel_delta=0,00006282271430024472`. Estes números vêm do estado e da
hora acima; não fecham a classificação como bug de língua sem investigação.

## Limitações honestas

- Typst não expõe reflexão pública de parâmetros (`calc.gcd.params()` e
  `text.params()` falham nos dois binários). Logo o inventário integral de
  callables/args/defaults não pode ser inferido por esse caminho; P1139 deve
  combinar extração de metadados com probes observáveis, sem tratar nomes Rust
  como paridade.
- A árvore SVG profunda inclui paths de glifos; o próximo passo deve separar
  agrupamento/morfologia de diferenças puramente serializacionais.
- O extrator PNG suporta PNG 8-bit RGB/RGBA, os formatos produzidos pelos dois
  binários nesta baseline; outro color type é erro de harness explícito.

## Gates

- 16 testes do runner: PASS.
- Manifesto: válido, 19 casos.
- Runner integral: PASS; todas as expectativas coincidiram.
- Suíte Cargo de `lab/parity`: PASS.
- `crystalline-lint .`: PASS, zero violações bloqueantes.
- `git diff --check`: PASS.

P1138 mede e prioriza; nenhum ficheiro L1–L4 foi alterado por este passo.
