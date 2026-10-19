# L0 — infra/export/tests/pdf_page_labels_tests
Hash do Código: 5031efe0

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdf_page_labels_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdf_page_labels_tests.rs:13` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Selected-page label regression coverage

Own in-memory public-API tests that decode the PageLabels number tree and style
into effective reader labels, not merely detect a catalog key.
Cover no selection, full/subrange/discontinuous/single/empty selection, Arabic/
Roman/literal prefix/suffix/no-pattern cases and shifted logical_number.
Retaining every physical page does not require a fallback corrective label tree.

Productive label/filter mapping is owned by
`00_nucleo/prompts/infra/export/builder/navigation.md`.
Check tag/archival/UA coexistence and catalog/tree coherence without fixed object
IDs or whole-PDF byte comparison.

## Verification

Preserve the fixture matrix and per-retained-page labels. Independent qpdf/pypdf
and original bilateral fixture evidence remain separate from local regression tests.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/export/tests/pdf_page_labels_tests.rs:111 (clippy::regex_creation_in_loops); 03_infra/src/export/tests/pdf_page_labels_tests.rs:116 (clippy::regex_creation_in_loops). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Compilar as duas Regex constantes de St e S uma vez por invocação do observador, antes do loop de entries. Manter padrões idênticos, capturas, defaults 1/None, interpretação de prefixos e todos os controles por página. Sem estado global ou regex compartilhada entre invocações.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.
