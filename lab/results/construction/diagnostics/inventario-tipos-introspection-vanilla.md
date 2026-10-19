# Inventário — tipos do módulo introspection vanilla

Pesquisa solicitada em `pesquisa-tipos-introspection-instrucao-claude-code.md`. Inventário literal de tipos `pub` definidos em `lab/typst-original/crates/typst-library/src/introspection/` e cruzamento com os 13 tipos do desenho cristalino.

Universo: 13 ficheiros (`mod.rs` + 12 sub-módulos), 3 983 linhas.

Sem decisão arquitectural, sem ADR, sem alteração ao desenho cristalino.

---

## Secção 1 — Lista de tipos vanilla

44 tipos `pub` (`struct`/`enum`/`trait`) + 5 funções `pub` (`analyze`, `here`, `locate`, `query`, `define`). As funções são listadas no fim para referência.

| Tipo | Kind | Ficheiro | Linhas | Responsabilidade |
|------|------|----------|-------:|------------------|
| `Introspect` | trait | `convergence.rs` | 287 | Interface para introspecções cujo output deve estabilizar entre iterações; usada para diagnóstico de não-convergência. |
| `Introspection` | struct | `convergence.rs` | 287 | `Arc<dyn Bounds>` — type-erased para guardar uma `Introspect` recordada durante compilação. |
| `History<'a, T>` | struct | `convergence.rs` | 287 | `[(&'a dyn Introspector, T); INSTANCES]` — histórico de valores observados em cada iteração para diagnóstico. |
| `Counter` | struct | `counter.rs` | 991 | Contador identificado por uma `CounterKey`. Tipo `#[ty(scope)]` exposto na linguagem. |
| `CounterKey` | enum | `counter.rs` | 991 | `Page` \| `Selector(Selector)` \| `Str(Str)` — identifica um contador. |
| `CounterUpdate` | enum | `counter.rs` | 991 | `Set(CounterState)` \| `Step(NonZeroUsize)` \| `Func(Func)` — update sobre um contador. |
| `Count` | trait | `counter.rs` | 991 | Elementos que têm comportamento de contagem especial — `fn update() -> Option<CounterUpdate>`. |
| `CounterState` | struct | `counter.rs` | 991 | `SmallVec<[u64; 3]>` — estado por nível de um contador. (NB: nome colide com `CounterState` cristalino, mas é tipo diferente — só guarda os números, não labels/TOC/lang/bib.) |
| `CounterUpdateElem` | struct | `counter.rs` | 991 | `#[elem(Construct, Locatable, Count)]` — elemento que aplica um update. |
| `CounterDisplayElem` | struct | `counter.rs` | 991 | `#[elem(Construct, Unqueriable, Locatable)]` — elemento que mostra valor de contador. |
| `ManualPageCounter` | struct | `counter.rs` | 991 | `physical: NonZeroUsize, logical: u64` — contador especializado de páginas. |
| `Introspector` | trait | `introspector.rs` | 695 | `#[comemo::track]` — interface principal: `query`, `query_first`, `position`, `page`, `path`, `document`, etc. |
| `EmptyIntrospector` | struct | `introspector.rs` | 695 | Implementação que devolve resultados vazios — null-object. |
| `ElementIntrospector<P>` | struct | `introspector.rs` | 695 | Implementação genérica em `P` (PagedPosition vs HtmlPosition). Indexa `(Content, P)` + `MultiMap<u128, Location>` + labels. |
| `ElementIntrospectorBuilder<P>` | struct | `introspector.rs` | 695 | Builder com pilha + sink + `seen` + `insertions` + `keys` + `locations` + `labels`. |
| `Locatable` | trait | `location.rs` | 385 | Marker — torna elemento disponível no Introspector. |
| `Unqueriable` | trait | `location.rs` | 385 | `: Locatable` — marker para elementos que não são queriable pelo utilizador. |
| `Tagged` | trait | `location.rs` | 385 | Marker para elementos taggados em PDF. |
| `Location` | struct | `location.rs` | 385 | `u128` hash — identifica unicamente um elemento no documento. Tipo `#[ty(scope)]` exposto na linguagem. |
| `LocationKey` | struct | `location.rs` | 385 | `u128` ordenável — wrapper para uso em sets. |
| `PositionIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — operação `Introspect` que devolve `PagedPosition`. |
| `PageIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — operação que devolve `NonZeroUsize` (número da página). |
| `PageNumberingIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — devolve `Option<Numbering>` da página. |
| `PageSupplementIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — devolve `Content` (supplement da página). |
| `PathIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — devolve `Option<VirtualPath>` do ficheiro. |
| `DocumentIntrospection` | struct | `location.rs` | 385 | `(Location, Span)` — devolve `Option<Location>` do documento que contém o elemento. |
| `Locator<'a>` | struct | `locator.rs` | 395 | Gera hashes únicos durante layout, com hash local + ponteiro para outer cached locator. |
| `SplitLocator<'a>` | struct | `locator.rs` | 395 | Versão dividida do `Locator` para gerar sublocators únicos. |
| `LocatorLink<'a>` | struct | `locator.rs` | 395 | Link para acesso on-demand através da fronteira de memoização (cache hit). |
| `MetadataElem` | struct | `metadata.rs` | 30 | `#[elem(Locatable)]` com `value: Value` — embed arbitrário no documento. |
| `DocumentPosition` | enum | `position.rs` | 167 | `Paged(PagedPosition)` \| `Html(HtmlPosition)` — generic sobre target. |
| `PagedPosition` | struct | `position.rs` | 167 | `page: NonZeroUsize, point: Point` — posição em documento paginado. |
| `HtmlPosition` | struct | `position.rs` | 167 | `element: EcoVec<usize>` + posição interna — posição em árvore HTML. |
| `InnerHtmlPosition` | enum | `position.rs` | 167 | `Frame(Point)` \| `Character(usize)` — posição precisa dentro de nó HTML. |
| `QueryIntrospection` | struct | `query.rs` | 285 | `(Selector, Span)` — operação `Introspect` que devolve `EcoVec<Content>`. |
| `QueryFirstIntrospection` | struct | `query.rs` | 285 | `(Selector, Span)` — devolve primeiro match. |
| `QueryUniqueIntrospection` | struct | `query.rs` | 285 | `(Selector, Span)` — devolve único match. |
| `QueryLabelIntrospection` | struct | `query.rs` | 285 | `(Label, Span)` — devolve match por label. |
| `State` | struct | `state.rs` | 522 | `key: Str, init: Value` — estado nomeado mutável durante a documentação. Tipo `#[ty(scope)]`. |
| `StateUpdate` | enum | `state.rs` | 522 | `Set(Value)` \| `Func(Func)` — operação sobre estado. |
| `StateUpdateElem` | struct | `state.rs` | 522 | `#[elem(Construct, Locatable)]` — elemento que executa update. |
| `Tag` | enum | `tag.rs` | 91 | `Start(Content, TagFlags)` \| `End(Location, u128)` — marcadores de início/fim. |
| `TagFlags` | struct | `tag.rs` | 91 | `introspectable: bool, tagged: bool` — flags semânticas do tag. |
| `TagElem` | struct | `tag.rs` | 91 | `#[elem(Construct, Unlabellable)]` com `tag: Tag` — wrapper element para layouters. |

