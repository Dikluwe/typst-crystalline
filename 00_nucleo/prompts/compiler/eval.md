# Prompt L0 — `compiler/eval` — dispatcher e contexto
Hash do Código: a7b22ccf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/mod.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O consumer possui `EvalContext`, os entrypoints públicos, a criação do scope
base, a passagem dupla, `eval_markup` e o dispatcher exaustivo de `Expr`; os
corpos especializados já delegam aos seus módulos donos.

## Contrato

### P1374 — relação observacional do carrier de altura de Block

Medição: a introdução de `BlockHeight` torna `Option<BlockHeight>` parte da
comparação estrutural já executada por `BlockElem::observation_relation`; sem
uma implementação própria, o consumer deixa de compilar por ausência de
`ObservationEq`.

Decisão: comparar payloads relativos somente com relativos e pesos
fracionários somente com fracionários; variantes distintas são
observacionalmente diferentes. O match é nominalmente exaustivo e não altera
callbacks, I/O, API pública nem a semântica de outros elementos.

### P1225/P1285 — formatter público de representação para serialização

L1 expõe `repr_stroke_value(&Stroke) -> String` exclusivamente para consumidores
que precisam da representação morfológica pública de `Stroke`. A função delega
ao owner `compiler/eval/repr.rs`; não expõe `repr_value` genérico, não faz I/O e
não autoriza fallback de tipos desconhecidos. Consumer imediato: serializer
JSON nominal da CLI em `02_shell/src/cli.rs`.

P1285 mede no vanilla ratificado
`lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362` que
todo `Value` fora do conjunto estruturado é serializado como string de sua
`repr` pública. Para que L2 cumpra esse contrato sem duplicar formatação nem
aceder ao módulo privado, o gate ADR-0127 propõe acrescentar:

```rust
pub fn repr_value_for_serialization(value: &Value) -> String;
```

A função é uma fachada pura e total sobre `repr::repr_value`; não serializa
JSON/YAML, não faz I/O, não usa `Debug` e não decide quais variantes são
estruturadas. Essa classificação permanece no owner L2
`00_nucleo/prompts/shell/cli.md` para a CLI; os encoders L1 propostos em
P1307-R3 têm classificação própria no owner loading, nunca nesta fachada.
`repr_stroke_value` permanece compatível.
Esta ampliação de assinatura pública foi **CONFIRMADA PELO DONO EM 2026-08-30**
no gate P1285 e está autorizada para materialização.

Os entrypoints constroem scope fresco e avaliam `Source` sem I/O direto.
`eval_expression` avalia código isolado. O dispatcher preserva spans, scopes,
short-circuit, joins e eventos de fluxo. `EvalContext` transporta limites,
metadados, target, features e `FlowEvent`, sem estado global mutável. Conteúdo
de introspecção pré-show e conteúdo final pós-show permanecem distintos;
evento residual no entrypoint é erro. Mudança pública, de default ou fase para
no gate ADR-0127.

### P1300 — conjunto global fechado de constructors de cor

#### Medição anterior à decisão

Em `2026-09-03T19:01:51.133010-03:00`–`19:01:55.717850-03:00`, no baseline
`1f082370e59939de7b57992e137a9f74bfb6758f`, a medição histórica
classificou, nos quatro perfis `default`, `html`, `a11y` e `html+a11y`, os
nomes bare `hsl`, `hsv`, `linear_rgb` e seus pares sob `std` como
`CRYSTALLINE_ONLY`. No mesmo corpus, `color.hsl`, `color.hsv`,
`color.linear-rgb` e os cinco globals ratificados, bare e sob `std`, foram
`MATCH_VALUE`, sem `EXECUTION_UNKNOWN`.

A fonte vanilla pinada `a51e02804`, em
`lab/typst-original/crates/typst-library/src/lib.rs:397-401`, registra somente
`luma`, `oklab`, `oklch`, `rgb` e `cmyk` no scope global. É inferência que a
causa da assimetria é o conjunto extra registrado no scope base cristalino.
Aceitação vanilla de algum alias, ou regressão de uma rota qualificada,
refutaria a inferência; nenhuma foi observada.

#### Decisão

`make_stdlib_with_features` registra exatamente cinco constructors globais de
cor: `rgb`, `luma`, `cmyk`, `oklab` e `oklch`. Não registra `hsl`, `hsv` ou
`linear_rgb`, nem inventa o spelling global `linear-rgb`.

Como `std` projeta a mesma stdlib não sombreada, `std.hsl`, `std.hsv`,
`std.linear_rgb` e qualquer spelling alternativo `std.linear-rgb` também não
constituem bindings. Esta regra independe das features `html` e
`a11y-extras`.

