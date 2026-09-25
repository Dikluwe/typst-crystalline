# Pipeline — L3 orquestração
Hash do Código: c1aa1309

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

## Módulo
`03_infra/src/pipeline.rs`

## Propósito

Orquestra o pipeline completo de compilação em L3. Esconde o
boilerplate `comemo` (Routines, Traced, Sink, Route) e expõe
APIs alto-nível à L4 (04_wiring) e aos testes.

## P1408 — provas de largura fracionária pública de `box`

O pipeline prova a morfologia integrada de `box(width: Nfr)` numa linha
finita: uma caixa `1fr` leva o suffix à margem, duas caixas positivas repartem
o restante segundo a razão dos pesos, `0fr` não recebe restante e `-1fr`
isolado recebe o restante conforme a regra `Fr::share` medida no vanila.

Os testes percorrem somente os itens do documento produzido e não serializam
ou acumulam dumps. Casos focais usam uma página pequena; o controle com
`repeat` conserva o teto de 1000 cópias. Soma zero e razão não finita devem
terminar sem deslocamento, loop ou alocação dependente da magnitude do peso.

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs`.

## P1247 — composição do contexto de destinos SVG

Esta composição é obrigação vigente do pipeline.

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

Esta composição é obrigação vigente do pipeline.

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

**Retificado por P1467** (ver seção própria abaixo): "o chamador de produção
L4 usa `Enabled` na ausência de `--no-pdf-tags`" deixa de ser exato quando o
documento contém `#set format.pdf(tagged: ...)`. A partir de P1467, a ausência
de decisão CLI (moderna e legada) não colapsa mais em `Enabled` — é preservada
como `None` até a pipeline poder consultar `document.format_options.pdf.tagged`.
O restante desta seção (threading de `PdfTags` até os três ramos `export_pdf*`,
independência de `StreamMode`, ausência de validação PDF/UA) permanece exato.

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

## P1486 — fechar o snapshot lógico antes do PDF (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

`realize_page_numberings` em `03_infra/src/pipeline.rs:1780-1856` já constrói
`PageStore` com `logical_numbers` a partir de `logical_start`; o resultado
final da compilação sai como `(PagedDocument, TagIntrospector)` em
`pipeline.rs:1558-1575`. O probe com `counter(page).update(10)` do diagnóstico
`typst-p1486-page-labels.md` mostra que índice físico não substitui esse
snapshot. A inferência de que a store devolvida acompanha o documento final
seria refutada por desalinhamento de comprimento ou número depois da última
passagem de layout. A igualdade de store/estrutura é mecânica; o rótulo PDF
e o número lógico de linguagem são semânticos (ADR-0107).

No bundle, `pipeline.rs:812-842` exporta documentos em ordem; em
`pipeline/context_stabilization.rs:829-915`, o `PageStore` concatena as
páginas de cada body paginado e não avança em HTML. O renderer do bundle
(`pipeline.rs:950-1070`) recebe o store global mas hoje devolve só bytes,
sem transportar o offset físico do documento. Inferência: o offset deve
acompanhar cada documento paginado; refutação seria uma sequência PDF/HTML/PDF
em que a label da segunda saída consultasse a posição errada no store.

### Decisão condicionada

Depois da última estabilização de layout, antes do shaping/export, copiar
para cada `doc.pages[i].logical_number` o valor de
`intr_for_positions.page_store.logical_number_for_page(i + 1)` do snapshot
final. Exigir alinhamento 1:1; não inventar número lógico a partir do índice
quando o store deveria estar completo. Documentos sem páginas ficam vazios.
Isto não move a realização entre fases nem cria novo callback. PNG/SVG/HTML
ignoram o novo campo e preservam seus comportamentos. A alteração depende do
campo público `Page.logical_number` em `entities/layout_types.md` P1486 e
aguarda a mesma confirmação ADR-0127.

No bundle, percorrer os documentos na ordem usada pela store global, com
offset físico acumulado apenas para bodies paginados (PDF/SVG/PNG); HTML e
assets não consomem entrada. Antes de exportar cada body paginado, copiar
`bundle_intr.page_store.logical_number_for_page(offset + i + 1)` para
`document.pages[i].logical_number`; avançar o offset por
`document.pages.len()`, inclusive para SVG/PNG. O renderer pode devolver
internamente a contagem de páginas para esse fim, sem mudar API pública.
Desalinhamento da store é erro, não fallback para índice exportado.
Testes incluem PDF/HTML/PDF com update do contador e seleção na segunda
saída, além de SVG/PNG entre dois PDFs.

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
layout recebe somente dados validados. Não existe implementação L3 de trait L1,
reentrada layout→eval ou execução em `native_math_cancel`. A ligação de fase
continua gate ADR-0127.

## Transporte do modo de serialização HTML

O pipeline expõe
`compile_to_html_string_with_features_and_serialization(world, source,
features, mode)`. Ele executa
`eval(target=html, features) → Content → export_html_with_serialization`
sem atravessar layout paginado.

As entrypoints anteriores delegam com `HtmlSerializationMode::Crystalline`.
O modo afeta somente o encode final: warnings, errors, feature gate, target e
conteúdo avaliado permanecem iguais. Não ler CLI/env, inferir pelo path,
duplicar o enum ou observar esse modo no pipeline paginado.

## P1360 — transporte da serialização legível por target

A pipeline oferece variantes explícitas que acrescentam `pretty: bool` às
entrypoints usadas por L4, preservando wrappers anteriores com `false`.
No PDF, o valor é encaminhado como `PdfSerializationMode` aos três ramos
font-aware, sem alterar `StreamMode`, `PdfTags`, features, documento ou fases
(este owner não *muta* o documento; a partir de P1467, as três novas entry
points *leem* `document.format_options.pdf.pretty` quando o caller não
decide — ver seção própria). No SVG, constrói
`SvgOptions { pretty, ..Default::default() }`. No HTML,
encaminha o valor ao exporter junto do `HtmlSerializationMode` já resolvido.
PNG não recebe parâmetro novo.

O pipeline não formata saída, não infere pretty pela extensão e não combina
pretty com modo compacto/tagging. As variantes de compatibilidade continuam
com saída compacta (`pretty=false`).

## P1340 — delegação da estabilização contextual seletiva

### Medição anterior à decisão

A medição histórica congela o antecedente: a expansão
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

## P1401 — avaliação contra documento compilado (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline P1401, `eval_expression_with_sink_features` delega diretamente à
avaliação isolada L1; já `compile_to_paged_document_full_error_and_features`
constrói o snapshot de introspecção do documento, mas não o disponibiliza ao
comando eval. A fonte vanila pinada compila o input antes de avaliar a expressão
com o introspector resultante.

### Decisão proposta

L3 expõe uma operação contextual paginada que recebe `World`, `Source`,
expressão e `Features`, executa as mesmas fases estabilizadas da compilação
paginada vigente e avalia a expressão com o `TagIntrospector` final coerente
com o documento. O retorno preserva `SourceResult<Value>` e warnings; não
exporta PDF nem conserva o `PagedDocument` além da chamada. O caminho isolado
permanece intacto. P1401 não adiciona target HTML, não reordena eval/layout e
não cria segunda implementação de introspecção. A nova API L3 aguarda o gate.

## P1403 — preservar PageStore no introspector devolvido

### Medição anterior à decisão

Com documento sem blocos `context`, `#set page(numbering: "I")` seguido de
avaliação por `--in` devolve página `1`, mas `location.page-numbering()` devolve
`none`. O loop legado realiza e injeta corretamente o `PageStore` em
`layout_intr`, porém devolve o clone pré-layout `intr_for_positions`, no qual
só injeta posições ao final. Assim o consumer contextual de P1403 não consegue
observar o numbering que o próprio pipeline já realizou.

### Obrigação

