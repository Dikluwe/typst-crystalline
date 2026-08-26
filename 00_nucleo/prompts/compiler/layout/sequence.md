# Prompt L0 — layout de sequência
Hash do Código: 031dc05a

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/sequence.rs`

## Contrato e aceitação

Iterar Content::Sequence com lookahead, sticky e colapso de spacing, delegando
cada filho ao dispatcher sem perder ordem, boundary ou evento de paginação.
