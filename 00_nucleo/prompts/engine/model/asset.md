# Prompt L0 — `model/asset` — resource placeholder `asset(...)`
Hash do Código: 33ccde16

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/structural.rs` + `01_core/src/entities/content.rs`
**Origem**: Passo 397 — materialização de `asset(...)` (M); extensão cristalina (não existe no vanilla como elemento).
**ADRs**: ADR-0033 (divergência intencional), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out).

---

## 1. Contexto

`asset(path)` é uma extensão cristalina para resources externos (imagens, fontes, dados). No vanilla os assets são geridos implicitamente pelo compilador; no cristalino explicita-se como elemento user-facing. Este passo apenas modela o elemento — o registry real fica scope-out.

## 2. Content variant

```rust
Asset {
    path: EcoString,
    kind: Option<EcoString>,  // "image" | "font" | "data" | None (inferido)
}
```

- `path` obrigatório — caminho do resource.
- `kind` opcional — se omitido, infere da extensão.

Comportamento:
- `PartialEq` estrutural.
- `plain_text` vazio.
- `map_content` no-op (sem filhos `Content`).
- `is_empty` devolve `true` (placeholder sem output visual).

## 3. Stdlib `native_asset`

`asset(path, kind:?)` → `Value::Content(Content::Asset)`.

- `path`: `Value::Str` obrigatório (posicional ou nomeado `path`).
- `kind`: `Value::Str` opcional; se omitido, infere:
  - `png|jpg|jpeg|gif|svg` → `"image"`
  - `ttf|otf|woff|woff2` → `"font"`
  - `json|yaml|csv|xml` → `"data"`
  - outro → `None`.

## 4. Layout

`Content::Asset` é placeholder — `layout_content` e `measure_content_constrained` são no-op.

## 5. Scope-out

- Resource registry real (carregamento / embed / deduplicação) — scope-out ADR-0054 graded.
- Sem paridade vanilla porque `asset` não existe como elemento vanilla.

## 6. Testes

- `asset("logo.png")` → `Content::Asset { path: "logo.png", kind: Some("image") }`.
- `asset("foo.bin", kind: "data")` → kind override.
- `asset("foo.bin")` → kind `None`.
- `asset(123)` → erro de tipo.
- E2E: `#asset("x")` não emite frames.
