# Pipeline — L3 orquestração
Hash do Código: 2d97726a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

## Módulo
`03_infra/src/pipeline.rs`

## Propósito

Orquestra o pipeline completo de compilação em L3. Esconde o
boilerplate `comemo` (Routines, Traced, Sink, Route) e expõe
APIs alto-nível à L4 (04_wiring) e aos testes.

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs`.

## P1247 — composição do contexto de destinos SVG

**Gate ADR-0127:** arquitetura aprovada pelo dono em 2026-08-28. L0 pré-código;
implementação condicionada a preseal segregado válido.

Depois de obter o `PagedDocument`, o caminho de compilação SVG seleciona a
página exportada e deriva um `SvgDestinationContext` somente das entradas de
`extracted_label_pages` cuja página corresponda ao índice selecionado, usando a
posição homóloga de `extracted_label_positions`. Ausência de posição não é
inventada e exclui o destino do contexto.

O pipeline passa esse contexto à variante explícita do exporter com ou sem
fontes. Não move os mapas para `Page`, não cria novo `FrameItem`, não repete
layout e não decide ortografia de IDs. O caminho atual exporta apenas a primeira
página; links para outras páginas permanecem `Unknown`, sem fragmento pendente.

Política futura de bundle, nomes de ficheiro ou rotas cross-page pertence ao
caller de composição e exige L0 próprio. A expansão pública de `link()` para
label/location/page-position é decisão separada.

## P1249 — composição da identidade de fonte de glifo proposta

**Gate ADR-0127:** arquitetura aprovada pelo dono em 2026-08-28. L0 pré-código;
implementação condicionada a preseal segregado válido.

No caminho SVG com fontes, a pipeline reutiliza a decisão já existente de
`FallbackFontMetrics::resolve_font_combo(base_char, style)` para cada
`FrameItem::Glyph`, associa um `GlyphFontRequest` normalizado ao `FontKey`
completo efetivamente resolvido e entrega essa associação no contexto imutável
do exporter. Não relê fonte, não repete layout e não altera
`FrameItem::Glyph`.

A chave distingue `base_char`, família solicitada, variante, variações e estado
matemático relevante; é independente de índice, endereço, ordem de travessia e
ocorrência. Entradas iguais têm resolução igual dentro da mesma compilação;
conflito entre resultados para a mesma chave invalida a entrada e conserva
`Unknown`. Ausência não escolhe índice zero por default. A coleta
recursa em background, body, foreground, Group, Link e Semantic como já ocorre
para fontes. O caminho sem fontes não constrói associação fictícia.

## Contrato

### P1140.4-A2 — introspecção runtime no caminho de produção

**Medição:** produção chamava só `introspect_with_introspector`, enquanto
`state.update(Func)`, `state.display`, `counter.display(callback)` e numbering
funcional de equação eram materializados somente no caminho runtime L1.

Depois de expandir `ContextBlock` e estabilizar Locations, L3 constrói
`Engine + EvalContext` e chama `introspect_with_runtime`. L3 não executa
callbacks nem duplica regras. Diagnósticos são propagados e layout recebe o
`TagIntrospector` runtime final. A primeira introspecção direta permanece só
para localizar/expandir context blocks. HTML fica fora sem medição própria.

### `eval_expression_with_sink` — P1137-B-001

**Medição anterior à decisão (2026-08-23):** L3 expõe apenas
`eval_to_module_with_sink(world, source)` em `03_infra/src/pipeline.rs:67-105`;
essa API avalia um documento markup e devolve `Module`. A CLI vanilla avalia
uma expressão code isolada (`typst-cli/src/eval.rs:102-129`). Adaptar um módulo
markup para extrair o valor seria morfologicamente incorreto.

**Decisão:** L3 expõe o adaptador alto nível:

```rust
pub fn eval_expression_with_sink(
    world: &dyn World,
    expression: &str,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

Ele delega ao entrypoint L1 `compiler::eval::eval_expression`, gere somente o
boilerplate necessário e devolve warnings sem formatar. Não serializa JSON/raw,
não escreve stdout e não cria um documento. Avaliação contextual e
introspector de documento ficam fora desta entrega.

### `eval_to_module_with_sink`

```rust
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>);
```

- Chama `eval()` com o boilerplate `comemo` completo.
- Warnings drenados do `Sink` e devolvidos separadamente.
- Não formata — retorna `Vec<SourceDiagnostic>` cru.
- Caller (CLI ou testes) decide formatação via
  `diagnostic_format::drain_diagnostics_to_stderr`.

### `compile_to_pdf_bytes`

```rust
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

- Pipeline `eval → introspect → layout → (dispatch export)`.
- **P617** — variantes `_with_document_id` aceitam um `Option<[u8; 16]>`
  externo e propagam-no ao `PdfBuilder`. Quando `Some`, esse valor
  fixa o `xmpMM:DocumentID`; `xmpMM:InstanceID` continua aleatório.
  Quando `None`, mantém o comportamento de P615 (aleatório). As
  funções públicas sem sufixo mantêm `None` e preservam a API
  existente.
- Dispatch font-aware multi-font (Passos 140B + 141 + 146,
  ADR-0055 `IMPLEMENTADO`; decisão 5 anotada por 146):
  - **Colecciona** todas as `FontList` distintas no
    `PagedDocument` em ordem de primeira ocorrência (atravessa
    `FrameItem::Text` e `FrameItem::Group` recursivamente).
    Dedup estrutural via `Vec::contains` (O(N²); N tipicamente
    pequeno).
  - Para cada `FontList`, **itera todas as famílias** em ordem
    (Passo 141): consulta
    `world.book().select(name, &FontVariant::default())` →
    índice, depois `world.font(index)` → bytes. **Primeira
    família a completar ambos os passos vence**. Cenário
    patológico (índice stale) não curto-circuita.
  - Filtra entries que não resolvem (silent drop).
  - Dispatch:
    - `[]` (vec vazio) → `export_pdf(&doc)` (fallback
      Helvetica Type1).
    - `[(_, bytes)]` (uma única) →
      `export_pdf_with_font(&doc, &bytes)` (caminho preservado
      do 140B/141 — output `/CrystallineFont` único).
    - `many` (2+) → `export_pdf_multifont(&doc, many)` —
      resource dict `/F1..N` com `/CrystallineFont1..N`; cada
      `FrameItem::Text` selecciona `/F{i+1}` por match
      estrutural contra a sua `style.font` (default `/F1`
      quando `style.font` é `None` ou não casa).
- **Multi-font per document** (ADR-0055 decisão 5,
  materializada no Passo 146): N fonts distintas → N
  `/Subtype /Type0` no PDF. Single-font como caso particular
  (preservado por dispatch).
- **Instrumentação de benchmark** (Passo 518): a struct
  `Timings` expõe `shape_ms` e `subset_ms` para permitir a
  análise de gargalos do pipeline de produção real. O tempo
  de `render_ms` passa a ser o tempo de export PDF *após* o
  subsetting.
- **Array fallback chain** (ADR-0055 decisão 4, Passo 141):
  dentro de uma `FontList`, todas as famílias são tentadas
  até resolver.
- Selecção usa `FontVariant::default()` (regular, normal, normal
  stretch). `weight`/`style` no documento continuam a ser
  renderizados via faux-bold/faux-italic (Passo 139) — selecção
  variant-aware é candidato ADR-0055bis.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

### `expand_context_blocks` (P506, corrigido P711)

```rust
pub fn expand_context_blocks(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content>;

fn collect_context_blocks(
    content: &Content,
    chain: &StyleChain,
) -> HashMap<u64, (Arc<ContextBlockElem>, StyleChain)>;
```

- Passo intermédio do pipeline, entre `introspect` e `layout`
  (`compile_to_pdf_bytes_full_error`, §P506): resolve cada
  `Content::ContextBlock` chamando a sua closure via `apply_func`
  e substituindo o nó pela `Content` resultante
  (`value_to_content`).
- **P711 — `StyleChain` da posição, não `default_chain()`.**
  Paridade com o vanilla (`typst-library/foundations/context.rs`
  `CONTEXT_RULE`): o `context` block é um show rule que recebe o
  `styles: StyleChain` **da posição onde aparece no documento**
  (o `StyleChain` acumulado por `#set` léxicos/`Content::Styled`
  ancestrais), não uma cadeia de defaults isolada. `collect_context_blocks`
  acumula essa cadeia durante o walk: parte de
  `StyleChain::default_chain()` e aplica `.push_styles(styles)`
  a cada `Content::Styled(inner, styles)` atravessado, associando
  a cadeia resultante (não só o `id`/elem) a cada `ContextBlockElem`
  encontrado. `expand_context_blocks` usa essa cadeia (não
  `default_chain()`) como `engine.styles` ao invocar a closure.
  Antes da correção, `.to-absolute()` (e qualquer resolução
  dependente de estilo) dentro de `context {...}` ignorava
  silenciosamente `#set text(size: ...)` e outros `#set`
  ancestrais, usando sempre os defaults (`size: 11.0`).
- **P1037 — o walk deixa de ser parcial.** A redacção anterior dizia:
  *"`collect_context_blocks` continua um walk parcial (Sequence, Styled,
  Strong, Emph, Heading) — `ContextBlock` não é esperado aninhado dentro
  de Grid/Table/etc. neste subset (scope-out pré-existente, inalterado
  por P711)"*. A premissa **"não é esperado aninhado" é falsa**: `#box[…]`
  e itens de lista são uso corrente. E a consequência não era um
  scope-out benigno — era resultado **silenciosamente vazio**, porque
  `substitute_context_blocks` troca por `Content::Empty` todo o
  `ContextBlock` cujo `id` não esteja em `resolved`, e um bloco que o walk
  não visita nunca lá chega. Medição em §P1037 abaixo.
  Os dois walks passam a ser exaustivos, delegando a descida ao
  `map_content` de L1 (match exaustivo sobre todos os containers, a fonte
  única da forma da árvore) em vez de reenumerarem containers em L3 —
  que era a duplicação que produziu o defeito.
