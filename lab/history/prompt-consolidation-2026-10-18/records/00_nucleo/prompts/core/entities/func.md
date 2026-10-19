# Prompt L0 — `entities/func` — funções e aplicação parcial
Hash do Código: 2c6a8035

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/func.rs`
Args é propriedade de `entities/args.md`.
**ADRs relevantes**: ADR-0016 (adiamento Routines), ADR-0017 (adiamento eval completo), ADR-0107 (paridade linguagem), ADR-0109 (atomização)

## Contexto

`Func` representa uma função Typst — closures definidas no documento, funções nativas (built-ins) e construtores de elemento de utilizador. `Args` representa os argumentos de uma chamada de função.

## Tipos públicos

### Func

```rust
#[derive(Clone)]
pub struct Func(pub(crate) Arc<FuncRepr>);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    Native(NativeFunc),
    /// P394 — native function com acesso ao `Scopes` e `Engine` actuais.
    NativeWithEngine(NativeFuncWithEngine),
    /// Lote F-3 inc-2 — construtor de elemento de utilizador.
    Element(ElementFunc),
    /// **P699** — função de plugin WASM. O nome do export é dinâmico, logo não
    /// cabe num fn-ptr nativo (`Native`/`NativeWithEngine`); a chamada é
    /// delegada ao `PluginHost` capturado. Ver `entities/plugin_func.md`.
    Plugin(crate::entities::plugin_func::PluginFunc),
    /// **P702** — aplicação parcial de argumentos (`f.with(...)`). Envolve a
    /// função original e os `Args` pré-ligados; ver secção "Variante `With`".
    With(Arc<(Func, Args)>),
}

pub struct ClosureRepr {
    pub name: Option<String>,
    pub params: Vec<ClosureParam>,
    pub body: SyntaxNode,           // clone O(1) via Arc interno
    pub captured: Arc<Scope>,       // eager snapshot do scope no momento da definição
    /// **P772q** — por que motivo este scope foi capturado: closure normal
    /// (`Expr::Closure`, `eval/closures.rs::eval_closure_expr`) ou bloco
    /// `context { }` (`Expr::Contextual`, `eval/mod.rs`). Paridade vanilla
    /// `CapturesVisitor::new(scopes, capturer)` — o capturer é decidido no
    /// momento da captura (definição), não da chamada. Usado por
    /// `apply_closure` para propagar a `Scopes` da chamada
    /// (`entities/scope.md#Capturer`).
    pub capturer: Capturer,
}

pub struct ClosureParam {
    pub name:    String,
    pub default: Option<Value>,
    /// **P724** — pattern completo de `Param::Pos` não-`Ident`
    /// (destructuring, parenthesized, placeholder): `SyntaxNode` owned
    /// (clone O(1) via Arc interno), reparseado na chamada via
    /// `Pattern::from_untyped` e ligado por `destructure_let` — o mesmo
    /// mecanismo do `body` (`Expr::from_untyped`). `None` para `Ident`
    /// posicional e para `Param::Named`. Quando `Some`, `name` é `""`
    /// (nunca consultado: named lookup com chave vazia não casa) e
    /// `default` é `None` (pattern posicional nunca tem default).
    pub pattern: Option<SyntaxNode>,
}
```

**Extensão (P724 — parâmetros com padrão de desestruturação)**: isolada
por P723 via `cetz` (`path-util.typ:453` —
`segments.enumerate().filter(((i, segment)) => ...)`): `eval_closure_expr`
descartava silenciosamente `Param::Pos` com pattern não-`Ident` (braço
`_ => None`), criando closures com parâmetros a menos que depois
rejeitavam o argumento (`unexpected argument`, P708). O vanilla
(`typst-eval/src/call.rs:655-665`) liga `Param::Pos` não-`Ident` via
`destructure` — a mesma entrada genérica de `let`/`for`. Medido no
vanilla:

```
#let f = ((a, b)) => a + b;  f((1, 2))       → 3
#let f = ((a, b), c) => a+b+c; f((1, 2), 3)  → 6   (mistura com posicional)
#let g(x, (a, b)) = x+a+b;    g(10, (1, 2))  → 13  (definição nomeada)
#let f = (_, y) => y;         f(1, 2)        → 2   (placeholder consome)
#let f = ((a, ..rest)) => rest.len(); f((1,2,3)) → 2  (spread no pattern)
```

A invariante P708 mantém-se intacta: `pattern` só existe para
`Param::Pos` (`default: None`); o discriminante posicional vs
keyword-only continua a ser `default`. **Mudança de comportamento
aceite**: `(_, y) => ...` (placeholder) passa a consumir o posicional
(antes: `unexpected argument`) — paridade com o vanilla, medido acima.
Divergência residual registada (pré-existente, fora do scope): posicional
em falta liga `Value::None` em vez de `missing argument` do vanilla;
sobre um pattern, `destructure_let(None)` erra `cannot destructure none`
em vez de `missing argument: pattern parameter`.

**Invariante (P708 — antes implícita, agora documentada)**: `default` não é
só "valor por omissão" — é o **discriminante entre parâmetro posicional e
parâmetro nomeado-com-default (keyword-only)**, herdado da distinção já
existente no parser (`entities/ast/expr.rs::Param::Pos` vs `Param::Named`,
espelhando o vanilla, `typst-syntax/src/ast.rs:2078-2085`):

- `default: None` ⟺ construído a partir de `Param::Pos` — parâmetro
  posicional, nunca tem valor por omissão no vanilla.
- `default: Some(v)` ⟺ construído a partir de `Param::Named` — parâmetro
  **keyword-only**: só pode ser preenchido por nome; nunca por posição
  (medido contra o vanilla: `#let f(close: false) = close; f(true)` →
  `error: unexpected argument`, não `close = true`).

