# Replay físico sem cópias completas no watch

As interfaces públicas `discover`, `replay_shared` e `finish_replay`
foram aprovadas pelo mantenedor em 20 de novembro de 2026, conforme a
[ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md).

A descoberta de um documento paginado entrega o corpo preparado, o
trace e as resoluções seladas à indexação. O replay fornece uma
referência compartilhada ao mesmo produto físico enquanto a
estabilização lê páginas, posições e numeração. A publicação aceita
recupera a alocação original quando passa a deter a única referência
após liberar a entrada correspondente do memo. Quando há outro leitor, a publicação faz uma cópia defensiva.
A finalização só libera a entrada que possui o produto selecionado,
preservando qualquer produto mais recente. Bundle mantém os documentos
próprios para contar e particionar páginas.

A admissão, a verificação das dependências e a validação do snapshot
de paginação continuam no memo. Os limites de retenção e o caminho
que produz documentos próprios são compartilhados pelas mesmas
verificações.
[Experimento de descoberta](../../lab/results/construction/diagnostics/references-500-discovery-trace-2026-11-20/report.md)
· [Experimento de replay](../../lab/results/construction/diagnostics/references-500-shared-replay-2026-11-20/report.md)

## Validação e proveniência

Os testes RED registraram duas cópias completas em duas chamadas de
descoberta e em duas chamadas de replay. Na árvore integrada, passaram
os dois casos, incluindo a transferência do endereço original do vetor
de páginas, a mutação isolada após a cópia defensiva e a publicação de
um resultado antigo com outra entrada retida. Passaram 10.516 testes de
biblioteca, com sete ignorados, e o linter completo, incluindo V5, V15
e V26.
[RED](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/red.log)
· [GREEN](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/green.log)
· [Linhagem](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/lineage.log)

Os dois builds usaram Rust 1.92.0 e Cargo offline em `/dev/shm`, com
a mesma receita de release. O controle foi recompilado de uma cópia
congelada da integração anterior sobre
`6e3ce8bd5d3551ed64fcdfbe9ef6767448bef0d3`. Os hashes da fonte
coincidem com os da integração anterior. O ELF reconstruído na cópia
em RAM tem um SHA-256 próprio,
`04a362ef6102c577f0ee6b8c8bf0155d269d6a4510eda0d2c5c96662298899d7`.
O candidato foi compilado da árvore de trabalho integrada com as três
interfaces. Os recibos identificam a lista exata das mudanças, seus
hashes e os horários das transições.
[Controle congelado](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/control-source.json)
· [Candidato congelado](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/source-freeze.json)
· [Diffstat de controle](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/control-worktree-diffstat.txt)
· [Diffstat do candidato](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/candidate-worktree-diffstat.txt)
· [Receita em RAM](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/ram.sh)

## Watch de references_500

O candidato tem SHA-256
`383887193ad378aceed2f3da966aeb396957ed1560409f5a00fcd51adc4cb1ef`.
A medição alternou 24 blocos pareados com o controle recompilado,
o candidato e um alias do controle. A variação de CPU é a mediana das
razões pareadas, com intervalo bootstrap de 95%. A latência termina na
publicação do PDF.

| Ciclo | CPU mediana controle → candidato | Δ CPU e IC 95% | Δ latência |
|---|---:|---:|---:|
| Abertura | 218,18 → 195,68 ms | −10,12% [−12,51% a −8,98%] | −10,11% |
| Primeiro comentário | 179,92 → 164,77 ms | −8,23% [−10,87% a −6,93%] | −5,93% |
| Segundo comentário | 176,89 → 161,13 ms | −8,26% [−11,12% a −6,05%] | −5,76% |
| Edição visível | 188,74 → 175,40 ms | −7,24% [−8,15% a −6,61%] | −5,17% |

O A/A na abertura foi de −1,24% [−2,59% a −0,30%]. Os intervalos
do A/A incluíram zero nos outros três ciclos. Os controles direcionados
confirmaram posições, links, dependências indiretas, numeração,
sublayouts e a separação dos documentos em Bundle.
[Tempos](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/timing.json)
· [Recibo](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/benchmark-receipt.json)
· [Amostras](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/nominal.json.gz)
· [Controles](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/validation.json)
· [Bundle](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/bundle/verification.json)

Os 288 PDFs do watch corresponderam às compilações frescas. O censo
separado observou três publicações de cada release, com quatro
travessias físicas por publicação em ambos. As oito cópias completas
do memo por publicação caíram para zero. O candidato transferiu o
documento selecionado uma vez por publicação. Os artefatos de build
foram removidos do tmpfs ao final.
[PDFs verificados](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/verification.json)
· [Censo do controle](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/census-control.json)
· [Censo do candidato](../../lab/results/construction/diagnostics/references-500-shared-replay-integration-2026-11-20/census-candidate.json)
