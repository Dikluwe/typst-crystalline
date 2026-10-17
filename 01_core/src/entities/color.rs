//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/color.md
//! @prompt-hash aa9413e7
//! @layer L1
//! @updated 2026-05-15
//!
//! **P257 (M9d / M7+5; ADR-0083 PROPOSTO Color paridade vanilla
//! com subset materializado)** — refactor de `entities/layout_types.rs::Color`
//! (`enum { Rgb, Rgba }` simplificado P25) para paridade
//! estrutural vanilla com 8 variantes (sRGB, Luma, LinearRgb,
//! Oklab, Oklch, Hsl, Hsv, Cmyk).
//!
//! Cumpre ADR-0029 §"Diagnosticar primeiro" + §"Simplificações
//! aceites apenas com ADR explícita". Scope-outs formalizados
//! em ADR-0083 (PDF native CMYK + operadores cor + ColorSpace
//! runtime + constantes nomeadas extras).
//!
//! Paridade observable estricta preservada: `Color::rgb(255, 0, 0)`
//! produz mesmos bytes PDF antes e depois de P257.

mod adjustments;
mod conversion;
#[cfg(test)]
use self::conversion::*;
mod mixing;

use crate::entities::layout_types::Ratio;
/// Representa uma cor em um de 8 espaços de cor (paridade
/// estrutural vanilla — `lab/typst-original/crates/typst-library/src/visualize/color.rs:194`).
///
/// **8 variantes** correspondendo aos 8 espaços vanilla.
/// Representação interna `f32` (paridade vanilla). Construtores
/// u8 (`Color::rgb(255, 0, 0)`) preservados para paridade
/// observable cristalino existente.
///
/// `PartialEq` derivado é exacto via `f32` bitwise (sem
/// tolerância em produção; paridade ADR-0028 regra herdada).
use std::sync::Arc;

#[derive(Clone)]
pub enum Color {
    /// sRGB color space (paridade vanilla `Rgb`).
    /// Componentes normalizados [0.0, 1.0]. Espaço default
    /// para input user (`rgb(r,g,b)` constrói este).
    Srgb { r: f32, g: f32, b: f32, a: f32 },
    /// D65 grayscale (paridade vanilla `Luma`).
    /// `l` = lightness [0.0, 1.0]; `a` = alpha.
    Luma { l: f32, a: f32 },
    /// Linear RGB color space (paridade vanilla `LinearRgb`).
    /// Componentes lineares (sem gamma); usado em conversões
    /// para/de Oklab.
    LinearRgb { r: f32, g: f32, b: f32, a: f32 },
    /// Oklab perceptual color space (paridade vanilla `Oklab`).
    /// `l` = lightness; `a`, `b` = canais opponent;
    /// `alpha` = transparência.
    Oklab { l: f32, a: f32, b: f32, alpha: f32 },
    /// Oklch — Oklab em coordenadas polares.
    /// `c` = chroma; `h` = hue (graus).
    Oklch { l: f32, c: f32, h: f32, alpha: f32 },
    /// HSL color space (paridade vanilla `Hsl`).
    /// `h` em graus; `s`, `l` normalizados.
    Hsl { h: f32, s: f32, l: f32, a: f32 },
    /// HSV color space (paridade vanilla `Hsv`).
    /// `h` em graus; `s`, `v` normalizados.
    Hsv { h: f32, s: f32, v: f32, a: f32 },
    /// CMYK color space (print). Componentes [0.0, 1.0].
    /// **PDF native `/DeviceCMYK` scope-out P257** —
    /// converte para sRGB no exporter (ADR-0083 §"Scope-out
    /// PDF native CMYK").
    Cmyk { c: f32, m: f32, y: f32, k: f32 },
    /// Cor spot com colorante e concentração preservados.
    Spot(SpotColor),
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Self::Srgb { r, g, b, a } => {
                if r == 0.0 && g == 0.454902 && b == 0.85098 && a == 1.0 {
                    f.write_str("blue")
                } else {
                    f.debug_struct("Srgb")
                        .field("r", &r)
                        .field("g", &g)
                        .field("b", &b)
                        .field("a", &a)
                        .finish()
                }
            }
            Self::Luma { l, a } => {
                f.debug_struct("Luma").field("l", &l).field("a", &a).finish()
            }
            Self::LinearRgb { r, g, b, a } => f
                .debug_struct("LinearRgb")
                .field("r", &r)
                .field("g", &g)
                .field("b", &b)
                .field("a", &a)
                .finish(),
            Self::Oklab { l, a, b, alpha } => f
                .debug_struct("Oklab")
                .field("l", &l)
                .field("a", &a)
                .field("b", &b)
                .field("alpha", &alpha)
                .finish(),
            Self::Oklch { l, c, h, alpha } => f
                .debug_struct("Oklch")
                .field("l", &l)
                .field("c", &c)
                .field("h", &h)
                .field("alpha", &alpha)
                .finish(),
            Self::Hsl { h, s, l, a } => f
                .debug_struct("Hsl")
                .field("h", &h)
                .field("s", &s)
                .field("l", &l)
                .field("a", &a)
                .finish(),
            Self::Hsv { h, s, v, a } => f
                .debug_struct("Hsv")
                .field("h", &h)
                .field("s", &s)
                .field("v", &v)
                .field("a", &a)
                .finish(),
            Self::Cmyk { c, m, y, k } => f
                .debug_struct("Cmyk")
                .field("c", &c)
                .field("m", &m)
                .field("y", &y)
                .field("k", &k)
                .finish(),
            Self::Spot(spot) => f.debug_tuple("Spot").field(&spot).finish(),
        }
    }
}

