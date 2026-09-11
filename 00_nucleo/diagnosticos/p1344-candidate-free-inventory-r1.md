# P1344 — inventário candidate-free e fronteiras owner-local

## Veredito

`CANDIDATE_FREE_TOPOLOGY_READY_FOR_CONTRACT`.

Regime: **executado sem atestacao de isolamento**. Esta autoridade congelou
somente L0, consumers e fronteiras físicas. Não escreveu nem julgou contrato,
oráculo, corpus, teste, selo ou implementação.

## Proveniência

Medição em `2026-09-10T23:14:43-03:00` sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada:

- SHA-256 de `git status --porcelain=v1 -z` antes dos outputs:
  `a381abd797e75c18dfd37a0708b806159486d219cc48d0b9b536c25ae880706c`;
- SHA-256 de `git diff --binary HEAD` antes dos outputs:
  `099901758e0dda60cc4c16de0091adfdcedd3fcc63029e1270c34147b60aadef`;
- stat nesse instante: `76 files changed, 10972 insertions(+), 833 deletions(-)`;
- passo P1344 lido pelo path explicitamente autorizado, SHA-256
  `1fff15c6f17800ca293379e17d8d2ed63b2fe9043a0aebb0baf16a9b9b0ea6f3`;
- baseline P1343 de 36 cápsulas, SHA-256
  `db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977`;
- blocker físico P1343 R3, SHA-256
  `c2af3f4b314660f620904d1fd7fa2bd0d5a5c9d8162d66edcf9aaa49e8a053d3`.

## Freeze L0 ↔ consumer

Os doze pares são distintos e 1:1. Os hashes completos vigentes são:

| Prompt L0 | SHA-256 L0 | Consumer | SHA-256 consumer |
|---|---|---|---|
| `prompts/infra/pipeline/context_stabilization.md` | `ee9b2fda…cc55` | `03_infra/src/pipeline/context_stabilization.rs` | `2f03c9e5…fa30` |
| `prompts/compiler/eval.md` | `580849e8…f7324` | `01_core/src/compiler/eval/mod.rs` | `8706947c…0dd` |
| `prompts/compiler/eval/bindings/value_methods.md` | `2a0febd2…9b07` | `01_core/src/compiler/eval/bindings/value_methods.rs` | `26865ce0…e36c` |
| `prompts/compiler/stdlib/counter.md` | `a7739f41…8a89` | `01_core/src/compiler/stdlib/counter.rs` | `239561a4…8220` |
| `prompts/entities/func.md` | `1dc7ae58…c8f87` | `01_core/src/entities/func.rs` | `02873c68…e314` |
| `prompts/entities/elements/counter_update.md` | `3a8cb1f6…5ea` | `01_core/src/entities/elements/counter_update.rs` | `d417085e…ec7b` |
| `prompts/compiler/introspect.md` | `13b5b367…90d2` | `01_core/src/compiler/introspect.rs` | `6d2de23b…747c` |
| `prompts/compiler/introspect/from_tags.md` | `04596c56…9a87` | `01_core/src/compiler/introspect/from_tags.rs` | `92d87a56…ec5` |
| `prompts/compiler/eval/call_dispatch.md` | `9d00297e…775` | `01_core/src/compiler/eval/call_dispatch.rs` | `16fa614b…588a` |
| `prompts/compiler/eval/closures.md` | `4909fe13…5e3e` | `01_core/src/compiler/eval/closures.rs` | `58b0a334…37df` |
| `prompts/entities/content.md` | `fdb33918…921a` | `01_core/src/entities/content.rs` | `3f2d8fd6…45c66` |
| `prompts/entities/counter_update.md` | `9d02b528…42c3` | `01_core/src/entities/counter_update.rs` | `6131b00c…4e74` |

Os hashes integrais, sem abreviação, ficam no freeze JSON. O comando
`crystalline-lint . --checks v5,v15,v26 --format text` terminou com
`✓ No violations found`: V5, V15 e V26 passaram antes de contrato.