Funções `pub` (não-tipos, listadas para completude):

| Função | Ficheiro | Responsabilidade |
|--------|----------|------------------|
| `analyze(...)` | `convergence.rs` | Executa análise de convergência sobre lista de `Introspection`s. |
| `here(...)` | `here.rs` | `#[func(contextual)]` — devolve `Location` actual no contexto. |
| `locate(...)` | `locate.rs` | `#[func(contextual)]` — localiza selector. |
| `query(...)` | `query.rs` | `#[func(contextual)]` — query sobre o documento. |
| `define(...)` | `mod.rs` | Hook que regista os tipos no scope global do utilizador. |

---

## Secção 2 — Cruzamento com desenho cristalino

| Tipo cristalino | Equivalente vanilla | Notas (ficheiro, dispersão) |
|------|------|-----|
| `Location` | `Location` (`location.rs`) | Equivalente directo. Vanilla: `struct Location(u128)`. |
| `Locator` | `Locator<'a>` (`locator.rs`) | Equivalente directo para o tipo principal. Vanilla disperse helpers em `SplitLocator` e `LocatorLink` no mesmo ficheiro. |
| `Tag` | `Tag` (`tag.rs`) | Equivalente directo. Vanilla complementa com `TagFlags` (struct de flags) e `TagElem` (wrapper element vtable). |
| `Introspector` | `Introspector` (trait, `introspector.rs`) + `ElementIntrospector<P>` (struct impl) | Vanilla é trait + impl genérica em `P` (target). Cristalino design não distingue por target → equivalência conceptual, forma diferente. Disperso por trait + builder + Empty + ElementIntrospector. |
| `Counter` | `Counter` (`counter.rs`) | Equivalente directo para o tipo exposto ao utilizador. Vanilla complementa com `CounterKey`, `CounterUpdate`, `CounterState` (apenas números), `Count` trait, `CounterUpdateElem`, `CounterDisplayElem`, `ManualPageCounter` no mesmo ficheiro (991 linhas). |
| `CounterRegistry` | **não existe como tipo nomeado em vanilla** | Responsabilidade dispersa: o registo de counters está implícito em `ElementIntrospector` (índice via `MultiMap<u128, Location>` + queries por `Selector`). Não há tipo `CounterRegistry`. |
| `State` | `State` (`state.rs`) | Equivalente directo para o tipo principal. Vanilla complementa com `StateUpdate` enum + `StateUpdateElem` wrapper. |
| `StateRegistry` | **não existe como tipo nomeado em vanilla** | Responsabilidade dispersa: o registo de states é gerido via `ElementIntrospector` (encontra `StateUpdateElem`s indexados por Location). Não há tipo `StateRegistry`. |
| `MetadataStore` | parcial — `MetadataElem` (`metadata.rs`) é o **carrier**; o "store" não existe como tipo nomeado | Vanilla: `MetadataElem` é só o elemento `#[elem(Locatable)]` que envolve um `Value`. O armazenamento e indexação é feito pelo `ElementIntrospector` via query por `MetadataElem` selector. Disperso entre o elem-vtable e o introspector. |
| `LabelRegistry` | **não existe como tipo nomeado em vanilla** | Responsabilidade dispersa: labels são indexadas dentro de `ElementIntrospector` (`labels: MultiMap<Label, usize>` + `ElementIntrospectorBuilder.labels`). Acesso via `Introspector::query` com `Selector::Label`. Não há tipo `LabelRegistry`. |
| `QueryEngine` | **não existe como tipo nomeado em vanilla** | Disperso: a função `query()` (`query.rs`) é a entry-point user-facing; o motor está nos `Introspector::query/query_first/...` (métodos do trait); a "história" para convergência está em `QueryIntrospection`/`QueryFirstIntrospection`/`QueryUniqueIntrospection`/`QueryLabelIntrospection` (4 structs `Introspect`). |
| `DocumentInfo` | parcial — `DocumentIntrospection` (`location.rs`) cobre **localização do documento** | Não há tipo agregador "DocumentInfo". `DocumentIntrospection(Location, Span)` devolve `Option<Location>` para "qual documento contém este elemento". Outros aspectos (numeração de páginas, supplement, path) estão em `PageNumberingIntrospection`/`PageSupplementIntrospection`/`PathIntrospection`. |
| `WalkContext` | **não existe** | Vanilla não faz walk explícito sobre `Content`. Usa fixpoint via `comemo` + `convergence::analyze` para iterar até as introspecções estabilizarem. Não há tipo `WalkContext`. |

