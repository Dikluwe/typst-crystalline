# Prompt L0 — `rules/scopes`
Hash do Código: ecc1fd7f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/scopes.rs`
**Criado em**: 2026-04-02 (Passo 31 — closures lazy capture / Scopes com captured)
**Atualizado em**: 2026-04-12 (restauro — expandido com `captured`, `with_parent`, `snapshot`, `push_scope`, `iter_all`)
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0023 (Scope/indexmap)

---

## Contexto e Objetivo

`Scopes<'a>` é a **pilha de âmbitos léxicos** durante a avaliação (`eval.rs`).
Mantém o âmbito activo (`top`), uma pilha de âmbitos anteriores (`scopes`),
um scope capturado opcional para closures (`captured`), e uma referência
opcional à biblioteca standard (`base`).

A regra de pesquisa é: **top → scopes (reverso, mais recente primeiro) → captured → base**.

Pertence a `rules/` porque é um mecanismo de execução (avaliador), não uma
entidade de dados. Depende apenas de `Scope`, `Value` e `Library` (todos L1).

### Closures e captura lazy (Passo 31)

No Typst, closures capturam o scope no momento da definição. O cristalino
implementa captura em duas fases:

1. **Snapshot (captura eager)**: `scopes.snapshot()` cria um `Scope` com todos
   os bindings visíveis. O resultado é envolvido em `Arc::new(...)` — partilhado
   por referência com custo O(1) por closure subsequente.
2. **Lookup (lazy)**: durante a chamada de closure, `Scopes::with_parent(arc)`
   cria uma nova pilha onde `captured` aponta para o scope da definição.

Origem: `lab/typst-original/crates/typst-library/src/foundations/context.rs` e `eval.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O. `Arc<Scope>` em `captured` é gestão de RAM (ADR-0029).
- Sem dependências externas.
- `Vec<Scope>` em `scopes` — clone é O(n); apenas `enter`/`exit` mutam a pilha.
- O campo `base: Option<&'a Library>` é somente leitura — não mutado após a criação.

---

## Instrução

### Estrutura pública

```rust
pub struct Scopes<'a> {
    /// Âmbito activo no momento.
    pub top: Scope,
    /// Âmbitos anteriores (mais antigo na posição 0, mais recente no fim).
    pub scopes: Vec<Scope>,
    /// Scope capturado pela closure — partilhado via Arc sem clone dos valores.
    /// Consultado após top/scopes e antes de base.
    pub captured: Option<Arc<Scope>>,
    /// **P772q** — por que motivo `captured` foi capturado (closure normal
    /// vs bloco `context`). `None` sse `captured` também for `None`. Usado
    /// só para escolher a mensagem de erro ao mutar um nome de `captured`
    /// — não afecta leitura (`get`) nem a pesquisa normal.
    pub captured_by: Option<Capturer>,
    /// Âmbito base — a Library (stdlib) do Typst. Somente leitura.
    pub base: Option<&'a Library>,
}
```

### Interface pública completa