impl PartialEq for Color {
    /// Comparação exacta via `f32::to_bits` (sem tolerância;
    /// paridade ADR-0028 regra herdada "sem tolerância em produção").
    /// Cores em variantes diferentes nunca são iguais (paridade
    /// vanilla — `Srgb(r=1)` ≠ `Luma(l=1)`).
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Srgb { r: r1, g: g1, b: b1, a: a1 },
                Self::Srgb { r: r2, g: g2, b: b2, a: a2 },
            ) => {
                r1.to_bits() == r2.to_bits()
                    && g1.to_bits() == g2.to_bits()
                    && b1.to_bits() == b2.to_bits()
                    && a1.to_bits() == a2.to_bits()
            }
            (Self::Luma { l: l1, a: a1 }, Self::Luma { l: l2, a: a2 }) => {
                l1.to_bits() == l2.to_bits() && a1.to_bits() == a2.to_bits()
            }
            (
                Self::LinearRgb { r: r1, g: g1, b: b1, a: a1 },
                Self::LinearRgb { r: r2, g: g2, b: b2, a: a2 },
            ) => {
                r1.to_bits() == r2.to_bits()
                    && g1.to_bits() == g2.to_bits()
                    && b1.to_bits() == b2.to_bits()
                    && a1.to_bits() == a2.to_bits()
            }
            (
                Self::Oklab { l: l1, a: a1, b: b1, alpha: alpha1 },
                Self::Oklab { l: l2, a: a2, b: b2, alpha: alpha2 },
            ) => {
                l1.to_bits() == l2.to_bits()
                    && a1.to_bits() == a2.to_bits()
                    && b1.to_bits() == b2.to_bits()
                    && alpha1.to_bits() == alpha2.to_bits()
            }
            (
                Self::Oklch { l: l1, c: c1, h: h1, alpha: alpha1 },
                Self::Oklch { l: l2, c: c2, h: h2, alpha: alpha2 },
            ) => {
                l1.to_bits() == l2.to_bits()
                    && c1.to_bits() == c2.to_bits()
                    && h1.to_bits() == h2.to_bits()
                    && alpha1.to_bits() == alpha2.to_bits()
            }
            (
                Self::Hsl { h: h1, s: s1, l: l1, a: a1 },
                Self::Hsl { h: h2, s: s2, l: l2, a: a2 },
            ) => {
                h1.to_bits() == h2.to_bits()
                    && s1.to_bits() == s2.to_bits()
                    && l1.to_bits() == l2.to_bits()
                    && a1.to_bits() == a2.to_bits()
            }
            (
                Self::Hsv { h: h1, s: s1, v: v1, a: a1 },
                Self::Hsv { h: h2, s: s2, v: v2, a: a2 },
            ) => {
                h1.to_bits() == h2.to_bits()
                    && s1.to_bits() == s2.to_bits()
                    && v1.to_bits() == v2.to_bits()
                    && a1.to_bits() == a2.to_bits()
            }
            (
                Self::Cmyk { c: c1, m: m1, y: y1, k: k1 },
                Self::Cmyk { c: c2, m: m2, y: y2, k: k2 },
            ) => {
                c1.to_bits() == c2.to_bits()
                    && m1.to_bits() == m2.to_bits()
                    && y1.to_bits() == y2.to_bits()
                    && k1.to_bits() == k2.to_bits()
            }
            (Self::Spot(a), Self::Spot(b)) => a == b,
            (
                Self::Srgb { .. }
                | Self::Luma { .. }
                | Self::LinearRgb { .. }
                | Self::Oklab { .. }
                | Self::Oklch { .. }
                | Self::Hsl { .. }
                | Self::Hsv { .. }
                | Self::Cmyk { .. }
                | Self::Spot(_),
                _,
            ) => false,
        }
    }
}

impl Color {
    // ── Construtores ─────────────────────────────────────────

    /// Constrói sRGB a partir de bytes u8 (paridade cristalino
    /// existente; alpha = 1.0).
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Srgb {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Constrói sRGB a partir de bytes u8 com alpha explícito.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Srgb {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Constrói sRGB direct f32 (sem normalização).
    pub fn srgb_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::Srgb { r, g, b, a }
    }

    /// Constrói Luma com alpha = 1.0.
    pub fn luma(l: f32) -> Self {
        Self::Luma { l, a: 1.0 }
    }

    /// Constrói LinearRgb.
    pub fn linear_rgb(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::LinearRgb { r, g, b, a }
    }

    /// Constrói Oklab.
    pub fn oklab(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self::Oklab { l, a, b, alpha }
    }

    /// Constrói Oklch.
    pub fn oklch(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        Self::Oklch { l, c, h, alpha }
    }

    /// Constrói Hsl (h em graus; s, l normalizados).
    pub fn hsl(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self::Hsl { h, s, l, a }
    }

    /// Constrói Hsv (h em graus; s, v normalizados).
    pub fn hsv(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self::Hsv { h, s, v, a }
    }

    /// Constrói Cmyk (componentes [0.0, 1.0]).
    pub fn cmyk(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self::Cmyk { c, m, y, k }
    }
}

// ── ColorSpace enum (P270) ──────────────────────────────────

/// Enumeração dos 8 ColorSpace materializados P257 (paridade vanilla).
///
/// **P270** — criado para suportar `gradient.linear/radial/conic`
/// `space:` named arg cross-variant (ADR-0091 EM VIGOR).
///
/// `Luma` ≡ vanilla `D65Gray` (nome cristalino histórico P257).
/// Demais 7 variants paridade nominal vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpace {
    Spot(SpotColorant),
    Oklab,
    Oklch,
    Srgb,
    Luma,
    LinearRgb,
    Hsl,
    Hsv,
    Cmyk,
}

/// Rejeição de conversões que fabricariam ou trocariam uma tinta spot.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpaceError {
    ProcessToSpot { target: SpotColorant },
    SpotToOther { source: SpotColorant, target: SpotColorant },
}

// ── P742 — Operadores de cor com semântica vanilla medida ───────────────────
//
// A semântica P476/P477 (lighten/darken/saturate via Oklch, negate em sRGB)
// foi **refutada por medição** (sonda P742 contra vanilla 0.15.0 969087ec +
// scratch com palette 0.7.6 — a crate que o vanilla usa). O vanilla opera
// **no espaço da própria cor** com a fórmula `increase` do palette e converte
// de volta ao espaço original nos operadores perceptuais. Ver
// `00_nucleo/prompts/entities/color.md` §"Operadores de cor (P476/P477,
// semântica corrigida em P742)".
//
// Helpers privados: duplicados de gradient.rs (circular dep impede import).

impl Color {
    /// **P744** — hex string em sRGB (com alpha quando < 1.0). Paridade
    /// vanilla `ProcessColor::to_hex` (`color.rs:1525-1531`).
    pub fn to_hex(self) -> String {
        let (r, g, b, a) = self.to_srgb();
        if a == 255 {
            format!("#{r:02x}{g:02x}{b:02x}")
        } else {
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        }
    }