Resumo do cruzamento:

- **Equivalência directa nomeada** (mesmo nome ou nome próximo, ficheiro próprio): 6 (Location, Locator, Tag, Introspector, Counter, State).
- **Parcial nomeado** (vanilla tem tipo carrier mas não agregador): 2 (MetadataStore→MetadataElem, DocumentInfo→DocumentIntrospection).
- **Sem equivalente nomeado em vanilla** (responsabilidade dispersa pelo Introspector): 4 (CounterRegistry, StateRegistry, LabelRegistry, QueryEngine).
- **Não existe em vanilla** (modelo arquitectural diferente): 1 (WalkContext — vanilla usa fixpoint comemo).

---

## Secção 3 — Tipos vanilla sem equivalente no desenho cristalino

37 dos 44 tipos vanilla não têm correspondência directa nos 13 do desenho cristalino. Classificação literal:

### vtable / proc-macro driven (existem por causa da arquitectura `#[elem]` ou `#[ty]` vanilla)

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `CounterUpdateElem` | `counter.rs` | `#[elem]` que executa update no AST. | vtable — vanilla precisa para `Element::ELEM` dispatch. Cristalino representa updates como `Content::CounterUpdate` variant. |
| `CounterDisplayElem` | `counter.rs` | `#[elem]` que exibe valor. | vtable — idem. Cristalino: `Content::CounterDisplay`. |
| `StateUpdateElem` | `state.rs` | `#[elem]` que executa update de estado. | vtable. Cristalino: provavelmente `Content::StateUpdate` futuro. |
| `MetadataElem` | `metadata.rs` | `#[elem(Locatable)]` carrier de Value arbitrário. | vtable carrier; conceito de "metadata atribuída a um nó" é necessário, forma vanilla não aplicável. |
| `TagElem` | `tag.rs` | `#[elem(Construct, Unlabellable)]` que contém `Tag`. | vtable — wrapper para que layouters possam observar Tags via tree walk. Cristalino: provavelmente `Content::Tag` directo. |
| `Locatable` | `location.rs` | Marker trait — torna elemento queriable. | vtable marker — Cristalino não usa marker traits per element; Content é enum fechado. |
| `Unqueriable` | `location.rs` | Marker trait — não-queriable pelo utilizador. | vtable marker — idem. |
| `Tagged` | `location.rs` | Marker trait — tagged em PDF. | vtable marker — propriedade pode ser bool em Content variant (cristalino). |
| `Count` | `counter.rs` | Trait — `fn update() -> Option<CounterUpdate>` em elementos. | vtable — derive macros. Cristalino: pattern match sobre `Content`. |
| `Introspect` | `convergence.rs` | Trait genérico para introspecções type-erased. | vtable — type erasure via `Arc<dyn Bounds>`. Cristalino: provavelmente match sobre enum de operações. |
| `Introspection` | `convergence.rs` | `Arc<dyn Bounds>` — type-erased introspect. | vtable — wrapper para storage heterogéneo. Cristalino não precisa se as operações forem enum fechado. |

