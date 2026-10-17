# Diagnóstico e proposta — grupos Table — 2026-10-17

Etapa de medição/proposta concluída. **A implementação aguarda aprovação ADR-0127.** Nenhum arquivo produtivo, L0 vigente, kernel vigente ou configuração mudou nesta etapa. O objetivo geral e os **131 V16** permanecem abertos.

[Revisão para aprovação](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/review.md) · [API e migração](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/api.md) · [R1–R20](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/table-row-groups-2026-10-17/test-plan.md).

## Resultado

A coleta tem 105 casos bilaterais e 210 invocações. Revelou a causa documental/estrutural das divergências de spans e dos níveis: fontes separadas perdem a ordem e os metadados; padding por contagem não é ocupação; validação estrutural e receitas precisam de fronteiras distintas. O contrato anterior sobre conteúdo válido ocultado não provava que estrutura inválida poderia ser ignorada. O pin refuta isso com colspan, duplicação, footer e show-set.

Presença lexical e identidade também estavam incompletas: raw Header/Footer/linhas perdem fields/func; Auto era promovido antes de ser necessário. Índices x/y de linhas são ignorados; None de stroke deixa tinta existente e explicit line ignora extensão de overhang. Todos os inputs/stdout/stderr/argv/binários/fontes/UTC estão em measurement/supplement/lines/styles-phase/metadata/svg .json.gz e seus arquivos de casos. Não chamar cada diferença sintática de um defeito: tbody opcional do controle é identificado na revisão.

## Proposta concreta

A revisão contém a nova fonte única, máscaras/índices tipados, plano por intervalos, transição pending/completo, fronteira de validação antes da Table e realização de Cells sobreviventes depois. São 53 L0s candidatos, 34 consumers de lógica/testes e 19 de pin somente, núcleo compartilhado refinado e um novo núcleo dependente. Tipos/constantes/método públicos e a migração estão descritos antes de qualquer alteração de API. O grafo candidato passou V1/V5/V15/V26 em cópia temporária. Isso não é GREEN funcional: código novo e R1–R20 ficam para depois da aprovação.

Permanecem RG2/table-root-fields, RG3/group-line-style-defaults, rich properties/gutters, repetição/semântica multipágina/PDF-UA, HTML Grid, math-stretch-composite, matching-local-cjk e matching-identical-rule-precedence. O objetivo inteiro não foi reduzido ao RG1.

## Proveniência e recursos

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC/status/git diff HEAD --stat/fontes/L0/hashes em before.json.gz e proposal-base.json.gz. Binário oracle SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin 9f2b6e8715237cb086899a42873660fe744622e8; as quatro fontes locais consultadas conferem com objetos Git desse commit (reference-source-hashes.json). SHA do Tekt por rodada está nas coletas; não usar --version como prova.

MemoryMax=12 GiB, swap zero, Cargo um job; execuções sequenciais, pico observado 486862848 bytes (0.453 GiB), nenhum OOM. Contadores em *-resources.json. Não houve novo build de produção ou repetição da suíte, pois a raiz ficou intacta; a suíte anterior de 8742 testes continua histórica para o mesmo freeze. Sem commit/push/merge/release/pin CI novo.
