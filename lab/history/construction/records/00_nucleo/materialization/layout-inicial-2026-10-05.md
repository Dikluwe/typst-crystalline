# Execução — reuso do primeiro candidato paginado

Continuação autorizada pelo usuário. Escopo: prova conservadora interna de independência do candidato inicial, conservando PageStore, posições, warnings e binding. L0 paged e L0 tests atualizados antes de código, com refinamento das chaves Styled medido na fonte antes da ampliação. Nenhuma API externa ou mudança de fase; fluxo contínuo ADR-0127.

RED: tabela multipágina executa dois layouts em vez de um. GREEN: uma chamada e todos os números lógicos, fallback com numbering e rejeições conservadoras. Suíte infra completa e release build passaram. Benchmark pareado, contagem release independente, controles vanilla pinados e geometria verificaram a alteração. Históricos anteriores permanecem intactos.

Resultados, estado exato, limitações e próximas frentes estão no [diagnóstico](../diagnosticos/layout-inicial-2026-10-05.md). Não foi implementado cache de frames de células nem reuso de documentos contextuais/complexos.
