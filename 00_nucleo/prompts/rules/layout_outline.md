# L0 — Layout: Tabela de Conteúdos
Hash do Código: f8cca175

## Módulo
`01_core/src/rules/layout/outline.rs`

## Propósito
Encapsula o braço `Content::Outline`. Lê `headings_for_toc` do
`Introspector` injectado e gera a sequência visual da TOC respeitando os
parâmetros do `OutlineElem` (`title`, `depth`, `indent`).

## Regras de negócio
- Não faz introspecção — apenas consome `headings_for_toc` já populado.
- Título da TOC:
  - `e.title.clone()` se presente;
  - caso contrário, heading de nível 1 com corpo `"Índice"`.
- Profundidade (P457): ignora entradas com `level > e.depth`.
- Indentação (P457/P502): consome `e.indent.is_active()`. `OutlineIndent::Auto`
  e `OutlineIndent::Bool(true)` activam indentação; `Bool(false)` desactiva.
  `Length`/`Function` são aceites mas tratados como `Auto` (renderização
  específica scope-out per ADR-0054).
- Para cada entrada, lê `runtime.known_page_numbers.get(&label)` antes de activar
  `is_readonly` para evitar borrow duplo.
- Activa `runtime.is_readonly = true` antes de `layout_content` e restaura
  `false` depois — bloqueia CounterUpdate/step durante o clone (DEBT-13).
- Número de página: `"  N"` se disponível em `known_page_numbers`; string vazia
  na Passagem 2 (draft). Acrescentado ao fim da linha.
- Não calcula números de página por si — lê-os do estado injectado.
- **Número da entrada (P359, DEBT-60 b, paridade vanilla).** A entrada usa o
  campo `number` devolvido por `headings_for_toc` (ex. `"1."`, `"1.1."`), **sem**
  o supplement `"Secção"`. Headings não-numerados → `number` é `None` e a linha
  começa directamente com o título.

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o título.
- `outline(depth: 1)` lista apenas headings de nível 1.
- `outline(indent: false)` não indenta entradas de nível 2 (corpo ainda
  presente; plain_text não preserva posição).
- `outline(title: [Sumário])` renderiza `"Sumário"` em vez de `"Índice"`.
- Heading de nível 2 com `indent: true` → linha indentada.
- Ausência de headings → TOC exibe apenas o título.
- `is_readonly = true` durante layout de cada linha → CounterUpdate no clone
  não avança contadores.

---

## P472 — List of Figures (`layout_lof`) e List of Tables (`layout_lot`)

### Despacho

No início de `layout_outline`, antes do caminho Headings:

```rust
match e.target {
    OutlineTarget::Figures => { layout_lof(layouter, e); return; }
    OutlineTarget::Tables  => { layout_lot(layouter, e); return; }
    OutlineTarget::Headings => {}
}
```

### `layout_lof`

1. Renderiza o título: `e.title.clone()` ou `Content::text("List of Figures")`.
2. Lê `layouter.introspector.figures_for_lof()` (slice de `(usize, String)`).
3. Para cada `(num, caption)`: emite `Content::text(format!("Figure {}  {}", num, caption))` + `flush_line()`.
4. Sem `is_readonly` (figuras não têm CounterUpdate no clone).
5. Divergência declarada: sem page numbers (requer 2-pass — scope-out).

### `layout_lot`

Idêntico a `layout_lof` mas com `tables_for_lot()` e prefixo `"Table"`.

### Scope-out explícito (P472)

- ~~Page numbers em LoF/LoT~~ — **resolvido em P488** (ver §P488 abaixo).
- LoF/LoT filtrado por `kind` (ex: só `"image"` vs `"table:custom"`) — futuro.
- `lof(title: [Custom])` / `lot(title: [Custom])` com Content arbitrário — suportado pelo campo `title: Option<Content>` mas sem testes de formatação rich.

---

## §P488 — Page numbers em LoF/LoT (2-pass)

**P488** implementa page numbers em `layout_lof` e `layout_lot` via mecanismo
de carry-forward entre iterações fixpoint (paralelo ao `known_page_numbers` da TOC).

### Mecanismo

`LayouterRuntimeState` recebe 4 novos campos (§P488):

```
figure_page_numbers: Vec<usize>   — páginas registadas nesta iteração (write)
table_page_numbers:  Vec<usize>   — idem para tabelas (write)
known_figure_page_numbers: Vec<usize>  — páginas da iteração anterior (read-only)
known_table_page_numbers:  Vec<usize>  — idem para tabelas (read-only)
```

`figure.rs::layout()` e `table.rs::layout()` registam a página actual quando
um elemento contado é renderizado. O `mod.rs` fixpoint lê `extracted_figure_page_numbers`
e `extracted_table_page_numbers` de `PagedDocument` e injeta como
`known_figure_page_numbers` / `known_table_page_numbers` na iteração seguinte.

### `layout_lof` pós-P488

```rust
let entries = layouter.introspector.figures_for_lof().to_vec();
let known_pages = layouter.runtime.known_figure_page_numbers.clone();

for (i, (num, caption)) in entries.iter().enumerate() {
    let page_num = known_pages.get(i).copied().unwrap_or(0);
    let line = if page_num > 0 {
        format!("Figure {}  {} . . . {}", num, caption, page_num)
    } else {
        format!("Figure {}  {}", num, caption)
    };
    layouter.layout_content(&Content::text(line));
    layouter.flush_line();
}
```

Matching por índice posicional (não por `figure_number`) — ambas as fontes seguem
ordem de documento.

### `layout_lot` pós-P488

Idêntico a `layout_lof` mas com `tables_for_lot()` e `known_table_page_numbers`.

### Convergência fixpoint

`mod.rs` fixpoint estendido: além de `extracted_label_pages == known_page_numbers`,
verifica `extracted_figure_page_numbers == known_figure_page_numbers` &&
`extracted_table_page_numbers == known_table_page_numbers` antes de retornar.

### Testes adicionados P488

- `p488_lof_sem_known_pages_usa_formato_sem_numero` — `known_figure_page_numbers` vazio → "Figure 1  Caption" (sem ". . . N")
- `p488_lof_com_known_pages_inclui_numero` — `known = [3]` → "Figure 1  Caption . . . 3"
- `p488_lot_com_known_pages_inclui_numero` — similar para tabelas
