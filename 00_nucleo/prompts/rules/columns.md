# Prompt L0 — `rules/columns` — Layout de colunas multi-página

Hash do Código: ccb812d3

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/rules/layout/columns.rs`  
**Ficheiros adjacentes**: `01_core/src/rules/layout/cursor.rs`, `01_core/src/rules/layout/mod.rs`  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), ADR-0054 (graded / scope-outs).

---

## Contexto

O consumer `Content::Columns` (e a forma sintética produzida por `#set page(columns: N)`) renderiza o corpo do documento em `count` colunas paralelas numa página. A implementação base (P537) suporta divisão explícita por `Content::Colbreak` e notas de rodapé por coluna. Este L0 estende o comportamento para corpos contínuos sem `colbreak()`, permitindo paginação automática entre colunas.

---

## Comportamento observável

### 1. Divisão explícita (`colbreak()`)

Quando o body contém `Content::Colbreak`, os segmentos delimitados por esses quebras são renderizados cada um numa coluna da mesma página. Se houver mais segmentos do que colunas, os segmentos excedentes continuam em colunas de páginas seguintes, uma coluna de cada vez.

### 2. Fluxo contínuo (sem `colbreak()`)

Quando o body não contém `colbreak()` suficientes para preencher todas as colunas, o mecanismo trata o body como um fluxo contínuo de texto/markup:

1. Preenche a primeira coluna da página actual até à altura útil da página.
2. Quando a primeira coluna enche, continua o conteúdo restante na segunda coluna da **mesma** página.
3. Repete até `count` colunas estarem cheias.
4. Só então cria uma nova página e continua na primeira coluna desta.
5. O processo repete até todo o body ser consumido.

### 3. Notas de rodapé

As notas de rodapé são colocadas no fundo da coluna onde são referenciadas (comportamento P537 preservado). Em modo de fluxo contínuo, o flush de notas acontece no fim de cada coluna preenchida.

### 4. Dimensões das páginas

Páginas criadas pelo mecanismo de colunas mantêm a largura total da configuração de página (`PageConfig.width`). Cada coluna ocupa `column_width` calculado como `(width - (count-1)*gutter) / count`. Não se criam páginas de largura de coluna.

---

## Implementação

A implementação fica no arquivo da feature (`rules/layout/columns.rs`), seguindo a forma B de ADR-0109. O `Layouter` expõe mecanismo suficiente para que `columns::layout` controle a passagem entre colunas e páginas:

- `columns::layout` configura o estado de colunas no `Layouter` (`page_columns`, `current_column`, `column_width`, `column_origin_x`, `column_mode`).
- O `Layouter` sabe que, quando está em modo colunas e `new_page()` é chamado por overflow de coluna, deve primeiro tentar avançar para a coluna seguinte da mesma página.
- Se ainda houver colunas livres na página actual, `new_page()` (ou o hook correspondente) guarda os items da coluna actual (com translação horizontal para a coluna correcta), reposiciona o cursor para o topo da próxima coluna e continua o layout.
- Se a página actual já usou todas as colunas, cria uma nova página física e reseta para a primeira coluna.
- Quando `columns::layout` termina, colunas parcialmente preenchidas são fechadas e os seus items são transladados para as posições finais.

A lógica de `flush_line`/`new_page` no `cursor.rs` mantém o critério de overflow (`cursor_y > height - margin`). A diferença está na ação de continuação: em vez de sempre criar uma nova página, pode avançar coluna dentro da página actual quando colunas estão activas.

---

## Restrições

- Não se remove o `match` exaustivo em `layout_content`.
- Não se introduz despacho dinâmico (`dyn`/vtable/PropMap).
- A lógica permanece em L1 (sem I/O, sem estado global).
- `#set page(columns: N)` a meio do documento continua scope-out per ADR-0054: a transformação AST `wrap_page_columns` aplica-se ao topo/escopo, não força nova paginação por mudança de configuração.
- Gutter personalizado via `#set page(gutter: ...)` continua scope-out.

---

## Critérios de verificação

- `#set page(columns: 2)\n#lorem(1200)` produz número de páginas próximo do vanilla (2 para A4) sem erros de sintaxe.
- Documento com `colbreak()` manual continua a funcionar como em P537.
- Notas de rodapé por coluna continuam posicionadas no fundo da coluna correcta.
- `cargo test --workspace` e `crystalline-lint .` limpos.
