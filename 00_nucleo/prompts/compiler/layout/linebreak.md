# Prompt L0 — consumer visual de linebreak
Hash do Código: f14daa83

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/linebreak.rs`

## Contrato e aceitação

Fechar linha explícita; quando `justify` é true distribuir espaço restante
pelas oportunidades reais antes do flush. Emitir boundary semântico
transparente quando aplicável. False/omitido não justificam e nenhum estado
vaza para a linha seguinte.