- Fora do escopo de P711, medido mas não corrigido aqui (passos
  próprios): `repr_value` formata `Value::Length`/`Ratio`/`Angle`/
  `Color`/`Stroke`/`Align` com `{:?}` do Rust em vez do repr Typst
  quando embutidos directamente em markup (bug pré-existente,
  independente de `context`); `measure()` devolve sempre `0pt`
  (dentro e fora de `context`) e não tem o gate "can only be used
  when context is known" do vanilla.

### P1037 — `#context` aninhado devolvia vazio em silêncio

**Data:** 2026-08-13 · **Proveniência:** `HEAD = 0c8b64a41` (P1033), árvore
com as edições de P1036 (`entities/counter_format.rs`,
`compiler/layout/heading.rs`, os seus L0s e um teste de caracterização) já
aplicadas; vanilla `/usr/local/bin/typst`
(md5 `36da18895eeb5e0136c068a7634e3f82`); cristalino `target/release/typst`
reconstruído dessa árvore. Medições 18:05–18:25 -03:00.

Documento com `#set heading(numbering: "1.")`, `= Alpha`, `= Beta` e a
mesma expressão `#context counter(heading).get()` em cinco posições:

| posição | vanilla | cristalino (antes) |
|---|---|---|
| topo da sequência | `(2,)` | `(2,)` ✅ |
| dentro de `#emph[…]` | `(2,)` | `(2,)` ✅ |
| dentro de `#par[…]` | `(2,)` | `(2,)` ✅ |
| dentro de `#box[…]` | `(2,)` | **vazio** ❌ |
| dentro de um item de lista `- …` | `(2,)` | **vazio** ❌ |

