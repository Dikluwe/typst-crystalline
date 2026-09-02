# Prompt L0 — `compiler/eval/call_dispatch` — dispatch de chamadas de função
Hash do Código: b4d881f1

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

## P1293.reopen-A — preservar âncoras sintáticas de `float.is-nan`

### Medição anterior à decisão

Em `2026-09-01T15:13:15-03:00`, sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitida, o
recibo segregado `p1293-implementation-receipt-a.md` de SHA-256
`14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`
registou quatro positivos verdes e cinco REDs end-to-end. Valores, tipos,
`repr` e mensagens já coincidem; somente os ranges UTF-8 half-open divergem:

| Forma em modo code | Candidato | Vanilla ratificado / contrato |
|---|---:|---:|
| `float.is-nan()` | `12..14` | `0..14` |
| `float.is-nan(0.0, 1.0)` | `12..22` | `18..21` |
| `float("NaN").is-nan(other: true)` | `19..32` | `20..31` |
| `float.is-nan("x")` | `12..17` | `13..16` |

No consumer vigente e ainda sem patch P1293 para esta reabertura,
`call_dispatch.rs:899-909` avalia a chamada ligada de `Value::Float` e só
depois entrega o `Args` agregado ao owner. O caminho genérico faz o mesmo em
`call_dispatch.rs:1213-1240`. Já o precedente interno medido em
`call_dispatch.rs:964-985` recolhe `call.span()`, cada positional e cada named
completo antes de `eval_args`; `call_dispatch.rs:1215-1235` demonstra ainda o
override pontual de `args.span` para `eval`. Logo a AST contém as quatro
âncoras exigidas antes de `Args` as colapsar, e o owner deste nó é o ponto
causal que ainda as possui.

### Classificação e decisão

O range do diagnóstico é observável da linguagem quando o erro o expõe
(ADR-0107), não detalhe de layout Rust. Como P1293 já congelou mensagem e span
exatos, esta é correção interna de paridade em fluxo contínuo ADR-0127: não
adiciona campo, trait ou assinatura Rust pública, não altera default e não
move avaliação entre eval/layout.

Somente para a chamada cuja identidade resolvida é a nativa estática
`float.is-nan`, e para a chamada ligada de `Value::Float` com método
`is-nan`, `eval_func_call` deve preservar antes de `eval_args`:

- o span da chamada inteira;
- o span de cada positional na ordem sintática;
- o span completo de cada named, por nome.

Esses metadados privados selecionam o `args.span` entregue ao owner sem mudar
valores, nomes ou ordem: chamada inteira para `missing self`; segundo
positional para `unexpected argument`; named completo para named desconhecido;
primeiro positional para falha de cast. A precedência diagnóstica vigente
permanece soberana quando mais de uma forma inválida coexistir. A extração
segue o precedente `CollectionCallSpans`, mas fica restrita a `is-nan`; não
cria fallback genérico por nome, parsing de texto ou armazenamento permanente.

É proibido alterar `entities::Args`, qualquer API pública, a ordem/quantidade
de avaliações, `trace_call`, mensagens, semântica booleana, defaults ou fase.
Caminhos sintéticos sem AST conservam o fallback agregado atual. Este Prompt
continua proprietário 1:1 apenas de
`01_core/src/compiler/eval/call_dispatch.rs`; o owner
`stdlib/foundations/float` conserva fórmula e validação semântica.

## P1293.reopen-B-spans — âncoras privadas das quatro identidades math

### Medição anterior à decisão

Em `2026-09-01T16:38:33-03:00`, sobre
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não
commitada identificada no recibo de implementação B SHA-256
`7ebff1e223ceebe789224ed18f2c0518990aae787d7f3e48c3f96bbfb3d792dd`,
os dez testes próprios de semântica, morfologia e layout ficaram GREEN:
`10 passed / 0 failed / 5375 filtered`. Os consumers B naquele bloqueio
tinham SHA-256
`437980d9843ed9159b7312efe47a09d9336d13c74d182ccc9d6c4d268554867d`
(`structural/math.rs`),
`9a24894c3341371a42afe782ad39c505aeea25d689f590cd0a6c9d60b5bbd80b`
(`eval/math.rs`),
`e9aad3461a6d0dc3152a1349f4f83fcb988e51666016b6ef4aa23e29ac05d21b`
(`repr.rs`) e
`5e3ed835337affe7aa8399d6ad8dc895ad1fe9f34344d23f7d367682f395241f`
(`layout/attach.rs`). Nenhum consumer desta secção tinha recebido escrita B.

As sondas black-box do mesmo recibo conservaram as mensagens, mas refutaram os
spans: `math.attach(1)` ancorou o argumento agregado em `1:11`, contra o valor
vanilla em `1:12`; `math.attach([x], t: 1)` ancorou em `1:11`, contra o valor
ofensivo em `1:20`; `math.mono(1)` não emitiu source/range porque o owner usa
`Span::detached()`, contra `1:10`; `math.script` compartilha a mesma causa.
Attach/binom qualificados entregam somente `args.span` agregado ao owner e,
portanto, seus missing/extra/cast/named não distinguem a âncora sintática.