Antes de devolver o caminho legado estabilizado, copiar o `PageStore` já
realizado de `layout_intr` para `intr_for_positions`. Não executar novo layout,
não recalcular numbering e não alterar o caminho seletivo, que já devolve a
candidate paginada. Posições finais continuam sendo injetadas depois do
shaping. Esta é correção interna de perda de snapshot em paridade contínua.

## P1406 — realização pós-introspecção de outline (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `fe3209458`, eval produz conteúdo e show rules antes de a
introspecção sintetizar `OutlineEntry`; o layout depois fabrica texto próprio.
No vanila ratificado, a sequência de entries é realizada sob o escopo lexical
do outline entre introspecção e layout. A fixture com dois outlines prova que a
pilha final/global não representa esse observável.

### Decisão proposta

No caminho paginado, L3 orquestra
`eval → introspecção → realização L1 de outlines → layout`. Cada ocorrência usa
seu snapshot lexical causal; L3 não interpreta selector, parent, entry ou show
default. Se páginas ou `prefix-info` mudarem, o candidato volta ao ciclo
limitado já contratado, sempre reconstruído da entrada não transformada. Só um
candidato convergido alcança shaping/export. HTML e LoF/LoT não atravessam esta
fase neste passo.

Confirmação humana explícita recebida em 2026-09-15; esta alteração da fronteira
eval/introspecção/layout está autorizada pelo gate ADR-0127.

## P1407 — oráculos focais do fill realizado

O pipeline não ganha fase nem regra de negócio. Seus testes paginados verificam
o fragmento público P1407: uma caixa `1fr` repete mais de uma vez, a largura
maior produz estritamente mais cópias que a menor e as cópias do fill default
permanecem dentro do mesmo link de destino. Esses oráculos não inspecionam o
estado privado nem fixam estrutura Rust.

## P1502 — controles de morfologia do repeat após box

### Medição anterior à decisão

No HEAD `f6d4f890141203bbdbbb91dd0ac8d5009c757abc`, worktree limpa,
o comando `cargo test -p typst-infra --lib p1407_` termina exit 101:
0/5 testes passam. A sonda bilateral P1498 mediu 23/1 pontos com
prefixo em repeat e 23/23 sem prefixo. A sonda de 2026-09-25
14:36:52–14:37:43 UTC mediu 18/18 sem segunda box e 18/1 ao inserir
body `rect` depois do repeat. O contrato produtivo é do owner boxed;
esta suíte observa somente o documento paginado.

### Obrigação da suíte

Conservar os cinco testes P1407 de unicidade A/B, sensibilidade à
largura, fill dentro do link, teto 1000 e gap/justify. Acrescentar
controle com e sem prefixo na mesma largura, duas boxes `1fr` irmãs
com repeats distintos e suffixes únicos, e body de bloco após repeat
pendente sem prefixo. Box aninhada é controle adicional para índices
fr/repeat locais. A aceitação é texto, escopo do link e posições
observáveis, não índices privados nem bytes de PDF. Nenhuma fase ou
regra de negócio é adicionada ao pipeline.

## P1411 — contexto produzido pela realização de outline

### Medição anterior à decisão

No baseline `82e210bd1826ccbf0cd2378cddc605fb2546cf34`, os callbacks
`show outline.entry` que devolvem `context [#it.page()]` ou
`context [#it.prefix()]` são executados somente pela realização P1406, depois
da expansão contextual original. O layout ignora o `ContextBlock` ainda não
expandido e o texto desaparece. O vanila ratificado produz respectivamente
`1` e `1.`. A tentativa falsificadora P1411 de fazer
`Content::map_content` descer em `Outline.realized_body` não altera o resultado:
o corpo ainda não existe quando a expansão original percorre a árvore.

### Decisão proposta

No ciclo paginado, depois de cada realização de outlines e antes do layout do
candidato, L3 deve descobrir os `ContextBlock` recém-produzidos na árvore
realizada, atribuir-lhes Locations causais por introspecção dessa árvore e
expandir seus callbacks com o snapshot corrente de páginas/posições. O conteúdo
expandido, nunca o candidato com blocos pendentes, segue para layout.

Produtores que já existiam na entrada da realização já foram expandidos pela
fase contextual original. A passagem pós-realização identifica esses mesmos
objetos causais e remove somente seus marcadores da vista temporária antes de
descobrir produtores novos; seus resultados irmãos permanecem intactos. Não
reexecutar um produtor antigo, não comparar apenas id numérico e não duplicar
sua contribuição ou seus warnings.

A repetição usa o orçamento de convergência já limitado da compilação e sempre
reconstrói o candidato a partir da entrada não transformada; não acumula bodies,
callbacks, warnings ou eventos de introspecção. O primeiro layout pode fornecer
o snapshot provisório necessário a `page()`, mas somente um candidato estável
alcança shaping/export. HTML continua fora da realização P1406. Matching e
execução de `outline.entry` permanecem em L1; L3 somente ordena as fases e usa
as fachadas existentes de introspecção e expansão contextual.

Esta inclusão executa callbacks contextuais após a realização P1406 e altera a
fronteira introspecção/realização/contexto/layout. Confirmação humana explícita
recebida em 2026-09-15; a mudança de fase está autorizada conforme ADR-0127.

## P1415 — realização de `par.line(numbering: function)` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `0a7de3dd9`, `layout_with_realized_math_callbacks` já executa o
protocolo puro `Pending → realizar em L3 → repetir`, com teto oito, mas só para
`cancel.angle`. O vanila chama o callback de `par.line` depois de conhecer cada
linha física e passa um único inteiro; a fixture binária falha com
`missing argument: b`. Eval antecipado e layout com Engine estão refutados.

### Decisão proposta

Substituir a coordenação exclusivamente math por coordenação combinada dos
stores independentes de math e `par.line`. Cada candidato chama a entry point
L1 combinada. Ao receber requests de linha, L3 constrói `Engine` com o
`StyleChain` e span capturados, chama `apply_func` com exatamente
`[Value::Int(number)]` e usa a conversão canônica `value_to_content`. Erro da
closure, aridade ou conversão aborta sem documento exportável; warnings da
chamada são preservados uma vez e em ordem.

Resoluções completas formam store validada e uma nova passagem. Transcript novo,
store parcialmente consumido ou requests de qualquer família ainda pendentes
continuam o ciclo sob teto oito. Somente `Complete(PagedDocument)` alcança
realização P1159, shaping ou export. Antes de cada relayout externo de page
numbering, os stores são reconstruídos para o novo candidato; não são cacheados
entre compilações.

Math e linha partilham somente a orquestração e o orçamento: uma família não
executa a outra nem altera sua chave. HTML permanece fora do layout paginado.
Não mover `Engine` para L1, não reentrar eval a partir do Cursor e não devolver
documento provisório. A nova fase e sua superfície pública aguardam confirmação
humana explícita.

## P1421 — expansão contextual no target HTML (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `ef33ba5204870646a7c4f3046d8b210d6cd8e8cf`, a fachada HTML
executa `eval(target=html) → module.content → export_html`. Um
`Content::ContextBlock` alcança diretamente o serializer e produz
`HTML export does not yet support this content`. O caminho paginado já possui
introspecção e expansão, mas o módulo de estabilização fixa o target de replay
em `Paged` e conserva markers locatáveis necessários ao layout.

O vanilla ratificado, sob `compile --features html --format html`, mede:

- `#context assert.eq(target(), "html")` → exit 0, body vazio;
- `#context target()` → `<body><p>html</p></body>`;
- `#context [#html.elem("div")]` → `<body><div></div></body>`.

### Decisão proposta

Depois de eval HTML e antes do exporter, L3 constrói a topologia de
introspecção do Content e delega ao owner privado de estabilização uma
preparação HTML sem layout. O conteúdo preparado substitui ContextBlocks pelos
resultados executados com `EvalTarget::Html` e features/full-error causais; a
vista final entregue ao serializer não contém o marker locatável paginado.
Warnings e erros das closures são anexados/devolvidos pelo canal existente.