O padrão bate exactamente com a whitelist do walk: as três posições que
funcionavam são as que `collect_context_blocks` atravessava. Este defeito
manifesta-se **sem show rule nenhuma** — é independente do achado #1 de
P1031.

> **Inferência levantada e depois refutada por medição (ADR-0108).** A
> hipótese de trabalho era que este walk parcial fosse *também* a causa do
> `#context` vazio dentro de show rules (achado #1 de P1031). **É falsa**:
> com os dois walks já exaustivos e a suite verde, os três documentos do
> achado #1 continuam exactamente como antes — `#show heading: it => [Nº
> #context counter(heading).get().first() — #it.body]` continua a dar
> `Nº — Alpha Nº — Beta` contra `Nº 1 — Alpha Nº 2 — Beta` do vanilla. São
> dois defeitos distintos com causas distintas; só o primeiro fecha aqui.

**A causa do segundo, localizada** (não corrigida — ver o bloco escalado
abaixo): a introspecção corre sobre a árvore **pré-show-rules**, e a
substituição sobre a **pós-show-rules**.

- `01_core/src/entities/module.rs:96` — `introspection_content` é, por
  definição, *"conteúdo original (pré-show-rules) para introspecção"*;
  `01_core/src/compiler/eval/mod.rs:458-474` guarda aí `original_content`.
- `03_infra/src/pipeline.rs` — `intr_content = module.introspection_content()`,
  logo `intr.context_block_locations` só conhece blocos que já existiam
  antes das show rules.
- `expand_context_blocks` itera `for (id, loc) in &intr.context_block_locations`.
  Um `ContextBlock` **criado pela** show rule tem `id` novo
  (`ctx.next_context_id()`, `eval/mod.rs:1174`) que nunca está nesse mapa.
- Não entrando em `resolved`, `substitute_context_blocks` troca-o por
  `Content::Empty` — a mesma última linha de ambos os defeitos, o que
  explica o sintoma partilhado e escondeu a diferença de causa.

> **ACHADO ESCALADO — o achado #1 de P1031 fica por fazer, nas suas duas
> faces.** Medido no mesmo documento (2026-08-13, binários acima):
>
> | forma | vanilla | cristalino |
> |---|---|---|
> | `#show heading: it => [Nº #counter(heading).get().first() — #it.body]` | `Nº 1 — Alpha Nº 2 — Beta` | **erro**: `counter.get() can only be used inside context` |
> | a mesma com `#context` explícito | `Nº 1 — Alpha Nº 2 — Beta` | `Nº — Alpha Nº — Beta` (vazio) |
>
> A primeira face é o gate `ctx.in_context`; a segunda é a árvore
> pré-show-rules descrita acima. **As duas fecham na mesma mudança**: dar
> ao corpo de uma show rule uma `Location` e um introspector que o
> conheçam. A documentação oficial diz *"Show rules provide context"*
> (`docs/content/reference/language/context.typ:13`, citada em
> `compiler/eval/show_rule_termination.md`).
>
> **Não corrigido em P1037.** As show rules são aplicadas durante o `eval`,
> **antes** de existirem `Location` e valores de counter, e a introspecção
> corre deliberadamente sobre a árvore anterior a elas (P498). Corrigir
> exige introspectar o conteúdo produzido pelas show rules — mover
> trabalho entre `eval` e `introspect`: **mudança de fase do pipeline**,
> gate ADR-0127 ponto 3, passo próprio. Reabre também o argumento de
> terminação antecipada de `compiler/eval/show_rule_termination.md` §3,
> cujo gatilho de reabertura essa secção já declara activo.

## Helpers privados de dispatch (Passos 140B + 141 + 146)

```rust
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList>;
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
fn resolve_fonts(
    font_lists: &[FontList],
    font_book:  &FontBook,
    world:      &dyn World,
) -> Vec<(FontList, Vec<u8>)>;

// Helper preservado do 140B (test-only no pipeline.rs):
#[allow(dead_code)]
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
```

- `collect_fonts_from_doc` (Passo 146): itera `doc.pages →
  items` recursivamente (atravessa `FrameItem::Group`) e
  devolve **todas** as `FontList` distintas em ordem de
  primeira ocorrência. Dedup estrutural via `Vec::contains`.
- `resolve_font` (Passo 141): itera `font_list.as_slice()` em
  ordem. Para cada família, consulta
  `font_book.select(name, &FontVariant::default())`; se devolve
  `Some(index)`, chama `world.font(index)`; se devolve
  `Some(font)`, devolve os bytes. **Primeira família a completar
  ambos os passos vence**. Cenário patológico (índice stale)
  não curto-circuita.
- `resolve_fonts` (Passo 146): map-filter de `resolve_font`
  sobre `&[FontList]`. Devolve `(FontList, bytes)` por entrada
  resolvida (silent drop para entries que não resolvem;
  consistente com 140B).
- `first_font_from_doc` (Passo 140B, **preservado em
  `#[allow(dead_code)]`**): historicamente o entry-point do MVP
  single-font (devolvia primeira `FontList`). Substituído pelo
  dispatch multi-font no Passo 146; permanece como referência
  e para os testes unitários do 140B continuarem activos.
- Funções privadas ao módulo e cobertas por testes unitários
  em `#[cfg(test)] mod tests`.

Multi-font materializada no Passo 146 (ADR-0055 decisão 5
anotada). Variant-aware (ADR-0055bis, candidata) e subsetting
(ADR-0056, candidata) permanecem fora do escopo actual.

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).