Medição direta repetida em `2026-09-01T16:44:26-03:00`: o consumer
vigente deste owner, SHA-256
`ceb021c1f9ec0edc5480005e61dc0cb24dcfd8f2ed0efc6e4c8c1499afeb54d5`,
preserva `FloatIsNanCallSpans` em
`01_core/src/compiler/eval/call_dispatch.rs:60-113`; captura a chamada ligada
antes de `eval_args` em `:953-960`; e captura a identidade nativa estática
antes do `eval_args` genérico em `:1274-1303`. A AST expõe também
`NamedArg::expr()`, além do span completo do named, logo consegue distinguir
valor ofensivo de argumento named desconhecido sem mudar `entities::Args`.

### Decisão estreita

O precedente privado P1293-A é estendido somente quando a identidade
**resolvida** da função nativa é `attach`, `binom`, `mono` ou `script`. Não se
reconhece texto do callee, nome de binding do usuário ou conteúdo-testemunha.
Antes de `eval_args`, o dispatch preserva transitoriamente:

- span da chamada inteira;
- span da expressão de cada positional, em ordem;
- para cada named, tanto o span completo quanto o span da expressão-valor;
- presença de spread, para impedir falsa precisão quando a expansão destrói
  a correspondência sintática simples.

Depois de avaliar callee e argumentos exatamente uma vez, o selector privado
substitui somente `args.span` antes de delegar ao owner existente. A tabela é
fechada pela assinatura já congelada:

| identidade | classe diagnóstica vigente | âncora privada |
|---|---|---|
| `attach` | base ausente | chamada inteira |
| `attach` | base/slot com cast inválido | valor do positional/named ofensivo |
| `attach` | segundo positional | segundo valor positional |
| `attach` | named fora de `t,b,tl,bl,tr,br` | named completo |
| `binom` | upper ou lower ausente | chamada inteira |
| `binom` | upper/lower com cast inválido | valor positional/named ofensivo |
| `binom` | named fora de `upper` | named completo |
| `mono` | body ausente | chamada inteira |
| `mono` | body inválido / segundo positional / qualquer named | primeiro valor / segundo valor / named completo, respectivamente |
| `script` | body ausente | chamada inteira |
| `script` | body inválido / segundo positional | primeiro / segundo valor positional |
| `script` | `cramped` não bool / outro named | valor de `cramped` / named completo |

O selector preserva a precedência diagnóstica vigente do owner downstream;
ele não muda qual erro ou mensagem vence quando coexistem invalidades, não
curto-circuita avaliação e não reimplementa casts ou constructors. Em caso
de spread ou ausência da correspondência estrutural esperada, conserva o
fallback agregado existente; não inventa span por heurística.

O owner `compiler/stdlib/structural/math.md` continua dono das mensagens,
assinaturas, casts e constructors de attach/binom. O owner
`compiler/stdlib/math_style.md` continua dono das mensagens e estilos de
mono/script e passa a consumir `args.span` somente nessas duas identidades.
Este owner continua 1:1 com `call_dispatch.rs`; os metadados não sobrevivem à
chamada e nenhum owner 1:N é criado.

Spans de erro são observáveis da linguagem (ADR-0107). Como o contrato P1293
já exige mensagem e intervalo exatos, esta é correção interna de paridade em
fluxo contínuo ADR-0127. É proibido alterar `entities::Args`, API pública,
ordem/quantidade de avaliações, mensagens, precedência, valores, morfologia,
layout, defaults, `trace_call` ou fase eval/layout. Refutam a decisão qualquer
regressão nos dez GREENs, qualquer mensagem/precedência diferente, captura
fora das quatro identidades, ou span que não seja chamada/positional/named
medido. Nenhum novo gate humano é necessário; novo owner ou contrato público
reabriria essa classificação e obriga parada.

## P1293.reopen-C — âncoras privadas das sete identidades HTML

### Medição anterior à decisão

O recibo residual P1293/C SHA-256
`4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mede `37/37` negativos com polaridade preservada e span agregado divergente;
em 29, a mensagem já é idêntica. `html.rs:819-843,859-862` recebe somente
`args.span`, enquanto `call_dispatch.rs:139-189` ainda possui, antes de
`eval_args`, spans de positional, named completo e valor named; a identidade
resolvida já é selecionada em `:1450-1489`. `entities::Args` deliberadamente
não conserva span por argumento e permanece fora do escopo.

### Decisão estreita

Estender o precedente privado de spans somente às identidades nativas
resolvidas `html.button`, `html.col`, `html.iframe`, `html.select`,
`html.template`, `html.video` e `html.wbr`. Antes de `eval_args`, preservar
transitoriamente chamada inteira, expressão de cada positional, named completo,
expressão-valor de cada named e presença de spread. Depois de avaliar tudo uma
única vez, selecionar apenas o `args.span` entregue ao owner HTML:

- named desconhecido, `data-*`, atributo cross-tag ou body named: named completo;
- cast/domain inválido de atributo: expressão-valor do named ofensivo;
- body em `col`/`wbr`: primeiro positional ofensivo;
- segundo body em tag normal: segundo positional ofensivo;
- forma sem correspondência segura ou com spread: fallback agregado vigente.

O owner HTML continua decidindo mensagem e precedência. Este nó não
reimplementa casts, atributos, aridade ou constructor, não reconhece texto do
callee e não persiste metadata em `Args`. Trata-se de span observável já
congelado, correção interna em fluxo contínuo ADR-0127. São proibidos API/campo
público, mudança de ordem/quantidade de avaliações, mensagem, default, fase ou
captura de outras identidades. Qualquer necessidade disso refuta a decisão.