O conjunto global não altera os oito fields qualificados de `color`, suas
nativas, cores predefinidas, operadores ou `color.space()`. A remoção dos três
aliases públicos é quebra de compatibilidade e permanece bloqueada pelo gate
humano ADR-0127 antes de qualquer código.

## Aceitação

Entrypoints, scope base, duas passagens, dispatcher, spans e fluxo residual são
cobertos pela suíte de eval; L1 permanece puro.

## P1215 — source numerizada em `eval_expression`

Medição mostrou que `eval_expression` avaliava via `native_eval` com
`Args::positional`, portanto com span detached. O entrypoint deve criar uma
`Source` em modo code com `world.main()` e o texto integral, avaliar os filhos
da raiz numerizada no mesmo scope fresco e devolver diagnósticos cujos spans
resolvem contra uma `Source` idêntica. Isto preserva assinatura, valores,
features e fase; apenas deixa de descartar localização pública no CLI `eval`.

## P1286 — markup smartquote consome o mesmo contrato de quotes

### Medição anterior à decisão

`eval_markup` já consulta `smartquote.enabled`/`smartquote.quotes`, mas resolve
o token antes do layout e não conhece `alternative`. A chamada programática
percorre layout. O baseline usa uma configuração única para ambas as faces.

### Decisão

Markup consulta `enabled`, `alternative` e o valor canônico de `quotes` da
StyleChain e usa o mesmo resolver puro de `compiler/lang/quotes`. Continua a
emitir `Content::Text` e não muda de fase; chamada programática continua a
emitir o leaf vigente. O fragmento P1286 exige igualdade morfológica dos pares
medidos entre `#set`/markup e chamada direta, sem alegar unificação do stack
contextual do vanilla.

## P1292 — namespace de `place.flush` preservado por `.with`

### Medição anterior à decisão

No baseline, o scope base registra `place` com `Func::native` e field access
falha. No vanilla ratificado, `place.flush` é função nomeada `flush`, e
`place.with(dx: 1pt).flush` resolve o mesmo membro e continua chamável.
`Func::With` cristalino já delega `scope()` à função interior.

### Decisão

O scope base constrói um namespace fechado de `place` com exatamente o membro
`flush`, ligado a `compiler/stdlib/layout.md::native_flush`, e registra `place`
por `Func::native_with_namespace`. A função principal continua a mesma
`native_place`; não vira Module, não perde chamabilidade e não copia argumentos
pré-ligados para `flush`. A delegação vigente de `Func::With::scope()` é
preservada, não duplicada neste owner.

Aceitação: `repr(place.flush) == "flush"`,
`repr(place.with(dx: 1pt).flush) == "flush"` e ambas as chamadas constroem a
mesma `Content::Flush`. Named alheio ao namespace não é inventado.

## P1288 — filtragem uniforme de bindings por feature (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs:117-120` transporta `Features` pelo path
HTML, e `:911-921` aplica um caso especial somente ao identificador `html`.
O módulo `pdf` é instalado integralmente em `:2113`; não há filtragem das três
funções `a11y-extras` no contexto efetivo.

### Decisão proposta

`EvalContext.features` usa o owner canônico `entities::compiler_features`.
A construção/resolução do scope expõe cada binding gated somente quando a
feature correspondente está presente. Sem `A11yExtras`,
`pdf.table-summary`, `pdf.header-cell` e `pdf.data-cell` não existem; com a
feature, os três existem em conjunto. `Html` continua independente e o target
não modifica o set.

O diagnóstico de binding gated ausente conserva a semântica pública medida de
feature não habilitada. O mecanismo concreto pode ser scope filtrado ou lookup
condicional; igualdade estrutural do scope não é contrato. Não criar binding
stub nem registrar só uma das três funções.

## P1293 — identidades públicas curtas nos namespaces `grid` e `table`

### Medição anterior à decisão