## Conversão de `layout_warnings` em diagnósticos

Após `layout_with_introspector_and_metrics`, a pipeline converte cada
entrada de `doc.layout_warnings` (strings puras produzidas em L1 — ex.:
aviso de body de footnote que excede a página/coluna, ver
`compiler/footnote_overflow_columns.md`) em
`SourceDiagnostic::warning(Span::detached(), msg)` e adiciona-a ao `Vec`
de `warnings` que a pipeline já retorna. L1 permanece data-only (ADR-0043):
nenhum `SourceDiagnostic` é construído em L1.

## P906 — `collect_fonts_in_items` cego a `FrameItem::Glyph`

**Contexto**: mecanismo de esticamento horizontal (`compiler/layout.md`
§P906) — achado mais fundo, na SELECÇÃO de quais fontes embutir no PDF
(distinto do embedding/subsetting, `export/builder.md` §P906).

**Achado, confirmado por instrumentação directa** (não inferido):
`collect_fonts_in_items` (usada por `collect_fonts_from_doc`, que decide
Cidfont-vs-Multifont e QUAIS fontes resolver) tinha o braço
`FrameItem::Glyph { .. } => {}` — ignorava por completo qualquer glifo de
esticamento matemático. Como a decisão Cidfont/Multifont é feita
EXCLUSIVAMENTE a partir de `style.font` visto em `Text`/`TextShaped`, uma
equação cujo ÚNICO conteúdo a precisar da fonte companion MATH fosse um
glifo de esticamento (`underbracket(a+b+c)` sem mais texto itálico
matemático à volta — caso isolado por outro achado incidental, P906
também, em `apply_math_default` não recursar `MathUnderover`) escolhia só a
fonte de corpo como candidata Cidfont única. O `glyph_id` do esticamento,
correcto na fonte MATH (confirmado via `FallbackFontMetrics`, que já
resolve correctamente para efeitos de LAYOUT), caía fora do subset embutido
— glifo `.notdef` no PDF real, apesar do mecanismo de layout estar
inteiramente correcto e testado.

