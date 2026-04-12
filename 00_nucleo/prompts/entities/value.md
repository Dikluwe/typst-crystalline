# Prompt L0 — `entities/value`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/value.rs`
**Criado em**: 2026-03-22 (Passo 13)
**Atualizado em**: 2026-04-12 (restauro — expansão para refletir Passos 15–25)
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0023 (indexmap em L1), ADR-0024 (EcoString em Value::Str), ADR-0025 (Int == Float), ADR-0028/ADR-0029 (tipos tipográficos)

---

## Contexto

`Value` é o **tipo de runtime** do compilador Typst — o enum que representa
todos os valores possíveis durante a avaliação (`eval.rs`). O original tem
~35 variantes. A migração adiciona variantes incrementalmente, protegida pela
regra: **não adicionar variantes sem ADR e tipo migrado**.

### Estado actual (Passo 25)

16 variantes implementadas:

- **Passo 13** — 5 primitivos: `None`, `Bool`, `Int`, `Float`, `Str`
- **Passo 15** — 4 variantes compostas: `Array`, `Dict`, `Module`, `Datetime`
- **Passo 16** — `Func`
- **Passo 18** — `Content`
- **Passo 25** — 5 tipos tipográficos: `Auto`, `Length`, `Ratio`, `Angle`, `Color`

~14 variantes futuras permanecem comentadas no código (não implementar sem ADR).

---

## Decisão sobre EcoString (ADR-0024)

`Value::Str` usa `EcoString` (crate `ecow`), não `String`. Durante `eval()`,
strings são passadas como argumentos, capturadas em closures e concatenadas
frequentemente. Clone com `EcoString` é O(1) para strings não mutadas
(copy-on-write). Com `String` seria O(n) em cada passagem — degradação
de performance no hot path.

Contraste com ADR-0015 (que removeu `ecow` do parser): o parser constrói strings
uma vez; `Value::Str` é clonado no hot path de `eval()`. Contextos opostos.

## Decisão sobre Dict (ADR-0023)

`Value::Dict` usa `IndexMap<EcoString, Value, FxBuildHasher>` para preservar
ordem de inserção e manter ergonomia com o Typst original. `indexmap` e
`rustc_hash` são autorizados em L1.

## Decisão sobre igualdade Int/Float (ADR-0025)

`Value` usa `#[derive(PartialEq)]`, portanto `Value::Int(1) == Value::Float(1.0)`
é `false` em Rust. Para manter paridade com a semântica do Typst original,
`eval_binary_op` em `eval.rs` trata `Eq`/`Neq` com tipos mistos explicitamente
(coerção `i as f64`). **Dois sistemas de igualdade coexistem**:
igualdade Rust (para testes e estruturas de dados) vs igualdade Typst (em eval).

---

## Restrições Estruturais

- Camada **L1**: zero I/O. `Arc` em `Module`, `Func` é gestão de RAM (ADR-0029).
- Fronteira deliberada: `_ => Ok(Value::None)` em `eval.rs` para variantes não
  implementadas — não adicionar variantes sem ADR.
- `serde` nunca entra em L1 — serialização é responsabilidade de L3 via DTO.
- `Value::Array` usa `Vec<Value>` — clone O(n). Documentado como dívida técnica.
  `Value::Dict` usa `IndexMap` com `FxBuildHasher`.

---

## Interface pública

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Passo 13 — primitivos
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(EcoString),              // ADR-0024

    // Passo 15 — compostos
    Array(Vec<Value>),
    Dict(IndexMap<EcoString, Value, FxBuildHasher>),  // ADR-0023
    Module(Module),              // Arc<ModuleInner> internamente — clone O(1)
    Datetime(Datetime),

    // Passo 16
    Func(Func),                  // Arc<FuncRepr> internamente — clone O(1)

    // Passo 18
    Content(Content),

    // Passo 25 — tipos tipográficos (ADR-0028/0029)
    Auto,
    Length(Length),
    Ratio(Ratio),
    Angle(Angle),
    Color(Color),

    // ~14 variantes futuras comentadas — NÃO implementar sem ADR e tipo migrado
}

