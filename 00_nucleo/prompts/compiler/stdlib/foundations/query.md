# Prompt L0 — `stdlib/foundations/query` — query, localização e metadados
Hash do Código: e97b8512

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`compiler/stdlib/foundations/query.rs:46–65` já tem locations ordenadas e
TagIntrospector concreto, mas clona apenas element_at. R5 mede escape entre
blocos de estilo e igualdade sem perda da identidade de ocorrência.

### Decisão proprietária

Ao obter uma Location, consultar diretamente `ctx.introspector.elements` e
clonar a entrada completa para `Value::LocatedContent(entry.clone(), loc)`.
Isso substitui somente a leitura/projeção P1151; conserva ordem, selector,
cardinalidade e fallback Value::Location quando não existe elemento sintético.
Não consultar a chain de quem chamou query, não copiar só entry.content().
O tipo de linguagem permanece content. Não corrigir Selector::Where ou
outras dívidas de seleção nesta migração. Aceitação cobre primeiro contexto,
clone escapado e duas ocorrências iguais com locations distintas.

---


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
