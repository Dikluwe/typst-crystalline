# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/eval.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `6d95f99f0d49bb62669e446b4c891b860934a93c4dd60538c1a7c9e2ff100bef`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/eval
Hash do Código: b263b887

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/eval.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/eval.rs`; run_eval pub(super), corpo inteiro preservado. EvalIntent.input None usa World mínimo; - lê stdin em uma Source transitória; path constrói World físico com root/main como medido. Fontes/CA e códigos de erro mantidos, sem centralizar com query se isso mudar a origem.

Materializa Source code da expressão para diagnósticos, chama operação contextual de L3 quando documento existe, operação com sink/features caso contrário. No Err drena errors antes de warnings, sem filtragem por mensagem, uma vez cada; no Ok warnings antes da serialização/valor. Resolver fontes pelo owner `00_nucleo/prompts/wiring/diagnostics.md` com primary transitória. L2 serialize_eval decide formato/pretty; bytes stdout sem alteração. Erro semântico/serialização exit 1, I/O exit 2 e sucesso exit 0. Não implementar query, serializer ou avaliação aqui.

## Aceitação

Suíte CLI mantém eval sem/contexto/--in/stdin, formatos JSON/YAML/raw, stderr/errors/warnings, features e certificados, sem modificar expected. Todos os corpos e chamadas são conservados.
