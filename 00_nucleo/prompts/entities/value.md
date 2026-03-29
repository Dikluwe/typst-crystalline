# Prompt L0 — entities/value

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/value.rs`
**ADRs relevantes**: ADR-0024 (ecow em L1 para Value::Str), ADR-0017 (adiamento eval)

## Contexto

`Value` é o tipo de runtime do compilador Typst — o enum que representa
todos os valores possíveis durante a avaliação. O original tem ~35 variantes.

**Subset inicial (Passo 13)**: 5 variantes de literais primitivos.
As restantes são listadas como comentários — fronteira deliberada,
não código incompleto. Não adicionar variantes sem ADR e tipo migrado.

## Decisão sobre EcoString (ADR-0024)

`Value::Str` usa `EcoString` (crate `ecow`), não `String`. Razão:
durante eval(), strings são passadas como argumentos, capturadas em
closures, e concatenadas frequentemente. Clone com `EcoString` é O(1)
para strings não mutadas (copy-on-write). Com `String` seria O(n) em
cada passagem — erro técnico grave de performance.

Contraste com ADR-0015 (que removeu ecow do parser): o parser constrói
strings uma vez durante o parse; Value::Str é clonado no hot path de
eval(). Contextos com características opostas.

## Interface pública

```rust
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(EcoString),  // ADR-0024

    // ~30 variantes futuras comentadas — não implementar sem ADR
}

impl Value {
    pub fn type_name(&self) -> &'static str
    pub fn is_none(&self) -> bool
    pub fn cast_bool(&self) -> Option<bool>
    pub fn cast_int(&self) -> Option<i64>
    pub fn cast_float(&self) -> Option<f64>  // aceita Int (coerção implícita Typst)
    pub fn cast_str(&self) -> Option<&str>
}

// Conversões From para ergonomia em eval() e testes
impl From<bool>      for Value
impl From<i64>       for Value
impl From<i32>       for Value
impl From<f64>       for Value
impl From<EcoString> for Value
impl From<&str>      for Value
impl From<String>    for Value
```

## Variantes futuras (comentadas — NÃO implementar sem ADR e tipo migrado)

Auto, Length, Angle, Ratio, Relative, Fraction, Color, Gradient,
Tiling, Symbol, Version, Bytes, Datetime, Decimal, Duration,
Content, Styles, Array, Dict, Func, Args, Type, Module, Dyn

## Critérios de Verificação

```
Dado Value::None
Então type_name() = "none", is_none() = true

Dado Value::Int(3)
Quando cast_float() chamado
Então Some(3.0) — coerção implícita

Dado Value::Bool(true)
Quando cast_float() chamado
Então None

Dado Value::from("hello")
Então Value::Str(EcoString)

Dado Scope::define("x", Value::Int(42))
Quando scope.get("x")
Então Some(&Value::Int(42))
```
