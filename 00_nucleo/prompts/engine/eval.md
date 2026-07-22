# Prompt L0 — rules/eval
Hash do Código: fac3c564

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/eval/mod.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0001 (comemo em L1), ADR-0024 (ecow/Value::Str), ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)

## Contexto

`eval()` é o motor de avaliação do compilador Typst. Recebe uma `Source`
e retorna um `Module` com os bindings definidos nesse ficheiro.

**Estado actual (Passo 15)**: travessia AST com control flow e ADR-0025.
Avalia literais, Ident, LetBinding, CodeBlock, Binary, Unary, Conditional (if/else),
WhileLoop, ForLoop (apenas Array). Fronteira deliberada: `_ => Ok(Value::None)`
para nós que requerem Content, Func, Styles.

## Assinatura pública

```rust
pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module>
```

**Invariante**: `eval.rs` não importa nada de `03_infra`. Acesso ao
world sempre via `TrackedWorld` (L1).

## Scope global

O entrypoint `pub fn eval` (`eval/mod.rs`) constrói o scope base do documento:

1. `make_stdlib(&inputs)` — todas as funções nativas (`type`, `len`, `rgb`,
   `table`, etc.) e os módulos builtin. **P731**: `calc`, `math`, `sym` e
   `sys` são `Value::Module` (paridade vanilla — medido: `type(calc)` →
   `module`; eram `Value::Dict`, o que bloqueava `#import calc: min, max`,
   usado pelo cetz em `aabb.typ:18`). `color`/`gradient` permanecem
   `Value::Dict` — o vanilla expõe-os como **tipo** (`type(color)` →
   `type`, medido); conversão registada como achado, fora do scope P731.
   `inputs` vem de `world.inputs()` (P694) e alimenta `sys.inputs`; os
   restantes módulos não dependem dele.
2. `predefined_color_bindings()` — atalhos `red`, `blue`, `green`, `black`, `white`,
   `yellow`, `cyan`, `magenta`, `none` (P492/P497).
3. `text` — função nativa `native_text` exposta globalmente para uso em show-rules
   (P492; ex.: `#show regex("\\d+"): it => text(red, it)`).
4. Elementos de utilizador registados no `ElementRegistry`.
5. **P709** — `std`: `Value::Module` com um clone do scope de (1)
   `make_stdlib`, definido **antes** de (1) ser espalhado (`scopes.define`)
   em `scopes` — dá acesso à stdlib não-sombreada mesmo que o documento
   redefina `length`/`calc`/etc. Ver §P709.

O scope base é depois herdado por closures e show-rules.

## §P694 — módulo builtin `sys` no scope global

`eval_with_full_error` lê `let inputs = world.inputs();` (default vazio) e
passa-o a `make_stdlib(&inputs)`, que regista `scope.define("sys",
make_sys_module(&inputs))`. `sys` é `Value::Module` (desde **P731** — era
`Value::Dict`; paridade vanilla `type(sys)` → `module`) com dois campos
no seu scope (`version: version(0, 15, 0)`, `inputs: dict` str→str) — ver
`rules/stdlib/sys.md`. A decisão de fiar `inputs` pelo `World` (e não por novos
parâmetros de `eval`/`pipeline`) está em `sys.md` e preserva a assinatura
pública do eval e os seus callers.

## §P685 — Tipos como valores no scope global

`make_stdlib()` passa a registar nomes de tipo como **valores** (`Value::Type`),
não só como funções:

- **Convertidos de função para tipo chamável**: `int`, `float`, `str`, `type`.
  Ficam `Value::Type(Type::Int | Float | Str | Type)`. A chamada continua a
  funcionar porque `eval_func_call` (`rules/eval/closures.rs`) despacha
  `Value::Type` chamável para o construtor nativo (`native_int`/`native_float`/
  `native_str`/`native_type`).
- **Novos bindings tipo** (sem colisão com nomes já registados): `bool`,
  `length`, `ratio`, `relative` (P842, #32 — paridade medida:
  `type(30% + 1em) == relative` → `true` no vanilla), `angle`, `fraction`,
  `array`, `dictionary`, `function`,
  `content`, `arguments`, `module`, `datetime`, `bytes`, `symbol`, `alignment`,
  `direction`, `location`. Nenhum é chamável (`bool(1)` → erro eval).
- **Não registados como tipo (débito)**: `color`, `gradient`, `stroke`, `regex`,
  `tiling`, `decimal`, `duration`, `version`, `label`, `state`, `counter`,
  `selector` — já existem como função/módulo; convertê-los quebraria
  `color.rgb`, `gradient.linear`, `regex(...)`, etc. `type(x) == color` fica
  `false` até reconciliação futura.

`native_type` devolve `Value::Type(v.type_of())`; a igualdade
`type(x) == length` é a de `#[derive(PartialEq)]` em `Value` (sem braço
especial em `eval_binary_op`).

Field access em `Value::Type` (`eval_field_access`, `rules/eval/bindings.rs`):
`int.min` / `int.max` → `i64::MIN`/`MAX`; `str.from-unicode` → função nativa.
Substitui o `Func::native_with_namespace` usado antes para estes campos.

## Passagem dupla do eval (P498)

`eval_with_full_error` corre o eval **duas vezes**:

1. `apply_show_rules = false` — produz `Module::introspection_content`, a árvore
   original de elementos locatable (heading, figure, metadata, etc.) antes de
   qualquer transformação de show-rule. Este conteúdo alimenta o
   `TagIntrospector`.
2. `apply_show_rules = true` — produz `Module::content`, o output renderizado
   pós-show-rules, usado pelo layout/PDF.

A flag `EvalContext::apply_show_rules` controla `intercept_content`: quando
`false`, as show-rules são registadas mas não aplicadas. Isto espelha o modelo
vanilla, onde o `Introspector` consulta os elementos originais, não o documento
renderizado.

## Variantes de Expr suportadas

- `Expr::Int` → `Value::Int(node.get())`
- `Expr::Float` → `Value::Float(node.get())`
- `Expr::Str` → `Value::Str(EcoString::from(node.get()))`
- `Expr::Bool` → `Value::Bool(node.get())`
- `Expr::None` → `Value::None`
- `Expr::Ident` → lookup em Scopes, erro se não encontrado
- `Expr::LetBinding` → eval_let: avalia init, define no scope activo
- `Expr::CodeBlock` → `scopes.enter()` antes do loop, evalua exprs
  sequencialmente via `body().exprs()`, `scopes.exit()` depois (P772l —
  paridade vanilla `typst-eval/src/code.rs:317-323`: o bloco introduz
  âmbito léxico próprio; ver §P772l), acumulando com `operators::join`
  por expressão (P728 — paridade vanilla `typst-eval/src/code.rs:57`:
  `output = join(output, value)`; `None` é identidade; combinações
  inválidas → erro; ver `rules/eval/ops.md` §P728)
- `Expr::ContentBlock` → mesmo âmbito léxico próprio via
  `scopes.enter()`/`scopes.exit()` em torno de `eval_markup` do corpo
  (P772l — paridade vanilla `typst-eval/src/code.rs:326-332`)
- `Expr::Binary(binary)` → eval_binary_op(binary.op(), lhs, rhs);
  `And`/`Or` têm braço dedicado com short-circuit (P728 — paridade vanilla
  `typst-eval/src/ops.rs:52-66`: `false and X` / `true or X` devolvem o
  lhs sem avaliar X)
- `Expr::Unary(unary)` → eval_unary_op(unary.op(), operand)
- `Expr::Conditional(cond)` → eval_conditional: condition(), if_body(), else_body()
- `Expr::WhileLoop(loop)` → eval_while: MAX_ITER=10_000 limite de segurança;
  o valor do corpo de cada iteração é acumulado com `operators::join`
  **entre iterações** (P729 — paridade vanilla `typst-eval/src/flow.rs:69,86`:
  `let mut output = Value::None` + `output = join(output, value)` antes do
  match de flow; o valor do corpo já vem com join intra-bloco via P728;
  medido: `#while i < 1 { i += 1; (1,); (2,) }` → `(1, 2)` no vanilla,
  descartado no cristalino pré-P729)
- `Expr::ForLoop(loop)` → eval_for: iterable() (não iter()), pattern via
  destructuring genérico `destructure_let` por item (P723 — inclui spread
  `..sink`, mensagens de aridade do vanilla; substitui o bind manual de
  P540), body(); o valor do corpo de cada iteração é acumulado com
  `operators::join` **entre iterações** (P729 — paridade vanilla
  `typst-eval/src/flow.rs:120,132`: qualquer tipo junta-se pela tabela de
  `join`, não só `Content`/`Str`; medido: `#for i in (1,) { (1,); (2,) }` →
  `(1, 2)` no vanilla, erro "corpo do for deve ser content" no cristalino
  pré-P729); `Value::None` como iterable é iterável vazio (sem parsing de
  array literal)

## §P635 — Mecanismo `FlowEvent` para controlo de fluxo

O cristalino implementa o mecanismo `FlowEvent` do vanilla
(`lab/typst-original/crates/typst-eval/src/flow.rs:14-22`) para que
`#break`, `#continue` e `#return` afectem de facto o fluxo de execução.

### Tipo e localização

Criar `01_core/src/engine/eval/flow.rs` em L1 com:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum FlowEvent {
    Break(Span),
    Continue(Span),
    Return(Span, Option<Value>, bool),
}
```

O método `FlowEvent::forbidden(&self) -> SourceDiagnostic` produz as mensagens
byte-idênticas ao vanilla (`lab/typst-original/crates/typst-eval/src/flow.rs:28-36`):

- `Break(span)` → `cannot break outside of loop`
- `Continue(span)` → `cannot continue outside of loop`
- `Return(span, _, _)` → `cannot return outside of function`

### Transporte no `EvalContext`

Adicionar a `EvalContext`:

```rust
pub flow: Option<FlowEvent>,
```

Inicializado a `None` em `EvalContext::new`. O campo é o equivalente cristalino
a `vm.flow` do vanilla; transporta o evento de controlo de fluxo para cima até
ao consumidor correcto (ciclo, função, ou entrypoint).

### Semântica no dispatcher (`eval_expr`)

As variantes `Expr::LoopBreak`, `Expr::LoopContinue` e `Expr::FuncReturn` deixam
de produzir erro no dispatcher topo. Em vez disso:

1. Se `ctx.flow` já for `Some`, não sobrescrever — preserva o primeiro evento
   (paridade com `vm.flow.is_none()` do vanilla).
2. Caso contrário, definir `ctx.flow` para o evento correspondente:
   - `LoopBreak(node)` → `FlowEvent::Break(node.span())`
   - `LoopContinue(node)` → `FlowEvent::Continue(node.span())`
   - `FuncReturn(node)` → avaliar o corpo opcional (`node.body()`); se houver
     valor, `FlowEvent::Return(span, Some(value), false)`; senão
     `FlowEvent::Return(span, None, false)`.
3. Devolver `Ok(Value::None)`.

### Consumo em ciclos (`eval_for`, `eval_while`)

Antes de entrar no loop, guardar e limpar o flow externo:

```rust
let flow = ctx.flow.take();
```

Após cada iteração (avaliação do corpo), inspeccionar `ctx.flow`:

- `Some(FlowEvent::Break(_))` → limpar (`ctx.flow = None`), sair do loop.
- `Some(FlowEvent::Continue(_))` → limpar (`ctx.flow = None`), continuar para a
  próxima iteração.
- `Some(FlowEvent::Return(..))` → sair do loop **sem limpar**, propagando o
  return para o contexto envolvente (função ou entrypoint).
- `None` → continuar normalmente.

No final do loop, se `flow` era `Some`, restaurar `ctx.flow = flow` para
propagar eventos de contextos externos (ex.: return de uma função externa).

### Consumo em funções (`apply_closure`)

Após avaliar o corpo da closure (`eval_expr(body_expr, ...)`), inspeccionar
`ctx.flow`:

- `Some(FlowEvent::Return(_, Some(explicit), _))` → limpar `ctx.flow`,
  devolver `Ok(explicit)`.
- `Some(FlowEvent::Return(_, None, _))` → limpar `ctx.flow`, devolver o valor
  produzido pelo corpo (normalmente `Value::None`).
- `Some(FlowEvent::Break(_) | FlowEvent::Continue(_))` → **não limpar**;
  devolver `Err(vec![flow.forbidden()])` (break/continue não são válidos
  directamente no corpo de uma função fora de um loop).
- `None` → devolver o valor do corpo.

A limpeza do `Return` ao sair da função evita que o evento seja re-interpretado
como "fora de função" pelo contexto de chamada.

### Consumo em blocos de código (`Expr::CodeBlock`)

O eval de `CodeBlock` itera sobre `body().exprs()`. Após cada expressão, se
`ctx.flow` ficar `Some`, interromper imediatamente a iteração e devolver o
último valor (o evento permanece em `ctx.flow` para o consumidor externo).

### Consumo em condicionais (`eval_conditional`)

Após avaliar o ramo `if` ou `else`, se `ctx.flow` contiver `Return`, marcar o
flag `conditional = true` (paridade com `flow.rs:55-57` do vanilla). Eventos
`Break`/`Continue` propagam-se naturalmente sem alteração.

### Consumo no entrypoint (`eval_with_full_error`)

Após `eval_markup(root, ...)` retornar, se `ctx.flow` ainda for `Some`,
devolver `Err(vec![flow.forbidden()])`. Isto cobre usos fora de contexto:
`#break`/`#continue` no topo do documento dão `cannot break outside of loop`;
`#return` no topo dá `cannot return outside of function`.

## §P634 — Controlo de fluxo fora de contexto

As mensagens de erro para `LoopBreak`, `LoopContinue` e `FuncReturn` fora de
contexto mantêm-se byte-idênticas ao vanilla
(`lab/typst-original/crates/typst-eval/src/flow.rs:28-36`):

- `Expr::LoopBreak` fora de loop → `cannot break outside of loop`
- `Expr::LoopContinue` fora de loop → `cannot continue outside of loop`
- `Expr::FuncReturn` fora de função → `cannot return outside of function`

Com P635, a detecção de "fora de contexto" deixa de ser feita no dispatcher
topo e passa a ser feita pelo consumidor que detém o `FlowEvent`: ciclos
consomem break/continue; `apply_closure` consome return; o entrypoint converte
qualquer evento residual em erro.

## Fronteira deliberada (actualizada por P634/P635)

O catch-all `_ => Ok(Value::None)` de `eval_expr` foi removido. O `match` deve
ser exaustivo: cada variante de `Expr` deve ter braço explícito ou estar
agrupada num braço documentado. Permanece `Ok(Value::None)` apenas para
variantes estritamente estruturais que não entram no dispatcher normal:

- Markup estrutural: `Text`, `Space`, `Parbreak`, `SmartQuote`, `TermItem`.
- Math estrutural: `MathText`, `MathIdent`, `MathShorthand`, `MathAlignPoint`,
  `MathDelimited`, `MathAttach`, `MathPrimes`, `MathFrac`, `MathRoot`.

Todas as outras variantes não migradas devem devolver um erro claro em vez de
`Value::None`. Em particular, `Expr::DestructAssignment` não está implementado
e deve produzir erro.

## Semântica Typst confirmada (ops.rs de referência)

- **Int/Int divisão → Float**: `5/2 = 2.5` (não truncamento)
- **Int overflow → Err**: `checked_add/sub/mul/neg`, mensagem "number too large"
- **Float → IEEE 754**: NaN e Inf propagados silenciosamente (sem guarda)
- **Divisão por zero → Err**: verificação `is_zero` antes do match
- **ADR-0025 — `Int == Float` → true em eval**: coerção explícita em eval_binary_op
  antes do wildcard `(Eq, a, b)`. `derive(PartialEq)` mantido para estruturas de dados.
  Coerção aplica-se também a ordenação (lt/leq/gt/geq com Int↔Float).
- **ADR-0107 — `Content == Content` é MORFOLÓGICO em eval** (P345): os braços
  `(Eq|Neq, Content(a), Content(b))` vêm **antes** do wildcard e comparam
  `a.morph_canon() == b.morph_canon()` — texto/markup/estilo semântico (`*bold*`)
  entram; estilo de **render** (o `TextStyle` assado, o transporte β1 `custom`, o
  `numbering_active` assado) sai. `it.body == [a]` casa por morfologia (fecha o
  Achado 2, P342). O `derive(PartialEq)` do Rust permanece **estrutural** (dois
  sistemas, ADR-0025) — `morph_canon` vive em `entities/content.md`.
- **Recursão de `#show` por PONTO-FIXO MORFOLÓGICO (P348, modelo α, ADR-0107)**: em
  `apply_show_rules` (`rules/eval/rules.rs`), o output de uma **element rule** que re-casa
  é **revisitado** num loop local até **ponto-fixo morfológico** (`morph_canon`, P345 — para
  quando a regra é no-op morfológica) ou até o **teto-64 backstop** (`MAX_SHOW_RULE_DEPTH`;
  não-convergente → erro `"maximum show rule depth exceeded"` + hints, byte-idêntico ao
  vanilla, ADR-0033). `active_guards` impede a recursão **durante** a chamada do recipe; a
  revisitação é o loop, após devolver. Caminho comum não paga `morph_canon` (checa do 2º
  passe). **Divergência consciente** vs vanilla: `#show heading: it => [= Z]` converge para
  "Z" (vanilla erra — termina por identidade de instância, mecânica/GEROU, P347b-d; a
  Revocation é INTERNA e **não** é reproduzida). Text rules (`map_text`) não recursam. Detalhe
  em `entities/f_fronteira_e1.md §3a.7-bis`.
- **Show-set (`#show k: set …`, P352, S5/`Transformation::Style`)**: `eval_show_rule` deteta o
  transform `Expr::SetRule`, **captura** o `Styles`/`StyleDelta` resultante (avaliando os args do
  set) **sem mutar `engine.styles` globalmente**, e regista a `ShowRule` com
  `Transformation::Style(styles)`. Em `apply_show_rules`, quando uma show-set casa o elemento
  (NodeKind/DynKind), o nó é **embrulhado** em `Content::Styled(elem, styles)` (o carregador da
  fatia 1, `f_fronteira_e1.md §3a.8`) e a regra **NÃO consome o passe** (espelha
  `map.apply(transform); continue` do vanilla, `typst-realize/src/lib.rs:458-464` /
  `styles.rs:504`). O elemento renderiza sob a chain aumentada; não é substituído. **Não** é
  válida sobre `Selector::Text`. **Fora de escopo**: caso 1 (composição multi-regra) — colide com
  o modelo α (relatório P352 §4); não materializado aqui. Detalhe em `entities/show.md`.
- **`#show "texto": …` com transformação Content/Func (P790)**: antes de P790, só
  `Transformation::Str` era aplicada sobre `Selector::Text` (`map_text`); `Content`/`Func`
  eram **descartados em silêncio** (achado P786, módulo `eval::rules`). P790 liga o splice:
  cada nó `Content::Text` é fatiado nas ocorrências do padrão e o replacement é emendado
  entre as fatias (`map_content`, sem reentrada no nó substituído — equivalente à
  `Revocation` do vanilla, `typst-realize/src/lib.rs:1391`). `Func` é chamada por ocorrência
  com o texto do match como `Content::Text`; output Content/Str é emendado, outro tipo é erro.
  O match é por nó de texto individual (cross-node: scope-out, ver `entities/show.md`).
  Selector de texto **vazio** (`#show "": …`) → erro `"text selector is empty"` (paridade
  vanilla `selector.rs:110`, medido por execução em P790).
- **`#show page: …` e `#show par: set block(spacing: ..)` (P790)**: `page`/`par` não existem
  como variáveis na stdlib cristalina — eram `unknown variable` fatal (achado P786). O vanilla
  tem element functions em scope e emite warnings específicos (`typst-eval/src/rules.rs:67-95`,
  medido palavra por palavra em P790). O cristalino intercepta os identificadores em
  `eval_show_rule` **antes** de avaliar o selector: `#show page: <qualquer transformação>` →
  warning `` `show page` is not supported and has no effect `` + hint
  `customize pages with \`set page(..)\` instead`, nenhuma regra registada;
  `#show par: set block(...)` com named `spacing`/`above`/`below` → warning
  `` `show par: set block(spacing: ..)` has no effect anymore `` + 2 hints
  (`write \`set par(spacing: ..)\` instead` / `this is specific to paragraphs as they are
  not considered blocks anymore`), nenhuma regra registada. Ambos exit 0, compilação prossegue.
  `#show par: <outra transformação>` → **erro explícito** (scope-out registado): no vanilla,
  `show par` é regra viva sobre `ParElem`; implementar show-par como element rule é candidato
  a passo futuro.
