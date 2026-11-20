# Reúso de layout em referências e no watch

O código produtivo incorpora a sequência cumulativa de quatro candidatos que
partiu de `b9914ad09`. Os prompts L0 descrevem os contratos em vigor. O
laboratório mantém o histórico de decisão, medições, controles e experimentos
inconclusivos.

| Mudança | Responsabilidade no código | Evidência no laboratório |
|---|---|---|
| Primeira travessia planejada com mapa anterior de rótulos sem leitores | A convergência observa o mapa até o fechamento, com PageStore injetado ou provisório. Uma leitura mantém o teste original de estabilidade. | [Store injetado](../../lab/results/construction/diagnostics/references-500-native-reuse-2026-11-19/report.md) e [store provisório](../../lab/results/construction/diagnostics/references-500-first-snapshot-2026-11-19/report.md) |
| Emissão do trecho preparado em nova baseline | O cache de pintura recompõe a coordenada vertical pela expressão original do item. A primeira palavra faz a admissão corrente. | [Emissão e fechamento](../../lab/results/construction/diagnostics/references-500-reused-lines-2026-11-19/report.md) |
| Reúso de métricas tipográficas | O provedor real retém bordas padrão e larguras shaped em tabelas limitadas sob o domínio imutável de fontes. | [Bordas de fechamento](../../lab/results/construction/diagnostics/references-500-reused-lines-2026-11-19/report.md) e [largura de item](../../lab/results/construction/diagnostics/references-500-item-width-2026-11-19/report.md) |

## Validação da integração

A comparação direta usou 24 blocos pareados no `watch` de `references_500`,
com o release anterior, um alias A/A e o build da árvore integrada. A
entrada tem 500 parágrafos gerados, 20 títulos e 20 referências.

| Ciclo | CPU mediana anterior → integrada | Δ CPU pareado (IC 95%) |
|---|---:|---:|
| Inicialização | 316,01 → 213,40 ms | −32,59% (−33,41% a −31,85%) |
| Comentário | 274,80 → 183,35 ms | −33,28% (−33,86% a −32,83%) |
| Edição visível | 288,14 → 197,45 ms | −31,49% (−31,98% a −30,26%) |

No comentário, a latência até publicar o PDF caiu de 347,81 para 256,61 ms.
O A/A teve mediana pareada de +0,27% de CPU (−0,31% a +1,04%).
Passaram o build release, o lint, 10.512 testes de biblioteca e 266
comparações de PDF, incluindo 28 verificações de raster. Os controles
de erro preservaram o diagnóstico, o último PDF e a recuperação.

A base foi `b9914ad09a078f1c71c2068cf1fad97255de8a9b`. Os SHA-256 dos ELFs
são `69eede6e0762e654a671fd8fe5e13b6f15bae1663bc14672f6a10ee9de009ec2`
(release anterior) e `9f487ff533af92476647ccf4c017df3698eb215ba1b96e1d5ef012e4cf1b215e`
(build integrado). As medições e os recibos completos ficam no
[relatório local da integração](../../lab/results/construction/diagnostics/references-500-integration-2026-11-19/report.md).

A [projeção genérica de largura](../../lab/results/construction/diagnostics/line-commit-generic-width-2026-11-18/report.md)
e a [redução das varreduras de fechamento](../../lab/results/construction/diagnostics/line-commit-scans-2026-11-18/report.md)
continuam como evidência experimental no laboratório local.
