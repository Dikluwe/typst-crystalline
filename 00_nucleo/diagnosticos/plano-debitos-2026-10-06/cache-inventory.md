# Inventário inicial de caches e invalidação

Inspeção da working tree de HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b.
`equation-red-state.json.gz` e `execution-state.json.gz` conservam fontes e
proveniência desta rodada. Este documento não mede hits, memória ou desempenho.

| Unidade e fonte | Chave/produto | Proprietário e duração | Invalidação atual | Exigência antes de ampliar |
|---|---|---|---|---|
| Module, 01_core/src/entities/module.rs:33,93-120 | Snapshot de TagIntrospector associado ao Module | Arc de ModuleInner; resultado de uma realização | Setters efetivos de content/introspection_content apagam snapshot | Dependências de recursos/estilos/contexto precisam entrar na validade entre avaliações |
| Sources, 03_infra/src/world/cache.rs:20-55,62-86 | FileId, caminho canônico e SourceSlot com OnceLock | SystemWorld; arquivos lidos uma vez por slot | Descarte do World; slot carregado não relê edição | Reusar World sem renovar slots manteria texto antigo |
| Bytes, 03_infra/src/world/cache.rs:145 e 03_infra/src/world.rs:83 | FileId → Arc<Vec<u8>> | Mesmo World | Descarte do World | Assets/imports alterados precisam de invalidação explícita, inclusive falhas anteriores |
| Catálogo/fontes/coverage, 03_infra/src/world.rs:65-96 e world/fonts.rs:98-104 | Slots/FontBook emparelhados e coverage por índice | World; catálogo lazy substituível por builders | Novo World ou instalação de catálogo nos builders | Identidade por slot não pode atravessar catálogos alterados sem prova |
| FallbackFontMetrics, 03_infra/src/font_metrics.rs:49-87 e caches.rs:58-98 | Larguras por texto/tamanho/família/estilo/direção/idioma/eixos/math; faces por slot | Instância empresta World; clones compartilham Arc dos mapas | Instância/World novos; contexto pode reutilizar o mesmo handle | Não transferir caches por slot entre Worlds diferentes; testar recursos, idioma, eixos e invalidations |
| ShapeCache/FaceCache, 03_infra/src/shaper/cache.rs:19-85 | Run: texto/slot/rtl/eixos f32 exatos/tracking/ssty/idioma; valor glyphs/width/fonte efetiva | Run local ao documento; FaceCache por instância/compilação | Nova instância | Preparação antes de hit e script não testado ainda exigem medição |
| MeasuredCells, 01_core/src/compiler/layout/grid/measurement_cache.rs:22-36,115-194 | Produto Auto por índice/row/col/origin/width e ambiente | Tentativa local de medição/emissão | take consome entrada; guarda rejeita alterações | Styled/Realized/texto composto/contexto exigem prova de efeitos, geometria e consumo |

Os owners desses arquivos estão nos headers e continuam 1:1:
`00_nucleo/prompts/entities/module.md`,
`00_nucleo/prompts/infra/system-world.md`,
`00_nucleo/prompts/infra/system-world/cache.md`,
`00_nucleo/prompts/infra/system-world/fonts.md`,
`00_nucleo/prompts/infra/font_metrics.md`,
`00_nucleo/prompts/infra/font_metrics/caches.md`,
`00_nucleo/prompts/infra/shaper/cache.md`,
`00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`.

## Ciclo de watch e dependências

`04_wiring/src/watch.rs:60-63` chama run_compile_observed em cada volta.
O constructor está em compile.rs no ramo de aquisição de World. A cada chamada
é construído um World distinto; arm/publicação/wait não conservam esse World.
Os recursos e caches ligados a ele não atravessam a edição seguinte.
Isto é prova de lifetime da fonte, não número de millis desperdiçados.

`01_core/src/contracts/world.rs:29-33` separa World de TrackedWorld; as chamadas
track em eval/route/sink e o introspector tracked não demonstram cache de
avaliação com dependências completas. Antes de especificar incremental, mapear
source/file/font/inputs/data/datetime/packages e recursos acessados por callback,
e medir os caminhos reais de invalidação. Ordem recomendada: conservar catálogo
imutável comprovadamente válido, depois renovar dados por compilação; a escolha
concreta de API/reset ainda depende da medição e do gate ADR-0127.

## Lifetime e Rust seguro

Há unsafe herdado nos constructors de CachedFace em
03_infra/src/shaper/cache.rs:96-100 e
03_infra/src/font_metrics/caches.rs:34-38, para criar um empréstimo estático dos
bytes owned. Os respectivos L0s mandam preservar esse encapsulamento existente.
Nenhum unsafe foi introduzido neste lote. A preferência atual por código seguro
não prova ausência de unsafe no repositório nem ratifica automaticamente sua
origem histórica. Reavaliar esse lifetime é um item de manutenção distinto da
retenção de World, com comparação de custo e revisão de L0 antes de alterações.
Este achado não demonstra por si defeito de memória na execução atual.
