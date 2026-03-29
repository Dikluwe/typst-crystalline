# Prompt L0 — layout

## Módulo
`01_core/src/rules/layout.rs`

## Propósito
Converte `Content` em `PagedDocument` com word-wrap e paginação básica.
Usa métricas monoespaçadas fixas (`FixedMetrics`) injectáveis via trait
`FontMetrics` — substituíveis por `FontBookMetrics` no Passo 20.

## Restrição arquitectural
Não depende de L3. Métricas de fonte reais (FontBook) são injectadas
por trait, não importadas directamente. `layout()` compila e testa em L1.

## Tipos e interface

### `FontMetrics` trait
```rust
pub trait FontMetrics {
    fn char_width(&self, c: char) -> Pt;
    fn line_height(&self) -> Pt;
    fn font_size(&self) -> Pt;
}
```

### `FixedMetrics`
Monoespaçado: `char_width = size * 0.6`, `line_height = size * 1.2`.

### `Layouter<M: FontMetrics>`
Máquina de estado: cursor_x, cursor_y, current_line buffer, paginação.

### `layout(content: &Content) -> PagedDocument`
API pública — usa `FixedMetrics::new(12.0)`.

## Comportamento
- `Content::Empty` → zero páginas
- Word-wrap: quebra quando palavra ultrapassa `page_width - MARGIN`
- Paginação: nova página quando `cursor_y > page_height - MARGIN`
- `flush_line()` move `current_line` para o frame actual
- `finish()` faz flush final e descarta página vazia

## Critérios de verificação
- `layout(&Content::Empty).pages.is_empty()`
- `layout(&Content::text("Hello world")).plain_text()` contém "Hello" e "world"
- 100 palavras → todos os items dentro dos limites da página (x<595, y<842)
- 50 palavras → múltiplas linhas (y_values.len() > 1)
- Pipeline parse→eval→layout sem crash
