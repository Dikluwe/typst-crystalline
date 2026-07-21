# Prompt L0 — `stdlib/structural` — módulo `structural`
Hash do Código: fa7ac28b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/structural.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos P69, P101, P154B, P155, P157A/B/C, P159A/C, P224, P295, P296, P296.1, P296.2, P397, P418/P419/P420/P429.
**ADRs**: ADR-0033 (divergência intencional), ADR-0037 (coesão por domínio), ADR-0054 (perfil graded / scope-outs), ADR-0060 (model structural roadmap), ADR-0061 (layout roadmap), ADR-0064 (smart option default), ADR-0107/ADR-0108/ADR-0109 (bibliografia CSL e asset).
**Convenções partilhadas**: ver `00_nucleo/prompts/engine/stdlib/_comum.md`.

---

## Módulo `structural` — funções nativas estruturais

Este módulo implementa as funções globais de conteúdo estrutural e semântico de Typst: estilo inline (strong/emph), verbatim (raw), separadores, termos, citações em bloco, tabela/grid, bibliografia/citação, hiperligações, notas de rodapé, elementos matemáticos accent/cancel/underover/op, e os wrappers de metadata `document`/`asset`.

Todas as funções partilham a assinatura padrão de `native_*`:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

A maioria ignora `ctx`/`world`/`current_file`; apenas `native_bibliography` usa `world`/`current_file` para carregar ficheiros `.bib`/`.yaml`/`.yml` e `ctx` para registar o estilo CSL resolvido.

---

### `native_strong(body)`

**Assinatura**: `strong(body: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`. Se omitido, produz `Content::Empty`.
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::Strong { body }` via `Content::strong(body)`. Serve como selector em show rules `#show strong: ...`.

**Paridade vanilla**: Equivalente a `#strong[body]` / `strong(body)`.

**Limitações / scope-outs**: Nenhuma conhecida nesta camada.

**Testes canónicos**:
```
strong([text]) -> Content::Strong { body: "text" }
strong("text") -> Content::Strong { body: "text" }
strong(123) -> Err "strong() espera content ou string"
strong([a], named:{x:1}) -> Err "argumento nomeado inesperado"
```

---

### `native_emph(body)`

**Assinatura**: `emph(body: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`. Se omitido, produz `Content::Empty`.
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::Emph { body }` via `Content::emph(body)`. Selector em show rules.

**Paridade vanilla**: Equivalente a `#emph[body]` / `emph(body)`.

**Limitações / scope-outs**: Nenhuma conhecida nesta camada.

**Testes canónicos**:
```
emph([text]) -> Content::Emph { body: "text" }
emph("text") -> Content::Emph { body: "text" }
emph(123) -> Err "emph() espera content ou string"
```

---

### `native_raw(text, lang:?, block:?)`

**Assinatura**: `raw(text: Str, lang: Str?, block: Bool?) -> Content`

**Argumentos**:
- 1º posicional `text`: `Str`. Se omitido, string vazia.
- `lang`: `Str` named opcional (default `None`).
- `block`: `Bool` named opcional (default `false`).

**Semântica**: Cria `Content::Raw { text, lang, block }` via `Content::raw(text, lang, block)`. Aceita apenas string porque não faz sentido semântico aceitar `Content` aqui.

**Paridade vanilla**: Equivalente a `` `text` `` inline ou bloco; `lang` é armazenado para syntax highlighting.

**Limitações / scope-outs**:
- Destaque de syntax real scope-out per ADR-0054 graded.

**Testes canónicos**:
```
raw("let x = 1") -> Content::Raw { text: "let x = 1", lang: None, block: false }
raw("fn main() {}", lang: "rust", block: true) -> Content::Raw { lang: Some("rust"), block: true }
raw() -> Content::Raw { text: "", lang: None, block: false }
raw([content]) -> Err "raw() espera string"
```

---

### `native_heading(level, body, numbering:?, outlined:?, bookmarked:?)`

**Assinatura**: `heading(level: Int, body: Content | Str, numbering: Str?, outlined: Bool?, bookmarked: Bool?) -> Content`

**Argumentos**:
- `level`: inteiro posicional 1..=6 (obrigatório).
- `body`: `Content` ou `Str` posicional (obrigatório).
- `numbering`: string named opcional. Se presente, activa numeração com o pattern indicado.
- `outlined`: booleano named opcional (default `true`). Se `false`, o heading não entra no índice do documento (`#outline()`). Quando `bookmarked` não é definido explicitamente, o valor efectivo de `bookmarked` segue `outlined`, pelo que `outlined: false` também exclui o heading da árvore de bookmarks PDF.
- `bookmarked`: booleano named opcional (default `auto`, que segue `outlined`). Controla a inclusão do heading na árvore de bookmarks PDF (`/Outlines`) independentemente de `outlined`. Quando explicitamente `false`, exclui só dos bookmarks; quando explicitamente `true`, força a inclusão nos bookmarks mesmo que `outlined` seja `false`.

**Semântica**: Cria um `Content::Heading(level, body, outlined, bookmarked)`. `HeadingElem` armazena `outlined: bool` e `bookmarked: Option<bool>` (`None` representa `auto`, i.e. segue `outlined`). O método `HeadingElem::is_bookmarked()` devolve `bookmarked.unwrap_or(self.outlined)`. Se `numbering` for `Some(pattern)`, embrulha o heading num `Content::Styled` que transporta `heading.numbering=true` e `heading.numbering.pattern=pattern`, a forma canônica de heading numerado. Também serve como selector em show rules (`#show heading: it => ...`).

**Paridade vanilla**: Equivalente a `heading(level, body, numbering: pattern, outlined: ..., bookmarked: ...)` em Typst 0.15.0. A partir de P606, o cristalino distingue `outlined` (índice do documento) de `bookmarked` (bookmarks PDF), com `bookmarked: auto` a seguir `outlined` por defeito.

**Limitações / scope-outs**: Nenhuma conhecida nesta camada para os parâmetros `outlined`/`bookmarked`.

