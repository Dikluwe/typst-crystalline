# Prompt L0 — `stdlib/foundations/query` — query, localização, metadados, selector
Hash do Código: 541d51d2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/query.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Funções

### `native_metadata` — `metadata(value)`

Embebe um valor opaco no content tree para consulta via introspector.

### `native_query` — `query(selector)`

Consulta o introspector e devolve array de `Content` (P844) ou `Location`.
Aceita string de kind, `<label>`, `Location`, `Selector`, `Label`, função de
elemento.

### `native_locate` — `locate(selector)`

Devolve a primeira `Location` do selector ou `none`.

### `native_here` — `here()`

Devolve `ctx.current_location` quando populado.

### `native_target` — `target()`

Devolve `"paged"` (cristalino só produz PDF). Requer contexto conhecido
(P821).

### `native_selector` — `selector(...)`

Constrói um `Value::Selector` a partir de string de kind, `<label>` ou função
de elemento.

## 2. Critérios de verificação

```
metadata("x")                    -> Content::Metadata
query("heading")                 -> array (vazio se iter 0)
query("<intro>")                 -> array com Location do label
locate("figure")                 -> Value::None (sem matches)
here() sem current_location      -> Err contextual
target() fora de context         -> Err "can only be used when context is known"
target() dentro de context       -> "paged"
selector("heading")              -> Selector::Kind(Heading)
selector("<intro>")              -> Selector::Label(intro)
```
