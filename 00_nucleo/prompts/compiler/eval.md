# Prompt L0 — `compiler/eval` — dispatcher e contexto
Hash do Código: c8173f60

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

## P1339 — observações contextuais seletivas (interface pública aprovada)

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumers intactos.
`eval/mod.rs:124-240` não possui registro de leituras contextuais. O snapshot
em `:148-155` é read-only no eval. `pipeline.rs:244-278` já conserva acesso
ao EvalContext depois de apply_func, inclusive quando este devolve Err;
o retorno atual não identifica se o bloco chegou a consultar um contador
filtrado. O recibo `diagnosticos/p1339-context-dependency-probe-runs.json`,
SHA-256 `4da38b499916ed3d5946bb16c904f2eaa268a4ec2f788cc0149927f0d83260d1`,
registra estado, UTC e compilação bilateral: update criado em context ainda
não existe no snapshot do assert dependente, que aborta antes da reintrospecção.

`Func::PartialEq/Hash` usa Arc para closures (`entities/func.rs:384-415`),
enquanto Debug omite sua captura (`:369-372`) e `hash_content` usa Debug
(`entities/content_hash.rs:25-29`). Nenhum deles prova estabilidade de uma
closure recriada. O desenho independente
`diagnosticos/p1339-stabilization-design.md` também refuta usar apenas o
resultado de uma leitura Element: outra consulta pode alterar a captura.

A fonte ratificada `introspection/convergence.rs:71-116,267-278` valida o
valor efetivamente observado, distinguindo inclusive tipo, não a igualdade
da linguagem dos argumentos da consulta. No recibo
`diagnosticos/p1339-stabilization-boundaries-runs.json`, SHA-256
`4d82648d895ec5dba23a9775c2bf09dfae84a175603f3ce6d742c8c7f2a09241`,
NaN na chave produz leitura estável zero; closure contextual nova produz
12 sem aviso; oscilação/crescimento terminam com valores e warnings de
não convergência, não erro universal. Proveniência integral no recibo.

### Decisão de desenho — validar entradas observadas, não comparar closures

A estabilização seletiva foi autorizada em
`diagnosticos/p1339-stabilization-approval.json`. Sua realização exige
registro **privado**, local a cada EvalContext, independente do introspector
e inicializado vazio por new. O snapshot e seus stores não são alterados por
uma leitura. Nada global, nenhum registry reflexivo, objeto dyn novo ou
metadata escondida em FlowEvent, campos de bibliografia, warnings ou strings.

O registro contém uma sequência fechada e tipada de observações das operações
introspectivas efetivamente executadas. O bit de seleção é marcado somente
por demanda de counter cuja árvore de Selector contenha Element; construir
counter/update, descobrir método ou atravessar ramo não executado não marca.
Registrar a demanda antes de resolver Location/label ou executar callback,
para que a dependência sobreviva ao Err da leitura. Cada bloco usa seu próprio
contexto; a marca de um irmão não contamina outro bloco.

Uma vez selecionado o bloco, **todas** as suas entradas introspectivas são
necessárias à validação, inclusive as lidas antes da primeira demanda Element.
Portanto o registro é causal desde o começo da avaliação contextual, não
começa retroativamente na primeira demanda filtrada. Operação, argumentos
efetivos, Location, span, StyleChain pertinente e resultado/erro observados
ficam no registro; não reconstruir por AST, nome, equality de Value ou texto
do diagnóstico. Os producers sem leitura filtrada não ganham nova semântica.

Escolher validação contra o snapshot candidato das requisições registradas,
reutilizando os owners semânticos das leituras, em vez de introduzir igualdade
estrutural geral de Func/Content. Capturas, código e World da avaliação
original permanecem os mesmos; se alguma entrada observada mudou, reavaliar
o bloco. Se todas permanecem válidas, conservar seu resultado original,
inclusive closures nele contidas, sem comparar endereço de novas closures.
Esta inferência pressupõe registro completo e determinismo dos inputs; leitura
não registrada, serviço opaco ou input fora do snapshot refutam a suficiência.
Incompletude nunca devolve validação positiva.