Em `2026-09-01T13:24:01-03:00`, no baseline
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` com working tree não commitada,
`01_core/src/compiler/eval/mod.rs:1804-1827,2059-2078` instalava os dez fields
corretos, mas passava nomes `grid_*`/`table_*` a `Func::native`. Os aliases flat
separados permanecem em `:2083-2110`. A fonte vanilla pinada
`layout/grid/mod.rs:422-438,574-677,767-768` e
`model/table.rs:289-305,494-613,732-733` declara nomes curtos. O registro P1293
mediu chamadas diretas e via `.with` para os dez siblings e confirmou que a
causa da diferença pública é somente o primeiro argumento textual da instância
namespaced.

### Decisão

Cada instância anexada ao namespace usa exatamente este nome público:

| Namespace | members e `repr` |
|---|---|
| `grid` | `cell`, `header`, `footer`, `hline`, `vline` |
| `table` | `cell`, `header`, `footer`, `hline`, `vline` |

Chaves de scope, function pointers, argumentos, payloads e chamada direta ou
via `.with(...)` permanecem os vigentes. Os seis aliases flat cristalinos
`grid_cell`, `grid_header`, `grid_footer`, `table_cell`, `table_header` e
`table_footer` conservam seus nomes históricos; P1293 não cria aliases flat
`*_hline`/`*_vline` nem renomeia símbolos Rust.

O baseline também mediu uma divergência preexistente fora das 15 diferenças
de superfície: `grid/table.header` e `footer` cristalinos exigem pelo menos uma
célula, enquanto o vanilla aceita zero ou vários children, e algumas aridades
extras alcançam serializer/diagnóstico diferente. P1293 **não** corrige nem
reivindica paridade dessa aridade/diagnóstico; os calls atuais devem ser
preservados byte a byte quanto a function pointer e comportamento. Uma mudança
nessa divergência exige medição e L0 próprios.

Aceitação exige os dez nomes curtos, os mesmos tipos de conteúdo, `.with`
preservado e aliases flat intactos. Algum sibling com underscore, pointer/call
alterado ou divergência fora de escopo relaxada bloqueia o lote. `Unknown`
nunca é sucesso. A mudança de identidade pública fica bloqueada pelo gate
humano P1293 antes do código.

## P1305-r2 — nome público do módulo global

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs:324` e `:562` constroem o global com
`Module::new("std", stdlib.clone())`, enquanto o vanilla ratificado
`a51e02804` constrói `Module::new("global", global)` em
`lab/typst-original/crates/typst-library/src/lib.rs:374`. O nome do binding
e o nome público do objeto são distintos no vanilla.

A medição independente,
registra HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b` mais diff P1303
não commitado, status/diff/stat, fixtures e saídas completas entre
`2026-09-07T13:48:28.580684+00:00` e
`2026-09-07T13:51:19.266604+00:00`. O baseline fresco tem;
o vanilla, `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
`named-modules`, `named-aliases` e `document-route` medem `module(std)`
contra `<module global>`, preservando lookup. A contraprova de import
`std.typ` com reexport integral impede corrigir isso somente no formatter.
O dono respondeu `Autorizado` à ampliação de construção neste owner e em
`compiler/eval/modules`; medição histórica registra essa exceção.

### Decisão

Nos entrypoints de expressão e documento, o objeto Module que transporta a
stdlib não sombreada guarda exatamente o nome público `global`, usando o
constructor e carrier existentes. O binding no scope base continua `std`;
nenhum binding global chamado `global` é criado como efeito dessa mudança.
Scope, valores, ordem, features, inputs e mecanismo de clonagem permanecem
os vigentes. Aliases transportam o mesmo nome público do objeto.

O owner de imports preserva separadamente o nome lexical do binding,
conforme seu contrato P1305-r2. Este owner não muda import, Module, APIs,
diagnósticos, serialização, defaults, fases ou render. Não acrescentar flags
de origem nem reconhecer o conteúdo da stdlib para recuperar identidade.
O formatter projeta o nome já guardado sob seu próprio L0.

Aceitação cobre expressão, documento e sua equivalência com a construção
do global dentro de imports, nos perfis default/html/a11y/html+a11y, com
aliases, `std.rgb`, `std.calc` e gates existentes preservados. Refutam-na
uma rota ainda nomeada `std`, binding `global` introduzido, mudança de
lookup/feature ou necessidade de novo carrier. É correção de morfologia da
linguagem em fluxo contínuo ADR-0127, na ampliação explicitamente autorizada;
não corresponde a nova política de produto. `Unknown` obrigatório bloqueia.

