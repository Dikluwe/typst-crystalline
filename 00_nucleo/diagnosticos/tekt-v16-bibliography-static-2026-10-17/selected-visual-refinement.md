
### Refinamento I2 — vista visual selecionada de Cell

selected-visual-attempt registra o primeiro GREEN: 5 passaram e o controle
de coexistência falhou na coleta (resolved contra a expectativa standalone).
A expectativa confundia a prioridade do mapper genérico com a vista ativa.
Fonte reavaliada: TableCellElem::visual, table_cell.rs:261–268, usa resolved,
senão standalone, senão body; finish_cell_product em realize/table_cells.rs:
1065–1080 publica o corpo realizado em resolved quando presente. O L0 da
entidade §correção de spans descreve coexistência: standalone conserva
propriedades observadas enquanto resolved mantém o plano ativo. A cláusula
de prioridade standalone do mapper é outra operação, não redefine visual().
A medição anterior que atribuía essa prioridade ao visual estava incorreta.

Testar separadamente resolved-only, standalone-only e coexistência. O ativo
é resolved quando existe, senão standalone; manter a origem raw e o carrier
inativo intactos. Registrar novo RED por Ref ainda presente no visual ativo,
sem mudar nenhum teste antigo de mapper nem corrigir seu contrato neste lote.
A conversão recebe arm específico de TableCell: copiar só o carrier ativo,
converter exclusivamente seu visual (ou raw se nenhum preparo), preservando
campos, presença, stage/guards/props e o outro carrier. Coleta usa visual()
existente. Nenhuma mudança em visual(), fases ou API. O desacordo dos
mappers genéricos com a vista ativa fica registrado para auditoria própria;
não usar a fixture anterior para mudar o produto.
