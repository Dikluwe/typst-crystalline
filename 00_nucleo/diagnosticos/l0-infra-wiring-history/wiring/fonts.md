# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/fonts.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `c2cc1af6959ef0fd59a358e809e61cb16d257ef760139ad4f0a43f6de7c38a6d`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/fonts
Hash do Código: a0c07ba3

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/fonts.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/fonts.rs`; run_fonts pub(super), corpo preservado. inventory_fonts L3 recebe font_paths/include_system; campos family/path/index/style/weight/stretch/embedded são transportados aos FontDisplayEntry L2, sem descoberta ou formatter próprio. format_fonts L2 recebe variants; escrever string em stdout; exit 0 ou I/O exit 2 com mesma mensagem.

## Aceitação

Processos existentes validam fontes embutidas sem sistema, variantes e campos; não alterar descoberta, ordenar novamente ou assumir paths da máquina como expected.