**Testes canónicos**:
```
heading(1, [Título]) -> Content::Heading { level: 1, body: "Título", outlined: true, bookmarked: None }
heading(2, "Sub", numbering: "1.1") -> Content::Styled(Heading { level: 2, body: "Sub", outlined: true, bookmarked: None }, heading.numbering=true, heading.numbering.pattern="1.1")
heading(1, [X], outlined: false) -> Content::Heading { level: 1, body: "X", outlined: false, bookmarked: None }
heading(1, [X], bookmarked: false) -> Content::Heading { level: 1, body: "X", outlined: true, bookmarked: Some(false) }
heading(1, [X], outlined: false, bookmarked: true) -> Content::Heading { level: 1, body: "X", outlined: false, bookmarked: Some(true) }
heading(0, [X]) -> Err "level deve estar entre 1 e 6"
heading(1, 123) -> Err "body espera content ou string"
heading(1, [X], numbering: 1) -> Err "numbering espera string"
heading(1, [X], outlined: 1) -> Err "outlined espera bool"
heading(1, [X], bookmarked: 1) -> Err "bookmarked espera bool"
```

---

### `native_outline(title?, depth:?, indent:?)`

**Assinatura**: `outline(title: Content | Str?, depth: Int?, indent: Length | Function | Auto | Bool?) -> Content`

**Argumentos**:
- `title`: primeiro argumento posicional ou named `title`; `Content` ou `Str`. Se omitido, `None` (layout renderiza `"Índice"`).
- `depth`: inteiro named opcional (default `3`). Deve ser `>= 1`.
- `indent`: `Length`, `Function`, `Auto` ou `Bool` named opcional (default `auto`).
  - `bool` preserva compatibilidade com código cristalino existente (`true` = indentação ativa, `false` = inactiva).
  - `length`/`function` são aceites e armazenados; renderização específica é scope-out per ADR-0054.

**Semântica**: Cria `Content::Outline(Arc::OutlineElem { title, depth, indent: OutlineIndent::... })`. O layout substitui este nó pela lista de headings do documento, respeitando `depth` e `indent` e usando `title` customizado quando presente. Também é locatável (`ElementKind::Outline`) para `query("outline")`.

**Paridade vanilla**: Equivalente a `#outline(...)` do Typst 0.14.2/0.15.0 com `indent: length | function | auto`; `bool` é extensão de compatibilidade cristalina.

**Limitações / scope-outs**: Outros campos do vanilla (`fill`, `entry`, etc.) são scope-out por ora; `length`/`function` em `indent` são aceites mas renderização específica scope-out.

**Testes canónicos**:
```
outline() -> Content::Outline { title: None, depth: 3, indent: Auto }
outline([Sumário]) -> Content::Outline { title: Some("Sumário"), depth: 3, indent: Auto }
outline(title: [Sumário], depth: 1, indent: false) -> Content::Outline { title: Some("Sumário"), depth: 1, indent: Bool(false) }
outline(indent: 1em) -> Outline { indent: Length(1em) }
outline(indent: it => it) -> Outline { indent: Function(...) }
outline(depth: 0) -> Err "depth deve ser >= 1"
outline(title: 1) -> Err "title espera content ou string"
outline(depth: "x") -> Err "depth espera int"
outline([A], title: [B]) -> Err "não pode usar título posicional e named simultaneamente"
```

---

### `native_divider()`

**Assinatura**: `divider() -> Content`

**Argumentos**: Nenhum. Rejeita argumentos posicionais e nomeados.

**Semântica**: Emite `Content::Divider` via `Content::divider()`.

**Paridade vanilla**: Equivalente a `#line()` / `line()` (separador horizontal).

**Limitações / scope-outs**: Nenhuma conhecida nesta camada.

**Testes canónicos**:
```
divider() -> Content::Divider
divider(1) -> Err "divider() não aceita argumentos posicionais"
```

---

### `native_terms(...)`

**Assinatura**: `terms(term1: description1, term2: description2, ...) -> Content`

**Argumentos**:
- Não aceita argumentos posicionais.
- Argumentos nomeados `term: description`, onde `term` é a chave (string) e `description` é `Content` ou `Str`.
- A ordem dos argumentos nomeados é preservada (`IndexMap`).

**Semântica**: Converte cada par `(key, value)` num `Content::TermItem { term, description }` e emite `Content::Terms { items }`.

**Paridade vanilla**: Forma chave:valor em vez da sintaxe de markup `/ term: description`. Divergência intencional de superfície (ADR-0033).

**Limitações / scope-outs**:
- Forma posicional e separador customizado scope-out per ADR-0054 graded.

**Testes canónicos**:
```
terms(foo: "bar", baz: [qux]) -> Content::Terms with two TermItem
terms("bar") -> Err "terms() espera argumentos nomeados"
terms(foo: 123) -> Err "descrição de 'foo' deve ser content ou string"
```

---

### `native_quote(body, attribution:?, block:?, quotes:?)`

**Assinatura**: `quote(body: Content | Str, attribution: Content | Str | None, block: Bool, quotes: Bool) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str` (obrigatório).
- `attribution`: `Content`/`Str`/`none` (named; default `None`).
- `block`: `Bool` (named; default `false`).
- `quotes`: `Bool` (named; default `true`).

**Semântica**: Emite `Content::Quote { body, attribution, block, quotes }`.

**Paridade vanilla**: Equivalente a `#quote(...)` com os mesmos argumentos.

**Limitações / scope-outs**: Nenhuma conhecida nesta camada.

**Testes canónicos**:
```
quote([To be]) -> Content::Quote { body: "To be", attribution: None, block: false, quotes: true }
quote([To be], attribution: "Shakespeare", block: true, quotes: false) -> Content::Quote { ... }
quote() -> Err "quote() exige body"
quote([x], foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

### `native_par(body, leading:?)` — P806

**Assinatura**: `par(body: Content | Str, leading: Length?) -> Content`

**Origem**: P806 (achado #12 de P798) — `#par[...]` dava `unknown variable: par`;
o vanilla aceita (`ParElem`, `typst-library/src/model/par.rs`).

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str` (obrigatório). Em falta →
  `missing argument: body` (literal vanilla). Tipo errado →
  `expected content, found {type_name()}` (convenção do projecto; o vanilla
  diz "integer" onde o `type_name()` do projecto diz "int" — divergência
  pré-existente de nomes de tipo, registada, fora de âmbito).
- `leading`: `Length` (named). Aplicado envolvendo o body em
  `Content::Styled` com o custom `"par.leading"` — o mesmo canal do
  `#set par(leading:)` (F-5b/P373).