`eval_closure_expr` (`rules/eval/closures.rs`) só tem dois pontos de
construção de `ClosureParam` (`Param::Pos` → `default: None`; `Param::Named`
→ `default: Some(...)`) — a invariante é garantida na origem, não precisa
de um campo extra (`kind`) para ser fiável. **P708 corrigiu**
`apply_closure`, que até então ignorava esta invariante e deixava
parâmetros keyword-only consumirem posicionais quando não vinham por nome
(ver `rules/eval.md` §P708 para o algoritmo de binding corrigido).

```rust
/// Função nativa implementada em Rust.
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&mut crate::compiler::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId) -> SourceResult<Value>,
}

/// P394 — native function com acesso ao `Scopes` e `Engine` actuais.
pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(&mut crate::compiler::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId, &mut crate::compiler::scopes::Scopes<'_>, &mut crate::entities::engine::Engine<'_>) -> SourceResult<Value>,
}

/// Lote F-3 inc-2 — construtor de elemento de utilizador.
pub struct ElementFunc {
    pub name: String,
    pub ctor: crate::entities::element_registry::ElementCtor,
}
```

`Func(Arc<FuncRepr>)` — clone O(1), consistente com `Module`.

`ClosureRepr.captured` — eager snapshot do scope no momento da definição. Semântica: captura por valor, não por referência. Divergência do original (que usa `comemo` para lazy access) — registada em DEBT.md.

## Namespace anexado (P493)

**P493 — Field access em funções com namespace:** algumas funções nativas do Typst expõem sub-funções via field access (ex.: `table.header`, `table.footer`, `table.cell`). Para suportar isto, `Func` pode ter um namespace anexado.

### Representação

Adicionar a `FuncRepr::Native` e `FuncRepr::NativeWithEngine` um campo opcional `namespace: Option<Arc<Scope>>`:

```rust
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(...) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}

pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(...) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}
```

- `namespace: None` — função sem sub-funções (`heading`, `figure`, etc.);
- `namespace: Some(scope)` — função com sub-funções acessíveis via field access (`table`, `list`, `enum`, etc.).

### Construtores

- `Func::native(name, call)` — cria `Func` com `namespace: None`.
- `Func::native_with_namespace(name, call, namespace)` — cria `Func` com namespace anexado.
- `Func::native_with_engine(name, call)` — cria `Func` com `namespace: None`.
- `Func::native_with_engine_and_namespace(name, call, namespace)` — cria `Func` com namespace anexado.

### Acesso

```rust
impl Func {
    /// Retorna o namespace anexado, se existir.
    pub fn namespace(&self) -> Option<&Scope>;
}
```

## Interface pública de Func

