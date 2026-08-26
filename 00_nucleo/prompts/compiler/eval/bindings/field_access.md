# Prompt L0 — `compiler/eval/bindings/field_access` — acesso a campo sobre valores e `Content`
Hash do Código: 6f7ae61e

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
