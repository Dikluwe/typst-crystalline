# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/query.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `adfaf9262bf55b0d773d5ed4b4466f96322340d55fabe18a36e2d7cc8e9722cb`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/query
Hash do Código: 3d51292a

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/query.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/query.rs`; run_query pub(super), corpo inteiro preservado. Emitir exatamente o warning de deprecação antes do resultado. - lê stdin e materializa Source markup transitória; path usa mesmo World físico e root/main, fontes e CA vigentes. Chama query_elements L3; drena warnings, depois errors se falha.

L2 serialize_query_with_format recebe elementos/field/one/pretty/format intactos, sem selectors, campos ou cardinalidade decididos em L4. Saída stdout bytes intactos, sucesso exit 0, falhas semânticas/serialização exit 1 e I/O/args exit 2. Drenagem sob `00_nucleo/prompts/wiring/diagnostics.md`.

## Aceitação

Processos existentes conferem selector, fields/one/format, stdin, source física, erro/deprecação e canais; não alterar parser/shell/infra.