- **`#show <lbl>: …` — selector por label (P791)**: antes de P791, `Value::Label` no selector
  caía no braço `other` de `eval_show_rule` (`selector inválido para show rule: label`,
  achado P786, módulo `foundations::selector`). O vanilla aceita (`Selector::Label`, match
  por `target.label()`, `foundations/selector.rs:140`). P791 adiciona a variante
  `Selector::Label(Label)` e a aplicação dedicada `intercept_labelled` (`rules/eval/rules.rs`),
  chamada no ponto de associação retroactiva de `<label>` em markup (Passo 56,
  `eval/mod.rs`) — o wrapper `Content::Label` é criado depois do corpo já interceptado, logo
  só regras de label casam aí (sem dupla aplicação). `it` = o **corpo** rotulado; a saída
  substitui o wrapper (label consumido); aplicação única, última-declarada primeiro.
  `Content` substitui; `Str` é erro (consistente com `NodeKind`); show-set (`Style`)
  embrulha o wrapper em `Content::Styled` (fold das show-set de label que casam).
  **Medições P790/P791 que enquadram o âmbito** (ADR-0108): em markup, `[`/`]` são
  **texto literal** no vanilla 0.15 (`typst-syntax/src/parser.rs:91-98` — cada bracket é
  um `Text` próprio; o cristalino já tinha paridade aqui). Medido em P791: `[orig] <sp>`
  + `#show <sp>: it => [LBL=#it]` produz `[origLBL=]` **byte-idêntico ao vanilla**, e
  `ABC <sp>` → `LBL=ABC` par (observável medido; a segmentação interna de nós de texto
  que o explica é inferência marcada — o vanilla casa o label no `Text("]")` final por
  separar cada bracket, e o cristalino reproduz o mesmo observável). Divergências
  registadas que ficam **fora do âmbito**: `query(<sp>)` sobre texto rotulado → vanilla
  `found=1`, cristalino `found=0` (**label em nó de texto não é indexado** pelo
  introspector — divergência separada pré-existente, fora do âmbito show, candidata a
  passo); `#label("nome")` (função, `stdlib/label.rs`) não passa por `intercept_labelled`
  (scope-out — o caminho de show-by-label cobre a sintaxe `<lbl>`).
- **Warning de label órfã (P802)**: quando `<lbl>` não tem elemento anterior anexável
  na sequência de markup (ex.: `<abc> Hello` — label em primeiro lugar), o cristalino
  emite `label `<abc>` is not attached to anything` via `Sink::warn_note` (span do nó
  label), paridade vanilla `typst-eval/src/markup.rs:66-70`. A label é descartada (como
  no vanilla) e os espaços recolhidos na procura do alvo são re-inseridos. Label
  anexada (incluindo a nó de texto, ex.: `Hello <abc>`) não avisa — medido no vanilla
  0.15.0. O warning vanilla "content labelled multiple times" (2ª label no mesmo
  elemento) continua **fora do âmbito** — registado, não implementado.
- **Flag de erro completo (P350c)**: `EvalContext.full_error` (default `false`, recebida via
  `eval_with_full_error` — `eval()` é o delegado com `false`; L1 não lê env). Quando ligada,
  o erro do teto ganha um **3º hint** classificando **cíclico** (morfologia do caminho repetiu)
  ou **não-convergente** (teto sem repetição) — **2 rótulos** (sem "converge-fundo": afirmar só
  o medido, ADR-0108). Mensagem base + 2 hints do vanilla **byte-idênticos** sem a flag; o
  histórico de morfologias só é alocado sob a flag (caminho quente intacto). Detalhe + débito
  (CLI + fio `RunIntent`→L3-interno) em `entities/f_fronteira_e1.md §3a.7-bis`.
- **BinOp variants**: `Add, Sub, Mul, Div, And, Or, Eq, Neq, Lt, Leq, Gt, Geq,
  Assign, In, NotIn, AddAssign, SubAssign, MulAssign, DivAssign`
- **UnOp variants**: `Pos, Neg, Not`

## §P536 — Metadados do documento (`#set document(...)`)

`#set document(title: ..., author: ..., keywords: ...)` é interceptado em
`eval_set_rule` (target `"document"`) e não emite aviso de "não suportado".
Os valores avaliados são acumulados em `EvalContext::document_info`:

- `title` → `Option<EcoString>`.
- `author` → `Option<EcoString>`; arrays de strings convertidos para uma única
  string separada por vírgula.
- `keywords` → `Option<EcoString>`; arrays convertidos da mesma forma.

No final do eval (`eval_with_full_error`), `ctx.document_info` é copiado para
o `Module` via `Module::set_document_info`. O pipeline transporta-o para
`PagedDocument::document_info`, e o exportador PDF (`PdfBuilder`) escreve o
`/Info` do PDF.

## §P636 — Validação de tipos em `#set` rules

Regras `#set` que aceitam um tipo específico devem rejeitar valores de tipo
errado com mensagem clara, em vez de ignorar silenciosamente. O formato segue
o vanilla (`foundations/cast.rs:325-335`): `expected {expected}, found {found}`,
usando os nomes de tipo de `Value::type_name()`.

### Nove casos corrigidos

| # | Regra | Propriedade | Tipo esperado | Local em `rules.rs` |
|---|---|---|---|---|
| 8 | `#set math.equation` | `numbering` | `str` ou `none` | `math.equation` arm (`.ok()` descarta erro) |
| 9 | `#set math.equation` | `numbering` | `str` ou `none` | match pós-avaliação |
| 10 | `#set figure` | `numbering` | `str` ou `none` | match de `figure.numbering` |
| 11 | `#set table` | `numbering` | `str` ou `none` | match de `table.numbering` |
| 12 | `#set page` | `numbering` | `str` ou `none` | match de `page.numbering` |
| 13 | `#set page` | `columns` | `int` ≥ 1 | match de `page.columns` |
| 14 | `#set text` | `weight` | `int` ou `str` | match de `text.weight` |
| 15 | `#set document` | `title` | `str` | `value_to_eco_string` |
| 15 | `#set document` | `author`, `keywords` | `str` ou array de `str` | `value_to_eco_string` |
| 16 | `#set page` | `width`, `height`, `margin` | `length`, `float` ou `int` | `extract_pt` |

### Semântica

- Propriedades com tipo único esperado (ex.: `page.width` → length/float/int):
  - Se o valor avaliado não for do tipo esperado, devolver
    `Err(vec![SourceDiagnostic::error(span, "expected {expected}, found {actual}")])`.
  - `Value::None` continua a significar "não alterar" / "herdar".
- Propriedades com múltiplos tipos válidos (ex.: `text.weight` → `int` ou `str`):
  - Aceitar os tipos válidos.
  - Para tipos inválidos, devolver erro no mesmo formato.
- `document.title`: a função `value_to_eco_string` deve devolver
  `Result<EcoString, SourceDiagnostic>`; só aceita `str` (no vanilla,
  `title: Option<Content>`; no cristalino, `DocumentInfo.title` é `EcoString`).
- `document.author`/`keywords`: a função `value_to_eco_string` aceita `str` ou
  array de `str`; arrays só são válidos se todos os elementos forem `str`
  (caso contrário, erro no elemento inválido).
- `math.equation` target pontuado: o erro de `eval_expr` (variável indefinida)
  já deve propagar; não descartar com `.ok()`.

### Critérios de verificação

- `eval_for_test(Source("#set page(width: 'foo')\n#let x = 1"))` → `Err` contendo
  `expected length, found str` (ou equivalente com os nomes de tipo do cristalino).
- `eval_for_test(Source("#set page(numbering: 123)\n#let x = 1"))` → `Err`.
- `eval_for_test(Source("#set page(columns: 'foo')\n#let x = 1"))` → `Err`.
- `eval_for_test(Source("#set document(title: 123)\n#let x = 1"))` → `Err` contendo
  `expected string, found int`.
- `eval_for_test(Source("#set document(title: (\"A\", \"B\"))\n#let x = 1"))` → `Err`
  contendo `expected string, found array`.
- `eval_for_test(Source("#set text(weight: 'foo')\n#let x = 1"))` → `Err`.
- `eval_for_test(Source("#set math.equation(numbering: 123)\n#let x = 1"))` → `Err`.
- `eval_for_test(Source("#set math.equation(numbering: nao_existe)\n#let x = 1"))` → `Err`
  (`unknown variable`).
- `eval_for_test(Source("#set figure(numbering: 123)\n#let x = 1"))` → `Err`.
- `eval_for_test(Source("#set table(numbering: 123)\n#let x = 1"))` → `Err`.
- Uso correcto dos nove casos continua a funcionar.

## §P757 — Resolução de `em` em dimensões de página

A função auxiliar `extract_pt` usada pelo arm `#set page(width: ..., height: ...,
margin: ...)` deve resolver `Value::Length` com `Length::resolve_pt(size_pt)`,
onde `size_pt` é o tamanho de fonte activo na `StyleChain` (`engine.styles.size()`),
não com `Length::abs.to_pt()`.

### Racional

`Length` é composto por uma parte absoluta (`Abs`) e uma parte relativa (`em`).
Usar apenas `l.abs.to_pt()` descarta a componente `em`, fazendo com que valores
como `width: 7em` sejam convertidos silenciosamente para `0 pt`. Isso quebra
qualquer documento que use dimensões de página relativas ao tamanho de fonte.

### Comportamento

- `#set page(width: 7em, height: 5em)` com font-size default (11 pt) →
  `77 pt × 55 pt`.
- `#set text(size: 12pt)` seguido de `#set page(width: 7em)` → `84 pt`.
- `float`/`int` continuam a ser aceites como valores absolutos em pt.
- `Value::None` continua a significar "não alterar".

### Critérios de verificação

- `layout_test("#set page(width: 7em, height: 5em)\nX").pages[0]` tem
  `width ≈ 77 pt` e `height ≈ 55 pt`.
- `layout_test("#set text(size: 12pt)\n#set page(width: 7em, height: 5em)\nX").pages[0]`
  tem `width ≈ 84 pt` e `height ≈ 60 pt`.

## Política IEEE 754 — propagação silenciosa (ADR-0101 EM VIGOR)

Operações binárias (`eval_binary_op`) e unárias (`eval_unary_op`) sobre
`Value::Float` propagam IEEE 754 **silenciosamente**:

- `5.0 / 0.5e-200` → `Float(Inf)` sem erro.
- `0.0 / 0.0` → `Err("cannot divide by zero")` (caso especial divisor
  zero literal — paridade vanilla `foundations/ops.rs::div::is_zero`).
- `0.0 * f64::INFINITY` → `Float(NaN)` sem erro.
- `Float(NaN) == Float(NaN)` → `Bool(false)` (semântica IEEE 754).
- `Float(NaN) < Float(5.0)` → `Bool(false)` (idem).

**Não invoca `guard_float`** — esse helper é **exclusivo de
`stdlib/calc.rs`** per ADR-0101 (divergência categorial consciente
entre `eval` permissivo e `stdlib` restritivo).

**Política transversal cristalina** (ADR-0101):
- `eval`/layout/operators (este sítio): **IEEE 754 puro** —
  paridade vanilla `foundations/ops.rs` total.
- `stdlib` funções matemáticas (`stdlib.md` §"Política IEEE 754"):
  rejeita NaN+Inf via `guard_float` — divergência consciente vanilla.

Esta dualidade declarativa foi **formalizada em ADR-0101** após
auditoria transversal P309 (catálogo 67 sítios L1).

Cross-references:
- `00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md` — política
  formalizada.
- `00_nucleo/prompts/engine/stdlib.md` §"Política IEEE 754" — sítio
  divergente (guard_float).
- `00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md` — catálogo
  P309.

## Integração com comemo — Cenário D (confirmado)

`eval_for_test<W: TrackedWorld>` em `#[cfg(test)]` coerce `&W` → `&dyn TrackedWorld`
e chama `dyn_world.track()` para obter `Tracked<dyn TrackedWorld>`. O `#[comemo::track]`
em `TrackedWorld` gera `impl Track for dyn TrackedWorld`.

`MockWorld` (local aos testes) implementa `World`; via blanket impl é `TrackedWorld`.

## Notas de implementação

- `Scopes::new` recebe `Option<&'a Library>` — usa `None` (stdlib vazia)
- `Scopes::enter()/exit()` em vez de `push()/pop()`
- `LetBindingKind::Normal(pattern)` → `pattern.bindings()` → primeiro Ident
- `Code::exprs()` já filtra trivia — não chamar `is_trivia()` adicionalmente
- `use BinOp::*` e `use UnOp::*` PROIBIDOS: o linter confunde com imports externos (V14)
  — usar sempre `BinOp::Add`, `UnOp::Neg`, etc. directamente nos braços do match

## Whitespace no markup — P622

O eval de markup distingue dois tokens de whitespace:

- `SyntaxKind::Space` → `Content::Space` (espaço inter-palavras; não quebra linha).
- `SyntaxKind::Parbreak` → `Content::Parbreak` (quebra de parágrafo; produzida por
  uma linha em branco no markup).

Anteriormente ambos os tokens eram mapeados para `Content::Space`, o que fazia com
que parágrafos separados fossem concatenados numa única linha visual. A distinção
permite ao layout inserir uma quebra de linha no ponto do `Parbreak`.

## Critérios de Verificação

```
eval_binary_op(Add, Int(1), Int(2))     → Ok(Int(3))
eval_binary_op(Add, Str("a"), Str("b")) → Ok(Str("ab"))
eval_binary_op(Div, Int(5), Int(2))     → Ok(Float(2.5))
eval_binary_op(Eq, Int(1), Float(1.0))  → Ok(Bool(false))
eval_binary_op(Div, Int(1), Int(0))     → Err(...)
eval_binary_op(Add, Int(MAX), Int(1))   → Err(...)
eval_unary_op(Not, Bool(true))          → Ok(Bool(false))
eval_for_test: Source("#let x = 1") → module.scope().get("x") = Some(&Value::Int(1))
```

## §P634 — Critérios adicionais de verificação

- `eval_for_test(Source("#break"))` → `Err` contendo `cannot break outside of loop`
- `eval_for_test(Source("#continue"))` → `Err` contendo `cannot continue outside of loop`
- `eval_for_test(Source("#return 1"))` → `Err` contendo `cannot return outside of function`
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P635 — Critérios adicionais de verificação

- `#break` dentro de `#for` pára o ciclo; output de `#for i in range(10) { if i == 3 { break } str(i) }`
  é `"012"`.