## P1307-R3 — namespaces dos encoders (gate público pendente)

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs:1723-1743`, no snapshot R3,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306
integral ali capturado, registra json/yaml/toml sem namespace e CBOR com
encode. Os registros P1307
e R2
medem os membros originais com kind function/repr encode, mantendo os pais
chamáveis. O vanilla é upstream ratificado `a51e02804`; cada registro conserva
argv, binários, features, fontes e saídas. Partial With tem repr anônima,
não a repr do membro original: são observáveis distintos.

### Decisão e aceitação

O único consumer `01_core/src/compiler/eval/mod.rs` registra json/toml/yaml
como as mesmas nativas decodificadoras, cada qual com namespace fechado
contendo apenas `encode`, ligado à nativa correspondente de loading. O membro
original tem nome público curto/repr `encode`. Não criar aliases globais
`json_encode` ou encoders de csv/xml/read. Não mudar CBOR ou gates PDF/HTML.
As três construções reais da stdlib convergem para este registro, independente
dos quatro perfis de feature, sem novo default de target ou fase.

Alias, shadowing legítimo, global, import e reexport conservam identidade do
valor resolvido. Namespace do pai parcialmente aplicado é acessível, mas seus
argumentos não são transferidos ao membro: `json.with(nope: 9).encode(1)`
continua uma chamada de encode de 1. A implementação desta semântica está nos
owners Func/call_dispatch, não em condições textuais neste registro.

Não criar sidecar de origens em EvalContext, nem alterar assinatura da fachada
de repr. É inferência que o namespace já existente basta; precisar de novo
campo de Func, lookup nominal ou nova rota por feature a refuta. A adição de
superfície e defaults de encode aguarda aprovação ADR-0127 deste contrato
concreto. Testes de presença, ausência, kind, repr, chamada do pai/membro,
With e perfis são obrigatórios; ausência baseline não é GREEN.

## P1308 — Source causal do entrypoint e origem privada de função

### Medição anterior à decisão

Baseline histórico
registra HEAD b303f1f15 mais working tree. `eval/mod.rs:347–363` cria Source
parse_code local, mas entrega World original ao Engine. A matriz R6 final
pinada no diagnóstico P1307 registra dez casos cujo stderr é prefixo do
esperado sem `while calling`; `call_dispatch.rs:1017–1023` não consegue
resolver o span via World.source. Número de Span sem sua Source não basta.

Na fonte ratificada `typst-eval/src/code.rs:148`, valores retornados de
expressões recebem span; `foundations/value.rs:210–214` delega Func para
spanned, que conserva origem preexistente (`func.rs:385–389`). Closures já
têm a origem dos parâmetros (`typst-eval/src/call.rs:638`). É inferência
que o transporte privado destes dados basta; precisar inferir origem por
Value igual ou alterar contrato público a refuta.

### Decisão autorizada

`eval_expression_with_features` usa um overlay privado de World por invocação,
contendo referência ao World externo e à mesma Source parse_code avaliada.
Somente `source(id)` para o id dessa Source devolve seu clone; qualquer outro
id e todos os outros métodos de World, inclusive os com defaults, delegam
ao World original. Não recriar biblioteca, fontes, inputs, plugins, resolução
de paths/packages, relógio ou caches. Sem I/O direto, variável global, mudança
de trait ou mutação do World externo. Engine e suas descidas usam o overlay;
o entrypoint de documento continua vigente. Diagnósticos retornados mantêm
spans resolvíveis contra Source idêntica, conforme P1215.

Em `eval_expr`, Func retornada com origem privada detached pode receber o
span da expressão produtora, usando helper crate-private do owner Func.
Não substituir origem já conhecida (closure/alias/With/factory). Outros
Values não mudam. Preservar avaliação única, retorno antecipado, flow e
erros; o enriquecimento não executa a função nem participa da igualdade.
Contextual mantém sua origem de body quando já conhecida. Esta captura
serve à âncora de filter, não cria rastreador global de valores.

Aceitação exige World mínimo sem Source da expressão, contenção de trace,
origens externas, aliases/With, inputs e delegação de serviços preservados.
Nenhuma alteração da regra de trace nem fabricação de mensagens por texto.

## Observações contextuais seletivas

Cada `EvalContext` mantém um registro privado, local à avaliação, das
operações introspectivas efetivamente executadas. O registro é independente do
introspector e inicia vazio. Não existe estado global, registry reflexivo,
metadata escondida em warnings ou strings.

A seleção ocorre quando uma leitura de counter usa Selector cuja árvore contém
Element. A demanda é marcada antes de resolver Location, label ou callback,
inclusive se a leitura falhar. Uma vez selecionado o bloco, todas as suas
entradas introspectivas já observadas participam da validação.

O registro conserva operação, argumentos efetivos, Location, span, StyleChain
pertinente e resultado ou erro. Não reconstrói por AST, nome, `Value::eq` ou
texto de diagnóstico.

### Interface

```rust
impl EvalContext {
    pub fn has_filtered_counter_reads(&self) -> bool;

    pub fn context_reads_valid_for(
        &self,
        candidate: &TagIntrospector,
        engine: &mut Engine<'_>,
    ) -> SourceResult<bool>;

