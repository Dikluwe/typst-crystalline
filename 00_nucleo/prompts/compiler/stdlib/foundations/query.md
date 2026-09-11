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

## P1339 — consumir seletor de elemento sem apagar filtros

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`:
`compiler/stdlib/foundations/query.rs:52-64,76-80` separa parsing,
consulta e projeção. O store completo já está disponível. A sonda
`diagnosticos/p1339-where-integration-probe-runs.json` registra query de
strong/emph aceitas e text.where rejeitado com `text is not locatable`;
fonte ratificada `introspection/query.rs:160-175` recebe LocatableSelector.

### Decisão

Conservar integralmente Selector::Element recebido, inclusive função e
grupo vazio, e delegar a seleção ao impl de `compiler/introspect.md`.
Validar locatability da nova folha antes de consultar: usar reconhecimento
nativo estático tipado do owner `compiler/eval/bindings/value_methods.md`,
nunca chamar o constructor, identificar só por nome ou usar resultado vazio
como prova de tipo inválido. Text rejeitado não pode virar array vazio;
Strong/Emph não podem ser rejeitados por ausência de ElementKind.
Em composições contendo Element, verificar suas folhas novas, conservando
as regras legadas das folhas antigas. Constructor selector e uso em show
não recebem esta restrição de locatability.

Preservar no resultado cada entrada completa com sua Location exata;
snapshot Some não sofre fallback por campo nem consulta à chain atual.
O caminho locate compartilha essa validação quando consome Element, sem
alterar sua política preexistente de primeiro resultado ou ausência.
Metadata/here/target e diagnóstico das formas antigas ficam inalterados.
Não executar callbacks nem adicionar outra passagem nesta nativa.

### P1339 — registrar a consulta, não apenas suas locations

Medição complementar no mesmo HEAD, consumer intacto:
`query.rs:53-65` projeta o store completo depois da seleção; `:77-81`
devolve a primeira Location/None; `:99-105` consulta current_location;
`:145-155` consulta contexto/target, sem ler o snapshot. Proveniência em
`diagnosticos/p1339-observation-integration-receipt.json`.

Após a validação de argumentos vigente, registrar query/locate com o Selector
efetivo e seus resultados completos no registro privado de EvalContext.
Query conserva ordem, cardinalidade, cada carrier e Location e a distinção
entre snapshot presente e fallback. Locations iguais com conteúdo/fields
alterados não provam que a entrada permaneceu igual. Locate registra a
primeira ocorrência ou ausência conforme sua política própria, não exige
que matches posteriores não observados permaneçam iguais.

Revalidar pela mesma seleção/projeção contra candidate, sem executar corpos
contextuais nem chamar constructors. Valores Func/Content dentro do resultado
não podem ser certificados por Debug, repr ou apenas igualdade de Location;
o comparador de observações de eval deve cobrir a entrada ou declarar sua
incompletude antes do selo. Um resultado vazio também é uma observação.

Here registra a Location contextual ou sua ausência. Como current_location
não é parte do snapshot candidato, sua estabilidade é precondição da
tentativa retida; mudança da identidade do bloco exige nova avaliação pela
pipeline, não replay com a Location antiga como prova. Target/features são
inputs da tentativa, não resultados de query: conservar os mesmos valores
durante sua revalidação. Não mudar sua propagação legada nesta extensão.

Registrar não seleciona o bloco: somente demanda de counter Element o faz.
Assim query/locate/here anteriores à demanda ficam disponíveis sem ampliar
o conjunto de blocos reexecutados. Metadata e criação de Selector continuam
sem leitura. Gates incluem consulta vazia que ganha ocorrência e mesma
Location com field alterado; nenhum deles passa só por contagem de páginas.

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