```rust
impl Func {
    pub fn closure(repr: ClosureRepr) -> Self;
    pub fn native(name: &'static str, call: NativeFn) -> Self;
    pub fn native_with_namespace(name: &'static str, call: NativeFn, namespace: Arc<Scope>) -> Self;
    pub fn native_with_engine(name: &'static str, call: NativeFnWithEngine) -> Self;
    pub fn native_with_engine_and_namespace(name: &'static str, call: NativeFnWithEngine, namespace: Arc<Scope>) -> Self;
    pub fn element(name: impl Into<String>, ctor: ElementCtor) -> Self;
    /// **P699** — constrói `Func` a partir de um `PluginFunc` (export WASM).
    pub fn plugin(p: crate::entities::plugin_func::PluginFunc) -> Self;
    /// **P702** — `f.with(args)`: devolve nova `Func` com `args` pré-ligados.
    pub fn with(self, args: Args) -> Self;
    pub(crate) fn repr(&self) -> &FuncRepr;
    pub fn element_name(&self) -> Option<&str>;
    pub fn name(&self) -> Option<&str>;
    pub fn native_fn_addr(&self) -> Option<NativeFn>;
    pub fn namespace(&self) -> Option<&Scope>;
    pub fn set_name(&mut self, name: String);
}
```

## Dependência de argumentos

`Args` e seu carrier são definidos exclusivamente em `entities/args.md`.
Este nó recebe esse valor inteiro; não possui nem duplica sua interface.

## Semântica confirmada

- **PartialEq por identidade, com excepção P742 para nativas**: duas `Func`
  são iguais se partilham o mesmo `Arc<FuncRepr>` (mesmo ponteiro) **ou** se
  ambas são nativas do mesmo kind (`Native`/`NativeWithEngine`) com o mesmo
  nome. Medição vanilla 0.15.0 (P742): `color.rgb == rgb` → `true`,
  `red.space() == rgb` → `true` — o vanilla materializa nativas como
  singletons estáticos (identidade); o cristalino cria Arcs frescos por
  lookup, logo a identidade equivalente é o nome. Closures, `Element`,
  `With` e `Plugin` mantêm identidade de ponteiro. Consistente com `Module`.
- **Clone O(1)**: `Arc::clone` — não clona o conteúdo da closure.
- **Debug**: `"<function>"` (string literal, nunca pânico).
- **Repr de closure (P744)**: `repr((x) => x + 1)` → `"(..) => .."` — o
  vanilla não mostra parâmetros nem corpo; closures nomeadas têm o mesmo
  repr. Nativas usam o nome (`repr(rgb)` → `"rgb"`); função sem nome mantém
  `"#function(...)"`.
- **Eager capture**: `ClosureRepr.captured` é um snapshot imutável; redefinições posteriores no scope pai não afectam a closure.
- **Namespace anexado**: clone O(1) via `Arc<Scope>`; field access em `Func` delega ao `Scope` se existir.

## Variante `Plugin` (P699)

`FuncRepr::Plugin(PluginFunc)` representa um **export de plugin WASM**
chamável como função da linguagem. O nome do export é dinâmico (vem do
módulo WASM), logo não cabe num fn-ptr nativo (`Native`/`NativeWithEngine`);
a chamada é delegada ao `PluginHost` capturado no `PluginFunc`
(ver `entities/plugin_func.md`).

- `Func::plugin(p)` constrói `Func(Arc::new(FuncRepr::Plugin(p)))`.
- `Func::name()` ⇒ `Some(&p.name)` (nome do export; usado em mensagens e
  na rejeição de named args).
- `Func::native_fn_addr()` ⇒ `None` (um plugin não tem endereço de fn
  nativa).
- `Func::namespace()` ⇒ `None` (um plugin não expõe sub-funções via field
  access neste passo).
- A aplicação (`apply_func` em `rules/eval/closures.rs`) delega a
  `call_plugin(p, args, span)`; ver `rules/stdlib/plugin.md`.

## Variante `With` (P702)

`FuncRepr::With(Arc<(Func, Args)>)` representa uma **aplicação parcial de
argumentos** — `f.with(a: 1, b: 2)` devolve uma nova `Func` que, quando
chamada, combina os `Args` pré-ligados com os da chamada final e delega à
função original. Funciona sobre **qualquer** variante de `Func` — `Closure`,
`Native`, `NativeWithEngine`, `Element`, `Plugin`, ou outro `With`
(encadeamento; sonda P702 confirmou `f.with(1).with(2)` funcionar por
recursão natural — cada nível de `With` funde os seus próprios `Args`
pré-ligados e delega ao nível interior).

