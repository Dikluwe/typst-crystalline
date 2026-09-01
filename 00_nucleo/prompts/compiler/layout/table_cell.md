# Prompt L0 — `table_cell`
Hash do Código: 87a6a377

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/table_cell.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Delegar TableCell ao body já posicionado pelo motor de table/grid, sem duplicar placement.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1288 — carrier semântico de célula (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/compiler/layout/table_cell.rs:14-20` apenas renderiza o body e
descarta kind/level/scope como identidade de frame. A fonte vanilla pinada
resolve a classe explícita antes da classe da linha em
`typst-pdf/src/tags/context/table.rs:98-126`.

### Decisão proposta

Preservar a função descendente e o render único do body, mas devolver/emitir o
envelope semântico fechado pedido por `layout/table.rs`, contendo classe
resolvida, level/scope quando Header, rowspan/colspan e filhos visuais. O
envelope não desenha, não muda bounds nem texto e não produz MCID em L1.

Data explícita dentro de header permanece Data. Header explícito conserva
scope Row/Column/Both e level positivo. A associação entre headers e data é
responsabilidade do exporter, não inferência visual do layout.