    pub fn context_nonconvergence_diagnostics(
        &self,
        history: &[TagIntrospector],
        engine: &mut Engine<'_>,
    ) -> SourceResult<Vec<SourceDiagnostic>>;
}
```

O getter é puro. A validação reexecuta somente as requisições observadas contra
o snapshot candidato, delegando aos owners de query, state, counter e Location.
Não reexecuta o corpo do bloco nem callbacks nunca demandadas. Engine, estilos,
capturas e recursos são os da avaliação original.

Todas as observações completas e preservadas produzem `true`; mudança,
sucesso que vira erro ou erro que vira sucesso produz `false`; falha do
mecanismo propaga `SourceDiagnostic`. Erro estável não transforma o corpo em
sucesso.

### Comparação fechada

O comparador privado retorna `Same`, `Different` ou `Unproven` e não muda
`PartialEq` ou `Hash` públicos.

- folhas comparam variante e dados observáveis; float usa bits IEEE para
  preservar sinal de zero e comparar uma observação NaN consigo própria;
- containers percorrem ordem, presença, conteúdo e metadata causal;
- `LocatedContent` inclui Location, carrier, fields e Content;
- funções e módulos só são `Same` quando sua identidade causal imutável foi
  preservada; closures recriadas não são comparadas por body ou execução;
- resultado e diagnóstico integral distinguem sucesso e erro.

`Unproven` nunca certifica estabilidade. Serviço opaco exige identidade
causal preservada; falta de prova invalida a reutilização.

### P1425 — presença de `inline` na relação observacional

Medição fail-closed no baseline causal
`0305568a7509653198f16a01441051fea47e304a`, tree
`e6785a80f345abae324c365df96126076b322c07`: em
`01_core/src/compiler/eval/mod.rs:2053-2063`, a implementação de
`ObservationEq` para `MathLimitsOverrideElem` desestrutura exaustivamente
`body`, `limits` e `inline` e compara os três. A adição P1425 de
`inline_explicit` tornaria esse consumer inválido por campo omitido (`E0027`)
e, se o destructuring fosse apenas afrouxado, permitiria declarar `Same` para
omissão e presença explícita do default.

O comparador deve incluir `inline_explicit` nominalmente na desestruturação e
na cadeia de `observation_relation`. O bit é morfologia causal da língua e
portanto duas observações que diferem somente nele são `Different`; não é
permitido ignorá-lo, inferi-lo de `inline` nem devolver `Unproven` para evitar
a comparação. Nenhuma outra variante, regra de estabilização, API ou fase
muda.

A varredura integral de Rust nesse baseline encontrou este como único
destructuring externo de `MathLimitsOverrideElem`. Os demais struct literals
pertencem ao owner da entidade; os callsites do constructor pertencem aos
owners já declarados de eval math, stdlib math, layout comum e aos testes
existentes cuja aridade requer reparo mecânico. Esta expansão do consumer foi
aprovada como revisão fail-closed R1 do contrato P1425.

### P1426 — presença do índice na relação observacional (APROVADO)

`ObservationEq` para `MathRootElem` deve desestruturar e comparar
`index_explicit` além de `index` e `radicand`. Ausência e presença explícita de
`none` são morfologias causalmente distintas e portanto `Different`; não se
pode inferir o bit do valor nem devolvê-lo como `Unproven`. A mudança do novo
campo público foi confirmada explicitamente pelo dono em 2026-09-16.

### Não convergência

A história é a sequência real de snapshots do orquestrador. O diagnóstico usa
as requisições da tentativa final projetadas sobre essa sequência, sem afirmar
que o corpo as executou em todas as rodadas. Não fabrica detalhe para consulta
opaca ou para leitura alheia.

Não há neste owner obrigação de ledger, DTO, JSON, hook ou harness de
instrumentação. Observação adicional exige owner próprio.

## P1401 — expressão com introspector injetado (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline P1401, `eval_expression_with_features` cria `EvalContext::new()`;
este inicializa `introspector` vazio. Assim `query`, `counter` e state não veem
o documento de `--in`, embora o contexto já possua o campo apropriado.

### Decisão proposta

L1 oferece um sibling público de avaliação de expressão que recebe
`&TagIntrospector`, instala um clone no `EvalContext` antes de avaliar e mantém
scope, parser, route, sink, diagnósticos, features e limites vigentes. O
entrypoint atual delega ao mesmo núcleo com `TagIntrospector::empty()`, de modo
que ausência de `--in` não muda. As varreduras recursivas de `Selector` deste
owner atravessam também `Before.base/end` e `After.base/start`; não colapsam a
árvore. Assinatura pública nova aguarda confirmação ADR-0127.

# P1402 — scope público de `outline`

O binding global `outline` usa namespace nativo contendo `entry`; `entry` usa
namespace contendo `body`. As identidades são funções nativas estáticas e
continuam selecionáveis por endereço, não por nome textual.

# P1403 — identidades de helpers da entry

O namespace nativo de `outline.entry` contém também `prefix` e `page`.
`prefix` é nativa comum; `page` é nativa com acesso a Engine/Scopes para
realizar numberings funcionais. `page` registra uma leitura causal fechada por
Location, reexecutável contra candidatos do fixpoint; não depende só da leitura
de `query`. Ambas preservam nome e identidade estáticos.

# P1404 — identidade de `outline.entry.inner`

O namespace da entry contém `inner` como nativa com Engine/Scopes. A nativa
reusa a leitura causal de `page`; não cria vocabulário paralelo. A comparação
observacional de `BoxedElem` inclui o carrier fracionário P1404.

# P1405 — identidade e leitura causal de `outline.entry.indented`

O namespace da entry contém `indented` como nativa com Engine/Scopes. Quando o
parent privado está presente na StyleChain, a nativa registra leitura causal
fechada contendo entry, prefix, inner e gap; o replay usa os mesmos estilos e o
novo candidato do introspector, permitindo que máximos `prefix-info` atinjam
fixpoint. Sem parent, o erro ocorre antes do registro. O diagnóstico de
não-convergência não reclassifica esta leitura como counter ou state.
# P1406 — snapshot causal de outline (PROPOSTO; gate autorizado)

Medição anterior à decisão: no baseline `fe3209458`, o intercept eager conhece
a pilha lexical no instante em que `outline()` devolve Content, enquanto a
síntese de `OutlineEntry` só ocorre depois na introspecção. O vanila ratificado
prova por dois outlines que a pilha final não pode substituir esse instante.

No passe renderizável, `intercept_content` anexa ao `OutlineElem` snapshots
privados e imutáveis das show rules e estilos correntes antes de aplicar regras
ao próprio outline. O passe original de introspecção não inventa snapshots.
Identidade/observação do outline preserva a presença explícita do título e a
causalidade das regras; o carrier não é campo da linguagem. Confirmação humana
explícita recebida em 2026-09-15.

# P1413 — namespace público `raw.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `e7b2ddd44`, `raw` é uma função nativa sem namespace. O vanila
expõe `raw.line`, cujo `repr` é `line`, como constructor e seletor de elemento.

