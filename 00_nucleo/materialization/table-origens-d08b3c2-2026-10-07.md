# D08B3-C2 — contrato ratificado e executado

Estado atual: contrato confirmado pelo dono (`confirmo`) em 2026-10-07 e materializado. Relatório de execução: `00_nucleo/diagnosticos/table-origens-d08b3c2-execucao-2026-10-07.md`. Os quatro candidatos e o manifesto abaixo permanecem como pacote original ratificado; não são os L0 vigentes. Ownership, pins, RED→GREEN, validações e limites estão no relatório e em `00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/final-results.json`.


Mudança exata: adicionar Content::Source(Arc<SourceContent>) e tipo público SourceContent { pub body: Content, pub span: Span }; conservar primeira origem do conteúdo, sem Location. CodeBlock passa a preservar Styled lexical e captura causal para realizar na inserção efetiva, supersedendo o atalho imediato A. Sem campo público novo em Table/Cell nem assinatura/flag nova.

Contrato completo: `00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. Manifesto SHA-256 5e5086404c6359dad6658b77dc465aa4a4a6bba4c99f7a505245092a0e07c79c.

Gate ADR-0127 §2.1/2.3/2.4 satisfeito pela confirmação humana deste pacote. C2 verificado; resíduo geométrico do produto misto de default show continua aberto como D08B3-C3.
