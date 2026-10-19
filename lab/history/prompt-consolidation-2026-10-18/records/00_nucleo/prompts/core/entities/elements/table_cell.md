# Prompt L0 — `entities/elements/table_cell` — `TableCellElem`
Hash do Código: 65edc838

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:1d697fae328277525e44cad674cecc537ceb23a1bb3ae11bb68a0a9575411ae4


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
lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz
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

lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz e
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
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
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

lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

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

## D08B3-C — defaults declarativos da Cell, fluxo contínuo

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz registra 58 sondas, UTC 2026-10-07T15:24:58.038947+00:00 até 15:25:13.727872+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/status/fontes/argv/cwd/ambiente completos. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3. isolated/table-set, campos individuais, late/local/reuse/spread e rejeições refutam o warning de target vazio e a ausência dos defaults. padding-span-one/empty-fixed-rows fixam spans 1 em vazios; auto-table-fallback preserva Auto explícito como delegação à Table, não ao set da Cell. show-cell-set-self altera avulsa mas não fill conhecido da Cell posicionada. Stroke/alignment fazem fold. refuters-before.json.gz estende com header, partial inset, args/condição e snapshot em Table; alias-inside-table é válido no vanilla e recursa indevidamente no Tekt.

Fonte: upstream typst-library/src/model/table.rs:734–778 declara campos, e layout/grid/resolve.rs:203–278 lê estilos antes de resolver/sobrescrever posições/fill; defaults de padding TableCell packed em model/table.rs:785–800 pinam spans 1. Tekt rules/set.rs:23–27 usa text_str de target e não reconhece FieldAccess; table_cells.rs:262–310 usa apenas propriedades lexicais no placement/herança; grid_placement.rs:80 lê x/y/spans diretamente; TableCell standalone não possui propriedades efetivas privadas. É língua (campos/ordem/efeitos/estilo/erros); a representação privada abaixo é decisão Tekt, não obrigação de copiar Packed ou igualdade Rust. Hipótese refutável por qualquer negativo destas coletas, perda de fonte lexical, mutação de valor cru, efeito repetido, guard de alias perdido ou regressão de geometria/cache.

### Responsabilidade exclusiva

Adicionar somente tipo pub(crate) TableCellProperties e campo pub(crate) properties: Option<TableCellProperties> a TableCellStandaloneResolved (que já contém campos privados e não admite literals externos). Tipo fechado possui x/y/colspan/rowspan Option<usize>, fill Option<Color>, stroke Option<Stroke>, align Option<Align2D>, inset Option<Sides<Option<Length>>>, breakable Option<bool>, presence TableCellPresence. Não adicionar ou alterar campo/método/tipo público, assinatura de trait ou TableCellElem. Campos lexicais continuam fonte; getters avulsos preparados delegam à propriedade privada efetiva, com defaults intrínsecos para omitidos. Métodos privados de projeção/cópia/placement podem compartilhar esse schema. map/clone transportam propriedades privadas sem callbacks/render. Source/body/kind originais conservados.

### Gate, aceitação e limites

Correção interna de paridade/mapeamento, ADR-0127 contínuo: nenhum contrato público, compatibilidade externa, flag/default de produto ou fase nova. Props e helpers são privados em carrier já não-construível externamente; lexicais não mudam. L0 editado primeiro, resselo restrito e RED→GREEN obrigatórios. Coletas bilaterais com erros/caret/efeitos, fonte/binaries/HEAD/UTC, geometria real; build debug/release/suítes e lint sem erros. Preservar árvore anterior sem commit/reset/resselo global. Subconjunto certificado: inteiros/autos, Color/None/Auto, Stroke uniforme/None, alinhamento, Length/inset parcial de Length/Auto, Bool/Auto e schemas já tipados. Gradient/tiling/functions, strokes por lado, inset percentuais e rich casts de constructor, grupos/spans multipágina/PDF-UA, set/text posicional e defaults gerais de Table continuam incompletos, com medição/L0 próprios antes de ampliar. Não certificar por warning fallback nem copiar defeitos upstream.

Precisão medida em refuters-before: stroke-constructor-fold e align-constructor-fold preservam o construtor inteiro contra set da Cell. self-set-does-not-change-placement muda colspan observado para 2 após a ocupação já feita com 1; não é prova de nova ocupação.

### Precisão dos spans observados após show-set (refutador C)

Medição anterior: refuters-before.json.gz / self-set-does-not-change-placement, vanilla devolve colspan 2 na célula real x0 e colspan 1 nos paddings x1/x2. Fonte pinada layout/grid/resolve.rs:218–244 captura spans de ocupação mas não assenta seus campos no conteúdo; model/table.rs:785–800 assenta 1 nos paddings. Logo ocupação e getter posterior são observáveis distintos. Decisão privada: a Cell posicionada conserva TableCellResolved como plano físico; o carrier standalone privado pode coexistir apenas para propriedades observadas da receita, mantendo campos resolvidos e sobrescrevendo spans lexicalmente omitidos pela declaração de show-set. Getters/canonicalização consultam essa projeção; placement do plano completo continua usando TableCellResolved. Padding sem fonte tem spans lexicais 1 para que show-set não os altere. Guards/stage continuam causais, sem novo contrato público nem mutação da fonte original. RED de span getter antes de implementar, GREEN mais geometria/regressões obrigatórios.

### Fold de declarações de Stroke — medição extra C

