# Prompt L0 — entities/scope

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