**Correcção**: `collect_fonts_from_doc`/`collect_fonts_in_items` ganham
`world: &dyn World`, constroem um `FallbackFontMetrics` local, e o braço
`FrameItem::Glyph { style, base_char, .. }` chama o novo método
`FallbackFontMetrics::resolve_font_combo(base_char, style)` (`infra/
font_metrics.md` §P906 — mesmo mecanismo `covering`/`resolve_primary_with_
math_fallback` já usado internamente para LAYOUT, agora reaproveitado para
devolver a IDENTIDADE da fonte, não dados de glifo) — o resultado entra na
mesma lista `seen`/dedup, como mais um span de "texto" para efeitos de
selecção.

## P836 — chave de fonte com variações explícitas

`collect_fonts_from_doc`/`resolve_fonts` passam a chavear por
`(FontList, FontVariant, FontVariations)` (o terceiro componente vem de
`TextStyle::variations`, `unwrap_or_default`): dois runs com o mesmo
`FontVariant` mas `variations:` distintas embutem fontes instanciadas
distintas — paridade vanilla (`FontInstance` chaveado por variações
completas).

O gate `needs_variable_font_instancer` e o desvio single-font→multi-font
passam a usar `axis_variations_for_text_style` (fusão derivados +
explícitos), de modo que um documento cuja única variação é explícita
(ex. `wght: 250` com weight regular) também dispara a instanciação.

---

## P844 (achado #54 de P831) — re-introspecção pós-expansão

- Nova função pública `expand_context_blocks_and_reintrospect` (expansão + `introspect_with_introspector` do conteúdo expandido). Sonda: o `ContextBlock` é locatable no walk de introspecção, mas a expansão substitui-o por conteúdo não-locatable; o Locator do walk de layout (invariante P185C) desfasava face ao introspector pré-expansão e `CounterRegistry::value_at` devolvia o snapshot anterior (headings renumeravam a partir do bloco: `1.|1.|2.` em vez de `1.|2.|3.`). A pipeline de produção usa a nova função e re-injecta os styles CSL no `BibStore` reconstruído; o introspector reconstruído também alimenta `intr_for_positions` (P535). Probes que confirmaram o mecanismo: `#metadata(1)` e `#counter(heading).step()` entre headings (locatable que permanece) não dessincronizam.


## P956 — `stream_mode` nas entry points PDF

ADR-0126 (emendada P956): todas as variantes `compile_to_pdf_bytes*`
(`compile_to_pdf_bytes`, `_with_timings`, `_with_timings_full_error`,
`_with_timings_full_error_and_document_id`, `_full_error`,
`_full_error_and_document_id`) ganham `stream_mode: StreamMode` como **último
parâmetro**, propagado ao dispatch de export (`export_pdf*` —
`infra/export/mod.md` §P956). Quebra de assinatura deliberada (mesma razão de
`mod.md` §P956: caller nenhum fica ambíguo sobre o modo).

