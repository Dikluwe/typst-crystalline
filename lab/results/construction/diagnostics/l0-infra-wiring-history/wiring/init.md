# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/init.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `e11f083226f8196d593212c67a6a8df1c9c9eca238294f3c73f6d630643d6f18`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/init
Hash do Código: 9d9d73b6

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/init.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/init.rs`; run_init pub(super), corpo preservado. initialize L3 recebe template/directory/cert_path; sucesso imprime destino e entrypoint nas mesmas linhas e exit 0; falha imprime error e exit 1. Resolução, validação, download, cleanup e escrita do projeto continuam em L3.

## Aceitação

Processos existentes usam fixture local para versão/template/destino existente/path inválido/falha parcial; não transformar falha em sucesso nem criar scaffold local em L4.
