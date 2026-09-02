# Prompt L0 — `compiler/eval/bindings/field_access` — acesso a campo sobre valores e `Content`
Hash do Código: d37b05cb

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/field_access.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0026 (`Content` como enum fechado)

---

## Contexto

Este nó resolve `a.b` como **leitura**: campo de dicionário, campo sintético de
um tipo primitivo (`.days` de uma duração, `.major` de uma versão, `.em` de um
comprimento), campo de `Content`, e os métodos de `Content`. É o caminho de
r-value, simétrico ao l-value de `bindings/access.md`.

No vanilla isto está distribuído por `foundations::value::field()` e
`Content::field()`; não há um ficheiro correspondente em `typst-eval`.

`eval_field_access` é a função de maior churn de todo o `bindings.rs`: 20
commits com mudança de corpo (critério 3), contra 8 do segundo colocado. É o
ponto onde cada tipo novo da língua ganha os seus campos.

## Restrições Estruturais

- L1 puro. `eval_value_field_access`, `eval_content_method`,
  `field_callee_error` e os helpers de `Content` **não recebem contexto** —
  operam sobre `Value`/`Content` já avaliados. Só `eval_field_access` recebe
  `Scopes`/`EvalContext`/`Engine`, para avaliar o alvo.
- `Content` é enum fechado (ADR-0026): o acesso a campo é `match` exaustivo,
  nunca reflexão.

## Instrução

### `eval_field_access(access, scopes, ctx, engine)`

Avalia o alvo e delega em `eval_value_field_access`. Antes de falhar, consulta
`field_callee_error` para produzir a mensagem certa quando o alvo é algo que
deveria ter sido chamado.

### `eval_value_field_access(target, field, span)`

| Tipo do alvo | Campos |
|---|---|
| `Dict` | a chave; erro `dictionary does not contain key "…"` se ausente |
| `Module` | binding exportado do módulo |
| `Content` | ver `content_field` |
| `Symbol`, `Func`, `Type` | campos/variantes do próprio tipo |
| `Duration` | `days`, `hours`, `minutes`, `seconds` |
| `Version` | `major`, `minor`, `patch` |
| `Length` | `abs`, `em` |
| `Relative` | `length`, `ratio` |
| `Args` | `positional`, `named` |
| `Stroke` | `paint`, `thickness` |
| `Point`/dimensões | `x`, `y` |

### `content_field(content, field) -> ContentField`

`match` exaustivo sobre o `Content` (ADR-0026) que devolve o campo pedido —
`body`, `text`, `level`, `depth`, `lang`, `outlined`, `bookmarked`, `delta`,
`first`/`last`, `len`, … — ou a indicação de campo inexistente. O enum
`ContentField` distingue "campo presente com valor" de "campo não existe neste
elemento", para a mensagem de erro ser exacta.

`content_set_fields` cobre os campos definidos por set-rule;
`content_elem_func` devolve a função do elemento (`.func()`);
`content_func_not_callable` produz o erro de callee.

### `eval_content_method(content, method, args, span)`

`func`, `has`, `at`, `fields`, `location` — os métodos de leitura de `Content`.
Usa `expect_positional`/`finish_args` de `super::method_dispatch`.

### `element_or_type_with_name` e `field_callee_error`

Produzem as mensagens que nomeiam correctamente elemento vs tipo, incluindo o
caso em que o alvo é `Symbol`/`Func`/`Type`/`Module` (nesses, `None` — não é
erro de campo).

## Critérios de Verificação

```
#let d = (a: 1); d.a                        → 1
#let d = (a: 1); d.b                        → Err "dictionary does not contain key \"b\""
#(1pt).em                                   → 0em
#duration(days: 2).days                     → 2
#version(1, 2, 3).major                     → 1
#[= T].level                                → 1
#[*x*].body                                 → conteúdo de x
#[= T].func()                               → heading
#[= T].has("level")                         → true
#[= T].fields()                             → dicionário com level e body
#(1).foo                                    → Err (nomeia "integer")
```

## P1291.cancel-angle-runtime — `text.size` contextual

### Medição anterior à decisão

`eval_field_access` já reconhece `Value::Func("text")` como fronteira de
propriedades contextuais e lê `text.lang` da `StyleChain` ativa. Quando a
callback selada de `math.cancel(angle:)` executa com o snapshot correto,
`context text.size` chega ao mesmo braço, mas cai no acesso comum de `Func` e
falha com `cannot access fields on type function`. A falha está no catálogo
fechado deste consumer, não no transporte de estilo nem no layouter.

