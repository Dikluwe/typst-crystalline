# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/tests.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `faee74db4c0d031423779bc31938a9d99d3e8a3d94e30b407fda97a9e67cd361`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — testes de SystemWorld
Hash do Código: 5e10bff4

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/tests.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

`03_infra/src/world.rs:873-1303` reúne MockWorld, testes com filesystem,
fontes, paths, bibliografia e pacotes, incluindo helpers de temporários.
Testes são consumer próprio pela ADR-0129; extrair não altera suas chamadas.

## Responsabilidade e aceitação

Preservar integralmente os testes existentes, seus nomes e caminho
`world::tests`. Estender a caracterização antes da extração para provar main
sintético sem I/O; source/erro memoizado; compartilhamento de bytes entre
paths e IDs; releitura após erro de bytes; identidade e dependências;
rejeição de escape lexical e coverage pré-materializada sem acesso à fonte.
Usar diretórios temporários locais e dados determinísticos. Rodar em série
para evitar interferência do teste legado que modifica XDG_DATA_HOME.
Nenhum teste exige rede. Igualdade de Arcs é um teste de mecanismo do cache,
não um critério de paridade da linguagem (ADR-0107).
