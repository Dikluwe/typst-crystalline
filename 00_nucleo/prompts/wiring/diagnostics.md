# L0 — wiring/diagnostics
Hash do Código: 52a3084e

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/diagnostics.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/diagnostics.rs:15` — `drain_to_stderr`.
- `04_wiring/src/diagnostics.rs:24` — `drain_to_stderr_with_primary`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/diagnostics.rs`.

## Owned contract

- Own drain_to_stderr and drain_to_stderr_with_primary. Resolve diagnostic and trace FileId sources once, deduplicate, and use the existing main fallback for absent identity.
- A supplied transient primary overrides World for its own id. Display relative-to-cwd paths where appropriate with existing input fallback, then invoke the L2 DiagnosticSource formatter and emit to stderr.
- Preserve spans, traces, order and formatting selection without new coloring, filtering, reordering or a World dependency in L2. Do not centralize stdin in this helper or merge distinct eval/query provenance.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Caminhos relativos entre diretórios — 2026-10-06

Medição anterior à decisão: diagnostic-paths-2026-10-06/before.json conserva HEAD, working tree, UTC, fontes, comandos e binários contra o pin 9f2b6e871. Para cwd work/a e arquivo project/src/main.typ irmão, Tekt exibe caminho absoluto; vanilla exibe ../../project/src/main.typ. diagnostics.rs:62-66 usa strip_prefix, que só cobre descendentes. A fonte pinada typst-cli/src/world.rs:157-168 usa pathdiff::diff_paths; seu lock fixa 0.2.3, checksum df94ce210e5bc13cb6651479fa48d14f601d9858cfe0467f43ae157023b938d3.

Correção interna de paridade ADR-0127: compor o utilitário puro pathdiff =0.2.3 para calcular o nome relativo à cwd disponível, incluindo ancestrais e irmãos. Não implementar outro algoritmo no wiring. Conservar path/input fallback quando cwd ou diferença não estão disponíveis, fontes transientes, dedup, spans, traces, ordem, cor e stderr. Sem filesystem adicional/canonicalização para apresentação, nova API, flag, fase ou mudança de resolução/sandbox dos arquivos. Pacotes e origem perdida de Content constante não são corrigidos inferindo path ou span nesta unidade. Validar fontes principais/importadas, query, eval contextual, entradas absolutas/relativas e descendentes.