### Decisão proposta

O binding global `raw` usa namespace nativo contendo `line`. `line` é uma
`Func::element` de identidade `raw.line`; seu constructor recebe
`number, count, text, body` posicionais, valida tipos/cardinalidade e produz o
carrier dinâmico proprietário de `entities/elements/raw.md`. A maquinaria
existente converte a função em `Selector::DynKind`, sem matching por nome
textual improvisado e sem nova variante `NodeKind`.

Esta ampliação de contrato público aguarda confirmação humana explícita antes
de L1.

# P1414 — namespace público `par.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `df196b35b`, `par` usa `Func::native` sem namespace e field access
falha. No vanila ratificado, o parent permanece chamável, `repr(par.line)` é
`"line"` e o membro rejeita construção manual.

### Decisão proposta

Registrar `par` por `Func::native_with_namespace`, preservando `native_par` e
anexando somente `line`, ligado a `structural/par.md::native_par_line` com nome
público curto. Não transferir argumentos de `par.with(...)`, não criar alias
flat e não mudar a identidade do parent. A ampliação aguarda confirmação
humana explícita.

# P1427 — igualdade observacional de `MathFrac` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`ObservationEq` compara hoje somente numerador, denominador e `line`; dois
conteúdos que diferem em estilo explícito ou proveniência de parênteses seriam
tratados como iguais apesar de poderem produzir morfologia/layout distintos.

### Decisão proposta

Comparar também estilo opcional e ambos os bits de deparenthesization. Não
normalizar `None` para `Vertical`: ausência e vertical explícito divergem em
`repr`. `line=false` permanece parte da observação para proteger `binom`.

## P1436 — igualdade observável de `MathLr` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

`ObservationEq` é exaustivo sobre `Content` e não possui a nova identidade.
Delegar somente ao body faria `size` explícito/cascata e presença divergentes
parecerem iguais apesar de alterarem `repr` ou geometria.

### Decisão proposta

Adicionar braço `MathLr` e implementação de `ObservationEq` que compare body,
`Rel<Length>` e `size_explicit`. A comparação é estrutural dos observáveis do
carrier; não resolve cascata, não executa layout e não equipara `100%`
explícito a ausência.

## P1437 — igualdade observável de `MathMid` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

`ObservationEq` é exaustivo e precisa preservar a nova identidade. O único
estado próprio de `MidElem` é o body; o vínculo ao `lr` é contexto de layout.

