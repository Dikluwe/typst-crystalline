# L0 — entities/content/tests/representation
Hash do Código: 90872d00

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/representation.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c

## Contract

Own regression coverage for construction, closed-variant representation, structural equality, cloning, defaults, and payload preservation. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

## Correção de expectativas após contrato ratificado de aspas

Medição: core-quotes-green.log registra falha de tipo nos testes históricos
que tratavam double como bool. O contrato ratificado distingue omitido None
e explícito Some; enabled:false permanece SmartQuote. Atualizar essas
expectativas para os campos semânticos, preservando coverage dos argumentos
e validação. Não muda contrato produtivo adicional (ADR-0127, fluxo contínuo).

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

Fixtures TableCell especificam presence coerente com seus argumentos explícitos. Preservar todos os observáveis/assertions de render existentes; não relaxar tolerâncias nem fabricar defaults conhecidos. Testes não autorizam mudanças produtivas adicionais.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Esta proposta muda o contrato público por campo/tipos novos: só materializar depois da confirmação humana do manifesto. Não altera fase/defaults de render.