**Paridade vanilla** (`foundations/func.rs:149-159,359-362,380-394` do
vanilla): `FuncInner::With(Arc<(Func, Args)>)`; na chamada,
`args.items = pre.items.chain(new.items)` (pré-ligados primeiro). O vanilla
usa uma representação **unificada** de `Args` (posicionais e nomeados no
mesmo vetor, cada item com `name: Option<EcoString>`); o cristalino separa
`items`/`named` (ADR-0107: divergência de **mecânica**, não de língua).

**Regra de fusão cristalina** (`merge_with_args`,
`compiler/eval/call_dispatch.rs`; contrato P1307-R3 abaixo):

- **Posicionais**: `pre.items` seguido de `new.items` — mesma ordem
  observável do vanilla (confirmado: `g.with(1, 2)` seguido de `g2(3)`, com
  `g(a,b,c) = a+b+c`, dá `6` no vanilla — `a=1,b=2,c=3`).
- **Nomeados**: a view continua projetando o último valor, mas todas as
  ocorrências pré-ligadas e novas permanecem no Args, na ordem causal.
  A antiga hipótese P702 de que sobrescrever o mapa bastava foi refutada
  pela medição P1307-R2 abaixo; ela não legitima descartar valores anteriores.

### Interface e dispatch

- `Func::with(self, args)` constrói
  `Func(Arc::new(FuncRepr::With(Arc::new((self, args)))))`.
- `Func::name()` ⇒ delega ao nome da função interna (`w.0.name()`) —
  consistente com o vanilla (`FuncInner::With(with) => with.0.name()`).
- `Func::native_fn_addr()` ⇒ `None` (uma aplicação parcial não é
  directamente um fn-ptr nativo).
- `Func::namespace()` ⇒ **delega** à função interna (`w.0.namespace()`).
  **Medido contra o vanilla** (não assumido): `table.with(columns:
  2).cell` compila no vanilla e devolve `function` — confirmado com
  `foundations/func.rs:269-277` (`Func::scope()`, o equivalente vanilla de
  `namespace()`), que tem o braço explícito
  `FuncInner::With(with) => with.0.scope()`. O sub-`Func` devolvido
  (`t.cell`) é o valor original do namespace, **não** herda os args
  pré-ligados de `t` — só a função `t` propriamente dita (chamada
  directamente) combina argumentos; `t.cell(...)` chama `table_cell`
  normalmente, porque `columns: 2` foi pré-ligado a `table`, não a `cell`.
- **Dispatch** (`apply_func`, `rules/eval/closures.rs`): funde os `Args`
  pela regra acima e chama `apply_func` recursivamente com a função
  interna — o encadeamento resolve-se por recursão, sem lógica extra.
- **Sintaxe** (`eval_func_call`, `rules/eval/closures.rs`): novo bloco de
  intercepção, mesmo padrão já em uso para `where`/`or`/`and` (P417/P423),
  `within` (P504), métodos de colecção (P466) e `state`/`counter` (P506) —
  se o callee é `FieldAccess` com campo `"with"` e o alvo avalia para
  `Value::Func`, avalia os argumentos da chamada e devolve
  `Value::Func(target.with(args))` **sem invocar** (`.with()` devolve uma
  função nova, não o resultado de a chamar). Aplica-se a **qualquer**
  `Value::Func` — nativa (com ou sem namespace), closure, elemento, plugin,
  ou já parcialmente aplicada. Se o alvo não for `Value::Func`, não
  intercepta (cai no caminho genérico de field access, que erra
  normalmente — nenhuma mudança de comportamento para não-funções).

## Critérios de Verificação

