# L0 — wiring/eval
Hash do Código: bb87fdb0

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/eval.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/eval.rs:16` — `run_eval`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/eval.rs`.

## Owned contract

- Own run_eval composition. No input uses minimal World; `-` uses stdin/transient Source; physical input uses current root/main/font/CA setup. Materialize the expression Source for accurate diagnostics.
- With a document call contextual L3 evaluation, otherwise isolated features/sink evaluation. Error diagnostics drain before warnings on Err; on Ok warnings precede L2 serialization/stdout. Preserve each diagnostic once and its origin.
- Semantic/serialization failures exit 1, input/output/argument failures exit 2 and success exits 0. Do not unify this path with query or move evaluation/serialization into wiring.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Compor for_stdin para documento vindo de stdin e source_for_expression para
a expressão em todos os modos de eval. Compor as novas fachadas Source-based
L3; não analisar expressão, alocar ids nem implantar lógica de eval em L4.
Falha da fábrica de Source é erro de aquisição (exit 2) com mensagem explícita.

No formatter existente, a Source primary é exclusivamente a expressão com
nome <input-expression>. Documento físico, stdin, imports e callbacks são
resolvidos pelo World por seus ids distintos. Preservar Source e bytes durante
toda a execução e drenagem; nunca remapear todos os erros pelo input_path.
Manter ordem de mensagens, hints/traces, códigos de saída e formato/pretty.
Não modificar query/compile/stdin, flags ou fases neste pacote.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

A composição de stdin fornece a cwd absoluta adquirida por std::env::current_dir
para que seu caminho de diagnóstico seja <stdin> e a base física permaneça a cwd.
Falha dessa aquisição segue exit 2; não adivinhar raiz alternativa.
