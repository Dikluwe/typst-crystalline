# L0 — html/pretty
Hash do Código: 2c7cbd45

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/pretty.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/pretty.rs:10` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Optional structural formatting

Own tokenization/classification, quoted tag termination, protected-context tracking,
pretty_html and structural newline helpers. Operate on already compact markup;
preserve every token's bytes. Insert two-space indentation per depth and final
newline only at structural boundaries. Protect phrasing, p/headings, title,
pre/textarea/script/style and td/th interiors. A > within a quoted attribute must
not end a tag. Retain the current tokenizer's supported scope, including raw SVG/
script limitations; no parser rewrite or escaping belongs here.

## Verification

Compare ordered DOM/escapes and protected text before/after formatting; exercise
quoted >, void elements, raw/title and phrasing spaces.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/export/html/pretty.rs:89 (clippy::manual_repeat_n). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Usar std::iter::repeat_n(' ', depth * 2) na mesma inserção de indentação. Preservar multiplicação, ordem, bytes e todos os contextos protegidos.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.