Comparar resultados com sua forma/tipo observáveis, não só values_eq; o teste
de estabilidade precisa ser reflexivo para uma observação idêntica, inclusive
request contendo NaN. A associação de updates ao contador continua usando
igualdade da linguagem, separada deste teste. Fold completo demandado e seu
erro posterior participam da observação, mesmo quando get devolve prefixo.

### Fronteira pública aprovada — única neste owner

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

O getter é puro. Os outros métodos revalidam somente requisições realmente
observadas, com recursos explícitos, delegando aos owners; não executam o corpo
do ContextBlock nem callbacks de chaves nunca demandadas. Engine de validação
é transacional e separado da avaliação produtiva; não publicar seus warnings
incidentalmente, duplicar updates ou modificar o snapshot fornecido. Recursos
lexicais/estilos das requisições são os capturados, não defaults inventados.

Validação positiva exige todas as observações completas e preservadas perante
candidate. Observação alterada, inclusive sucesso que vira erro ou vice-versa,
produz false para que o corpo seja reavaliado; falha do próprio mecanismo de
validação propaga SourceDiagnostic, nunca true. Um erro igual e persistente
da leitura pode ser validado como estável, mas o Err do corpo continua erro:
true não significa que o bloco ou documento teve sucesso.

Diagnose recebe a história de snapshots efetivamente usada pelo orquestrador,
em ordem, sem fabricar passagens nem observar contador alheio. Devolve apenas
os diagnósticos próprios de não convergência medidos, com span e histórico da
consulta; não decide exportação, não transforma erros em avisos e não declara
convergência por ter esgotado o teto. O formato da CLI continua no seu owner.

Não acrescentar campos públicos, variantes públicas ou trait nesta proposta.
As assinaturas existentes ficam. Entretanto a inclusão de armazenamento
privado em EvalContext pode quebrar sua construção por literal fora do crate:
o baseline tem somente campos públicos. A busca dirigida nas quatro camadas
encontrou definição/construtor, não initializer externo; isso não prova ausência
de consumidores externos. **Os três métodos públicos e essa restrição de
compatibilidade foram aprovados pelo dono**, conforme
`diagnosticos/p1339-observation-interface-approval.json`. Essa aprovação
específica sucede a autorização de estabilização; não dispensa os gates de
integração, contrato, selo e RED anteriores ao código.

### Integração dos owners e limite de completude

O recibo `diagnosticos/p1339-observation-integration-receipt.json` fixa a
auditoria dos acessos ao snapshot nos owners de eval/stdlib. As obrigações
de registro/replay ficam individualizadas em stdlib/counter, stdlib/state,
stdlib/foundations/query, eval/call_dispatch (Location) e
eval/bindings/value_methods (resolução de label antes da leitura de state).
O armazenamento e o despacho fechado de revalidação pertencem a este owner;
os helpers internos delegam a semântica aos owners de cada operação. Não
introduzir um novo módulo sem proprietário nem funções públicas adicionais.

Observar o retorno do produtor da entrada, não reexecutar consumidores desse
retorno. Assim query registra carriers completos, state.display registra o
valor antes da callback e Location registra a projeção efetivamente pedida.
Callbacks de display executadas no corpo usam o mesmo registro causal. O
replay do fold CounterUpdate::Func é distinto: pertence à própria leitura
de counter e continua obrigatório, com contexto isolado conforme from_tags.

Medição adicional no HEAD acima: `entities/engine.rs:43-57` fornece World e
StyleChain por referência; `pipeline.rs:244-268` cria ctx/Engine por bloco e
clona sua chain; `call_dispatch.rs:1416-1419,1476-1504` usa estilos/métricas
em measure/layout; `field_access.rs:486-500` lê text.size/lang. Essas entradas
não vêm de TagIntrospector. Sua preservação não pode ser demonstrada pelo
argumento candidate dos métodos aprovados.