- `#break` dentro de `#while` pára o ciclo.
- `#continue` dentro de `#for`/`#while` salta para a iteração seguinte.
- `#return` dentro de função devolve o valor dado e interrompe o corpo.
- Ciclos aninhados: `#break` só afecta o ciclo mais interno.
- `#break`/`#continue`/`#return` fora de contexto mantêm os erros de P634.
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P820 — `Deprecation` de símbolos em math + mensagem inglesa em `eval_math_callee` (achado #7 de P810)

Medição na fonte vanilla 0.15.0 (binário + codex `sym.txt`):

- `$join$` / `$join.r$` → vanilla **warning** `` `join` is deprecated, use `bowtie.big` instead `` (span na raiz, @1:1, exit 0); cristalino dava `error: unknown variable: join` (exit 1).
- `#sym.join` → mesmo warning com span no campo (@1:5, exit 0); cristalino dava `module 'sym' does not contain field "join"`.
- `$bowtie.big$` / `$bowtie$` / `#sym.bowtie.big` → vanilla exit 0 (renders ⨝ / ⋈ / ⨝); cristalino dava `error: variável desconhecida: bowtie` (português, sem hints).
- `$foo.bar$` → vanilla `error: unknown variable: foo` + 2 hints (inglês); cristalino dava `variável desconhecida: foo` (português) — sub-achado (b) de P810 §7.

Regras (`eval/math.rs`, `eval/bindings.rs`):

- Na resolução de `MathIdent` (bare em `eval_math_expr` e raiz de field access em `eval_math_callee`): após `sym_lookup` resolver, se `sym_deprecation(name)` retornar mensagem, emitir warning verbatim com span no ident — o símbolo **resolve** (nunca erro).
- Em `eval_field_access` (modo código): se o target é o módulo `sym` e o campo está em `SYM_DEPRECATED`, emitir o warning com span no campo **antes** de resolver.
- Em `eval_math_callee`, o braço `MathIdent` desconhecido deixa de usar a mensagem portuguesa `variável desconhecida: {name}` e passa a usar `unknown_variable_math` (P780) — paridade de mensagem com o caminho bare.
- A tabela de dados e os grupos `join`/`bowtie` vivem em `engine/stdlib/sym.rs` (L0 `sym.md` §7); scope-out das 13 depreciações ao nível de variante registado lá.

Critérios de verificação (binário): ver `sym.md` §7.

## §P837 — `#set text(top-edge:/bottom-edge:)`: string fora do domínio é erro verbatim; `Length` é aceite (achados #22/#23 de P831)

Medição na fonte vanilla 0.15.0 (binário + `text/mod.rs:1169-1248`, `text/font/mod.rs:276-289`):

- (a) `#set text(top-edge: "middle")` → vanilla **erro** `expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length` (exit 1; cast de `TopEdgeMetric`); cristalino aceitava em silêncio e caía no default (exit 0). Idem `bottom-edge` com `expected "baseline", "descender", "bounds", or length`. Os domínios são distintos: `"descender"` só é válido em bottom, `"ascender"`/`"cap-height"`/`"x-height"` só em top (medido: cruzados → erro nos dois).
- (b) Tipo que não é string nem length (ex.: `top-edge: 3`) → vanilla **erro** com sufixo `, found integer` + hint `a length needs a unit - did you mean 3pt?` (hint só para `Int`, mesmo de §P816(c)).
- (c) `#set text(top-edge: 18pt, bottom-edge: -4pt)` → vanilla **aplica** os comprimentos (medido por `pdftotext -bbox`: baseline desce `18pt − cap-height`; 2ª linha desce 4pt com `bottom-edge: -4pt`); cristalino descartava `Value::Length` (arm só tratava `Value::Str`).

Regras para `eval_set_rule` com target `"text"` (`01_core/src/engine/eval/rules.rs`):

- Os arms `top-edge`/`bottom-edge` validam contra os domínios enumerados (`TOP_EDGE_METRICS`/`BOTTOM_EDGE_METRICS`) e rejeitam o resto com o helper `edge_cast_error` (mensagem verbatim; string inválida sem sufixo `found`; outros tipos com `, found {type}` no vocabulário do vanilla; hint `did you mean {i}pt?` para `Int`). Span na expressão do valor.
- `Value::Length` é aceite e propagado pelo canal custom (`text.top-edge`/`text.bottom-edge`) até `TextStyle` como `TextEdge::Length` — ver `engine/layout.md` §P762/P837 para o tipo `TextEdge` e a semântica de resolução (`length.resolve_pt(size)` a partir da baseline; bottom negativo = abaixo da baseline).
- `"bounds"` é válido nos dois domínios e é aceite; a resolução via bbox do glyph (vanilla `TextEdgeBounds::Glyph`) não existe no contrato `FontMetrics::text_edges` (não recebe glyphs) — cai no fallback defensivo de `edge_offset_pt` (default). Limitação registada; implementação de `bounds` real é scope futuro.

Critérios de verificação (binário, mensagens verbatim a bater com o vanilla):

- `#set text(top-edge: "middle")` → erro `expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length`, exit 1.
- `#set text(bottom-edge: "middle")` → erro `expected "baseline", "descender", "bounds", or length`, exit 1.
- `#set text(top-edge: "descender")` / `#set text(bottom-edge: "ascender")` → erro (domínios cruzados), exit 1.
- `#set text(top-edge: 3)` → erro com `, found integer` + hint `did you mean 3pt?`, exit 1.
- `#set text(top-edge: 18pt, bottom-edge: -4pt)` → geometria igual à do vanilla (`pdftotext -bbox`), exit 0.
- Controlo: nomes enumerados válidos (`"ascender"`, `"cap-height"`, `"x-height"`, `"baseline"`, `"descender"`, `"bounds"`) continuam a funcionar, exit 0.

## §P816 — `#set text(...)`: nome inválido é erro; tipo errado é erro; fonte desconhecida é warning (achado #3 de P810)

Medição na fonte vanilla 0.15.0 (binário `lab/typst-original/target/release/typst` + código):

- (a) `#set text(nonexistent-prop: 12pt)` → vanilla **erro** `unexpected argument: nonexistent-prop` (exit 1; `foundations/args.rs:257-266`, `Args::finish`); cristalino emitia **warning** `propriedade '...' ainda não suportada` + hint ADR-0040 e saía com **exit 0** (`eval/rules.rs`, arm `_` do `set text`).
- (b) `#set text(font: "FamiliaQueNaoExiste")` → vanilla **warning** `unknown font family: familiaquenaoexiste` (nome lowercased, span no valor do argumento, exit 0; `text/mod.rs:1577-1588` `check_font_list`, chamada no parse do arg `font` em `text/mod.rs:170-176`); cristalino em silêncio total.
- (c) `#set text(size: 12)` → vanilla **erro** `expected length, found integer` + hint `a length needs a unit - did you mean 12pt?` (exit 1; `foundations/cast.rs:325-343`); cristalino aceitava em silêncio (arm `size` ignorava não-`Length`).
- Lista de propriedades settable do `TextElem` vanilla: `text/mod.rs:182-788` (campos `pub` do `#[elem]`).

Regras para `eval_set_rule` com target `"text"` (`01_core/src/engine/eval/rules.rs`):

- **(a) Nome:** o arm `_` (propriedade não capturada) passa a distinguir duas classes:
  - Nome **fora** da lista de propriedades do `TextElem` vanilla → **erro hard** `unexpected argument: {name}` com span no nome do argumento (mesmo formato do arm `bold`/`italic` de §P665). A lista vive na constante `VANILLA_TEXT_SET_PROPS` em `rules.rs`.
  - Nome **dentro** da lista mas ainda não capturado (ex.: `hyphenate`, `stroke`, `baseline`) → mantém o warning de scope-out do Passo 107 (hint ADR-0040). A promessa do Passo 107/DEBT-49 restringe-se a propriedades válidas no vanilla.
  - Consequência: `#set text(leading: ...)` (propriedade de `par`, não de `text`) promove de warning (Passo 134) a erro hard — paridade medida: vanilla responde `unexpected argument: leading`.
- **(c) Tipo:** os arms `size` e `tracking` (ambos `Length` no vanilla) rejeitam valores não-`Length` com `expected length, found {type}` + hint `a length needs a unit - did you mean {i}pt?` quando o valor é `Int` (helper `expected_length_error`, reusa `type_mismatch` de P636). Span na expressão do valor.
- **(b) Fonte:** no arm `font`, após construir o array de famílias, cada nome literal (`Value::Str` de topo ou `name` Str dentro de dict) ausente de `engine.world.book()` (`select_family`) emite warning `unknown font family: {nome lowercased}` com span no valor do argumento — mesmo ponto lógico do vanilla (avaliação do set rule, não do shaping). Nomes em regex (`Value::Regex`) não são verificados, tal como no vanilla, que só avisa sobre `FontFamily` literais.

Critérios de verificação (binário, exit codes a bater com o vanilla):

- `#set text(nonexistent-prop: 12pt)` → erro `unexpected argument: nonexistent-prop`, exit 1.
- `#set text(size: 12)` → erro `expected length, found integer` + hint `did you mean 12pt?`, exit 1.
- `#set text(tracking: 1)` → erro análogo com hint `did you mean 1pt?`.
- `#set text(font: "FamiliaQueNaoExiste")` → warning `unknown font family: familiaquenaoexiste`, exit 0.
- Controlo: `#set text(size: 12pt, font: <família existente>)`, `#set text(hyphenate: true)` (warning, exit 0), `#set text(weight: "bold")` continuam a funcionar.

## §P665 — Reverter `text.bold`/`text.italic` como argumentos nomeados; adicionar `text.style`

Medição na fonte vanilla 0.15.0 (`lab/typst-original/crates/typst-library/src/text/mod.rs`):

- O elemento `text` expõe as propriedades `weight` e `style`, não `bold` nem `italic`.
- `#set text(bold: true)` e `#set text(italic: true)` produzem `unexpected argument: bold` / `unexpected argument: italic`.
- `#set text(weight: "bold")` e `#set text(style: "italic")` são as formas canónicas.

Regras para `eval_set_rule` com target `"text"`:

- Remover os arms `bold` e `italic` do match de propriedades de `#set text(...)`.
- Adicionar arm `style` que aceita `"normal"`, `"italic"` ou `"oblique"` e propaga para a chain como `"text.style"`.
- `bold` e `italic` devem agora produzir erro hard: `unexpected argument: {key}`.
- O markup `*...*` e `_..._` continua a usar os campos tipados `bold`/`italic` internos (`Style::bold`, `Style::italic`) — não é afectado pela mudança de linguagem.
- O layout (`engine/layout/text.rs`) lê `"text.style"` e traduz `"italic"`/`"oblique"` para `italic = true`, coexistente com o campo tipado `italic`.

Critérios de verificação:

- `#set text(bold: true)` → `Err` com `unexpected argument: bold`.
- `#set text(italic: true)` → `Err` com `unexpected argument: italic`.
- `#set text(weight: "bold")` e `#set text(style: "italic")` continuam a funcionar.
- `*negrito*` e `_itálico_` continuam a funcionar.
- Testes que usavam `#set text(bold: ...)`/`#set text(italic: ...)` são migrados para `weight`/`style`.

## §P616 — Validação de `dir` em `#set text(...)`

Medição na fonte vanilla 0.15.0:

- `lab/typst-original/crates/typst-library/src/text/mod.rs:1258-1259`: o cast
  `Smart<Dir> → TextDir` rejeita direcções de eixo Y com
  `bail!("text direction must be horizontal")`.
- `lab/typst-original/tests/suite/layout/inline/bidi.typ:70`: o teste de suite
  espera `// Error: 16-19 text direction must be horizontal` para
  `#set text(dir: ttb)`.

Regra para `eval_set_rule` com target `"text"` e propriedade `"dir"`:

- Aceitar `Value::Dir(Dir::LTR)` e `Value::Dir(Dir::RTL)` e propagar para a
  chain como `"text.dir"` (sem alteração de comportamento existente).
- Rejeitar `Value::Dir(Dir::TTB)` e `Value::Dir(Dir::BTT)` com erro hard,
  mensagem byte-idêntica ao vanilla: `"text direction must be horizontal"`.
- O span do diagnóstico deve apontar para a expressão do argumento
  (`named.expr().span()`), de modo a reproduzir o enquadramento do erro do
  vanilla.

Esta validação é **paridade da linguagem** (semântica da propriedade `dir` do
elemento `text`): o Typst actual não implementa escrita vertical de texto, e
rejeitar explicitamente é preferível a aceitar silenciosamente e renderizar
horizontal. A escrita vertical enquanto funcionalidade nova permanece fora do
scope actual (ver `00_nucleo/diagnosticos/paridade-producao-p614.md`).


## §P679 — `#import` de ficheiros locais

Medição na fonte vanilla 0.15.0 (`lab/typst-original/crates/typst-eval/src/import.rs`):

- `import.rs:22` — o `source` do import é avaliado (`source_expr.eval(vm)?`); para uma
  string literal isto produz `Value::Str`.
- `import.rs:33-39` — `Value::Str(path) => import(...)`: o caminho é resolvido e o
  ficheiro avaliado; `source` é substituído pelo `Value::Module` resultante
  (`replaced_source = true`).
- `import.rs:60-75` — `as nome`: o módulo é ligado no escopo sob `new_name`.
- `import.rs:78-106` — bare import (`imports() == None`, sem `as`): liga o módulo sob
  `bare_name()` (file_stem do caminho). Caminho dinâmico →
  `"dynamic import requires an explicit name"` (linha 96); file_stem não-identificador →
  `"module name would not be a valid identifier"` (linha 100).
- `import.rs:108-111` — `Imports::Wildcard`: itera `scope.iter()` do módulo e liga cada
  binding no escopo do importador.
- `import.rs:113-121` — `Imports::Items`: para cada item, lookup no escopo do módulo; se
  falta, `error!(component.span(), "unresolved import")` (linha 121); rename de item
  (`saudacao as ola`) liga sob `bound_name`.

Classificação (ADR-0108): a forma `#import "f.typ": ...` é **sintaxe/morfologia** da
linguagem (paridade); a mensagem de erro é **observável mecânico** (paridade ao nível do
texto do erro); o algoritmo interno de resolução (`import`/`import_file`, `Tracepoint`) é
**mecânica** e diverge de propósito (P329).

Semântica a implementar em `eval_module_import` (`01_core/src/engine/eval/modules.rs`),
dispatcher `Expr::ModuleImport` em `mod.rs`:

1. O `source` tem de ser `Expr::Str` (caminho literal). Qualquer outra forma → erro
   `import: caminho deve ser uma string literal`. (O cristalino não avalia o source como
   expressão dinâmica neste passo — alinhado ao facto de o vanilla só aceitar path/
   módulo/função/tipo, e de P679 cobrir ficheiros locais.)
2. Caminho a começar por `@` (pacote `@preview/...`): desde P681, resolvido via
   `engine.world.resolve_package(&spec)` com `spec = PackageSpec::from_str(&path)?`,
   que devolve o `Source` do entrypoint; a partir daí reaproveita exactamente o mesmo
   fluxo do ficheiro local (ciclo, `eval_imported_file`, bindings). A resolução
   (data dir → cache dir, manifesto `typst.toml`, `entrypoint`) é I/O em L3
   (`SystemWorld::resolve_package`); L1 só consome o contrato. Pacote ausente →
   erro claro (`pacote '...' não encontrado na cache local; download ainda não
   implementado (P-γ)`). Em P679 este braço devolvia "pacotes ainda não suportados";
   agora resolve offline — download continua fora do scope (P-γ).
3. Resolução do ficheiro: `engine.world.include_source(engine.current_file, &path)`
   (caminho relativo ao directório do ficheiro actual; regista o ficheiro no world).
   Erro de resolução (ficheiro inexistente) propaga a mensagem do world.
4. Detecção de ciclo: `engine.route.contains(src_id)` → erro
   `ciclo de importação detectado: ficheiro ... já está na cadeia de avaliação activa`
   (mesma família da mensagem de `#include`, em `modules.rs`).
5. Avaliação do ficheiro importado num **módulo isolado**: scope base próprio (stdlib +
   cores predefinidas + `text`), `Engine` local com `styles`/`show_rules`/`active_guards`
   próprios (não partilhados com o importador), `EvalContext` local, `route` estendida
   (`Route::extend(engine.route).with_id(src_id)`) e `current_file = src_id`. O conteúdo
   (markup) do ficheiro importado é **descartado** — só os bindings (`#let`/`#fn`) contam.
   `ctx.flow` no ficheiro importado (`#return`/`#break` solto) produz erro (`forbidden()`).
   Confirmado por sonda: ficheiro importado vê a stdlib (`range(3) → (0, 1, 2)`) e o seu
   markup solto não aparece no documento importador.
6. Construção do módulo: `Module::new(module_name, module_scope)` onde `module_name =
   bare_name()` (file_stem). Bindings no escopo do chamador conforme `imports()`:
   - `None`: define o módulo sob `new_name` (se `as`) ou `bare_name` → `Value::Module`.
   - `Wildcard`: define cada `(name, value)` de `module.scope().iter()`.
   - `Items`: para cada item, `module.scope().get(orig).cloned()`; se `None` → erro
     `unresolved import: \`{orig}\`` no span do item; senão define sob `bound_name`
     (igual a `orig` para `Simple`, ou `new_name` para `Renamed`).
7. Valor de retorno da expressão: `Value::None` (em markup é descartado; os efeitos são
   os bindings no escopo).

Critérios de verificação:

- `#import "u.typ": saudacao` liga `saudacao`; `#saudacao("Mundo")` → `Olá, Mundo!`.
- `#import "u.typ": *` liga todos os bindings (`saudacao`, `PI`).
- `#import "u.typ": saudacao as ola` liga sob `ola`.
- `#import "u.typ"` liga o módulo sob o file_stem; `#p679-utils.saudacao("Mundo")` funciona.
- `#import "u.typ" as u` liga sob `u`.
- Ciclo `a → b → a` → erro contendo `ciclo de importação detectado`.
- Ficheiro inexistente → erro de resolução (file not found).
- Item inexistente → erro contendo `unresolved import`.
- `#include` continua a funcionar sem regressão.
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P683 — `#import` a partir de módulo / field-access

Medição na fonte vanilla 0.15.0 e sonda local (`typst 0.15.0 (969087ec)`):

- `#import "a.typ" as modulo_a` seguido de `#import modulo_a: valor` → aceite; o
  `source` do segundo import avalia para o `Value::Module` ligado sob `modulo_a`,
  e `valor` é extraído do scope desse módulo.
- O padrão real de `cetz` é `#import deps.oxifmt: strfmt` e
  `#import util: typst-length`: a fonte é um identificador (`util`) ou um
  field-access (`deps.oxifmt`) que **resolve para `Value::Module`**, com `: items`.
  `deps.typ` é só `#import "@preview/oxifmt:0.2.0"` (bare), logo `deps.oxifmt` é o
  módulo do pacote `oxifmt` acedido via field-access sobre o módulo `deps`.
- `#import pacote.valor` onde `valor` **não** é módulo (ex.: string) → o vanilla
  trata o resultado como caminho e falha com `file not found`; o cristalino
  recusa com erro claro (ver abaixo) — **paridade ao nível de "é erro"**, não de
  texto (ADR-0107).

Classificação (ADR-0108): aceitar uma expressão de fonte que resolve para
`Value::Module` é **sintaxe/morfologia** da linguagem (paridade); o algoritmo de
resolução e o texto exacto do erro são **mecânica** (divergem de propósito, P329).

Semântica a implementar em `eval_module_import` (`01_core/src/engine/eval/modules.rs`):

1. Se `source` é `Expr::Str` → fluxo de P679/P681 (ficheiro local / pacote), sem
   alteração; nome por omissão = `bare_name()` (file_stem).
2. Caso contrário, avaliar a expressão de fonte no scope do chamador
   (`eval_expr(source_expr, scopes, ctx, engine)`):
   - `Value::Module(m)` → usa `m` directamente (sem resolução de ficheiro, sem
     detecção de ciclo, sem `eval_imported_file`); nome por omissão = `m.name()`.
   - qualquer outro `Value` → erro
     `import: a fonte tem de ser um caminho string ou um módulo, recebeu {tipo}`
     no span da fonte.
3. A aplicação de bindings (`None`/`Wildcard`/`Items`) é a mesma de P679; o bare
   import liga sob `new_name` (`as`) ou o nome por omissão calculado acima.

Critérios de verificação:

- `#import "u.typ" as u` + `#import u: saudacao` → `#saudacao("Mundo")` = `Olá, Mundo!`.
- field-access que resolve para módulo (`deps.oxifmt`-like) → import de item funciona.
- `#import "u.typ": saudacao` (string, P679) sem regressão.
- `#import "@preview/..."` (pacote, P681) sem regressão.
- fonte que avalia para não-módulo → erro claro contendo `tem de ser um caminho string ou um módulo`.
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P702 — `.with(...)` — intercepção em `eval_func_call`

Isolado por P701 via `cetz` (`matrix.typ:8`, `calc.round.with(digits:
precision)`): `.with(...)` (aplicação parcial de argumentos) não existia —
gap de linguagem geral, não específico de `cbor`/plugins. Semântica de
`FuncRepr::With` e a regra de fusão de `Args` (posicionais pré-ligados
primeiro, nomeados com o mais recente a vencer em colisão) estão em
`entities/func.md` §"Variante `With`". Esta secção documenta só o ponto de
intercepção sintáctica em `eval_func_call`.

**Mecanismo** (mesmo padrão de P417 `where`, P423 `or`/`and`, P504 `within`,
P466 métodos de colecção, P506 `state`/`counter`): novo bloco, antes do
caminho genérico (`callee = eval_expr(call.callee())`), que:

1. Testa se `call.callee()` é `Expr::FieldAccess` com `field == "with"`.
2. Avalia o alvo (`access.target()`). Se **não** for `Value::Func`, não
   intercepta — cai no caminho genérico (field access normal, que erra como
   sempre erraria para esse tipo/campo; nenhuma mudança de comportamento
   para não-funções, incluindo um eventual dict/módulo com uma chave/export
   literalmente chamado `with`).
3. Se for `Value::Func`, avalia os argumentos da chamada
   (`eval_args(call.args(), ...)`) e devolve
   `Ok(Value::Func(target.with(args)))` — **sem** invocar `apply_func`
   (`.with()` devolve uma função nova, não o resultado de a chamar).

Aplica-se a qualquer `Value::Func`: nativa com ou sem namespace (`table`,
`curve`, `cbor`, `calc.round`), closure de utilizador, elemento, plugin, ou
outra função já parcialmente aplicada (encadeamento — resolvido por
recursão em `apply_func`, sem lógica extra aqui).

Critérios de verificação:

- `calc.round.with(digits: 2)(3.14159)` → `3.14` (nativa com namespace).
- `{let g(a,b,c) = a+b+c; g.with(1,2)(3)}` → `6` (closure, posicionais).
- `{let h(a, named: 10) = a+named; h.with(named: 20)(5)}` → `25` (closure,
  nomeado sobrepõe default).
- `{let f(a,b,c)=a+b+c; f.with(1).with(2)(3)}` → `6` (encadeamento).
- Um dict com chave `"with"` continua a funcionar por field access normal
  (não intercetado, porque o alvo não é `Value::Func`).
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P707 — `arguments` com métodos (`.pos()`, `.named()`)

Isolado por P706 via `cetz` (`drawable.typ:70,92`, `util.typ:33,34,39,163,171`,
`coordinate.typ:391` — `sink.pos()` sobre um `..sink` variádico):
`Value::Args` só expunha `positional`/`named` como **campos** (P504), não
como métodos. `args.named()` "funcionava" parcialmente por acidente — o
field access dava o `Dict`, e depois a chamada `()` falhava com "não é
possível chamar dictionary" (dispatcher genérico de `FuncCall`,
`closures.rs:428`); `args.pos()`/`.len()`/`.at()` falhavam logo no field
access ("campo desconhecido").

### Assinatura completa do vanilla, confirmada (não assumida)

`foundations/args.rs:320-449` (`#[scope] impl Args`): `len()`, `at(key,
default:)`, `pos()` (nome real do método é `to_pos`, exposto como `pos` via
`#[func(name = "pos")]`), `named()` (idem, `to_named`), `filter(test)`,
`map(mapper)`. **Não existe `.pairs()` em `Args`** — esse método existe em
`Dict` (`foundations/dict.rs:270`), confundido inicialmente pelo passo
anterior por aparecer perto na mesma sonda (`coordinate.typ:90`,
`c.bary.pairs()`, onde `c.bary` é `Dict`, não `Args`).

Medido com documento real:
```
let f(..args) = (args.pos(), args.named(), args.len(), args.at(0), args.at("x"))
f(1, 2, x: 3, y: 4)
→ ((1, 2), (x: 3, y: 4), 4, 1, 3)
```

### Âmbito medido (cetz) — só `.pos()` e `.named()`

`grep` exaustivo aos usos de `.at(`/`.len(` em `cetz` confirma: **nenhum**
é chamado sobre um valor `Args` real (todos são `Dict`/`Array` — `style.at`,
`ctx.at`, `radii.at`, `V.at`, `pts.at`, etc., já suportados). Só `.pos()` e
`.named()` aparecem sobre sinks `..x` variádicos. Implementados **só**
estes dois — mesma disciplina de P703/P705 (não generalizar sem
consumidor medido).

**Scope-out explícito**: `.len()`, `.at(key, default:)`, `.filter(test)`,
`.map(mapper)` em `Args` — não implementados, sem consumidor medido.
`.filter()`/`.map()` precisariam de acesso a `Engine`/`Context` (chamar um
`Func`), mais trabalho do que os dois métodos medidos.

### Mecanismo

Mesmo padrão de P417/P423/P504/P466/P506/P702 — novo bloco em
`eval_func_call`, antes do caminho genérico: se o callee é `FieldAccess`
com campo `"pos"` ou `"named"` e o alvo avalia para `Value::Args`, devolve
directamente `Value::Array(a.items.clone())` / `Value::Dict(a.named.clone())`
— **sem** passar pelo field access genérico de `Value::Args` (P504), que
continua a existir inalterado para `.positional`/`.named` (sem parênteses).

Critérios de verificação:

- `{let f(..args) = args.pos(); f(1, 2, x: 3)}` → `(1, 2)`.
- `{let f(..args) = args.named(); f(1, 2, x: 3)}` → `(x: 3)`.
- `.positional`/`.named` (campo, sem parênteses, P504) sem regressão.
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P708 — Binding de parâmetros keyword-only não consome posicionais

Isolado por P707 via `cetz` (`draw/shapes.typ:582`, `line(..pts-style,
close: false, name: none)`): `apply_closure` tratava **todo** `ClosureParam`
da mesma forma — se não vinha por nome, tentava a próxima posição em
`args.items`. Isto está errado para parâmetros construídos a partir de
`Param::Named` (`nome: default` na assinatura) — esses são **keyword-only**
no vanilla, nunca preenchíveis por posição (`entities/func.md` §"Invariante
P708" documenta a distinção via `default.is_some()`, já garantida na
construção em `eval_closure_expr`).

### Medido contra o vanilla (`ADR-0114` — sonda antes da correcção)

