# Prompt L0 — `compiler/eval/call_dispatch` — dispatch de chamadas de função
Hash do Código: cdeb74dd

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/call_dispatch.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/mod.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: dispatch de chamadas com intercepção de métodos especiais

---

## Contexto

Este nó contém o **dispatch de chamadas de função** do eval: dada uma expressão de chamada (`Expr::FuncCall`), avalia o callee, os argumentos, e aplica a função resultante. Inclui intercepções sintácticas para métodos especiais (`where`, `and`/`or`, `within`, mutating methods, métodos de coleção, `with`, `to-absolute`, `measure`, `layout`, métodos de `state`/`counter`/`color`/`version`/`args`/`content`, etc.) antes de cair no caminho genérico.

Extraído de `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

> **Fonte de paridade dos métodos especiais (P1031)** — cada método interceptado existe
> na linguagem Typst e está documentado. As referências abaixo são o doc comment `#[func]`
> do vanilla ratificado (`e0e8ca4d`), que é o texto publicado em `typst.app/docs`
> (`lab/typst-original/docs/README.md`: o conteúdo do site é *"supplemented by Typst markup
> in Rust doc comments throughout the codebase [...] those affected by the `#[elem]`, `#[ty]`,
> and `#[func]` macros"*).
>
> | Método | Doc comment do vanilla | Página publicada |
> |---|---|---|
> | `.where(..)` | `crates/typst-library/src/foundations/func.rs:412-422` | `reference/foundations/function/#definitions-where` |
> | `.or(..)` | `crates/typst-library/src/foundations/selector.rs:177-178` | `reference/foundations/selector/#definitions-or` |
> | `.and(..)` | `crates/typst-library/src/foundations/selector.rs:188-189` | `reference/foundations/selector/#definitions-and` |
> | `.within(..)` | `crates/typst-library/src/foundations/selector.rs:275-285` | `reference/foundations/selector/#definitions-within` |
> | `.with(..)` | `crates/typst-library/src/foundations/func.rs:390-397` | `reference/foundations/function/#definitions-with` |
> | `.to-absolute()` | `crates/typst-library/src/layout/length.rs:140-158` | `reference/layout/length/#definitions-to-absolute` |
> | `measure(..)` | `crates/typst-library/src/layout/measure.rs:47` | `reference/layout/measure` |
> | `layout(..)` | `crates/typst-library/src/layout/layout.rs:65` | `reference/layout/layout` |
> | `counter(..).at/step/update/display` | `crates/typst-library/src/introspection/counter.rs:383,456,490,506` | `reference/introspection/counter` |
> | `state(..).at/update` | `crates/typst-library/src/introspection/state.rs:274` e seguintes | `reference/introspection/state` |
>
> Guardas de não-regressão no cristalino: `01_core/src/compiler/eval/tests.rs:7192`
> (`p417_heading_where_retorna_selector`), `:7340` (`.and` em show rule), `:11543`
> (`p504_within_selector_value`), `:1497` (`p713_cetz_canvas_length_to_absolute_div_1cm`).
>
> **Natureza da citação**: literal para a *existência e assinatura* de cada método (o doc
> comment nomeia o método e os seus parâmetros); a **ordem** de intercepção sintáctica no
> cristalino (§2, item 1) **não** vem da documentação — é decisão de implementação, provada
> só pelas guardas acima.

---

## Instrução

### 1. Contrato público

```rust
pub fn apply_func(
    func: Func,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;

pub(super) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args>;

pub(super) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;
```

Funções privadas do nó:

- `trace_call` — envolve erros de uma chamada com `Tracepoint::Call` quando o span da chamada não contém o erro.
- `merge_with_args(pre: &Args, new: Args) -> Args` — funde args pré-ligados por `.with(...)` com os da chamada final.
- `call_plugin(p: &PluginFunc, args: &Args) -> SourceResult<Value>` — aplica um export de plugin WASM.
- `eval_location_method(loc, method, ctx) -> SourceResult<Value>` — resolve métodos de instância de `Value::Location`.

### 2. Comportamento

Manter exatamente o comportamento actual:

- `eval_func_call` avalia chamadas sintácticas, aplicando intercepções na ordem correcta (method calls especiais antes do caminho genérico).
- `apply_func` despacha sobre `FuncRepr`: `Closure` via `closures::apply_closure`, `Element` via ctor, `Plugin` via `call_plugin`, `With` via merge recursivo, `Native`/`NativeWithEngine` via fn-ptr.
- `eval_args` avalia argumentos posicionais/nomeados/spread, espelhando `ast::Args::eval` do vanilla.
- `trace_call` só adiciona trace quando o erro está fora do span da chamada.
- `call_plugin` rejeita named args e exige `Value::Bytes` posicionais.
- `eval_location_method` lê do `ctx.introspector`, com defaults seguros quando a posição ainda não está disponível.

### 3. Gatilhos de reabertura

- Novo `FuncRepr` ou novo tipo de função chamável.
- Nova intercepção de method call especial.
- Mudança de fase (eval ↔ layout) no processamento de chamadas.

## P1215 — metadados internos de span para collections

Na intercepção sintáctica de métodos de coleção, `eval_func_call` preserva,
antes de consumir os argumentos, o span da chamada inteira, de cada positional
e de cada named completo. Esses metadados são passados ao owner
`stdlib/collections` somente para escolher a âncora diagnóstica medida; não
entram em `entities::Args`, não mudam a ordem de avaliação e não constituem
API pública. Caminhos sintéticos/field access continuam com fallback seguro.

---

## Critérios de verificação

```
Dado #let f(x) = x + 1; f(2) → 3
Dado #let g = f.with(1); g() → 2
Dado #calc.max(1, 2) → 2
Dado plugin("foo")(bytes) → bytes de retorno
Dado #loc.page() → int (via introspector)
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — despacho fechado de `Value::Type`

No braço `Value::Type(t)`, o `match` deve delegar diretamente:

- `Decimal` → `native_decimal`; `Duration` → `native_duration`;
- `Regex` → `native_regex`; `Selector` → `native_selector`;
- `Stroke` → `native_stroke`; `Tiling` → `native_tiling`;
- `Version` → `native_version`.

Os `Args`, `EvalContext`, `World` e `FileId` recebidos são passados sem
normalização adicional, para preservar integralmente aridade, defaults,
mensagens e valores dos construtores já testados. Tipos não chamáveis mantêm
`type {name} does not have a constructor`. O `match` continua exaustivo e sem
despacho dinâmico.

## P1140.2 — medição e despacho de `Label`

Medido no vanilla: `label` é tipo chamável, `label("x") == <x>` e um segundo
posicional é erro. No cristalino anterior, o callee era `Value::Func` e
produzia `Content`. O braço fechado `Value::Type(t)` acrescenta
`Type::Label => native_label(ctx, &args, world, current_file)`. Nenhuma
intercepção sintáctica especial, registry ou normalização de args é criada.

## P1144 — métodos de instância de `Value::Gradient`

Medição no vanilla ratificado `a51e02804`
(`visualize/gradient.rs:719-874`) confirmou onze métodos públicos, disponíveis
em forma estática e de instância. No bloco já existente de dispatch por valor,
o braço fechado `Value::Gradient` só intercepta esses onze nomes e delega ao
owner `stdlib/gradients`, após avaliar os argumentos uma vez. O owner sintetiza
o gradiente como primeiro positional e usa as mesmas funções nativas da forma
estática. Nome desconhecido continua no caminho genérico de field access.

Esta intercepção é eval puro e não muda fase do pipeline. É glue interno de
paridade sobre domínio já nuclearizado; não altera o contrato público Rust nem
defaults (ADR-0127, fluxo contínuo).

## P1145 — `gradient.sharp` e `gradient.repeat`

A fonte ratificada `a51e02804:visualize/gradient.rs:545-716` acrescenta duas
transformações públicas anteriores aos onze membros inventariados em P1144.
Após aprovação do gate P1145, o braço fechado `Value::Gradient` também
interceptar `sharp` e `repeat`, avaliar os argumentos uma vez e delegar às
nativas homónimas em `stdlib/gradients`. A forma estática recebe `self` como
primeiro positional e usa a mesma implementação. Nomes desconhecidos continuam
no caminho genérico.

O dispatch em si é glue interno; a paragem ADR-0127 é causada pelo campo
público `anti_alias` necessário na entidade, não por mudança de fase neste nó.
O dono aprovou o gate antes da materialização.

## P1147 — métodos de instância de `Value::Int`

Sondas coincidentes nos dois binários ratificados confirmam forma estática e
de instância para `signum`, `bit-not`, `bit-and`, `bit-or`, `bit-xor`,
`bit-lshift`, `bit-rshift` e `to-bytes`. O braço fechado `Value::Int`
intercepta somente esses nomes, avalia args uma vez e delega ao owner com o
receiver como primeiro positional. `from-bytes` não é método de instância.
Isto é glue interno puro, sem mudança de fase ou contrato Rust público.

## P1149 — preservação sintática de labels estáticos de `counter`

O call dispatch intercepta somente chamadas estáticas `counter.at(counter,
<label>)` e `counter.display(counter, ..., at: <label>, ...)` quando o literal
label seria perdido pela avaliação genérica. Ele converte o nó em
`Value::Label` e continua a delegar ao owner `stdlib/counter`; não resolve o
counter nem a location neste módulo. As formas por string, `Location` e demais
valores continuam no caminho genérico. Match fechado, sem registry.

Sondas coincidentes no vanilla ratificado confirmam equivalência com as formas
de instância. É glue interno de paridade e não muda contrato ou fase.

## P1157 — `location.page-numbering()` preserva Numbering

### Medição antes da decisão

Vanilla devolveu o próprio callback, com repr `(..) => ..`, e não a vista
formatada da página.

### Decisão

O método consulta `Introspector::page_numbering(location)` e converte
`Pattern` em `Value::Str`, `Func` em `Value::Func`, ausência em `Value::None`.
Não consulta as vistas realizadas e não aplica callback. A alteração depende do
contrato tipado aprovado em P1157.

## P1159 — consulta efetiva de `page-numbering`

O braço `page-numbering` consulta o Numbering cru através do introspector e
devolve `Str`, `Func` ou `None`. Nunca consulta a vista realizada nem aplica o
callback.

## P1284 — equivalência ligada/não ligada sem duplicação

### Medição antes da decisão

O contrato `C-P1284-v2` mede que os métodos P1284 devem funcionar em duas
formas: `value.method(args)` e `type.method(value,args)`. A primeira passa por
intercepção sintáctica quando mutação, contexto ou preservação de AST a exige;
a segunda resolve um `Func` pelo `field_access` e entra em `apply_func`.

### Decisão

- Avaliar receiver e argumentos exatamente uma vez. A intercepção só escolhe
  rota e delega ao owner: coleções/arguments → `stdlib/collections`; cor →
  `stdlib/color` via `value_methods`; state → `stdlib/state`; selectors
  `and/or/within` → `value_methods`; location → `eval_location_method`;
  comprimento/duração/direção/alinhamento → helpers internos fechados.
- Wrappers de `field_access` sintetizam o mesmo receiver como primeiro
  positional. Não copiam parser, callback, defaults, erro ou metadata.
- `push/pop/insert/remove` ligados preservam o caminho sintáctico de l-value.
  A função não ligada não adquire autoridade para mutar um binding remoto.
- `length.to-absolute` e métodos de `state`/`location` preservam o gate de
  contexto; a forma estática não contorna introspecção nem inventa defaults.
- `arguments.len` conta posicionais e named no nível da linguagem sem mudar a
  assinatura ou a semântica Rust pública de `Args`.

O match continua fechado. Nomes desconhecidos caem no field access normal; a
intercepção não transforma presença desconhecida em sucesso.

## P1289 — chamada ligada de `float.is-infinite`

### Medição anterior à decisão

No vanilla pinado, `(0.0).is-infinite()`, `(42.5).is-infinite()`,
`(float.inf).is-infinite()`, `(-float.inf).is-infinite()` e
`(float.nan).is-infinite()` produzem, respectivamente,
`false`, `false`, `true`, `true`, `false`. A chamada ligada rejeita positional
extra e named desconhecido com os mesmos diagnósticos da forma estática.

### Decisão

O match fechado de chamadas reconhece `Value::Float` somente para
`is-infinite`, avalia os argumentos uma vez e delega ao owner
`stdlib/foundations/float`, que sintetiza o receiver e chama a mesma nativa da
forma estática. Nome desconhecido cai no erro genérico vigente. Não expor o
método ligado como valor e não duplicar `f64::is_infinite` neste nó.


### Gates

`selector.before/after` não entram no match: faltam variantes públicas e
consumers exaustivos precisam de revisão. Estado:
`BLOCKED_ADR0127_PUBLIC_CONTRACT`. Também não se interceptam aproximações por
`Within`/`And`/`Or`. `color.spot/tint` e `outline.entry/*` conservam os gates
do seu L0. Nenhum campo/variante/método de trait/assinatura pública ou mudança
de fase é autorizado por esta secção.

### Critérios

Comparar resultado e erro das duas formas para cada método, inclusive ausência,
positional excedente, named desconhecido, tipo errado e default. Callbacks
preservam ordem e curto-circuito. Métodos contextuais falham fora de contexto.
Repetir e reordenar probes independentes produz o mesmo resultado.

## P1284-v5 — `Type::Arguments` é construtor chamável

### Medição antes da decisão

O oráculo selado usa `arguments(1, x: 2)` como pré-condição de
`arguments.len`. Na fonte vanilla ratificada
`foundations/args.rs:320-340`, `Args::construct` é `#[func(constructor)]`,
recebe os argumentos externos como variádicos e devolve o `Args` integral.
O L0 v4 deste owner enumerava vários braços chamáveis de `Value::Type`, mas
não legitimava `Type::Arguments`; portanto a afirmação anterior de que wrappers
privados e representação existente bastavam era incompleta.

### Decisão

No match fechado de `Value::Type`, `Type::Arguments` é chamável e constrói
`Value::Args` a partir do `Args` já avaliado da chamada:

- aceita zero ou mais posicionais e zero ou mais named;
- preserva ordem, valores, nomes e span disponíveis; named não é tratado como
  argumento desconhecido do constructor;
- `arguments()` produz arguments vazio;
- `arguments(1, x: 2)` contém um positional e um named, e sua superfície de
  linguagem tem `len() == 2`, `pos() == (1,)`, `named() == (x: 2)`;
- `type(arguments) == type` e `type(arguments(...)) == arguments`.

O constructor não reimplementa os seis métodos, que continuam pertencendo a
`stdlib/collections`; apenas materializa o receiver necessário. Não alterar o
método Rust público `Args::len`, que continua contando somente posicionais na
entidade cristalina: o total da linguagem é responsabilidade do wrapper.

Esta correção reutiliza `Type::Arguments`, `Value::Args` e `Args` existentes,
sem campo/variante/trait/assinatura Rust pública ou mudança de fase. É paridade
em fluxo contínuo ADR-0127. Mutação que omita o braço, descarte named, reordene
ou recrie `Args` sem os metadados disponíveis viola o contrato.
