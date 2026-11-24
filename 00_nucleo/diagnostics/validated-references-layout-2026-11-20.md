# Reúso de paginação física e largura de linha no watch

| Mudança | Responsabilidade no código | Evidência no laboratório |
|---|---|---|
| Reúso entre snapshots de paginação | Convergência instala o observador. O memo revalida número lógico efetivo, identidade do plano, métricas e introspecção. Referências de página e numeração funcional exigem o snapshot inteiro. | [Leituras de paginação](../../lab/results/construction/diagnostics/references-500-page-read-replay-2026-11-20/report.md) |
| Reúso da passagem inicial sem paginação injetada | Convergência certifica a primeira passagem após concluir os callbacks e a cobertura. A aceitação do memo ainda depende das respostas observadas. | [Prosa numerada e contraprova em referências](../../lab/results/construction/diagnostics/references-500-first-unseeded-replay-2026-11-20/report.md) |
| Reúso de largura no fechamento de linha | A emissão entrega ao próximo `Parbreak` a identidade da cauda pintada. O fechamento combina a largura retida sob a mesma época de fontes com as medições atuais do prefixo e da borda. | [Identidade da cauda](../../lab/results/construction/diagnostics/references-500-line-identity-2026-11-20/report.md) |

Na primeira transição de `references_500`, a consulta contextual
`counter_values_at` muda e impede o reúso da passagem inicial. O mesmo
caminho foi favorável na prosa numerada sem essa dependência. A retenção
da cauda recupera a largura horizontal pela identidade do produto de
pintura. O fechamento continua executando os efeitos de estilo e fluxo.

O [replay sem cópias](../../lab/results/construction/diagnostics/references-500-shared-replay-2026-11-20/report.md)
mediu ganho próprio. A aprovação das interfaces e a medição posterior
estão no [diagnóstico do replay compartilhado](validated-shared-physical-replay-2026-11-20.md).
As extensões verticais, incluindo a
[borda inferior](../../lab/results/construction/diagnostics/references-500-tail-line-bottom-2026-11-20/report.md),
ficaram no laboratório, onde as reduções de chamadas ainda não sustentaram
um ganho isolado de CPU.

## Validação do código integrado

Os dois testes falharam antes da implementação, respectivamente por repetir a
travessia mesmo com a numeração equivalente e por recalcular a largura da
cauda certificada. Na árvore integrada, passaram os 10.514 testes de
biblioteca, com sete ignorados. O build release e o linter completo passaram,
incluindo V5, V15 e V26. O Cargo 1.92.0 executou offline, com os artefatos e
temporários em `/dev/shm`.

O build usou a árvore de trabalho sobre
`6e3ce8bd5d3551ed64fcdfbe9ef6767448bef0d3`. O controle é o ELF da
integração de 19 de novembro, com SHA-256
`9f487ff533af92476647ccf4c017df3698eb215ba1b96e1d5ef012e4cf1b215e`.
O ELF desta integração tem SHA-256
`1c5801a124268bed47d56214f85a949ad339f4dc7c5e6d4a94a4bad5d39e49be`.
A árvore foi congelada em 20 de novembro às 23:48 UTC antes dos builds.
Os horários exatos, a lista e os hashes dos arquivos alterados, os logs
e a receita em RAM estão no diretório de evidência.
[Fontes medidas](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/source-freeze.json)
· [Diffstat da árvore](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/worktree-diffstat.txt)
· [RED](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/red.log)
· [GREEN](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/green.log)
· [Receita](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/ram.sh)

## Watch de references_500

A comparação direta usou 24 blocos pareados com a integração anterior como
controle e um braço A/A executando o mesmo ELF do controle. A CPU é a
mediana das variações pareadas, com intervalo bootstrap de 95%. A latência
vai até a publicação do PDF.

| Ciclo | CPU mediana antes → depois | Δ CPU e IC 95% | Δ latência |
|---|---:|---:|---:|
| Abertura | 236,59 → 220,56 ms | −6,57% [−8,76% a −5,28%] | −6,48% |
| Primeiro comentário | 206,21 → 184,43 ms | −11,60% [−14,32% a −7,30%] | −8,35% |
| Segundo comentário | 204,37 → 181,47 ms | −10,59% [−13,25% a −9,32%] | −7,80% |
| Edição visível | 220,94 → 195,27 ms | −11,87% [−14,19% a −9,16%] | −9,11% |

Os quatro intervalos do A/A incluíram zero. Os resultados quantificam o
efeito do conjunto integrado. O ganho individual de cada mudança foi
medido nos experimentos da seção inicial, com os controles respectivos.
[Tempos](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/timing.json)
· [Recibo](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/benchmark-receipt.json)
· [Amostras](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/nominal.json.gz)

Os 288 PDFs do watch corresponderam às compilações frescas. Passaram
também 28 comparações direcionadas de posição, links, alteração em
dependência indireta, paginação e sublayouts. Grid, Box, Place e Columns
foram exercitados com o mesmo documento. Bundle preservou os dois PDFs,
a numeração local e os rótulos de página.
[Verificação nominal](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/verification.json)
· [Casos direcionados](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/validation.json)
· [Bundle](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/bundle/verification.json)

Na prosa numerada, 12 blocos pareados mediram reduções de CPU de 19,02%
na abertura, 23,25% e 18,22% nos comentários e 22,07% na edição visível.
Os quatro intervalos de CPU ficaram abaixo de zero e os intervalos A/A
incluíram zero. Os 144 PDFs corresponderam às saídas frescas.
[Prosa numerada](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/numbered/summary.json)

Após salvar os resultados, o Git excedeu o limite de endereçamento do
executor de benchmark ao mapear um pack. A conferência final foi repetida
com janelas de mapeamento menores, sob o mesmo limite, e confirmou
os hashes da árvore congelada. Os artefatos da compilação foram removidos
do tmpfs após a execução.
[Verificação final da árvore](../../lab/results/construction/diagnostics/references-500-integration-2026-11-20/post-run-source-verification.txt)
