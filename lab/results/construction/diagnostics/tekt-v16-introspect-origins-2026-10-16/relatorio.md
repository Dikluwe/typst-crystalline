# V16 — origens, extração e Source

Etapa concluída; objetivo geral continua aberto.

## Resultado

**V16: 240 → 239.** Além da emissão corrigida em extract_payload, foram fechados os casos latentes T027 e T158/T160 e três predicados com fallback implícito. São seis decisões protegidas contra variantes futuras: uma sobre Value e cinco sobre Content. Os IDs T158/T160 pertencem ao mesmo ponto após a união anterior e não foram contados como dois matches novos.

A auditoria também confirmou e corrigiu falha real: Source(Heading/Strong/Emph/Metadata) perdia o payload do body, e Located(Source(Heading)) não qualificava como origem preparada. Normalizar pelo source_body existente restaura o núcleo de origem transparente. needs_fresh_index também normaliza Source, conservando o fallback das classes especiais. Nenhuma Location, payload próprio de Source, callback adicional, API, modo padrão ou fase nova foi criado. A coleta conserva as Locations já preparadas sem duplicação, inclusive no original de Realized.

V19 **892 → 900 notas**, somente os oito grupos uniformes previstos nos L0. V20 permanece **871 notas**. Não houve supressões, exceções globais ou mudança de configuração. [Lista de correções e grupos](lista-correcoes.md) · [patch completo da etapa](stage.patch) · [pendências](lista-pendencias.md).

## Verificação

- RED funcional: **cinco falhas e seis controles passando** antes da implementação, em [baseline-source-red.log](baseline-source-red.log). GREEN: **11 passaram**, em [validation-controls.log](validation-controls.log).
- Os testes existentes single_evaluation_ passaram antes/depois, preservando execução única e conteúdo/introspecção de origem/produto.
- Sentinela Value no workspace real: barreira selecionada **0 → 1 E0004**, total **28 → 29**; sentinela Content separada: **0 → 5 E0004**, total **17 → 22**. Sem outros erros, sem mutação dos fontes produtivos; entradas, logs e resultados de cada sonda foram preservados.
- Suíte completa: **8651 passaram, zero falhas, 7 ignorados preexistentes**, 11 testes a mais que o último baseline completo. [Resultado workspace](workspace/workspace-result.json).
- Build, fmt, Clippy all-targets com warnings/expectativas não cumpridas como erro, Rustdoc workspace com -D warnings, V1/V5/V15/V26 estritos, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json).
- [architecture-delta.json](architecture-delta.json) compara todos os resultados SARIF, preservando mensagens, níveis, multiplicidade e posições após mapear deslocamentos de linha. Remove somente o V16 selecionado e adiciona somente oito V19. O gate global continua exit 1 pelos 239 V16 pendentes.

## Refinamentos e integridade

O guard de resselo impediu aplicar um pin bruto de bytes do TOML em vez do digest canônico; a tentativa ficou em cópia descartável, com o log preservado. O novo consumer pin de source-origin usa o digest canônico **4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67**. O arquivo do núcleo não mudou; seu SHA dos bytes é distinto e consta no snapshot. V26 valida o grafo final.

A primeira captura other @ (grupo nominal) acrescentava profundidade V20 sem necessidade. [flattening-before.json.gz](flattening-before.json.gz) e preview-architecture.log preservam a medição; o L0 foi refinado antes da troca por grupo plano, reutilizando node no corpo por alpha-renomeação. A exaustividade e as mesmas chamadas/curto-circuito foram conservadas, sem a nota V20 adicional. [flat-check.json](flat-check.json).

Somente quatro consumers e seus quatro owners mudaram no inventário produtivo: eval/mod.rs, layout/realized.rs, introspect/extract_payload.rs e introspect/locatable.rs. Este último recebeu somente documentação/metadados; o guard confirma código executável intacto. Assinaturas públicas, módulos e campos permanecem. As delegações existentes foram preservadas, acrescentando somente a normalização Source descrita acima. [code-guard.json](code-guard.json), materialization.json e l0-refinement.json preservam as versões prescritas e o ajuste plano. A revisão vigente substitui a ressalva histórica de fallback aberto no extractor; OutlineEntry continua pendência explícita.

## RAM e proveniência

Execução por cgroup com teto **12 GiB**, swap **0**, Cargo **um job** e comandos pesados sequenciais. Pico: **12884901888 bytes (12.000 GiB)**; sem OOM ou OOM kill. Maior contador memory.events/max: 1099; os registros preservam a pressão no limitador. Não se infere ganho de desempenho desta correção. [Recursos](validation-resources.json).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) conserva UTC, lista exata git diff HEAD --stat, status e hashes completos; [final-state.json.gz](final-state.json.gz) conserva o estado após validação. As fontes permaneceram congeladas durante os gates. Oracle da suíte: pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, verificado pelo executor. Linter: `12797073972e375d9ab078cf6750c8b30b71e2b9`, binário SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Não houve commit, push, release, merge ou alteração de pin do CI nesta etapa.

## Continuidade

Restam **239 emissões V16** e a investigação do alinhamento/certificado de preparação de OutlineEntry. A divergência true/None e o caminho sintetizado são anteriores e foram preservados de modo explícito; não foram declarados resolvidos pela exaustividade. O próximo lote deve medir esse caso e seus produtores/travessia antes de decidir. O fechamento desta etapa não encerra o objetivo geral.