```rust
impl<'a> Scopes<'a> {
    /// Cria nova pilha com top vazio e base opcional.
    pub fn new(base: Option<&'a Library>) -> Self

    /// Cria uma pilha para chamada de closure com o scope capturado como parent.
    /// Lookup order: top (params) → captured (scope da definição).
    /// O Arc é partilhado — sem clone dos valores da captura.
    /// **P772q** — `capturer` regista por que motivo o scope foi capturado
    /// (`ClosureRepr.capturer`, `entities/func.md`); fica em `captured_by`.
    pub fn with_parent(parent: Arc<Scope>, capturer: Capturer) -> Scopes<'static>

    /// Captura todos os bindings visíveis num snapshot Scope (eager).
    /// Ordem de inserção: captured → scopes → top (mais recente sobrescreve).
    /// Custo: O(N) uma vez; depois partilhado em O(1) por cada closure.
    pub fn snapshot(&self) -> Scope

    // ── Gestão de pilha ─────────────────────────────────────────────────
    /// Entra num novo âmbito: empurra top para scopes, cria novo top vazio.
    pub fn enter(&mut self)

    /// Sai do âmbito activo: restaura o anterior. Retorna o âmbito saído.
    pub fn exit(&mut self) -> Scope

    /// Empurra um Scope pré-populado como novo âmbito activo.
    /// Usado por apply_closure para criar o ambiente de chamada.
    pub fn push_scope(&mut self, scope: Scope)

    // ── Bindings ────────────────────────────────────────────────────────
    /// Define um binding no âmbito activo (top).
    pub fn define(&mut self, name: impl Into<String>, value: Value)

    /// Pesquisa um nome do âmbito mais local para o mais global.
    /// Ordem: top → scopes (reverso) → captured → base.
    /// **P772n**: `base` deixou de ser stub — consulta real a
    /// `base.global` como último recurso.
    pub fn get(&self, name: &str) -> Option<&Value>

    /// **P780** — pesquisa local/utilizador, sem consultar `base` (stdlib
    /// global). Ordem: top → scopes (reverso) → captured — igual a `get`
    /// menos o último passo. Paridade `Scopes::get_in_math` (vanilla,
    /// `foundations/scope.rs:75-92`), usado por `compiler/eval/math.rs`
    /// para resolver `MathIdent` sem deixar o scope global do stdlib
    /// "vazar" para modo math (`$str$` sem binding local não deve
    /// resolver directo à função `str` — só via `std.str`).
    pub fn get_local(&self, name: &str) -> Option<&Value>

    /// **P780** — `true` se `name` existe em `base.global` (stdlib). Só
    /// para escolher o hint de `unknown_variable_math` — não é usado
    /// para resolução (ver `get_local`).
    pub fn has_global(&self, name: &str) -> bool

    /// Itera sobre todos os bindings visíveis (para snapshot e diagnóstico).
    /// Ordem: captured → scopes[0] → ... → top (mais recente sobrescreve).
    pub fn iter_all(&self) -> impl Iterator<Item = (&str, &Value)> + '_

    /// **P772n** — true se `name` só é alcançável via `base` (stdlib e
    /// outros bindings seedados antes do âmbito do documento começar),
    /// nunca via `top`/`scopes` (mutável) nem `captured` (fecho). Usado
    /// pelo caller de atribuição (`eval/bindings.rs::access`) para
    /// escolher entre `"cannot mutate a constant: {name}"` e
    /// `"unknown variable: {name}"` quando `get_mut` falha — paridade
    /// vanilla `Scopes::get_mut` (`foundations/scope.rs:63-70`).
    pub fn is_constant(&self, name: &str) -> bool

    /// **P772q** — `Some(capturer)` sse `name` está em `captured` (não em
    /// `top`/`scopes`, que têm precedência). `capturer` vem de
    /// `captured_by`, propagado desde `ClosureRepr.capturer` no momento em
    /// que a closure/`context` foi definida. Usado pelo caller de
    /// atribuição para escolher entre as duas mensagens
    /// "variables from outside the {function|context expression}...".
    /// Precedência com `is_constant`: `captured_by` é verificado primeiro
    /// (paridade vanilla: `get_mut` bem sucedido sobre um binding
    /// `Captured` falha só em `.write()`, antes de qualquer verificação
    /// de `base` acontecer — `captured` "ganha" a `base` por estar mais
    /// próximo na cadeia de pesquisa).
    pub fn captured_by(&self, name: &str) -> Option<Capturer>
}
```

---

## Protecção de bindings constantes (P772n)

`Scopes::get_mut` (já existente desde P715) **nunca** pesquisa `captured`
nem `base` — só `top`/`scopes`. Isto por si só já torna qualquer binding
de `base` estruturalmente imutável, **sem precisar de nenhum campo
adicional em `Binding`** (ao contrário da hipótese inicial de P772n, que
assumia um `BindingKind::Const`; a sonda desse passo confirmou que o
vanilla protege a stdlib por **separação estrutural de âmbito**
(`base` nunca alcançado por `get_mut`), não por uma flag por-binding —
`BindingKind`/`Capturer` no vanilla existe só para o caso, distinto, de
variável capturada por closure/`context`, fora do âmbito de P772n).

