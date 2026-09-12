# Prompt L0 — `compiler/eval` — dispatcher e contexto
Hash do Código: e1b27745

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
`1f082370e59939de7b57992e137a9f74bfb6758f`, a matriz bilateral
`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json` (SHA-256
`1678b1aed89bfff7581a8fdd998a32f57df18f134226e680596c587620ea3efe`)
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
`model/table.rs:289-305,494-613,732-733` declara nomes curtos. O recibo P1293
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

A medição independente `00_nucleo/diagnosticos/p1305-r2-pre-measurement.json`,
SHA-256 `fd6354824e500e61f18b14116dd54b4f4838691289226a7f7e04236258dd9da0`,
registra HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b` mais diff P1303
não commitado, status/diff/stat, fixtures e saídas completas entre
`2026-09-07T13:48:28.580684+00:00` e
`2026-09-07T13:51:19.266604+00:00`. O baseline fresco tem SHA-256
`4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d`;
o vanilla, `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
`named-modules`, `named-aliases` e `document-route` medem `module(std)`
contra `<module global>`, preservando lookup. A contraprova de import
`std.typ` com reexport integral impede corrigir isso somente no formatter.
O dono respondeu `Autorizado` à ampliação de construção neste owner e em
`compiler/eval/modules`; `p1305-r2-reopening.md` registra essa exceção.

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

`01_core/src/compiler/eval/mod.rs:1723-1743`, no snapshot R3 SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306
integral ali capturado, registra json/yaml/toml sem namespace e CBOR com
encode. Os recibos P1307 SHA-256
`f1191d3de029dabf6f66f87872bee8146aa43106ae54e84facc22952afd238d0`
e R2 SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`
medem os membros originais com kind function/repr encode, mantendo os pais
chamáveis. O vanilla é upstream ratificado `a51e02804`; cada recibo conserva
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

Baseline `p1308-baseline.json` SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`
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

### Não convergência

A história é a sequência real de snapshots do orquestrador. O diagnóstico usa
as requisições da tentativa final projetadas sobre essa sequência, sem afirmar
que o corpo as executou em todas as rodadas. Não fabrica detalhe para consulta
opaca ou para leitura alheia.

Não há neste owner obrigação de ledger, DTO, JSON, hook ou harness de
instrumentação. Observação adicional exige owner próprio.