- Propriedades vanilla conhecidas mas **não honradas** (`justify`,
  `spacing`, `linebreaks`, `first-line-indent`, `hanging-indent`,
  `justification-limits`): aceites e **ignoradas em silêncio** (funções
  nativas não têm acesso ao `Sink` para avisar; `#set par` avisa porque
  corre no eval com engine). Registado como limitação.
- Named desconhecido → erro `argumento nomeado inesperado em par(): '{key}'`.

**Semântica**: Devolve o body **directamente** (sem wrapper) — no cristalino
os parágrafos são implícitos (texto plano em `Sequence`; não existe
`Content::Par` — `element_kind.rs`), logo o caso standalone
(`#par[conteúdo]` como documento) é idêntico ao vanilla. **Limitação
registada**: o vanilla `ParElem` é block-level (`Texto #par[x] fim` quebra
em 3 parágrafos); o cristalino emenda o body no fluxo corrente. Criar
`Content::Par` ou quebrar o fluxo é candidato a passo futuro se o caso
mid-paragraph aparecer em corpus real.

**Testes canónicos**:
```
par([abc]) -> Content "abc" (devolvido directamente)
par(leading: 20pt)[abc] -> Content::Styled(_, custom "par.leading"=20pt)
par(justify: true)[abc] -> Content "abc" (justify aceite e ignorado)
par() -> Err "missing argument: body"
par(5) -> Err "expected content, found int"
par([x], foo: 1) -> Err "argumento nomeado inesperado em par(): 'foo'"
```

---

### `native_table(...)`

**Assinatura**: `table(..children: Content | Str, columns:?, rows:?, stroke:?, fill:?) -> Content`

**Argumentos**:
- `..children`: variádicos posicionais `Content` ou `Str`.
- `columns`: `Array<TrackSizing>` (named; default `[Auto]`).
- `rows`: `Array<TrackSizing>` (named; default `[Auto]`).
- `stroke`: `Stroke` (named; paridade P227).
- `fill`: `Color` (named; paridade P228).

**Semântica**: Cria `Content::Table(Arc<TableElem { columns, rows, children, header, footer, stroke, fill }>)`. Children são distribuídos por colunas via `idx % num_cols` reutilizando o algoritmo de grid.

**P772v — `table.header(...)`/`table.footer(...)` como row-group real**: `header`/
`footer` **não** são argumentos nomeados (paridade vanilla — `#table(header: ..)`
erra com "argumento nomeado inesperado"). São extraídos do loop de resolução dos
children posicionais: `Content::TableHeader`/`Content::TableFooter` não
incrementam col/row; são guardados em `header`/`footer: Option<Content>` (só um
de cada — erro explícito em duplicado) e passados ao `TableElem`. Extensão
directa do mesmo mecanismo de `native_grid` (P772i) — ver
`00_nucleo/prompts/engine/layout.md` secção `grid.header(...)`/`grid.footer(...)`
para o mecanismo de row-group no layout.

**Paridade vanilla**: Subset minimal per ADR-0054 graded (P157A).

**Limitações / scope-outs** (ADR-0054 graded):
- `gutter` / `column_gutter` / `row_gutter`.
- `inset` / `align` / `fill` (adicionalmente scope-out em P157A, mas `fill` e `stroke` foram depois incluídos em P227/P228).
- `TableHLine`/`TableVLine`.

**Namespace anexado (P493b/P496):** a função `table` expõe `table.header`,
`table.footer` e `table.cell` via field access. Os nomes flat
(`table_header`, `table_footer`, `table_cell`) continuam disponíveis.