`is_constant` fecha o único buraco que essa estrutura por si só não
resolve: **a mensagem de erro**. Sem ela, `access()` (`eval/bindings.rs`)
não tem como distinguir "não encontrado em lado nenhum" (`unknown
variable`) de "encontrado em `base`, mas `base` não é mutável" (`cannot
mutate a constant`) — ambos os casos fazem `get_mut` devolver `None`.

Quem popula `base` (bootstrap do avaliador, `eval/mod.rs`/`eval/modules.rs`)
deixa de fazer `scopes.define(name, ...)` directamente em `top` para a
stdlib/cores/`std`/`text`/elementos de utilizador — em vez disso constrói
um `Scope` próprio e embrulha-o em `Library::with_global(scope)`
(`world-types.md`), passado a `Scopes::new(Some(&library))`. Um `#let`
do documento que sombreia um nome da stdlib (`#let calc = 5`) continua a
criar um binding **normal** em `top` — mutável como qualquer outro,
porque `get_mut` encontra-o aí **antes** de sequer considerar `base`
(paridade vanilla, medido em P772n: `#let calc = 5; #{ calc = 10 }`
compila sem erro no vanilla).

---

## Mensagem correcta para variável capturada (P772q)

Vanilla protege variáveis capturadas por um mecanismo **diferente** do de
`base`/`is_constant`: `Scopes::get_mut` (vanilla) encontra a variável
capturada normalmente (foi inserida numa scope alcançável) — é
`Binding::write()`, chamado **depois**, que falha ao ver
`kind == BindingKind::Captured(capturer)` (`foundations/scope.rs:313-323`).
O cristalino não replica essa estrutura (exigiria `kind` em `Binding`,
schema ainda adiado por ADR-0017) — replica só o **observável**: a
mensagem certa, por um caminho estruturalmente diferente.

`captured_by(name)` cobre o buraco que a exclusão estrutural de `captured`
(P715 — `get_mut` nunca o pesquisa) deixa na mensagem de erro, do mesmo
jeito que `is_constant` cobre o buraco equivalente para `base`. Ordem de
verificação em `access()` (`eval/bindings.rs`) quando `get_mut` falha:

1. `captured_by(name)` — `Some(capturer)` → `"variables from outside the
   {function|context expression} are read-only and cannot be modified"`.
2. `is_constant(name)` — `true` → `"cannot mutate a constant: {name}"`.
3. Senão → `"unknown variable: {name}"`.

Esta ordem já é consistente com a implementação de `is_constant` (que já
verifica `captured` antes de `base`, devolvendo `false` — "não constante"
— para nomes só alcançáveis via `captured`) — `captured_by` só precisa de
ser consultado **antes** para produzir a mensagem certa nesse caso, em vez
de cair no `unknown_variable` genérico.

### `capturer` — de onde vem

`ClosureRepr.capturer` (`entities/func.md`) é decidido no momento da
**definição** da closure/`context`, não da chamada — paridade
`CapturesVisitor::new(scopes, capturer)` do vanilla
(`typst-eval/src/call.rs:576` para `Closure`, `typst-eval/src/code.rs:394`
para `Contextual`). `apply_closure` (`eval/closures.rs`) propaga
`closure.capturer` para `Scopes::with_parent(closure.captured,
closure.capturer)`.

---

## Pesquisa restrita a scope local — `get_local`/`has_global` (P780)