Esta fase não cria `PagedDocument`, positions ou PageStore, não chama layout,
não entra no fixpoint paginado e não altera modo de serialização, pretty,
feature gate ou assinatura pública das fachadas HTML. Leituras que exigem
estabilidade paginada e qualquer ContextBlock residual continuam fail-closed;
não são convertidos em vazio nem em sucesso. O exporter conserva o diagnóstico
para conteúdo realmente fora do seu subset e não aprende a ignorar context.

O caminho paginado preserva target `Paged`, markers e coordenação vigente. A
mudança `eval → introspecção → expansão contextual → export HTML` é uma mudança
de fase e aguarda confirmação humana explícita conforme ADR-0127 antes de
qualquer código.

## P1424 — resolução paginada localizada de `html.frame` (APROVADO NO GATE ADR-0127 EM 2026-09-16)

### Medição anterior à decisão

Depois de P1421, HTML segue `eval(Html) → introspecção → preparação contextual
→ export`. A árvore pode preservar semântica, mas não existe fase capaz de
converter uma ocorrência `HtmlFrame` em SVG. O vanilla faz essa conversão
localmente com target interno paged, sem converter o documento HTML inteiro em
`PagedDocument`.

### Contrato proposto

Inserir, somente quando a árvore preparada contém `Content::HtmlFrame`, a fase
local `resolve_html_frames` entre preparação contextual e export. Ela percorre
as ocorrências em ordem, conserva os estilos efetivos, chama a entry point
intrínseca L1 com target paged, resolve fontes/imagens pelo World existente e
fornece ao exporter o SVG inline correspondente. Ausência de frames mantém o
caminho e bytes atuais.

Não paginar o documento HTML inteiro, não trocar `EvalTarget::Html` da
avaliação externa, não ativar feature implicitamente e não guardar SVG em L1.
Falha de layout/render da ilha aborta a compilação; não apagar, substituir por
texto ou emitir `<frame>`. A nova fase foi confirmada pelo dono em 2026-09-16
e fica limitada pelo escopo de P1424.

## P1462 — extração e resolução de `format.svg.pretty` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `7442d33b6bf48f136826d28cf71bd62f8710a432`, as fachadas SVG
sem argumento explícito delegam `false`; a fachada ativa recebe `pretty: bool`
e constrói `SvgOptions` diretamente. A pipeline não consulta conteúdo nem
`PagedDocument`. O cristalino não possui opção documental.

No vanilla ratificado, probes válidos numa página vazia de 10 pt mediram:

```text
documento ausente + CLI ausente       -> compacto
documento true    + CLI ausente       -> pretty
documento true    + CLI false         -> compacto
documento false   + CLI true          -> pretty
```

O primeiro SVG tem 243 bytes e o segundo 248 bytes/3 LF no corpus medido; esses
números são evidência auxiliar daquela fixture, não contrato de bytes. A
morfologia normalizada do SVG permanece igual.

### Extração documental

No caminho paginado, chamar `collect_document_format_options` sobre o conteúdo
realizado final que alimenta o candidato de layout. A extração ocorre depois
das realizações/contextos necessários e antes de devolver um
`PagedDocument` completo. Em `Complete(mut document)`, associar exatamente as
opções extraídas ao campo `document.format_options`. Erro de regra em container
aborta pelo canal `SourceResult`; não exportar documento provisório.

Relayouts do ciclo calculam as opções do respectivo candidato sem acumular
estado entre iterações. A operação é pura L1; L3 não percorre `Content`, não
interpreta chaves de style e não duplica a mensagem de erro.

### Precedência e APIs SVG

Adicionar uma entry point pública explícita que receba
`pretty_override: Option<bool>`. O valor efetivo passado a `SvgOptions` é:

```text
pretty_override.unwrap_or(document.format_options.svg.pretty)
```

As fachadas SVG sem argumento `pretty` delegam `None`, permitindo a opção do
documento. A fachada de compatibilidade que já recebe `pretty: bool` delega
`Some(pretty)`, preservando a semântica de override explícito e sua assinatura.
O exporter `infra/export/svg.rs` continua a receber somente o booleano final e
não aprende precedência.

Este fio aplica o Núcleo Tekt `format-option-precedence` pinado acima. O SVG
interno de `html.frame` não é um export do documento e permanece controlado
pelo modo HTML; não deve observar `format.svg`. PDF, PNG e HTML permanecem
inalterados até P1463–P1465.

### Aceitação e gate

Testes de pipeline cobrem as quatro linhas da matriz, default, regras root,
rejeição em containers, erro de tipo/named vindo de eval e neutralidade de
PDF/PNG/HTML/frame HTML. Comparar estrutura/whitespace pertinente, sem fixar
bytes auxiliares como contrato.

A nova API pública e a extração antes do documento completo foram
materializadas depois da confirmação humana de 2026-09-19.

## P1463 — resolução de `format.png.ppi` e override CLI (MATERIALIZADO)

### Medição anterior à decisão

No baseline `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`, todas as fachadas
PNG convergem para
`compile_to_png_bytes_with_timings_full_error_and_features`, que constrói
`RenderOptions::default()` separadamente nos ramos com e sem fontes. Logo a
pipeline usa sempre 2 pixels por ponto e não consulta
`document.format_options.png` nem caller externo.

No vanilla ratificado, o override CLI é convertido por `ppi / 72.0`, enquanto
o valor documental é entregue diretamente como pixels por ponto. Numa página
de 72 pt × 36 pt, documento `ppi: 72` produziu 5184 × 2592; CLI `--ppi 72`
produziu 72 × 36. Essa assimetria medida é contrato deste pin e não deve ser
normalizada por intuição de unidade.

### API e resolução

Adicionar a entry point pública explícita:

```rust
pub fn compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: Features,
    ppi_override: Option<f64>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings)
```

As fachadas PNG antigas preservam assinaturas e delegam `None`. A entry point
vigente com `features` também delega `None`; L4 chama a nova entry point.
Depois de obter o `PagedDocument`, resolver exatamente:

```text
Some(cli_ppi) -> cli_ppi / 72.0
None          -> document.format_options.png.ppi
```

Converter o resultado para `f32` somente ao construir um único
`RenderOptions { pixel_per_pt, render_bleed: false }`, usado identicamente nos
ramos `export_png` e `export_png_with_fonts`. Não recalcular default por ramo,
não consultar regra depois do documento e não fazer o documento vencer
`Some`.

Este fio aplica o Núcleo `format-option-precedence` pinado acima. A conversão
`/72` é específica do caller PNG e permanece neste prompt, não no Núcleo. O
exporter `infra/export/render.rs` continua ignorante de CLI/documento e não
muda. O caminho continua exportando somente a primeira página.

### Neutralidade e aceitação

PPI não altera eval, layout em pontos, PDF, SVG, HTML, `pretty`, `compact`,
tags, features, timings nem resolução de fontes. Testes de pipeline cobrem
default documental, documento 72/2/1.5, CLI 72/288, precedência explícita,
zero/negativo finito quanto à dimensão segura, ramos com/sem fontes e
neutralidade de outros targets. Não congelar bytes PNG nem alegar não finitos.

Os testes funcionais protegem a conversão documental, a divisão específica da
CLI, a precedência, o tratamento de `None`, os dois ramos de fontes e a
neutralidade dos outros targets.

A confirmação humana posterior ao commit causal P1463 foi recebida em
2026-09-19; a API de override parcial e a resolução PNG foram materializadas e
validadas.

## P1465 — coleta e resolução de `format.html.pretty` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, a entry point HTML
mais completa recebe `pretty: bool`; as fachadas anteriores delegam `false`.
Depois do eval semântico, a pipeline prepara contexto e exporta sem consultar
`DocumentFormatOptions`. O mesmo booleano recebido já alimenta o encoder HTML
e o `SvgOptions` usado para cada `html.frame`.

