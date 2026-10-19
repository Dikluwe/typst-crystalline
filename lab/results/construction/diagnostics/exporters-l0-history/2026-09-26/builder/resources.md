# Prompt L0 — `infra/export/builder/resources`
Hash do Código: aae8dcfe

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/resources.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:215-385`, `:394`, `:2239-2595`
continham estes helpers e emissores.

Color spaces spot, gradients e image XObjects formavam uma familia coesa no
monolito: alocam e emitem recursos referenciados pelas paginas, mas nao
decidem navegacao, metadata ou o envelope xref.

## Responsabilidade

Este modulo calcula entradas `/ColorSpace`, emite objetos de gradients e
image XObjects e preserva a numeracao recebida do orquestrador. Deteccao e
decodificacao continuam nos modulos especializados existentes; aqui vive
somente a agregacao desses recursos no builder.

## Criterios de verificacao

- A ordem, IDs e dicionarios de recursos nao mudam pela extracao.
- Spot colors, gradients e imagens continuam cobertos pelas suites existentes.
- Nenhuma API publica ou default e introduzido.
