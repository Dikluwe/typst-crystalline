# Auditoria a continuar — operações genéricas e vista ativa de Cell

Estado: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. Fonte exata em selected-visual-red-state.json.gz; horário/hashes e textos preservados.

A leitura de `TableCellElem::visual()` prioriza resolved, depois standalone e body. `realize/table_cells.rs::finish_cell_product` atualiza resolved quando ele existe. A cláusula de coexistência no L0 da entidade explica que standalone pode conservar propriedades observadas enquanto resolved mantém o plano ativo.

Já `TableCellElem::map_content_if_changed`, `map_text` e `Content::map_content_pre` priorizam standalone. O L0 de reconstrução de 2026-10-10 prescreve essa prioridade de descida. O teste antigo change_aware_prepared_cell_preserves_properties_guards_and_hidden_body a exige no cenário de coexistência. Esse teste e esses contratos não foram alterados pela bibliografia.

Não inferir que a prioridade é correta ou incorreta em todas as operações a partir de uma só leitura. Auditar callers e distinguir transformação da contribuição visível de transformação da representação/propriedades observadas; produzir caso de linguagem/render/efeitos que refute ou confirme a divergência. O vanilla não possui nossos carriers e não pode ser usado como oráculo de igualdade de estruturas internas.

I2 precisa converter a mesma vista ativa que coleta e que será consumida pelo walker/layout. O reparo específico de conversão não autoriza uma mudança silenciosa nos mappers genéricos. A auditoria ampla permanece requisito aberto do objetivo global; não foi encerrada ou silenciada por este lote.
