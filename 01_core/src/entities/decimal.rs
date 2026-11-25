//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/decimal.md
//! @prompt-hash e2c0c297
//! @layer L1
//! @updated 2026-06-22
//!
//! Valor decimal de precisão fixa — `Value::Decimal`.
//! Passo 399: tipo L1 puro, zero I/O; usa `rust_decimal::Decimal` (28 dígitos).

use rust_decimal::prelude::*;
pub use rust_decimal::Decimal as InnerDecimal;
use std::str::FromStr;

/// Valor decimal com precisão fixa (28 dígitos).
///
/// Wrapper sobre `rust_decimal::Decimal` — é `Copy` (128-bit stack value),
/// puro-Rust e sem alocação dinâmica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Decimal(pub InnerDecimal);

impl Decimal {
    /// Cria um decimal a partir de uma mantissa e escala.
    ///
    /// Exemplo: `Decimal::new(12345, 2)` representa `123.45`.
    pub fn new(num: i64, scale: u32) -> Self {
        Self(InnerDecimal::new(num, scale))
    }

    /// Parse a partir de string decimal.
    #[expect(
        clippy::should_implement_trait,
        reason = "S003: parser de domínio Option preservado; FromStr exige Result e um contrato de erro distinto"
    )]
    pub fn from_str(s: &str) -> Option<Self> {
        InnerDecimal::from_str(s).ok().map(Self)
    }

    /// Conversão a partir de `f64` (pode perder precisão; rejeita NaN/Inf).
    pub fn from_f64(f: f64) -> Option<Self> {
        InnerDecimal::from_f64(f).map(Self)
    }

    /// Conversão exata a partir de `i64`.
    pub fn from_i64(i: i64) -> Self {
        Self(InnerDecimal::from(i))
    }

    /// Valor absoluto (paridade vanilla `calc.abs(decimal)`).
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Arredonda para -∞ (paridade vanilla `calc.floor(decimal)`).
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Arredonda para +∞ (paridade vanilla `calc.ceil(decimal)`).
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Trunca em direcção a zero (paridade vanilla `calc.trunc(decimal)`).
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    /// Parte fraccionária (paridade vanilla `calc.fract(decimal)`).
    pub fn fract(self) -> Self {
        Self(self.0.fract())
    }

    /// Conversão para `i64` (`None` se fora do alcance ou com parte
    /// fraccionária não nula — usar após `floor`/`ceil`/`trunc`).
    pub fn to_i64(self) -> Option<i64> {
        i64::try_from(self.0).ok()
    }

    pub(crate) fn checked_div(self, other: Self) -> Option<Self> {
        self.0.checked_div(other.0).map(Self)
    }

    pub(crate) fn checked_rem(self, other: Self) -> Option<Self> {
        self.0.checked_rem(other.0).map(Self)
    }

    pub(crate) fn checked_div_euclid(self, other: Self) -> Option<Self> {
        let quotient = self.0.checked_div(other.0)?.trunc();
        let remainder = self.0.checked_rem(other.0)?;
        if remainder >= InnerDecimal::ZERO {
            Some(Self(quotient))
        } else if other.0.is_sign_positive() {
            quotient.checked_sub(InnerDecimal::ONE).map(Self)
        } else {
            quotient.checked_add(InnerDecimal::ONE).map(Self)
        }
    }

    pub(crate) fn checked_rem_euclid(self, other: Self) -> Option<Self> {
        let remainder = self.0.checked_rem(other.0)?;
        if remainder >= InnerDecimal::ZERO {
            Some(Self(remainder))
        } else {
            remainder.checked_add(other.0.abs()).map(Self)
        }
    }

    /// Potência inteira com verificação de overflow (paridade vanilla
    /// `calc.pow(decimal, int)` → `rust_decimal::Decimal::checked_powi`).
    pub fn checked_powi(self, exp: i64) -> Option<Self> {
        rust_decimal::MathematicalOps::checked_powi(&self.0, exp).map(Self)
    }

    /// Arredondamento com `digits` casas, midpoint **away from zero**
    /// (paridade vanilla `calc.round(decimal, digits:)`; vanilla
    /// `foundations/decimal.rs:159`). `digits` negativo arredonda
    /// antes do ponto decimal (`round(3333.45, digits: -2) = 3300`).
    pub fn round_with_digits(self, digits: i32) -> Option<Self> {
        use rust_decimal::RoundingStrategy;
        if let Ok(positive) = u32::try_from(digits) {
            return Some(Self(self.0.round_dp_with_strategy(
                positive,
                RoundingStrategy::MidpointAwayFromZero,
            )));
        }
        // `digits` negativo: arredonda para múltiplos de 10^(-digits).
        // `digits == i32::MIN` overflow a negar — tratar como "além de
        // qualquer dígito inteiro possível" (resultado zero com sinal).
        let Some(ndigits) = digits.checked_neg().and_then(|d| u32::try_from(d).ok())
        else {
            let mut zero = InnerDecimal::ZERO;
            zero.set_sign_negative(self.0.is_sign_negative());
            return Some(Self(zero));
        };
        let mut num = self.0;
        let old_scale = num.scale();
        let ten_to_digits = rust_decimal::MathematicalOps::checked_powi(
            &InnerDecimal::TEN,
            i64::from(ndigits),
        );
        let (Ok(_), Some(factor)) = (num.set_scale(old_scale + ndigits), ten_to_digits)
        else {
            // Escalar mais do que qualquer quantidade de dígitos inteiros.
            let mut zero = InnerDecimal::ZERO;
            zero.set_sign_negative(self.0.is_sign_negative());
            return Some(Self(zero));
        };
        // Após `set_scale(old_scale + ndigits)`, `num` vale self / 10^ndigits.
        let rounded =
            num.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero);
        rounded.checked_mul(factor).map(Self)
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn approved_traits_decimal_display_preserves_scale() {
        fn render<T: std::fmt::Display + ToString>(value: &T) -> (String, String) {
            (format!("{value}"), ToString::to_string(value))
        }
        for (value, expected) in [
            (Decimal::new(12300, 4), "1.2300"),
            (Decimal::new(-12300, 4), "-1.2300"),
            (Decimal::default(), "0"),
        ] {
            assert_eq!(render(&value), (expected.into(), expected.into()));
        }
        assert!(Decimal::from_str("invalid").is_none());
    }

    use super::*;

    #[test]
    fn checked_division_covers_signs_zero_and_overflow() {
        let d = Decimal::from_i64;
        for (a, b, quotient, remainder) in
            [(7, 2, 3, 1), (-7, 2, -4, 1), (7, -2, -3, 1), (-7, -2, 4, 1), (-6, 2, -3, 0)]
        {
            assert_eq!(d(a).checked_div_euclid(d(b)), Some(d(quotient)));
            assert_eq!(d(a).checked_rem_euclid(d(b)), Some(d(remainder)));
        }
        assert_eq!(d(-7).checked_rem(d(2)), Some(d(-1)));
        assert_eq!(d(1).checked_div(d(0)), None);
        assert_eq!(d(1).checked_rem(d(0)), None);
        assert_eq!(d(1).checked_div_euclid(d(0)), None);
        assert_eq!(d(1).checked_rem_euclid(d(0)), None);
        let max = Decimal::from_str("79228162514264337593543950335").unwrap();
        assert_eq!(max.checked_div(Decimal::new(1, 1)), None);
    }

    #[test]
    fn decimal_new() {
        let d = Decimal::new(12345, 2);
        assert_eq!(d.to_string(), "123.45");
    }

    #[test]
    fn decimal_from_str_ok() {
        let d = Decimal::from_str("1.2345678901234567890123456789").unwrap();
        assert_eq!(d.to_string(), "1.2345678901234567890123456789");
    }

    #[test]
    fn decimal_from_str_invalid() {
        assert!(Decimal::from_str("abc").is_none());
        assert!(Decimal::from_str("").is_none());
    }

    #[test]
    fn decimal_from_f64() {
        let d = Decimal::from_f64(1.5).unwrap();
        assert_eq!(d.to_string(), "1.5");
    }

    #[test]
    fn decimal_from_f64_nan_rejected() {
        assert!(Decimal::from_f64(f64::NAN).is_none());
        assert!(Decimal::from_f64(f64::INFINITY).is_none());
        assert!(Decimal::from_f64(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn decimal_from_i64() {
        let d = Decimal::from_i64(42);
        assert_eq!(d.to_string(), "42");
    }

    #[test]
    fn decimal_equality_ignores_trailing_zeros() {
        let a = Decimal::new(100, 2); // 1.00
        let b = Decimal::new(10, 1); // 1.0
        assert_eq!(a, b);
    }

    #[test]
    fn decimal_copy() {
        let a = Decimal::new(7, 0);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn decimal_default_is_zero() {
        assert_eq!(Decimal::default().to_string(), "0");
    }
}