**Testes canónicos**:
```
table[A][B] -> TableElem { columns: [Auto], rows: [Auto], children: ["A", "B"], stroke: None, fill: None }
table(columns: (auto, auto), [A], [B], [C]) -> columns [Auto, Auto], children [A,B,C]
table(foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

### `native_table_cell(body, x:?, y:?, colspan:?, rowspan:?, stroke:?, fill:?, align:?, inset:?, breakable:?)`

**Assinatura**: `table_cell(body: Content | Str, x: Int | Auto | None, y: Int | Auto | None, colspan: Int | Auto | None, rowspan: Int | Auto | None, ...) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str` (obrigatório).
- `x`/`y`: `Int` ≥ 0, `auto` ou `none` (default `None`, i.e., auto-placement; ADR-0064 Caso A).
- `colspan`/`rowspan`: `Int` ≥ 1, `auto` ou `none` (default `None`, i.e., 1; ADR-0064 Caso C).
- `stroke`: `Stroke` (named; P230).
- `fill`: `Color` (named; P230).
- `align`: `Alignment` ou `Str` parseável (named; P235).
- `inset`: `Length` uniforme, `Float`/`Int` convertido para pt (named; P235).
- `breakable`: `Bool` (named; P235).

**Semântica**: Cria `Content::TableCell(Arc<TableCellElem { body, x, y, colspan, rowspan, stroke, fill, align, inset, breakable }>)`.

**Paridade vanilla**: `table.cell` acessível via namespace anexado em `table`; o nome flat `table_cell` continua disponível.

**Limitações / scope-outs**:
- `x`/`y`/`colspan`/`rowspan` são armazenados mas **ignorados em layout** — algoritmo de placement diferido em DEBT-34e (per ADR-0054 graded).
- Atributos de célula vanilla `kind` / `is_repeated` scope-out.

**Testes canónicos**:
```
table_cell([A]) -> TableCellElem { body: "A", x: None, y: None, colspan: None, rowspan: None, ... }
table_cell([A], x: 1, y: 2, colspan: 2, rowspan: 3) -> x Some(1), y Some(2), colspan Some(2), rowspan Some(3)
table_cell([A], colspan: 0) -> Err "table_cell(colspan:): valor 0 < 1"
```

---

### `native_table_header(body, repeat:?)` / `native_table_footer(body, repeat:?)`

**Assinatura**: `table_header(body: Content | Str, repeat: Bool) -> Content` / `table_footer(body: Content | Str, repeat: Bool) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str` (obrigatório).
- `repeat`: `Bool` (named; ADR-0064 Caso D; default `true`).

**Semântica**: Emite `Content::TableHeader { body, repeat }` / `Content::TableFooter { body, repeat }`.

**Paridade vanilla**: `table.header`/`table.footer` acessíveis via namespace anexado em `table`; os nomes flat `table_header`/`table_footer` continuam disponíveis.

**Limitações / scope-outs**:
- `repeat` armazenado mas **ignorado em layout** — repetição em page breaks diferida em DEBT-56 (refactor multi-region).
- `level: NonZeroU32` e `repeat-rows: Smart<usize>` scope-out.
- Children estruturados `Vec<TableItem>` divergem — cristalino usa `body` simples.

**Testes canónicos**:
```
table_header([H]) -> TableHeader { body: "H", repeat: true }
table_footer([F], repeat: false) -> TableFooter { body: "F", repeat: false }
table_header(123) -> Err "table_header() espera content ou string"
```

---

### `native_grid_cell(body, x:?, y:?, colspan:?, rowspan:?, stroke:?, fill:?, align:?, inset:?, breakable:?)`

**Assinatura**: idem `table_cell`.

**Argumentos**: Idem `table_cell`, mas para `GridCell`.

**Semântica**: Cria `Content::GridCell(Arc<GridCellElem { body, x, y, colspan, rowspan, stroke, fill, align, inset, breakable }>)`.

**Paridade vanilla**: Naming flat `grid_cell` em vez de `grid.cell` (ADR-0033).

**Limitações / scope-outs**:
- Diferente de `table_cell`, `x`/`y`/`colspan`/`rowspan` são **resolvidos pelo Grid layouter** (DEBT-34e fechada em P224.C).
- `align`/`fill`/`stroke`/`inset`/`breakable` per-cell armazenados; override em layout reutiliza `.or()` contra valores de grid-level.

**Testes canónicos**:
```
grid_cell([A], x: 1, colspan: 2) -> GridCellElem { x: Some(1), colspan: Some(2), ... }
grid_cell(123) -> Err "grid_cell() espera content ou string"
```

---

### `native_grid_header(body, repeat:?)` / `native_grid_footer(body, repeat:?)`

**Assinatura**: `grid_header(body: Content | Str, repeat: Bool) -> Content` / `grid_footer(body: Content | Str, repeat: Bool) -> Content`

**Argumentos**: Idem `table_header`/`table_footer`.

**Semântica**: Emite `Content::GridHeader { body, repeat }` / `Content::GridFooter { body, repeat }`.

**Paridade vanilla**: Naming flat (ADR-0033).

**Limitações / scope-outs**:
- `repeat` armazenado mas **ignorado em layout** (paridade P157C; DEBT-56).

**Testes canónicos**:
```
grid_header([H]) -> GridHeader { body: "H", repeat: true }
grid_footer([F], repeat: false) -> GridFooter { body: "F", repeat: false }
```

---

### `native_bibliography(...)`

**Assinatura**: `bibliography(entries: Array<Dict> | Str, title:?, style:?, locale:?) -> Content`

**Argumentos**:
- `entries`: `Array<Dict>` posicional ou named; ou `Str` como path para ficheiro `.bib`/`.yaml`/`.yml` (P419/P450).
- `title`: `Content`/`Str` (named; ADR-0064 Caso A; default `Content::heading(1, "Bibliography")`).
  - Omitido → `BibliographyElem.title = Some(Content::heading(1, "Bibliography"))` (P479 — paridade vanilla: a bibliography tem sempre um título que é um `heading` de nível 1).
  - `title: none` → `BibliographyElem.title = None` (sem título).
  - `title: "Custom"` ou `title: [Custom]` → `BibliographyElem.title = Some(conteúdo do arg)` (texto/content sem wrapper de heading — comportamento user-facing).
- `style`: `Str` (named; nome CSL built-in como `"ieee"`, `"apa"`).
- `locale`: `Str` (named; locale override como `"en-US"`, `"pt-PT"`).

**Semântica**: Constrói `Content::Bibliography(Arc<BibliographyElem { entries, path, title, style, locale }>)`. Cada Dict de entrada valida 4 campos obrigatórios (`key`, `author`, `title`, `year` com `year >= 0`) e 12 campos opcionais string (`volume`, `pages`, `journal`, `publisher`, `url`, `doi`, `editor`, `series`, `note`, `isbn`, `location`, `organization`).

**P479 — título padrão como heading**: quando o utilizador não especifica `title:`, `BibliographyElem.title` recebe `Some(Content::heading(1, Content::text("Bibliography")))`. Isto torna o título da bibliografia visível ao introspector pré-layout (`walk` arm `Content::Bibliography` recursivo em `e.title`), alinhando a contagem de headings com o vanilla (que gera um heading para o título da bibliografia). O `bibliography::layout` renderiza `e.title` via `layout_content` sem alteração. A regra "medir antes de decidir" (ADR-0108) confirma: corpus `visual/cite-bibliography.typ` → cristalino=0, vanilla=1 antes de P479.

Carregamento por path (P419/P450):
- `.bib` → parser BibTeX minimal custom (`rules/eval/bibtex.rs`); suporta os tipos
  `article`, `book`, `inproceedings`, `misc`, `phdthesis`, `techreport`.
- `.yaml`/`.yml` → parser `hayagriva::io` existente.
- `.json` scope-out.

Se `style` for fornecido, resolve-o em eval time (built-in ou path `.csl` via `World::read_bytes`) e regista-o no `EvalContext` para transporte até ao `BibStore` (P420/P429).

**Paridade vanilla**: Subset linguístico per ADR-0054 graded. Input literal e por path suportados.

**Limitações / scope-outs**:
- Renderização CSL real em `engine/layout/bib_csl.rs` — ver scope-out dedicado abaixo.
- `sources` (parsing externo genérico), `full`, `lang`, `region` scope-out.

**Testes canónicos**:
```
bibliography(((key: "k", author: "A", title: "T", year: 2024),)) -> BibliographyElem with one BibEntry
bibliography("refs.bib") -> BibliographyElem { path: Some("refs.bib"), entries: loaded }
bibliography(((key: "k", author: "A", title: "T", year: -1),)) -> Err "year espera int >= 0"
```

---

### `native_cite(key, supplement:?, form:?, style:?)`

**Assinatura**: `cite(key: Str, supplement: Content | Str | None, form: Str | Auto | None, style: Str | None) -> Content`

**Argumentos**:
- 1º posicional `key`: `Str` não vazia.
- `supplement`: `Content`/`Str`/`none` (named; default `None`).
- `form`: `"normal"`, `"prose"`, `"author"`, `"year"`, `auto` ou `none` (named; default `None`, resolvido a `Normal` em layout).
- `style`: `"numeric"`, `"author-date"`, `"alphabetic"` ou `none` (named; **P468**; default `None`, resolvido a `Numeric` em layout).

**Semântica**: Emite `Content::Cite { key, supplement, form, style }`.

**P468 — CitationStyle**:
`style` aceita strings lowercase canonical via `extract_citation_style`. `None` resolve a `CitationStyle::Numeric` em layout (default cristalino). Strings inválidas produzem erro com lista dos valores válidos.

**Paridade vanilla**: Subset básico de citações.

**Limitações / scope-outs**:
- Não valida `key ∈ Bibliography.keys` — introspection runtime adiada per ADR-0017.
- Propagação automática de `bibliography.style` para cite — scope-out.
- Renderização CSL real em `engine/layout/cite.rs`.

**Testes canónicos**:
```
cite("k") -> CiteElem { key: "k", supplement: None, form: None, style: None }
cite("k", supplement: "p. 3", form: "prose") -> CiteElem { supplement: Some("p. 3"), form: Some(Prose) }
cite("k", style: "numeric") -> CiteElem { style: Some(Numeric) }
cite("k", style: "author-date") -> CiteElem { style: Some(AuthorDate) }
cite("") -> Err "cite() key não pode ser vazia"
cite("k", form: "bad") -> Err "form 'bad' inválido"
cite("k", style: "bad") -> Err "cite(): style 'bad' inválido ..."
```

---

### `native_link(url, body?)`

**Assinatura**: `link(url: Str, body: Content) -> Content`

**Argumentos**:
- 1º posicional `url`: `Str` não vazia.
- 2º posicional `body`: `Content` (opcional; se omitido, body = URL como texto).
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::Link { url, body }`.

