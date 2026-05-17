//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/color.md
//! @prompt-hash 20a91590
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
#[derive(Debug, Copy, Clone)]
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
}

impl PartialEq for Color {
    /// Comparação exacta via `f32::to_bits` (sem tolerância;
    /// paridade ADR-0028 regra herdada "sem tolerância em produção").
    /// Cores em variantes diferentes nunca são iguais (paridade
    /// vanilla — `Srgb(r=1)` ≠ `Luma(l=1)`).
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Srgb { r: r1, g: g1, b: b1, a: a1 },
             Self::Srgb { r: r2, g: g2, b: b2, a: a2 }) =>
                r1.to_bits() == r2.to_bits()
                && g1.to_bits() == g2.to_bits()
                && b1.to_bits() == b2.to_bits()
                && a1.to_bits() == a2.to_bits(),
            (Self::Luma { l: l1, a: a1 },
             Self::Luma { l: l2, a: a2 }) =>
                l1.to_bits() == l2.to_bits()
                && a1.to_bits() == a2.to_bits(),
            (Self::LinearRgb { r: r1, g: g1, b: b1, a: a1 },
             Self::LinearRgb { r: r2, g: g2, b: b2, a: a2 }) =>
                r1.to_bits() == r2.to_bits()
                && g1.to_bits() == g2.to_bits()
                && b1.to_bits() == b2.to_bits()
                && a1.to_bits() == a2.to_bits(),
            (Self::Oklab { l: l1, a: a1, b: b1, alpha: alpha1 },
             Self::Oklab { l: l2, a: a2, b: b2, alpha: alpha2 }) =>
                l1.to_bits() == l2.to_bits()
                && a1.to_bits() == a2.to_bits()
                && b1.to_bits() == b2.to_bits()
                && alpha1.to_bits() == alpha2.to_bits(),
            (Self::Oklch { l: l1, c: c1, h: h1, alpha: alpha1 },
             Self::Oklch { l: l2, c: c2, h: h2, alpha: alpha2 }) =>
                l1.to_bits() == l2.to_bits()
                && c1.to_bits() == c2.to_bits()
                && h1.to_bits() == h2.to_bits()
                && alpha1.to_bits() == alpha2.to_bits(),
            (Self::Hsl { h: h1, s: s1, l: l1, a: a1 },
             Self::Hsl { h: h2, s: s2, l: l2, a: a2 }) =>
                h1.to_bits() == h2.to_bits()
                && s1.to_bits() == s2.to_bits()
                && l1.to_bits() == l2.to_bits()
                && a1.to_bits() == a2.to_bits(),
            (Self::Hsv { h: h1, s: s1, v: v1, a: a1 },
             Self::Hsv { h: h2, s: s2, v: v2, a: a2 }) =>
                h1.to_bits() == h2.to_bits()
                && s1.to_bits() == s2.to_bits()
                && v1.to_bits() == v2.to_bits()
                && a1.to_bits() == a2.to_bits(),
            (Self::Cmyk { c: c1, m: m1, y: y1, k: k1 },
             Self::Cmyk { c: c2, m: m2, y: y2, k: k2 }) =>
                c1.to_bits() == c2.to_bits()
                && m1.to_bits() == m2.to_bits()
                && y1.to_bits() == y2.to_bits()
                && k1.to_bits() == k2.to_bits(),
            _ => false,
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

    // ── Conversões ───────────────────────────────────────────

    /// Converte para sRGB byte `(r, g, b, a)` em [0, 255].
    /// Consumer principal: PDF exporter (4 caminhos
    /// `to_rgba_f32` cumulativos).
    pub fn to_srgb(&self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.to_rgba_f32();
        (
            (r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        )
    }

    /// Converte para sRGB normalizado `(r, g, b, a)` em [0.0, 1.0].
    /// Preservado para compatibilidade hot path PDF exporter
    /// cristalino existente.
    pub fn to_rgba_f32(&self) -> (f32, f32, f32, f32) {
        match *self {
            Self::Srgb { r, g, b, a } => (r, g, b, a),
            Self::Luma { l, a } => (l, l, l, a),
            Self::LinearRgb { r, g, b, a } => {
                // Gamma 2.2 inversa (linear → sRGB).
                (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b), a)
            }
            Self::Oklab { l, a, b, alpha } => {
                let (lin_r, lin_g, lin_b) = oklab_to_linear_rgb(l, a, b);
                (linear_to_srgb(lin_r), linear_to_srgb(lin_g), linear_to_srgb(lin_b), alpha)
            }
            Self::Oklch { l, c, h, alpha } => {
                // Polar → cartesiano (a, b).
                let h_rad = h.to_radians();
                let a = c * h_rad.cos();
                let b = c * h_rad.sin();
                let (lin_r, lin_g, lin_b) = oklab_to_linear_rgb(l, a, b);
                (linear_to_srgb(lin_r), linear_to_srgb(lin_g), linear_to_srgb(lin_b), alpha)
            }
            Self::Hsl { h, s, l, a } => {
                let (r, g, b) = hsl_to_rgb(h, s, l);
                (r, g, b, a)
            }
            Self::Hsv { h, s, v, a } => {
                let (r, g, b) = hsv_to_rgb(h, s, v);
                (r, g, b, a)
            }
            Self::Cmyk { c, m, y, k } => {
                // CMY → RGB: r=(1-c)(1-k), g=(1-m)(1-k), b=(1-y)(1-k).
                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);
                (r, g, b, 1.0)
            }
        }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    Oklab,
    Oklch,
    Srgb,
    Luma,
    LinearRgb,
    Hsl,
    Hsv,
    Cmyk,
}

// ── Helpers conversões ──────────────────────────────────────

/// Linear RGB → sRGB (gamma encoding).
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Oklab → Linear RGB (matriz LMS).
/// Algoritmo: Björn Ottosson <https://bottosson.github.io/posts/oklab/>.
fn oklab_to_linear_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let l_3 = l_ * l_ * l_;
    let m_3 = m_ * m_ * m_;
    let s_3 = s_ * s_ * s_;

    let r = 4.0767416621 * l_3 - 3.3077115913 * m_3 + 0.2309699292 * s_3;
    let g = -1.2684380046 * l_3 + 2.6097574011 * m_3 - 0.3413193965 * s_3;
    let b = -0.0041960863 * l_3 - 0.7034186147 * m_3 + 1.7076147010 * s_3;
    (r, g, b)
}

/// HSL → RGB (h em graus; s, l normalizados).
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

/// HSV → RGB (h em graus; s, v normalizados).
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(r > 180 && r < 200,
            "linear 0.5 → sRGB ~188 (gamma 2.4); obtido {}", r);
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
    fn color_copy_clone_disponivel() {
        let c = Color::rgb(100, 200, 50);
        let c2 = c;  // Copy
        let c3 = c.clone();
        assert_eq!(c, c2);
        assert_eq!(c, c3);
    }
}