```
Func::closure(...).clone() == Func::closure(...)  → false (Arc distintos)
let f1 = Func::closure(...); let f2 = f1.clone(); f1 == f2  → true (mesmo Arc)
format!("{:?}", func)  → "<function>"
Args::positional(vec![]).is_empty()  → true
Args::positional(vec![Value::Int(1)]).len()  → 1
// P702 — with()
calc.round.with(digits: 2)(3.14159)                  → 3.14
{let g(a,b,c) = a+b+c; g.with(1,2)(3)}               → 6
{let h(a, named: 10) = a+named; h.with(named: 20)(5)} → 25
{let f(a,b,c) = a+b+c; f.with(1).with(2)(3)}         → 6      (encadeamento)
{let h(a,x:10,y:20)=a+x+y; h.with(x:100).with(y:200)(1)} → 301 (encadeamento nomeado)
type(table.with(columns: 2))       → function   (namespace delega através de With)
type(table.with(columns: 2).cell)  → function   (sub-função, não herda columns: 2)
// P724 — patterns de desestruturação em parâmetros
{let f = ((a, b)) => a + b; f((1, 2))}              → 3
{let f = ((a, b), c) => a + b + c; f((1, 2), 3)}    → 6
{let g(x, (a, b)) = x + a + b; g(10, (1, 2))}       → 13
{let f = (_, y) => y; f(1, 2)}                      → 2
{let f = ((a, ..rest)) => rest.len(); f((1, 2, 3))} → 2
```

## Scope-outs

- Namespace anexado não se aplica a closures (`FuncRepr::Closure`) nem a elementos de utilizador (`FuncRepr::Element`) neste passo.
- Não implementar field access mutável (set) no namespace.
- Colisões `.with` são agora medidas e cobertas por P1307-R3; validação
  de assinatura de outros owners não é alterada automaticamente por Func.

## P1148 — igualdade e hash para callbacks de counter

`CounterUpdate::Func` precisa permanecer comparável e hashável quando
transportado por `ElementPayload`. `Func` implementa `Eq` e `Hash` coerentes
com a igualdade vigente: nativas do mesmo kind usam o nome; as demais
representações usam identidade do `Arc`. O hash nunca executa a função nem
inspeciona o scope capturado.

## P1307-R3 — With conserva o Args causal, sem novo FuncRepr

### Medição anterior à decisão

Sobre HEAD `b303f1f15b610e09872b567027e0d806387fde8c` e working tree P1306
preservado, `entities/func.rs:39-42,237-238` já guarda `Arc<(Func, Args)>`.
Na matriz histórica,
`with-json-invalid-first-occurrence` e o paralelo TOML rejeitam o primeiro
`pretty:"bad"` mesmo quando o último é true. `with-spread-arguments-f/g`
preserva erros em origens distintas após factory. A fonte ratificada
`lab/typst-original/crates/typst-library/src/foundations/func.rs:372-374`
concatena pre/new sem apagar ocorrências. A separação física em views não
é por si divergência de língua; perder esses valores/origens é.

### Decisão sujeita ao gate

A representação e assinatura de Func não mudaram em R3; o span privado
de P1308 abaixo é uma revisão distinta. With guarda o Args inteiro,
incluindo o carrier definido em `entities/args.md`, e clone/alias/retorno
preservam esse objeto; não adiciona mapa lateral de spans a Func.
`Func::with` não valida aridade, named ou tipos da função interna nem a
executa. A aplicação e a combinação pertencem a `compiler/eval/call_dispatch.md`;
a seleção semântica de erro pertence à nativa chamada.

`Func::namespace` continua delegando ao original: `json.with(...).encode`
retorna o membro original sem herdar os argumentos do parent.
`json.encode.with(...)` é aplicação parcial do próprio encoder.
Os três encoders originais têm repr `encode`; suas aplicações parciais têm
repr `(..) => ..`, conforme os casos R2 de identidade. A decisão de impressão
pertence a `compiler/eval/repr.md`, sem mudar `Func::name`/namespace.

A igualdade/Hash de Func e a captura eager permanecem vigentes. Testemunhas
futuras: alias e cadeias With preservam origens; parent e encoder With não
se confundem; produzir With inválido ainda retorna função; só a chamada
posterior valida. Gate ADR-0127 permanece causado pelo contrato Args e pelos
encoders; esta limpeza de ownership textual não é um novo consumer.

## P1308 — origem diagnóstica privada de Func

### Medição anterior à decisão

