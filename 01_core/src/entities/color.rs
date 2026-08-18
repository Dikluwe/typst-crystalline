//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/color.md
//! @prompt-hash 18570291
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
            _other => false, // neutro: variantes de cores em espaços distintos são estritamente desiguais
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
/// Converte float em [0.0, 1.0] para u8 em [0, 255] com arredondamento
/// "round ties to even" (paridade palette crate / Vanilla Typst, Hacker's Delight).
#[inline]
pub fn f32_to_u8_ties_even(val: f32) -> u8 {
    let scaled = (val.clamp(0.0, 1.0) * 255.0).min(255.0);
    const C23: u32 = 0x4b00_0000;
    let f = scaled + f32::from_bits(C23);
    (f.to_bits().saturating_sub(C23)) as u8
}

    pub fn to_srgb(&self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.to_rgba_f32();
        (
            Self::f32_to_u8_ties_even(r),
            Self::f32_to_u8_ties_even(g),
            Self::f32_to_u8_ties_even(b),
            Self::f32_to_u8_ties_even(a),
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
                (
                    linear_to_srgb(lin_r),
                    linear_to_srgb(lin_g),
                    linear_to_srgb(lin_b),
                    alpha,
                )
            }
            Self::Oklch { l, c, h, alpha } => {
                // Polar → cartesiano (a, b).
                let h_rad = h.to_radians();
                let a = c * h_rad.cos();
                let b = c * h_rad.sin();
                let (lin_r, lin_g, lin_b) = oklab_to_linear_rgb(l, a, b);
                (
                    linear_to_srgb(lin_r),
                    linear_to_srgb(lin_g),
                    linear_to_srgb(lin_b),
                    alpha,
                )
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

/// sRGB → linear RGB (inverso de gamma 2.2). Duplicado de `gradient::srgb_to_linear`.
fn srgb_to_linear_p476(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// linear sRGB → Oklab. Duplicado de `gradient::linear_rgb_to_oklab` (P270).
fn linear_rgb_to_oklab_p476(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_56 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_71 * b;
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();
    (
        0.210_454_26 * l_ + 0.793_617_8 * m_ - 0.004_072_047 * s_,
        1.977_998_5 * l_ - 2.428_592_2 * m_ + 0.450_593_7 * s_,
        0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77 * s_,
    )
}

/// Qualquer Color → Oklab (l, a, b, alpha). Base de `mix` e `to_oklch_p476`.
fn to_oklab_p476(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
        _ => {
            let (r, g, b_c, alpha) = c.to_rgba_f32();
            let (lab_l, lab_a, lab_b) = linear_rgb_to_oklab_p476(
                srgb_to_linear_p476(r),
                srgb_to_linear_p476(g),
                srgb_to_linear_p476(b_c),
            );
            (lab_l, lab_a, lab_b, alpha)
        }
    }
}

/// Qualquer Color → Oklch (l, c, h, alpha). Base de `rotate` e testes.
fn to_oklch_p476(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklch { l, c, h, alpha } => (l, c, h, alpha),
        _ => {
            let (l, a, b, alpha) = to_oklab_p476(c);
            let chroma = (a * a + b * b).sqrt();
            let h = b.atan2(a).to_degrees().rem_euclid(360.0);
            (l, chroma, h, alpha)
        }
    }
}

/// **P742** — fórmula `increase` do palette 0.7.6
/// (`macros/lighten_saturate.rs`): para `f >= 0`, `c + max(0, 1-c) * f`;
/// para `f < 0`, `c + max(0, c) * f`. Clamp [0, 1].
fn increase_p742(c: f32, f: f32) -> f32 {
    let difference = if f >= 0.0 { (1.0 - c).max(0.0) } else { c.max(0.0) };
    (c + difference * f).clamp(0.0, 1.0)
}

/// **P742** — sRGB → HSV (hexcone standard = palette
/// `hsv.rs:274-310`, com guarda de negativos e hue `rem_euclid`).
fn rgb_to_hsv_p742(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r.max(0.0);
    let g = g.max(0.0);
    let b = b.max(0.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    if d == 0.0 || max == 0.0 {
        return (0.0, 0.0, max);
    }
    let s = d / max;
    let h = hue_hexcone_p742(r, g, b, max, d);
    (h, s, max)
}

/// **P742** — sRGB → HSL (hexcone standard; mesmo hue de HSV).
fn rgb_to_hsl_p742(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r.max(0.0);
    let g = g.max(0.0);
    let b = b.max(0.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let l = (max + min) / 2.0;
    if d == 0.0 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let h = hue_hexcone_p742(r, g, b, max, d);
    (h, s, l)
}

/// Hue hexcone em graus (partilhado por HSV/HSL — paridade palette).
fn hue_hexcone_p742(r: f32, g: f32, b: f32, max: f32, d: f32) -> f32 {
    let h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    h.rem_euclid(360.0)
}

/// **P742** — sRGB → Luma (palette: luminância linear Rec.709 re-codificada
/// com a curva sRGB).
fn srgb_to_luma_p742(r: f32, g: f32, b: f32) -> f32 {
    let y = 0.2126 * srgb_to_linear_p476(r)
        + 0.7152 * srgb_to_linear_p476(g)
        + 0.0722 * srgb_to_linear_p476(b);
    linear_to_srgb(y)
}

impl Color {
    /// Espaço da cor (mapeamento 1:1 variante → `ColorSpace`; paridade
    /// vanilla `ProcessColor::space`, `color.rs:1535`).
    pub fn space(self) -> ColorSpace {
        match self {
            Color::Srgb { .. } => ColorSpace::Srgb,
            Color::Luma { .. } => ColorSpace::Luma,
            Color::LinearRgb { .. } => ColorSpace::LinearRgb,
            Color::Oklab { .. } => ColorSpace::Oklab,
            Color::Oklch { .. } => ColorSpace::Oklch,
            Color::Hsl { .. } => ColorSpace::Hsl,
            Color::Hsv { .. } => ColorSpace::Hsv,
            Color::Cmyk { .. } => ColorSpace::Cmyk,
        }
    }

    /// **P742** — conversão para outro espaço. Hub sRGB: `to_rgba_f32` +
    /// conversão para o destino. Cmyk naive (o ICC do vanilla é scope-out
    /// ADR-0083). Mesmo espaço → identidade (paridade vanilla `to_space`).
    pub fn to_space(self, target: ColorSpace) -> Color {
        if self.space() == target {
            return self;
        }
        match target {
            ColorSpace::Oklab => {
                let (l, a, b, alpha) = to_oklab_p476(self);
                Color::oklab(l, a, b, alpha)
            }
            ColorSpace::Oklch => {
                let (l, c, h, alpha) = to_oklch_p476(self);
                Color::oklch(l, c, h, alpha)
            }
            _ => {
                let (r, g, b, a) = self.to_rgba_f32();
                match target {
                    ColorSpace::Srgb => Color::srgb_f32(r, g, b, a),
                    ColorSpace::Luma => Color::Luma { l: srgb_to_luma_p742(r, g, b), a },
                    ColorSpace::LinearRgb => Color::linear_rgb(
                        srgb_to_linear_p476(r),
                        srgb_to_linear_p476(g),
                        srgb_to_linear_p476(b),
                        a,
                    ),
                    ColorSpace::Hsv => {
                        let (h, s, v) = rgb_to_hsv_p742(r, g, b);
                        Color::hsv(h, s, v, a)
                    }
                    ColorSpace::Hsl => {
                        let (h, s, l) = rgb_to_hsl_p742(r, g, b);
                        Color::hsl(h, s, l, a)
                    }
                    ColorSpace::Cmyk => {
                        let k = 1.0 - r.max(g).max(b);
                        if k >= 1.0 {
                            Color::cmyk(0.0, 0.0, 0.0, 1.0)
                        } else {
                            Color::cmyk(
                                (1.0 - r - k) / (1.0 - k),
                                (1.0 - g - k) / (1.0 - k),
                                (1.0 - b - k) / (1.0 - k),
                                k,
                            )
                        }
                    }
                    ColorSpace::Oklab | ColorSpace::Oklch => {
                        unreachable!("Oklab/Oklch tratados acima")
                    }
                }
            }
        }
    }

    /// **P742** — aumenta luminância por `factor` **no espaço da própria
    /// cor** (paridade palette `Lighten::lighten` medida): `increase` sobre
    /// os canais de estímulo (Srgb/LinearRgb: r,g,b; Luma/Oklab/Oklch/Hsl:
    /// l; Hsv: v). Cmyk (tipo próprio do vanilla, `color.rs:2235`):
    /// `u - u*factor` por componente. Alpha preservado.
    pub fn lighten(self, factor: f32) -> Self {
        match self {
            Color::Srgb { r, g, b, a } => Color::srgb_f32(
                increase_p742(r, factor),
                increase_p742(g, factor),
                increase_p742(b, factor),
                a,
            ),
            Color::Luma { l, a } => Color::Luma { l: increase_p742(l, factor), a },
            Color::LinearRgb { r, g, b, a } => Color::linear_rgb(
                increase_p742(r, factor),
                increase_p742(g, factor),
                increase_p742(b, factor),
                a,
            ),
            Color::Oklab { l, a, b, alpha } => {
                Color::oklab(increase_p742(l, factor), a, b, alpha)
            }
            Color::Oklch { l, c, h, alpha } => {
                Color::oklch(increase_p742(l, factor), c, h, alpha)
            }
            Color::Hsl { h, s, l, a } => Color::hsl(h, s, increase_p742(l, factor), a),
            Color::Hsv { h, s, v, a } => Color::hsv(h, s, increase_p742(v, factor), a),
            Color::Cmyk { c, m, y, k } => {
                let f = |u: f32| (u - u * factor).clamp(0.0, 1.0);
                Color::cmyk(f(c), f(m), f(y), f(k))
            }
        }
    }

    /// **P742** — diminui luminância por `factor`: `lighten(-factor)` para
    /// as variantes palette (paridade `Darken::darken`); Cmyk:
    /// `u + (1-u)*factor` por componente (`color.rs:2240`).
    pub fn darken(self, factor: f32) -> Self {
        match self {
            Color::Cmyk { c, m, y, k } => {
                let f = |u: f32| (u + (1.0 - u) * factor).clamp(0.0, 1.0);
                Color::cmyk(f(c), f(m), f(y), f(k))
            }
            _ => self.lighten(-factor),
        }
    }

    /// **P744** — converte a cor para um vec4 no espaço indicado. Usado por
    /// `mix` (paridade vanilla `ProcessColor::to_vec4`).
    fn to_vec4_in_space(self, space: ColorSpace) -> [f32; 4] {
        let c = if self.space() == space { self } else { self.to_space(space) };
        match c {
            Color::Srgb { r, g, b, a } => [r, g, b, a],
            Color::Luma { l, a } => [l, 0.0, 0.0, a],
            Color::LinearRgb { r, g, b, a } => [r, g, b, a],
            Color::Oklab { l, a, b, alpha } => [l, a, b, alpha],
            Color::Oklch { l, c, h, alpha } => [l, c, h, alpha],
            Color::Hsl { h, s, l, a } => [h, s, l, a],
            Color::Hsv { h, s, v, a } => [h, s, v, a],
            Color::Cmyk { c, m, y, k } => [c, m, y, k],
        }
    }

    /// **P744** — constrói uma cor a partir de um vec4 no espaço indicado.
    fn from_vec4_in_space(v: [f32; 4], space: ColorSpace) -> Self {
        match space {
            ColorSpace::Srgb => Color::srgb_f32(v[0], v[1], v[2], v[3]),
            ColorSpace::Luma => Color::Luma { l: v[0], a: v[3] },
            ColorSpace::LinearRgb => Color::linear_rgb(v[0], v[1], v[2], v[3]),
            ColorSpace::Oklab => Color::oklab(v[0], v[1], v[2], v[3]),
            ColorSpace::Oklch => Color::oklch(v[0], v[1], v[2].rem_euclid(360.0), v[3]),
            ColorSpace::Hsl => Color::hsl(v[0].rem_euclid(360.0), v[1], v[2], v[3]),
            ColorSpace::Hsv => Color::hsv(v[0].rem_euclid(360.0), v[1], v[2], v[3]),
            ColorSpace::Cmyk => Color::cmyk(v[0], v[1], v[2], v[3]),
        }
    }

    /// Interpolação linear entre `self` e `other` no espaço indicado.
    /// `weight` [0.0, 1.0]: 0.0 = self; 1.0 = other.
    /// Default `space: None` = Oklab (paridade P476/P742). O resultado fica
    /// no espaço indicado (P744).
    pub fn mix(self, other: Self, weight: f32, space: Option<ColorSpace>) -> Self {
        let space = space.unwrap_or(ColorSpace::Oklab);
        let mut c0 = self.to_vec4_in_space(space);
        let mut c1 = other.to_vec4_in_space(space);
        let t = weight.clamp(0.0, 1.0);

        // Para espaços com hue, percorre o círculo cromático pelo caminho
        // mais curto (paridade vanilla `mix_iter` hue_index).
        let hue_idx = match space {
            ColorSpace::Oklch => Some(2),
            ColorSpace::Hsl | ColorSpace::Hsv => Some(0),
            _other => None, // neutro: espaços cromáticos sem componente angular/hue não necessitam de correcção circular
        };
        if let Some(idx) = hue_idx {
            if (c0[idx] - c1[idx]).abs() > 180.0 {
                if c0[idx] < c1[idx] {
                    c0[idx] += 360.0;
                } else {
                    c1[idx] += 360.0;
                }
            }
        }

        let mixed = [
            c0[0] + (c1[0] - c0[0]) * t,
            c0[1] + (c1[1] - c0[1]) * t,
            c0[2] + (c1[2] - c0[2]) * t,
            c0[3] + (c1[3] - c0[3]) * t,
        ];
        Color::from_vec4_in_space(mixed, space)
    }

    /// **P744** — negação no espaço indicado (default Oklab). Converte para o
    /// espaço, aplica a negação própria desse espaço, converte de volta ao
    /// espaço original de `self`.
    pub fn negate(self, space: Option<ColorSpace>) -> Self {
        let original = self.space();
        let space = space.unwrap_or(ColorSpace::Oklab);
        let negated = match self.to_space(space) {
            Color::Srgb { r, g, b, a } => Color::srgb_f32(1.0 - r, 1.0 - g, 1.0 - b, a),
            Color::Luma { l, a } => Color::Luma { l: 1.0 - l, a },
            Color::LinearRgb { r, g, b, a } => {
                Color::linear_rgb(1.0 - r, 1.0 - g, 1.0 - b, a)
            }
            Color::Oklab { l, a, b, alpha } => Color::oklab(1.0 - l, -a, -b, alpha),
            Color::Oklch { l, c, h, alpha } => Color::oklch(1.0 - l, c, h + 180.0, alpha),
            Color::Hsl { h, s, l, a } => Color::hsl(h + 180.0, s, l, a),
            Color::Hsv { h, s, v, a } => Color::hsv(h + 180.0, s, v, a),
            Color::Cmyk { c, m, y, k } => Color::cmyk(1.0 - c, 1.0 - m, 1.0 - y, k),
        };
        negated.to_space(original)
    }

    /// **P744** — rotação de hue no espaço indicado (default Oklch). Só é
    /// válida em espaços com hue (Oklch, Hsl, Hsv); os restantes devolvem
    /// `None` para o chamador emitir o erro vanilla.
    pub fn rotate(self, angle_deg: f32, space: Option<ColorSpace>) -> Option<Self> {
        let original = self.space();
        let space = space.unwrap_or(ColorSpace::Oklch);
        if !matches!(space, ColorSpace::Oklch | ColorSpace::Hsl | ColorSpace::Hsv) {
            return None;
        }
        let rotated = match self.to_space(space) {
            Color::Oklch { l, c, h, alpha } => Color::oklch(l, c, h + angle_deg, alpha),
            Color::Hsl { h, s, l, a } => Color::hsl(h + angle_deg, s, l, a),
            Color::Hsv { h, s, v, a } => Color::hsv(h + angle_deg, s, v, a),
            _ => unreachable!("to_space(hue-space) devolve sempre o espaço hue"),
        };
        Some(rotated.to_space(original))
    }

    /// **P742** — aumenta saturação por `factor` (paridade palette medida):
    /// Hsl/Hsv → `increase` sobre a saturação; restantes → converte para
    /// Hsv, `increase` sobre `s`, converte de volta ao espaço original.
    /// Luma → `None` (o vanilla erra "cannot saturate grayscale color" —
    /// o chamador em rules emite a mensagem verbatim com span).
    pub fn saturate(self, factor: f32) -> Option<Self> {
        match self {
            Color::Luma { .. } => None,
            Color::Hsl { h, s, l, a } => {
                Some(Color::hsl(h, increase_p742(s, factor), l, a))
            }
            Color::Hsv { h, s, v, a } => {
                Some(Color::hsv(h, increase_p742(s, factor), v, a))
            }
            _ => {
                let original = self.space();
                let Color::Hsv { h, s, v, a } = self.to_space(ColorSpace::Hsv) else {
                    unreachable!("to_space(Hsv) devolve sempre Hsv")
                };
                Some(Color::hsv(h, increase_p742(s, factor), v, a).to_space(original))
            }
        }
    }

    /// **P742** — diminui saturação por `factor` (paridade vanilla:
    /// `saturate(-factor)`). Luma → `None`.
    pub fn desaturate(self, factor: f32) -> Option<Self> {
        self.saturate(-factor)
    }

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

    /// **P744** — ajusta o alpha por `scale`. `scale > 0` → opacify;
    /// `scale < 0` → transparentize. Fórmula vanilla (`scale_alpha`):
    /// `alpha' = alpha + scale * (scale > 0 ? 1 - alpha : alpha)`.
    fn scale_alpha(self, scale: f32) -> Option<Self> {
        let factor = if scale > 0.0 { 1.0 - self.alpha() } else { self.alpha() };
        self.with_alpha((self.alpha() + scale * factor).clamp(0.0, 1.0))
    }

    /// Alpha da cor (Cmyk não tem → `None` no caller).
    fn alpha(&self) -> f32 {
        match *self {
            Color::Srgb { a, .. } => a,
            Color::Luma { a, .. } => a,
            Color::LinearRgb { a, .. } => a,
            Color::Oklab { alpha, .. } => alpha,
            Color::Oklch { alpha, .. } => alpha,
            Color::Hsl { a, .. } => a,
            Color::Hsv { a, .. } => a,
            Color::Cmyk { .. } => 1.0,
        }
    }

    /// Cria cópia da cor com novo alpha (Cmyk não tem alpha → `None`).
    fn with_alpha(self, alpha: f32) -> Option<Self> {
        Some(match self {
            Color::Srgb { r, g, b, .. } => Color::srgb_f32(r, g, b, alpha),
            Color::Luma { l, .. } => Color::Luma { l, a: alpha },
            Color::LinearRgb { r, g, b, .. } => Color::linear_rgb(r, g, b, alpha),
            Color::Oklab { l, a, b, .. } => Color::oklab(l, a, b, alpha),
            Color::Oklch { l, c, h, .. } => Color::oklch(l, c, h, alpha),
            Color::Hsl { h, s, l, .. } => Color::hsl(h, s, l, alpha),
            Color::Hsv { h, s, v, .. } => Color::hsv(h, s, v, alpha),
            Color::Cmyk { .. } => return None,
        })
    }

    /// **P744** — torna a cor mais transparente. `None` = Cmyk (sem alpha).
    pub fn transparentize(self, factor: f32) -> Option<Self> {
        self.scale_alpha(-factor)
    }

    /// **P744** — torna a cor mais opaca. `None` = Cmyk (sem alpha).
    pub fn opacify(self, factor: f32) -> Option<Self> {
        self.scale_alpha(factor)
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
    fn color_copy_clone_disponivel() {
        let c = Color::rgb(100, 200, 50);
        let c2 = c; // Copy
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
        let m50 = c.mix(d, 0.5, Some(ColorSpace::Srgb));
        assert_eq!(m50.to_hex(), "#805a88");

        // 25% d:
        let m25 = c.mix(d, 0.25, Some(ColorSpace::Srgb));
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
        assert!(g.saturate(0.2).is_none());
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
        assert_eq!(c.lighten(0.2), Color::cmyk(0.4, 0.4, 0.4, 0.4));
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
        let roundtrip = VANILLA_RED.to_space(ColorSpace::Hsv).to_space(ColorSpace::Srgb);
        assert_eq!(roundtrip.to_srgb(), VANILLA_RED.to_srgb());
    }

    #[test]
    fn p742_to_space_identidade_mesmo_espaco() {
        assert_eq!(VANILLA_RED.to_space(ColorSpace::Srgb), VANILLA_RED);
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
        let lightened = red.lighten(0.0);
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
        let darkened = blue.darken(0.0);
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
        let (l0, a0, b0, _) = to_oklab_p476(red);
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
        let (l0, a0, b0, _) = to_oklab_p476(blue);
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
        let (l0, _, _, _) = to_oklab_p476(red);
        let (l1, _, _, _) = to_oklab_p476(blue);
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
        match c.to_space(ColorSpace::Hsv) {
            Color::Hsv { h, s, v, .. } => (h, s, v),
            _ => unreachable!("to_space(Hsv) devolve sempre Hsv"),
        }
    }

    #[test]
    fn p477_saturate_zero_nao_altera_chroma() {
        let red = Color::rgb(255, 0, 0);
        let (_, c0, _, _) = to_oklch_p476(red);
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
        let (h0, s0, v0) = hsv_of(c);
        assert!((s0 - 0.5).abs() < 1e-5);
        let sat = c.saturate(0.1).unwrap();
        let (h1, s1, v1) = hsv_of(sat);
        assert!((s1 - 0.55).abs() < 1e-4, "s esperado 0.55; obtido {s1}");
        assert!((h1 - h0).abs() < 1e-3, "hue preservado");
        assert!((v1 - v0).abs() < 1e-5, "value preservado");
        assert_eq!(sat.to_srgb(), (255, 115, 115, 255));
    }

    #[test]
    fn p477_desaturate_zero_nao_altera_chroma() {
        let blue = Color::rgb(0, 0, 255);
        let (_, c0, _, _) = to_oklch_p476(blue);
        let (_, c1, _, _) = to_oklch_p476(blue.desaturate(0.0).unwrap());
        assert!((c1 - c0).abs() < 1e-5);
    }

    #[test]
    fn p477_desaturate_grande_clampado_a_zero() {
        // Vanilla: desaturate(100%) zera `s` em HSV preservando v → branco
        // (não cinzento — corrigido P742). Chroma Oklch ≈ 0 mantém-se.
        let red = Color::rgb(255, 0, 0);
        let desat = red.desaturate(1.0).unwrap();
        let (_, c, _, _) = to_oklch_p476(desat);
        assert!((c - 0.0).abs() < 1e-5, "desaturate(1.0) → c=0; obtido {}", c);
        assert_eq!(desat.to_srgb(), (255, 255, 255, 255), "v=1 preservado → branco");
    }

    #[test]
    fn p742_saturate_preserva_hue_e_value() {
        let (h0, s0, v0) = hsv_of(VANILLA_RED);
        let sat = VANILLA_RED.saturate(0.2).unwrap();
        let (h1, s1, v1) = hsv_of(sat);
        assert!(s1 > s0, "s deve aumentar: s0={s0}, s1={s1}");
        assert!((h1 - h0).abs() < 1e-3, "hue preservado");
        assert!((v1 - v0).abs() < 1e-5, "value preservado");
        // Valor medido no vanilla: red.saturate(20%) → rgb("#ff372b").
        assert_eq!(sat.to_srgb(), (255, 55, 43, 255));
    }

    #[test]
    fn p742_desaturate_preserva_hue_e_value() {
        let blue = Color::rgb(0, 0, 255);
        let (h0, s0, v0) = hsv_of(blue);
        let desat = blue.desaturate(0.05).unwrap();
        let (h1, s1, v1) = hsv_of(desat);
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
        assert_eq!(red.rotate(180.0, Some(ColorSpace::Hsl)).unwrap().to_hex(), "#00ffff");
        assert_eq!(red.rotate(180.0, Some(ColorSpace::Hsv)).unwrap().to_hex(), "#00ffff");
    }

    #[test]
    fn p744_mix_default_igual_oklab() {
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        assert_eq!(
            red.mix(blue, 0.5, None).to_hex(),
            red.mix(blue, 0.5, Some(ColorSpace::Oklab)).to_hex(),
        );
    }

    #[test]
    fn p744_mix_no_espaco_indicado() {
        // O resultado fica no espaço indicado: mix em Oklab → Oklab;
        // mix em Srgb → Srgb (repr hex).
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        let mixed_oklab = red.mix(blue, 0.5, Some(ColorSpace::Oklab));
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
        assert_eq!(c.transparentize(0.5).unwrap().to_hex(), "#ff413640");
        assert_eq!(c.opacify(0.5).unwrap().to_hex(), "#ff4136c0");
    }

    #[test]
    fn p744_transparentize_opacify_cmyk_none() {
        let c = Color::cmyk(0.1, 0.2, 0.3, 0.4);
        assert!(c.transparentize(0.5).is_none());
        assert!(c.opacify(0.5).is_none());
    }
}
