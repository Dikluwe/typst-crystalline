# Prompt L0 — `Decimal` — precisão fixa decimal
Hash do Código: c4ae9676

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/decimal.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 399 — modelagem de `Value::Decimal` (S); primeiro tipo S puro após abertura do portão ADR-0017.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (operações aritméticas scope-out).

---

## 1. Contexto

O Typst vanilla expõe `decimal` como tipo de vírgula fixa em base 10 (função `decimal("...")`) — **não** de precisão arbitrária; ver a citação em §2. Em cristalino, o variant `Value::Decimal` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto em P395 e `Bytes` modelado em P398, modela-se agora `Decimal` como tipo puro.

## 2. Tipo L1

```rust
// 01_core/src/entities/decimal.rs
pub use rust_decimal::Decimal as InnerDecimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);
```

- `rust_decimal::Decimal` é puro-Rust, `Copy` (128-bit stack value), zero alloc.
- Precisão fixa 28 dígitos — suficiente para paridade linguagem (ADR-0107).

> **Fonte de paridade (P1031)** — doc comment `#[ty]` do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/foundations/decimal.rs`, publicado em
> `typst.app/docs/reference/foundations/decimal/`:
>
> - **Tipo e propósito** — `decimal.rs:15-18`: *"A fixed-point decimal number type. This type
>   should be used for precise arithmetic operations on numbers represented in base 10. A
>   typical use case is representing currency."*
> - **Construtor por string** — `decimal.rs:26-30`: *"To create a decimal number, use the
>   `{decimal(string)}` constructor, such as in `{decimal("3.141592653")}` _(note the double
>   quotes)._ This constructor preserves all given fractional digits, provided they are
>   representable as per the limits specified below (otherwise, an error is raised)."*
>   Confirma a forma de superfície `decimal("...")` do contexto acima.
> - **Limites** — `decimal.rs:72-79`: *"A `decimal` number has a limit of **28 to 29**
>   significant base-10 digits. […] The maximum and minimum `decimal` numbers have a value of
>   `{79228162514264337593543950335}` and `{-79228162514264337593543950335}` respectively. In
>   contrast with `float`, this type does not support infinity or NaN, so overflowing or
>   underflowing operations will raise an error."*
>
> **Precisão da redacção acima**: *"precisão de arbitrária"* no contexto e *"precisão fixa
> 28 dígitos"* aqui são ambas aproximações. A linguagem diz **fixed-point**, com **28 a 29**
> dígitos significativos (não exactamente 28) e um máximo nomeado, que é exactamente
> `2^96 − 1` — o mesmo limite de `rust_decimal`. A escolha de `rust_decimal` está, portanto,
> alinhada com a linguagem por construção, e não só "suficiente". Corrigido o enquadramento;
> os números permanecem os do vanilla.
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