No baseline P1308,
`entities/func.rs:20` armazena somente Arc da representação. O parâmetro
lexical da closure não está no carrier; seu body não identifica o mesmo range.
Vanilla ratificado `foundations/func.rs:380–389` guarda Span e o preenche
somente se detached; `:405–409` conserva esse span ao criar With.
`foundations/args.rs:410–412` ancora falha de bool de filter em test.span().
Logo origem da função, não origem do valor filtrado, é o dado necessário.

### Decisão autorizada

Func pode guardar um Span **privado**, separado do Arc<FuncRepr>, com helpers
`pub(crate) fn diagnostic_span(&self) -> Span` e
`pub(crate) fn with_diagnostic_span(self, span: Span) -> Self`.
O segundo preenche somente quando detached. Nenhum campo/método público novo;
constructors públicos conservam assinaturas e inicializam detached. Clone
copia o span, With conserva o span da função interna, set_name não o muda.
Helpers não procuram AST, World, valores iguais, nomes ou ponteiros para
reconstruir origem. Captura da origem pertence aos owners de avaliação.

Não incluir esse Span em Eq/Hash, Debug, repr, namespace, name ou identidade
do Arc. A origem não altera captura eager, parâmetros, Args ou aplicação.
Closures sintéticas sem AST continuam detached; nenhuma origem é inventada.
Testes devem distinguir funções iguais/aliased com origens preservadas,
With, chamadas repetidas e ausência legítima de Source. É transporte interno
de diagnóstico autorizado em P1308, sem nova API pública ou fase.

## P1342 — carrier privado da ocorrência de callback

### Medição anterior à decisão

A medição histórica registra que o Func exterior With é
transportado intacto dentro de `CounterUpdate::Func` e que o alvo interno é o
mesmo `Func` clonado no dispatch. Esse é o carrier real mínimo; não é necessário
alterar `ContextBlockElem`, `CounterRegistry`, `Value` ou `ElementPayload`.

### Decisão vigente

Sob `cfg(p1339_observation)`, `Func` possui um terceiro campo privado opcional
com carrier P1342. O carrier contém handle append-only compartilhado, célula,
id de ocorrência e span de origem. Construtores começam em None; clone/alias
preservam o handle; o owner counter o anexa somente ao Func efetivamente
classificado como callback da ocorrência focal. `Func::with` conserva qualquer
carrier anterior, mas a anexação ao wrapper exterior não altera o inner.

Helpers cfg-only expõem ao crate instalar/consultar o carrier e atribuir IDs
locais aos objetos reais. O campo não participa de Eq, Hash, Debug, repr, nome,
namespace, `diagnostic_span`, identidade do Arc, dispatch ou serialização
normal. Sem o cfg o campo e os helpers não existem. Não usar endereço como
expectativa estável, side table global, nome ou output para reconstrução.

Este carrier é entidade somente de observação e não constitui contrato público
normal conforme ADR-0127. Seu escopo termina no binding real P1342; não autoriza
rastreio geral de Func, retenção P1340, NT01–NT06 ou nova política de produto.

## P1344 — ownership local dos helpers de observação de `Func`

### Medição anterior à decisão

O blocker P1343 R3 mediu que a fronteira candidata fica entre o fim de
`Func::namespace` e o `}` que fecha `impl Func`. Nessa posição Rust aceita
somente associated items de `Func`; tentar declarar `impl Content` ou
`impl CounterUpdate` produz `implementation is not supported in traits or
impls`. Fechar/reabrir braces ou usar container não enumerado não é uma
materialização válida da cápsula.

### Decisão vigente

Sob `cfg(p1339_observation)`, o owner `entities/func.rs` declara somente os
tipos privados do carrier/ledger e associated methods cujo receiver/retorno é
de `Func` ou do próprio carrier. Ele não declara inherent impls, métodos ou
reexports pertencentes a `Content` ou `CounterUpdate`.

As projeções owner-local dessas duas entidades pertencem respectivamente a
`entities/content.md` e `entities/counter_update.md`. A separação não muda o
terceiro campo privado P1342, clone/alias, Eq, Hash, Debug, repr, name,
namespace, dispatch, API normal ou comportamento sem o cfg. Não usar trait de
extensão, macro, impl não local ou helper genérico para contornar ownership.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