`get_local` existe porque `get_in_math` (vanilla) tem um fallback
**diferente** de `get` normal: em vez de cair em `base.global` (stdlib
completo) como último recurso, cai em `base.math.scope()` (só os
operadores/símbolos matemáticos). Se `compiler/eval/math.rs` usasse `get`
directamente para resolver `MathIdent`, um nome de stdlib não-sombreado
(`str`, `int`, `calc`, ...) resolveria **silenciosamente** em modo math
(errado — medido: `$str$` no vanilla **erra** "unknown variable: str",
não mostra a função `str`). `get_local` pára exactamente onde `get_in_math`
pararia — antes de `base` — deixando o caller (`math.rs`) decidir o que
fazer a seguir (símbolo Unicode, operador `math`, ou erro).

`has_global` é **só** para a mensagem de erro: distingue "`str` existe na
stdlib mas não está disponível em math" (3 hints) de "`foobarbaz` não
existe em lado nenhum" (2 hints) — paridade `unknown_variable_math(var,
in_global)` (vanilla). Nunca usado para resolução.

---

## Critérios de Verificação

```
// Define e lookup no top
Scopes::new(None) + define("x", Value::None) → get("x") = Some(...)

// Lookup percorre a pilha
Scopes::new(None) + define("x", ..) + enter() → get("x") = Some(...)

// exit remove bindings do filho
Scopes::new(None) + enter() + define("local", ..) + exit()
→ get("local") = None

// enter/exit simétrico
define("global", ..) + enter() + define("local", ..) + exit()
→ get("global") = Some(..)
→ get("local")  = None

// Sombra: filho oculta pai com mesmo nome
define("x", V1) + enter() + define("x", V2)
→ top.get("x") = Some(V2)
→ scopes.last().get("x") = Some(V1)

// P772n — base é lido, nunca mutável, is_constant distingue os dois erros
library = Library::with_global({"calc": Value::Module(..)})
Scopes::new(Some(&library)) → get("calc") = Some(..)
Scopes::new(Some(&library)) → get_mut("calc") = None
Scopes::new(Some(&library)) → is_constant("calc") = true
Scopes::new(Some(&library)) → is_constant("nope") = false  // não existe em lado nenhum

// Sombra de nome de base continua mutável (paridade vanilla)
Scopes::new(Some(&library)) + define("calc", Value::Int(5))
→ get_mut("calc") = Some(&mut Value::Int(5))  // encontrado em top, base nem chega a ser consultado
→ is_constant("calc") = false

// P772q — captured_by distingue Function de Context
captured = Arc::new({"x": Value::Int(1)})
Scopes::with_parent(captured.clone(), Capturer::Function) → captured_by("x") = Some(Capturer::Function)
Scopes::with_parent(captured.clone(), Capturer::Context)  → captured_by("x") = Some(Capturer::Context)
Scopes::with_parent(captured, Capturer::Function)         → captured_by("nope") = None

// Sombra de nome capturado por parâmetro/`let` local continua mutável
Scopes::with_parent(captured, Capturer::Function) + define("x", Value::Int(2))
→ get_mut("x") = Some(&mut Value::Int(2))  // encontrado em top, captured nem chega a ser consultado
→ captured_by("x") = None

// P780 — get_local pára antes de base; has_global só reporta base
library = Library::with_global({"str": Value::Func(..)})
Scopes::new(Some(&library)) → get_local("str") = None       // não em top/scopes/captured
Scopes::new(Some(&library)) → get("str")       = Some(..)   // get normal cai em base
Scopes::new(Some(&library)) → has_global("str") = true

Scopes::new(Some(&library)) + define("str", Value::Int(5))
→ get_local("str") = Some(&Value::Int(5))  // top tem prioridade sobre base
```

---

## Resultado Esperado

- `01_core/src/compiler/scopes.rs` com `Scopes<'a>` e testes co-localizados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/compiler/scopes.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-02 | Criação — Passo 31: closures lazy capture; campo `captured: Option<Arc<Scope>>` | `scopes.rs` |
| 2026-04-12 | Restauro — expandido com `with_parent`, `snapshot`, `push_scope`, `iter_all`, critérios completos | `scopes.md` |
