# Prompt L0 — harness focal de `outline.entry.indented`
Hash do Código: 8d3e05da

**Camada:** L1 — teste de integração, sem consumer produtivo
**Ficheiro alvo exclusivo:** `01_core/tests/p1405_outline_indented.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição antes da decisão

O teste monolítico de `typst-core` excedeu a memória disponível em passos
anteriores. O contrato P1405 pode ser discriminado por um harness externo que
instancia somente a nativa e seus colaboradores públicos, sob teto explícito.

## Contrato do harness

O consumer deve verificar indentação fixa multiplicada pelo nível 0-based,
função chamada com esse nível, máximo automático por nível, fallback `1.2em`,
prefixo ausente, `gap`, booleano falso, lado lógico RTL, preservação de Content,
retorno inválido da função e rejeição de parent ausente. Nenhum caso pode
fabricar parent na entry ou alegar o caminho positivo de show rule reservado a
P1406.

Este L0 legitima somente o teste indicado e não altera a API produtiva.
