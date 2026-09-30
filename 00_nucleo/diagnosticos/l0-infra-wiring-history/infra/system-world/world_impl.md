# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/world_impl.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `81481fa627d6a6f2fcfa77ef641b8e17ce5d63a89392bc084975e543fb7b7915`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: implementação do trait World
Hash do Código: 7fd0048a

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/world_impl.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

`03_infra/src/world.rs:655-870` implementa World com source/bytes, fontes,
paths, pacotes, relógio, inputs e plugins. O contrato está em
`00_nucleo/prompts/contracts/world.md`; a extração não modifica assinaturas
ou a fase que chama cada método.

## Responsabilidade

Possui a única implementação `impl World for SystemWorld`. Cache, fontes,
paths e pacotes delegam estaticamente aos helpers descendentes de world.
library/book/main retornam o estado da fachada. inputs clona SysInputs;
plugin_host clona Arc quando presente, ou retorna None. Não criar novo trait,
vtable ou reexport de módulos privados para viabilizar a separação.

`today(None)` mantém data local; `today(Some(offset))` mantém UTC mais duração
exata convertida com overflow tratado sem panic. Retorna somente data e
conserva falha de relógio/local offset como None (P1146). Não trocar default.

## Aceitação

As chamadas públicas continuam `typst_infra::world::SystemWorld` através do
trait existente, conservando retornos, erros e builders encadeáveis.
