# Prompt L0 — `entities/elements/table_cell` — `TableCellElem`
Hash do Código: f830d43a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_cell.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellElem {
    pub body:      Content,               // era Box<Content>
    pub x:         Option<usize>,
    pub y:         Option<usize>,
    pub colspan:   Option<usize>,
    pub rowspan:   Option<usize>,
    pub stroke:    Option<Stroke>,
    pub fill:      Option<Color>,
    pub align:     Option<Align2D>,
    pub inset:     Option<Sides<Length>>,
    pub breakable: Option<bool>,
}
```

`Content::TableCell { … }` → `Content::TableCell(Arc<TableCellElem>)`.
Construtor ergonómico preservado: `Content::table_cell(...)`.

> **`Hash` manual via Debug**: `Stroke`/`Color`/`Align2D`/`Sides<Length>`
> carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for TableCellElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (transparente; spans runtime-only, `content.rs:1813`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1662`) |
| `map_content` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2289`) |
| `map_text` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2530`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1953`).

## P1288 — classificação semântica explícita da célula (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/entities/elements/table_cell.rs:20-33` mede apenas body e campos
geométrico-visuais. A fonte vanilla pinada mede o carrier fechado
`TableCellKind::{Header(NonZeroU32, TableHeaderScope), Footer, Data}` e scopes
Both/Column/Row em `pdf/accessibility.rs:274-310`, enquanto `table.cell.kind`
começa em auto em `model/table.rs:772-778`.

### Decisão proposta

Adicionar classificação explícita capaz de distinguir `Auto`,
`Header { level positivo, scope: Column|Row|Both }` e `Data`. `Auto` permite
que o contexto `table.header`/`table.footer` derive a classe; Header/Data
explícitos vencem essa derivação. Em particular, `pdf.data-cell` dentro de
header permanece Data e nunca é promovido de volta a Header.

`map_content`, `map_text`, constructors, clone e hash preservam kind/level/
scope junto dos campos existentes. `plain_text`, body e render visual não são
alterados. A representação Rust pode usar enum fechado em vez de três campos;
não pode codificar a classe por string, style visual ou posição inferida.

## D08B1 — projeção existente, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz
registra UTC, diff HEAD --stat, hashes de fontes/binários, argv/cwd e resultados
contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Vanilla model/table.rs:734–778 declara body e propriedades; Tekt
entities/elements/table_cell.rs:62–109 não implementa getter; content/identity.rs:107
usa table.cell; content_methods.rs:226–254 não projeta a função real;
rules/show.rs:300–320 omite a nativa. Body/fields/func-eq e show explícito
refutam paridade. É semântica/morfologia, não igualdade Rust ou bytes PDF.
Não se infere intenção da estrutura upstream.

### Responsabilidade exclusiva

Getter Element expõe body e valores Some de x/y/colspan/rowspan/stroke/fill/align/inset/breakable. Inset uniforme projeta Relative com rel zero; não uniforme projeta dict left/top/right/bottom. Não expor kind interno.

### Limites e aceitação

RED→GREEN e coleta bilateral; cargo build e crystalline-lint sem erros.
Esta correção não muda contrato público de tipos/assinaturas nem fase.
D08B permanece incompleto: D08B2 completa presença omitido/auto/none por
contrato próprio; D08B3 completa realização de posições/estilos/padding/show.
Option None não prova omissão nem none explícito: não certificar esses casos.

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