### Decisão proposta

Adicionar braço `MathMid` e comparar somente o body. Não resolver contexto,
classe ou stretch; `MathMid(body)` nunca é igual ao body nu nem a um
`MathDelimited` semelhante.

## P1439 — igualdade observável de `MathOverbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

Body, valor da annotation e presença explícita alteram morfologia ou `repr`;
o match exaustivo ainda não possui essa identidade.

### Decisão proposta

Adicionar braço `MathOverbrace` e comparar body, annotation e
`annotation_explicit`. Não comparar glifos ou caixas derivadas, não equiparar
omissão a `none` explícito e não igualar ao `MathUnderover` mecânico.

## P1440 — igualdade observável de `MathOverbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, valor da annotation e presença explícita alteram morfologia ou `repr`;
o match exaustivo ainda não possui essa identidade.

### Decisão proposta

Adicionar braço `MathOverbracket` e comparar body, annotation e
`annotation_explicit`. Não comparar glifos ou caixas derivadas, não equiparar
omissão a `none` explícito e não igualar a `MathOverbrace` nem ao
`MathUnderover` mecânico.

## P1441 — igualdade observacional de `MathOverline` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

A identidade nova é estrutural e body-only; não existe campo cosmético
textual nem caixa de layout em seu contrato.

### Decisão proposta

Adicionar braço `MathOverline` e comparar exclusivamente o body. Não comparar
regra, constantes ou estilo cramped derivados e não igualar a
`MathUnderline` nem ao `Overline` textual.

## P1442 — igualdade observável de `MathOverparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, valor da annotation e presença explícita alteram morfologia ou `repr`;
o match exaustivo ainda não possui essa identidade.

### Decisão proposta

Adicionar braço `MathOverparen` e comparar body, annotation e
`annotation_explicit`. Não comparar glifo ou caixas derivadas, não equiparar
omissão a `none` explícito e não igualar a outro spreader nem ao
`MathUnderover` mecânico.

## P1443 — igualdade observável de `MathOvershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, valor da annotation e presença explícita alteram morfologia ou `repr`;
o match exaustivo ainda não possui essa identidade.

### Decisão proposta

Adicionar braço `MathOvershell` e comparar body, annotation e
`annotation_explicit`. Não comparar glifo ou caixas derivadas, não equiparar
omissão a `none` explícito e não igualar a outro spreader nem ao
`MathUnderover` mecânico.

## P1444 — igualdade observável de `MathPrimes` (MATERIALIZADO)

### Medição anterior à decisão

O count é o único estado público do elemento e distingue
`math.primes(3) == math.primes(4)`. Texto visualmente semelhante não possui a
mesma identidade.

### Decisão proposta

Adicionar braço `MathPrimes` e comparar exclusivamente `count`. Não comparar
glyph, largura ou frame derivados e não equiparar o carrier a `MathText` ou
`MathSequence`.
## P1446 — igualdade observacional de `MathStretch` (PROPOSTA; gate ADR-0127)

`Value::Content(MathStretch)` participa da igualdade observacional por body,
tamanho e presença causal. Omitido e `100%` explícito não são colapsados para
fins de representação/fields, ainda que o alvo geométrico efetivo coincida.
Adicionar braços exaustivos sem wildcard e preservar o carrier nas passagens
de avaliação.

## P1448 — igualdade observável de `MathUnderbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, valor da annotation e presença explícita alteram morfologia, fields ou
repr; o match exaustivo ainda não possui essa identidade.

### Decisão proposta

Adicionar braço `MathUnderbrace` e comparar body, annotation e
`annotation_explicit`. Não comparar glifos ou caixas derivadas, não equiparar
omissão a `none` explícito e não igualar ao `MathUnderover` mecânico.

## P1449 — igualdade observável de `MathUnderbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, annotation e presença explícita alteram morfologia, fields ou repr; o
match exaustivo não possui a identidade `underbracket`.

### Decisão proposta

Adicionar braço `MathUnderbracket` e comparar body, annotation e
`annotation_explicit`. Não comparar U+23B5 ou caixas derivadas, não colapsar
omissão com `none` explícito e não equiparar ao `MathUnderover` mecânico.

## P1450 — igualdade observável de `MathUnderparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, annotation e presença explícita alteram morfologia, fields ou repr; o
match exaustivo não possui a identidade `underparen`.

### Decisão proposta

Adicionar braço `MathUnderparen` e comparar body, annotation e
`annotation_explicit`. Não comparar U+23DD ou caixas derivadas, não colapsar
omissão com `none` explícito nem equiparar ao `MathUnderover` mecânico.

## P1451 — igualdade observável de `MathUndershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

