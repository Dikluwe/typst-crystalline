# Prompt L0 — rules/eval
Hash do Código: b06b1007

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/mod.rs`
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

1. `make_stdlib()` — todas as funções nativas (`type`, `len`, `rgb`, `table`, etc.).
2. `predefined_color_bindings()` — atalhos `red`, `blue`, `green`, `black`, `white`,
   `yellow`, `cyan`, `magenta`, `none` (P492/P497).
3. `text` — função nativa `native_text` exposta globalmente para uso em show-rules
   (P492; ex.: `#show regex("\\d+"): it => text(red, it)`).
4. Elementos de utilizador registados no `ElementRegistry`.

O scope base é depois herdado por closures e show-rules.

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
- `Expr::CodeBlock` → evalua exprs sequencialmente via `body().exprs()`
- `Expr::Binary(binary)` → eval_binary_op(binary.op(), lhs, rhs)
- `Expr::Unary(unary)` → eval_unary_op(unary.op(), operand)
- `Expr::Conditional(cond)` → eval_conditional: condition(), if_body(), else_body()
- `Expr::WhileLoop(loop)` → eval_while: MAX_ITER=10_000 limite de segurança
- `Expr::ForLoop(loop)` → eval_for: iterable() (não iter()), pattern().bindings()
  (incluindo destructuring de tuplo: `(i, x)` atribui posicionalmente de cada
  item `Value::Array`), body(); cada iteração avalia o corpo e concatena os
  valores `Content`/`Str` produzidos numa `Content::sequence`; `Value::None`
  no corpo é ignorado; `Value::None` como iterable é iterável vazio (sem
  parsing de array literal)

## §P635 — Mecanismo `FlowEvent` para controlo de fluxo

O cristalino implementa o mecanismo `FlowEvent` do vanilla
(`lab/typst-original/crates/typst-eval/src/flow.rs:14-22`) para que
`#break`, `#continue` e `#return` afectem de facto o fluxo de execução.

### Tipo e localização

Criar `01_core/src/rules/eval/flow.rs` em L1 com:

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
- `00_nucleo/prompts/rules/stdlib.md` §"Política IEEE 754" — sítio
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
- O layout (`rules/layout/text.rs`) lê `"text.style"` e traduz `"italic"`/`"oblique"` para `italic = true`, coexistente com o campo tipado `italic`.

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

Semântica a implementar em `eval_module_import` (`01_core/src/rules/eval/modules.rs`),
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

Semântica a implementar em `eval_module_import` (`01_core/src/rules/eval/modules.rs`):

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