No vanilla ratificado, ausência de caller consulta `format.html.pretty`; um
caller explícito true ou false vence o documento; ausência em ambos usa false.
Regra root true produz whitespace estrutural pretty e regra em container falha
antes de exportar. É inferência que coletar sobre o conteúdo avaliado antes da
preparação contextual preserva os envelopes `Content::Styled` necessários.
Perda desses envelopes, mudança do DOM ou necessidade de paginar o documento
HTML refuta a inferência e exige reabrir o L0.

### API e resolução

Adicionar uma entry point pública explícita:

```rust
pub fn compile_to_html_string_with_features_and_serialization_and_pretty_override(
    world: &dyn World,
    source: &Source,
    features: Features,
    mode: HtmlSerializationMode,
    pretty_override: Option<bool>,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

As fachadas HTML sem parâmetro `pretty` delegam `None`. A fachada de
compatibilidade que recebe `pretty: bool` conserva assinatura e delega
`Some(pretty)`. Após eval e antes de `prepare_html`, chamar uma vez
`collect_document_format_options(&content)` e resolver:

```text
pretty_override.unwrap_or(options.html.pretty)
```

Falha de coleta aborta antes do encoder. O booleano resolvido é a única decisão
entregue tanto a `mode.export_with_resolved_frames` quanto ao `SvgOptions` do
resolvedor de `html.frame`; não consultar o documento novamente dentro da
closure e não permitir divergência entre HTML externo e frame inline.

Este fio aplica o Núcleo `format-option-precedence` pinado acima. A pipeline
não interpreta chaves, não valida named, não muda DOM/escaping nem ativa a
feature. O gate HTML existente continua anterior ao eval. SVG, PNG, PDF,
timings, fontes, introspecção e serialização Crystalline/Vanilla permanecem
semanticamente estáveis.

### Aceitação e gate

Testes de pipeline cobrem default compacto, documento true/false, regra vazia,
repetição, dois overrides explícitos, rejeição nested, propagação de erro de
eval e igualdade do DOM entre modos. Cobrir também `html.frame` com a mesma
decisão final e neutralidade dos outros targets. Não congelar bytes integrais.

A confirmação humana posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20. A
API tri-state, a consulta documental e a decisão única para HTML/`html.frame`
foram materializadas e cobertas por testes focais e funcionais.

## P1466 — estabilização PNG e preservação de `page-numbering` (PROPOSTA; AGUARDA GATE ADR-0127)

### Retificação da unidade PNG

A regra P1463 que entregava o PPI documental diretamente a
`RenderOptions::pixel_per_pt` fica superada. A semântica pública é pixels por
polegada para as duas origens e a pipeline resolve uma única vez:

```text
effective_ppi = ppi_override.unwrap_or(document.format_options.png.ppi)
pixel_per_pt  = effective_ppi / 72.0
```

O resultado convertido para `f32` alimenta o mesmo `RenderOptions` nos ramos
com e sem fontes. `144` documental e `--ppi 144` produzem as mesmas dimensões;
o override explícito continua vencendo. Uma página A4 default mede
`1191 × 1684` pixels, não `85720 × 121232`. Zero e valores negativos finitos
continuam sob a política segura já materializada no rasterizador.

### Valor contextual de `location.page-numbering()`

Em 2026-09-20, no pin `586e1bd43`, o probe com numbering closure e
`#context repr(here().page-numbering())` produziu `(..) => ..`. O cristalino
deve preservar `Numbering::Func` no snapshot injetado durante a estabilização
e devolver a própria `Value::Func` ao contexto; o conteúdo já realizado da
margem é uma vista separada e não substitui esse valor. Corrigir o primeiro
ponto de perda comprovado sem executar relayout adicional, duplicar callback
ou fabricar a string de repr.

Aceitação exige o callback visível na margem e sua repr contextual
`(..) => ..`, além dos quatro caminhos PNG que antes abortavam. Esta mudança
do default PNG requer confirmação posterior ao commit causal conforme
ADR-0127.

## P1467 — resolução documental de `format.pdf.tagged`/`format.pdf.pretty` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `306ca50022104619447fae2625090b8d9c7b0a35`,
`compile_to_pdf_bytes_impl` (`03_infra/src/pipeline.rs:1390-1401`) recebe
`pdf_tags: PdfTags` e `serialization: PdfSerializationMode` já resolvidos
pelo caller; nunca consulta `doc.format_options` (populado em
`:1046` pela mesma função, antes do render em `:1477`). Ao contrário de
`resolve_svg_pretty` (`:1964`) e `resolve_html_pretty` (`:250`), não existe
`resolve_pdf_tags`/`resolve_pdf_pretty`: os 13 entry points públicos
`compile_to_pdf_bytes*` (incluindo os 3 oráculo) tomam o enum já decidido e o
repassam sem transformação aos três ramos `export_pdf*` font-aware.

Medido em 2026-09-20 com `/usr/local/bin/typst` (vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256 confirmado idêntico ao
pin): `#set format.pdf(tagged: false)` root produz PDF sem
`/StructTreeRoot`/`/MarkInfo`; ausência da regra produz PDF tagueado (auto
resolve habilitado sem seleção de páginas); `--pdf-tagged=false` e
`--no-pdf-tags` vencem a regra do documento; `--pretty` vence
`#set format.pdf(pretty: false)`. O Núcleo `format-option-precedence` pinado
acima (`absence-defers-to-document`, `no-early-collapse`) já proíbe
exatamente o colapso hoje presente nesta função para o eixo PDF; SVG/HTML já
cumprem o Núcleo desde P1462/P1465.

### API e resolução

Introduzir dois resolvedores privados, no mesmo padrão de
`resolve_svg_pretty`/`resolve_html_pretty`:

```rust
fn resolve_pdf_tags(
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfTags {
    let enabled = match tagged_override {
        Some(explicit) => explicit,
        None => match options.pdf.tagged {
            PdfTagged::Explicit(value) => value,
            // Sem seleção de páginas (P1468 revisita): auto resolve habilitado.
            PdfTagged::Auto => true,
        },
    };
    if enabled { PdfTags::Enabled } else { PdfTags::Disabled }
}

fn resolve_pdf_pretty(
    pretty_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> PdfSerializationMode {
    if pretty_override.unwrap_or(options.pdf.pretty) {
        PdfSerializationMode::Pretty
    } else {
        PdfSerializationMode::Compact
    }
}
```

Acrescentar três entry points públicas explícitas, análogas às `_override` já
existentes para PNG/SVG/HTML, que substituem `pdf_tags: PdfTags` por
`pdf_tagged_override: Option<bool>` e `serialization: PdfSerializationMode`
por `pretty_override: Option<bool>`:

```rust
pub fn compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings)

pub fn compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)

pub fn compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: Features,
    pretty_override: Option<bool>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)
```

`compile_to_pdf_bytes_impl` troca seus parâmetros `pdf_tags: PdfTags` e
`serialization: PdfSerializationMode` pelos dois overrides acima. Imediatamente
depois de obter `doc` (logo após `:1405-1415`, antes da resolução de fontes),
chamar os dois resolvedores uma única vez e vincular os nomes locais `pdf_tags`
e `serialization` já usados pelo resto do corpo da função — nenhuma outra
linha do corpo muda. Os três ramos `export_pdf*` continuam recebendo os enums
já resolvidos e continuam ignorantes de CLI/documento/override.

Os 10 entry points públicos pré-existentes que hoje recebem `pdf_tags`/
`serialization` já resolvidos **preservam assinatura e comportamento**: cada
um passa a delegar para a nova entry point equivalente envolvendo seu próprio
valor em `Some(...)` (`Some(matches!(pdf_tags, PdfTags::Enabled))` para o eixo
tagged; `Some(matches!(serialization, PdfSerializationMode::Pretty))` para o
eixo pretty), reproduzindo exatamente o comportamento anterior — um override
explícito sempre vence, então o documento nunca é consultado por esses
caminhos de compatibilidade. Nenhuma assinatura pública existente quebra;
somente as três entry points novas expõem a consulta documental, e são elas
que L4 passa a chamar (ver `wiring.md` P1467).

