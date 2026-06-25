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

Resolve o número do elemento associado ao label e renderiza texto.

### Algoritmo

1. Construir `Label(elem.name.to_string())`.
2. Caminho P462 (`Content::Label` numérico):
   - `counter_key_for_label(&label)` → `Some(key)`.
   - `query_by_label(&label)` → `Some(loc)`.
   - Formatar o número:
     - `key == "heading"` → `formatted_counter_at("heading", loc)` (ex: `"1.1"`).
     - `key == "equation"` → `flat_counter_at("equation", loc)` formatado como
       `(n)`.
     - `figure:*` / `table` → `flat_counter_at(key, loc)` → `n`.
   - Combinar com `supplement` ( explícito ou default por tipo).
   - Renderizar `Content::text(supplement + formatted)`.
3. Fallback legacy (`Content::Labelled`):
   - `figure_number_for_label(&label)` → `"Fig. {n}"`.
   - `resolved_label_for(&label)` → texto resolvido (ex: `"Secção 1"`).
4. Label não encontrada → renderizar `"?"`.

### Supplements default por counter key

| key | supplement |
|---|---|
| `figure:*` | `"Fig. "` |
| `table` | `"Table "` |
| `heading` / `equation` | nenhum |

### Scope-out

- `@x` syntax sugar (lexer/parser).
- PDF `/GoTo` links internos (P463).
- `form` e i18n de supplements.
- Warning em label não encontrado.
