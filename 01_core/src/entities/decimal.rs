//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/decimal.md
//! @prompt-hash f0e71379
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal(pub InnerDecimal);

impl Decimal {
    /// Cria um decimal a partir de uma mantissa e escala.
    ///
    /// Exemplo: `Decimal::new(12345, 2)` representa `123.45`.
    pub fn new(num: i64, scale: u32) -> Self {
        Self(InnerDecimal::new(num, scale))
    }

    /// Parse a partir de string decimal.
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

    /// Representação textual decimal pura (sem notação científica).
    pub fn to_string(&self) -> String {
        self.0.to_string()
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

impl Default for Decimal {
    fn default() -> Self {
        Self(InnerDecimal::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
