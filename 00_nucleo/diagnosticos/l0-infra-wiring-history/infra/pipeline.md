# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `a2054d93934fd9665c6b2bd9c9f80a41c0b19cd05206a5532bc1d229db9e28a2`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — fachada e avaliação L3
Hash do Código: 246d49ec

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Consumer único:** `03_infra/src/pipeline.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato da fachada

Conserva os adaptadores de expressão/módulo, o boilerplate comemo de
avaliação, a política de assets fora de bundle, Timings e sua serialização,
as fachadas públicas de expansão contextual e os walkers causais de contexto.
Reexporta explicitamente todas as fachadas públicas e carriers bundle
existentes, com assinaturas, defaults e caminhos externos idênticos.

As responsabilidades específicas são transferidas aos owners abaixo. Cada
um legitima somente seu consumer, em relação 1:1 (ADR-0129):

- `00_nucleo/prompts/infra/pipeline/paged.md` — paged.
- `00_nucleo/prompts/infra/pipeline/html.md` — html.
- `00_nucleo/prompts/infra/pipeline/bundle.md` — bundle.
- `00_nucleo/prompts/infra/pipeline/pdf.md` — pdf.
- `00_nucleo/prompts/infra/pipeline/raster.md` — raster.
- `00_nucleo/prompts/infra/pipeline/svg.md` — svg.
- `00_nucleo/prompts/infra/pipeline/resources.md` — resources.
- `00_nucleo/prompts/infra/pipeline/tests.md` — tests.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md` — sessão seletiva existente.

A descendência permite reuso de helpers internos com pub(super), sem
dispatch dinâmico, registry novo ou inversão de camada. A sessão seletiva
conserva seu algoritmo e usa os mesmos helpers através do pai. Os núcleos
pinados continuam a representar exclusivamente as invariantes compartilhadas.

## Aceitação da atomização

A suíte pré-existente conserva os oráculos, as fachadas externas continuam
compilando e os benchmarks de produção exercitam PDF, PNG, SVG, HTML e bundle.
Baseline e resultado ficam no diagnóstico; falhas já presentes não viram
prova de regressão nem de sucesso. Executar build, testes e crystalline-lint,
registrando separadamente violações preexistentes e do lote.

## `eval_expression_with_sink` — P1137-B-001

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

## `eval_to_module_with_sink`

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

## `expand_context_blocks` (P506, corrigido P711)

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

## P1037 — `#context` aninhado devolvia vazio em silêncio

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

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).

## P844 (achado #54 de P831) — re-introspecção pós-expansão

- Nova função pública `expand_context_blocks_and_reintrospect` (expansão + `introspect_with_introspector` do conteúdo expandido). Sonda: o `ContextBlock` é locatable no walk de introspecção, mas a expansão substitui-o por conteúdo não-locatable; o Locator do walk de layout (invariante P185C) desfasava face ao introspector pré-expansão e `CounterRegistry::value_at` devolvia o snapshot anterior (headings renumeravam a partir do bloco: `1.|1.|2.` em vez de `1.|2.|3.`). A pipeline de produção usa a nova função e re-injecta os styles CSL no `BibStore` reconstruído; o introspector reconstruído também alimenta `intr_for_positions` (P535). Probes que confirmaram o mecanismo: `#metadata(1)` e `#counter(heading).step()` entre headings (locatable que permanece) não dessincronizam.

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