### Decisão

No braço existente `text.<campo>`, `size` devolve `Value::Length` a partir do
accessor tipado `engine.styles.size()`. Esse accessor observa primeiro o slot
`StyleDelta.size` do topo — inclusive o `TextStyle.size` math derivado que o
transcript empurra —, depois o custom lexical compatível e, na ausência de
ambos, o default vigente `11pt`. Ler somente `custom("text.size")` é proibido,
pois perde redução de script/cramped. `lang` permanece inalterado e qualquer
outro campo continua no despacho fechado normal, sem namespace fabricado ou
fallback reflexivo.

Este consumer apenas lê o contexto quando a expressão contextual é executada.
Não clona nem altera `Func`, não executa callback, não conhece requests math e
não move avaliação para layout. Aceitação: `context text.size` observa um
`#set text(size: 19pt)` capturado; dentro de script observa o tamanho math
reduzido do snapshot (por exemplo `20pt × 0,7 = 14pt`); ausência observa
`11pt`; campo desconhecido mantém o erro vigente.

## Resultado Esperado

- `eval_field_access`, `eval_value_field_access`, `eval_content_method`,
  `field_callee_error` visíveis em `eval`; `ContentField` e os helpers de
  `Content` privados ao nó.

## P1146 — fields do valor-tipo `datetime`

Medição nos dois binários do vanilla ratificado: `datetime.day(d)` funciona e
`type(datetime.day) == function`, enquanto `d.day` falha com
`cannot access fields on type datetime`. Portanto, o braço fechado
`Value::Type(Type::Datetime)` delega os fields ao owner
`foundations::datetime_type_field`; não se cria field access nem intercepção de
método para `Value::Datetime`. Field desconhecido mantém o erro do valor-tipo.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — preservação de namespaces/fields

Converter os sete bindings para `Value::Type` não autoriza perder fields
públicos. Todo field já observado em algum binding anterior deve ser resolvido
por braço explícito `(Type::<Kind>, field)` e devolver a mesma função/constante
que o namespace anterior. Field inexistente mantém o erro vigente. Não se usa
mapa reflexivo nem fallback genérico; a cobertura é estática e exaustivamente
testada por kind.

## P1140.4-C — campos mínimos da equação passada a `supplement`

### Medição antes da decisão

No vanilla pinado, a callback `supplement: it =>
[#repr(type(it))|#repr(it.block)|#it.body]` produziu
`content|true|x + y`. O teste E2E cristalino chegou à callback, mas falhou em
`field_access.rs:105` com `equation does not have field "block"`;
`content_field` (`field_access.rs:430-470`) não tem braço Equation.

### Decisão

`content_field` ganha braço explícito para `Content::Equation`: `block` devolve
`Value::Bool` e `body` devolve `Value::Content`. `content_set_fields` enumera
`block`, `body` nessa ordem. Não se cria fallback reflexivo nem se expõem os
campos ainda não materializados por valores inventados.

## P1140.5-A — acesso ao campo `alt` em equações styled

### Medição antes da decisão

Vanilla `fields()` medido inclui `alt: "description"`, `alt: none` e
`alt: ""` quando explícitos; o campo aparece antes de `body`. O cristalino
transportará `alt` na style chain, enquanto `EquationElem` permanece mínimo.

### Decisão

O acesso a `Content::Styled` reconhece descendente Equation e resolve o delta
top-wins `equation.alt`: `Str`/`None` tornam-se campo set; ausência mantém o
estado não explícito/default. `block` e `body` continuam delegados ao elemento.
`content_set_fields` preserva ordem pública e não assa `alt` em `plain_text`.

## P1147 — fields do valor-tipo `int`

O braço `Type::Int` preserva `min`/`max` e delega os outros nove fields ao
owner `foundations::int_type_field`. A fonte e as sondas ratificadas medem
formas estática e de instância para funções com `self`; `from-bytes` permanece
somente estática. Field desconhecido mantém o erro do valor-tipo.

## P1148 — fields do valor-tipo `counter`

`Value::Type(Type::Counter)` delega os seis fields públicos ao owner
`counter_type_field`. O field access só descobre a função; contexto,
introspecção e semântica permanecem no owner/dispatch existente.

## P1150 — fields do valor-tipo `content`

