# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/completions.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `1909a98db04bfeb732eb390aa13dc94267397bb541006889fd3a6b50d802d6b7`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/completions
Hash do Código: a0f0267c

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/completions.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/completions.rs`; run_completions pub(super), corpo preservado. Encaminhar intent.shell ao gerador L2, escrever bytes stdout intactos; exit 0 ou I/O exit 2. Nenhuma dependência clap em L4, árvore CLI nova, template local ou arquivo de completion escrito.

## Aceitação

Suítes existentes validam Bash/Nushell e shell inválido via L2, determinismo/canais e ausência de arquivos. Não mudar o gerador.