extra-first.json.gz (fontes/HEAD/estado/UTC/binários completos) compara two-stroke-sets: set stroke:red seguido de 2pt produz 2pt + red no getter avulso e posicionado vanilla. stroke-render-inherits-cell-style resolve o constructor 2pt com paint red da Cell antes do fallback da Table 3pt, embora stroke-constructor-fold avulso exponha somente o constructor 2pt. Fonte pinada model/table.rs:765 declara #[fold]; resolve.rs:225–237 usa stroke.resolve(styles), distinto do getter cru. Decisão: declarações sucessivas foldam componentes specified ao publicar Value::Stroke declarativo; nenhum carrier/callback é guardado em Styles. Constructor stroke permanece inteiro no getter avulso; resolução posicionada folda o constructor com a base capturada Cell e depois Table. Adicionar somente campo privado stroke_inherited: Option<Stroke> em TableCellProperties para transportar essa base causal, sem assá-la no getter lexical. ObservationEq inclui base; canonicalização ignora base derivada de render. Table já resolvida tem base None. Clone/aliases conservam base anterior; None explícito vence. RED→GREEN antes de código; correção privada contínua ADR-0127, nenhum contrato público/fase/default novo. A cláusula de precedência inteira do constructor vale para o getter avulso, não para a resolução fold de stroke, conforme medição.

### Show-set acumulado e bordas — precisão C

show-fold-before.json.gz e geometry-final.json.gz registram perda de red ao acumular show-set e desenho preto 1pt com campos conhecidos red 2pt. Aplicar helper privado puro chain_styles da Cell na acumulação de deltas cell.stroke; delegar componente de desenho ao owner borders, que lê o plano externo conhecido, sem fallback de uma Cell resolvida. Todos os outros estilos/guards/ordem permanecem. Esse helper somente folda Value::Stroke declarativo, nunca carrier/layout/callback; não cria API pública, campo/fase/default novo. RED→GREEN e recoleta real de stroke são obrigatórios.

## D08B3-C — tinta do plano da Table separada do default show da Cell

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz conserva PDF/SVG, fontes, HEAD/working tree, UTC/argv/hashes completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. fill-with-body-recipe e fill-with-hidden-recipe conservam retângulo red no vanilla, ausente Tekt. stroke-with-body-recipe conserva red 2pt vanilla contra black 1pt Tekt; default-with-body-recipe conserva black 1pt nos dois. Fonte vanilla layout/grid/resolve.rs:218–237 e :270–282 retorna Cell.fill/stroke antes de seu body passar por show; typst-layout grid/layouter.rs desenha esses campos independentemente do default show. Tekt cell_body.rs:61–63 condiciona fill à sobrevivência do snapshot, borders.rs:43–48 lê stroke lexical. O refinamento A mediu espaçamento/ab, não tinta: sua generalização de sobrevivência para fill/stroke é refutada e supersedida por esta cláusula.

Decisão contínua interna ADR-0127: TableCell posicionada usa fill/stroke/breakable do plano externo TableCellResolved, independentemente de recipe it/body/substituição; None conhecido vence qualquer fallback de container. Apenas inset/alignment derivados do default show continuam dependentes do snapshot sobrevivente reconhecido por prepared_default, com geometria/medidas sob owners existentes. Borders consome diretamente stroke externo conhecido; Cell body emite o fill externo antes do produto visual. Não executa conteúdo ocultado nem muda fonte lexical, API pública, fases, render default novo ou política Grid/crua. Mantém bounds/coalescência, clipping/caudas, ordem de efeitos e estado causal. RED→GREEN de tinta com produto substituído, células conhecidas None e regressão visual SVG real; builds/suítes/lint/preservação necessários.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Medição regressions/presence empty-body: Source(Empty) deixou de projetar body como sequence vazia. Getter body conserva a canonicalização existente de Empty através de Source.body, sem alterar presença/casts/preparo ou campos públicos. Source não vira func empty onde body já era sequence na linguagem.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/core/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Getter projeta Stroke próprio via From<Stroke>. Fold do delta lê referências Stroke; substitui somente o Arc do delta pelo resultado completo, preservando o snapshot do base. Campos públicos TableCell/stroke continuam Option<Stroke>, presença e specified não mudam.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Reconstrução somente após mudança — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz` registra HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado/status/diffstat, UTC, script/fixtures/interpositor/CPU/comandos e release SHA-256 `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`: 483.823 pedidos glibc em local-set-1600, acima do gate pré-fixado de 470.000. `entities/content/traversal.rs:25-55,235-264` reconstrói sequências/carriers/estilos/Table/Cell mesmo se todos os callbacks retornam None; `entities/elements/table.rs:85-118` e `entities/elements/table_cell.rs:308-344` clonam metadados e contribuições visuais. O vanilla pinado visita referências e materializa apenas após mudança (`lab/typst-original/crates/typst-realize/src/lib.rs:242-291,353-410`). Isso é uma hipótese de custo, sem exigir sua vtable ou igualdade Rust (ADR-0107/0026/0109).

Decisão interna (ADR-0127): helper crate-private retorna None quando a contribuição visual fica inalterada. Prioridade de descida: standalone.visual, depois resolved.visual, senão body raw; nunca visitar a origem escondida junto com visual. Na mudança, clonar Cell e o plano aplicável preservando presence, stage, guards, props, origem e campos não-content; sem mudança, permitir ao owner `00_nucleo/prompts/core/entities/content/traversal.md` reutilizar o Arc original. O método público existente de Element conserva assinatura e usa a mesma implementação; não acrescentar método de trait, executar Engine ou preparar Cell em entidade. Testes de raw/resolved/standalone e gates pareados acima são obrigatórios.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
