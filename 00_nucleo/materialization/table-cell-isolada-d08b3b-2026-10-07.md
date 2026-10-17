# D08B3-B — proposta de defaults isolados e regras por ocorrência Cell

Estado: ratificado pelo dono (`confirmo`) e materializado. Relatório: `00_nucleo/diagnosticos/table-cell-isolada-d08b3b-execucao-2026-10-07.md`. O manifesto/candidatos originais permanecem preservados. Manifesto SHA-256 `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`.

A medição anterior à decisão está em `00_nucleo/diagnosticos/table-cell-isolada-d08b3b-medicao-2026-10-07.md` e `table-cell-isolada-d08b3b-2026-10-07/complete.json.gz`: 53 sondas, pin/binários/HEAD/working tree/UTC/fontes/comandos completos. O contrato e critérios completos estão em `table-cell-isolada-d08b3b-2026-10-07/contract.md`; os 20 candidatos completos usam os caminhos canônicos de seus owners dentro desse diretório, com respectivas bases em `base/`. Nenhum candidato em materialization é Prompt L0 vigente.

Mudança pública exata: adicionar `TableCellElem.standalone: Option<Arc<TableCellStandaloneResolved>>`. Novo carrier público com `visual: Content` e campos privados `stage: TableCellStage`, `guards: Vec<RuleId>`. `TableCellResolved` existente recebe guards privados. Nenhuma assinatura/trait/flag pública muda. Os carriers avulso e posicionado são mutuamente exclusivos, sem coordenadas falsas. Constructors continuam crus; getters preparados conhecem defaults intrínsecos. A origem/body lexical é preservada, só o visual completo é emitido.

Guard por regra e ocorrência distingue it/alias de Cell recém-construída. Uma receita anterior não consumida continua elegível; igualdade morfológica não encerra reconstrução Cell nova. O produto emitido segue o limite causal e o erro usa o span do constructor. A política dos demais elementos conserva seus owners.

O núcleo proposto `00_nucleo/prompts/_nuclei/table/cell-occurrence.toml` tem pin `d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb`. Manifesto guarda hashes de base de código/L0, candidatos, prompts com pins e coleta. Antes de instalar, verificar manifesto/arquivos e reconciliar qualquer deriva sem resetar a árvore. Aprovação autoriza somente este recorte; depois, RED→GREEN, resselo restrito, coletas bilaterais e revalidações Table, Raw, math, matrix, query, suítes/builds/linter.

Paragem por ADR-0127 §2.1: “adicionar, remover ou alterar campo público de qualquer entidade”. A aprovação A anterior excluiu B; o pedido de prosseguir autorizou medição/preparação, não ratificou este contrato ainda inexistente. A confirmação deste manifesto ratifica o campo/carrier e candidatos completos. Não há código produtivo alterado nesta etapa.

D08B3-C fica explicitamente responsável pelo negativo `set table.cell` e defaults de estilo, com medição/L0 próprios. Não certificar o universo Cell ou os defaults gerais de Table por este pacote.