01_core/src/entities/func.rs:317 (C024).
O snapshot before.json.gz em lab/results/construction/diagnostics/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Manter literalmente o contrato público de native_fn_addr e o ponteiro nativo estático que ele retorna. Autorizar #[expect(clippy::type_complexity, reason = "native_fn_addr: ABI nativa explícita preservada")], somente no método. Não criar trait object, alias público novo, wrapper de chamada ou alterar ABI, identidade, None e variantes.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Exaustividade de representação e identidade — V16, 2026-10-16

### Medição anterior à decisão

01_core/src/entities/func.rs:398. Fonte e owner vigentes lidos e conferidos por hash antes da decisão. `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, status e git diff HEAD --stat. O SARIF anterior registra onze avisos e uma nota V16 nestes sete owners. Foram também medidos os fallbacks sem emissão; estes fazem parte do mesmo recorte. Classificação: padrão Rust e evolução do enum são mecânica; resultados/campos/identidade observáveis e erros devem permanecer. Não se infere intenção upstream ou ganho para IA.

### Responsabilidade exclusiva

element_name: None nomeia Closure/Native/NativeWithEngine/Plugin/With; Element permanece Some. namespace: None nomeia Closure/Element/Plugin; Native/NativeWithEngine/With permanecem. PartialEq: depois do atalho Arc e dos dois pares nativos, false cobre primeiro eixo nominal das seis variantes e segundo eixo _. Hash: fallback por ponteiro nomeia Closure/Element/Plugin/With e preserva discriminante e Arc::as_ptr. Nenhum nome/namespace é nova identidade; span continua excluído; não chamar funções, ctor ou host. Adicionar matriz das seis representações com clone/hash, nomes nativos iguais no mesmo kind e rejeição entre kinds, Element e With/Plugin; host/ctor não podem ser executados pelo teste nem pelas projeções.

### Aceitação e gate

Manutenção interna ADR-0127 em fluxo contínuo: nenhuma definição de enum, campo, trait, assinatura pública, default, dependência, compatibilidade ou fase muda. Conservar cada corpo, ordem e guard; não mover lógica entre owners nem criar helper para esconder o catchall. Padrões nominais devem forçar erro de compilação ao surgir variante nova; comentários de neutralidade não servem como supressão.

Antes do código, testes funcionais locais e gate V16 RED por escopo. Executar sonda de evolução em cópia descartável do workspace: acrescentar somente uma variante sentinela a Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate, sem alterar as definições produtivas. Usar cargo check --offline --locked -p typst-core com um job e cache de artefatos compartilhado; registrar fontes/hashes/argv/UTC/diagnósticos. Para cada um dos 21 matches selecionados, a baseline deve demonstrar a ausência de E0004 naquele match; GREEN exige E0004 ali, independentemente de erros de exaustividade já emitidos por outros matches na cópia. O controle de compilação sem sentinelas e os testes de comportamento aprovam o código real; não usar a sonda negativa como prova de paridade. Não editar source produtivo durante comandos.

Resselo restrito, direto e reverso, sem reparo global da working tree. Revalidar build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, suíte workspace completa, Rustdoc -D warnings e quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF sem avisos novos e diff --check. Limite MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1, tarefas pesadas sequenciais. Nenhuma tolerância/fixture histórica é relaxada. As pendências V16 externas, inclusive T027/T158/T160 rastreadas sem emissão, continuam abertas.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/`
conserva before.json.gz com UTC, status, diff HEAD --stat e hashes exatos;
baseline-reuse.json prova igualdade de fontes/configuração/binário com a
matriz final anterior de 3956 casos em oito perfis. before-batch.json.gz
mede 144 casos detalhados; before-compat.json.gz mede 456 controles de
nomes, aliases e namespace. Cada sonda conserva argv, binários e saídas.
O pin é `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

As quinze rotas restantes são rejeitadas por where/kind no Tekt e aceitas
no pin. format.rs:17–27/96–146 compartilha native_format entre os cinco
formatos, logo só ponteiro não distingue schemas. pinned-schema.json.gz
conserva nove declarações upstream com blobs/hash/linhas e atributos elem.
FuncRepr é pub(crate) em entities/func.md:21 e entities/func.rs:24; a nota
anterior de que o L0 o mostrava público foi refutada em retificacao-func-repr.md.
public-contract-before.json fixa assinaturas, reexports e structs públicos
antes da mudança; a classificação é representação interna e paridade, não
novo contrato público ou inferência de intenção por render.

### Responsabilidade exclusiva

Adicionar `pub(crate) enum FormatElementKind { Svg, Png, Pdf, Html, Bundle }`
fechado e Copy/Eq, e `FuncRepr::Format(FormatElementKind, NativeFunc)` à enum
privada. Func, NativeFunc, NativeFuncWithEngine, ElementFunc, ClosureRepr e
ClosureParam conservam literalmente campos e contratos externos atuais.
Apenas o constructor interno `pub(crate) fn native_format(kind: FormatElementKind,
native: NativeFunc) -> Self` inicializa a nova variante com span detached.
Não criar nova interface pública, alias público de ABI, closure de wrapper,
side table ou alocação compartilhada global.

Nome/namespace/native_fn_addr tratam Format como Native de quatro argumentos:
retornam os mesmos dados do payload original. element_name continua None para
ambos, reservado ao construtor dinâmico Element. set_name continua só Closure.
Clone continua Arc O(1); With conserva nome/namespace/span e native_fn_addr None.
Eq público agrupa Native e Format em ambos os eixos e compara nomes como antes;
Hash usa o mesmo prefixo 0/nome. Engine mantém prefixo1 e variantes restantes
prefixo2/Arc; span e kind não entram no Eq/Hash público.

A tabela de reconhecimento, em seu owner, usa a testemunha para os formatos.
A obrigação anterior de native_fn_addr não mudar None/ponteiro é preservada
para todos os valores existentes: factories antes Native e depois Format
continuam Some do mesmo ponteiro; os outros seis tipos mantêm os casos antigos.
Testes de layout imprimem sizeof Func/FuncRepr/NativeFunc/NativeFuncWithEngine
no RED e no GREEN para medir o custo, sem fazer layout Rust virar paridade.

### Aceitação e fronteiras

Consumir as invariantes compartilhadas do Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml`. As novas
alternativas Native/Format são agrupáveis porque usam o mesmo payload/ABI
para projeção e execução; cada uma deve ser exercitada nos testes de nome,
namespace, ponteiro, Eq/Hash, clone/With e despacho. V19 novo nessas uniões
deve ser inventariado e ligado aos testes; não adicionar suppressions.
A nova variante deve ser explicitamente nomeada nos matches fechados.

