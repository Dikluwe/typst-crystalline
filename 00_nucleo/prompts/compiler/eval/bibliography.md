# L0 — compiler/eval/bibliography
Hash do Código: ca1b5bb0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bibliography.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bibliography.rs:34` — `pub fn load_bib_entries_from_path`.
- `01_core/src/compiler/eval/bibliography.rs:54` — `pub fn parse_bibliography`.
- `01_core/src/compiler/eval/bibliography.rs:102` — `pub fn resolve_style`.
- `01_core/src/compiler/eval/bibliography.rs:115` — `fn load_csl_style_from_path`.
- `01_core/src/compiler/eval/bibliography.rs:143` — `fn hay_entry_to_bib_entry`.
- `01_core/src/compiler/eval/bibliography.rs:186` — `fn person_to_string`.

## Contract

Own bibliography loading/parsing and CSL style resolution. Obtain bytes through
World, preserve relative FileId/path resolution, decode UTF-8 and select BibTeX
or YAML/YML by the existing extension policy. Preserve unsupported-format,
encoding, parse, empty-key and missing-field diagnostics. Convert Hayagriva
entries and persons into the existing BibEntry domain representation.

Resolve built-in CSL styles or load a relative custom CSL through World with the
current validation and errors. L1 performs no direct filesystem/network I/O.
Do not change citation realization, layout, registry or the public signatures.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/eval/bibliography.rs` nas linhas 34 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `load_bib_entries_from_path` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O wrapper interno de leitura legado é chamado somente pelo teste de UTF-8. O módulo pai é pub(crate), sem reexport externo. parse_bibliography e resolve_style permanecem produtivos; a nativa bibliography continua resolvendo/lendo sua fonte e chamando parse_bibliography.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.
