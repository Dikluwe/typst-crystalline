# L0 — stdlib/structural/table_grid
Hash do Código: 61e3f6d5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

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

## D08B2 — presença lexical ratificada em 2026-10-07

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

### Contrato ratificado e responsabilidade exclusiva

Somente native_table_cell captura presence depois de validar o argumento no seu value_span. Corpo mantém o cast atual. Aceitar auto apenas nos campos medidos; reject none fora de fill/stroke e auto em spans/stroke com mensagens e spans pinados. Não modificar extratores compartilhados de grid/container para obter isso. Fill/stroke none explícitos assentam presença; argumentos omitidos deixam Omitted/false. Stroke Length/Color conserva os componentes explicitamente fornecidos ao projetar repr. Header/footer promovem kind preservando presence. Não implementar ainda relative inset/dict/stroke por lados/paint ricos fora dos carriers existentes; esses casos continuam dívida declarada.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

### Refinamento medido de casts e limites — D08B2

Antes da decisão, cast-refuters.json (com pin/HEAD/working tree/binários/UTC)
mede x:-1 → number must be at least zero, spans zero/negativos → number must
be positive e x:float → expected integer or auto. Ancorar estes erros no
value_span; corrigir apenas table.cell, sem alterar o extrator de grid.
As cláusulas D08B2 supersedem as anteriores sobre equivalência none/omissão
e aceitação de auto/none em spans somente para table.cell.
Strokes Length/Color assentam somente componentes fornecidos, como a coleta
raw-options exige, sem mudar o extrator global.
Cast-refuters também mede vanilla aceitando stroke:0pt e rejeitando
align:string/inset:integer. Preservar por ora os caminhos legados Tekt desses
casos: não certificar sua paridade nem corrigir compatibilidade adicional
neste pacote. Inset relativo/dict e strokes/paint ricos seguem fora do escopo.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Native Table/Cell constrói valores crus, source_span=Args.span e resolved=None. Table registra rows_explicit pelo argumento, mesmo quando tracks normaliza para [Auto]. Constructors/header/footer preservam source/lexical presence. Assentar os componentes realmente fornecidos no stroke do container também, sem mexer em extrator global. Resolução/ocupação/defaults derivados/execução não pertencem a este parser. Casts e fronteira rica fora de escopo permanecem.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.
