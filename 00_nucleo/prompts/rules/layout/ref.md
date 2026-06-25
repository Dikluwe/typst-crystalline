# Prompt L0 — `rules/layout/ref` — layout de `Content::Ref`
Hash do Código: —

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/references.rs`

---

## `layout_ref`

```rust
pub(super) fn layout_ref<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &RefElem,
)
```

Resolve o número do elemento associado ao label e envolve o texto num
`FrameItem::Link` clicável para o destino interno (P463).

### Algoritmo de resolução de texto

1. Construir `Label(elem.name.to_string())`.
2. Caminho P462 (`Content::Label` numérico):
   - `counter_key_for_label(&label)` → `Some(key)`.
   - `query_by_label(&label)` → `Some(loc)`.
   - Formatar o número:
     - `key == "heading"` → `formatted_counter_at("heading", loc)` (ex: `"1.1"`).
     - `key == "equation"` → `flat_counter_at("equation", loc)` formatado como
       `(n)`.
     - `figure:*` / `table` → `flat_counter_at(key, loc)` → `n`.
   - Combinar com `supplement` (explícito ou default por tipo).
3. Fallback legacy (`Content::Labelled`):
   - `figure_number_for_label(&label)` → `"Fig. {n}"`.
   - `resolved_label_for(&label)` → texto resolvido (ex: `"Secção 1"`).
4. Label não encontrada → `"?"`.

### Envolver em link (P463)

- Renderizar `Content::text(texto_resolvido)`.
- Coletar os `FrameItem`s produzidos (podem estar em `current_line` e/ou
  `current_items`, dependendo de flush).
- Calcular bbox com `link_bbox`.
- Empurrar `FrameItem::Link { target: LinkTarget::Destination(label), items, pos, size }`.

Todo `ref` é clicável; se o destino não existir, o PDF reader não navega.

### Supplements default por counter key

| key | supplement |
|---|---|
| `figure:*` | `"Fig. "` |
| `table` | `"Table "` |
| `heading` / `equation` | nenhum |

### Scope-out

- `@x` syntax sugar (lexer/parser).
- `form` e i18n de supplements.
- Warning em label não encontrado.
- Link styling (cor/sublinhado).
- Opção `link: false` para desactivar.
