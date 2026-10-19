# Prompt L0 — `Decimal` — fixed-point decimal
Hash do Código: eb3bae87

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/decimal.rs`
**Integration consumer:** Value has its own owner; it is not a second consumer of this L0.

## Evidence before specification

Historical upstream `e0e8ca4d:foundations/decimal.rs:15–18,26–30,72–79` defines fixed-point base10 arithmetic, decimal("...") preserving representable fractional digits,28–29 significant digits and limits ±79228162514264337593543950335 (2^96−1). It has no infinity/NaN; unrepresentable/overflow/underflow operations reject. The old arbitrary-precision or exactly 28-digit wording is superseded by these limits. rust_decimal matches this representation; bigdecimal heap-backed arbitrary precision is outside scope.

Historical public-type probes in `lab/results/construction/diagnostics/superficie-linguagem-p1140*` against pin `a51e02804` found repr(type(PATH))=`type` upstream versus `function` locally for seven constructors. The catalog and 22 probes retained baseline bytes after extraction. This is public language semantics, not Rust mechanics.

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` with uncommitted tree, `lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz` records source/owner/spans/UTC/status/diffstat for manual Default at decimal.rs:127. Its nongeneric body exactly delegates to InnerDecimal default, allowing derive without new public bounds. `lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/before.json.gz` freezes the 13-warning interface inventory: from_str's Option differs from FromStr's Result and error contract. These are mechanical checks, not parity/performance/readability measurements.

## Data, construction and conversion

```rust
pub use rust_decimal::Decimal as InnerDecimal;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Decimal(pub InnerDecimal);
```

Retain the pure Rust 128-bit stack value with no allocation. new(num:i64,scale:u32) delegates to InnerDecimal::new; from_str(&str)->Option<Self> wraps parser success; from_f64(f64)->Option<Self> uses its documented lossy conversion and rejects NaN/Inf; from_i64 is exact. Derived Default delegates only to InnerDecimal::default, preserving mantissa/scale/numeric methods/other traits.

Value::Decimal retains type_name `decimal`, equality/hash integration, cast_decimal borrowing and From<Decimal>. Identity casts retain Decimal; Int/Float/Str follow the conversions above, other types reject. Global decimal is Value::Type(Type::Decimal), called through the static dispatcher and existing constructor: type(decimal)==type and repr(type(decimal))==`type`. Native-constructor deferral in the original introduction is historical, not a current prohibition.

Keep from_str's Option and consumer-owned language diagnostics; do not invent FromStr/Err or alter rejection. Its sole nominal expectation is `#[expect(clippy::should_implement_trait, reason="S003: parser de domínio Option preservado; FromStr exige Result e um contrato de erro distinto")]`. No impl/module/crate suppression, threshold change or extension to Styles::from_iter is permitted.

## Ratified Display contract

The formatting/construction package's base.json.gz freezes four implementations/L0/hash/HEAD/tree/UTC/diffstat. Option parsers and domain next methods were deliberately classified separately; the duration-before-probe documents its i128 range correction without copying an upstream defect. The owner approved manifest `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8` on 2026-10-16, receipt `lab/results/construction/diagnostics/clippy-enums-2026-10-16/approval-receipt.json`. This satisfies the former public/compatibility gate for three Display implementations, one FromIterator and three inherent-to-blanket ToString migrations, including T1–T5 but no release.

Implement std::fmt::Display by delegating to InnerDecimal Display; remove only the inherent public to_string. Normal and UFCS ToString use the blanket trait. Preserve decimal text/scale/trailing zeros/signs and no new scientific notation. Earlier to_string examples now denote this ToString interface. Do not add FromStr, fields/variants/flags/defaults/phases/dependencies/suppressions or modify arithmetic/round/casts/Default. Generic trait reliance makes rollback a compatibility change.

## Acceptance and incomplete scope

Preserve new/from_str/from_i64/from_f64,1.0==1.00, Copy/Default, Value discrimination/type_name/cast/From and valid/invalid Int/Float/Str controls. The initial pure-entity slice deferred arithmetic (+,−,*,/,%,pow) and comparison consumers despite local PartialOrd; this is integration scope, not proof they remain absent globally. No decimal-suffix literal exists in the referenced language.

For nominal lint maintenance require L0 first, restricted reseal, lint RED→GREEN with unfulfilled expectations as error and unchanged functional bodies/expectations. For the approved trait package T1 requires generic Display+ToString/collect::<Styles> RED before traits and GREEN afterward; T2 retains Decimal1.2300 zeros, negative/zero/Option rejection and Version empty/1.2.0 components; T3 retains corrected Duration MIN/MAX/MIN+1/zero/negative boundaries through format!/ToString; T4 empty collect and Weight400,700→Some700 with other fields unchanged and inherent adapter compatibility; T5 preserves language/cast/repr/show/CLI and full workspace/build/Clippy/expectations/Rustdoc/fmt/lineage. Use 12 GiB, swap 0, one job, unchanged expected outputs. No release is authorized by that package.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/decimal.md` preserves the original evidence and chronology.
