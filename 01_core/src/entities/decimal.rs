//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/decimal.md
//! @prompt-hash 2d04eaef
//! @layer L1
//! @updated 2026-06-22
//!
//! Valor decimal de precisão fixa — `Value::Decimal`.
//! Passo 399: tipo L1 puro, zero I/O; usa `rust_decimal::Decimal` (28 dígitos).

pub use rust_decimal::Decimal as InnerDecimal;
use rust_decimal::prelude::*;
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
        let b = Decimal::new(10, 1);  // 1.0
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
