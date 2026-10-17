# L0 — compiler/eval/table
Hash do Código: d3dcb76f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/table.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/table.rs:15` — `fn normalize_table_cell`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `crates/typst-library/src/model/table.rs:494`.
- `crates/typst-library/src/model/table.rs:524`.
- `crates/typst-library/src/model/table.rs:732`.
- `model/table.rs:496-498`.
- `model/table.rs:734-780`.
- `model/table.rs:746-748`.
- `model/table.rs:750-752`.
- `model/table.rs:781-784`.

## Contract

Own only normalize_table_cell, the content-or-table.cell cast used by its
callers. An existing Content::TableCell clones the complete cell, preserving all
visual, geometric and accessibility fields. Other Value::Content becomes a cell
with that body, unset x/y/colspan/rowspan/stroke/fill/align/inset/breakable and
TableCellKind::Auto. Other values fail with `expected content, found <long type>`
at the existing detached span using the canonical type-name formatter.

This helper neither constructs a table namespace nor evaluates set rules,
numbering, captions or PDF headers. Those behaviors retain their respective
stdlib/rules/PDF owners; caller-specific argument diagnostics are not duplicated
in this cast. Do not discard fields by unwrapping an already constructed cell.

## D08B2 — presença lexical proposta; não vigente antes do gate ADR-0127

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato proposto e responsabilidade exclusiva

Clone de TableCell conserva presence. Raw content normalizado em cell recebe presença Omitted/false integral, body conhecido. Não resolver grid neste cast.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Esta proposta muda o contrato público por campo/tipos novos: só materializar depois da confirmação humana do manifesto. Não altera fase/defaults de render.
