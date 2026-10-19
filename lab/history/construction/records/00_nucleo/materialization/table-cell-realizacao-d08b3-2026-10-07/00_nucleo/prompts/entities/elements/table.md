# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: 4eb19477

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table.rs`
**Origem**: modelo D (ADR-0105), **Lote 12 P327** (bloco grid/table cell).
**P459**: campo `caption` opcional para numeração automática via `table.numbering`.
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**. Contentor —
`map_*` recursam em cada `children` e no `caption`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub hlines:   Vec<TableHLineElem>,  // P512
    pub vlines:   Vec<TableVLineElem>,  // P512
    pub header:   Option<Content>,      // P772v — row-group real, mesmo mecanismo de GridElem (P772i)
    pub footer:   Option<Content>,      // P772v
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
    pub caption:  Option<Content>,
}
```

**P772v**: `header`/`footer` guardam `Content::TableHeader`/`Content::TableFooter`
(um de cada, no máximo — erro explícito em duplicado, mesma regra de `GridElem`)
extraídos do loop de resolução de `native_table` (`stdlib/structural.rs`), não
argumentos nomeados (paridade vanilla: `table.header(...)`/`table.footer(...)`
são elementos-filho — ver `lab/typst-original/.../model/table.rs:495,525`).
Consumidos por `layout_grid` (`compiler/layout/grid.rs`) como row-group — mesmo
mecanismo de `GridElem.header`/`.footer`, ver
`00_nucleo/prompts/compiler/layout.md` secção `grid.header(...)`/`grid.footer(...)`.

`Content::Table { … }` → `Content::Table(Arc<TableElem>)`. Construtores
ergonómicos: `Content::table(columns, rows, children)` (caption `None`) e
`Content::table_with_caption(columns, rows, children, caption)` (P459).

> **Nota (P661):** a tabela com `caption` numerada por P459 continua a ser
> um `Content::Table`, não uma `figure`. Logo, não responde a show rules de
> `figure.where(kind: table)`.

> **`Hash` manual via Debug**: `TrackSizing` (colunas/linhas) e `Stroke`/`Color`
> carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`.
> `PartialEq` deriva.

## `impl Element for TableElem`

| método | comportamento |
|---|---|
| `plain_text` | `children` concatenados com espaço; `caption` prefixado quando presente (P459) |
| `is_empty` | `children.is_empty() && caption.as_ref().is_none_or(\|c\| c.is_empty())` |
| `map_content` | **recursivo** em cada `children`, `header`, `footer` e no `caption`, preserva columns/rows/hlines/vlines/stroke/fill |
| `map_text` | **recursivo** em cada `children`, `header`, `footer` e no `caption`, preserva columns/rows/hlines/vlines/stroke/fill |
| `get_field`/`element_kind`/`to_payload` | default (excepto `element_kind`/`to_payload`, P459 — ver código) |

## `eq`

`#[derive(PartialEq)]` compara todos os 10 campos (paridade `content.rs`).

## P1288 — summary semântico de acessibilidade (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/entities/elements/table.rs:29-42` mede `TableElem` sem summary.
A fonte vanilla pinada mede `summary: Option<EcoString>` interno em
`typst-library/src/model/table.rs:271-277` e `pdf.table-summary` substituindo
somente esse valor em `pdf/accessibility.rs:137-143`.

### Decisão proposta

Adicionar a `TableElem` um summary semântico opcional, distinto de caption e
texto visível. Só a omissão pública é convertida em `None` interno;
`summary: none` explícito é rejeitado no cast antes de alcançar a entidade;
string vazia permanece presente. Todo
constructor, clone, hash, igualdade mecânica, `map_content` e `map_text`
preserva o campo. `plain_text`/`is_empty` o ignoram, pois summary não é texto
visual nem body. `pdf.table-summary` substitui só esse campo.

O storage Rust exato é mecânica; o observável é a preservação do summary até
a estrutura `/Table` no PDF tagueado sem alterar texto, páginas, boxes ou
geometria.

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

Adicionar em entities/elements/table o carrier exato:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableGridResolved {
    pub cells: Vec<Content>,
    pub num_cols: usize, pub num_rows: usize,
    pub header_rows: usize, pub footer_start: usize,
    pub(crate) complete: bool,
}
```

TableElem recebe `pub source_span: Span`, `pub rows_explicit: bool` e
`pub resolved: Option<Arc<TableGridResolved>>`. Constructor cru tem resolved=None.
Rows explícitas são distinguidas do fallback [Auto] omitido; sem children e sem
rows explícitas não fabricar célula. Header/footer e children originais
permanecem como fonte lexical. Plano contém somente origens de células reais ou
vazias, não entries duplicadas dos spans; células são ordenadas row-major.
header_rows é faixa [0,n); footer_start é num_rows quando não há footer.
Getter/serialização não expõem esses carriers como campos novos da linguagem.
Travessia visual/emptiness/plain_text com resolved Some consomem cells completas
uma vez, não os children/header/footer crus em paralelo. Map/clone preservam
fontes e transportam visual; não executar receita em entidades.


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