    /// **P742** — componentes da cor para `components()` (paridade vanilla
    /// `ProcessColor::components`, `color.rs:1455-1522`): Cmyk ignora o flag
    /// alpha; as restantes omitem o alpha quando `include_alpha == false`.
    pub fn components(self, include_alpha: bool) -> Vec<ColorComponent> {
        use ColorComponent::*;
        let mut v = match self {
            Color::Srgb { r, g, b, a } => vec![Ratio(r), Ratio(g), Ratio(b), Ratio(a)],
            Color::Luma { l, a } => vec![Ratio(l), Ratio(a)],
            Color::LinearRgb { r, g, b, a } => {
                vec![Ratio(r), Ratio(g), Ratio(b), Ratio(a)]
            }
            Color::Oklab { l, a, b, alpha } => {
                vec![Ratio(l), Float(a), Float(b), Ratio(alpha)]
            }
            Color::Oklch { l, c, h, alpha } => {
                vec![Ratio(l), Float(c), Angle(h.rem_euclid(360.0)), Ratio(alpha)]
            }
            Color::Hsl { h, s, l, a } => {
                vec![Angle(h.rem_euclid(360.0)), Ratio(s), Ratio(l), Ratio(a)]
            }
            Color::Hsv { h, s, v, a } => {
                vec![Angle(h.rem_euclid(360.0)), Ratio(s), Ratio(v), Ratio(a)]
            }
            Color::Spot(spot) => return vec![Ratio(spot.tint.get() as f32)],
            Color::Cmyk { c, m, y, k } => {
                return vec![Ratio(c), Ratio(m), Ratio(y), Ratio(k)]
            }
        };
        if !include_alpha {
            v.pop();
        }
        v
    }
}

/// **P742** — componente heterogéneo de `Color::components` (o domínio não
/// conhece `Value`; a camada rules mapeia para `Value::Ratio`/`Float`/`Angle`).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorComponent {
    /// Componente em [0, 1] — imprime como percentagem (`Ratio` do vanilla).
    Ratio(f32),
    /// `a`/`b` de Oklab, chroma de Oklch (float sem unidade).
    Float(f32),
    /// Hue em graus (`Angle` do vanilla; `rem_euclid 360` — paridade
    /// `hue_angle`, `color.rs:2167`).
    Angle(f32),
}

/// **P1473/P1482** — `color.spot(name, fallback)` conserva o colorante
/// como tipo irmão de `Color`, com `.tint()` a produzir `Color::Spot` e
/// `Type::SpotColorant` próprio (`entities/value.md` §P1473). Construtor,
/// `repr` e cast do `fallback` (rejeita `spot`-de-`spot` com "expected
/// color, found spot colorant") vivem no owner produtivo
/// `compiler/stdlib/color.rs`; esta entidade não revalida.
#[derive(Debug, Clone, PartialEq)]
pub struct SpotColorant {
    /// `None` = sem nome (vanilla aceita `color.spot(none, ..)`).
    pub name: Option<ecow::EcoString>,
    /// Sempre um `Color` de processo — nunca `Spot`; validado pelo caller
    /// antes de chegar aqui.
    pub fallback: Color,
}

/// Valor cromático spot; a intensidade não altera a identidade do colorante.
#[derive(Debug, Clone, PartialEq)]
pub struct SpotColor {
    pub colorant: Arc<SpotColorant>,
    pub tint: Ratio,
}

impl SpotColor {
    pub fn to_process(&self) -> Color {
        self.colorant.fallback.clone().lighten(1.0 - self.tint.get() as f32)
    }
}

