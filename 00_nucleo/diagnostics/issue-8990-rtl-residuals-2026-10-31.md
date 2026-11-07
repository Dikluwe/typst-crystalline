# Resíduos RTL após #8990 — correção interna e gate pendente

Em 2026-10-31, na branch compartilhada `Tekt`, HEAD
`3c6b0ce7b7632cc74b004cea6681e4746e63d28d` mais a árvore de trabalho concorrente,
foi corrigida a direção LTR/RTL explicitamente declarada dentro de `math.op`.
O classificador preserva `text.dir` local e o shaper usa a base já resolvida
na medição/emissão. Direção ausente mantém o algoritmo automático anterior.
Não houve novo contrato público, fase, catálogo de fontes, commit ou push.

L0 precedeu o código, com RED funcional e refutação renderizada da primeira
candidata. No estado final congelado passaram 466 testes de matemática,
75 do shaper, build em RAM, lint/lineage, fmt e diff-check. A matriz final
verifica 24 direções internas e preserva fisicamente 42 casos sem override.
Seis diferenças de contorno/avanço no ponto de operadores hebraicos RTL
permanecem idênticas ao baseline; paridade física completa não foi declarada.

Continuam pendentes de aprovação humana, por alterarem defaults:

- A: direção `auto`/ausente da prosa derivada do idioma efetivo, com overrides
  LTR/RTL explícitos e restauração lexical preservados. Exigência documentada.
- B: base local LTR no operador matemático sem override, preservando runs
  hebraicos RTL e prioridade da direção interna. Decisão adicional de produto,
  não inferida automaticamente do oracle.

Nenhum código de A/B foi implementado. A parada foi solicitada expressamente
na tarefa e segue ADR-0127 §2. O caso original #8990 com RTL exterior permanece
correto. Mudanças concorrentes foram preservadas; target/TMPDIR exclusivos em
RAM, um job, offline/locked, sem incremental/debuginfo e limite de 12 GiB.

[Relatório e proveniência completa](../../lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/report.md).
[Proposta concreta de L0 para o gate](../../lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/gate-default-direction-proposal.md).
Os recibos fixam HEAD, diffstats exatos, fontes, instantes UTC e hashes dos
binários e pares próprios; os negativos intermediários continuam preservados.