- O dispatch font-aware (`export_pdf` / `export_pdf_with_font` /
  `export_pdf_multifont`) passa o modo recebido a qualquer dos três ramos.
- `compile_to_png_bytes*` / `compile_to_svg_string*` **inalterados** — a flag
  `--compact` não tem significado fora do PDF (ver `shell/cli.md` §P956).

## Pipeline HTML — P1137-X-002 / ADR-0128

**Medição:** todos os exports atuais atravessam
`compile_to_paged_document_full_error`; isso perde estrutura semântica.

```rust
pub fn compile_to_html_string(
    world: &dyn World,
    source: &Source,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Executa eval com target HTML, obtém `Content` e chama `export_html` diretamente;
não chama layout paginado. Warnings/errors seguem o contrato existente.

### P1165 — propagação de features (RASCUNHO; ADR-0127)

**Medição:** `compile_to_html_string` em `pipeline.rs:130-140` não recebe
features e sempre autoriza o caminho, enquanto o vanilla recusa o formato HTML
sem `Feature::Html`. Após aprovação, a configuração pura de features entra na
pipeline e é passada ao eval. A pipeline verifica/propaga o gate antes do
export; target HTML e feature continuam eixos independentes. O default dos
entrypoints existentes é coleção vazia; não há leitura de env em L1/L3.

## P1140.5-A — preservação de grupos semânticos

### Medição antes da decisão

Os walkers da pipeline (`pipeline.rs:1106,1161`) descem apenas por Group/Link;
não há ramo semântico. O artefato PDF cristalino medido é `Tagged: no`.

### Decisão

Todas as passagens de fonte/shaping/normalização atravessam
`FrameItem::Semantic.items` como container transparente e preservam
kind/placement/alt byte a byte. Nenhuma passagem converte `alt` em texto nem
remove o wrapper. HTML continua eixo separado até medir seu contrato próprio.

## P1140.6 — threading de `PdfTags`

### Medição antes da decisão

As entry points PDF propagam apenas `StreamMode`; a pipeline não representa o
default tagueado medido no vanilla nem um caminho explícito sem tags.

### Decisão

Todas as variantes públicas `compile_to_pdf_bytes*` recebem `pdf_tags:
PdfTags` como último parâmetro, depois de `stream_mode`, e o propagam sem
transformação aos três ramos `export_pdf*`. O chamador de produção L4 usa
`Enabled` na ausência de `--no-pdf-tags` e `Disabled` quando a flag está
presente. PNG, SVG e HTML permanecem inalterados. `PdfTags` não depende de
`StreamMode` e não ativa validação PDF/UA nem diagnóstico por falta de alt.

## P1159 — realização de page numbering

Após o primeiro layout, a pipeline realiza cada `Numbering` com Engine em duas
vistas: visível recebe `[current, total]` e referência recebe `[current]`.
Constrói um `PageStore` completo, reinjeta posições e páginas no introspector e
executa novo layout. O ciclo é limitado a cinco passagens e converge pelo
número de páginas, snapshots lógicos, objetos crus e conteúdo realizado.
Erros do callback são devolvidos como `SourceDiagnostic`; L3 não duplica o
dispatch Pattern/Func, que pertence a `stdlib/numbering`.

A vista unária é realizada somente para as páginas que são alvo de um
`ref(form: "page")` resolvido pelo primeiro layout. Isso preserva a diferença
observável de aridade: uma função binária é válida como numbering visível num
documento sem referência, mas falha com `missing argument: total` quando a
referência exige a chamada unária. Páginas não referenciadas não executam o
callback unário nem produzem warnings laterais desse consumer.

A substituição contextual preserva o próprio `ContextBlock` como marcador
zero-size antes do conteúdo realizado. O marcador mantém a Location estável e
permite selar a sua Position; em cada iteração a expansão parte do conteúdo
original, portanto o marcador não se acumula nem duplica conteúdo visível.

Seleção e recolha de fontes percorrem `background`, `items` e `foreground` em
ordem. Running matter realizado não pode ficar fora do conjunto de fontes
embutidas só por residir na camada marginal.

## P1286 — validação global de attachments sem nova fase

### Medição anterior à decisão

O erro vanilla de duplicado depende da coleção global final; uma nativa
isolada não consegue observá-lo. `compile_to_pdf_bytes*` já devolve
`Result<_, Vec<SourceDiagnostic>>`, enquanto entrypoints diretos do exporter
são infalíveis.

### Decisão condicionada ao gate

Depois do layout e antes do dispatch font-aware, a pipeline valida
`PagedDocument.attachments` em ordem. Nome virtual derivado repetido produz
`attempted to attach file {path} twice` e aborta antes de exportar. O builder
recebe somente a lista validada. Não alterar assinatura pública, `World`,
ordem eval→layout→export, PNG/SVG/HTML ou defaults. Entry points diretos do
exporter têm como precondição um `PagedDocument` validado; isso não substitui
o erro de linguagem no pipeline de produção.

## P1288 — features também no pipeline paginado (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`03_infra/src/pipeline.rs:77-82` aceita features somente no adaptador de eval;
`:111-150` possui helper target+features e default vazio, e `:156-197` aplica
o gate somente ao HTML. Os entrypoints paginados/PDF não recebem a coleção,
portanto `a11y-extras` não pode chegar ao eval de produção PDF.

### Decisão proposta

Os entrypoints paginados usados por L4 recebem ou delegam a variantes que
recebem `entities::compiler_features::Features` e o encaminham sem transformação
ao mesmo eval. As variantes de compatibilidade sem parâmetro continuam com
conjunto vazio; não existe ativação por PDF, HTML, `PdfTags`, `StreamMode` ou
exporter.

Depois do layout, a pipeline preserva integralmente uma única árvore semântica
de tabela até o builder, inclusive THead/TBody quando aplicáveis e identidade
lógica de headers repetidos em várias páginas. Transporta separadamente os
vínculos físicos por página necessários a StructParents, MCID e ParentTree.
Não reclassifica células, não deriva summary de caption, não transforma
metadata em texto e não muda a ordem eval→introspect→layout→export. Uma árvore
ausente/ambígua continua falha ou Unknown conforme o contrato congelado; nunca
ganha fallback de sucesso.

## P1291 — realização entre passagens de `cancel.angle` (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

A pipeline já é o ponto onde callbacks que exigem `Engine` são executadas:
page numbering é realizada entre passagens de layout em
`03_infra/src/pipeline.rs:671-735`. Em contraste, o `MathLayouter` L1 não
possui `World`, `Scopes` ou `Engine`. O vanilla chama `CancelAngle::Func` com o
ângulo default somente depois de medir o body e faz cast do retorno para Angle
(`lab/typst-original/crates/typst-layout/src/math/cancel.rs:92-103`).

### Decisão proposta

No caminho paginado, a pipeline chama a entry point de passagem com store
vazia e recebe `Pending(transcript)`; o documento provisório já foi descartado
em L1 e não fica acessível. Fora do layouter, onde `Engine` existe, realiza cada
request em ordem por `apply_func(func, [Value::Angle(default)], ...)`, usando o
`StyleChain` e span capturados. Somente retorno convertível a `Angle` gera uma
`MathCancelResolution`; erro da closure ou cast inválido aborta sem exportar o
documento provisório.

As resoluções completas formam `SealedMathCallbacks`. A pipeline repete layout
com essa store. `Complete(document)` encerra a realização; novo
`Pending(transcript)` invalida a store, realiza o transcript novo e repete sob
teto finito. Teto excedido ou store parcialmente consumida é diagnóstico de
não convergência, nunca sucesso, `Unknown` rebaixado ou `Auto` final.

A pipeline encapsula isso num ciclo `stabilize_math_layout`: antes do layout
inicial e antes de **cada** relayout externo do ciclo P1159, repete
`Pending → realizar → nova passagem` até obter `Complete`. A rotina L1 pode
internamente tentar mais de um `Layouter::new` por TOC, mas somente o transcript
da tentativa candidata retorna ao L3; transcripts de tentativas internas
rejeitadas são descartados. `realize_page_numberings` recebe exclusivamente um
documento `Complete`. Se a store de page numbering provocar relayout, esse
novo candidato volta a estabilizar callbacks math antes de alimentar a próxima
realização de numbering. Esse ciclo não é mesclado com
`compiler::introspect::run_fixpoint` e não compartilha estado com ele.

Uma request corresponde a uma chamada da função. Assim `cross=true` produz e
executa duas requests consecutivas, conforme as duas chamadas reais de
`draw_cancel_line` no vanilla. A store é local ao ciclo de compilação, não é
global nem cacheada entre compilações. PNG/SVG reutilizam o documento paginado
já estabilizado. HTML não atravessa layout math paginado e fica fora até
medição própria.

Esta forma replica P240/P241/P1159: L3 realiza callbacks entre passagens e L1
layout recebe somente dados selados. Não existe implementação L3 de trait L1,
reentrada layout→eval ou execução em `native_math_cancel`. A ligação de fase
continua gate ADR-0127.

## P1293.reopen-C — transporte do modo de serialização HTML (PROPOSTO; STOP ADR-0127)

### Medição anterior à decisão

O recibo residual P1293/C SHA-256
`4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mede que ambas as formas, cristalina conservadora e vanilla contextual, são
HTML válido. No estado recebido, `03_infra/src/pipeline.rs:160` expõe
`compile_to_html_string` e `:171` expõe
`compile_to_html_string_with_features`; busca read-only não encontra parâmetro
ou tipo de modo. Ambas as entry points existentes têm, portanto, comportamento
cristalino vigente.

