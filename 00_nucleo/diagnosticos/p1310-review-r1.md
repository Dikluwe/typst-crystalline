# P1310 — veredito independente final

**Aprovado no fragmento especificado, com limitação temporal A/B explícita.**
O recibo canônico é `p1310-verification.json`; hashes, UTC, HEAD e diff/stat
estão em `p1310-review-evidence.json`.

O achado R0 foi resolvido sem alterar produto ou L0: R2 mudou somente a
política e os envelopes de 15 casos named-prefix, usando os dados baseline
pré-candidato. Conferi todos os campos alterados e a igualdade dos demais
casos e fixtures. A classificação R2 aconteceu depois do candidato; a
entrega não a apresenta como congelamento integral anterior à implementação.
Esse limite é aceitável para o regime A/B proporcional declarado, sem selo
do protocolo completo nem atestação técnica de isolamento.

Recalculei as 1036 células de cada uma das três ordens: todas satisfazem suas
políticas, sem Unknown ou instabilidade. O replay P1308 preserva 1962
envelopes e muda exatamente vinte diagnósticos dos cinco decoders, agora
iguais ao vanilla pinado. O diff usa o tipo público canônico e value_span
da primeira ocorrência posicional; preserva detached e a ordem de validação.
Os gates de build, RED→GREEN, workspace, fmt, linhagem e diff-check passaram.
Linter: zero erros, 240 warnings, 1136 infos; workspace: 6624 testes aprovados,
zero falhas, três doctests ignorados.

Symbol e as demais exclusões permanecem explicitadas no veredito. Não há
achado acionável pendente neste recorte. Não se atesta paridade geral,
ausência de dívida de mutação ou isolamento do filesystem compartilhado.