Portanto a revalidação pressupõe a mesma instância lógica de bloco/captura,
Location contextual, chain de entrada, target/features e recursos de Engine
da tentativa original. O orquestrador só reaproveita um registro enquanto
mantém esses inputs por construção; se substitui o produtor/captura/chain,
descarta o registro descendente e avalia novamente. Não considerar estilos
imutáveis durante todo o corpo: Engine.styles é mutável, e uma requisição
captura a chain do ponto efetivo em que foi executada. Replay não usa a chain
final do corpo como substituta. Não alterar os accessors de estilo nem as
fases measure/layout incidentalmente. Mudança/opacidade de World ou recursos
não coberta pelo contrato impede certificar a tentativa.

Este contrato não afirma já existir rastreio completo. Antes do selo, fixar
a cobertura do comparador de resultados, inclusive Func/Content/valores
opacos provenientes de query/state/page-numbering, e demonstrar as
precondições de preservação dos inputs externos ao snapshot. A auditoria de
acessos diretos não é prova dessa cobertura. Não materializar um predicado
que retorna true ignorando categoria não integrada. Registro, replay,
diagnóstico e teste de completude são obrigações do mesmo P1339, não
sucessores implicitamente considerados prontos.

Gates: erro antes/depois da demanda; irmão legado; leitura não Element antes
de Element; captura alterada sem alteração de páginas; NaN; get anterior a
callback inválida; callback nunca demandada; tentativas com warnings; aliases
e chamadas estáticas/ligadas; validado-com-erro não vira sucesso. Segregação,
contrato, selo e RED continuam anteriores à implementação.

### P1339 — comparação fechada e retenção causal

Medição anterior à decisão: `entities/content.rs:3495-3500` deixa Metadata,
State e StateUpdate fora de PartialEq, portanto nem reflexividade existe
para todas as entradas; `entities/value.rs:26-44` conserva Content e fields
separadamente. `entities/func.rs:65-75,384-395` conserva captura eager mas
usa identidade para closures na igualdade atual. O diagnóstico
`diagnosticos/p1339-observation-design-resolution.md`, com hashes/UTC e
fontes desse HEAD, refuta tanto usar PartialEq/Debug como teste geral quanto
substituir identidade observável por igualdade estrutural de closures novas.

O comparador privado de observações distingue Same, Different e Unproven.
Não modifica igualdade/hash públicos de Value, Func, Content ou CounterKey.
Same é prova suficiente para reutilizar; Different invalida; Unproven não
certifica estabilidade nem se converte em warning vanilla de oscilação.

- Folhas fechadas comparam variante e todos os dados observáveis. Float e
  folhas geométricas usam bits IEEE, preservando sinal de zero e tornando
  reflexiva uma observação com o mesmo NaN. Isso não muda associação de
  updates, que continua com igualdade de linguagem não reflexiva para NaN.
- Arrays, dicts, Args, Selector, State e Counter percorrem conteúdo/ordem,
  presença e metadata causal que afete diagnóstico. LocatedContent inclui
  Location, carrier Some/None, fields completos e Content, sem canonização
  morfológica nem preenchimento de defaults.
- Content fechado percorre seus variants e dados, filhos, estilos e callbacks.
  Igualdade pronta só pode ser usada em folhas auditadas sem Value/Func/floats
  ocultos. A desestruturação explícita e o match exaustivo impedem um novo
  campo/variant silenciosamente aceito. Retenção da MESMA instância imutável
  é atalho suficiente apenas quando sua geração causal foi preservada.
- Func/Module retidos pelo mesmo produtor imutável preservam sua identidade.
  Nativas conservam variante, executável, nome e namespace; With conserva
  função e Args. Função recriada não é declarada idêntica só por body/capturas
  iguais. Não executar função para decidir sua categoria ou comparar saída.
  Um produtor selecionado cujas leituras continuam válidas conserva a saída
  original; não recriar sua closure durante a validação.
- Resultado e diagnóstico integral da operação distinguem sucesso/erro.
  Mesmo erro pode ser estável sem fazer o corpo passar. Traces posteriores
  da chamada permanecem no erro retido do corpo, sem reconstrução textual.

