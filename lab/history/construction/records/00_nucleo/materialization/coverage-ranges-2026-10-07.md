# Prioridade2 — pacote completo preparado

Estado atual: pacote ratificado com “confirmo” e executado. A preparação original abaixo é histórica; três L0 instalados, código/testes materializados, selos e release validados. Relatório final: `00_nucleo/diagnosticos/coverage-ranges-execucao-2026-10-07.md`.

Contrato completo: `00_nucleo/materialization/coverage-ranges-2026-10-07/contract.md`. Três candidatos L0 completos e owners exclusivos em manifest.json. Manifesto SHA-256 `f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549`. A confirmação do pacote autoriza a instalação L0, RED→GREEN, adapter/fallback, builds/lint e medições finais sem novas paragens por detalhes privados.

Evidência: constructor e extrator permanecem byte-idênticos aos do experimento isolado, segundo `00_nucleo/diagnosticos/coverage-ranges-preparacao-2026-10-07/audit.json`. A melhora53.19→10.85ms é histórica e isolada; ganho do release atual ainda não medido.

Gate: `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` §2 — “PARAGEM OBRIGATÓRIA antes de escrever código (confirmar com o dono)” para contrato público. Há construtor público novo; este é o motivo exato da pausa. Todos os arquivos produtivos/L0/binário atuais foram preservados, conforme preservation.json.


Execução fechada no escopo ratificado: 7.941 testes aprovados, linter sem erros, 1.140 faces equivalentes, controle Hello ~27% mais rápido; números/proveniência/negativas em coverage-ranges-2026-10-07. Candidatos/contrato/manifesto originais conservados. Sem commit; prioridade 3 não implementada.
