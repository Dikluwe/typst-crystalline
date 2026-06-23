# Prompt L0 — rules/eval
Hash do Código: 3a971209

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0001 (comemo em L1), ADR-0024 (ecow/Value::Str)

## Contexto

`eval()` é o motor de avaliação do compilador Typst. Recebe uma `Source`
e retorna um `Module` com os bindings definidos nesse ficheiro.

**Estado actual (Passo 15)**: travessia AST com control flow e ADR-0025.
Avalia literais, Ident, Let, CodeBlock, Binary, Unary, Conditional (if/else),
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
- `Expr::ForLoop(loop)` → eval_for: iterable() (não iter()), pattern().bindings(),
  body(); Value::None tratado como iterável vazio (sem parsing de array literal)

## Fronteira deliberada

`_ => Ok(Value::None)` — nós não implementados retornam None sem erro.
Permite encontrar `#let x = 1` dentro de markup sem avaliar texto puro.
Requer Content, Func, Styles para implementação completa (ADR-0017).

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
  quando a regra é no-op morfológico) ou até o **teto-64 backstop** (`MAX_SHOW_RULE_DEPTH`;
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