Adicionar ao owner da entidade os tipos públicos fechados:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CellFieldPresence { #[default] Omitted, Auto, Explicit }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TableCellPresence {
    pub x: CellFieldPresence, pub y: CellFieldPresence,
    pub colspan: bool, pub rowspan: bool,
    pub fill: CellFieldPresence, pub stroke: bool,
    pub align: CellFieldPresence, pub inset: CellFieldPresence,
    pub breakable: CellFieldPresence,
}
```

TableCellElem recebe `pub presence: TableCellPresence`; todos os campos
atuais conservam seus tipos. Body permanece sempre conhecido. Omitted não
projeta campo; Auto projeta Value::Auto; Explicit projeta o valor conservado.
Somente fill/stroke explícitos podem ter Option None e projetam Value::None.
X/y/align/inset/breakable Explicit exigem Some; Auto exige None. Colspan e
rowspan presentes exigem Some positivo. Não codificar presença no kind,
em strings, Value genérico, PropMap ou registry. map_content/map_text,
clone/Hash preservam presence. Getter desconhecido continua None.


### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

### Morfologia do corpo vazio — refinamento interno medido

Antes da decisão, table-cell-presenca-2026-10-07/after.json.gz, caso
empty-body, registra body {func:empty} no Tekt e {func:sequence,children:[]}
no vanilla pinado, com HEAD/working tree/fontes/UTC/binários.
Getter body projeta Content::Empty armazenado como Sequence vazio da linguagem,
sem mutar body, alterar fase ou fabricar propriedades. Body não vazio permanece
com sua morfologia original. RED→GREEN e nova medição bilateral obrigatórios.

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

Adicionar em entities/elements/table_cell os tipos/carrier exatos:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableCellStage { Prepared, RecipeApplied, Complete }
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellResolved {
    pub x: usize, pub y: usize,
    pub colspan: usize, pub rowspan: usize,
    pub fill: Option<Color>, pub stroke: Option<Stroke>,
    pub align: Option<Align2D>, pub inset: Sides<Length>,
    pub breakable: bool, pub visual: Content,
    pub(crate) stage: TableCellStage,
}
```

TableCellElem recebe `pub source_span: Span` e
`pub resolved: Option<Arc<TableCellResolved>>`. Os campos anteriores e presence
permanecem lexicais. Constructor cru tem resolved=None; source_span é causal,
não propriedade da linguagem. Body getter conserva o corpo original (incluindo
Sequence vazia canônica); resolved Some faz os demais nove getters projetarem
valores conhecidos. Align None resolvido é Auto conhecido, fill/stroke None
resolvidos são None conhecidos. A projeção visual usa resolved.visual, nunca
substitui body lexical. Clone/maps/hash/observação conservam o carrier e stage.
Somente stage Complete pode chegar à publicação/render; Prepared é estado de
resolução e RecipeApplied é snapshot de it cuja regra já foi consumida, mas cujo
corpo ainda deve ser realizado se emitido. Aliases desse snapshot não reaplicam
a própria receita. Essa marca é nominal, privada à crate, sem Location/registry.


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

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Adicionar somente `pub standalone: Option<Arc<TableCellStandaloneResolved>>` a TableCellElem, preservando source_span/resolved/body/presence e todos os campos atuais. O owner da entidade declara o tipo público fechado:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellStandaloneResolved {
    pub visual: Content,
    pub(crate) stage: TableCellStage,
    pub(crate) guards: Vec<crate::entities::show::RuleId>,
}
```

TableCellResolved existente recebe somente `pub(crate) guards: Vec<RuleId>`; stage continua privado Prepared/RecipeApplied/Complete. Não acrescentar campo, método ou assinatura pública adicional. `resolved` e `standalone` são mutuamente exclusivos; constructor novo tem ambos None e nenhum guard consumido. Cell colocada usa somente resolved, Cell isolada usa somente standalone. Ao colocar uma Cell ainda lexical em Table, a preparação de Table substitui qualquer estado de inserção anterior pelo estado da nova ocorrência conforme a origem aceita; nunca fabricar x/y=0 para auto. Aliases do argumento preparado conservam guards da ocorrência. Recém-construída não herda guards da Cell externa, mesmo com body igual.

Apenas Cell preparada avulsa projeta defaults intrínsecos através de getters: x/y/fill/align/inset/breakable omitidos viram Auto; colspan/rowspan omitidos viram Int(1); stroke omitido vira dict vazio. Propriedade lexical explicitamente conhecida vence default, preservando None/Auto e valores tipados existentes. Body permanece fonte lexical/morfologia original. Nenhum default da Table é herdado fora de sua ocupação. Getter cru segue unknown/omission de D08B2. Default stroke vazio não é Stroke default nem None. O estado visual não expõe propriedade nova da linguagem. Não usar Value/strings/styles como armazenamento genérico de carrier nem registry; Value é somente projeção dos getters.

Possuir defaults/getters e carrier; plain_text/is_empty/map_content/map_text consomem visual preparado exclusivo e preservam body lexical. Helpers de visual/estado privados à crate podem evitar duplicação, sem dependência de compiler/render. Não mudar assinatura pública de Element ou constructor.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.