// `Hash` manual via `Debug` — `Color` não implementa `Hash` (`#[derive(Copy,
// Clone)]` acima, sem `Hash`; os campos `f32` não são `Hash` no Rust std).
// Mesmo padrão já em `entities/elements/list_item.rs` (`ListItemElem`/
// `EnumItemElem`).
impl std::hash::Hash for SpotColorant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v16_color_equality_keeps_spaces_bits_and_spot_identity() {
        let colors = [
            Color::srgb_f32(0.5, 0.5, 0.5, 1.0),
            Color::luma(0.5),
            Color::linear_rgb(0.5, 0.5, 0.5, 1.0),
            Color::oklab(0.5, 0.0, 0.0, 1.0),
            Color::oklch(0.5, 0.0, 0.0, 1.0),
            Color::hsl(0.0, 0.0, 0.5, 1.0),
            Color::hsv(0.0, 0.0, 0.5, 1.0),
            Color::cmyk(0.0, 0.0, 0.0, 0.5),
            Color::Spot(SpotColor {
                colorant: Arc::new(SpotColorant {
                    name: Some("ink".into()),
                    fallback: Color::rgb(0, 0, 0),
                }),
                tint: Ratio(0.5),
            }),
        ];
        for (i, left) in colors.iter().enumerate() {
            for (j, right) in colors.iter().enumerate() {
                assert_eq!(left == right, i == j, "spaces {i}/{j}");
            }
            assert_eq!(left, &left.clone());
        }
        assert_ne!(
            Color::srgb_f32(0.0, 0.0, 0.0, 1.0),
            Color::srgb_f32(-0.0, 0.0, 0.0, 1.0)
        );
        let nan = Color::srgb_f32(f32::NAN, 0.0, 0.0, 1.0);
        assert_eq!(nan, nan.clone());
    }

    // ── sRGB ──

    #[test]
    fn srgb_construcao_rgb_u8_paridade_observable() {
        let c = Color::rgb(255, 0, 0);
        if let Color::Srgb { r, g, b, a } = c {
            assert_eq!(r, 1.0);
            assert_eq!(g, 0.0);
            assert_eq!(b, 0.0);
            assert_eq!(a, 1.0);
        } else {
            panic!("esperado Srgb");
        }
    }

    #[test]
    fn srgb_to_srgb_roundtrip() {
        let c = Color::rgb(255, 0, 128);
        assert_eq!(c.to_srgb(), (255, 0, 128, 255));
    }

    #[test]
    fn srgb_partial_eq_exacto_via_bits() {
        let a = Color::rgb(255, 0, 0);
        let b = Color::rgb(255, 0, 0);
        assert_eq!(a, b);
        let c = Color::rgb(254, 0, 0);
        assert_ne!(a, c);
    }

    // ── Luma ──

    #[test]
    fn luma_construcao() {
        let c = Color::luma(0.5);
        if let Color::Luma { l, a } = c {
            assert_eq!(l, 0.5);
            assert_eq!(a, 1.0);
        } else {
            panic!("esperado Luma");
        }
    }

    #[test]
    fn luma_to_srgb_cinza() {
        let c = Color::luma(0.5);
        let (r, g, b, a) = c.to_srgb();
        assert_eq!(r, g);
        assert_eq!(g, b);
        assert_eq!(a, 255);
        assert!((r as i32 - 128).abs() <= 1); // ~50% gray
    }

    // ── LinearRgb ──

    #[test]
    fn linear_rgb_construcao_preserva_f32() {
        let c = Color::linear_rgb(0.5, 0.5, 0.5, 1.0);
        if let Color::LinearRgb { r, g, b, a } = c {
            assert_eq!(r, 0.5);
            assert_eq!(g, 0.5);
            assert_eq!(b, 0.5);
            assert_eq!(a, 1.0);
        } else {
            panic!("esperado LinearRgb");
        }
    }

    #[test]
    fn linear_rgb_to_srgb_gamma_inversa() {
        // LinearRgb 0.5 → sRGB ≠ 127 (gamma inversa).
        let c = Color::linear_rgb(0.5, 0.5, 0.5, 1.0);
        let (r, _, _, _) = c.to_srgb();
        // sRGB gamma para 0.5 linear ≈ 0.735 → ~188 (não 127).
        assert!(r > 180 && r < 200, "linear 0.5 → sRGB ~188 (gamma 2.4); obtido {}", r);
    }

    // ── Oklab ──

    #[test]
    fn oklab_construcao() {
        let c = Color::oklab(1.0, 0.0, 0.0, 1.0);
        if let Color::Oklab { l, a, b, alpha } = c {
            assert_eq!(l, 1.0);
            assert_eq!(a, 0.0);
            assert_eq!(b, 0.0);
            assert_eq!(alpha, 1.0);
        } else {
            panic!("esperado Oklab");
        }
    }

    #[test]
    fn oklab_to_srgb_branco_l1() {
        let c = Color::oklab(1.0, 0.0, 0.0, 1.0);
        let (r, g, b, a) = c.to_srgb();
        // L=1.0 → branco (≈255,255,255).
        assert!(r >= 250, "L=1 → r próximo 255; obtido {}", r);
        assert!(g >= 250);
        assert!(b >= 250);
        assert_eq!(a, 255);
    }

    #[test]
    fn oklab_to_srgb_preto_l0() {
        let c = Color::oklab(0.0, 0.0, 0.0, 1.0);
        let (r, g, b, a) = c.to_srgb();
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }

    // ── Oklch ──

    #[test]
    fn oklch_construcao() {
        let c = Color::oklch(0.5, 0.0, 0.0, 1.0);
        if let Color::Oklch { l, c: chroma, h, alpha } = c {
            assert_eq!(l, 0.5);
            assert_eq!(chroma, 0.0);
            assert_eq!(h, 0.0);
            assert_eq!(alpha, 1.0);
        } else {
            panic!("esperado Oklch");
        }
    }

    #[test]
    fn oklch_chroma_zero_eh_gris() {
        // c=0 → cor sem chroma → cinza.
        let c = Color::oklch(0.5, 0.0, 0.0, 1.0);
        let (r, g, b, _) = c.to_srgb();
        // Cinza: r ≈ g ≈ b.
        let avg = (r as i32 + g as i32 + b as i32) / 3;
        assert!((r as i32 - avg).abs() <= 3);
        assert!((g as i32 - avg).abs() <= 3);
        assert!((b as i32 - avg).abs() <= 3);
    }

    // ── Hsl ──

    #[test]
    fn hsl_construcao() {
        let c = Color::hsl(0.0, 0.0, 0.5, 1.0);
        if let Color::Hsl { h, s, l, a } = c {
            assert_eq!(h, 0.0);
            assert_eq!(s, 0.0);
            assert_eq!(l, 0.5);
            assert_eq!(a, 1.0);
        } else {
            panic!("esperado Hsl");
        }
    }

    #[test]
    fn hsl_to_srgb_cinza_s0() {
        // HSL s=0 → cinza.
        let c = Color::hsl(120.0, 0.0, 0.5, 1.0);
        let (r, g, b, _) = c.to_srgb();
        assert_eq!(r, g);
        assert_eq!(g, b);
        // l=0.5 → ~127.
        assert!((r as i32 - 128).abs() <= 1);
    }

    #[test]
    fn hsl_to_srgb_vermelho_puro() {
        // HSL(0, 100%, 50%) → vermelho puro (255, 0, 0).
        let c = Color::hsl(0.0, 1.0, 0.5, 1.0);
        let (r, g, b, _) = c.to_srgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    // ── Hsv ──

    #[test]
    fn hsv_construcao() {
        let c = Color::hsv(120.0, 0.5, 0.8, 1.0);
        if let Color::Hsv { h, s, v, a } = c {
            assert_eq!(h, 120.0);
            assert_eq!(s, 0.5);
            assert_eq!(v, 0.8);
            assert_eq!(a, 1.0);
        } else {
            panic!("esperado Hsv");
        }
    }

    #[test]
    fn hsv_to_srgb_branco_s0_v1() {
        let c = Color::hsv(0.0, 0.0, 1.0, 1.0);
        let (r, g, b, _) = c.to_srgb();
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
    }

    // ── Cmyk ──

    #[test]
    fn cmyk_construcao() {
        let c = Color::cmyk(0.0, 0.5, 1.0, 0.0);
        if let Color::Cmyk { c, m, y, k } = c {
            assert_eq!(c, 0.0);
            assert_eq!(m, 0.5);
            assert_eq!(y, 1.0);
            assert_eq!(k, 0.0);
        } else {
            panic!("esperado Cmyk");
        }
    }

    #[test]
    fn cmyk_zero_eh_branco() {
        // CMYK(0,0,0,0) → branco (255, 255, 255).
        let c = Color::cmyk(0.0, 0.0, 0.0, 0.0);
        let (r, g, b, _) = c.to_srgb();
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
    }

    #[test]
    fn cmyk_k1_eh_preto() {
        // CMYK k=1 → preto (0, 0, 0).
        let c = Color::cmyk(0.0, 0.0, 0.0, 1.0);
        let (r, g, b, _) = c.to_srgb();
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    // ── Cross-variant ──

    #[test]
    fn srgb_e_luma_iguais_nunca_iguais() {
        // Variants diferentes nunca eq (paridade vanilla).
        let a = Color::Srgb { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
        let b = Color::Luma { l: 0.5, a: 1.0 };
        assert_ne!(a, b);
    }

    #[test]
    fn color_clone_disponivel() {
        let c = Color::rgb(100, 200, 50);
        let c2 = c.clone();
        let c3 = c.clone();
        assert_eq!(c, c2);
        assert_eq!(c, c3);
    }

    // ── P476 — lighten / darken / mix / negate ──
    //
    // **P742 — semântica corrigida por medição vanilla** (sonda: vanilla
    // 0.15.0 969087ec + palette 0.7.6; o `red` vanilla é
    // `rgb(1.0, 0.254902, 0.211765)` = `#ff4136`, não `#ff0000`):
    // negate opera em Oklab por default e volta ao espaço original — o
    // critério antigo "negate(vermelho) = ciano" estava errado.

    const VANILLA_RED: Color = Color::Srgb { r: 1.0, g: 0.254902, b: 0.211765, a: 1.0 };

    #[test]
    fn p1076_mix_srgb_ties_to_even_paridade_vanilla() {
        // Caso do Achado #11 do P1031:
        let c = Color::rgb(255, 65, 54);
        let d = Color::rgb(0, 116, 217);

        // 50% mix em sRGB: 90.5 -> 90 (0x5a), não 91 (0x5b)
        let m50 = c.clone().mix(d.clone(), 0.5, Some(ColorSpace::Srgb));
        assert_eq!(m50.to_hex(), "#805a88");

        // 25% d:
        let m25 = c.clone().mix(d.clone(), 0.25, Some(ColorSpace::Srgb));
        assert_eq!(m25.to_hex(), "#bf4e5f");

        // 75% d:
        let m75 = c.mix(d, 0.75, Some(ColorSpace::Srgb));
        assert_eq!(m75.to_hex(), "#4067b0");

        // Branco e preto 50%:
        let white = Color::rgb(255, 255, 255);
        let black = Color::rgb(0, 0, 0);
        let grey = white.mix(black, 0.5, Some(ColorSpace::Srgb));
        assert_eq!(grey.to_hex(), "#808080");
    }

    #[test]
    fn p1076_ties_to_even_halfway_behavior() {
        // Testa especificamente que .5 arredonda para o número par mais próximo:
        // 90.5 / 255.0 -> 90 (par)
        assert_eq!(Color::f32_to_u8_ties_even(90.5 / 255.0), 90);
        // 91.5 / 255.0 -> 92 (par)
        assert_eq!(Color::f32_to_u8_ties_even(91.5 / 255.0), 92);
        // 127.5 / 255.0 -> 128 (par)
        assert_eq!(Color::f32_to_u8_ties_even(127.5 / 255.0), 128);
    }

    #[test]
    fn p742_negate_vermelho_puro_via_oklab() {
        // Medido scratch palette 0.7.6: negate(srgb(1,0,0)) → #005688.
        let red = Color::srgb_f32(1.0, 0.0, 0.0, 1.0);
        assert_eq!(red.negate(None).to_srgb(), (0, 86, 136, 255));
    }

    #[test]
    fn p742_lighten_vanilla_red_medido() {
        // Medido vanilla: red.lighten(20%) → rgb("#ff675e").
        assert_eq!(VANILLA_RED.lighten(0.2).to_srgb(), (255, 103, 94, 255));
    }

    #[test]
    fn p742_darken_vanilla_red_medido() {
        // Medido vanilla: red.darken(20%) → rgb("#cc342b").
        assert_eq!(VANILLA_RED.darken(0.2).to_srgb(), (204, 52, 43, 255));
    }

    #[test]
    fn p742_negate_vanilla_red_medido() {
        // Medido vanilla: red.negate() → rgb("#004b74").
        assert_eq!(VANILLA_RED.negate(None).to_srgb(), (0, 75, 116, 255));
    }

    #[test]
    fn p742_rotate_90_vanilla_red_medido() {
        // Medido vanilla: red.rotate(90deg) → rgb("#87a100").
        assert_eq!(VANILLA_RED.rotate(90.0, None).unwrap().to_srgb(), (135, 161, 0, 255));
    }

    #[test]
    fn p742_saturate_vanilla_red_medido() {
        // Medido vanilla: red.saturate(20%) → rgb("#ff372b").
        assert_eq!(VANILLA_RED.saturate(0.2).unwrap().to_srgb(), (255, 55, 43, 255));
    }

    #[test]
    fn p742_desaturate_vanilla_red_medido() {
        // Medido vanilla: red.desaturate(20%) → rgb("#ff675e").
        assert_eq!(VANILLA_RED.desaturate(0.2).unwrap().to_srgb(), (255, 103, 94, 255));
    }

    #[test]
    fn p742_saturate_desaturate_luma_none() {
        // Vanilla: "cannot saturate/desaturate grayscale color" — o domínio
        // devolve None; o chamador em rules emite o erro verbatim.
        let g = Color::luma(0.5);
        assert!(g.clone().saturate(0.2).is_none());
        assert!(g.desaturate(0.2).is_none());
    }

    #[test]
    fn p742_lighten_zero_identidade_mesma_variante() {
        // A cor resultante mantém a variante original (vanilla devolve o
        // espaço original — medido: repr é rgb(...) e não oklch(...)).
        // lighten/darken são bitwise-exactos a factor 0; saturate/desaturate
        // fazem roundtrip HSV → comparação por bytes sRGB.
        assert_eq!(VANILLA_RED.lighten(0.0), VANILLA_RED);
        assert_eq!(VANILLA_RED.darken(0.0), VANILLA_RED);
        assert_eq!(VANILLA_RED.saturate(0.0).unwrap().to_srgb(), VANILLA_RED.to_srgb());
        assert_eq!(VANILLA_RED.desaturate(0.0).unwrap().to_srgb(), VANILLA_RED.to_srgb());
        assert!(matches!(VANILLA_RED.saturate(0.0), Some(Color::Srgb { .. })));
    }

    #[test]
    fn p742_cmyk_lighten_darken_formulas_vanilla() {
        // Cmyk é tipo próprio do vanilla (color.rs:2235-2243):
        // lighten(u) = u - u*f; darken(u) = u + (1-u)*f.
        let c = Color::cmyk(0.5, 0.5, 0.5, 0.5);
        assert_eq!(c.clone().lighten(0.2), Color::cmyk(0.4, 0.4, 0.4, 0.4));
        assert_eq!(c.darken(0.2), Color::cmyk(0.6, 0.6, 0.6, 0.6));
    }

    #[test]
    fn p742_luma_lighten_increase() {
        // Luma usa a fórmula palette: l + (1-l)*f.
        assert_eq!(Color::luma(0.5).lighten(0.2), Color::luma(0.6));
    }

    #[test]
    fn p742_space_mapeamento_variantes() {
        assert_eq!(VANILLA_RED.space(), ColorSpace::Srgb);
        assert_eq!(Color::luma(0.5).space(), ColorSpace::Luma);
        assert_eq!(Color::linear_rgb(0.1, 0.2, 0.3, 1.0).space(), ColorSpace::LinearRgb);
        assert_eq!(Color::oklab(0.5, 0.1, 0.1, 1.0).space(), ColorSpace::Oklab);
        assert_eq!(Color::oklch(0.5, 0.1, 30.0, 1.0).space(), ColorSpace::Oklch);
        assert_eq!(Color::hsl(0.0, 1.0, 0.5, 1.0).space(), ColorSpace::Hsl);
        assert_eq!(Color::hsv(0.0, 1.0, 1.0, 1.0).space(), ColorSpace::Hsv);
        assert_eq!(Color::cmyk(0.0, 0.0, 0.0, 0.0).space(), ColorSpace::Cmyk);
    }

    #[test]
    fn p742_to_space_roundtrip_srgb_hsv_bytes() {
        let roundtrip = VANILLA_RED
            .to_space(ColorSpace::Hsv)
            .unwrap()
            .to_space(ColorSpace::Srgb)
            .unwrap();
        assert_eq!(roundtrip.to_srgb(), VANILLA_RED.to_srgb());
    }

    #[test]
    fn p742_to_space_identidade_mesmo_espaco() {
        assert_eq!(VANILLA_RED.to_space(ColorSpace::Srgb).unwrap(), VANILLA_RED);
    }

    #[test]
    fn p1252_to_space_luma_preserva_alpha_known_upstream_bug() {
        let source = Color::srgb_f32(1.0, 0.254902, 0.211765, 0.4);
        let Ok(Color::Luma { l, a }) = source.to_space(ColorSpace::Luma) else {
            panic!("to_space(Luma) deve produzir Luma");
        };
        assert_eq!(a.to_bits(), 0.4_f32.to_bits());
        assert!(
            (l - 0.5402).abs() < 0.00005,
            "P1239 fecha luminância sem reverter alpha: {l}"
        );
    }

    #[test]
    fn p1239_luma_red_usa_matriz_srgb_palette() {
        let Ok(Color::Luma { l, a }) = VANILLA_RED.to_space(ColorSpace::Luma) else {
            panic!("red.to_space(Luma) deve produzir Luma");
        };
        assert!((l - 0.5402).abs() < 0.00005, "witness RED P1239: {l}");
        assert_eq!(a, 1.0);
    }

    #[test]
    fn p1253_oklab_red_usa_constantes_completas_palette() {
        let (l, a, b, alpha) = to_oklab_p476(VANILLA_RED);
        assert!((l - 0.6595).abs() < 0.00005, "L público arredondado: {l}");
        assert_eq!(a.to_bits(), 0.199_735_3_f32.to_bits());
        assert_eq!(b.to_bits(), 0.108_188_12_f32.to_bits());
        assert_eq!(alpha.to_bits(), 1.0_f32.to_bits());
    }

    #[test]
    fn p1271_luma_white_para_oklab_segue_d65_palette() {
        let Ok(Color::Oklab { l, a, b, alpha }) =
            Color::luma(1.0).to_space(ColorSpace::Oklab)
        else {
            panic!("esperado Oklab")
        };
        assert_eq!(l.to_bits(), 0x3f80_0000);
        assert_eq!(a.to_bits(), 0x3740_0000);
        assert_eq!(b.to_bits(), 0x381b_8000);
        assert_eq!(alpha.to_bits(), 0x3f80_0000);
    }

    #[test]
    fn p742_components_srgb_com_e_sem_alpha() {
        let com = VANILLA_RED.components(true);
        assert_eq!(com.len(), 4);
        assert!(matches!(com[0], ColorComponent::Ratio(v) if (v - 1.0).abs() < 1e-6));
        assert!(
            matches!(com[1], ColorComponent::Ratio(v) if (v - 0.254902).abs() < 1e-5)
        );
        assert!(
            matches!(com[2], ColorComponent::Ratio(v) if (v - 0.211765).abs() < 1e-5)
        );
        assert!(matches!(com[3], ColorComponent::Ratio(v) if (v - 1.0).abs() < 1e-6));
        let sem = VANILLA_RED.components(false);
        assert_eq!(sem.len(), 3);
    }

    #[test]
    fn p742_components_oklab_e_oklch_tipos() {
        let o = Color::oklab(0.6, 0.1, -0.05, 1.0).components(true);
        assert!(matches!(o[0], ColorComponent::Ratio(_)));
        assert!(matches!(o[1], ColorComponent::Float(v) if (v - 0.1).abs() < 1e-6));
        assert!(matches!(o[2], ColorComponent::Float(v) if (v + 0.05).abs() < 1e-6));
        assert!(matches!(o[3], ColorComponent::Ratio(_)));
        let c = Color::oklch(0.6, 0.2, 30.0, 1.0).components(true);
        assert!(matches!(c[1], ColorComponent::Float(v) if (v - 0.2).abs() < 1e-6));
        assert!(matches!(c[2], ColorComponent::Angle(v) if (v - 30.0).abs() < 1e-6));
    }

    #[test]
    fn p742_components_cmyk_ignora_alpha() {
        let c = Color::cmyk(0.1, 0.2, 0.3, 0.4).components(false);
        assert_eq!(c.len(), 4, "Cmyk ignora o flag alpha (paridade vanilla)");
    }

    #[test]
    fn p476_negate_preserva_alpha() {
        let c = Color::srgb_f32(0.5, 0.5, 0.5, 0.3);
        let n = c.negate(None);
        let (_, _, _, a) = n.to_rgba_f32();
        assert!((a - 0.3).abs() < 1e-5, "alpha deve ser preservado; obtido {}", a);
    }

    #[test]
    fn p476_lighten_zero_nao_altera_luminancia() {
        let red = Color::rgb(255, 0, 0);
        let lightened = red.clone().lighten(0.0);
        let (l0, c0, h0, _) = to_oklch_p476(red);
        let (l1, c1, h1, _) = to_oklch_p476(lightened);
        assert!((l1 - l0).abs() < 1e-4, "l deve ser igual; delta={}", (l1 - l0).abs());
        assert!((c1 - c0).abs() < 1e-4);
        let _ = h0;
        let _ = h1;
    }

    #[test]
    fn p476_lighten_um_da_branco_oklch() {
        let red = Color::rgb(255, 0, 0);
        let lightened = red.lighten(1.0);
        let (l, _, _, _) = to_oklch_p476(lightened);
        assert!((l - 1.0).abs() < 1e-4, "lighten(1.0) → l clamped a 1.0; obtido {}", l);
    }

    #[test]
    fn p476_darken_zero_nao_altera_luminancia() {
        let blue = Color::rgb(0, 0, 255);
        let darkened = blue.clone().darken(0.0);
        let (l0, _, _, _) = to_oklch_p476(blue);
        let (l1, _, _, _) = to_oklch_p476(darkened);
        assert!((l1 - l0).abs() < 1e-4);
    }

    #[test]
    fn p476_darken_um_da_preto_oklch() {
        let blue = Color::rgb(0, 0, 255);
        let darkened = blue.darken(1.0);
        let (l, _, _, _) = to_oklch_p476(darkened);
        assert!((l - 0.0).abs() < 1e-4, "darken(1.0) → l clamped a 0.0; obtido {}", l);
    }

    #[test]
    fn p476_mix_peso_zero_igual_a_self() {
        let red = Color::srgb_f32(1.0, 0.0, 0.0, 1.0);
        let blue = Color::srgb_f32(0.0, 0.0, 1.0, 1.0);
        let (l0, a0, b0, _) = to_oklab_p476(red.clone());
        let mixed = red.mix(blue, 0.0, None);
        let (l1, a1, b1, _) = to_oklab_p476(mixed);
        assert!((l1 - l0).abs() < 1e-4);
        assert!((a1 - a0).abs() < 1e-4);
        assert!((b1 - b0).abs() < 1e-4);
    }

    #[test]
    fn p476_mix_peso_um_igual_a_other() {
        let red = Color::srgb_f32(1.0, 0.0, 0.0, 1.0);
        let blue = Color::srgb_f32(0.0, 0.0, 1.0, 1.0);
        let (l0, a0, b0, _) = to_oklab_p476(blue.clone());
        let mixed = red.mix(blue, 1.0, None);
        let (l1, a1, b1, _) = to_oklab_p476(mixed);
        assert!((l1 - l0).abs() < 1e-4);
        assert!((a1 - a0).abs() < 1e-4);
        assert!((b1 - b0).abs() < 1e-4);
    }

    #[test]
    fn p476_mix_meio_esta_entre_red_e_blue() {
        let red = Color::srgb_f32(1.0, 0.0, 0.0, 1.0);
        let blue = Color::srgb_f32(0.0, 0.0, 1.0, 1.0);
        let (l0, _, _, _) = to_oklab_p476(red.clone());
        let (l1, _, _, _) = to_oklab_p476(blue.clone());
        let mixed = red.mix(blue, 0.5, None);
        let (lm, _, _, _) = to_oklab_p476(mixed);
        let expected_l = (l0 + l1) / 2.0;
        assert!(
            (lm - expected_l).abs() < 1e-4,
            "l médio esperado {}; obtido {}",
            expected_l,
            lm
        );
    }

    // ── P477 — saturate / desaturate ──

    // ── P477 — saturate / desaturate (semântica corrigida P742: via HSV) ──

    fn hsv_of(c: Color) -> (f32, f32, f32) {
        match c.to_space(ColorSpace::Hsv).unwrap() {
            Color::Hsv { h, s, v, .. } => (h, s, v),
            _ => unreachable!("to_space(Hsv) devolve sempre Hsv"),
        }
    }

    #[test]
    fn p477_saturate_zero_nao_altera_chroma() {
        let red = Color::rgb(255, 0, 0);
        let (_, c0, _, _) = to_oklch_p476(red.clone());
        let (_, c1, _, _) = to_oklch_p476(red.saturate(0.0).unwrap());
        assert!(
            (c1 - c0).abs() < 1e-5,
            "saturate(0) não deve alterar chroma; delta={}",
            (c1 - c0).abs()
        );
    }

    #[test]
    fn p742_saturate_aumenta_saturacao_hsv() {
        // Semântica vanilla (palette): increase sobre `s` em HSV. Vermelho
        // puro (s=1) não muda; cor com s=0.5 passa a s=0.55 → g e b descem.
        let c = Color::srgb_f32(1.0, 0.5, 0.5, 1.0);
        let (h0, s0, v0) = hsv_of(c.clone());
        assert!((s0 - 0.5).abs() < 1e-5);
        let sat = c.saturate(0.1).unwrap();
        let (h1, s1, v1) = hsv_of(sat.clone());
        assert!((s1 - 0.55).abs() < 1e-4, "s esperado 0.55; obtido {s1}");
        assert!((h1 - h0).abs() < 1e-3, "hue preservado");
        assert!((v1 - v0).abs() < 1e-5, "value preservado");
        assert_eq!(sat.to_srgb(), (255, 115, 115, 255));
    }

    #[test]
    fn p477_desaturate_zero_nao_altera_chroma() {
        let blue = Color::rgb(0, 0, 255);
        let (_, c0, _, _) = to_oklch_p476(blue.clone());
        let (_, c1, _, _) = to_oklch_p476(blue.desaturate(0.0).unwrap());
        assert!((c1 - c0).abs() < 1e-5);
    }

    #[test]
    fn p477_desaturate_grande_clampado_a_zero() {
        // Vanilla: desaturate(100%) zera `s` em HSV preservando v → branco
        // (não cinzento — corrigido P742). Chroma Oklch ≈ 0 mantém-se.
        let red = Color::rgb(255, 0, 0);
        let desat = red.desaturate(1.0).unwrap();
        let (_, c, _, _) = to_oklch_p476(desat.clone());
        assert!((c - 0.0).abs() < 1e-5, "desaturate(1.0) → c=0; obtido {}", c);
        assert_eq!(desat.to_srgb(), (255, 255, 255, 255), "v=1 preservado → branco");
    }

    #[test]
    fn p742_saturate_preserva_hue_e_value() {
        let (h0, s0, v0) = hsv_of(VANILLA_RED);
        let sat = VANILLA_RED.saturate(0.2).unwrap();
        let (h1, s1, v1) = hsv_of(sat.clone());
        assert!(s1 > s0, "s deve aumentar: s0={s0}, s1={s1}");
        assert!((h1 - h0).abs() < 1e-3, "hue preservado");
        assert!((v1 - v0).abs() < 1e-5, "value preservado");
        // Valor medido no vanilla: red.saturate(20%) → rgb("#ff372b").
        assert_eq!(sat.to_srgb(), (255, 55, 43, 255));
    }

    #[test]
    fn p742_desaturate_preserva_hue_e_value() {
        let blue = Color::rgb(0, 0, 255);
        let (h0, s0, v0) = hsv_of(blue.clone());
        let desat = blue.desaturate(0.05).unwrap();
        let (h1, s1, v1) = hsv_of(desat.clone());
        assert!(s1 < s0, "s deve diminuir: s0={s0}, s1={s1}");
        assert!((h1 - h0).abs() < 1e-3, "hue preservado");
        assert!((v1 - v0).abs() < 1e-5, "value preservado");
        let (_, _, _, a) = desat.to_rgba_f32();
        assert!((a - 1.0).abs() < 1e-5, "alpha preservado");
    }

    // ── P744 ──

    #[test]
    fn p744_negate_com_space_rgb() {
        // Medido vanilla: red.negate(space: rgb) → rgb("#00bec9").
        assert_eq!(VANILLA_RED.negate(Some(ColorSpace::Srgb)).to_hex(), "#00bec9");
    }

    #[test]
    fn p744_negate_default_igual_oklab() {
        assert_eq!(
            VANILLA_RED.negate(None).to_hex(),
            VANILLA_RED.negate(Some(ColorSpace::Oklab)).to_hex()
        );
    }

    #[test]
    fn p744_rotate_default_igual_oklch() {
        assert_eq!(
            VANILLA_RED.rotate(90.0, None).unwrap().to_hex(),
            VANILLA_RED.rotate(90.0, Some(ColorSpace::Oklch)).unwrap().to_hex(),
        );
    }

    #[test]
    fn p744_rotate_rgb_devolve_none() {
        assert!(VANILLA_RED.rotate(90.0, Some(ColorSpace::Srgb)).is_none());
    }

    #[test]
    fn p744_rotate_hsl_e_hsv_suportados() {
        // Vermelho puro em Hsl/Hsv com rotação 180° → cyan (#00ffff).
        let red = Color::rgb(255, 0, 0);
        assert_eq!(
            red.clone().rotate(180.0, Some(ColorSpace::Hsl)).unwrap().to_hex(),
            "#00ffff"
        );
        assert_eq!(red.rotate(180.0, Some(ColorSpace::Hsv)).unwrap().to_hex(), "#00ffff");
    }

    #[test]
    fn p744_mix_default_igual_oklab() {
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        assert_eq!(
            red.clone().mix(blue.clone(), 0.5, None).to_hex(),
            red.mix(blue, 0.5, Some(ColorSpace::Oklab)).to_hex(),
        );
    }

    #[test]
    fn p744_mix_no_espaco_indicado() {
        // O resultado fica no espaço indicado: mix em Oklab → Oklab;
        // mix em Srgb → Srgb (repr hex).
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        let mixed_oklab = red.clone().mix(blue.clone(), 0.5, Some(ColorSpace::Oklab));
        assert!(matches!(mixed_oklab, Color::Oklab { .. }));
        let mixed_srgb = red.mix(blue, 0.5, Some(ColorSpace::Srgb));
        assert!(matches!(mixed_srgb, Color::Srgb { .. }));
    }

    #[test]
    fn p744_mix_hue_short_path() {
        // Duas cores com hue a 350° e 10° — interpolação deve ir pelo
        // caminho curto (média = 0°, não 180°).
        let c1 = Color::hsl(350.0, 1.0, 0.5, 1.0);
        let c2 = Color::hsl(10.0, 1.0, 0.5, 1.0);
        let mixed = c1.mix(c2, 0.5, Some(ColorSpace::Hsl));
        let Color::Hsl { h, .. } = mixed else { panic!("esperado Hsl") };
        // Média pelo caminho curto: (350 + 370) / 2 = 360 → 0°.
        assert!(
            (h.rem_euclid(360.0)).abs() < 1.0
                || (h.rem_euclid(360.0) - 360.0).abs() < 1.0,
            "hue médio pelo caminho curto; obtido {h}"
        );
    }

    #[test]
    fn p744_to_hex_opaque_e_transparente() {
        assert_eq!(Color::rgb(255, 65, 54).to_hex(), "#ff4136");
        assert_eq!(Color::rgba(255, 65, 54, 128).to_hex(), "#ff413680");
    }

    #[test]
    fn p744_transparentize_opacify() {
        let c = Color::rgba(255, 65, 54, 128);
        assert_eq!(c.clone().transparentize(0.5).unwrap().to_hex(), "#ff413640");
        assert_eq!(c.opacify(0.5).unwrap().to_hex(), "#ff4136c0");
    }

    #[test]
    fn p744_transparentize_opacify_cmyk_none() {
        let c = Color::cmyk(0.1, 0.2, 0.3, 0.4);
        assert!(c.clone().transparentize(0.5).is_none());
        assert!(c.opacify(0.5).is_none());
    }
}