Sondas coincidentes nos dois binários ratificados confirmam cinco functions:
`func`, `has`, `at`, `fields`, `location`. `content_type_field` expõe wrappers
não ligados que recebem `Value::Content` primeiro e delegam ao mesmo
`eval_content_method` usado pelos métodos de instância.

Medição com `strong[Hi]` confirma igualdade de `fields`, `has`, `at`,
`default:`, `func` e `location`; `content.func(strong[Hi]) == strong` é true e
location inline é `none`. Match fechado, sem duplicar `content_field` ou
`content_set_fields`. É glue interno de paridade, fluxo contínuo ADR-0127.

## P1151 — `content.location()` para conteúdo introspectado (GATE ADR-0127)

### Medição antes da decisão

Os dois binários vanilla ratificados devolvem Location presente para conteúdo
vindo de `query`, `none` para conteúdo inline e Locations distintas para duas
headings de morfologia idêntica. Igualdade, `repr` e `fields` não incorporam a
Location. No cristalino P1150, o método só recebe `&Content` e retorna sempre
`none`; a Location já foi descartada por `native_query`.

### Decisão condicionada ao gate

O despacho de métodos deve aceitar tanto conteúdo declarativo sem metadado
quanto o valor locatável de `entities/value.md` P1151. `func`, `has`, `at` e
`fields` delegam ao mesmo `Content`; `location` devolve `Value::Location(loc)`
somente no segundo caso e `Value::None` no primeiro. As formas estática e de
instância permanecem equivalentes. Não consultar o introspector por igualdade,
não mover a decisão para layout e não adicionar Location a cada elemento.

A mudança depende da nova representação pública em `Value`; parar antes do
código conforme ADR-0127.

## P1161 — modifiers sobre valor multi-codepoint (GATE ADR-0127)

Medição vanilla: `emoji.heart.arrow` → `💘`, `emoji.heart.excl` → `❣️` e
`emoji.heart.nope` falha com `unknown symbol modifier` no span de `nope`.
`eval_value_field_access` continua a delegar em `Symbol::modified`; o valor
selecionado passa a ser o `EcoString` integral da variant, sem alterar a
mensagem ou o span. Não duplicar seleção de variants neste nó.

## P1284 — catálogo fechado de valores-tipo, constantes e wrappers

### Medição antes da decisão

`C-P1284-v2` mede 180 paths residuais e exige que membros de instância sejam
também projetados como funções não ligadas no valor-tipo. A representação
cristalina pré-candidata já possui `Value`/`Type`/entidades suficientes para o
lote contínuo, exceto `selector.before/after`, `color.spot/tint` e
`outline.entry/*`, bloqueados por ADR-0127.

### Decisão

`eval_value_field_access(Value::Type(t), field, span)` mantém um match fechado
e delega a descoberta ao owner semântico. Este nó possui somente o glue de
lookup e a classificação do valor devolvido; não duplica callbacks, unidades,
introspecção, fórmulas de cor nem mutação.

| Valor-tipo | Fields autorizados em P1284 | Owner semântico / kind |
|---|---|---|
| `array` | 32 métodos de instância + `range` (33 fields) de `stdlib/collections` | `function` |
| `dictionary` | os 9 métodos de `stdlib/collections` | `function` |
| `str` | 20 métodos de instância, inclusive `to-unicode`, + `from-unicode` (21 fields) | `function`; conversões no owner `foundations/str` |
| `bytes` | `at`, `len`, `slice` | `function`, owner `stdlib/collections` |
| `arguments` | `at`, `filter`, `len`, `map`, `named`, `pos` | `function`, owner `stdlib/collections` |
| `color` | catálogo vigente + `map` | `map` é `module`, delegado a `stdlib/color` |
| `direction` | `axis`, `end`, `inv`, `sign`, `start`, `from`, `to`; `btt/ltr/rtl/ttb` | funções; quatro constantes `direction` |
| `alignment` | `axis`, `inv`; `bottom/center/end/horizon/left/right/start/top` | funções; oito constantes `alignment` |
| `duration` | `days`, `hours`, `minutes`, `seconds`, `weeks` | `function` |
| `length` | `cm`, `inches`, `mm`, `pt`, `to-absolute` | `function` |
| `selector` | `and`, `or`, `within` | `function`, owner de orchestration `value_methods` |
| `state` | `at`, `final`, `get`, `update` | `function`, owner `stdlib/state` |
| `location` | `page`, `page-numbering`, `position` | `function`, owner `call_dispatch` |