**Paridade vanilla**: Equivalente a `#link("url")[body]`.

**Limitações / scope-outs**: Layout real de hiperligações (`FrameItem::Link`, bbox, links internos, sublinhado azul) — ver `00_nucleo/prompts/engine/layout/link.md`.

**Testes canónicos**:
```
link("https://x.com") -> LinkElem { url: "https://x.com", body: "https://x.com" }
link("https://x.com", [clique]) -> LinkElem { body: "clique" }
link("") -> Err "URL não pode ser vazia"
link("u", 123) -> Err "body deve ser content"
```

---

### `native_footnote(body, numbering:?)`

**Assinatura**: `footnote(body: Content | Str, numbering: Str?) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str` (obrigatório).
- `numbering`: `Str` named opcional (default `None`).

**Semântica**: Emite `Content::Footnote { body, numbering }`.

**Paridade vanilla**: Fase 1 P295 marker-only; `numbering` é aceite e armazenado.

**Limitações / scope-outs** (ADR-0054 graded):
- Uso de `numbering` no marcador de rodapé scope-out.
- `FootnoteBody::Reference(Label)` (multi-ref) scope-out.
- Body armazenado mas renderização no rodapé diferida.

**Testes canónicos**:
```
footnote([nota]) -> FootnoteElem { body: "nota", numbering: None }
footnote([nota], numbering: "1.") -> FootnoteElem { body: "nota", numbering: Some("1.") }
footnote(123) -> Err "footnote() espera content ou string"
```

---

### `native_accent(base, accent)`

**Assinatura**: `accent(base: Content | Str, accent: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `base`: `Content` ou `Str`.
- 2º posicional `accent`: `Content` ou `Str`.
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::MathAccent { base, accent }`.

**Paridade vanilla**: Equivalente a `accent(base, accent)` em math mode.

**Limitações / scope-outs** (P296, ADR-0054 graded):
- Cosméticos `size`, `dotless`, `inverted` scope-out.
- Shaping real de acentos matemáticos (OpenType math) — ver scope-out dedicado abaixo.

**Testes canónicos**:
```
accent("x", ".") -> MathAccent { base: "x", accent: "." }
accent("x") -> Err "accent() exige accent"
```

---

### `native_cancel(body)`