Body, annotation e presença explícita alteram morfologia, fields ou repr; o
match exaustivo não possui a identidade `undershell`.

### Decisão proposta

Adicionar braço `MathUndershell` e comparar body, annotation e
`annotation_explicit`. Não comparar U+23E1 ou caixas derivadas, não colapsar
omissão com `none` explícito nem equiparar ao `MathUnderover` mecânico.

## P1461 — registro da raiz pública `format` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

No baseline P1461, HEAD
`7392f490397c56876fcf6134e86c44b528b396bb`,
`make_stdlib_with_features` em
`01_core/src/compiler/eval/mod.rs:7594-7773` importa as factories de módulos e
registra `html` diretamente, mas não importa nem registra `format`. Probes
bilaterais nos perfis `default`, `a11y-extras`, `html` e `html+a11y` mediram a
raiz vanilla como módulo nomeado `format`; o cristalino falha antes do field
access com `unknown variable format`.

É inferência que uma única inserção no scope global basta também para
`std.format`, porque `std` já projeta a stdlib não sombreada e o clone de
`Module` preserva o carrier por `Arc`. A necessidade de binding `std` especial,
mudança em `Module`, lookup condicional ou novo carrier refuta a inferência.

### Decisão proposta

`make_stdlib_with_features` importa a factory crate-private
`make_format_module` pela fachada e registra incondicionalmente:

```rust
scope.define("format", Value::Module(make_format_module()));
```

O mesmo registro ocorre em toda construção real da stdlib, independentemente
de target, `html` e `a11y-extras`. Não criar binding global `global`, `svg`,
`png`, `pdf` ou novo `html`; não registrar filhos, aliases, placeholders ou
valores marcadores. A ordem interna de inserção não é contrato, desde que os
bindings existentes permaneçam semanticamente preservados.

O nome e o scope vazio pertencem ao owner
`compiler/stdlib/format.md`; esta emenda legitima somente a ligação ao scope
global. Aceitação exige `format` e `std.format` como o mesmo módulo nominal,
alias estável, diagnóstico genérico de field ausente e ausência de todos os
filhos. Código, testes e header de linhagem aguardam confirmação humana
posterior ao commit causal P1461.

## P1464 — composição única de `pdf` e projeção em `format.pdf` (PROPOSTO)

### Medição anterior à decisão

No baseline `a1286f958f15a539dca17a3696ef83ef9e5b1b78`,
`make_stdlib_with_features` registra `format` antes de construir e registrar o
módulo global `pdf`. Por isso a factory de `format` não recebe a instância PDF
efetiva e `format.pdf` permanece ausente. `make_pdf_module(features)` já aplica
o gate `A11yExtras` e devolve todos e somente os bindings produtivos que P1464
deve reutilizar.

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`format.pdf.attach == pdf.attach` e `format.pdf.artifact == pdf.artifact`; com
`a11y-extras`, as três igualdades adicionais também são verdadeiras. É
inferência que construir uma única instância efetiva e clonar seus valores de
binding preserva essa identidade linguística. Necessidade de duplicar factory,
inferir feature por nome ou alterar `Value`, `Module`, `Func` ou `Scope`
refutaria a inferência e bloquearia o recorte.

### Decisão de composição

`make_stdlib_with_features` chama `make_pdf_module(features)` exatamente uma
vez. Mantém o `Value` resultante localmente, entrega por referência essa mesma
projeção efetiva a `make_format_module` e registra:

```text
format = módulo format que contém a função format.pdf projetada
pdf    = o mesmo valor de módulo PDF efetivo já construído
```

Clonar o carrier imutável para inserção no scope é permitido; reconstruir o
módulo, suas funções ou catálogo é proibido. A ordem interna das definições é
mecânica desde que ambos os bindings recebam a mesma projeção e todos os
bindings anteriores permaneçam semanticamente estáveis. O owner
`compiler/stdlib/format.md` decide os defaults e o namespace da função; este
owner decide somente a construção única e a ligação no scope global.

O conjunto efetivo de features continua vindo exclusivamente do argumento
`features`. `html`, target e exporter não ativam `a11y-extras`; sem a feature,
o trio não aparece nem no módulo global nem na projeção. Não criar aliases
globais novos, wrappers, stubs, segunda chamada a `make_pdf_module` ou lógica
de exportação.

Aceitação exige identidade pública dos cinco pares, os quatro perfis
`default`/`a11y-extras`/`html`/`html+a11y`, módulo global `pdf` inalterado e
`format.html` ausente. Esta mudança pública aguarda confirmação humana
posterior ao commit causal P1464; antes dela não há autorização para código ou
teste RED.
