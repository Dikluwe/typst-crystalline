# L0 — stdlib/structural/table_grid
Hash do Código: d4d8c07e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_grid.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/table_grid.rs:50`, `01_core/src/compiler/stdlib/structural/table_grid.rs:302`, `01_core/src/compiler/stdlib/structural/table_grid.rs:337` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own table/grid container, cell and header/footer argument parsing and carriers, including the local usize/align/inset/bool extractors. Preserve paired table/grid validation except the explicitly specified semantic table-header level exception. Cells preserve x/y/colspan/rowspan and placement carriers; table/grid geometry resolution belongs to layout. Auto or None becomes None for optional usize placement; Align is direct or parsed from Str with Align2D::from_string; uniform Length inset becomes Sides<Length>; Bool retains the requested default. Preserve fill:none and stroke:none on their supported container/cell routes; zero-thickness remains rejected rather than PDF hairline.

Headers/footers preserve repeat=true default and explicit false. table.header accepts strictly positive integer level (default 1) and cells: raw content becomes Header Column-scope TableCell, an Auto cell is promoted likewise, and explicit Header/Data classification is retained. table combines distinct levels into one logical group in order and rejects duplicate levels, preserving the default-two-header diagnostic. This semantic PDF exception does not authorize level on grid.header. Keep geometry/repeat compatibility.

Reuse default_hline_stroke from `00_nucleo/prompts/compiler/stdlib/structural/table_lines.md`; do not duplicate line construction/rendering. Keep the table/grid symmetrical surface except for the documented header semantics, with no I/O or dynamic dispatch.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

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

Somente native_table_cell captura presence depois de validar o argumento no seu value_span. Corpo mantém o cast atual. Aceitar auto apenas nos campos medidos; reject none fora de fill/stroke e auto em spans/stroke com mensagens e spans pinados. Não modificar extratores compartilhados de grid/container para obter isso. Fill/stroke none explícitos assentam presença; argumentos omitidos deixam Omitted/false. Stroke Length/Color conserva os componentes explicitamente fornecidos ao projetar repr. Header/footer promovem kind preservando presence. Não implementar ainda relative inset/dict/stroke por lados/paint ricos fora dos carriers existentes; esses casos continuam dívida declarada.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Esta proposta muda o contrato público por campo/tipos novos: só materializar depois da confirmação humana do manifesto. Não altera fase/defaults de render.