`typst-syntax/src/ast.rs:2078-2085` confirma a distinção `Param::Pos` /
`Param::Named` / `Param::Spread` já existe no parser (cristalino já a
espelha em `entities/ast/expr.rs`, idêntico). Casos medidos:

```
let f(a, b, close: false) = (a, b, close)
f(1, 2)              → (1, 2, false)             — named por omissão
f(1, 2, close: true)  → (1, 2, true)              — named explícito
f(1, 2, 3)             → Err "unexpected argument" — 3º posicional sem
                                                      posição correspondente
                                                      (close é keyword-only)

let f(..args, close: false) = args.pos().len()
f(1, 2, 3)             → 3   — os 3 posicionais vão todos para o sink;
                                close continua a aceitar só por nome
```

### Bug confirmado (antes da correcção) — dois sintomas do mesmo erro

1. **Posicionais perdidos**: `f(..args, close: false)` chamado com
   `f(1,2,3)` sem passar `close:` → `args.pos().len()` dava `2`, não `3`
   (o parâmetro `close`, não passado, "roubava" o 3º posicional).
2. **Aceitação silenciosa, tipo errado**: `f(a, b, close: false)` chamado
   com `f(1,2,3)` (sem sink) → devolvia `(1, 2, 3)` com `close = 3` (um
   `Int`, nunca deveria existir), em vez de `Err "unexpected argument"`.

### Correcção

`apply_closure` (`rules/eval/closures.rs`): o loop de binding só tenta
`args.items.get(pos_idx)` quando `param.default.is_none()` (parâmetro
posicional, per a invariante de `entities/func.md`). Para
`param.default.is_some()` (keyword-only), só `args.named.get(...)` é
consultado; se ausente, usa o default — **nunca** avança `pos_idx`.

Após o loop, se **não** há `sink_name` (P504) e sobram itens em
`args.items[pos_idx..]`, é um **erro** — `"unexpected argument"` (mensagem
verbatim do vanilla) — em vez de descarte silencioso. Com sink, o
comportamento é inalterado (P504): os itens remanescentes vão para o
`Value::Args` do sink.

**Fechado em P733** (era scope-out explícito de P708): argumento
**nomeado** que não corresponde a nenhum parâmetro declarado e não há sink
— medido, vanilla erra (`"unexpected argument: z"`), cristalino aceitava
silenciosamente. Corrigido em §P733 (validação de `args.named`, consumo
por `shift_remove`).

Critérios de verificação:

- `f(a,b,close:false)` com `f(1,2)`/`f(1,2,close:true)`/`f(1,2,3)` →
  `(1,2,false)` / `(1,2,true)` / `Err "unexpected argument"`.
- `f(..args,close:false)` com `f(1,2,3)` → `args.pos().len() == 3`.
- Closures só-positionais e só-com-sink (sem parâmetros keyword-only)
  sem regressão — o `if param.default.is_none()` preserva o caminho
  antigo exactamente para esses casos.
- `cargo test --workspace` continua a passar — **incluindo uma varredura
  alargada do corpus de testes existente** (ADR-0114: bug de mecanismo
  central, alcance potencialmente amplo, não só `cetz`).
- `crystalline-lint .` limpo.

## §P733 — Argumento nomeado extra sem parâmetro é erro (`unexpected argument: {name}`)

Fecha o scope-out explícito de §P708. Mecanismo vanilla
(`typst-eval/src/call.rs:650-694` + `foundations/args.rs:259-268`):
`args.named(name)` **consome** o nomeado durante o binding; o sink recebe
`args.take()` — só os não consumidos; `args.finish()` reporta o primeiro
argumento não consumido na ordem original (`unexpected argument: {name}`
se nomeado, `unexpected argument` se posicional).

Medido contra o vanilla (binário `lab/typst-original/target/release/typst`):

```
let f(a) = a
f(1, z: 2)             → Err "unexpected argument: z"
f(1, 2, z: 3)          → Err "unexpected argument"        — posicional reportado primeiro
f(1, z: 2, y: 3)       → Err "unexpected argument: z"     — primeiro na ordem de inserção
f(1, z: 2, 3)          → Err "unexpected argument: z"     — ordem original entre tipos

let f(a, ..rest) = rest
f(1, z: 2, y: 3)       → arguments(z: 2, y: 3)            — sink absorve, sem erro

let f(a, named: 10, ..rest) = rest
f(1, named: 20, z: 3)  → arguments(z: 3)                  — sink exclui o nomeado consumido
```

Cristalino antes de P733: `f(1, z: 2)` aceite silenciosamente (exit 0);
o sink recebia `args.named` **inteiro**, incluindo nomes consumidos por
parâmetros.

### Correcção

`apply_closure` (`rules/eval/closures.rs`): o lookup de nomeados no loop
de binding passa de `args.named.get(name)` a consumo por `shift_remove(name)`
— o nomeado consumido sai do mapa. Após o loop: com sink, o `Value::Args`
do sink recebe só os nomeados não consumidos; sem sink, verificação na
ordem medida para estruturas separadas — posicional primeiro
(`"unexpected argument"`, P708), depois o primeiro nomeado remanescente na
ordem de inserção (`"unexpected argument: {k}"`).

**Divergência de canto registada**: com `Args` separados em `items`/`named`
(estrutura cristalina), a ordem entre tipos do vanilla (`f(1, z: 2, 3)` →
"unexpected argument: z" — o primeiro na ordem original) não é reproduzível
sem maquinaria extra; o cristalino reporta o posicional primeiro. Registado
em `00_nucleo/diagnosticos/achados-adiados-cetz.md`.

Critérios de verificação:

- `f(1, z: 2)` sem sink → `Err "unexpected argument: z"`.
- `f(1, named: 20)` com parâmetro `named:` → `(1, 20)` — sem regressão.
- `f(1, z: 2, y: 3)` com sink → sink absorve ambos (`rest.named()` correcto).
- Sink com parâmetro nomeado consumido (`f(1, named: 20, z: 3)`) → sink
  contém só `z: 3`.
- `f(1, 2, z: 3)` → `Err "unexpected argument"` (posicional primeiro).
- Testes P504/P707/P708/P715/P724 intactos; `cargo test --workspace` verde;
  `crystalline-lint .` limpo.

## §P709 — Módulo `std` (acesso à stdlib não-sombreada)

Isolado por P708 via `cetz` (`canvas.typ:32,42,170,173,180`, `util.typ:197,340,341,344`,
`styles.typ:191` — `std.length`, `std.measure`, `std.color`, `std.stroke`,
`std.curve`, `std.gradient`, `std.tiling`): quando o próprio pacote
sombreia um nome de builtin (`#let length = ...`), `std.<nome>` continua a
dar acesso à versão original — mecanismo ausente no cristalino
(`error: unknown variable: std`).

### Mecanismo vanilla confirmado (não assumido)

`foundations/scope.rs:24,51-56` do vanilla: `Scopes.base: Option<&Library>`
é uma camada de fallback SEPARADA do stack `top`/`scopes` (bindings do
utilizador). `Scopes::get` procura primeiro no stack do utilizador, depois
em `base.global.scope()`; **só se `var == "std"` e não encontrado em
nenhum dos dois**, devolve `&base.std`. `Library::std`
(`typst-library/src/lib.rs:180,231`) é `Binding::detached(global.clone())`
— um **clone independente** do módulo `global` (a stdlib inteira),
capturado uma única vez na construção da `Library`, antes de qualquer
`#let` do documento.

**Sombreamento de `std` em si — medido, não assumido**: `#let std =
"oops"; #std` → `"oops"` no vanilla (funciona como qualquer outro nome,
**não** é protegido de shadowing por `#let`). O `cannot_mutate_constant`
visto em `Scopes::get_mut` aplica-se a outra operação (mutação directa,
não à criação de um novo binding via `#let`) — não é relevante para o
mecanismo de `std` em si.

### Arquitetura cristalina — mais simples que a do vanilla

O cristalino **não tem** uma camada `base` separada do stack de scopes —
`make_stdlib()` (`eval/mod.rs:931`) devolve um `Scope` que é espalhado
directamente em `scopes` (`for (name, binding) in stdlib.iter() {
scopes.define(...) }`) antes de `scopes.enter()` empurrar um novo frame
para o corpo do documento. Como o stdlib já vive num frame que os `#let`
do documento nunca mutam (só sombreiam, empilhando por cima), **não é
preciso replicar o mecanismo de fallback dedicado do vanilla** — basta:

```rust
let stdlib = make_stdlib(&inputs);
scopes.define("std", Value::Module(Module::new("std", stdlib.clone())));
for (name, binding) in stdlib.iter() {
    scopes.define(name, binding.value().clone());
}
```

`std` participa do scope normal (sombreável como qualquer outro nome,
paridade com o comportamento medido acima) e `.field` sobre
`Value::Module` já é tratado pelo dispatcher existente de P679
(`bindings.rs`, `Value::Module(m) => m.scope().get(...)`)  — `std.calc`,
`std.length`, etc. funcionam **sem** código de dispatch novo.

### Âmbito medido — só o scope de `make_stdlib`

`grep` exaustivo aos usos de `std.` em `cetz` confirma: `color`, `curve`,
`gradient`, `length`, `measure`, `stroke`, `tiling` — todos vivem em
`make_stdlib()`. **Scope-out explícito**: `std` não inclui
`predefined_color_bindings()` (`red`/`blue`/...), `text`, nem elementos do
`ElementRegistry` — sem consumidor medido em `cetz`; se um pacote real
precisar de `std.red` ou `std.<elemento-de-utilizador>`, revisitar então.

Critérios de verificação:

- `{let length = 5; std.length}` → `length` (tipo builtin, não sombreado).
- `{let calc = "x"; std.calc.round(3.7)}` → `4` (sub-módulo através de
  `std` funciona apesar do sombreamento).
- `{let std = "oops"; std}` → `"oops"` (sombreável como qualquer nome).
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P710 — `Length.to-absolute()`

Isolado por P709 via `cetz` (`canvas.typ:36`, `util.typ:131`):
`Value::Length` não tinha nenhum campo ou método. Medido contra o vanilla
(`foundations/layout/length.rs:96-161`) — `#[scope] impl Length` expõe
`.pt()`, `.mm()`, `.cm()`, `.inches()`, `.to_absolute()` (nome exposto
como `to-absolute`); campos `.abs`/`.em` vêm de `#[ty(scope, cast)]` no
próprio `struct Length` (não do bloco `#[scope]`).

### Semântica confirmada, não assumida

```
(6pt).to-absolute()                      → 6pt          (em=0, inalterado)
(6pt + 10em).to-absolute()  [size: 12pt] → 126pt        (6 + 10*12)
(6pt).pt()/.mm()/.cm()/.inches()          → 6/2.116.../0.211.../0.0833...
(6pt).abs                                 → 6pt
(40em + 2pt).abs                          → 2pt          (só a parte abs)
(3em + 5pt).em                            → 3            (só a parte em)
(6pt + 1em).pt()                          → Err "cannot convert a length
                                             with non-zero em units
                                             (`6pt + 1em`) to pt"
(6pt).to-absolute()  [sem `context`]      → Err "can only be used when
                                             context is known"
```

### Âmbito medido — só `.to-absolute()`

`grep` exaustivo a `cetz`: **só** `.to-absolute()` é usado
(`canvas.typ:36`, `util.typ:131`). **Scope-out explícito**: `.pt()`,
`.mm()`, `.cm()`, `.inches()`, `.abs`, `.em` — não implementados, sem
consumidor medido.

### Mecanismo — tamanho de texto já resolvível, sem gate de `context`

`StyleChain::size(&self) -> f64` (`entities/style_chain.rs:433-443`) **já
existe** e resolve o tamanho de texto actual em pontos (`text.size`
tipado ou `custom`, top-wins, default `11.0`) — mesma função usada pelo
layout de texto. `.to-absolute()` implementado como nova intercepção em
`eval_func_call` (mesmo padrão de P417/P423/P504/P466/P506/P702/P707):
se o callee é `FieldAccess` com campo `"to-absolute"` e o alvo avalia
para `Value::Length`, devolve
`Value::Length { abs: Abs(l.abs.to_pt() + l.em * engine.styles.size()), em: 0.0 }`.

**Divergência documentada (mecânica, não língua)**: o vanilla restringe
`.to-absolute()` a dentro de um bloco `context` (precisa de
`Tracked<Context>`; fora disso, erro `"can only be used when context is
known"`). O cristalino **não replica este gate** — `engine.styles` é
sempre acessível em qualquer ponto do eval (não há distinção entre
"scripting simples" e "contexto resolvido" nesta arquitectura, ver nota
em `rules/stdlib/context.md`/`entities/context_block.md`), logo
`.to-absolute()` funciona tanto dentro como fora de `context {...}`,
sempre com o `text.size` ambiente correcto. Justificação: `cetz` só usa
`.to-absolute()` dentro de `context {...}` (`canvas.typ:26`, a função
inteira é `context { ... }`), logo o valor produzido é idêntico ao
vanilla nesse caso — só o *gate* de erro fora de contexto diverge, sem
consumidor medido que dependa dele.

Critérios de verificação:

- `(6pt).to-absolute()` → `6pt` (em=0, inalterado).
- `(6pt + 10em).to-absolute()` com `#set text(size: 12pt)` → `126pt`.
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P712 — `measure()` real (intercepção + gate de `context`)

Isolado por P711 (achado lateral não relacionado): `measure()` devolvia
sempre `Abs(0.0)`, dentro e fora de `context`, sem erro nenhum
(`native_measure`, `stdlib/layout.rs`, delegava a `measure_content`,
`layout/helpers.rs`, que só trata `Content::Shape`/`Content::Sequence`
— texto nunca é medido). Confirmado no vanilla
(`layout/measure.rs:46-105`): `#[func(contextual)]` — exige `Context`
(erro `"can only be used when context is known"` fora de `context`);
dentro, invoca `(engine.library.routines.layout_frame)(...)` — um
layout REAL sobre o `body`, numa região `Region::new(.., Abs::inf())`
(sem `width`/`height` explícitos) — e devolve `frame.size()`.

### Mecanismo — duas peças, porque `NativeFn` não tem `engine`

`measure`, como qualquer stdlib fn registada via `Func::native(name,
fn)`, tem assinatura genérica `(ctx: &mut EvalContext, args: &Args,
world: &dyn World, file: FileId)` — **sem** `engine.styles` nem
`ctx.in_context` gate útil sem mais. Mesmo problema já resolvido por
P702/P707/P710 (intercepção antes do dispatch genérico); aqui a
intercepção reconhece **duas formas sintácticas**: `measure(...)`
(`Expr::Ident`) e `std.measure(...)`/`x.measure(...)`
(`Expr::FieldAccess`, forma qualificada — o caminho real do `cetz`,
`util.typ:197`: `std.measure(cnt)`). Verifica primeiro a forma
sintáctica do nome (`"measure"`, sem side-effects), só depois avalia o
callee e compara **identidade de fn-ptr** (`native_fn_addr`, mesmo
padrão de `bindings::eval_element_where`, `bindings.rs:354`) contra
`native_measure as fn(_, _, _, _) -> _` — não o nome, para não capturar
um `measure` sombreado pelo utilizador (`#let measure = ...`).

Se a identidade bate:

1. Gate `ctx.in_context` — mesma convenção já usada por
   `counter.get()`/`state.get()` (`stdlib/counter.rs:144`,
   `stdlib/state.rs:63`): fora de `context`, erro `"measure() can only
   be used inside context"` (paridade de comportamento com o vanilla;
   texto adaptado à convenção já estabelecida no cristalino, não o
   texto exacto do vanilla com hints).
2. Dentro: `extract_measure_body` (validação de argumentos, partilhada
   com o `NativeFn` fallback — `stdlib/layout.rs`) + `measure_content_real`
   (`layout/mod.rs` §P712, `engine/layout.md` §"`measure_content_real`") —
   layout real e isolado via `layout_sub_frame` (Passo 629), não
   aproximação manual por tipo de `Content`. Devolve `Value::Dict {
   width, height }`.

`native_measure` (o `NativeFn` em si) passa a ser só o fallback de
invocação indirecta (`measure` como valor de primeira classe — sem
consumidor medido); sem `engine.styles`, **falha sempre** em vez de
devolver `(0, 0)` silenciosamente (ADR-0108 — falhar alto é preferível
a um valor errado sem aviso, quando não há consumidor real a proteger).

### Divergência documentada (mecânica, não língua — ADR-0107)

`FixedMetrics` (monoespaçado, 0.6×size/codepoint) — L1 não tem acesso a
métricas de fonte reais (`FallbackFontMetrics` é L3, `03_infra`). A
largura devolvida é real e proporcional ao conteúdo dado o motor de
layout usado (mesmo `layout_sub_frame` do documento principal), mas não
byte-exacta ao vanilla (shaping real via `rustybuzz`). Sem consumidor
medido que dependa do valor exacto (`cetz` usa `measure()` para
decisões geométricas relativas, não comparação com uma constante).

Critérios de verificação:

- `measure("x")` fora de `context` → `Err "measure() can only be used
  inside context"` (idem `std.measure("x")`).
- `#let measure = (x) => x + 1; measure(4)` → `5` (sombreado, não
  intercepta).
- Dentro de `context`, `measure(texto)`/`measure(shape)` devolve
  dimensões reais > 0, proporcionais ao conteúdo (validado por
  reprodução manual — a resolução de `context` só corre em L3,
  `expand_context_blocks`, fora do alcance do harness L1 de
  `eval/tests.rs`; ver `00_nucleo/diagnosticos/paridade-producao-p712.md`).
- `cargo test --workspace` continua a passar.
- `crystalline-lint .` limpo.

## §P715 — Desestruturação (`let`), atribuição por desestruturação e atribuição simples/composta

Isolado por P714 via `cetz` (`coordinate.typ:259,264`: `(ctx, p) =
resolve(ctx, p)`) — `Expr::DestructAssignment` era um stub
(`"destructuring assignment is not yet implemented"`). Sonda revelou
um problema mais fundamental e anterior: `eval_let` (`bindings.rs`)
**já estava errado** para `let (a, b) = ...` — usava
`pattern.bindings().into_iter().next()`, ligando **só o primeiro**
ident **ao valor inteiro** (não ao elemento correspondente), e nunca
definia os restantes. Medido: `#let (a, b) = (1, 2); #b` → `error:
unknown variable: b` (antes desta correcção). Confirmado que
atribuição simples (`x = 5`) **também** não tinha braço em
`eval_binary_op` (`"cannot apply Assign to int and int"`) — as duas
formas partilham a mesma necessidade de raiz: mutar um binding já
existente, não criar um novo.

### Mecanismo vanilla confirmado (`typst-eval/binding.rs`, `access.rs`)

Um único `destructure_impl` genérico, parametrizado por uma função `f`
que decide o que fazer com cada folha (ident + valor):
`destructure()` (para `let`) define um novo binding; `DestructAssignment::eval`
muta um binding existente via `Access` (`ast::Expr::access(vm)`,
`access.rs:14-27` — suporta `Ident`, `Parenthesized`, `FieldAccess`
(mutação de campo de dict), `FuncCall` só para métodos "accessor"
como `.at()`). Suporta array (posicionais + `..sink`) e dict (`ident`
shorthand = `key: key`, `key: pattern` renomeia/aninha, `..sink`
recolhe chaves não usadas), recursivo (padrões aninhados). Mensagens
de erro exactas: `"cannot destructure {ty}"`, `"cannot destructure
named pattern from an array"`, `"cannot destructure unnamed pattern
from dictionary"`, `"{quantifier} elements to destructure"` (com hint
`"the provided array has a length of {len}, but the pattern expects
{expected}"`), `"cannot mutate a temporary value"`.

### Implementação — mirror do mecanismo, scope-out medido do `Access` genérico

- **`Scope::get_mut`/`Scopes::get_mut`** (`entities/scope.rs`,
  `rules/scopes.rs`) — novo: acesso mutável a um binding já existente
  (não cria, distinto de `define`). `Scopes::get_mut` pesquisa `top` →
  `scopes` (mesma ordem de `get`), **não** pesquisa `captured` nem
  `base` diretamente — devolve `None` nesses casos, e o caller
  (`access()`, `eval/bindings.rs`) distingue-os via `captured_by`
  (P772q, §P772q) e `is_constant` (P772n, §P772n) antes de cair no
  "unknown variable" genérico (com hint condicional — P772r, §P772r).
- **`destructure_pattern`/`destructure_array`/`destructure_dict`**
  (`bindings.rs`) — mirror exacto de `destructure_impl`/
  `destructure_array`/`destructure_dict` do vanilla, incluindo as
  mensagens de erro e o hint de aridade. `f: &F where F: Fn(&mut
  Scopes, Expr, Value) -> SourceResult<()>` — mesma forma do vanilla
  (`Fn`, não `FnMut`: a função não captura `scopes`, recebe-o como
  argumento em cada folha).
