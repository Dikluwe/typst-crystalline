# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/info.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `2383634a44de5a7c18d228d8b68127693a451632510b0df46e2d8656b5cd0c2f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/info
Hash do Código: 50377ddb

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/info.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/info.rs`; run_info pub(super), corpo preservado. snapshot L3, PARITY_VERSION L1, commit L2, plataforma e BTreeMap env formam InfoData L2; nomes/campos e features default false mantidos. Certificado só reporta presença combinando flag/env snapshot, sem expor path/bytes. Transportar paths de fontes/pacotes como medido.

Formato Some(Json) chama format_json com pretty; None format_human. Falha de serializer/stdout exit 2 com mesmos diagnósticos, sucesso exit 0. Não inspecionar TLS nem inventar feature efetiva; esta extração não materializa os rascunhos históricos do L0.

## Aceitação

Processos existentes verificam schema/commit integral, humano, segredo/CA como presença, pretty e formatos inválidos.
