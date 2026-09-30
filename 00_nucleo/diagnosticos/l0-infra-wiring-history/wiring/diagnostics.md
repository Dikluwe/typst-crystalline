# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/diagnostics.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `20b94b2e40d60dcb110d3df27e23342d24c5646025a800518020a43a66a2ac4f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/diagnostics
Hash do Código: c7d8e923

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/diagnostics.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/diagnostics.rs`. drain_to_stderr e drain_to_stderr_with_primary passam a pub(super); corpos conservados. Materializar fontes dos FileId do span e trace, deduplicar, fallback main quando não há id; primary transitória vence World para seu id. Display path relativo ao cwd quando aplicável, fallback input quando path indisponível. Chamar DiagnosticSource/format_diagnostic L2 e eprint sem inventar mensagens, cores, spans, regiões, filtro ou ordem entre coleções.

World/Source permanecem L3/L1; formatter L2 não recebe World. Não transformar em serializer, logging ou helper de domínio. Nenhum tipo novo, contrato público ou nova política de I/O.

## Aceitação

Suítes de processo de eval/query/compile conferem fonte primária, traces/imports, canais e cores. Não centralizar leitura de stdin nem alterar nomes de fontes ao extrair.