Este fio aplica o Núcleo `format-option-precedence` pinado acima:
`Some` explícito vence, `None` é preservado até o documento, e o default por
formato (`auto`→habilitado sem páginas; `false` para pretty) só se aplica
quando caller e documento silenciam. `pdf_tagged_override`/`pretty_override`
não alteram `StreamMode`, features, `document_id`, timings nem os três ramos
font-aware. PNG, SVG e HTML permanecem inalterados.

### Neutralidade e aceitação

`tagged`/`pretty` não alteram eval, layout, PNG, SVG, HTML, `compact`,
`document_id`, timings, resolução de fontes ou o caminho oráculo além do que
já é medido (oráculo aplica a mesma resolução, sem mudar a transformação de
paridade de operador). Testes de pipeline cobrem: `None`/`None` com documento
`Auto`/`false` (habilitado, compacto), `None`/`None` com documento explícito
nos dois campos, `Some` explícito vencendo documento nos dois sentidos para
cada campo, os dois campos definidos independentemente, orthogonalidade com
`compact`/PPI/serialização HTML, e que as 10 entry points de compatibilidade
preservam exatamente seu comportamento anterior (regressão zero). Não
congelar bytes PDF íntegros como contrato; inspecionar presença de
`/StructTreeRoot`/`/MarkInfo` e a forma de serialização (whitespace
estrutural), nunca igualdade total de bytes.

Este é um dos quatro L0s do gate ADR-0127 de P1467 (mudança de comportamento
por defeito do produto quando o documento contém `#set format.pdf(...)`).

A confirmação humana explícita posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3` foi recebida em 2026-09-20.
Agente A implementou `resolve_pdf_tags`/`resolve_pdf_pretty` e as três
entry points `_tagged_and_pretty_override` sem ver os testes, fazendo as 10
entry points de compatibilidade delegarem preservando comportamento; Agente
B escreveu 11 testes de pipeline a partir do mesmo baseline sem ver a
implementação (RED por erro de compilação — entry point inexistente). O
teste independente passou de RED a GREEN (79/79 no módulo `pipeline::tests`,
incluindo os 11 novos e os pré-existentes de SVG/PNG/HTML) após a
integração, sem regressão.

## P1468 — resolução de `format.pdf.pages` e interação com `tagged` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No vanilla ratificado, a resolução acontece em dois pontos distintos:

1. **CLI-only, antes de compilar** (`typst-cli/src/compile.rs:162-165`): se
   `--pdf-tagged` é explicitamente `true` **e** `--pages` está presente,
   `bail!("cannot enable PDF tags when exporting a page range")` — sem sequer
   compilar o documento. Medido: exit 1, sem tentar layout.
2. **Depois de compilar e resolver com o documento**
   (`typst-pdf/src/lib.rs:95-118`, `PdfOptions::resolve`): combina override
   (CLI) com `doc: &PdfFormatOptions` primeiro; só então, se `pages` (já
   resolvido) está presente:
   - `tagged` resolvido `== Custom(true)` → erro
     `"cannot enable tagged PDF and export a page range"` (só alcançável
     quando é o **documento** que pede `tagged: true`, porque o caso CLI já
     foi barrado no ponto 1);
   - `tagged` resolvido `.is_auto()` → warning
     `"exporting a page range disables PDF tagging"` com hints
     `"the resulting PDF will be inaccessible"` e
     `"set \`pdf(tagged: false)\` to silence this warning"`, e o tagged
     efetivo final é `false`;
   - `tagged` resolvido `== Custom(false)` → nenhum diagnóstico, tagged
     efetivo `false`.

Medido também: quando **nenhuma** flag CLI de tagging está presente mas
`--pages` está, o vanilla emite o warning CLI `"using \`--pages\` implies
\`--pdf-tagged=false\`"` com hints próprios (diferente do warning
documental — `compile.rs:168-177`); quando `--pdf-tagged=false` é explícito
junto de `--pages`, nenhum warning é emitido. Isto é uma terceira via de
diagnóstico (CLI, ausência de decisão), distinta das duas acima.

### Contrato

Ampliar `resolve_pdf_tags` (P1467) para receber também `pages_override:
Option<PageRanges>` e devolver, além de `PdfTags`, o `PageRanges` efetivo e
os diagnósticos aplicáveis:

```rust
fn resolve_pdf_pages_and_tags(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, Vec<SourceDiagnostic>)>
```

Precedência de `pages`: `pages_override` (CLI) vence `options.pdf.pages`
(documento) vence `None` (todas as páginas) — mesmo Núcleo
`format-option-precedence` já pinado, sem colapso antecipado.

Com `pages` resolvido `Some(_)`:

- se `options.pdf.tagged == PdfTagged::Explicit(true)` **e** não há
  `tagged_override` que já tenha vencido com `Some(false)` — isto é, o
  tagged efetivo (pela mesma regra de `resolve_pdf_tags`) resolveria
  `Enabled` só por causa do documento — devolver `Err` com a mensagem
  documental exata acima. A checagem CLI-only (`tagged_override ==
  Some(true)` + `pages_override.is_some()`) **não** pertence a este owner —
  já foi barrada em `wiring.md` antes de a pipeline ser chamada;
- caso contrário, se o tagged efetivo resolveria `Enabled` só porque
  `options.pdf.tagged == PdfTagged::Auto` (nem CLI nem documento pediram
  explicitamente) → produzir o warning documental exato (2 hints) e forçar
  `PdfTags::Disabled`;
- se o tagged efetivo já resolveria `Disabled` (CLI ou documento
  explícitos) → `PdfTags::Disabled`, sem diagnóstico novo.

Com `pages` resolvido `None`: comportamento idêntico a P1467
(`resolve_pdf_tags` inalterado nesse ramo).

O warning/erro CLI de "ausência de decisão + `--pages` presente" (terceira
via medida acima) pertence a `wiring.md`, não a este owner — a pipeline só
produz o diagnóstico **documental**. As três entry points
`_tagged_and_pretty_override` (P1467) ganham `pages_override: Option<PageRanges>`
como novo último parâmetro, renomeadas para `_tagged_and_pretty_and_pages_override`;
as 3 entry points `_tagged_and_pretty_override` de P1467 continuam a existir
e delegam `None` para `pages_override`, preservando comportamento exato (10 +
3 entry points de compatibilidade, todas byte-idênticas ao que já produzem
hoje). O `PageRanges` efetivo é passado à camada de export
(`infra/export/mod.md`/`infra/export/builder.md`) para reindexar páginas,
links, destinos e outline.

### Aceitação e gate

Testes de pipeline cobrem: `pages` ausente (comportamento P1467 idêntico),
`pages` presente com `tagged` documental `Explicit(true)` (erro), `Auto`
(warning + Disabled), `Explicit(false)` (Disabled sem warning), precedência
CLI/documento de `pages`, e que as três novas entry points delegam
corretamente das antigas. Este é um dos L0s do gate ADR-0127 de P1468
(mudança de comportamento por defeito, novos diagnósticos).

A confirmação humana explícita posterior aos commits causais somente-L0
`9bb20b8e8`/`70d09eae2` foi recebida em 2026-09-20/21. Agente A implementou
`resolve_pdf_pages_and_tags` e as três entry points `_and_pages_override`
sem ver os testes (removendo os imports das três funções de export sem
`_and_pages` que ficaram órfãs — correção mecânica de integração, papel C);
Agente B escreveu 10 testes a partir do mesmo baseline sem ver a
implementação (RED por erro de compilação). O teste independente passou de
RED a GREEN (89/89 no módulo `pipeline::tests`, incluindo os 10 novos e os
79 pré-existentes) após a integração, sem regressão.

## P1469 — resolução de `format.pdf.standard` e interação com `tagged`/`pages` (ESCOPO REDUZIDO — contrato de linguagem) (MATERIALIZADO; gate ADR-0127 satisfeito)