**Assinatura**: `cancel(body: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`.
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::MathCancel { body }`.

**Paridade vanilla**: Equivalente a `cancel(body)` em math mode.

**Limitações / scope-outs** (P296, ADR-0054 graded):
- `length`, `inverted`, `cross`, `angle`, `stroke` scope-out.
- Renderização real do cancelamento (traço diagonal) — ver scope-out dedicado abaixo.

**Testes canónicos**:
```
cancel("x") -> MathCancel { body: "x" }
cancel() -> Err "cancel() exige body"
```

---

### `native_underover(base, under:?, over:?)`

**Assinatura**: `underover(base: Content | Str, under: Content | Str | None, over: Content | Str | None) -> Content`

**Argumentos**:
- 1º posicional `base`: `Content` ou `Str`.
- `under`: `Content`/`Str`/`none` (named; default `None`).
- `over`: `Content`/`Str`/`none` (named; default `None`).

**Semântica**: Emite `Content::MathUnderover { base, under, over }`. Agregação cristalina de vários elementos vanilla (`underline`, `overline`, `underbrace`, `overbrace`, etc.) num único variant.

**Paridade vanilla**: Divergência intencional ADR-0033 / ADR-0054 graded — Typst vanilla fragmenta em 12+ elementos.

**Limitações / scope-outs**:
- Renderização real de under/over (posicionamento, braces extensíveis) — ver scope-out dedicado abaixo.

**Testes canónicos**:
```
underover("x", under: "i") -> MathUnderover { base: "x", under: Some("i"), over: None }
underover("x", over: "j") -> MathUnderover { ..., over: Some("j") }
underover(123) -> Err "base espera content ou string"
```

---

### `native_op(text, limits:?)`

**Assinatura**: `op(text: Content | Str, limits: Bool) -> Content`

**Argumentos**:
- 1º posicional `text`: `Content` ou `Str`.
- `limits`: `Bool` (named; default `false`).

**Semântica**: Emite `Content::MathOp { text, limits }`. `limits: true` afecta o layout de `MathAttach` (heurística de limit-style).

**Paridade vanilla**: Equivalente a `op("lim", limits: true)`.

**Limitações / scope-outs**:
- Shaping real de operadores matemáticos (tamanho, posicionamento de limits) — ver scope-out dedicado abaixo.

**Testes canónicos**:
```
op("lim", limits: true) -> MathOp { text: "lim", limits: true }
op("sin") -> MathOp { text: "sin", limits: false }
op(123) -> Err "op() text espera content ou string"
```

---

### `native_document(...)`

**Assinatura**: `document(title:?, author:?, date:?, keywords:?) -> Content`

Ver prompt dedicado `00_nucleo/prompts/engine/model/document.md`. Resumo:
- `title`: `Content` (named; default `None`).
- `author`: `Str` ou `Array<Str>` (named; default `[]`).
- `date`: `Datetime` (named; default `None`).
- `keywords`: `Str` ou `Array<Str>` (named; default `[]`).
- Rejeita argumentos nomeados desconhecidos.
- Produz `Content::Document { title, author, date, keywords }` (metadata pura, no layout output).

---

### `native_asset(...)`

**Assinatura**: `asset(path: Str, kind:?) -> Content`

Ver prompt dedicado `00_nucleo/prompts/engine/model/asset.md`. Resumo:
- `path`: `Str` (posicional ou named `path`; obrigatório).
- `kind`: `Str` (named; se omitido, infere da extensão: `png/jpg/...` → `"image"`, `ttf/otf/...` → `"font"`, `json/yaml/csv/xml/...` → `"data"`, outro → `None`).
- Produz `Content::Asset { path, kind }` (placeholder, sem layout output).

---

### `native_list(..items, marker:?, marker-align:?, indent:?, body-indent:?, tight:?)` — P470/P504/P505

**Assinatura**: `list(..items: Content | Str, marker: Str?, marker-align: Align?, indent: Length?, body-indent: Length?, tight: Bool?) -> Content`

**Argumentos**:
- `..items`: variádicos posicionais `Content` ou `Str`.
- `marker`: `Str` named opcional. Se omitido, marcador default (`"•"`) via
  `ListMarker::Default`; se presente, `ListMarker::Custom(marker)`.
- `marker-align`: `Align` named opcional (P504). Aceite e propagado para cada
  item; efeito visual scope-out.
- `indent`: `Length` named opcional (P505). Indentação do marker em relação à
  margem esquerda. Default `0pt`.
- `body-indent`: `Length` named opcional (P505). Indentação do corpo do item em
  relação ao marker. Default `0pt`.
- `tight`: `Bool` named opcional (P505). `true` (default) não adiciona espaço
  entre itens; `false` adiciona espaçamento de parágrafo entre itens.

**Semântica**: Para cada item posicional, cria um `Content::ListItem(Arc<ListItemElem { body: item, marker, marker_align, indent, body_indent, tight }>)`. Devolve `Content::Sequence` de todos os itens (ou item único se só um). Sem itens → `Content::Empty`.

**Paridade vanilla**: Equivalente a `#list(marker: "→", indent: 1.5em, body-indent: 0.5em, tight: false, [a], [b])` do Typst vanilla. Divergência mecânica: os valores `indent`/`body-indent`/`tight` são replicados em cada `ListItemElem` (o cristalino não materializa um container `ListElem`). A paridade é com a **linguagem** (semântica/resultado visual), não com a estrutura de dados (ADR-0107).

**Scope-out explícito (P470/P504/P505)**:
- Marcadores por nível (`marker: ([•], [–], [·])`) — futuro.
- `marker: Content` (marcador como bloco arbitrário) — futuro.
- Efeito visual de `marker-align` (P504) — campo aceite e propagado.

