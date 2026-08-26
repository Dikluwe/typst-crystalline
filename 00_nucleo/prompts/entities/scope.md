# Prompt L0 — entities/scope
Hash do Código: ee0fb4e2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/scope.rs`
**ADRs relevantes**: ADR-0023 (indexmap em L1), ADR-0017 (adiamento eval), ADR-0018 (rustc_hash)

## Contexto

`Scope` é o container de nomes do compilador Typst — um mapa de
identificadores para bindings. A ordem de declaração é semanticamente
significativa em Typst (ex: ordem de importação, sombra de nomes),
por isso usa `IndexMap` com `FxBuildHasher` em vez de `HashMap`.

`Binding` neste passo mantém apenas `value: Value(())` — os campos
adicionais do original (`kind`, `span`, `category`, `deprecation`)
dependem de `Value` real, `Span` e tipos não migrados. São adicionados
quando `Value` real migrar (ADR-0017).

`Scopes<'a>` (pilha de Scope com lifetime de parent) existe no original
mas não é incluído neste passo — pertence ao mecanismo de `eval()`,
não à entidade de domínio. Incluir apenas se necessário para `Module`.

## Interface pública

```rust
pub struct Binding { value: Value }
impl Binding {
    pub fn new(value: Value) -> Self
    pub fn value(&self) -> &Value
    pub fn into_value(self) -> Value
}

pub struct Scope { map: IndexMap<String, Binding, FxBuildHasher> }
impl Scope {
    pub fn new() -> Self
    pub fn define(&mut self, name: impl Into<String>, value: Value)
    pub fn get(&self, name: &str) -> Option<&Value>
    pub fn get_binding(&self, name: &str) -> Option<&Binding>
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Binding)>
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
}
impl Default for Scope

/// **P772q** — por que motivo um scope foi capturado (closure vs bloco
/// `context`). Paridade vanilla `foundations/scope.rs::Capturer` — usado
/// para a mensagem de erro correcta ao tentar mutar uma variável
/// capturada ("...outside the function..." vs "...outside the context
/// expression..."). Diferente de `BindingKind` (ainda omitido, ADR-0017):
/// não é uma flag por-binding — é um único valor por invocação de
/// closure, guardado em `ClosureRepr.capturer` (`entities/func.md`) e
/// propagado para `Scopes` (`rules/scopes.md`) no momento da chamada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capturer {
    /// Capturado por uma closure normal (`#let f() = { ... }`).
    Function,
    /// Capturado por um bloco `context { ... }`.
    Context,
}
```

## Campos omitidos de Binding

Do original:
- `kind: BindingKind` — depende de `Value` real, `Func`, `NativeFunc` — **omitido** (ADR-0017)
- `span: Span` — depende de Span real (já existe em L1); poderia incluir mas
  acoplaria Binding a Span sem necessidade neste passo — **omitido** (ADR-0017)
- `category: Option<Category>` — tipo não migrado — **omitido** (ADR-0017)
- `deprecation: Option<Box<Deprecation>>` — tipo não migrado — **omitido** (ADR-0017)

## Por que IndexMap e não HashMap

`HashMap` do std não preserva ordem de inserção. A ordem de declaração
de bindings em Typst é semanticamente significativa: um nome declarado
antes de outro deve aparecer antes na iteração do scope. `IndexMap`
preserva esta propriedade com custo O(1) de lookup (ADR-0023).

## Por que FxBuildHasher

Identificadores Typst são tipicamente curtos (< 32 bytes). `FxHash`
é optimizado para chaves curtas — mais rápido que `SipHash` (padrão
do std) sem sacrificar qualidade de hashing (ADR-0018).

## Critérios de Verificação

```
Dado Scope vazio
Quando get() for chamado
Então None

Dado Scope com binding "x"
Quando get("x") for chamado
Então Some(&Value(()))

Dado Scope com bindings inseridos em ordem [z, a, m]
Quando iter() for chamado
Então ordem preservada: [z, a, m]

Dado Scope com bindings [a, b] e redefinição de "a"
Quando iter() for chamado
Então posição de "a" mantida: [a, b] (não [b, a])

Dado Scope vazio
Então is_empty() = true, len() = 0
```

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|-------------------|
| 2026-07-17 | P772q — `Capturer` (enum `Function`/`Context`), paridade vanilla `foundations/scope.rs`. Fecha P772l §2.2 (mensagem de mutação de variável capturada) | `scope.md`, `scope.rs` |
