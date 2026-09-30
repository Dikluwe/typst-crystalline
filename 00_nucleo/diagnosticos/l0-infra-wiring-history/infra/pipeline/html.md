# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/html.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `c2d0084c3ef08efc97ef62f50b0fce0f0c9014519e271f873fcdcb879b54d312`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — html
Hash do Código: 2772de8b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/html.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Orquestra o target semântico HTML e suas ilhas html.frame. Conserva o gate de features, a precedência de pretty, o target causal e a preparação contextual. Fornece posições intrínsecas à sessão seletiva sem criar paginação global HTML.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

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