Nove pares históricos não ligados a `Func` são byte-idênticos ao freeze
P1342. Nos três sources ressellados (`func.rs`, `content.rs` e
`counter_update.rs`), normalizar somente o valor de oito hexadecimais da linha
`@prompt-hash` torna cada arquivo byte-idêntico ao mesmo arquivo em HEAD. Os
SHA-256 normalizados são, respectivamente, `aad67e4e…1627`,
`cb397c76…ce51` e `c780130c…0686`. Portanto o delta produtivo é zero: só os
headers de linhagem mudaram.

Uma busca fechada por markers `P1342`, `P1343` e `P1344` nos doze consumers
devolveu zero ocorrências. Não existe candidato produtivo a contaminar este
freeze.

## Overlay de 38 cápsulas

O overlay P1344 preserva, por referência hash-pinned, a ordem, kind,
replacement, cfg, âncoras e restrições das 36 cápsulas P1343. A única redução
semântica nessa base é `P1343-FUNC-CARRIER-HELPERS`: sua posição permanece
entre o fim de `Func::namespace` e o `}` de `impl Func`, mas sua produção passa
a aceitar exclusivamente associated methods de `Func`. Os helpers de
`Content` e `CounterUpdate` ficam explicitamente proibidos nessa fronteira.

As duas adições fechadas são:

| Cápsula | Owner físico | Cobertura redistribuída | Kind/cfg | Replacement |
|---|---|---|---|---|
| `P1344-CONTENT-CARRIER-HELPERS` | primeiro `impl Content`, imediatamente antes de `elem_name` | H11 | `insert` / `p1339_observation` | vazio |
| `P1344-COUNTER-UPDATE-CARRIER-HELPERS` | único `impl CounterUpdate`, imediatamente antes de `step` | H10, H12A, H12B | `insert` / `p1339_observation` | vazio |

Com essa redistribuição, `P1343-FUNC-CARRIER-HELPERS` conserva H07, H08, H09,
H13, H14 e H15. Nenhuma das dezenove rows runtime é removida ou reinterpretada;
o overlay muda apenas quem pode declarar o helper associado.

As âncoras antes/depois têm uma ocorrência cada no arquivo candidate-free:

- Content: `4147f096…5af9` / `c2cc892d…0a20`;
- CounterUpdate: `272bfe6c…269` / `5e218903…664e`.

Os dois arquivos novos não continham cápsulas P1343. Logo essas fronteiras não
se sobrepõem às 36 anteriores; as 38 IDs e os 12 paths são únicos no overlay.

## Prova física anterior ao contrato

Em `/dev/shm/p1344-owner-grammar`, cópias integrais dos três sources receberam
somente um associated method cfg-only de sua própria entidade, no ponto exato
da cápsula proposta. `rustfmt --edition 2021 --emit stdout <copy>` parseou as
três cópias com exit 0:

- `func.rs`: `63c5a3653107face726ef532d5a2b4fca8f53865f41edef1e8492e6d58a407db`;
- `content.rs`: `51b31aeaf23f68ebde7b4e744ab9886af6d240816588d42ff9a4e8df45ad5852`;
- `counter_update.rs`: `e4179882dc0d739abfdc039ac455678fa7ce70263128496808aae7234ac670ba`.

Cada inserção ocorreu depois do `{` do inherent impl já existente e antes do
primeiro/próximo associated item. O probe contém zero fechamento/reabertura de
impl, zero wrapper, zero const/macro container e zero impl não local. Assim, as
três categorias cabem fisicamente nos owners sem repetir a contradição P1343.

## Limite da conclusão

`READY` significa apenas que a topologia candidate-free é internamente
consistente e parser-valid. Não significa contrato selado, mutation score,
RED/GREEN, implementação correta nem aceite de P1342/P1344. O workspace é
compartilhado; não há atestação técnica de isolamento.
