//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/rel.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-06-25
//!
//! P469 — `Rel<T>`: valor relativo (percentual de contexto + offset absoluto).
//! Instanciado para `Length` neste passo.

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::entities::layout_types::Length;

/// Valor relativo: fração de um contexto + offset absoluto.
///
/// Ex: `50% + 2cm` = `Rel { rel: 0.5, abs: Length::cm(2.0) }`.
/// Ex: `100% - 1em` = `Rel { rel: 1.0, abs: Length::em(-1.0) }`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rel<T> {
    /// Fração do contexto (0.5 = 50%).
    pub rel: f64,
    /// Offset absoluto do mesmo tipo base.
    pub abs: T,
}

impl<T: Default> Default for Rel<T> {
    fn default() -> Self {
        Self { rel: 0.0, abs: T::default() }
    }
}

impl<T> Rel<T> {
    /// Constrói um `Rel` a partir de uma percentagem pura.
    pub fn from_percent(pct: f64) -> Self
    where
        T: Default,
    {
        Self { rel: pct / 100.0, abs: T::default() }
    }
}

impl<T: Add<Output = T>> Add for Rel<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { rel: self.rel + rhs.rel, abs: self.abs + rhs.abs }
    }
}

impl<T: Sub<Output = T>> Sub for Rel<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { rel: self.rel - rhs.rel, abs: self.abs - rhs.abs }
    }
}

impl<T: Neg<Output = T>> Neg for Rel<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { rel: -self.rel, abs: -self.abs }
    }
}

impl<T: Mul<f64, Output = T>> Mul<f64> for Rel<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { rel: self.rel * rhs, abs: self.abs * rhs }
    }
}

impl<T: Mul<f64, Output = T>> Mul<Rel<T>> for f64 {
    type Output = Rel<T>;
    fn mul(self, rhs: Rel<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: Div<f64, Output = T>> Div<f64> for Rel<T> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { rel: self.rel / rhs, abs: self.abs / rhs }
    }
}

// ── Instanciação para Length ────────────────────────────────────────────────

impl Rel<Length> {
    /// `Rel<Length>` zero (0% + 0pt).
    pub fn zero() -> Self {
        Self { rel: 0.0, abs: Length::ZERO }
    }

    /// Resolve para comprimento absoluto dado um contexto (ex: largura do container).
    pub fn resolve(&self, context: Length) -> Length {
        context * self.rel + self.abs
    }

    /// Verifica se a parte absoluta é zero.
    pub fn is_abs_zero(&self) -> bool {
        self.abs.is_zero()
    }
}

// Para permitir `Rel<Length> + Length` e `Length + Rel<Length>`.
impl Add<Length> for Rel<Length> {
    type Output = Self;
    fn add(mut self, rhs: Length) -> Self::Output {
        self.abs = self.abs + rhs;
        self
    }
}

impl Add<Rel<Length>> for Length {
    type Output = Rel<Length>;
    fn add(self, rhs: Rel<Length>) -> Self::Output {
        rhs + self
    }
}

impl Sub<Length> for Rel<Length> {
    type Output = Self;
    fn sub(mut self, rhs: Length) -> Self::Output {
        self.abs = self.abs - rhs;
        self
    }
}

impl Sub<Rel<Length>> for Length {
    type Output = Rel<Length>;
    fn sub(self, rhs: Rel<Length>) -> Self::Output {
        let mut r = -rhs;
        r.abs = self + r.abs;
        r
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_from_percent() {
        let r = Rel::<Length>::from_percent(50.0);
        assert_eq!(r.rel, 0.5);
        assert!(r.abs.is_zero());
    }

    #[test]
    fn rel_add_length() {
        let r = Rel::<Length>::from_percent(50.0) + Length::cm(2.0);
        assert_eq!(r.rel, 0.5);
        assert_eq!(r.abs, Length::cm(2.0));
    }

    #[test]
    fn rel_resolve() {
        let r = Rel::<Length>::from_percent(50.0) + Length::cm(2.0);
        let resolved = r.resolve(Length::cm(10.0));
        // 50% de 10cm = 5cm; + 2cm = 7cm.
        // P1093: com razão exacta 3600/127 o resultado difere no último bit
        // de f64 — comparação com tolerância (< 1e-12 pt).
        let expected = Length::cm(7.0);
        assert!(
            (resolved.abs.0 - expected.abs.0).abs() < 1e-12,
            "resolved={:?}, expected={:?}",
            resolved,
            expected
        );
    }

    #[test]
    fn rel_mul_f64() {
        let r = Rel::<Length>::from_percent(50.0) * 2.0;
        assert_eq!(r.rel, 1.0);
        assert!(r.abs.is_zero());
    }

    #[test]
    fn rel_length_addition_commutes() {
        let a = Length::cm(1.0) + Rel::<Length>::from_percent(50.0);
        let b = Rel::<Length>::from_percent(50.0) + Length::cm(1.0);
        assert_eq!(a, b);
    }
}
