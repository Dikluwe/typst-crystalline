# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/watch.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `94a638ed8651dc10bda60fdd69e2c8ac548b7d6e20540c2d19fb4a245493a35a`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/watch
Hash do Código: 4189d4ae

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/watch.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/watch.rs`. Único dono de run_watch e helpers write/emit_watch_block, watch_line_at/block_at/block_now, prepare_watch_cycle_with/cycle e watch_status_for_outcome. Somente run_watch passa a pub(super). O loop compõe run_compile_observed do owner `00_nucleo/prompts/wiring/compile.md`, eviction existente e capacidades de watch L3; não implementa polling/fingerprints ou servidor.

Preservar rejeição de stdout, seleção de servidor Html/Bundle salvo no_serve, porta/no_reload, anúncio da URL, staging por PID e loop. Capturar Instant antes da formatação; status terminal é emitido pela callback antes da drenagem. Elapsed/has_warnings/exit escolhem WatchStatus tipado; time local com fallback UTC permanece em L4. Blocos Compiling têm um LF; demais dois LF. Writer executa write_all depois flush, propagando falhas; wrapper stderr preserva expect.

Depois de compilar, fallback de dependências, arm, publish/publish_directory no sucesso, abandon/abandon_directory no erro. Falha de publicação exit 2; atualizar snapshot HTTP somente após publicação bem-sucedida. Preservar último artefato/snapshot após erro, evict(10) e wait_for_change_since com 100 ms. Published é o mesmo carrier privado do parent. Não mudar defaults HTTP ou acrescentar capacidade nova.

## Aceitação

Mover sem modificar corpos os cinco testes P1362/P1363/P1364/P1365/P1367. Sentinelas in-memory/literais continuam sem relógio real; processos existentes conferem dependências, recovery, servidor, flags e bundle. P1366 pertence ao compile. APIs L3 recebem os mesmos argumentos na mesma ordem.