### type-erased convergence helpers (concept needed; forma vanilla específica)

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `History<'a, T>` | `convergence.rs` | `[(&'a dyn Introspector, T); INSTANCES]` — histórico de outputs por iteração. | Conceito necessário se cristalino implementar fixpoint multi-iteração. Forma vanilla é genérica + dyn-Introspector — cristalino pode usar `Vec<T>` por iteração se estrutura for fechada. |
| `EmptyIntrospector` | `introspector.rs` | Null-object — devolve vazio para tudo. | Conceito necessário se Introspector for trait; redundante se Introspector for struct concreta opcional (`Option<&Introspector>`). |
| `ElementIntrospector<P>` | `introspector.rs` | Generic implementation com posição P. | Genérico em `P` para Paged vs Html targets. Cristalino paged-only → `P` desnecessário. |
| `ElementIntrospectorBuilder<P>` | `introspector.rs` | Builder com pilha + insertions + seen + maps. | Builder é detalhe de impl. Cristalino pode construir directamente sem builder. |

### Position types — concept needed; forma vanilla cobre HTML

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `DocumentPosition` | `position.rs` | Enum `Paged \| Html`. | vtable-target generic; cristalino paged-only → não precisa do enum, só de `PagedPosition` directo. |
| `PagedPosition` | `position.rs` | `(page, point)`. | Conceito necessário. Forma simples; cristalino pode usar tipo equivalente ou `(usize, Point)` directo. |
| `HtmlPosition` | `position.rs` | Posição em árvore HTML. | Específico ao target HTML. Não aplicável a cristalino paged-only. |
| `InnerHtmlPosition` | `position.rs` | Sub-posição num nó HTML. | Idem — específico HTML. |

### Counter sub-types — concept needed

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `CounterKey` | `counter.rs` | Enum `Page \| Selector \| Str`. | Conceito necessário (forma de identificar contador). Forma vanilla aplicável. |
| `CounterUpdate` | `counter.rs` | Enum `Set \| Step \| Func`. | Conceito necessário (operação sobre contador). `Func` variant é vanilla-specific (depende de Func). |
| `CounterState` (vanilla) | `counter.rs` | `SmallVec<[u64; 3]>` — estado por nível. | Conceito necessário (estado interno do contador). NB: nome colide com `CounterState` cristalino, que é tipo diferente (agregador L1 com labels/TOC/lang/bib_entries). |
| `ManualPageCounter` | `counter.rs` | `(physical, logical)` para página. | Conceito necessário se cristalino quiser distinguir página física vs lógica. Forma simples. |