- **`destructure_let`** — a `f` de `let`: só aceita `Expr::Ident`,
  `scopes.define(...)`. Reescreve `eval_let` para chamar isto em vez
  do `.next()` quebrado; a nomeação pós-hoc de closures (`#let f = (n)
  => ...` → `f` sabe o seu nome para recursão) preservada, mas
  restrita ao caso `Pattern::Normal(Expr::Ident(_))` (não faz sentido
  para padrões de desestruturação — o vanilla também não a faz em
  `destructure()`).
- **`eval_destruct_assignment`** (nova, chamada por
  `Expr::DestructAssignment` em `eval/mod.rs`) — a `f` de atribuição:
  `Expr::Ident` → `scopes.get_mut(name)`, erro `"unknown variable"` se
  ausente; qualquer outra folha → `"cannot mutate a temporary value"`
  (**scope-out medido**: sem consumidor em `cetz` para `FieldAccess`/
  `FuncCall` accessor como alvo de desestruturação).
- **`eval_assign`** (nova, chamada por `Expr::Binary` quando
  `op` é `Assign`/`AddAssign`/`SubAssign`/`MulAssign`/`DivAssign`,
  interceptado em `eval_expr` **antes** do dispatch genérico — o `lhs`
  não pode ser avaliado como valor, precisa do nome para mutar) — só
  `Expr::Ident` como alvo (**scope-out medido**, mesmo motivo); `+=`
  etc. lêem o valor actual via `scopes.get`, aplicam
  `operators::eval_binary_op` com o operador subjacente
  (`Add`/`Sub`/`Mul`/`Div`), e escrevem via `scopes.get_mut`.

### Scope-out medido — `Access` genérico (`FieldAccess`/`FuncCall` como alvo)

`cetz` usa activamente `arr.at(i) = valor` (`hobby.typ:51-58,138-242`)
e `dict.campo = valor` (`drawable.typ:36`) — ambos exigem o `Access`
completo do vanilla (mutação via referência, não substituição do valor
inteiro). **Não implementado neste passo** — confirmado como o próximo
bloqueio de `cetz` após esta correcção (`"cannot mutate a temporary
value"`); candidato a P716.

Critérios de verificação:

```
#let (a, b) = (1, 2)                    → a=1, b=2 (ambos, não só a)
#let (_, b) = (1, 2)                    → b=2 (placeholder ignora)
#let (first, ..rest) = (1,2,3,4)        → first=1, rest=(2,3,4)
#let ((a, b), c) = ((1, 2), 3)          → a=1, b=2, c=3 (aninhado)
#let (x: a) = (x: 1)                    → a=1 (dict renomeado)
#let (a, ..rest) = (a:1, b:2, c:3)      → a=1, rest=(b:2, c:3)
#let (a, b, c) = (1, 2)                 → Err "not enough elements to
                                            destructure" + hint
#let (a, b) = 5                         → Err "cannot destructure int"
#{ (ctx, p) = (10, 20) }  (ctx/p já definidos) → muta ambos in-place
#{ (nope, x) = (1, 2) }                 → Err "unknown variable: nope"
#{ x = 5 }  (x já definido)             → muta x in-place
#{ x += 10 }, x -= .., x *= .., x /= .. → aritmética composta
#{ nope = 1 }                           → Err "unknown variable: nope"
#{ 5 = 1 }                              → Err "cannot mutate a
                                            temporary value"
```

- `cargo test --workspace` → sem regressão (3841 vs 3820 antes de
  P715: +21 testes novos).
- `crystalline-lint .` limpo.

## §P716 — `Access` genérico: `dict.campo` e accessor methods como alvos de atribuição

Fecha o scope-out medido de §P715: `cetz` usa `arr.at(i) = valor`
(`hobby.typ:51-58,138-242`) e `dict.campo = valor` (`drawable.typ:36,57`)
— ambos produziam `"cannot mutate a temporary value"`. Exigem o
mecanismo `Access` do vanilla: referência mutável ao **local**
(elemento/campo), não substituição do valor completo do binding.

### Mecanismo vanilla confirmado (`typst-eval/access.rs`, `methods.rs`, `ops.rs`)

- **Lista completa de accessor methods** (`methods.rs:19-21`): `first`,
  `last`, `at` — array suporta os três (`first_mut`/`last_mut`/`at_mut`,
  `foundations/array.rs:104-119`); dict só `at` (`Dict::at_mut`,
  `foundations/dict.rs:99-104`). Não há outros.
- **`Access`** (`access.rs:14-27`) cobre exactamente 4 formas de alvo:
  `Ident` (`scopes.get_mut`), `Parenthesized` (recursivo no interior),
  `FieldAccess` (`access_dict` no target + `Dict::at_mut` no campo),
  `FuncCall` (só se o callee for `FieldAccess` cujo campo é accessor
  method: avalia os **args primeiro**, depois `access` recursivo do
  target, depois `call_method_access`; `access.rs:56-74`). Qualquer
  outra expressão: **avalia** (para efeitos) e erra
  `"cannot mutate a temporary value"`.
- **Caso especial de `apply_assignment`** (`ops.rs:77-85`): `=` puro
  (não `+=` etc.) com lhs `FieldAccess` **não** passa pelo `at_mut` do
  campo — faz `access_dict` no target + `Dict::insert` (**cria** a
  chave se não existir). Ordem de avaliação (`ops.rs:74`): **rhs
  primeiro**, depois o access do lhs. A forma composta lê o valor
  actual com `mem::take` no local e escreve o resultado do operador
  subjacente (`ops.rs:87-90`).
- **`access_dict`** (`access.rs:76-107`) com target não-dict:
  `Symbol`/`Content`/`Module`/`Func`/`Args` → `"cannot mutate fields on
  {ty}"`; `fields_on(ty)` vazio → `"{ty} does not have accessible
  fields"`; senão (Version, Length, Rel, Stroke, Alignment —
  `fields.rs:77-91`) → `"fields on {ty} are not yet mutable"` + hint
  `"try creating a new {ty} with the updated field value instead"`.
- **Desestruturação-atribuição** usa o mesmo `Access` em cada folha
  (`binding.rs:30-42`) — **sem** o caso especial de insert.
- **`call_method_access`** (`methods.rs:66-98`): tipo sem accessor →
  `"cannot mutate a temporary value"` se o tipo tem um método com esse
  nome (ex.: `str.at`), senão `"type {ty} has no method \`{method}\`"`.

Comportamento medido (compile do vanilla, working tree em `c69f40187`):

```
#{ d.a = 10 }        (a existe)      → muta;  #{ d.novo = 5 } → INSERE
#{ d.novo += 1 }                     → Err dictionary does not contain key "novo"
                                        + hint use `insert` to add or update values
#{ d.at("novo") = 7 }                → Err (mesma mensagem + hint)
#{ arr.at(1) = 20 }                  → muta o elemento
#{ arr.at(5) = 20 }  (len 3)         → Err array index out of bounds
                                        (index: 5, len: 3)   [sem sufixo "no default"]
#{ arr.at(1, default: 0) = 20 }      → Err unexpected argument: default
#{ arr.at(0, 1) = 5 }                → Err unexpected argument
#{ arr.at() = 1 }                    → Err missing argument: index
#{ d.at() = 1 }                      → Err missing argument: key
#{ arr.at("x") = 1 }                 → Err expected integer, found string
#{ arr.first() = 100 }               → muta;  vazio → Err array is empty
#{ arr.last() = 300 }                → muta;  vazio → Err array is empty
#{ s.at(0) = "x" }   (s: str)        → Err cannot mutate a temporary value
#{ x.at(0) = 1 }     (x: int)        → Err type integer has no method `at`
#{ s.len() = 1 }     (não-accessor)  → Err cannot mutate a temporary value
#{ x.a = 1 }         (x: int)        → Err integer does not have accessible fields
#{ c.body = [x] }    (c: content)    → Err cannot mutate fields on content
#{ v.major = 9 }     (v: version)    → Err fields on version are not yet mutable
                                        + hint try creating a new version with the
                                        updated field value instead
#{ (arr.at(0), arr.at(1)) = (9, 8) } → muta ambos
#{ (d.novo,) = (2,) }                → Err dictionary does not contain key "novo"
```

Nota de paridade da mensagem: nos erros acima o vanilla usa o nome
**longo** do tipo (`integer`, `string`, `boolean`), não o curto do
`type_name()` cristalino (`int`, `str`, `bool`). A mensagem de erro é o
observável (ADR-0107) — usar o nome longo nestes erros.

### Implementação — mirror em `bindings.rs`

- **`access(expr, scopes, ctx, engine) -> SourceResult<&mut Value>`** —
  mirror do trait `Access` do vanilla, como free function (o cristalino
  não tem `Vm`; recebe as três partes). 4 braços + fallback avalia-e-erra.
  `Ident` ausente → `scopes.captured_by(name)` → `scopes.is_constant(name)`
  → `unknown variable` (nesta ordem; P772q verificado primeiro). Ver §P772n
  e §P772q.
- **`access_dict(fa, scopes, ctx, engine) -> SourceResult<&mut IndexMap<…>>`**
  — mirror de `access_dict`, com os três níveis de erro medidos acima.
  Braço "not yet mutable": Version, Length, Relative, Stroke, Align (o
  espelho de `fields_on`); Duration cristalino (P412, campos de leitura)
  fica **fora** — o vanilla responde `"duration does not have accessible
  fields"` e a mensagem é o observável.
- **`is_accessor_method`** (`first`/`last`/`at`) e
  **`call_method_access(value, method, args, span)`** — mirror exacto,
  incluindo a ordem: `expect` do posicional → `at_mut` → *depois* o
  check de args excedentes (`args.finish()`: named → `"unexpected
  argument: {name}"`, posicional → `"unexpected argument"`). Índice
  negativo conta do fim (`locate_opt`, mesma regra de `array.at` P714);
  a mensagem de out-of-bounds na escrita **não** tem o sufixo
  `"and no default value was specified"` (esse é só do read com
  `default:` possível).
- **`eval_assign` reescrito** como mirror de `apply_assignment`: rhs
  primeiro; caso especial `Assign`+`FieldAccess` → `access_dict` +
  `insert`; senão `access(lhs)` + `mem::replace` do local + operador
  subjacente para as formas compostas (deixa de ler via
  `scopes.get`+clone — o local mutável já dá o valor actual).
- **`destructure_pattern`/`destructure_array`/`destructure_dict`**
  passam a enfiar `ctx`/`engine` (assinatura de `f` ganha os dois) — a
  folha de `eval_destruct_assignment` passa a `*access(expr, …)? = value`
  (mirror de `binding.rs:30-42`); a folha de `destructure_let` continua
  a ignorá-los (só `Ident` + `define`).
- `Scopes::get_mut` (§P715) inalterado.

### Critérios de verificação

- Os snippets da tabela acima, cada um com o resultado/erro medido.
- `#let d = (a: 1, b: 2); #{ d.a = 10 }; #d` → `(a: 10, b: 2)`.
- `#let arr = (1, 2, 3); #{ arr.at(1) = 20 }; #arr` → `(1, 20, 3)`.
- Aninhado: `#{ let n = (xs: (1, 2)); n.xs.at(0) = 9 }` — `access`
  recursivo (`FuncCall` accessor cujo target é `FieldAccess`).
- Sem regressão: mecanismo `Ident` (§P715) e `cargo test --workspace`.
- `crystalline-lint .` limpo.

## §P717 — Métodos mutantes (`push`, `pop`, `insert`, `remove`)

Isolado por P716 via `cetz`: `campo desconhecido em array: 'push'`.
Uso medido em `cetz` 0.5.2: `.push(` 63×, `.insert(` 26×, `.pop(` 1×,
`.remove(` 1× — os **quatro** têm consumidor; sem questão de scope-out
por falta de uso. Mecanismo irmão do §P716, sobre a mesma fundação
`access()`.

### Mecanismo vanilla confirmado (`methods.rs`, `call.rs`)

- **Lista completa** (`typst-eval/methods.rs:9-16`):
  `is_mutating_method` = `push`/`pop`/`insert`/`remove`;
  `is_dict_mutating_method` = `insert`/`remove` (dict não tem
  `push`/`pop`). Não há outros.
- **Despacho** (`call.rs:33-43` + `maybe_resolve_mutating`,
  `call.rs:189-212`): `FuncCall` com callee `FieldAccess` e método
  mutante → **args avaliados primeiro** (`call.rs:196-198`), depois
  `access()` do target (o mesmo do §P716 — targets temporários erram
  `cannot mutate a temporary value` **antes** de qualquer resolução;
  `(1, 2).push(3)` medido). Com o local em mão:
  - `Dict` + método não-dict-mutante (`push`/`pop`) → no vanilla cai
    para a resolução normal, que termina em ``type dictionary has no
    method `push` `` (medido) — dicts deliberadamente não resolvem
    campos como métodos (`eval_field_callee`, doc `call.rs:233-238`).
  - `Array`/`Dict` → `call_method_mut`, devolve o output (`pop`/
    `remove` devolvem o elemento removido; `push`/`insert` devolvem
    none).
  - Outros tipos → cai para a resolução normal com o valor clonado
    (ex.: módulo com função chamada `insert` continua a funcionar;
    `str` termina em ``type string has no method `push` ``, medido).
- **`call_method_mut`** (`methods.rs:24-63`): array `push(value)`;
  `pop()` (vazio → `array is empty`); `insert(index, value)` — `locate`
  com `end_ok=true` (`array.rs:246-257`: índice == len permitido,
  negativo conta do fim, `insert(-1, 9)` em `(1,2,3)` → `(1, 2, 9, 3)`
  medido; fora de limites → `array index out of bounds (index: 4, len:
  3)` **sem** sufixo); `remove(index, default:)` (`array.rs:261-274`:
  `locate_opt` → remove e devolve; fora de limites → `default` **sem
  mutar**, senão erro **com** sufixo `and no default value was
  specified`). Dict `insert(key, value)` (cria ou substitui);
  `remove(key, default:)` (`dict.rs:241-251`: devolve o removido;
  ausente → `default`, senão `dictionary does not contain key "x"`
  **sem hint** — ao contrário do `at_mut` do §P716). `args.finish()`
  corre **depois** da mutação (`a.remove(0, bad: 1)` → mutação feita,
  erro `unexpected argument: bad`).

Comportamento medido (vanilla em `89712433b`, binário
`lab/typst-original/target/release/typst`):

```
#{ arr.push(4) }                  → (1, 2, 3, 4); bloco devolve none
#let x = a.pop()                  → x=3, a=(1, 2)
#{ arr3.insert(1, 99) }           → (1, 99, 2, 3)
#{ a.insert(-1, 9) }              → (1, 2, 9, 3)
#{ a.insert(4, 9) }   (len 3)     → Err array index out of bounds (index: 4, len: 3)
#{ arr4.remove(1) }               → devolve 2, resta (1, 3)
#{ a.remove(5) }      (len 3)     → Err ... (index: 5, len: 3) and no default value was specified
#{ a.remove(5, default: 9) }      → devolve 9, array intacto
#{ a.remove(0, bad: 1) }          → Err unexpected argument: bad
#{ a.pop(1) }                     → Err unexpected argument
#{ a.push() }                     → Err missing argument: value
#{ a.insert(1) }                  → Err missing argument: value
#{ a.insert("x", 9) }             → Err expected integer, found string
#{ d.insert("b", 2) }             → cria a chave
#{ d.insert(5, 2) }               → Err expected string, found integer
#{ d.remove("x") }                → Err dictionary does not contain key "x"  [sem hint]
#{ d.remove("x", default: 7) }    → devolve 7
#{ d.push(2) }                    → Err type dictionary has no method `push`
#{ s.push("c") }      (s: str)    → Err type string has no method `push`
#{ (1, 2).push(3) }               → Err cannot mutate a temporary value
```

### Implementação

- **`bindings.rs`**: `is_mutating_method`/`is_dict_mutating_method`
  (listas exactas), `call_method_mut` (mirror, reutiliza
  `expect_positional`/`finish_args`/`long_type_name`/`missing_key`-sem-
  hint do §P716) e **`try_eval_mutating_method`** (`pub(super)`, mirror
  de `maybe_resolve_mutating`): avalia args, `access()` do target,
  e despacha:
  - `Dict` + `push`/`pop` → erro verbatim ``type dictionary has no
    method `{method}` `` (mesmo observável do fall-through vanilla, sem
    a maquinaria — dicts não resolvem campos como métodos);
  - `Array`/`Dict` → `call_method_mut`;
  - `Module`/`Func`/`Type`/`Symbol`/`Content` → devolve `None` =
    **fall-through** para a cadeia existente de `eval_func_call`
    (campos destes tipos podem resolver para função — ex.: módulo com
    função `insert`). **Divergência medida e aceite**: o fall-through
    cristalino re-avalia target e args (o vanilla passa os já
    avaliados) — dupla avaliação de efeitos só neste caminho; sem
    consumidor em `cetz` com args com efeitos.
  - Restantes tipos (escalares, `str`, `bytes`, …) → erro verbatim
    ``type {ty} has no method `{method}` `` com nome longo (§P716).
- **`closures.rs` `eval_func_call`**: intercepção do método mutante
  **antes** do bloco P466 (que avalia o target como valor — clone), no
  topo da cadeia de intercepções de `FieldAccess`.

### Critérios de verificação

- Os snippets da tabela acima, cada um com o resultado/erro medido.
- Retorno como expressão: `#let x = a.pop()` liga `x` ao removido.
- Sem regressão: §P715/§P716 (`arr.at(1) = 20`, `d.a = 10`) e
  `cargo test --workspace`.
- `crystalline-lint .` limpo.

## §P718 — Spread em literais de array/dict e em argumentos de chamada

Isolado por P717 via `cetz`: `#let x = (1, ..(2, 3))` dava `(1)` (len 1),
não `(1, 2, 3)` (len 3) — spread em literal de array **ignorado**
(`Expr::Array` filtrava só `ArrayItem::Pos`; `DictItem::Spread(_) => {}`
era no-op). Sonda confirmou que `eval_args` (spread em chamadas) tem o
mesmo gap **e** tem consumidor real e pesado em `cetz` (`func(..c)`,
`resolve(ctx, ..c)`, `fast-line(..pts, …)` — dezenas de sítios). Tamanho
efectivo: **M**, como o passo previa.

### Comportamento vanilla confirmado (`typst-eval/code.rs`, `call.rs`)

