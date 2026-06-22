# Prompt L0 — `Decimal` — precisão fixa decimal
Hash do Código: a13d637d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/decimal.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 399 — modelagem de `Value::Decimal` (S); primeiro tipo S puro após abertura do portão ADR-0017.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (operações aritméticas scope-out).

---

## 1. Contexto

O Typst vanilla expõe `decimal` como tipo de precisão arbitrária (função `decimal("...")`). Em cristalino, o variant `Value::Decimal` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto em P395 e `Bytes` modelado em P398, modela-se agora `Decimal` como tipo puro.

## 2. Tipo L1

```rust
// 01_core/src/entities/decimal.rs
pub use rust_decimal::Decimal as InnerDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);
```

- `rust_decimal::Decimal` é puro-Rust, `Copy` (128-bit stack value), zero alloc.
- Precisão fixa 28 dígitos — suficiente para paridade linguagem (ADR-0107).
- `bigdecimal` (precisão arbitrária com heap) é scope-out; `rust_decimal` é a opção pragmática.

## 3. Construtores e acesso

```rust
impl Decimal {
    pub fn new(num: i64, scale: u32) -> Self { Self(InnerDecimal::new(num, scale)) }
    pub fn from_str(s: &str) -> Option<Self> { InnerDecimal::from_str(s).ok().map(Self) }
    pub fn from_f64(f: f64) -> Option<Self> { InnerDecimal::from_f64(f).map(Self) }
    pub fn from_i64(i: i64) -> Self { Self(InnerDecimal::from(i)) }
    pub fn to_string(&self) -> String { self.0.to_string() }
}

impl Default for Decimal {
    fn default() -> Self { Self(InnerDecimal::default()) }
}
```

## 4. Variant `Value::Decimal`

```rust
// 01_core/src/entities/value.rs
Decimal(crate::entities::decimal::Decimal),
```

Atualizações necessárias:
- `type_name()` → `"decimal"`.
- `PartialEq` via derive (a struct já implementa).
- `Hash` via derive/Debug formatting (padrão existente).
- `cast_decimal()` para extrair `&Decimal`.
- `From<Decimal> for Value`.

## 5. Cast

- `Value::Decimal(d) -> Decimal`: identidade.
- `Value::Int(i) -> Decimal`: `Decimal::from_i64(i)` (preciso).
- `Value::Float(f) -> Decimal`: `Decimal::from_f64(f)` (com perda documentada; `None` para NaN/Inf).
- `Value::Str(s) -> Decimal`: `Decimal::from_str(s)` (fallible).
- Outros tipos → erro de tipo.

## 6. Repr

Quando existir `native_repr`, deve devolver a string literal decimal pura (`"123.45"`), sem notação científica. `rust_decimal::Decimal::to_string()` já produz esse formato.

## 7. Scope-out

- `native_decimal(...)` stdlib — passo futuro (P400 ou agregado).
- Operações aritméticas (`+`, `-`, `*`, `/`, `%`, `pow`) — scope-out ADR-0054 graded.
- Comparações (`<`, `>`, `<=`, `>=`) — `PartialOrd` existe no tipo; consumer futuro.
- Literal suffixo decimal — não existe no vanilla.

## 8. Testes

- Construção `Decimal::new`, `from_str`, `from_i64`, `from_f64`.
- Igualdade (`1.0 == 1.00`), `Copy`, `Default`.
- `Value::Decimal` discriminação, `type_name`, `cast_decimal`, `From`.
- Cast a partir de `Int`, `Float`, `Str` (válido e inválido).