### Location sub-types

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `LocationKey` | `location.rs` | Wrapper ordenável (`Ord`) sobre Location. | Conceito necessário se cristalino usar BTreeMap/BTreeSet por Location. Vanilla evita derivar Ord no Location próprio. |
| `PositionIntrospection` | `location.rs` | `(Location, Span)` — operação que devolve PagedPosition. | vtable — vanilla cria struct `Introspect`-impl por operação. Cristalino: provavelmente método em `Introspector`. |
| `PageIntrospection` | `location.rs` | Idem — devolve número de página. | vtable. |
| `PageNumberingIntrospection` | `location.rs` | Idem — devolve numbering pattern. | vtable. |
| `PageSupplementIntrospection` | `location.rs` | Idem — devolve supplement. | vtable. |
| `PathIntrospection` | `location.rs` | Idem — devolve VirtualPath. | vtable. |
| `DocumentIntrospection` | `location.rs` | Idem — devolve Location do documento contentor. | vtable. |

### Locator sub-types

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `SplitLocator<'a>` | `locator.rs` | Versão dividida para gerar sublocators. | Detalhe de impl — cristalino sem comemo não precisa de splitting do mesmo modo. |
| `LocatorLink<'a>` | `locator.rs` | Link com `OnceLock<Resolved>` para cache hit. | Optimização de memoização comemo. Não aplicável se cristalino não usar comemo aqui. |

### Query sub-types

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `QueryIntrospection` | `query.rs` | `(Selector, Span)` — operação Introspect. | vtable — uma struct por operação. Cristalino: chamada directa a método do QueryEngine. |
| `QueryFirstIntrospection` | `query.rs` | Idem — first match. | vtable. |
| `QueryUniqueIntrospection` | `query.rs` | Idem — unique match. | vtable. |
| `QueryLabelIntrospection` | `query.rs` | `(Label, Span)` — operação por label. | vtable. |

### State sub-types

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `StateUpdate` | `state.rs` | Enum `Set(Value) \| Func(Func)`. | Conceito necessário (operação sobre estado). `Func` é vanilla-specific. |

### Tag sub-types

| Tipo | Ficheiro | Responsabilidade | Avaliação |
|------|----------|------------------|-----------|
| `TagFlags` | `tag.rs` | `introspectable: bool, tagged: bool`. | Conceito necessário; forma vanilla aplicável (par de bools). |

---

## Notas de método

- "Equivalente vanilla" foi identificado pela responsabilidade declarada no doc-comment, não por nome literal. Caso o vanilla agregue várias responsabilidades num só tipo (e.g. `ElementIntrospector` faz registo de labels + counters + queries), o cruzamento na §2 marca a dispersão.
- Tipos privados (e.g. `RawContent`, `Bounds` em `convergence.rs`, `LinkKind` em `locator.rs`) não foram listados — só `pub` no module root ou re-exportado via `pub use self::xxx::*`.
- `pub` em sub-módulos `pub` (e.g. `CounterKey`, `TagFlags`) foi incluído porque é re-exportado pelo `mod.rs` via `pub use self::counter::*` etc.
- Cinco funções `pub` foram listadas no fim da §1 para completude, mas não são tipos.
- A coluna "Linhas" da §1 reporta o tamanho do ficheiro inteiro, não do tipo individual; muitos ficheiros agregam vários tipos relacionados.
- Tipos `#[elem]` foram marcados como "vtable-driven" porque a sua existência depende directamente da arquitectura proc-macro vanilla. O conceito que carregam (e.g. "elemento que aplica counter update") pode ser necessário no cristalino, mas a forma struct-com-vtable não é.
- Tipos `Introspect`-impl (e.g. `QueryIntrospection`, `PageIntrospection`) existem porque vanilla precisa de cada operação ser type-erased via `Arc<dyn Bounds>` para `convergence::analyze`. Se cristalino não usar fixpoint por convergência type-erased, estes tipos não têm correspondente directo.

## Lacunas

- A pesquisa não inspeccionou `ElementIntrospector` em detalhe além dos campos públicos visíveis. Os campos privados (e.g. `MultiMap<u128, Location>`) são listados como descrição mas não exaustivamente analisados.
- O documento de desenho cristalino mencionado (`desenho-introspection-fixpoint.md`) não foi encontrado no repositório com esse nome literal. A lista de 13 tipos foi extraída directamente do enunciado da pesquisa (§2 do ficheiro de instruções).
- Não foi inspeccionada a integração com `comemo` (`Track`, `Tracked`, `TrackedMut`) ao nível dos métodos do `Introspector` trait — apenas que o trait usa `#[comemo::track]`.