Serviços opacos exigem preservação de identidade/imutabilidade causal, não
apenas dyn_eq, nome ou endereço de um estado mutável. O pipeline CLI injeta
ElementRegistry vazio (`pipeline.rs:140-142`), e o caminho de Element/Dynamic
externo depende desse registro (`eval/mod.rs:653-658`). Esse acesso por
extensão Rust não é promovido à equivalência pelo P1339. Plugin específico
permanece a fronteira opcional explicitada na sonda; se um caso obrigatório
real alcançar Unproven, o selo é bloqueado em vez de excluir o caso ou
inventar erro de produto. O contrato deve exercer opacidade deliberada
separadamente dos casos que exigem Preserved.

A história usada por context_nonconvergence_diagnostics é a sequência
[I0,...,Ik] do orquestrador. Usar somente as requisições efetivas da tentativa
final com seus próprios argumentos/capturas/estilos; projetá-las sobre os
snapshots anteriores não significa que o corpo as executou em cada rodada.
Comparar as duas últimas projeções decide os detalhes de não convergência;
não fabricar detalhe para consulta opaca ou só porque outra leitura mudou.
I0 corresponde a run 1; no teto I4 corresponde a run 5 e I5 a final.

## P1341 — projeções tipadas para o ledger de teste

### Medição anterior à decisão

No baseline P1341, a projeção privada de `Value::Dict` era um objeto JSON e o
registro de callbacks conservava apenas fase e espécie. Essa forma perde ordem,
não distingue identidade/produção das funções e permitiu omissões coerentes no
ataque materializado R2. O pré-selo P1341 R5 exige lista ordenada de pares e
binding causal verificável, sem IDs runtime congelados no oráculo.

### Decisão

Somente sob `cfg(p1339_observation)`, projetar todo valor em variante fechada e
tipada. `Dict` é `{"Dict": [[chave, valor-tipado], ...]}` na ordem real de
iteração, recursivamente; não converter em mapa JSON, ordenar ou deduplicar.
Callbacks registram o `Func` efetivamente despachado, carrier/origem e fase no
ponto da chamada, usando identidades runtime apenas dentro da célula. O ledger
verifica domínios e não-alias entre runtime/func/carrier/value/location/snapshot;
essas identidades são evidência de continuidade local, nunca expectativas ex
ante nem nova igualdade pública.

O schema de observação é fechado e fail-closed, inclusive para tipos inválidos;
`bool` não satisfaz campo inteiro. `Unknown` é posterior a todos os predicados.
Não mudar `Value`, `Func`, `EvalContext` público, avaliação, igualdade ou output
normal; nenhum callback é repetido para observar.

## P1342 — carrier de execução e produção real de Dict

### Medição anterior à decisão

`diagnosticos/p1342-topology-audit-r1.md` mede `EvalContext` como o contexto
comum do dispatch, da closure e do produtor `Expr::Dict`; o Dict nasce como
`IndexMap`, enquanto a projeção JSON final P1341 perde binding e ordem tipada.

### Decisão vigente

Sob `cfg(p1339_observation)`, `EvalContext` transporta opcionalmente o handle
do ledger P1342 e o id da ocorrência de callback ativa. `new()` começa sem
handle. Helpers internos instalam/clonam esse estado a partir da sessão ou do
carrier do `Func`; appends sem handle são no-op. Nenhuma API normal, trait,
igualdade, diagnóstico, fluxo ou resultado é alterado.

Na conclusão bem-sucedida do braço real `Expr::Dict`, depois de avaliar uma
vez os itens e antes de devolver `Value::Dict`, registrar o próprio valor:
pares na ordem do `IndexMap`, chaves/valores em projeção fechada e tipada,
span/kind do nó e ocorrência ativa. Não ordenar, deduplicar, converter primeiro
em objeto JSON ou reconstruir do output. O Dict pré-ligado e o Dict `witness`
são eventos distintos; o body escolhe causalmente o segundo, sem ordinal fixo.

Um `Expr::Contextual` criado dentro de execução instrumentada pode registrar
id e closure reais; o ContextBlock raiz preexistente é vinculado no ponto real
`Session::execute`, sem carrier novo no elemento. Esta cláusula revoga no
fragmento P1342 a construção post-hoc de callbacks/Dicts de P1341 e não cobre
a matriz lifecycle/profile ou a política geral de opacidade.