Ver `entities/document_format_options.md` P1469 para a nota de escopo
reduzido (decisão do dono, 2026-09-21): esta secção resolve *nomes* e
*versão efetiva*, não conformidade estrutural.

### Medição anterior à decisão

No vanilla ratificado (`typst-pdf/src/lib.rs:120-143`, já citado em
`infra/pipeline.md` P1468), depois de resolver `pages`/`tagged` (checagem já
materializada em P1468), uma segunda checagem itera os quatro perfis
acessíveis (`A_1a`, `A_2a`, `A_3a`, `UA_1`): se algum está no conjunto de
standards **já resolvido** (CLI combinado com documento) e o `tagged`
**final** (já considerando a checagem de `pages` anterior) é `false`, falha:

```text
tagged_final == false
  E standards_final.contains(a-1a|a-2a|a-3a|ua-1)
    -> "cannot set `pdf(tagged: false)` when exporting a {name} document"
    -> hint "using `pdf(pages: ...)` implies `pdf(tagged: false)`" SÓ SE
       pages estava ativo E tagged não foi explicitamente `false` por CLI/documento
       (isto é, a causa raiz do `false` foi o colapso por `pages`, não uma escolha)
```

Esta é a checagem **documental completa** (depois de CLI+documento
combinados); a checagem CLI-only equivalente (só a lista crua da CLI, antes
de compilar) pertence a `wiring.md` P1469, mesmo padrão de duas camadas já
usado para `pages`+`tagged` em P1467/P1468.

### Contrato

Ampliar `resolve_pdf_pages_and_tags` (P1468) para
`resolve_pdf_pages_tags_and_standards`, recebendo também `standard_override:
Option<PdfStandards>` (já validado por `wiring.md` a partir da lista crua da
CLI — a pipeline não valida conflitos entre nomes CLI, só resolve
precedência) e devolvendo também o `PdfStandards` efetivo:

```rust
fn resolve_pdf_pages_tags_and_standards(
    pages_override: Option<PageRanges>,
    tagged_override: Option<bool>,
    standard_override: Option<PdfStandards>,
    options: &DocumentFormatOptions,
) -> SourceResult<(Option<PageRanges>, PdfTags, PdfStandards, Vec<SourceDiagnostic>)>
```

Ordem de resolução (preserva P1468 integralmente, acrescenta um passo
final):

1. `pages_final = pages_override.or_else(|| options.pdf.pages.clone())`.
2. `standards_final = standard_override.unwrap_or_else(|| options.pdf.standard
   .clone().unwrap_or_default())` — precedência `CLI > documento >
   PdfStandards::default()`, mesmo Núcleo `format-option-precedence` já
   pinado.
3. Resolver `tagged` exatamente como P1468 (considerando `pages_final`):
   produz `PdfTags` e os diagnósticos de `pages`+`tagged` já existentes.
4. Se `standards_final.archival` é `Some(A1a|A2a|A3a)` **ou**
   `standards_final.accessibility` é `Some(Ua1)`, e o `PdfTags` do passo 3 é
   `Disabled`: falha com a mensagem documental exata acima. O hint condicional
   só aparece quando a causa raiz de `Disabled` foi `pages_final.is_some()`
   sem um `tagged` explícito `false` de CLI/documento — reaproveitar a
   informação já calculada no passo 3 para decidir isto, sem recalcular.

As três entry points `_tagged_and_pretty_and_pages_override` (P1468) ganham
`standard_override: Option<PdfStandards>` como novo último parâmetro,
renomeadas para `_tagged_and_pretty_and_pages_and_standard_override`; as três
antigas de P1468 continuam a existir e delegam `None` para `standard`
(comportamento byte-idêntico preservado — default `PdfStandards::default()`
quando o documento também não decide).

O `PdfStandards` efetivo (versão + archival + accessibility) é passado à
camada de export (`infra/export/mod.md`/`infra/export/builder.md` P1469)
para ajustar o header `%PDF-{version}`. Este owner não sabe que existe
`krilla`, não valida conformidade estrutural e não altera a estrutura
interna do PDF além da versão declarada.

### Aceitação e gate

Testes de pipeline cobrem: `standard` ausente (comportamento P1468 idêntico,
default `1.7`); `standard` presente sem perfil acessível + `tagged: false`
(sem erro); perfil acessível + `tagged` documental `Explicit(false)` (erro,
sem hint de pages); perfil acessível + `pages` ativo sem `tagged` explícito
(erro, com hint de pages); perfil acessível + `tagged: true`/documento sem
decisão (sem erro, tagged habilitado); precedência CLI/documento de
`standard`; e que a nova entry point delega corretamente da antiga quando
`standard` é `None`. Este é um dos L0s do gate ADR-0127 de P1469 (mudança de
comportamento por defeito, novos diagnósticos).

A confirmação humana explícita posterior aos commits causais somente-L0
`65e68ad45`/`e14002d55` foi recebida em 2026-09-21. Agente A implementou
`resolve_pdf_pages_tags_and_standards` como um wrapper sobre
`resolve_pdf_pages_and_tags` (P1468, preservada sem alteração — evita
qualquer risco de regressão nos testes já existentes dessa função) e as três
entry points `_and_standard_override` sem ver os testes; Agente B escreveu 8
testes a partir do mesmo baseline sem ver a implementação. O teste
independente passou de RED (erro de compilação) a GREEN (97/97 no módulo
`pipeline::tests`, incluindo os 8 novos e os 89 pré-existentes) após a
integração, sem regressão.

## P1480 — PDF/UA-1 exige `document(title:)`: novo erro de compilação

Ver `typst-passo-1480.md` §0 para o achado de integração completo (owner
original era só `infra/export/builder.md`; este owner foi trazido para
dentro do escopo pela mesma medição). Gate ADR-0127 disparado — critério
(2) novo comportamento por defeito de uma flag já existente
(`--pdf-standard ua-1`) e (4) quebra de compatibilidade para quem hoje
depende do sucesso silencioso; humano confirmou inclusão no escopo em
chat, 2026-09-22, registrado em `typst-passo-1480.md` §0 antes desta
edição de L0.

### Medição anterior à decisão

`/usr/local/bin/typst compile --pdf-standard ua-1 <fixture sem
document(title:)>`: exit 1, `error: PDF/UA-1 error: missing document
title` + hint `set the title with \`set document(title: [...])\``.
Cristalino (mesmo commit `90aad760b`, `target/release/typst`): exit 0,
PDF emitido sem título, não-conforme, sem qualquer diagnóstico —
divergência confirmada por medição direta bilateral, não inferida.

Fonte vanilla: `krilla::configure::validate::ValidationError::
NoDocumentTitle`, convertida em `SourceDiagnostic` por
`typst-pdf/src/convert.rs:748-752` (mensagem) e `:535` (`prefix =
"{validators} error:"`, onde `{validators}` é a lista dos validadores
falhados — só `"PDF/UA-1"` no caso isolado desta fixture; combinações
com archival ficam fora do escopo desta medição, não testadas).

### Contrato

1. Checagem nova em `compile_to_pdf_bytes_impl`, logo após
   `resolve_pdf_pages_tags_and_standards` devolver `Ok` (mesmo ponto em
   que `pdf_standards`/`doc` já estão resolvidos), mesmo padrão do erro
   de anexo duplicado já existente na função (`attachment_paths`,
   alguns parágrafos abaixo): `if matches!(pdf_standards.accessibility,
   Some(PdfAccessibility::Ua1)) && doc.document_info.title.is_none() {
   return Err(...) }`.
2. Mensagem e hint byte-idênticos ao vanilla medido acima:
   `"PDF/UA-1 error: missing document title"` +
   `.with_hint("set the title with \`set document(title: [...])\`")`.
   Não generalizar para uma lista de validadores combinados (`prefix`
   dinâmico do vanilla) — fora do escopo desta medição; fixo em
   `"PDF/UA-1"` até uma medição futura confirmar o comportamento
   combinado.