As constantes qualificadas reutilizam o mesmo `Value` do binding global:
`direction.rtl == rtl`, `alignment.left == left`, etc. `axis` devolve
`"horizontal"`/`"vertical"` ou `none` para alinhamento 2D; `inv`,
`start/end`, `sign`, `from/to` seguem as tabelas fechadas das entidades
existentes. Nenhuma constante é embrulhada como função.

Cada wrapper de instância recebe `self` como primeiro positional e encaminha
à mesma função que a forma ligada. Nome, `repr`, ordem, required/named,
variadic, settable e default são os do owner. Field desconhecido mantém a
mensagem `type <nome> does not contain field <field>`; não há fallback
reflexivo.

### Gates negativos

- `selector.before` e `selector.after` permanecem ausentes:
  `BLOCKED_ADR0127_PUBLIC_CONTRACT`; não mapear para `Within`/`And`/`Or`.
- `color.spot`, `color.spot.tint` e `outline.entry/*` permanecem bloqueados;
  não fabricar `dict`, `none`, stub ou módulo aproximado.
- Este L0 não autoriza campo, variante, trait, assinatura Rust pública, default
  de produto ou mudança de fase. Se um wrapper não couber no glue interno,
  parar no ADR-0127.

### Verificação

Para todo field: existência, kind, `repr`, metadata, forma ligada/não ligada,
chamada real, defaults e erros. Sentinelas: `array.len((1,2,3)) == 3`,
`arguments.len(arguments(1,x:2)) == 2`, `direction.rtl == rtl`,
`alignment.left == left`, `type(color.map) == module`. A mera resolução do
nome sem chamada real é insuficiente.

## P1289 — descoberta fechada de `float.is-infinite`

### Medição anterior à decisão

O vanilla pinado devolve `(function, "is-infinite")` para o field do valor-tipo;
o cristalino pré-candidato falha com
`type float does not contain field "is-infinite"`. Chamadas ligadas existem,
mas obter `(1.0).is-infinite` como valor continua a falhar no vanilla com
`cannot access fields on type float`.

### Decisão

O braço `Value::Type(Type::Float)` delega a descoberta ao owner
`foundations::float_type_field`. Este nó não contém a fórmula, não cria
reflexão para `Value::Float` e não intercepta a chamada ligada. Field
desconhecido mantém `type float does not contain field "<field>"`.

## P1293.reopen-A — span do campo ligado `is-nan` sem chamada

### Medição anterior à decisão

Em `2026-09-01T15:13:15-03:00`, sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitida, o
recibo segregado `p1293-implementation-receipt-a.md` de SHA-256
`14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`
mediu `float("NaN").is-nan` com a mensagem pública correta, mas range
cristalino `0..19` contra `13..19` no vanilla ratificado e no contrato.

No consumer vigente e ainda sem patch desta reabertura,
`field_access.rs:106` entrega `access.span()` ao lookup comum e
`field_access.rs:441-446` reutiliza esse span total no diagnóstico. A própria
AST já expõe `access.field().span()` — precedente local em
`field_access.rs:99-101` —, que corresponde exatamente ao identificador
`is-nan` medido. Esse erro não atravessa a nativa nem `Args`; portanto este nó
é o owner causal da quinta âncora.

### Classificação e decisão

O span publicado pelo diagnóstico é linguagem sob ADR-0107. A intenção P1293
já exige o range exato; trocar somente a âncora interna é correção de paridade
em fluxo contínuo ADR-0127, sem contrato Rust público, default ou mudança de
fase.

Quando `eval_field_access` recebe exatamente `Value::Float` e o campo
`is-nan` como acesso ligado sem chamada, o erro vigente
`cannot access fields on type float` deve usar `access.field().span()`, nunca
o span total do acesso. Mensagem, severidade, hints e ausência do valor ligado
permanecem idênticos. Todos os outros targets, fields e erros conservam a
âncora atual; não generalizar esta regra, não criar reflexão e não fabricar um
método como valor.

É proibido alterar `entities::Args`, API pública, entidade, default, ordem de
avaliação ou fase. Este Prompt continua proprietário 1:1 apenas de
`01_core/src/compiler/eval/bindings/field_access.rs`; `call_dispatch` possui as
quatro âncoras de chamadas e `stdlib/foundations/float` possui a função.
