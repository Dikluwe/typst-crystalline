# Prompt L0 — `stdlib/tiling` — constructor `tiling(...)`
Hash do Código: 7b0edf0f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/visualize.rs`
**Origem**: Passo 396 — materialização do constructor `tiling()` (M); depende P395.
**ADRs**: ADR-0017 (portão aberto por P395), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out).

---

## 1. Contexto

P395 modelou `Value::Tiling`, `TilingBody`, `TilingRelative` e `Paint::Tiling`. Este passo
materializa o constructor user-facing `tiling(...)` e activa o consumer layout (fallback
Color) para provar que o tipo funciona no pipeline.

## 2. Função nativa

`native_tiling(ctx, args, world, current_file)`:

- `body` (positional obrigatório):
  - `Value::Color(c)` → `TilingBody::Color(c)`.
  - `Value::Content(Content::Image(img))` → `TilingBody::Image(*img)`.
  - `Value::Str(path)` → resolve via `world.read_bytes(current_file, path)` e constrói
    `ImageElem` (graded — se não houver world, retorna erro claro).
  - `Value::Tiling(t)` → identidade (devolve o mesmo `Value::Tiling`).
  - `Value::Gradient(_)` → erro `"gradient em tiling não suportado — scope-out ADR-0054"`.
  - outro → erro de tipo.
- `size` (named, opcional):
  - `Value::Length(l)` → `Size::uniform(Pt(l.abs.to_pt()))`.
  - `Value::Array([w, h])` com dois `Length` → `Size { width, height }`.
  - `Value::None` | `Value::Auto` → `None`.
  - outro → erro.
- `spacing` (named, opcional): mesmo formato que `size`.
- `relative` (named, opcional):
  - `"self"` → `TilingRelative::Itself`.
  - `"parent"` → `TilingRelative::Parent`.
  - outro → erro.

Devolve `Value::Tiling(Arc::new(tiling))`.

## 3. Helpers

- `extract_size(value, fn_name, field) -> SourceResult<Option<Size>>`.
- `parse_relative(value) -> SourceResult<TilingRelative>`.

## 4. Registo

Registar em `make_stdlib`:

```rust
scope.define("tiling", Value::Func(Func::native("tiling", native_tiling)));
```

## 5. Consumer layout

`Paint::Tiling` já existe em `entities/paint.rs` com `to_color()` fallback. Verificar que
consumers de `Paint` em layout/export não dão panic — usam `to_color()` ou match
exhaustivo. Não adicionar pattern fill real (scope-out ADR-0054).

## 6. Scope-out

- Pattern fill PDF real — ADR-0054 graded (XL futuro).
- `Gradient` como body de `tiling` — rejeitado com erro.
- Semântica real de `relative: "parent"` — armazenado, mas consumer igual a `"self"`.

## 7. Testes

- `tiling(red)` → `Value::Tiling` com `TilingBody::Color`.
- `tiling(red, size: 50pt)` → `size` uniforme.
- `tiling(red, size: (50pt, 30pt))` → `Size` diferenciado.
- `tiling(red, relative: "parent")` → `TilingRelative::Parent`.
- `tiling(tiling(red))` → identidade.
- `tiling(123)` → erro de tipo.
- `tiling(gradient.linear(...))` → erro ADR-0054.
- `#rect(fill: tiling(red))` → layout aceita e emite shape com fallback Color.