3. Não muda a assinatura pública de `export_pdf*`
   (`03_infra/src/export/mod.rs`) — continuam infalíveis, `-> Vec<u8>`.
   A família `compile_to_pdf_bytes_*` já é `Result<Vec<u8>,
   Vec<SourceDiagnostic>>` desde P1468/P1469; este é mais um ramo de
   erro na mesma família, não uma mudança de contrato dela.
4. Título vazio (`document(title: [])`) não é medido aqui — só o eixo
   `None` vs `Some`. Não inventar comportamento para esse caso.

### Aceitação e gate

RED: `ua-1` sem título compila com exit 0 hoje (estado atual,
confirmado). GREEN: mesma combinação retorna `Err` com a mensagem
exata acima; `ua-1` com título continua a compilar normalmente (controlo
de não-regressão); nenhum outro perfil (archival isolado, ausência de
standard) ganha esta checagem — só `accessibility == Some(Ua1)`. Fluxo
contínuo após este L0 (a paragem ADR-0127 já foi satisfeita pela
confirmação humana registrada em `typst-passo-1480.md` §0).

## P1481 — validação de alt ausente sob PDF/UA-1 (IMPLEMENTADO; confirmado em 2026-09-23)

Medição bilateral do corpus `p1481-corpus.typ` (SHA-256
`90475d9f6bd2b6964495861203c480af064b7273c14dc0feb492f79444dbd7d4`):
com `image(alt: "A small test image")` o vanilla compila e passa
veraPDF UA-1 106/0. Sem `alt` (fixture SHA-256
`7d3b364ffe1e979d127cf303a2f13b55ad969d7d844aea831c549d6f058ae504`),
o vanilla recusa compilar: `PDF/UA-1 error: missing alt text`,
hint `make sure your images and equations have alt text`
(`typst-pdf/src/convert.rs:720-725`). O cristalino hoje rejeita
`alt` na avaliação e aceita uma imagem sem descrição até export.

Contrato confirmado: depois de resolver standards e documento, rejeitar
imagem real com `alt == None` sob UA-1 com essa mensagem e hint;
`Some("")` permanece valor fornecido, sujeito às regras externas
do perfil. Respeitar `pdf.artifact`: decoração sob seu envelope não
precisa de alt. Sem UA-1, não criar esse erro. O erro usa span de
origem se houver carrier; span detached segue a limitação existente
de `native_image` até um passo próprio. Isto muda o comportamento
de uma flag já existente e depende de `ImageElem.alt` público:
Gate ADR-0127 satisfeito em 2026-09-23.

## P1485 — compilação multi-arquivo do target bundle (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, os
entrypoints L3 de `03_infra/src/pipeline.rs` devolvem um único
PagedDocument, HTML string ou bytes de formato único; não há
coleção de arquivos. No vanilla pinado,
`typst-bundle/src/lib.rs:112-305` realiza top-level bundle,
recolhe `asset`/`document`, rejeita conteúdo solto, detecta
paths duplicados após normalização e compila cada documento no
target PDF/SVG/PNG/HTML; `typst-bundle/src/export.rs:23-113`
devolve map path→bytes. As sondas bilaterais com payload, docs
mistos, duplicatas e HTML estão em
`00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`.
Uma função `asset` que apenas retorna Content não pode produzir
essa árvore; supor o contrário é refutado pelo output medido.

### Decisão proprietária

Adicionar entrypoint L3 explícita, com assinatura e carriers:

```rust
pub struct BundleFile {
    pub path: String,         // relativo, normalizado e validado
    pub data: Vec<u8>,        // bytes finais do arquivo
    pub is_html_document: bool, // semântica do documento, não extensão
}
pub struct BundleCompileOptions {
    pub full_error: bool,
    pub features: Features,
    pub document_id: Option<[u8; 16]>,
    pub pdf_stream_mode: StreamMode,
    pub pdf_serialization: PdfSerializationMode,
    pub pdf_tagged_override: Option<bool>,
    pub pdf_pages_override: Option<PageRanges>,
    pub pdf_standard_override: Option<PdfStandards>,
    pub png_ppi_override: Option<f64>,
    pub html_serialization: HtmlSerializationMode,
    pub pretty_override: Option<bool>,
}
pub fn compile_to_bundle_files(
    world: &dyn World, source: &Source, options: BundleCompileOptions,
) -> (Result<Vec<BundleFile>, Vec<SourceDiagnostic>>,
      Vec<SourceDiagnostic>, Timings);
```

L4 transmite os overrides já parseados; ausência permanece `None`
para a precedência documental de cada formato. `pdf_stream_mode`
usa verbose por omissão e `pdf_serialization` usa compacto por
omissão; `html_serialization` conserva a escolha vanilla/cristalino
existente. `is_html_document` deriva do formato semântico do
documento, não da extensão de um asset: o servidor só injeta reload
em documento HTML. Nenhum caller recebe coleção parcial em erro.
O eval root usa target Bundle somente quando selecionado e
habilitado; top-level permite
`document`, `asset` e tags, não texto solto. Asset passa
path/payload sem conversão de formato; `Str` já foi convertido
a UTF-8 no constructor L1. Document seleciona formato por
extensão ou propriedade explícita e usa o pipeline paginado ou
HTML sem converter HTML a páginas (ADR-0128). HTML requer
`Feature::Html` adicional. Formato desconhecido, colisão,
path inválido, documento de imagem com mais de uma página e
conteúdo top-level não permitido são diagnósticos antes de
qualquer write. O mapa final não atribui sucesso parcial.

Reutilizar exporters e opções por formato quando aplicáveis,
sem copiar mecânica vanilla de paralelismo, Arc, introspector
ou bytes de PDF/SVG como critério de paridade. Introspecção,
labels e contexto entre documentos são semântica de linguagem;
se não estiverem íntegros, P1485 continua aberto e a lacuna
é medida, não convertida em MATCH. L3 não grava no diretório
final; `00_nucleo/prompts/wiring.md` possui a composição do write. Qualquer
novo endpoint público ou mudança de fase eval→layout exige
confirmação ADR-0127.

## P1485.2 — continuidade global de Locations no bundle (ratificado; ADR-0127)

### Medição anterior à decisão

Com P1485 materializado na working tree não commitada sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, o bundle
valida todos os paths e emite assets, PDF/SVG/PNG/HTML, HTTP e
live reload. Os testes independentes B têm 8/9 GREEN; o único
RED extrai `1. Beta` de `b.pdf` onde o vanilla pinado
`586e1bd43` extrai `2. Beta`. O primeiro PDF extrai
`1. Alpha` em ambos. A causa medida é a divergência entre
o `Locator` do introspector global
(`01_core/src/compiler/introspect.rs:743-765`) e o
`Locator::new()` de cada layout
(`01_core/src/compiler/layout/mod.rs:1108,3423-3424`).
`context_stabilization.rs:408-423` também reintrospecta só o
body selecionado. O diagnóstico, fixtures, horas e estado Git
estão no Passo 1485. Sem corrigir os dois eixos, P1485 permanece
aberto por esta própria especificação (§P1485 acima).
Classificação ADR-0107: numeração, consultas, referências e
diagnósticos são observáveis da linguagem; seeds, iterações,
estrutura dos arquivos e algoritmo de coordenação são mecânica.
O resultado vanilla não demonstra intenção de usar o mesmo
algoritmo. Inferência: o seed global corrige a numeração, mas
não basta para contexto selecionado porque a reintrospecção
atual é local. Refutação: com seeds e candidato global corretos,
um caso cruzado ainda divergir; ou um contexto local demonstrar
dependência que o candidato global não consegue validar.


### Decisão ratificada para o caller L3

