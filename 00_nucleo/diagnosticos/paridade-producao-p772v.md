# P772v — `table()` header/footer como row-groups (extensão do padrão de `grid()`)

> **Passo:** 772v
> **Data:** 2026-07-17
> **Commit-base:** working tree após P772u (não commitado no início deste passo).
> **Dependências:** P772i (mecanismo original, `row_group_cells`/`layout_grid` em `01_core/src/rules/layout/grid.rs`), P772f (achado original do scope-out #16, variante table).

---

## 1. Sonda — paridade de API `table.header`/`footer` vs `grid.header`/`footer`

```bash
grep -n "#\[elem(name = \"header\"\|#\[elem(name = \"footer\"" \
  lab/typst-original/crates/typst-library/src/model/table.rs
```

Note: `table.rs` vive em `model/`, não em `layout/` como `grid/mod.rs` — a
localização difere, mas a estrutura não.

`model/table.rs:495` (`TableHeader`) e `:525` (`TableFooter`) comparados
directamente com `layout/grid/mod.rs:580`/`:608` (`GridHeader`/`GridFooter`):
**estruturalmente idênticos** — mesmos campos (`repeat: bool` default `true`;
`level: NonZeroU32` default `1`, só no header; `children` variádico), mesma
doc ("Just like the @table.header element, the footer can repeat itself...").
Única diferença: `children: Vec<TableItem>` vs `Vec<GridItem>` (tipos de item
distintos, mesma forma).

Confirmado também que o vanilla não tem estilo visual por omissão específico
de `table()` para header/footer:

```
typst-layout/src/grid/mod.rs:111    fn layout_grid(...)  → GridLayouter::new(...).layout(engine)
typst-layout/src/grid/mod.rs:113-119 fn layout_table(...) → GridLayouter::new(...).layout(engine)  (idêntico)
```

Os dois wrappers são literalmente idênticos — nenhuma lógica de estilo
divergente entre `table()` e `grid()` no motor de layout. A hipótese de
"table() pode ter negrito/linha de destaque por omissão em headers" (nomeada
no passo como algo a não assumir) é **refutada** por leitura directa: não há
tal mecanismo em nenhum dos dois.

### Estado do cristalino antes deste passo

```bash
grep -n "struct TableElem\|header\|footer" 01_core/src/entities/elements/table.rs
```

`TableElem` (`01_core/src/entities/elements/table.rs`) **não tinha** campos
`header`/`footer` (ao contrário de `GridElem`, que os ganhou em P772i).
`native_table` (`stdlib/structural.rs`) tratava `Content::TableHeader`/
`TableFooter` no braço genérico do loop de resolução — mesmo scope-out #16 de
P772f, variante `table`. `TableHeaderElem`/`TableFooterElem`
(`entities/elements/table_header.rs`/`table_footer.rs`) já existiam com a
forma correcta (`body: Content, repeat: bool`) — o Modelo D (P320) já os tinha
criado; só faltava o wiring `TableElem` → `layout_grid`.

`native_table_header`/`native_table_footer` **já** colectavam todos os
argumentos posicionais (não só `.first()`) — corrigido em paralelo com
`native_grid_header`/`native_grid_footer` em P772i, mesmo sem o `TableElem`
ainda ter os campos para os consumir. Confirmado por leitura do código
(comentário explícito "P772i — colectar todos os argumentos posicionais, não
só o primeiro").

---

## 2. Implementação

### 2.1 L0s actualizados antes do código

- `00_nucleo/prompts/entities/elements/table.md` — struct `TableElem`
  actualizada com `hlines`/`vlines` (P512, já no código mas não documentados)
  e `header`/`footer` (P772v); `map_content`/`map_text`/`eq` actualizados.
- `00_nucleo/prompts/rules/layout.md` — secção "`grid.header(...)`/
  `grid.footer(...)` como row-groups (P772i)" ganhou subsecção "`table()` —
  extensão do mecanismo (P772v)", substituindo a antiga nota "Âmbito não
  coberto (`table()`)" que registava o scope-out.
- `00_nucleo/prompts/rules/layout/table.md` — passo 4 do layout de
  `Content::Table` actualizado para mencionar `header`/`footer`.
- `00_nucleo/prompts/rules/stdlib/structural.md` — secção `native_table`
  actualizada com a extracção de `header`/`footer` do loop de resolução.

### 2.2 Código

1. **`TableElem`** (`01_core/src/entities/elements/table.rs`): campos
   `header: Option<Content>`/`footer: Option<Content>` adicionados, espelhando
   `GridElem`. `map_content`/`map_text` recursam nos dois campos.
2. **`native_table`** (`01_core/src/rules/stdlib/structural.rs`): loop de
   resolução dos children posicionais ganhou dois braços novos —
   `Value::Content(c @ Content::TableHeader(_))` e
   `Content::TableFooter(_)` — que não incrementam col/row, guardam em
   `header`/`footer: Option<Content>` (erro explícito "não pode haver mais do
   que um header/footer" em duplicado), mesma lógica de `native_grid`
   (P772i). `header`/`footer` passados à construção final de `TableElem`.
3. **`layout_grid`** (`01_core/src/rules/layout/grid.rs`, motor partilhado por
   `Content::Grid` e `Content::Table` desde a atomização P379): os fechos de
   `header_cells`/`footer_cells` ganharam os braços
   `Content::TableHeader(e) => row_group_cells(&e.body, num_cols)` e
   `Content::TableFooter(e) => ...` — o motor `row_group_cells` já era
   genérico (preenche até múltiplo de `num_cols`, cola antes/depois das
   células normais); só faltava o braço de `match` para reconhecer os
   variantes `Table*`.
4. **`table.rs`** (`01_core/src/rules/layout/table.rs`, layout de
   `Content::Table`): a chamada a `layout_grid` passa
   `e.header.as_ref(), e.footer.as_ref()` em vez de `None, None`.

Nenhuma mudança em `TableHeaderElem`/`TableFooterElem`,
`native_table_header`/`native_table_footer` — já estavam correctos.

---

## 3. Validação

### 3.1 Coordenadas contra o vanilla (`mutool trace`)

```typst
#table(
  columns: 2,
  table.header[Nome][Idade],
  [Ana], [30],
  [Bruno], [25],
)
```

Vanilla (`lab/typst-original/target/release/typst`): 3 linhas, x-columns
75.866/113.773 (Libertinus Serif), y = 83.10 / 100.34 / 117.58 (espaçamento
constante 17.24pt entre as 3 linhas — header incluído).

Cristalino: 3 linhas, x-columns **idênticas** entre header e dados —
70.867/101.524 em **todas** as três linhas (header "Nome"/"Idade" na mesma
coluna x que "Ana"/"Bruno" e "30"/"25" respectivamente); y = 771.023 /
756.635 / 742.247 (espaçamento constante 14.388pt — diferença absoluta do
vanilla por causa da fonte de fallback usada nesta máquina, não da estrutura
— ver nota §3.3). Confirma por coordenadas, não só ausência de erro: header
ocupa a sua própria linha, colunas alinhadas com os dados, mesma disciplina
de P772i.

### 3.2 Bug do `.first()` não se repete

```typst
#table(columns: 2, table.header[A][B])
```

Confirmado (via teste automatizado, §3.4) que ambas as células ("A" e "B")
aparecem — `native_table_header` já as colectava todas desde P772i.

### 3.3 Nota — aviso de fonte não relacionado com este passo

`mutool draw`/`mutool trace` nesta máquina reportam "ignored error when
loading embedded font; attempting to load system font" mesmo para um
documento `Hello world` sem tabelas — **pré-existente, não introduzido por
P772v** (confirmado reproduzindo o mesmo aviso num documento trivial). Não
bloqueia a validação estrutural (posições/colunas), que é o que este passo
mede.

### 3.4 Testes automatizados novos

`01_core/src/rules/layout/tests.rs`, secção "P772v — table.header(...)/
table.footer(...) como row-groups" (mesmo padrão dos testes `p772i_grid_*`):

- `p772v_table_header_multi_celula_preserva_todas_as_celulas`
- `p772v_table_header_nao_desalinha_colunas_seguintes`
- `p772v_table_footer_aparece_apos_dados`
- `p772v_table_header_footer_nomeados_dao_erro`
- `p772v_table_multiplos_headers_da_erro`

### 3.5 Suite completa

```
cargo build --release --workspace --tests   → 0 erros
cargo test --workspace --release
  typst-core:   4197 passed, 0 failed  (+5 novos, P772v)
  typst-infra:   647 passed, 0 failed, 5 ignored
  typst-shell:    33 passed, 0 failed
  typst-wiring:    2 passed, 0 failed
  cli (integration): 29 passed, 0 failed
  crystalline_lint (integration): 2 passed, 0 failed
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente sobre
  package_version_resolution.md, não relacionado)
```

---

## Critério de fecho do passo

- [x] Paridade de API `table.header`/`footer` vs `grid.header`/`footer`
      confirmada contra o vanilla (estruturalmente idênticos; nenhum estilo
      visual por omissão divergente — `layout_table`/`layout_grid` são
      wrappers idênticos sobre o mesmo `GridLayouter`).
- [x] `TableElem` com campos `header`/`footer`.
- [x] `Content::TableHeader`/`TableFooter` distinguidos no loop de resolução
      de `native_table`.
- [x] Coordenadas confirmadas contra o vanilla (`mutool trace` — colunas
      idênticas entre header e dados, header na sua própria linha).
- [x] Múltiplas células em `table.header[A][B]` preservadas (bug `.first()`
      de P772i não se repetiu — já estava corrigido).
- [x] `cargo test --workspace` verde (4197 em typst-core, +5 novos).
- [x] `crystalline-lint .` zero violações.
- [x] L0s actualizados (4 ficheiros — `table.md` da entidade, `layout.md`,
      `layout/table.md`, `stdlib/structural.md`).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772v.md`.

---

## Próximo passo

Conforme P772t: resíduo de risco plausível do inventário original —
`foundations::target_`, `plugin_`, `image::pdf`, `layout::frame`, `math` —
23 itens em 5 módulos.