impl Value {
    pub fn type_name(&self) -> &'static str  // "none", "bool", "int", "float", "str",
                                              // "array", "dictionary", "module", "datetime",
                                              // "function", "content", "auto",
                                              // "length", "ratio", "angle", "color"
    pub fn is_none(&self) -> bool
    pub fn cast_bool(&self)  -> Option<bool>
    pub fn cast_int(&self)   -> Option<i64>
    pub fn cast_float(&self) -> Option<f64>  // aceita Int (coerção implícita)
    pub fn cast_str(&self)   -> Option<&str>
    pub fn cast_array(&self) -> Option<&[Value]>
    pub fn cast_dict(&self)  -> Option<&IndexMap<EcoString, Value, FxBuildHasher>>
}

// Conversões From para ergonomia em eval() e testes
impl From<bool>       for Value
impl From<i64>        for Value
impl From<i32>        for Value
impl From<f64>        for Value
impl From<EcoString>  for Value
impl From<&str>       for Value
impl From<String>     for Value
impl From<Vec<Value>> for Value
impl From<IndexMap<EcoString, Value, FxBuildHasher>> for Value
impl From<Module>     for Value
impl From<Datetime>   for Value
impl From<Func>       for Value
impl From<Content>    for Value
impl From<Length>     for Value
impl From<Ratio>      for Value
impl From<Angle>      for Value
impl From<Color>      for Value
```

---

## Critérios de Verificação

```
Value::None.type_name()         = "none"
Value::None.is_none()           = true
Value::Int(0).is_none()         = false

Value::Int(3).cast_float()      = Some(3.0)     // coerção implícita
Value::Float(3.14).cast_float() = Some(3.14)
Value::Bool(true).cast_float()  = None

Value::from("hello")            → Value::Str(EcoString)
Value::from(42i64)              → Value::Int(42)
Value::from(3.14f64)            → Value::Float(3.14)
Value::from(true)               → Value::Bool(true)

// EcoString — clone O(1) e igualdade
Value::Str("a".into()) == Value::Str("a".into())  = true
Value::Str("a".into()) != Value::Str("b".into())

// Array — clone O(n), independência garantida
let v1 = Value::Array(vec![Value::Int(1)]);
let mut v2 = v1.clone();
// mutar v2 não afecta v1

// Dict
Value::Dict(m).type_name() = "dictionary"
cast_dict().unwrap().get("k") = Some(&Value::Int(1))

// Module — clone O(1) via Arc
Value::Module(m1) clone → type_name() ainda "module"

// Tipos tipográficos (Passo 25)
Value::Length(Length::pt(12.0)).type_name() = "length"
Value::Ratio(Ratio(0.5)).type_name()        = "ratio"
Value::Angle(Angle::deg(90.0)).type_name()  = "angle"
Value::Color(Color::rgb(0,0,0)).type_name() = "color"
Value::Auto.type_name()                     = "auto"

// Scope integration
Scope::define("x", Value::Int(42))
scope.get("x") = Some(&Value::Int(42))
```

---

## Variantes futuras (comentadas no código — NÃO implementar sem ADR)

`Relative`, `Fraction`, `Gradient`, `Tiling`, `Symbol`, `Version`,
`Bytes`, `Decimal`, `Duration`, `Styles`, `Args`, `Type`, `Dyn` (~13 restantes)

---

## Resultado Esperado

- `01_core/src/entities/value.rs` com todas as variantes até ao Passo 25
- Testes co-localizados em `#[cfg(test)]` cobrindo cada variante e os critérios acima

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 13: 5 primitivos | `value.rs` |
| 2026-03-24 | Passo 15: Array, Dict, Module, Datetime | `value.rs` |
| 2026-03-25 | Passo 16: Func | `value.rs` |
| 2026-03-26 | Passo 18: Content | `value.rs` |
| 2026-03-28 | Passo 25: Auto, Length, Ratio, Angle, Color (ADR-0028) | `value.rs` |
| 2026-04-12 | Restauro — prompt expandido para refletir Passos 15–25; sem mudanças no código | `value.md` |