- **Array literal** (`code.rs:224-271`, `ast::Array::eval`): por item —
  `None` → ignora (não altera `all_dict_spreads`); `Array` → `extend`
  (`all_dict_spreads = false`); `Dict` → se `all_dict_spreads` continua
  `true` **e** todos os itens restantes também são spreads de dict (
  lookahead via `items.all(...)`, avaliando-os) → erro `cannot spread
  dictionary into array` **com hint** (`add a colon to create a
  dictionary instead: `` `{fixed}` ``, `fixed` = texto fonte do literal
  com o primeiro `(` substituído por `(: `); senão → mesmo erro **sem**
  hint; outro tipo → `cannot spread {ty} into array`. `Pos` sempre marca
  `all_dict_spreads = false`.
- **Dict literal** (`code.rs:273-306`, `ast::Dict::eval`): `None` →
  ignora; `Dict` → `extend`; outro → `cannot spread {ty} into
  dictionary`.
- **Argumentos de chamada** (`call.rs:367-416`, `ast::Args::eval`):
  `None` → ignora; `Array` → cada elemento vira posicional; `Dict` →
  cada par vira nomeado; **`Args`** → funde posicionais e nomeados
  (reencaminhamento de `..rest`, ex. `f(..b)` onde `b` é um sink `..b`
  de outra closure); outro tipo → `cannot spread {ty}` — **sem** o
  sufixo "into X" (mensagem distinta da dos literais).

Comportamento medido (vanilla em `a8383e1e6`):

```
(1, ..(2, 3), 4)              → (1, 2, 3, 4)
(a: 1, ..(b: 2, c: 3), d: 4)  → (a: 1, b: 2, c: 3, d: 4)
(..(), 1, ..())                → (1,)
(1, ..none, 2)                 → (1, 2)
(a: 1, ..none, b: 2)           → (a: 1, b: 2)
(..(1,2), ..(a: 1))             → Err cannot spread dictionary into array
                                    [sem hint — Array spread antes zera
                                    all_dict_spreads]
(..(a:1), ..(b:2))               → Err cannot spread dictionary into array
                                    + hint add a colon … `(: ..(a:1), ..(b:2))`
f(..5)  (spread int em chamada)  → Err cannot spread integer  [sem "into"]
f(..(1,2), ..(3,4))  (sink ..a)  → a.pos() = (1, 2, 3, 4)
f(1, ..(x:1), ..(y:2), 3)        → (a.pos(), a.named()) = ((1,3),(x:1,y:2))
g(..a)=a; f(..b)=g(..b); f(1,2,x:3) → r.pos()=(1,2), r.named()=(x:3)
                                        [reencaminhamento de Value::Args]
```

Confirmado (rest params, `#let f(..pts) = pts.pos().len()`): **fora do
scope deste passo** — já implementado (P504), sem regressão a verificar
aqui só pela sua presença.

### Implementação

- **`mod.rs` `Expr::Array`**: reescrito para percorrer
  `Vec<ArrayItem>` colectado (permite lookahead por índice).
  `all_dict_spreads` seguido item a item; `Dict` com lookahead via
  `remaining_are_dict_spreads` (nova, `bindings.rs`-adjacente em
  `mod.rs`: itera os itens restantes, `Spread` que avalia a `Dict` →
  continua, qualquer outra coisa → `false`); `fixed` via
  `arr.to_untyped().clone().into_text()` (`SyntaxNode::into_text`,
  reconstrói o texto do literal) + `replacen('(', "(: ", 1)`.
- **`mod.rs` `Expr::Dict`**: braço `DictItem::Spread` deixa de ser
  no-op — `None`/`Dict`/erro, mirror directo.
- **`closures.rs` `eval_args`**: braço `Arg::Spread` deixa de ser
  no-op — `None`/`Array`→items/`Dict`→named/`Args`→funde ambos/erro
  (mensagem **sem** "into", distinta dos literais).
- **`long_type_name`** (P716, `bindings.rs`) passa a `pub(super)` —
  reaproveitado nos três sítios acima para o nome longo do tipo nas
  mensagens de erro (ADR-0107: a mensagem é o observável).

### Critérios de verificação

- Os snippets da tabela acima, cada um com o resultado/erro medido
  (incluindo o texto exacto do hint).
- Reencaminhamento `Value::Args` (`g(..b)` dentro de `f(..b) = g(..b)`)
  — mecanismo distinto de array/dict, sem consumidor confirmado em
  `cetz` neste exacto padrão, mas medido no vanilla e implementado por
  ser a mesma função `Args::eval` (não há como scope-out parcial sem
  duplicar a função).
- Sem regressão: rest params (`..pts` em definição de closure, P504) e
  `cargo test --workspace`.
- `crystalline-lint .` limpo.

## §P719 — For-loop sobre `Dict` (`for (key, value) in dict`)

Isolado por P718 via `cetz`: `for (key, value) in dict {...}` errava
`"não é possível iterar sobre dictionary"` — `eval_for` (`control_flow.rs`,
P540) só tinha braço para `Value::Array`. Consumidor real e directo:
`styles.typ:189,322,354` (`for (key, value) in dict { ... }`, resolução
de estilos — sempre a forma de 2 nomes, sem `.values()`/`.keys()`).

### Mecanismo vanilla confirmado (`typst-eval/flow.rs:114-190`, `cast.rs:186-189`)

`ast::ForLoop::eval` despacha por **tipo do iterável**, não por padrão:
`Value::Array` e `Value::Dict` chamam a **mesma** macro `iter!` — a única
diferença é a fonte do iterador (`array` vs `dict.iter()`, que devolve
pares `(&Str, &Value)`). Cada item passa por `.into_value()` antes de
`destructure(vm, pattern, value)`: `IntoValue for (&Str, &Value)`
(`foundations/cast.rs:186-189`) converte o par em `Value::Array([Str(key),
value])` — **o mesmo formato usado pela iteração de array com
destructuring de 2 elementos** (`Value::Array` cujo padrão é `(a, b)`).
Não há bind especial: o mecanismo de "um nome liga o valor inteiro, N
nomes destroem posicionalmente" já usado pela iteração de array
(`destructure`, mesma função de `binding.rs` usada por `let`/atribuição)
aplica-se automaticamente ao par (chave, valor) sem código extra.

Comportamento medido (vanilla em `f571a3644`):

```
for (k, v) in (a: 1, b: 2) [#k=#v ]     → "a=1 b=2 "
for k in (a: 1, b: 2) [#k ]              → "(\"a\", 1) (\"b\", 2) "
                                             [um nome liga o PAR inteiro,
                                             não só a chave]
for v in (a: 1, b: 2).values() [#v ]     → "1 2 "  [.values() já existe]
for (a, b, c) in (x: 1, y: 2) [x]        → Err not enough elements to
                                             destructure + hint (length
                                             of 2, but pattern expects 3)
                                             [mesma família de mensagem
                                             de P715 destructure_array,
                                             não a do cristalino — ver
                                             nota de divergência pré-
                                             -existente abaixo]
```

Ordem de iteração: inserção (`IndexMap`, tanto no vanilla `dict.iter()`
quanto no cristalino `Value::Dict(IndexMap<...>)::into_iter()`) —
confirmado com `(z: 1, a: 2, m: 3)` → `z a m` nos dois lados.

### Nota de divergência pré-existente (medida, **não** corrigida neste passo)

A mensagem de aridade errada em `for` já divergia do vanilla **antes**
deste passo, só para `Value::Array` (P540): cristalino usa `"cannot
destructure {n} values into {m} bindings"`; vanilla usa `"not enough
elements to destructure"` + hint (mesma família de `wrong_number_of_
elements`, P715 `bindings.rs`). Medido: `for (a, b, c) in ((1,2),) [x]`
→ mensagens diferentes nos dois lados, reproduzível já em P540, sem
relação com `Dict`. **Fora do scope deste passo** (que é especificamente
"for-loop sobre Dict") — o reaproveitamento de `run_for_loop` faz esta
divergência pré-existente aplicar-se também ao caminho de `Dict` (era
inevitável: os dois tipos partilham o mesmo bind), mas não a introduz.
Candidato a passo futuro dedicado à paridade da mensagem de aridade do
`for` (Array e Dict).

### Implementação

- **`control_flow.rs` `eval_for`**: extraído `run_for_loop(items: Vec
  <Value>, loop_expr, scopes, ctx, engine)` — o corpo do antigo braço
  `Value::Array` (bind de padrão, corpo, `break`/`continue`/`return`),
  inalterado, agora reaproveitado por dois braços:
  - `Value::Array(items)` → `run_for_loop(items, ...)` directo.
  - `Value::Dict(dict)` (**novo**) → `dict.into_iter().map(|(k, v)|
    Value::Array(vec![Value::Str(k), v])).collect()`, depois
    `run_for_loop(items, ...)` — mirror de `IntoValue for (&Str,
    &Value)`. Zero código de bind novo: o braço `bindings.len() == 1`
    vs `> 1` já existente em `run_for_loop` cobre "um nome liga o par"
    e "dois nomes destroem", automaticamente.
- Nenhuma mudança a `destructure_pattern`/`Scopes` (P715/716) —
  mecanismo de `for` é independente (`Pattern::bindings()` acha lista
  plana, não usa `destructure_pattern` recursivo).

### Critérios de verificação

- Os snippets da tabela acima, cada um com o resultado medido.
- Ordem de inserção preservada (`z a m`, não alfabética).
- Dict vazio (`(:)`) → zero iterações, sem erro.
- Sem regressão: iteração sobre array (P540, incluindo `.enumerate()`)
  e `cargo test --workspace`.
- `crystalline-lint .` limpo.

## §P723 — `for` delega o binding ao destructuring genérico (spread `..sink`)

Isolado por P723 via `cetz` (segundo bloqueio da sonda do passo, após
`assert.eq`): `for (kind, ..args) in segments` errava `"cannot destructure
4 values into 2 bindings"` (cristalino, `control_flow.rs` P540). O item é
um segmento de path de 4 elementos (`("c", p1, p2, p3)` — curva cúbica) e
o padrão usa **spread** para recolher os pontos. Consumidor real:
`path-util.typ:106` (pacote `@preview/cetz:0.5.2`), no caminho de
`line`/`circle` do documento de reprodução da cadeia.

A premissa do passo (namespace de `curve`) foi **refutada** pela sonda —
o namespace existe desde P513; o bloqueio do `for` foi identificado com
build instrumentado temporário (mensagem de erro com repr do item e
bindings), revertido a seguir. Achado lateral medido: `curve(...)`
compila mas renderiza página em branco — bug de render separado, fora
deste passo (registado em `achados-adiados-cetz.md`).

### Mecanismo vanilla confirmado (`typst-eval/flow.rs:114-162`)

`ast::ForLoop::eval` chama **`destructure(vm, pattern, value)`** por item
(flow.rs:128) — o destructuring genérico de `binding.rs` (o mesmo de
`let` e atribuição), **com suporte a `..sink`**. Não há lógica de bind
própria do `for`: um `Pattern::Normal`/`Placeholder` liga/descarta o item
inteiro; `Pattern::Destructuring` consome posicionais e o spread absorve
o resto (`sink_size = 1 + len - item_count`); aridade errada →
`wrong_number_of_elements` (`binding.rs:180-209`) — `"too many elements
to destructure"` / `"not enough elements to destructure"` + hint
(`"the provided array has a length of {len}, but the pattern expects
{expected}"`).

Comportamento medido (cristalino antes do fix, vanilla em `f571a3644`):

```
for (kind, ..args) in (("c", 1, 2, 3),) → cristalino: Err cannot
    destructure 4 values into 2 bindings
    vanilla: kind="c", args=(1, 2, 3)
for (a, b) in ((1, 2, 3),)              → cristalino: Err cannot
    destructure 3 values into 2 bindings
    vanilla: Err too many elements to destructure + hint
for (a,) in ((5,), (6,))                → cristalino: a liga o array
    inteiro (bindings.len()==1 define directo, sem validar o padrão)
    vanilla: a=5, a=6 (destructuring de tuplo de 1 elemento)
```

### Divergência pré-existente fechada neste passo

A nota de §P719 ("mensagem de aridade do `for` diverge do vanilla",
P540) fica **fechada**: a delegação faz o `for` produzir exactamente as
mensagens de `wrong_number_of_elements` (já mirror do vanilla desde
P715), com hint. Item correspondente em `achados-adiados-cetz.md`
marcado como fechado.

### Implementação

- **`control_flow.rs` `run_for_loop`**: o bloco manual de bind
  (`bindings.is_empty()` / `bindings.len() == 1` define directo / resto
  destrói posicionalmente com mensagem própria) é **substituído** por
  `destructure_let(loop_expr.pattern(), item, scopes, ctx, engine)?` —
  a mesma entrada usada pelo `#let`. O `let bindings = ...` deixa de
  existir.
- **`bindings.rs` `destructure_let`**: passa a `pub(super)` (era
  privada) — única mudança de visibilidade.
- Cobertura automática: `for x in arr` (Normal → define), `for _ in
  arr` (Placeholder → descarta), `for (k, v) in dict` (pares
  `Array([k,v])` de §P719 → destructure_array), `for (a, ..rest) in
  arr` (**novo** — spread), `for (a,) in ((1,),)` (**corrigido** —
  destrói o tuplo de 1 elemento).
- Mudança de comportamento aceite (paridade com o vanilla, medido
  acima): pattern destructuring vazio `for () in (1,)` passa a errar
  (`cannot destructure integer`) em vez de ser no-op — o vanilla valida
  o pattern contra cada item.

### Critérios de verificação

- `for (kind, ..args) in (("c", 1, 2, 3),)`: `kind == "c"`,
  `args == (1, 2, 3)` (teste E2E).
- `for (a, b) in ((1, 2, 3),)` → erro `"too many elements to
  destructure"` (mensagem vanilla, não a antiga do cristalino).
- `for (a,) in ((5,), (6,))` → `a == 5, 6` por iteração.
- Sem regressão: `for x in arr`, `for (k, v) in dict` (§P719),
  `.enumerate()`, e `cargo test --workspace`.
- `crystalline-lint .` limpo; `--fix-hashes` actualiza os dois headers
  (`control_flow.rs`, `bindings.rs`).

## §P739C — display de Float em interpolação de markup

Medição prévia (ADR-0108; vanilla em `f571a3644`, cristalino no commit
base de P739): na conversão de valor embedded para texto em markup
(braço P545 de `eval/mod.rs`), o cristalino usava `repr_value` para
todos os tipos — `#(1.0)` renderizava `1.0`, `#(4/2)` renderizava
`2.0`. O vanilla usa o **Display de f64** (inteiros exactos sem `.0`):
`#(1.0)` → `1`, `#(4/2)` → `2`, `#(2.5)` → `2.5`, `#(1.5e3)` →
`1500`. Divergência de **língua** (morfologia do texto produzido),
fechada neste passo.

### Regra

- No braço `other` da interpolação de markup, `Value::Float(f)`
  converte-se com `format!("{f}")` (Display de f64 do Rust — paridade
  com o vanilla). Os restantes tipos mantêm `repr_value`.
- **`repr` inalterado**: `repr(1.0)` continua `"1.0"` (o observável do
  `repr` é outro — forma literal — e já está em paridade).

### Critérios de verificação

- `#(1.0)` → texto `1`; `#(4/2)` → `2`; `#(2.5)` → `2.5`;
  `#(1.5e3)` → `1500`; `#(0.1)` → `0.1` (testes + E2E `pdftotext`
  comparado com o vanilla).
- `#repr(1.0)` → `1.0` (não-regressão).
- `crystalline-lint .` limpo; `--fix-hashes` actualiza `eval/mod.rs`.

## §P740A — Warning "this return unconditionally discards the content before it"

Medição prévia (ADR-0108; sonda P740A): `#{ [conteúdo]; return "x" }`
no vanilla emite **duas** mensagens — o erro "cannot return outside of
function" **e** a warning "this return unconditionally discards the
content before it" + hint "try omitting the `return` to automatically
join all values"; com um update de state/counter no conteúdo descartado,
junta um segundo hint "state/counter updates are content that must end
up in the document to have an effect". O cristalino só emitia o erro —
a warning era o único consumidor do flag `conditional` de P729
(`code.rs:413-430` do vanilla, `warn_for_discarded_content`).

### Regra (mirror de `warn_for_discarded_content`)

No braço `Expr::CodeBlock`, após o loop de join (P728): se
`ctx.flow == Some(FlowEvent::Return(span, Some(_), false))` (return
incondicional com valor) **e** o output acumulado é `Value::Content`,
emitir a warning via canal tracked do Sink (`warn_note2`, P740A —
dois hints). O segundo hint dispara quando a travessia do conteúdo
encontra `State`/`StateUpdate`/`CounterUpdate`/`CounterDisplay`/
`CounterDisplayCallback` (paridade do seletor `State|Counter` do
vanilla; desce em `Sequence` e `Styled`).

A emissão acontece **antes** do chamador tratar o flow — a warning
coexiste com o erro "cannot return outside of function" (medido no
vanilla). `{ 1; return "x" }` (join não-Content) não emite.

### Critérios de verificação

- `#{ [conteúdo]; return "x" }` → warning + hint base (teste com sink
  exposto; E2E confirma a impressão do warning pelo binário).
- Com `state(...).update(...)` no conteúdo → os dois hints.
- `{ 1; return "x" }` → sem warning (não-regressão).

## §P740C — Ordem entre tipos no erro de argumento extra — scope-out reforçado (custo medido)

Sonda P740C (confirma P733): `#let f(a) = a; f(1, z: 2, 3)` → vanilla
reporta `"unexpected argument: z"` — o primeiro extra na **ordem
original** da lista única `Args.items` (`Arg { name, value }`);
o cristalino reporta o posicional primeiro (`"unexpected argument"`),
porque guarda `items: Vec<Value>` e `named: IndexMap` separados, sem
ordem intercalada.

**Decisão: scope-out reforçado.** A paridade exacta exigiria migrar
`Args` para lista única — custo medido (ADR-0108): 335 usos de
`args.items`, 676 usos de `.named` em 26 ficheiros da stdlib, mais 31
construções directas de `Args {}`. Desproporcional para um caso de
canto cosmético (dois extras de tipos diferentes; ambos os compiladores
erram, só difere qual é reportado). O item permanece em
`achados-adiados-cetz.md` com este custo registado.

## §P772n — `cannot_mutate_constant`: bootstrap deixa de achatar a stdlib em `top`

Sonda P772n mediu: vanilla (`foundations/scope.rs:63-70`,
`Scopes::get_mut`) recusa mutar um nome que só existe em `base.global`
(stdlib) — mensagem uniforme `"cannot mutate a constant: {name}"`,
**sem variação por contexto** (confirmado com `calc`/`image`/`table`/
`std`, todos idênticos). Um `#let calc = 5` local, que sombreia o nome no
`top`/`scopes`, torna-o um binding **normal**, livremente mutável — sem
erro (medido: `#let calc = 5; #{ calc = 10 }` compila no vanilla).

Mecanismo vanilla: **estrutural**, não uma flag por-binding.
`Scopes::get_mut` só pesquisa `top`/`scopes` — nunca `base`. A mensagem
"cannot mutate a constant" vs "unknown variable" vem de uma verificação
extra só no caminho de erro: se o nome existe em `base.global` (ou é
literalmente `"std"`), é constante; senão é desconhecido.
`BindingKind`/`Capturer` (P772l §2.2) é um mecanismo **diferente**, só
para variável capturada por closure/`context` — **não** está envolvido
aqui, e não foi implementado neste passo (decisão explícita, ver relatório
`paridade-producao-p772n.md`).

Causa raiz no cristalino (`61b7edee7`, antes desta correcção): o
bootstrap do avaliador (`eval/mod.rs::eval_with_full_error::run_pass`,
espelhado em `eval/modules.rs::eval_imported_file`) fazia
`scopes.define(name, ...)` para *cada* item da stdlib, cores predefinidas,
`std`, `text` e elementos de utilizador — todos a aterrar em `scopes.top`,
seguido de `scopes.enter()` para abrir um novo `top` para o corpo do
documento. Isto tornava a stdlib **indistinguível** de bindings normais:
`get_mut("calc")` encontrava-a no frame de `scopes` (empurrado pelo
`enter()`) e mutava-a sem erro. `Scopes.base: Option<&'a Library>` já
existia (`rules/scopes.md`) mas era sempre `None` em todos os call sites
— `Library` (`world-types.md`) era um stub opaco `()` desde a criação
(Passo 4/5, nunca completado).

### Correcção

1. `Library` (`world-types.md`) ganha um campo `global: Scope`.
   `Library::new()` mantém-se (produz `global` vazio) — preserva os ~30
   mocks de `World` em testes não relacionados com eval/stdlib.
   `Library::with_global(scope)` é o construtor novo, usado só no
   bootstrap real.
2. `eval/mod.rs`/`eval/modules.rs`: em vez de `scopes.define(...)` +
   `scopes.enter()`, constrói um `Scope` local (`global`) com a mesma
   sequência de definições de sempre, embrulha-o em
   `Library::with_global(global)`, e chama
   `Scopes::new(Some(&library))`. Sem `enter()` inicial — o `top` fica
   directamente disponível para o corpo do documento/módulo importado
   (o `scopes.exit()` final continua a funcionar sem alteração: sem
   `scopes` empurrado, `pop().unwrap_or_default()` devolve `Scope`
   vazio, e o `top` do documento é devolvido como sempre).
3. `Scopes::get` (`rules/scopes.md`) deixa de ter `base` como stub —
   consulta real `base.global.get(name)` como último recurso.
4. `Scopes::is_constant(name) -> bool` (novo, `rules/scopes.md`) — true
   sse `name` só é alcançável via `base`.
5. `eval/bindings.rs::access`, braço `Expr::Ident` em mutação: quando
   `get_mut` falha, `is_constant` decide entre as duas mensagens.

### Critérios de verificação

- `#{ calc = 5 }`, `#{ image = 5 }`, `#{ table = 5 }`, `#{ std = 5 }` →
  `error: cannot mutate a constant: {name}`, span no identificador.
- `#let calc = 5; #{ calc = 10 }; #calc` → sem erro, `calc` mostra `10`
  (sombra local continua mutável).
- `#{ zzz = 5 }` (nome nunca definido) → continua `unknown variable: zzz`
  (não regride).
- Ficheiro importado (`#import`) com `#{ calc = 5 }` no seu próprio
  top-level → mesmo erro (mesma correcção em `eval_imported_file`).

## §P772l — `CodeBlock`/`ContentBlock` não isolavam bindings de `let` (fuga de âmbito)

Sonda P772l (varredura `foundations::scope`) mediu, com `mutool`/`pdftotext`
em documento real:

```
#let x1 = 1
Bloco: #{ let x1 = 2; x1 }
Depois: #x1
```

Vanilla (`typst-eval/src/code.rs:317-332`, `ast::CodeBlock::eval` e
`ast::ContentBlock::eval`): ambos chamam `vm.scopes.enter()` antes de avaliar
o corpo e `vm.scopes.exit()` depois — o bloco introduz um âmbito léxico
próprio. Saída: `Bloco: 2` / `Depois: 1`.

Cristalino (commit `61b7edee7`, antes desta correcção): `Expr::CodeBlock` e
`Expr::ContentBlock` em `01_core/src/engine/eval/mod.rs` construíam
`styles`/`show_rules` locais (Passos 94/95, P340) mas avaliavam o corpo
directamente no `scopes` do chamador — sem `scopes.enter()`/`exit()`. Um
`let` dentro do bloco mutava a entrada existente no âmbito do chamador (ou
criava uma nova lá) e **sobrevivia à saída do bloco**. Saída medida:
`Bloco: 2` / `Depois: 2` — a fuga também reproduzida em `#if cond { let x =
.. }` (o corpo do ramo é um `CodeBlock`) e em `#[ #let x = ..; .. ]`
(`ContentBlock`). `#while`/`#for` não tinham o problema — os seus corpos
(`control_flow::eval_while`/`eval_for`) já isolavam o âmbito por outro
caminho, não partilhado com `CodeBlock`/`ContentBlock`.

Este era **o mesmo mecanismo `Scopes::enter()`/`exit()`** já especificado e
testado em `rules/scopes.md`/`scopes.rs` (shadowing, `enter`/`exit`
simétrico) — o bug não estava na struct `Scopes`, estava na ausência da
chamada nos dois pontos de consumo em `eval/mod.rs`. Não introduz tipo,
dependência ou decisão arquitectural nova: usa a API já aprovada.

**Correcção**: `scopes.enter()` imediatamente antes do corpo (loop de
`CodeBlock` / `eval_markup` de `ContentBlock`) e `scopes.exit()`
imediatamente depois, espelhando literalmente o vanilla — incluindo não
limpar em caminho de erro (`?` salta o `exit()`, inofensivo porque aborta a
compilação, igual ao vanilla).

### Critérios de verificação

- `#let x = 1; #{ let x = 2; x }; #x` → `2`, depois `1` (não `2`, `2`).
- `#if true { let x = 2; x }` seguido de `#x` fora → mesmo padrão.
- `#[ #let x = 2; #x ]` seguido de `#x` fora → mesmo padrão.
- `#while`/`#for` — não regride (já isolavam correctamente).
- `cargo test --workspace` verde; `crystalline-lint .` zero violações.

## §P772q — mensagem correcta para mutação de variável capturada

P772l §2.2 mediu `#let x = 1; #let f() = { x = 2 }; #f()` → cristalino dava
`"unknown variable: x"`; vanilla dá `"variables from outside the function
are read-only and cannot be modified"` (`foundations/scope.rs:316-323`).
`#context { x = 2 }` dá a mensagem irmã, "...outside the context
expression...". P772n **não** cobre este caso — protege `base` por
exclusão estrutural de `Scopes.captured` do alcance de `get_mut`; a
variável capturada por closure/`context` é um mecanismo diferente no
vanilla.

### Mecanismo vanilla — diferente do de `base` (medido, não suposto)

`vm.scopes.get_mut(&self).and_then(|b| b.write().map_err(Into::into))`
(`typst-eval/src/access.rs:38-39`): `get_mut` **encontra** a variável
capturada normalmente (o vanilla insere o scope capturado numa camada
alcançável, não numa separada e sempre ignorada como o `captured` do
cristalino) — é `Binding::write()`, chamado **depois**, que falha ao ver
`kind == BindingKind::Captured(capturer)`. O `capturer` (`Function` ou
`Context`) é decidido no momento em que `CapturesVisitor` percorre o corpo
da closure/`context` **na definição**
(`typst-eval/src/call.rs:576` — `Capturer::Function`;
`typst-eval/src/code.rs:394` — `Capturer::Context`), não na chamada.

Replicar esta estrutura exigiria `kind: BindingKind` em `Binding`
(`entities/scope.md`) — schema ainda adiado por ADR-0017, e que P772n já
confirmou não ser necessário para `cannot_mutate_constant`. Para P772q,
manter `Binding` como está e replicar só o **observável** (ADR-0107): a
mensagem certa, por um mecanismo estruturalmente diferente do vanilla.

### Correcção

1. `Capturer` (novo enum, `entities/scope.md`): `Function` | `Context`.
2. `ClosureRepr.capturer: Capturer` (novo campo, `entities/func.md`) —
   `Capturer::Function` em `eval_closure_expr` (`Expr::Closure`,
   `closures.rs`); `Capturer::Context` na construção do
   `ContextBlockElem` (`Expr::Contextual`, `eval/mod.rs`).
3. `Scopes.captured_by: Option<Capturer>` (novo campo, `rules/scopes.md`)
   + `with_parent(parent, capturer)` (assinatura alterada — só um
   call site em produção, `apply_closure`) + `captured_by(name) ->
   Option<Capturer>` (novo método).
4. `apply_closure` (`eval/closures.rs`) propaga `closure.capturer` para
   `Scopes::with_parent`.
5. `eval/bindings.rs::access`, braço `Expr::Ident` em mutação: ordem
   `captured_by` → `is_constant` → `unknown_variable`.

### Critérios de verificação

```
#let x = 1
#let f() = { x = 2 }
#f()
  → error: variables from outside the function are read-only and cannot be modified

#let x = 1
#context { x = 2 }
  → error: variables from outside the context expression are read-only and cannot be modified

#let x = 1
#{ x = 2 }
#x
  → sem erro, "2" (mutação normal fora de closure não regride)

#{ calc = 5 }
  → error: cannot mutate a constant: calc (P772n não regride)
```

## §P772r — hint de subtracção em `unknown_variable`

P772l §2.4 mediu `#foo-bar` → `"unknown variable: foo-bar"` idêntico nos
dois compiladores, mas o vanilla acrescenta um hint que o cristalino não
tinha. Este L0 documentava essa ausência como o estado aceite (§P715,
acima) — substituído aqui pela especificação do hint.

### Heurística exacta (sonda, não suposição)

`foundations/scope.rs::unknown_variable` (linha 424-437):

```rust
fn unknown_variable(var: &str) -> HintedString {
    let mut res = HintedString::new(eco_format!("unknown variable: {var}"));
    if var.contains('-') {
        res.hint(eco_format!(
            "if you meant to use subtraction, \
             try adding spaces around the minus sign{}: `{}`",
            if var.matches('-').count() > 1 { "s" } else { "" },
            var.replace('-', " - ")
        ));
    }
    res
}
```

Condição: **qualquer** hífen no nome (`contains('-')`) — sem verificar se
as partes à volta do hífen são identificadores válidos ou nomes
conhecidos (medido: `#foo-bar-baz` com três partes nenhuma delas
definida ainda ganha o hint). Plural "signs" quando há mais de um
hífen; singular "sign" para um só. Sem hífen → sem hint, mensagem base
inalterada.

Esta é a **mesma função** usada pelo vanilla tanto para leitura
(`Scopes::get`) como para mutação (`Scopes::get_mut`, braço de fallback)
— por isso a correcção cobre os dois caminhos do cristalino
(`eval_expr`, `Expr::Ident` em `eval/mod.rs`; `access()`, `Expr::Ident`
em `eval/bindings.rs`) via um único helper partilhado.

### Correcção

`unknown_variable(span, name) -> SourceDiagnostic` (novo, `pub(super)`,
`eval/bindings.rs`, ao lado de `missing_key` que já usava
`SourceDiagnostic::with_hint` para outro caso) — mensagem base sempre;
hint condicional pela heurística acima. Chamado por:

- `eval_expr`, `Expr::Ident` (`eval/mod.rs`) — leitura.
- `access()`, `Expr::Ident`, braço final (após `captured_by`/
  `is_constant`, P772q/P772n) (`eval/bindings.rs`) — mutação.

### Critérios de verificação

```
#foo-bar
  → unknown variable: foo-bar
  → hint: if you meant to use subtraction, try adding spaces around the minus sign: `foo - bar`

#foo-bar-baz
  → unknown variable: foo-bar-baz
  → hint: ...around the minus signs: `foo - bar - baz`   (plural)

#simplyunknown
  → unknown variable: simplyunknown
  → sem hint

#{ foo-bar = 1 }
  → unknown variable: foo-bar
  → hint idêntico ao caso de leitura (mesmo helper, caminho de mutação)
```

---

## §P772y — `eval_math_expr` (`rules/eval/math.rs`): callees namespaced e args `Str` em `FuncCall` de modo math

### Contexto

P772y implementou `math.class(class, body)` (`rules/stdlib/structural.md`
§P772y). Ao validar `math.class("relation", sym.suit.heart)` dentro de
`$...$` (a sintaxe exacta que a feature precisa de suportar), a sonda
revelou dois gaps pré-existentes, ambos em `eval_math_expr`
(`Expr::FuncCall`, antes de P772y):

1. **Callee não-bare**: `let name = match call.callee() { Expr::MathIdent(ident)
   => ..., _ => return Ok(Content::Empty) };` — qualquer callee que não
   fosse um `Expr::MathIdent` simples (ex.: `math.class`, um
   `Expr::FieldAccess`) caía no `_` e desaparecia silenciosamente, sem
   erro. Medido: `$x math.class("relation", sym.suit.heart) y$` produzia
   página só com `x y` (sem erro de compilação).
2. **Args sempre `Content`**: o mecanismo P510 (chamadas a `Value::Func`
   do scope, usado por `bb(x)`/`bold(x + y)`) avalia **todo** argumento
   posicional/nomeado via `eval_math_expr` e embrulha em
   `Value::Content` — correcto para esses casos (todos os args são
   sempre conteúdo math), mas errado para `math.class`, cujo 1º
   argumento é uma `Str` (`"relation"`). Um literal string nesta posição
   não tem arm dedicado em `eval_math_expr` — cai no `_ => Ok(Content::Empty)`
   final, produzindo `Value::Content(Content::Empty)` em vez de
   `Value::Str`.

### Correcção

**`eval_math_callee`** (novo, `fn`, privado ao módulo) — resolve o callee
de uma `FuncCall` em modo math quando não é `MathIdent` bare:

```rust
fn eval_math_callee(
    scopes: &mut Scopes<'_>, ctx: &mut EvalContext, engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value>
```

- `Expr::FieldAccess(access)`: resolve `access.target()` recursivamente
  via `eval_math_callee`, depois faz field access no `Value` resultante —
  só `Value::Module` (scope) e `Value::Dict` (chave) são suportados (os
  casos relevantes para namespaces de função: `math.xxx`, `calc.xxx`);
  outros tipos produzem erro claro.
- `Expr::MathIdent(ident)`: resolve **directamente no scope**
  (`scopes.get(ident.get())`) — **não** delega ao `eval_expr` genérico
  (`rules/eval/mod.rs`), porque esse trata `Expr::MathIdent` como
  "fronteira deliberada" e devolve sempre `Value::None` (ver `## Fronteira
  deliberada`, acima) — delegar aqui reproduziria exactamente o bug
  original (`campo 'class' não existe em none`).
- outro `expr`: delega ao `eval_expr` genérico (cobertura futura,
  ex. `Expr::Ident` se um dia aparecer como target em modo math).

Em `Expr::FuncCall`, o braço `_` do match original (`call.callee()`)
passa a chamar `eval_math_callee`; se o resultado for `Value::Func`,
aplica-se o **mesmo mecanismo P510** (args avaliados, `apply_func`); caso
contrário, erro `"chamada em modo math espera função, recebeu {tipo}"`
em vez do antigo `Content::Empty` silencioso.

**`eval_math_arg_value`** (novo, `fn`, privado ao módulo) — usado **só**
no novo caminho de callee namespaced (não no P510 bare-ident original,
para não arriscar regressão em `bb`/`bold`/etc.):

```rust
fn eval_math_arg_value(
    scopes: &mut Scopes<'_>, ctx: &mut EvalContext, engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value>
```

`Expr::Str(s) => Value::Str(...)` directo; qualquer outro expr passa por
`eval_math_expr` e embrulha em `Value::Content` (comportamento antigo,
preservado).

### Scope-out explícito (registado, não silencioso — ADR-0108)

Esta correcção resolve **só** o caminho `math.class(...)` chamado como
`FuncCall` bare dentro de `$...$`. **Não** resolve dois gaps maiores,
mais gerais, descobertos na mesma sonda e deliberadamente deixados fora
deste passo (P772y é sobre espaçamento por `MathClass`, não sobre
resolução geral de identificadores/expressões em modo math):

1. **Bare `MathIdent` não resolve variável do utilizador**: `#let loves =
   math.class(...); $x loves y$` renderiza `loves` como texto literal
   (5 glifos `l`,`o`,`v`,`e`,`s`), não como o `Content` vinculado —
   `Expr::MathIdent` (arm principal de `eval_math_expr`, não o callee)
   só resolve símbolos Unicode (`ident_to_unicode`) e operadores
   `math` (`lookup_math_op`); qualquer outro nome vira sempre
   `Content::MathIdent(name)` (texto), mesmo que esteja vinculado no
   scope a um `Content`/`Value::Symbol`.
2. **`#expr` em modo math não faz splice**: `$x #loves y$`, `$x
   #sym.suit.heart y$`, `$x #heartcontent y$` — todos medidos a produzir
   página **sem nenhum item** para o valor interpolado (nem erro, nem
   conteúdo) — gap no caminho de interpolação `#` dentro de `$...$`,
   distinto dos dois acima.

Ambos medidos com `mutool trace` (P772y, mesma sonda) e confirmados
**pré-existentes** (reproduzidos com `#sym.suit.heart`/`#heartcontent`,
sem qualquer relação com `math.class`/`MathClassOverride`). Candidatos a
passo dedicado próprio — âmbito maior (afecta toda a interpolação de
variáveis em modo math, não só uma função), precisa de sonda e decisão
registada próprias.

### Critérios de verificação

```
$x math.class("relation", "z") y$
  → mesmo delta de posição x que "$x = y$" no mesmo contexto (THICK ambos
    os lados) — confirma callee resolvido, arg Str correcto, override
    aplicado ao espaçamento.

$x math.class("bad", "z") y$
  → Err "class(): 'bad' não é uma MathClass reconhecida"

$x sym.suit.heart y$   (sem #, bare field access, FORA do FuncCall)
  → ainda produz página vazia para o símbolo — não é um FuncCall, não
    passa por eval_math_callee; gap de bare-FieldAccess-fora-de-chamada
    não coberto por esta correcção (não é o caso de uso de math.class).
```

---

## §P780 — `Expr::MathIdent` bare resolve variável do utilizador em modo math

### Contexto

P772y §3.6 mediu, como efeito colateral da validação de `math.class()`,
que `#let loves = ...; $x loves y$` renderiza `loves` como 5 glifos
literais (`l`,`o`,`v`,`e`,`s`), não o `Content` vinculado. P772l §2.5 já
tinha catalogado este debt, ligado a P301
(`00_nucleo/diagnosticos/diagnostico-auto-lookup-math-passo-301.md` §A.5).
**Confirmado neste passo: é o mesmo debt** — não um achado novo — fechado
aqui.

### Sonda — mecanismo exacto do vanilla (medido, não assumido)

**Fronteira letra-única vs multi-carácter é decidida no LEXER, não no
eval.** `typst-syntax/src/lexer.rs:742-753`: ao consumir um identificador
em modo math, se o resultado for **um único grapheme**, o token emitido é
`SyntaxKind::MathText` (nunca passa por resolução de scope); só sequências
de **mais de um grapheme** tokenizam `SyntaxKind::MathIdent` (ou
`MathFieldAccess`). `engine/lexer/math.rs` (cristalino) já replica isto
**exactamente** (confirmado por leitura — `is_math_id_start`/
`is_math_id_continue`, mesmo `if last_index == 0 { MathText } else { ... }`)
— não há trabalho adicional a fazer aqui, a fronteira já vem resolvida
pelo lexer antes de `eval_math_expr` sequer ver o token.

`ast::MathIdent::eval` (vanilla, `typst-eval/src/math.rs:45-57`) chama
`vm.scopes.get_in_math(&self)` **incondicionalmente** — sem heurística
extra. `Scopes::get_in_math` (`foundations/scope.rs:75-92`): `top →
scopes (reverso) → base.math.scope()`, erro `unknown_variable_math` se
nada encontrado. **Local/utilizador tem prioridade absoluta sobre
símbolos/operadores conhecidos** — confirmado por compilação real:

```
#let sin = 42;    $sin$    → mostra "42" (não o operador sin)
#let alpha = [x]; $alpha$  → mostra "x" (não α)
```

### Correcção

`eval_math_expr` (`Expr::MathIdent`, arm principal — **distinto** do
caminho de callee que P772y já corrigiu em `eval_math_callee`) ganha um
passo 0, antes de tudo o resto:

```rust
if let Some(value) = scopes.get_local(name) {
    return Ok(value_to_display_content(value.clone()).unwrap_or(Content::Empty));
}
```

`Scopes::get_local` (novo, `engine/scopes.rs`, ver `scopes.md` §P780) —
top → scopes → captured, **sem** cair em `base` — paridade exacta do
fallback restrito de `get_in_math` (que cai em `base.math`, não
`base.global`). Não há fronteira letra-única/multi-letra a replicar
aqui: o lexer já garante que `ident` é sempre multi-grapheme quando esta
arm é alcançada.

Se não encontrado localmente: os passos 1 (`ident_to_unicode`) e 2
(`lookup_math_op`) mantêm-se inalterados, na mesma ordem. Se **nenhum**
dos três resolver: `Err(unknown_variable_math(...))` — substitui o
fallback pré-P780 ("manter como `MathIdent`", regressão pré-P301
preservada desde P303).

`value_to_display_content` (novo, `engine/eval/mod.rs`) — extraído do
bloco P545 de interpolação `#{expr}` em markup (comportamento **byte-
idêntico** preservado; markup passou a chamar a função em vez do match
inline). Conversão genérica `Value → Option<Content>`: `Value::Content`
passa directo, `Str`/`Symbol`/`State` convertem, `Counter`/`None`/texto
vazio → `None` (nada a mostrar), outros valores (`Int`/`Float`/...) via
`repr_value`. Paridade conceptual com `Value::display()` (vanilla,
`ExprExt::eval_display`), que o cristalino não tem como método unificado.

`unknown_variable_math(span, name, in_global)` (novo, `engine/eval/
math.rs`) — **distinto** de `unknown_variable` (P772r): hints diferentes,
medidos directamente contra o vanilla (`foundations/scope.rs:439-472`,
NÃO assumidos de `unknown_variable`):

| Caso | Hint(s) |
|---|---|
| `none`/`auto`/`false`/`true` | 1: "adicionar `#` antes: `#none`" |
| conhecido em `base.global`, não em math (`has_global`) | 3: "não disponível directamente em math"; "`#nome` em código"; "`std.nome` em math" |
| desconhecido de todo | 2: espaçar letras (`f o o`); citar como texto (`"foobarbaz"`) |

### Débito descoberto e corrigido no mesmo passo — símbolo `product`

A sonda expôs (via regressão de teste pré-existente,
`layout_prod_com_limites_nao_panica`, que usava `$product_(k=1)^n$` como
fixture) que `product` **não estava** em `ident_to_unicode`
(`engine/math/symbols.rs`), só `prod` (nome **não-canónico** — `codex`
`sym.txt:525` só define `product ∏`, não `prod`; confirmado: `$product$`
resolve ∏ no vanilla real, `$prod$` **erra** "unknown variable: prod").
Adicionado `"product" => Some("∏")` a par de `"prod"` (mantido por
compatibilidade retroactiva, remoção não avaliada — fora de âmbito).

Isto também **corrige o diagnóstico de P772y §3.6.3** — o glifo `♥`
(U+2665) NÃO tem um gap de cobertura de fonte; a causa era exactamente
este debt (`MathIdent` bare não resolvia `loves`, logo o body de
`math.class(...)` nunca chegava ao layout). Medido pós-correcção:
`#let loves = math.class("relation", sym.suit.heart); $x loves y$`
renderiza `♥` correctamente (`unicode="♥" glyph="heart"`). O gap real que
**persiste** é mais estreito: `$sym.suit.heart$` **bare** (field access
directo numa sequência math, fora de qualquer `FuncCall`/`MathIdent`) —
cai no `_ => Ok(Content::Empty)` genérico de `eval_math_expr`, caminho
ainda não coberto (nem por P772y nem por P780).

### Critérios de verificação

```
#let myvar123 = 5;    $myvar123$    → "5" (scope local resolve)
$foobarbaz$                          → Err "unknown variable: foobarbaz" + 2 hints
#let r = [nunca];     $r$            → símbolo itálico "𝑟" (letra única, lexer MathText, ignora binding)
#let str = 5;         $str$          → "5" (sombra local vence stdlib)
$str$ (sem binding local)            → Err "unknown variable: str" + 3 hints (in_global)
#let loves = math.class("relation", "z");  $x loves y$
  → mesmo delta de "x" que "$x = y$" (THICK, via math.class + resolução
    de variável combinadas)
```

---

## §P782 — splice de `#expr`/field-access bare em modo math

### Contexto

P772y §3.6.2 mediu, P780 reconfirmou como débito **distinto** do fechado em
P780 (`MathIdent` bare por *nome*): `#expr` dentro de `$...$` e field access
bare (`sym.suit.heart` sem `#`) caíam no catch-all final de
`eval_math_expr` (`_ => Ok(Content::Empty)`), descartados em silêncio, sem
erro. Casos confirmados: `$x #sym.suit.heart y$`, `$x #hc y$` (`hc`
vinculado a `Content`/`Value::Symbol`), `$sym.suit.heart$` bare.

### Sonda — mecanismo exacto do vanilla (medido, não assumido)

Vanilla tem uma distinção arquitectural que o cristalino **não replica**:
`Expr::MathFieldAccess` (`typst-syntax/src/ast.rs:296`) é um variant
**dedicado** do `Expr` genérico, produzido pelo lexer math para field
access bare (`MathAccess::MathIdent | MathAccess::MathFieldAccess`,
recursivo) — distinto de `#expr` (que produz um `Expr::FieldAccess`
genérico via `embedded_code_expr`, o mesmo caminho de código normal).
`ast::Math::exprs()` devolve o `Expr` genérico (`typst-syntax/src/
ast.rs:889-891`) — **qualquer** variant, específico de math ou não, passa
por `expr.eval_display(vm)` = `self.eval(vm)?.display()`
(`typst-eval/src/math.rs:176-184`) — conversão genérica `Value → Content`.

**Confirmado por leitura de fonte, não suposição**: no **cristalino**,
`engine/parse/math.rs:63-64` (`SyntaxKind::MathIdent | SyntaxKind::
FieldAccess => { ... }`, comentário "The lexer manages creating full
FieldAccess nodes if needed") — o lexer math monta directamente um nó
`SyntaxKind::FieldAccess` **genérico** (não um `MathFieldAccess` dedicado)
para `sym.suit.heart` bare. Isto significa que, nesta arquitectura
específica, `#sym.suit.heart` (via Hash) e `sym.suit.heart` bare produzem
**literalmente o mesmo** `Expr::FieldAccess` — ao contrário do vanilla,
onde são dois `Expr` variants distintos. Consequência prática: um único
braço de `eval_math_expr` cobre os dois casos nesta implementação (não é
uma suposição de que "é o mesmo fix" — é uma consequência estrutural
verificada da forma como o parser cristalino monta a árvore).

### Correcção

Catch-all final de `eval_math_expr` (antes `_ => Ok(Content::Empty)`)
substituído por dois braços:

```rust
other @ Expr::FieldAccess(_) => {
    let value = eval_math_callee(scopes, ctx, engine, other)?;
    Ok(value_to_display_content(value).unwrap_or(Content::Empty))
}
other => {
    let value = eval_expr(other, scopes, ctx, engine)?;
    Ok(value_to_display_content(value).unwrap_or(Content::Empty))
}
```

- `Expr::FieldAccess` passa por `eval_math_callee` (P772y, generalizado por
  este passo — ver docstring actualizada da função), **não** por
  `eval_expr` directo: o alvo do access (`sym` em `sym.suit.heart`) é
  lexado como `Expr::MathIdent` mesmo dentro do nó `FieldAccess` — `eval_
  expr` genérico trata `Expr::MathIdent` como fronteira deliberada
  (`Value::None`), o que faria o field access falhar com "field access não
  suportado em none" (medido — mesma causa-raiz que P772y já tinha
  contornado para o caminho de callee, agora reaproveitada aqui).
- Qualquer outro `Expr` (`Ident`, `LetBinding`, literais, ...) passa por
  `eval_expr` genérico directo — paridade conceptual com `eval_display`
  (vanilla). Seguro por construção: `Expr::MathIdent`/`MathText`/etc.
  nunca chegam a este catch-all (apanhados pelos braços específicos
  anteriores no match); os que chegam e não são reconhecidos por
  `eval_expr` (variantes markup-only) já devolvem `Value::None` lá
  ("Fronteira deliberada"), convertendo para `Content::Empty` aqui —
  mesmo resultado do comportamento antigo, sem regressão.

**`eval_math_callee`** (P772y) ganhou um braço `Value::Symbol` — `sym.
suit.heart` resolve `sym` (módulo), depois `.suit` (`Value::Symbol`, grupo
de variantes) e `.heart` (aplica modifier) — paridade `eval_field_access`
(P765a, `s.modified(field)`), duplicada aqui porque `eval_field_access`
recursa via `eval_expr(access.target())`, que falharia no mesmo alvo
`MathIdent`.

**Bónus não-planeado, descoberto pela mesma correcção**: `#let x = 5`
dentro de `$...$` antes caía no mesmo catch-all e **nunca mutava o
scope** (a atribuição nunca executava). Passa agora a executar de facto,
via `eval_expr`'s braço `Expr::LetBinding`. Não é regressão — é o
comportamento correcto que o catch-all antigo impedia.

### Critérios de verificação

```
$sym.suit.heart$              → ♥ (field access bare)
$x #sym.suit.heart y$         → x ♥ y (field access via #)
#let hc = sym.suit.heart; $x #hc y$   → x ♥ y (Ident via #, vinculado a Symbol)
$#let zval = 5; zval$         → 5 (LetBinding em math agora executa)
#let myvar123 = 5; $myvar123$ → 5 (não-regressão P780, MathIdent bare)
$undef$                        → Err "unknown variable: undef" (não-regressão P780)
```

**⚠ Correcção P825 (sub-B de P810 §12)**: o primeiro critério
(`$sym.suit.heart$` → ♥) foi **refutado por medição do vanilla 0.15.0** —
ver §P825 abaixo. A regra de P782 "bare e `#` produzem o mesmo
`Expr::FieldAccess`" mantém-se, mas o braço `Expr::MathIdent` de
`eval_math_callee` agora **rejeita módulos globais** (comportamento
vanilla), pelo que `$sym.suit.heart$` bare volta a ser erro.

## §P825 — módulos globais não são acessíveis bare em modo math (sub-B de P810 §12)

**Medição (sonda `temp/p825/b*.typ`, vanilla 0.15.0):** `$ math.class("relation", "x") $`,
`$ sym.suit.heart $`, `$ calc.gcd(4, 6) $`, `$ emoji.face $` → todos
`error: unknown variable: <mod>` no ident do alvo, com 3 hints verbatim:
```
hint: `<mod>` is not available directly in math, but is in the standard library
hint: to access `<mod>` in code mode you can add a hash: `#<mod>`
hint: or access `<mod>` in math mode by using the `std` module: `std.<mod>`
```
Continuam a funcionar bare (medido): funções expostas no scope math
(`class("relation", x)`, `mat(...)`, `lr(...)`, `text("hi")`), bindings de
utilizador (`#let d = 5; $ d $`, closures `#let f = (x) => x; $ f(1) $`) e
o módulo **`std`** (`$ std.math.class("relation", "x") $` compila).

**Regra (implementada em `eval_math_callee`, arm `Expr::MathIdent` —
reforço da validação de P782, não duplicação):** um `MathIdent` bare que
resolve para `Value::Module` global → `unknown_variable_math` (o erro já
existente, verbatim); excepção `std`. O caminho via `#` (target
`Expr::Ident` de código) não é afectado — cai no braço `other => eval_expr`
e resolve o módulo normalmente. Funções (`Value::Func`) e outros valores
não são afectados.

**Nota**: esta regra **revoga** o critério de P782 `$sym.suit.heart$` → ♥
(medição incorrecta da época — o vanilla rejeita). O teste
`p782_field_access_bare_resolve_simbolo` foi actualizado em P825 para
asserir o erro, com a razão registada.


## §P786a — Propagação integral de erros sintáticos (substitui a filtragem selectiva de P648/P649)

**Decisão:** `eval_with_full_error` propaga **todo** erro sintático do parser
(`root.errors()`) como `SourceDiagnostic::error` fatal — não mais apenas
`InvalidHexNumber`/`InvalidUnicodeCodepoint`. A filtragem selectiva de P648/P649
fica **revogada** (era mitigação contra falsos positivos do parser de P634).

**Evidência que legitima a mudança (ADR-0108, medida em 2026-07-20, commit
`0774275f`):** survey empírico de 20 construções válidas (probe
`p786a_probe_valid_constructs_error_nodes`) produziu **zero** falsos positivos.
O único caso suspeito — `#` dentro de bloco de código — é erro **genuíno**:
o vanilla 0.15.0 também o rejeita (`error: the character '#' is not valid in
code` + 2 hints, exit 1), enquanto o cristalino compilava com exit 0.
Os falsos positivos da era P634 (smart quotes, `#set` em blocos) já não
existem no parser actual.

**Roteamento por severidade (T1 do mesmo passo):** erros com
`SyntaxErrorKind::NoTextWithinStars | NoTextWithinUnderscores` (produzidos em
`engine/parse/markup.rs`, ver parse.md §P786a) **não abortam** — seguem para o
`sink` como `SourceDiagnostic::warning` com hint (paridade vanilla:
`warning: no text within stars` + hint, exit 0). Todos os demais kinds são
fatais. Hints dos `SyntaxError` são propagados via `SourceDiagnostic::with_hint`.

**Critérios de aceitação:**

- `#let x = (` → `Err` contendo `unclosed delimiter`; nenhum PDF gerado (exit 1 na CLI).
- `#let x = { #set text(fill: red) [body] }` → `Err` contendo `not valid in code`.
- `**` → eval `Ok` + warning `no text within stars` com o hint vanilla
  (`using multiple consecutive stars (e.g. **) has no additional effect`).
- `__` → eval `Ok` + warning `no text within underscores` análogo.
- A bateria de construções válidas do survey (smart quotes, `#set` top-level,
  math, destructuring, tabelas, referências, emoji, etc.) continua sem
  error nodes — guarda de não-regressão em `tests.rs`.

## §P815 — `eval_field_callee`: método inexistente e dict-key-call (achado #2 de P810)

**Decisão:** em `eval_func_call` (`engine/eval/closures.rs`), **depois** de
todos os despachos de método legítimos (P417/P423/P504/P717/P466/P702/P710/
P712/P742/P792/P796/P506/P707), um callee `target.field` chamado como função
cujo alvo não é `Symbol`/`Func`/`Type`/`Module` (os únicos que o vanilla
deixa chamar campos directamente — `call.rs:258-263`) produz os erros
verbatim do vanilla via `bindings::field_callee_error`
(`engine/eval/bindings.rs`), mirror do ramo de erro de `eval_field_callee`
(vanilla `typst-eval/src/call.rs:258-345`):

- **Campo inexistente** → `{kind} {name} has no method `{field}`` —
  `("type", long_type_name)` ou `("element", elem_name)` para content
  (mirror de `element_or_type_with_name`, `call.rs:359-365`). Ex.:
  `type integer has no method `foo``, `type array has no method `zzz``,
  `element strong has no method `zzz``.
- **Campo existente em dict** → `cannot directly call dictionary keys as
  functions` + hint `to access the `{field}` key, remove the function
  arguments: `{full_text}`` (ou `to call the stored function, wrap the field
  access in parentheses: `({full_text})(..)`` se o valor for função) + hint
  `dictionary keys cannot be used with method syntax as keys could conflict
  with built-in method names`. Efeito medido: funções guardadas em dict keys
  deixam de ser chamadas directamente (o cristalino chamava-as — bug medido
  em P815); a forma `(d.f)(..)` continua válida.
- **Campo existente em args** → `cannot directly call named argument fields
  as functions` + 2 hints análogos (`argument` / `named arguments cannot…`).
- **Campo existente noutros tipos** (length `.abs`, content `.body`, …) →
  `` `{field}` is not a valid method for {kind} `{name}` `` + hint
  `to access the `{field}` field, remove the function arguments:
  `{full_text}``.

`full_text` = texto do nó FieldAccess (`access.to_untyped().clone().into_text()`).
O vanilla **não** usa distância de edição nestas mensagens (`call.rs:339-340`
— decisão deliberada da fonte); a hipótese do prompt de P815 está refutada
pela fonte e pela sonda. As variantes de hint de math mode (`in_math`,
`call.rs:310-311`) ficam scope-out — o cristalino não tem flag de contexto
math no eval; medido que `$#d.x()$` no vanilla usa os mesmos hints não-math.

**Critérios de aceitação (medidos no vanilla, verbatim):**

- `#(1).foo()` → `type integer has no method `foo``; idem float/array/string.
- `#let d = (x: 1)\n#d.x()` → `cannot directly call dictionary keys as
  functions` + os 2 hints acima.
- `#let d = (f: x => x*2)\n#d.f()` → mesmo erro + hint `to call the stored
  function, wrap the field access in parentheses: `(d.f)(..)``.
- `#d.zzz()` (chave ausente) → `type dictionary has no method `zzz``.
- `#(10pt).abs()` → `` `abs` is not a valid method for type `length` `` + hint.
- `#strong[x].body()` → `` `body` is not a valid method for element `strong` ``
  + hint; `#strong[x].zzz()` → `element strong has no method `zzz``.
- Controlos: métodos reais (`at`/`len`/`insert`/…), `#"ab".push("c")`
  (`cannot mutate a temporary value`), `(d.f)(21)` → `42`, e `#d.x` sem
  parênteses ficam intactos.

## §P829-B — métodos de `content`: `func`/`has`/`at`/`fields`/`location`

**Decisão:** os cinco métodos do `#[scope]` de `Content` do vanilla
(`foundations/content/mod.rs:510-590` — a lista é exaustiva, confirmada na
fonte) existem no cristalino via `bindings::eval_content_method`
(`engine/eval/bindings.rs`), despachados numa intercepção de `eval_func_call`
(`engine/eval/closures.rs`) **antes** do fallback P815 e depois de todos os
despachos legítimos. Assinaturas e retornos replicam o vanilla (medido em
`temp/p829/b*.typ`):

- `func()` → a função do elemento (`Value::Func::native(elem_name, ctor)`).
  Igualdade por nome (P742) ⇒ `strong[x].func() == strong` → true (medido).
  Variantes sem constructor nativo exposto (Sequence, Styled, Label, math,
  Dynamic, …) devolvem `Func` com o nome do elemento e um ctor fallback que
  erra `calling this element function is not supported` — caso **não medido**
  no vanilla (elementos internos), mensagem própria.
- `has(field)` → true só para campo **assente no constructor** (ver
  `entities/elements/heading.md` §P829 — máscara `set_fields`).
- `at(field, default:?)` → valor do campo assente; não assente →
  `default` ou erro verbatim `field "{f}" in {elem} is not known at this
  point and no default was specified`; inexistente → `default` ou
  `{elem} does not have field "{f}" and no default was specified`.
- `fields()` → dict dos campos assentes, na ordem de declaração do vanilla
  (heading: `level, depth, outlined, bookmarked, body` — medido
  `(level: 2, body: [H])`, `(depth: 1, body: [H])`).
- `location()` → sempre `none`. Medido: content inline → none nos dois
  binários. **Divergência registada:** content fornecido por show rule/query
  tem `location(..)` no vanilla; o cristalino não retém metadados de location
  em `Content` (requer introspecção de locations — desproporcional, fora do
  passo).

Erros de argumentos verbatim (medidos b13–b17): `missing argument: field`,
`expected string, found {tipo}`, `unexpected argument`,
`unexpected argument: {nome}`.

**Scope-outs registados (medidos):** `label` em `has`/`fields` (no cristalino
a label é nó irmão `Content::Label`, não metadado — modelo diferente do
vanilla); `numbering`/`offset`/`supplement` de heading (não modelados no
`HeadingElem` — `has` devolve false como o vanilla para campos não assentes,
mas `at` sem default diverge na mensagem: `{elem} does not have field` vs
`field ... is not known at this point`); `strong(delta:)` (a native não
aceita named — scope-out pré-existente; `delta` conta como declarado-não-
assente para as mensagens).

## §P829-C — despacho de erro de chamada em modo math

**Decisão:** em `eval_math_expr` (`engine/eval/math.rs`, braço `FuncCall`
com callee não-`MathIdent`), um callee `Expr::FieldAccess` avalia o target
**uma única vez** (via `eval_math_callee`, como já fazia) e passa por
`bindings::field_callee_error` (P815) **antes** de resolver o campo — o
vanilla usa a MESMA rotina de chamada dentro e fora de math
(`call.rs:eval_field_callee`; medido c1–c4 em `temp/p829/`). Alvos
`Symbol`/`Func`/`Type`/`Module` devolvem `None` e seguem o caminho normal
(`math.class`, `sym.suit`, … intactos). Isto substitui a mensagem própria
`chamada em modo math espera função, recebeu {tipo}` para callees
`target.field`; elimina também o bug de chamar funções guardadas em dict keys
dentro de math (`$#d.f()$` — medido: o cristalino chamava-a). O scope-out de
P815 das variantes de hint `in_math` fica **superado** por esta secção (o
vanilla usa os hints não-math dentro de math — confirmado c1).

**Critérios (verbatim, medidos):** `$#d.x()$` →
`cannot directly call dictionary keys as functions` + 2 hints;
`$#d.zzz()$` → `type dictionary has no method `zzz``;
`$#d.f()$` (função guardada) → mesmo erro + hint `(d.f)(..)`;
`$#(1).foo()$` → `type integer has no method `foo``; controlo `$#d.x$`
(sem chamada) intacto.

## §P829-D — campos vs métodos em `arguments`/`array` — scope-out formal (decisão do dono pendente)

**Medição** (fixtures `temp/p829/d*.typ`, os dois binários — relatório P829):

| caso | vanilla | cristalino |
|---|---|---|
| `arr.len` (sem parênteses) | erro `cannot access fields on type array` | `3` (campo P493a) |
| `arr.first`/`arr.last` | erro idem | valores (campo P493a) |
| `args.positional` | erro `no named argument "positional"` | array (campo P504) |
| `args.named` (sem parênteses) | erro `no named argument "named"` | dict (campo P504) |
| `args.key` (named arg como campo) | `42` | erro `arguments does not contain field "key"` |
| `args.at("key")`/`args.at(0)` | valor | erro `type arguments has no method `at`` |
| `args.pos()`/`args.named()` | funciona | funciona (P707, paridade ✓) |

Métodos de `arguments` do vanilla (`args.rs`): `len`, `at`, `pos`, `named`,
`filter`, `map` — o cristalino tem `pos`/`named` (P707); `len`/`at`/
`filter`/`map` já eram scope-out de P707 ("sem consumidor medido").

**Scope-out formal (padrão P807/P812-C):** converter os campos P493a/P504 em
métodos (ou removê-los) é uma alteração **estrutural e desproporcional** ao
ganho medido (paridade de mensagem de erro em casos de borda — m12, d1–d4):
(1) reverte duas decisões de L0 vigentes (campos de `arguments` — P504, ver
§P707 acima; campos de `array` — P493a, `engine/eval/field-access.md` §4) —
pela regra do repositório, conflito com L0 vigente é decisão do dono, não
opção do executor; (2) os campos estão fixados
por testes deliberados (`p504_arguments_positional_field`,
`p707_arguments_pos_e_named_nao_regridem_campos`) e pelo caso de paridade
`args_field` (`lab/parity/tests/structural_parity.rs:1891`); (3) documentos
que usem `arr.len`/`args.positional` como campo passariam a erro. A direcção
inversa (named-arg-como-campo `args.key`, `args.at`/`len`/`filter`/`map`)
seria aditiva mas é trabalho novo com L0 próprio. **Não implementar
parcialmente** — aguarda decisão do dono (manter divergência consciente ou
abrir passo dedicado).

## P836 — `#set text(variations:)` 

`variations` entra em `VANILLA_TEXT_SET_PROPS` (campo `#[fold] #[ghost]`
settable no vanilla — medido em `temp/p836/s2_setrule.typ`, exit 0) e
ganha braço no dispatch de `eval_set_rule`: o valor é validado por
`FontVariations::from_value` (mensagens/hints do vanilla, ver
`entities/font_variations.md`) e empurrado para a chain via
`push_custom("text.variations", dict)`, como os demais campos
`text.<campo>` do F-5b (P373). O resolver
`StyleChain::variations()` aplica o fold por tag entre níveis.

---

## P844 (achados #49/#50 de P831) — dispatch de métodos `at`/`final`

- `eval_state_method` ganhou os braços `at` (nova `state_at_dispatch` — `Location` directa ou `<label>` via introspector; mensagens verbatim medidas no vanilla 0.15.0) e `final`. `eval_counter_method_value` ganhou o braço `final` e o braço `at` passa a aceitar `Value::Location` além de label/string; a helper P506 `extract_label_from_args` foi absorvida pelo braço e removida. Validação de argumentos precede o gate de contexto (ordem medida no vanilla).