**Testes canônicos**:
```
list([a], [b]) -> Sequence[ListItem{body:"a", marker:None, indent:None, body_indent:None, tight:None}, ListItem{body:"b", marker:None, ...}]
list([a], marker: "→") -> Sequence[ListItem{body:"a", marker:Some(Custom("→")), ...}]
list([a], indent: 1.5em, body-indent: 0.5em, tight: false) -> Sequence[ListItem{..., indent:Some(1.5em), body_indent:Some(0.5em), tight:Some(false)}]
list(marker: "→") -> Empty  (sem itens)
list([a], marker: 1) -> Err "list(marker:) espera string ou array"
list([a], indent: "x") -> Err "list(indent:) espera length"
list([a], foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

### `native_lof(title:?)` — P472

**Assinatura**: `lof(title: Content?) -> Content`

**Argumentos**:
- `title`: `Content` named opcional. Se omitido, título default `"List of Figures"` em `layout_lof`.
- Não aceita argumentos posicionais.

**Semântica**: Produz `Content::lof(title)` — atalho para `Content::outline_with_target(title, OutlineTarget::Figures)`. Renderizado por `layout_lof` que itera `Introspector::figures_for_lof()`.

**Paridade vanilla**: Divergência intencional — vanilla não tem `lof()` global; cristalin introduz como função stdlib por conveniência.

**Scope-out (P472)**: page numbers em LoF (requer 2-pass — DEBT). Filtro por kind.

**Testes canónicos**:
```
lof() -> Content::Outline { target: Figures, title: None, depth: 3, indent: true }
lof(title: [Lista de Figuras]) -> Outline { target: Figures, title: Some("Lista de Figuras") }
lof(foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

### `native_lot(title:?)` — P472

**Assinatura**: `lot(title: Content?) -> Content`

**Argumentos**:
- `title`: `Content` named opcional. Se omitido, título default `"List of Tables"` em `layout_lot`.
- Não aceita argumentos posicionais.

**Semântica**: Produz `Content::lot(title)` — atalho para `Content::outline_with_target(title, OutlineTarget::Tables)`. Renderizado por `layout_lot` que itera `Introspector::tables_for_lot()`.

**Paridade vanilla**: Divergência intencional — vanilla não tem `lot()` global; cristalin introduz como função stdlib por conveniência.

**Scope-out (P472)**: page numbers em LoT (requer 2-pass — DEBT). Filtro por kind.

**Testes canónicos**:
```
lot() -> Content::Outline { target: Tables, title: None, depth: 3, indent: true }
lot(title: [Lista de Tabelas]) -> Outline { target: Tables, title: Some("Lista de Tabelas") }
lot(foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

### `native_enum(..items, numbering:?, start:?, indent:?, body-indent:?, tight:?)` — P470/P505

**Assinatura**: `enum(..items: Content | Str, numbering: Str?, start: Int?, indent: Length?, body-indent: Length?, tight: Bool?) -> Content`

**Argumentos**:
- `..items`: variádicos posicionais `Content` ou `Str`.
- `numbering`: `Str` named opcional. Se omitido, `EnumNumbering::Decimal` (default `"N."`);
  se presente, `EnumNumbering::from_pattern(s)`.
- `start`: `Int` named opcional (P470). Offset inicial (≥ 1). Default `1`.
- `indent`: `Length` named opcional (P505). Indentação do rótulo numérico em
  relação à margem esquerda. Default `0pt`.
- `body-indent`: `Length` named opcional (P505). Indentação do corpo do item em
  relação ao rótulo. Default `0pt`.
- `tight`: `Bool` named opcional (P505). `true` (default) não adiciona espaço
  entre itens; `false` adiciona espaçamento de parágrafo entre itens.

**Semântica**: Para cada item posicional com índice `i` (1-based) e `start` dado,
cria
`Content::EnumItem(Arc<EnumItemElem { number: Some(start + i - 1), body: item, numbering, indent, body_indent, tight }>)`.
Devolve `Content::Sequence` de todos os itens. Sem itens → `Content::Empty`.

**Paridade vanilla**: Equivalente a `#enum(numbering: "a)", indent: 1.5em, body-indent: 0.5em, tight: false, [primeiro], [segundo])`. Subset P470: apenas `"1."`, `"a)"`, `"A)"`, `"i)"`. Divergência mecânica: os valores `indent`/`body-indent`/`tight` são replicados em cada `EnumItemElem` (o cristalino não materializa um container `EnumElem`). A paridade é com a **linguagem** (semântica/resultado visual), não com a estrutura de dados (ADR-0107).

**Scope-out explícito (P470/P505)**:
- Patterns completos com prefixo/sufixo arbitrário.
- `full:` (numeração hierárquica).

**Testes canónicos**:
```
enum([a], [b]) -> Sequence[EnumItem{number:Some(1), body:"a", numbering:None, indent:None, body_indent:None, tight:None}, EnumItem{number:Some(2), body:"b", numbering:None, ...}]
enum([a], [b], numbering: "a)") -> items com numbering: Some(LowerAlpha)
enum([a], numbering: "?!") -> item com numbering: Some(Custom("?!")) (pattern desconhecido armazenado)
enum([a], [b], start: 3) -> Sequence[EnumItem{number:Some(3), ...}, EnumItem{number:Some(4), ...}]
enum([a], indent: 1.5em, body-indent: 0.5em, tight: false) -> Sequence[EnumItem{..., indent:Some(1.5em), body_indent:Some(0.5em), tight:Some(false)}]
enum([a], numbering: 1) -> Err "enum(numbering:) espera string"
enum([a], indent: "x") -> Err "enum(indent:) espera length"
enum([a], foo: 1) -> Err "argumento nomeado inesperado 'foo'"
```

---

## Scope-outs transversais (P430)

| Área | Scope-out | Notas |
|------|-----------|-------|
| **CSL styling** (`bibliography` / `cite`) | Renderização real de estilos CSL (IEEE, APA, etc.), locales, ficheiros `.csl` customizados, múltiplas bibliografias, `title` customizado. | Lógica vive em `rules/eval/bibliography.rs` e `engine/layout/bib_csl.rs`; o struct armazena `style`/`locale`. |
| **OpenType shaping real** (`accent` / `cancel` / `underover` / `op`) | Posicionamento de acentos, traços de cancelamento, braces extensíveis, tamanho de operadores, limit placement. | Variants `MathAccent`/`MathCancel`/`MathUnderover`/`MathOp` existem em L1; layout matemático renderiza-os. |
| **Multi-region grid/table** | Repetição de `table_header`/`table_footer`/`grid_header`/`grid_footer` em page breaks; colocação algorítmica real em tabelas (DEBT-34e para table; grid já resolvido em P224.C). | Campos `repeat` e `x/y/colspan/rowspan` de `TableCell` são armazenados mas ainda não consumidos pelo layouter. |

---

## Critérios de Verificação

```
// strong / emph / raw
strong([A]) -> Content::Strong { body: "A" };  strong("A") -> idem;  strong(1) -> Err
emph([A]) -> Content::Emph { body: "A" };  emph("A") -> idem;  emph(1) -> Err
raw("x") -> Content::Raw { text: "x", syntax: None, block: false };  raw([x]) -> Err

// heading / outline / divider
heading() -> Err "heading() como função directa não suportada"
outline() -> Content::Outline { title: None, depth: 3, indent: true }
outline([Sumário], depth: 1) -> Content::Outline { title: Some("Sumário"), depth: 1, indent: true }
divider() -> Content::Divider;  divider(1) -> Err

// terms / quote
terms(a: "b") -> Content::Terms with one TermItem;  terms("b") -> Err
quote([A]) -> Content::Quote { body: "A", block: false, quotes: true };  quote() -> Err

// table / table_cell / table_header / table_footer
table([A],[B]) -> TableElem { columns: [Auto], rows: [Auto], children: ["A","B"] }
table_cell([A], x: 1, colspan: 2) -> TableCellElem { x: Some(1), colspan: Some(2) }
table_cell([A], colspan: 0) -> Err
table_header([H]) -> TableHeader { body: "H", repeat: true }
table_footer([F], repeat: false) -> TableFooter { body: "F", repeat: false }

// grid / grid_cell / grid_header / grid_footer
grid_cell([A], x: 1, colspan: 2) -> GridCellElem { x: Some(1), colspan: Some(2) }
grid_header([H]) -> GridHeader { body: "H", repeat: true }
grid_footer([F], repeat: false) -> GridFooter { body: "F", repeat: false }

// bibliography / cite
bibliography(((key:"k", author:"A", title:"T", year:2024),)) -> BibliographyElem with one entry
bibliography(((key:"k", author:"A", title:"T", year:-1),)) -> Err
cite("k") -> CiteElem { key: "k" };  cite("") -> Err;  cite("k", form: "bad") -> Err

// link / footnote
link("u") -> LinkElem { url: "u", body: "u" };  link("u", [b]) -> LinkElem { body: "b" }
footnote([n]) -> FootnoteElem { body: "n" }

// math: accent / cancel / underover / op
accent("x", ".") -> MathAccent { base: "x", accent: "." }
cancel("x") -> MathCancel { body: "x" }
underover("x", under: "i") -> MathUnderover { base: "x", under: Some("i") }
op("lim", limits: true) -> MathOp { text: "lim", limits: true }

// document / asset
document(title: [T]) -> Content::Document { title: Some("T"), author: [], date: None, keywords: [] }
asset("logo.png") -> Content::Asset { path: "logo.png", kind: Some("image") }
asset("x.bin", kind: "custom") -> Asset { kind: Some("custom") }

// list / enum (P470/P504/P505)
list([a], [b]) -> Sequence[ListItem{marker:None, indent:None, body_indent:None, tight:None}, ListItem{marker:None, ...}]
list([a], marker: "→") -> ListItem com marker: Some(Custom("→"))
list([a], indent: 1.5em, body-indent: 0.5em, tight: false) -> ListItem com indent:Some(1.5em), body_indent:Some(0.5em), tight:Some(false)
list([a], marker: 1) -> Err "list(marker:) espera string ou array"
list([a], indent: "x") -> Err "list(indent:) espera length"
enum([a], [b]) -> Sequence[EnumItem{number:Some(1), numbering:None, indent:None, body_indent:None, tight:None}, EnumItem{number:Some(2), ...}]
enum([a], [b], numbering: "a)") -> items com numbering: Some(LowerAlpha)
enum([a], [b], start: 3) -> EnumItem{number:Some(3)}, EnumItem{number:Some(4)}
enum([a], indent: 1.5em, body-indent: 0.5em, tight: false) -> EnumItem com indent:Some(1.5em), body_indent:Some(0.5em), tight:Some(false)
enum([a], numbering: 1) -> Err "enum(numbering:) espera string"
enum([a], indent: "x") -> Err "enum(indent:) espera length"

// lof / lot (P472)
lof() -> Content::Outline { target: Figures, title: None }
lof(title: [Lista de Figuras]) -> Outline { target: Figures, title: Some(...) }
lot() -> Content::Outline { target: Tables, title: None }
lot(title: [Lista de Tabelas]) -> Outline { target: Tables, title: Some(...) }

// make_math_module — P480 (equation alias)
make_math_module().get("equation") == Value::None  // alias; namespace vanilla

// native_math_class — P772y
math.class("relation", "z") -> Content::MathClassOverride { class: Relation, body: "z" }
math.class("bad", "z") -> Err "class(): 'bad' não é uma MathClass reconhecida"
math.class("relation") -> Err "class() exige body como 2.º argumento posicional"
```

---

## P772y — `native_math_class` / `math.class(class, body)`

**Assinatura**: `class(class: Str, body: Content | Str | Symbol) -> Content`

**Argumentos**:
- 1º posicional `class`: `Str` — uma das 15 strings vanilla
  (`entities/math_class.rs::parse_math_class`; kebab-case só para
  `"glyph-part"`).
- 2º posicional `body`: `Content`, `Str`, ou `Symbol` (P471 — símbolo
  Unicode como body, paridade com a conversão de markup em `eval/mod.rs`).
- Não aceita argumentos nomeados.

**Semântica**: Emite `Content::MathClassOverride { class, body }` — força a
`MathClass` de `body` para efeitos de espaçamento automático
(`rules/math/layout/spacing.rs`), sem afectar o layout/renderização do
`body` em si. Ver `entities/elements/math_class_override.md`.

**Registo**: vive no **scope do módulo `math`** (`make_math_module()`), não
no scope global — ao contrário de `cancel`/`accent`, que são globais
(`rules/eval/mod.rs`).

**Paridade vanilla**: `ClassElem` (`math/mod.rs`) — `#[elem(Mathy)] pub
struct ClassElem { #[required] pub class: MathClass, #[required] pub body:
Content }`.

**Pré-requisito descoberto e resolvido no mesmo passo (P772y)**: o
avaliador de modo math (`rules/eval/math.rs::eval_math_expr`, arm
`Expr::FuncCall`) só despachava chamadas cujo callee fosse um
`Expr::MathIdent` bare (`cancel(x)`, `bb(x)`, ...) — um callee namespaced
como `math.class(...)` (`Expr::FieldAccess`) caía no `_ => return
Ok(Content::Empty)` e desaparecia silenciosamente. Ver
`rules/eval.md` §P772y para a extensão (`eval_math_callee`,
`eval_math_arg_value`) que resolve callees `FieldAccess` e permite
argumentos `Str` literais (não só `Content`) em chamadas math namespaced.

**Testes canónicos**:
```
class("relation", "z") -> MathClassOverride { class: Relation, body: "z" }
class("relation", sym.suit.heart) -> MathClassOverride { class: Relation, body: "♥" }
class("bad-name", "z") -> Err "não é uma MathClass reconhecida"
class("relation") -> Err "exige body como 2.º argumento posicional"
class() -> Err "exige o nome da classe como 1.º argumento posicional"
class("relation", "z", extra: 1) -> Err "argumento nomeado 'extra' não suportado"
```

---

## P480 — `equation` alias em `make_math_module`

`make_math_module()` insere `"equation": Value::None` no scope do módulo math
(**P731**: o módulo passou de `Value::Dict` a `Value::Module` — paridade
vanilla `type(math)` → `module`).

Razão: vanilla expõe `math.equation` como namespace de selector. Cristalino
regista aqui para que `scope.get("math").equation` resolva (mesmo que o valor
seja `Value::None` porque não existe `native_equation` em L1).

`parse_selector("math.equation")` em L3 `query-helpers.md` é o mecanismo
primário de paridade de selector — o scope entry é complementar.

---

## P726 — `fill: none` / `stroke: none` em `table`/`table.cell`/`grid.cell`

Mesmo achado de `layout.md` P726 (bloqueio cetz `canvas.typ:111,129`):
vanilla aceita `none` (= omitir o argumento) em `fill`/`stroke`; o
cristalino rejeitava com `"espera Color, recebeu none"`. Medido função a
função contra o vanilla (exit 0 em todos): `table(fill|stroke: none)`,
`table.cell(fill|stroke: none)`, `grid.cell(fill|stroke: none)`.

### Semântica de implementação

- `native_table` (`structural.rs:675-689`): `Some(Value::None)` junta-se
  a `None => None` nos matches de `stroke` e `fill` — idiom já usado em
  `caption` (`structural.rs:694`).
- Loops de `table_cell`/`grid_cell` (`structural.rs:875-882, 1102-1109`):
  - `"stroke"` → `if matches!(value, Value::None) { stroke = None } else { stroke = Some(extract_stroke(...)?) }`;
  - `"fill"` → braço `Value::None => fill = None` antes do erro de tipo.

`extract_stroke` não muda (ver `layout.md` P726). hline/vline
(`table.hline`/`table.vline`, stroke não-opcional na entidade):
scope-out medido — ver `layout.md` P726.
