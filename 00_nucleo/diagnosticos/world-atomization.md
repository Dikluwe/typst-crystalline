# Atomização de SystemWorld

Registro: 2026-09-26T09:20:48-03:00. HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; working tree não commitado. Proveniência completa (stat global, untracked, hashes do lote e registros de validação): [world-atomization-manifest.json](world-atomization-manifest.json).

## Resultado

`03_infra/src/world.rs` passou de 1303 para 206 linhas. Mantém estado, construtores, erros, inputs e composição de plugins. Descendentes privados possuem lógica de cache, paths, pacotes, fontes/cobertura e a única implementação de World; testes mantêm `world::tests`. Caminho público, assinaturas e comportamento foram preservados. Não foram feitos ajustes de performance, cache, concorrência ou sandbox.

| Arquivo | Responsabilidade | Linhas |
|---|---|---:|
| world/cache.rs | SourceSlot, registro, source e bytes | 137 |
| world/paths.rs | Raízes, leitura/include e bibliografia | 183 |
| world/packages.rs | Bases, identidade, manifestos e downloader/CA | 174 |
| world/fonts.rs | Builders, preload e coverage híbrida | 195 |
| world/world_impl.rs | Adapter estático do trait e relógio/inputs/plugins | 119 |
| world/tests.rs | MockWorld e caracterizações | 533 |

## Gates e verificação

- L0 atualizado antes da extração; cada um dos sete consumers possui owner 1:1. Pin de custom-ca-cert transferido ao owner packages. Resselo oficial restrito ao lote em cópia isolada, com preflight V15/V26 e verificação V5/V15/V26 limpa; nenhum reparo global.
- Gate estrutural RED antes da extração (descendentes ausentes), GREEN depois (arquivos presentes e fachada sem implementações delegadas). Não se inventou mudança de comportamento para forçar falha em refactor.
- Baseline: 25 testes existentes; sete caracterizações adicionadas e aprovadas ainda no monólito. Final: 32 testes World e 114 testes selecionados do pipeline passaram; `cargo build --offline` passou.
- Comparação AST: 92 funções/métodos, zero diferenças de assinatura/corpo sob a normalização explicitada no manifesto. Não é prova formal de paridade; serve para detectar deriva acidental na movimentação.
- `crystalline-lint .` terminou com exit 0, conservando warnings/infos globais. Tentativas intermediárias observaram drift V5 em plugin_host sendo alterado por outra tarefa; esses arquivos foram preservados. A última verificação global estrita V5/V15/V26 terminou com exit 0 e `No violations found`. Isso não implica ausência dos warnings/infos dos demais checks.

## Caracterizações adicionais

Main sintético sem arquivo; conteúdo/erro de source memoizado; Arc de bytes compartilhado entre caminhos legacy/rooted e leitura por ID; tentativa de bytes após erro; IDs canônicos e inventário; origem rooted e rejeição de escape lexical; coverage pré-materializada sem fonte física. Asserts mecânicos de Arc são garantia do cache, não critério de paridade de língua.

## Próximo refinamento

Os novos módulos oferecem localidade para investigação. Antes de otimizar, estabelecer microbenchmarks separados de hit/miss de cache, paths e consulta de coverage, com fixture e I/O fora do loop quando se pretende medir somente o caminho aquecido. Esta extração não demonstra ganho de tempo.