Fluxo contínuo ADR-0127: nenhuma definição, assinatura, campo ou trait
externo novo, compatibilidade, default ou fase muda. Antes do código, RED
funcional dos quinze casos por factories atuais e controle de observação
real contra Native não marcada. GREEN exige corpus completo, 144 casos de
campos/chamadas e 456 controles de compatibilidade preservados fora de
where/kind. Comparar public-contract-before/after e sizeof antes/depois.

L0 antes do código; ownership/V26 antes de resselo restrito aos onze pares.
Build, suíte workspace, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26, reverse
e diferencial global sem novo erro/aviso. Comandos pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Rastrear notas V19 legítimas com cobertura.

Esta etapa fecha admissão where/kind, não todas as operações de seleção,
show/query, síntese, morfologia dos carriers ou counter. As frentes FG1, RM1,
SY1, RG1 e V16 anteriores preservam seus escopos/gates; não aplicar candidatos
antigos por cópia nem publicar.


### Cobertura direta das alternativas V19 — 2026-10-17

Medição anterior à decisão: candidate-1/state.json.gz e
differential-review.json em
lab/results/construction/diagnostics/native-format-catalogue-2026-10-17 registram oito
notas V19 novas e quatro atualizadas. Estas últimas apenas incluem Format
no padrão e elevam a contagem; não são violações removidas. O primeiro
comparador exigia mensagem idêntica e por isso recusou a atualização.

Este owner recebe testes diretos complementares; não mudar o comportamento
produtivo, suprimir notas ou usar somente um caminho como prova do grupo.
Estender a matriz preexistente das seis representações para sete, incluindo
Format de mesmo nome/namespace, Eq/Hash compatível com Native e rejeição
contra Engine/Closure/Element/Plugin/With; conferir native_fn_addr e namespace
para cada alternativa. Acrescentar prova de que os cinco kinds, com mesmo
nome/callback que panica se chamado, têm schemas distintos pela testemunha,
enquanto Native comum e With não adquirem essa capacidade.