Depois da validação de paths, conservar a ordem dos bodies e seus
estilos herdados. Obter introspector global e um
`LocatorSeed` por body da entrypoint dona em
`00_nucleo/prompts/compiler/introspect.md`. Coordenar a
preparação contextual do conjunto pelo descendente
`00_nucleo/prompts/infra/pipeline/context_stabilization.md`:
toda seleção recompõe/reintrospecta **todos** os bodies antes
de considerar estável o snapshot; não promover o
`prepared.settled` de um documento produzido sob snapshot
anterior. Paged usa a nova entrypoint de layout com seed;
Html usa target semântico e seu exporter, sem layout paginado
exceto as ilhas `html.frame`. Revalidar referências, counters,
labels e contextos cruzados antes do primeiro write. Uma falha
aborta o conjunto, preservando publicação atômica de L4.

As APIs atuais de compilação single-file permanecem iguais e
não recebem novo default. O bundle não fabrica padding
`Metadata`, não deriva offset de `elements.len()`, não
tenta corrigir apenas a string renderizada de numbering.
Aceitação exige `1. Alpha`/`2. Beta` nos dois PDFs,
query de três headings em ambos os HTMLs, `target()`
correto por documento, mistura de formatos e provas de
contexto/refs cross-document sem sucesso parcial.
Os contratos públicos L1 dos quais a implementação depende tiveram
o gate ADR-0127 confirmado pelo dono em 2026-09-24.

## P1485.3 — isolamento lógico Page no target bundle (ratificado; ADR-0127)

### Medição anterior à decisão

L3 obtém starts por body (`03_infra/src/pipeline.rs:759-761`), mas
prepara contexto como Sequence (`03_infra/src/pipeline/context_stabilization.rs:742-745`).
Na sonda RED `p1485_bundle_html_nao_herda_page_counter_do_pdf`, HTML herda
5 do PDF anterior; vanilla pinado observa 1. Update explícito no HTML deve
continuar observável (sonda vanilla: 7). Medidas/proveniência no diagnóstico
P1485 de integração. Número exposto é semântica; Sequence/seed/histórico
são mecânica (ADR-0107). Inferência: nova introspecção runtime com fronteiras
corrige a divergência sem separar o estado global. Refutação: leitura Page
cruzada errada ou perda de query, heading, label, state ou Location.

### Decisão condicionada

O bundle conserva bodies ordenados e usa snapshots puro/runtime com as
mesmas fronteiras Page de `compiler/introspect.md` P1485.3. Não resetar
heading, state nem outra chave; não hardcodar 1 no serializer HTML. Nenhuma
nova fachada pública L3, default ou write parcial. Aceitação inclui PDF
update(5) → HTML Page 1, HTML update(7) → Page 7, PDFs com início Page local
e numbering de headings global, refs/contextos entre documentos.
`.final()` multi-documento segue fora deste contrato e não pode ser
declarado verde por essas sondas. A API pública L1 foi confirmada
pelo dono em 2026-09-24.

## P1486.1 — vista marginal local do PageStore no bundle

### Medição anterior à decisão

`03_infra/src/pipeline.rs:1026-1036` entrega a store global do bundle ao
layout de cada body; `render_bundle_document` já conhece `page_offset`.
`bundle_page_views` concatena os números lógicos e numerações visíveis
(`pipeline/context_stabilization.rs:829-915`). A fonte L1 de margem
consulta páginas por índice local, mas refs entre documentos consultam
`numbering_for_page` e `reference_numbering_for_page` com índice global
(`compiler/layout/references.rs:92-119`). O diagnóstico P1486 mostra
o desvio de margem no documento único; inferência: alinhar apenas as
vistas marginais na cópia usada para layout resolve o segundo PDF sem
deslocar refs. Refutação: bundle PDF/HTML/PDF ou PDF/SVG/PNG/PDF com
update/refs cruzadas perder número, contexto ou texto.

### Decisão privada

Antes do layout de cada body paginado com offset não zero, projetar
`logical_numbers` e `visible_numberings` do store global para índices
locais (`local_i → page_offset + local_i`) na cópia do introspector
usada somente nessa renderização. Preservar sem deslocamento o total,
`numberings`, `numbering_spans`, `reference_numberings` e `supplements`:
esses eixos continuam globais para refs e consultas. O `bundle_intr`
original, usado no bind final e demais bodies, não muda. Validar offset
contra o total; desalinhamento continua erro. Não há API pública nova
nem mudança de fase; a aceitação mede margem desenhada e rótulo PDF.

## P1493 — fase localizada de posições em `html.frame` (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae`, o caminho HTML executa
contextos e remove markers antes de resolver os frames. A resolução P1424
obtém somente SVG/Page, tarde demais para replay. O RED bilateral permanece
nos três perfis; múltiplas posições e frame aninhado passam no vanilla e
falham no cristalino.

### Contrato ratificado

Entre a introspecção HTML e a projeção terminal, a pipeline fornece ao owner de
estabilização uma vista das ilhas intrínsecas. Um coletor privado percorre as
ocorrências com `StyleChain` causal, chama a fachada detalhada L1 e associa
cada `ContextBlock id` à `Location` da topologia candidata. Frames
aninhados colapsam no frame externo; sua posição final é relativa à ilha
externa. Ocorrências distintas devem ter identidades distintas.

A fase só é ativada por leitura efetiva de `location.position()` em contexto
HTML. Ela não pagina o documento inteiro, não cria `PagedDocument` global,
não injeta PageStore e não altera o target externo. O exporter recebe conteúdo
sem markers somente depois de estabilidade. O mesmo mecanismo participa de
bodies HTML no bundle misto; bodies paginados conservam seus stores atuais.

Colisão, id ausente, topologia ambígua ou tentativa sem correspondência falha
fechado.
`location.page()` permanece `none`; a Page intrínseca nunca vira página
pública. Esta nova fase foi confirmada pelo dono em 2026-09-24.

### Limite medido fora do gate P1493

Uma sonda adversarial confirmou que o vanilla também resolve a posição de uma
Location obtida por `query` dentro de uma ilha quando o leitor está fora dela.
Esse caso não pode ser acrescentado por ordinal no desenho atual: `Location` é
um contador raw reutilizável entre gerações e faria um alvo removido herdar a
posição de outro elemento. P1493 materializa somente a identidade causal de
`ContextBlock`/`here()` ratificada acima. Generalizar o store exige passo e gate
próprios para identidade estável de Locations; não fabricar associação aqui.

## P1494 — transporte da origem de espaço entre targets (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, o consumer
03_infra/src/pipeline.rs não tem diff. HTML standalone avalia com
EvalTarget::Html e passa o conteúdo por prepare_html antes do exporter
(:301-313,345-348). O bundle extrai os bodies originais e chama
prepare_bundle antes de renderizar cada documento (:885-947); só na
renderização HTML projeta seu body para nós semânticos (:1119-1145).
collect_bundle_entries aceita Content::Space como separador de topo
(:1010-1035), e bundle_html_semantic_content clona folhas restantes
(:1052-1116). A sonda P1489-S-001 observa no HTML uma diferença de forma
textual. eval/mod.rs:6970 e rules.rs:982-1055 mostram perda de origem e
matching por folha antes da serialização. Inferência: converter a nova
origem em Space ao extrair um body impediria a normalização L1 posterior;
a refutação seria um body com origem preservada após essa extração.

### Decisão ratificada

Tratar SpaceFromNewline como Space apenas na política de separadores
fora de Document/Asset no topo do bundle. Dentro de cada Document,
preservar a variante no body clonado, na preparação contextual e na
projeção posterior até o ponto L1 que normaliza o grupo textual.
bundle_html_semantic_content não colapsa a origem antecipadamente.
Paged, HTML standalone e cada body HTML/PDF do bundle observam a mesma
regra linguística de normalização e show, sem reparo específico de
target, exporter ou extensão de arquivo. Conservar fronteiras Page,
Locations, target por body, html.frame e publicação atômica já
contratadas. Nenhuma fachada pública L3 muda.

Aceitação compara texto realizado e sentinelas CJK em HTML standalone,
PDF e bundle misto; espaço explícito, latim e coreano são controles.
O gate ADR-0127 da variante pública e da fase P1494 foi confirmado
pelo dono em 2026-09-24; os consumers L3 estão autorizados.