### Contrato público proposto

L3 usa o enum público único definido pelo owner do exporter,
`HtmlSerializationMode::{Crystalline, Vanilla}`, e acrescenta uma entry point
explícita completa:

```rust
pub fn compile_to_html_string_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    features: Features,
    mode: HtmlSerializationMode,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Ela preserva o pipeline semântico `eval(target=html, features) → Content →
export_html_with_serialization(content, mode)`, sem passar por layout paginado.
As APIs públicas existentes `compile_to_html_string` e
`compile_to_html_string_with_features` permanecem compatíveis e delegam com
`HtmlSerializationMode::Crystalline`. Não há leitura de CLI/env, estado global,
inferência pelo output path ou ativação de feature pelo modo.

O modo afeta somente o encode HTML no fim do mesmo pipeline; warnings, errors,
feature gate, target e conteúdo avaliado permanecem idênticos. A assinatura
pública nova e o default preservado exigem confirmação humana ADR-0127 antes de
código ou resselo. Refutam a forma: mudar APIs antigas, mover lógica entre
eval/layout/export, duplicar o enum ou observar o modo em pipeline paginado.

## P1340 — delegação da estabilização contextual seletiva

### Medição anterior à decisão

`diagnosticos/p1340-baseline.json` congela o antecedente: a expansão
`:244-281` aborta por Err e o ciclo `:721-760` termina por páginas iguais.
As cláusulas P1339 antes contidas nesta seção continuam necessárias, mas
sua coordenação passa ao owner individualizado, evitando acrescentar essa
responsabilidade ao monólito.

### Contrato proprietário de integração

Declarar módulo privado descendente `context_stabilization`. O owner
`infra/pipeline/context_stabilization.md` legitima somente seu consumer.
As fachadas públicas de expansão e compilação preservam suas assinaturas;
a pipeline entrega World/Source, conteúdo original e introspector de origem
à sessão seletiva. Nenhum caller implementa segunda regra de estabilização.

No caminho paginado com seleção efetiva, receber conteúdo, introspector,
documento e warnings finais da coordenação seletiva; seguir com os mesmos
passos de metadados, shaping e exportação. Não executar depois outro ciclo
contextual/P1159 sobre esse resultado. Sem seleção, preservar o percurso
legado e seus diagnósticos. Os helpers privados de layout math, page numbering
e introspecção runtime continuam disponíveis ao descendente, sem alargar
visibilidade pública ou transferir suas próprias responsabilidades.

As fachadas de expansão delegam sua parte à mesma sessão, sem alegar
estabilidade de páginas que não produziram. HTML mantém caminho semântico
separado. A individualização não corrige show rules/P1037 incidentalmente,
não altera features/defaults nem fecha o P1339. Os L0s de leituras e de eval
permanecem proprietários da semântica e de sua revalidação.

## P1353 — aposentadoria dos hooks de fronteira

Os hooks `post_eval`, `compilation_returned` e `exporter_dispatched`, bem como o
`include!` de testes externos condicionados por `p1339_observation`, foram
sucedidos por P1353 e deixam de ser obrigação materializável. Seus registros
anteriores continuam históricos, não fonte de linhagem para instrumentação.

O pipeline produtivo preserva avaliação, introspecção, preparação contextual,
layout, shaping e exportação nas mesmas fases e com os mesmos defaults. Não
executar callback adicional nem fabricar observações de fronteira. Qualquer
instrumentação futura exige nova medição, atualização L0 e `cfg` formalizado.
