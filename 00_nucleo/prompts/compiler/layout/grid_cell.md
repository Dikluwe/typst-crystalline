# Prompt L0 — `grid_cell`
Hash do Código: eccb5d14

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/grid_cell.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Delegar GridCell ao layout do body já realizado pelo grid, sem reinterpretar placement ou estilos da célula.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.
