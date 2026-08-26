# Prompt L0 — `stdlib/foundations/query` — query, localização e metadados
Hash do Código: fd4f6cc1

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

O construtor `native_selector`, o mapeamento de funções de elemento e o parser
partilhado pertencem ao L0 irmão `foundations/selector.md` desde P1140.1-A.
`native_query` e `native_locate` chamam estaticamente
`selector::parse_selector_arg`; o comportamento e as mensagens não mudam.

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

## P1151 — preservar Location no resultado de `query`

### Medição

No vanilla pinado, `query(heading)` sobre duas headings `Same` devolve dois
valores de tipo `content`: eles são iguais em linguagem e têm os mesmos
`repr`/`fields`, mas `location()` existe nos dois e as Locations são distintas.
No cristalino, `native_query` obtém primeiro `Vec<Location>` e consulta
`element_at(loc)`, porém descarta `loc` ao construir `Value::Content`.

### Decisão condicionada ao gate ADR-0127

Quando `element_at(loc)` existir, devolver o valor locatável especificado em
`entities/value.md` P1151, contendo o `Content` clonado e o `loc` exato.
Manter o fallback `Value::Location(loc)` apenas para introspectors sintéticos
sem elemento. Não procurar Location por `PartialEq`, hash morfológico ou
primeira ocorrência: conteúdos idênticos podem ter Locations diferentes.

Aceitação: `query` preserva ordem; cada resultado continua sendo `content`;
dois conteúdos idênticos continuam iguais; seus `location()` podem diferir;
conteúdo inline continua com `none`.
