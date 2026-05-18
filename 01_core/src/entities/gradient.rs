//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient.md
//! @prompt-hash 8d9730a3
//! @layer L1
//! @updated 2026-05-15
//!
//! **P262 (M9d / M7+5; ADR-0087 PROPOSTO Gradient Linear-only;
//! cumpre ADR-0086 §"Critério revisão" Paint::Gradient variant
//! activada; precedente N=3 do pattern P257/P261)** — wrapper
//! enum sobre tipos de gradient (Linear apenas; Radial/Conic
//! comentários reserva). Sub-componente `GradientStop` com
//! `Option<Ratio>` paridade vanilla auto-spacing.
//!
//! **ColorSpace fixo Oklab** per decisão user P262 Q2 (paridade
//! vanilla default). Interpolação em Oklab via `Color::oklab`;
//! conversão final PDF via `Color::to_rgba_f32()`.
//!
//! **ADR-0039 preservado**: TextStyle.fill: Option<Color>
//! inalterado.
//!
//! Cross-references:
//! - Vanilla `lab/typst-original/.../visualize/gradient.rs` (1366 LoC).
//! - ADR-0087 — Gradient Linear-only (IMPLEMENTADO P262).
//! - ADR-0086 — Paint wrapper (Solid only → §"Critério revisão"
//!   cumprido por este passo).

use std::sync::Arc;
use crate::entities::color::{Color, ColorSpace};
use crate::entities::layout_types::{Angle, Ratio};

/// Sub-componente per ADR-0029 §exclusões.
///
/// `offset: Option<Ratio>` permite auto-spacing (paridade vanilla):
/// stops com `None` recebem distribuição uniforme entre stops
/// com offset explícito (ou nos extremos implícitos 0% / 100%).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    pub color:  Color,
    pub offset: Option<Ratio>,
}

impl GradientStop {
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }

    pub fn unspaced(color: Color) -> Self {
        Self { color, offset: None }
    }
}

/// P273 — Define a que bounding box o gradient é relativo.
///
/// Paridade vanilla `RelativeTo { Self_, Parent }`. Cristalino
/// simplifica `Smart<RelativeTo>` → `Option<RelativeTo>` per
/// ADR-0064 §Caso A (`None` = Auto = `Self_` default).
///
/// **Default `Self_`** (vanilla `unwrap_or_else(|| if on_text {
/// Parent } else { Self_ })` simplificado — cristalino ignora
/// contexto `on_text` por enquanto; materializável futuro se
/// necessário).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeTo {
    /// Relative ao próprio shape (bounding box self).
    #[default]
    Self_,
    /// Relative ao parent container (bounding box parent).
    Parent,
}

/// Linear gradient — paridade vanilla LinearGradient.
///
/// ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
/// default). Campos `space`/`relative`/`anti_alias` vanilla
/// não materializados; ver ADR-0087 §scope-outs.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::linear(...)` = `ColorSpace::Oklab`
    /// (preserva P262 behavior bit-exact).
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field per ADR-0091
    /// §"Anotação cumulativa P273". `None` = Auto = `Self_` default;
    /// preserva P262/P263/P270.1/P270.2 bit-exact.
    pub relative: Option<RelativeTo>,
}

impl Linear {
    /// Calcula offsets efectivos com auto-spacing aplicado
    /// (paridade vanilla — stops com offset=None recebem
    /// distribuição uniforme entre stops com offset explícito
    /// ou extremos implícitos 0.0 / 1.0).
    ///
    /// Algoritmo:
    /// 1. Identifica runs consecutivos de offset=None.
    /// 2. Cada run delimitado por offset explícito (ou extremos
    ///    implícitos 0.0 / 1.0).
    /// 3. Interpola offsets em [prev, next] distribuição uniforme.
    pub fn effective_offsets(&self) -> Vec<f32> {
        let n = self.stops.len();
        if n == 0 { return Vec::new(); }
        if n == 1 {
            return vec![self.stops[0].offset.map(|r| r.0 as f32).unwrap_or(0.0)];
        }

        let mut offs: Vec<Option<f32>> = self.stops.iter()
            .map(|s| s.offset.map(|r| r.0 as f32))
            .collect();

        // Extremos implícitos quando ausentes.
        if offs[0].is_none() { offs[0] = Some(0.0); }
        if offs[n - 1].is_none() { offs[n - 1] = Some(1.0); }

        // Preenche runs de None entre offsets explícitos.
        let mut result = vec![0.0_f32; n];
        let mut i = 0;
        while i < n {
            if let Some(v) = offs[i] {
                result[i] = v;
                i += 1;
                continue;
            }
            // Encontra próximo offset explícito.
            let mut j = i;
            while j < n && offs[j].is_none() { j += 1; }
            // i-1 tem offset explícito (já preenchido em result[i-1]).
            // j tem offset explícito (offs[j]).
            let prev = result[i - 1];
            let next = offs[j].unwrap();
            let gap = j - i + 1;
            for k in 0..gap {
                result[i + k] = prev + (next - prev) * ((k + 1) as f32) / (gap as f32);
            }
            i = j;
        }
        result
    }

    /// Amostra a cor interpolada em parâmetro t ∈ [0, 1].
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P262
    /// behavior bit-exact (dispatcher chama `interpolate_oklab` original
    /// para arm Oklab).
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let offs = self.effective_offsets();
        let n = self.stops.len();
        if n == 0 { return Color::rgb(0, 0, 0); }
        if n == 1 { return self.stops[0].color; }

        // Encontrar par [i, i+1] tal que offs[i] <= t <= offs[i+1].
        for i in 0..(n - 1) {
            let o0 = offs[i];
            let o1 = offs[i + 1];
            if t >= o0 && t <= o1 {
                let local_t = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                return interpolate_in_space(
                    self.stops[i].color, self.stops[i + 1].color, local_t, self.space);
            }
        }
        // Fallback (clamp): extremo apropriado.
        if t <= offs[0] { self.stops[0].color } else { self.stops[n - 1].color }
    }
}

/// Interpolação linear em Oklab.
///
/// Converte ambas as cores para representação Oklab (via
/// to_rgba_f32 → linear path); interpola linearmente os componentes
/// L, a, b; retorna `Color::oklab` com alpha interpolado.
fn interpolate_oklab(c0: Color, c1: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (l0, a0, b0, alpha0) = color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, alpha1) = color_to_oklab_with_alpha(c1);
    Color::oklab(
        l0 + (l1 - l0) * t,
        a0 + (a1 - a0) * t,
        b0 + (b1 - b0) * t,
        alpha0 + (alpha1 - alpha0) * t,
    )
}

/// Converte qualquer Color para Oklab (L, a, b, alpha).
///
/// Para cores já em Oklab retorna campos directos; para outras
/// converte via sRGB → linear → Oklab (caminho inverso).
///
/// **P268.2**: promovido a `pub` para acessibilidade cross-crate
/// (L3 `oklab_delta_e` reutilização literal — ver anotação ADR-0089
/// §"Anotação cumulativa P268.2" + `diagnostico-adaptive-n-passo-268-2.md`
/// §A.1). Function body preservada literal P262.
/// **P272**: `oklab_delta_e` removed (ADR-0090 REVOGADO; adaptive N
/// não aplicável a Coons). `color_to_oklab_with_alpha` permanece
/// `pub` — usada por outros helpers L1/L3 (Oklab interpolation arms
/// L1 + multispace helpers L3).
pub fn color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
        _ => {
            // Converter via sRGB → linear → Oklab.
            let (r, g, b, alpha) = c.to_rgba_f32();
            let lin_r = srgb_to_linear(r);
            let lin_g = srgb_to_linear(g);
            let lin_b = srgb_to_linear(b);
            let (lab_l, lab_a, lab_b) = linear_rgb_to_oklab(lin_r, lin_g, lin_b);
            (lab_l, lab_a, lab_b, alpha)
        }
    }
}

/// Gamma 2.2 inversa (sRGB → linear).
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// linear sRGB → Oklab (paridade ICC; constantes da publicação Björn Ottosson 2020).
fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5  * r + 0.680_699_56 * g + 0.107_396_96  * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_71  * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    let l_lab = 0.210_454_26 * l_ + 0.793_617_8   * m_ - 0.004_072_047 * s_;
    let a_lab = 1.977_998_5  * l_ - 2.428_592_2   * m_ + 0.450_593_7   * s_;
    let b_lab = 0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77  * s_;

    (l_lab, a_lab, b_lab)
}

// ── P270 — Multi-space interpolation helpers ────────────────────

/// Dispatcher central: interpola `c0` ↔ `c1` no espaço `space` em `t`.
///
/// **P270**: arm `Oklab` chama `interpolate_oklab` P262 literal
/// (preserva bytes P262/P264/P267 bit-exact). Demais 7 arms chamam
/// helpers per-space.
fn interpolate_in_space(c0: Color, c1: Color, t: f32, space: ColorSpace) -> Color {
    let t = t.clamp(0.0, 1.0);
    match space {
        ColorSpace::Oklab     => interpolate_oklab(c0, c1, t),
        ColorSpace::Oklch     => interpolate_oklch(c0, c1, t),
        ColorSpace::Srgb      => interpolate_srgb(c0, c1, t),
        ColorSpace::Luma      => interpolate_luma(c0, c1, t),
        ColorSpace::LinearRgb => interpolate_linear_rgb(c0, c1, t),
        ColorSpace::Hsl       => interpolate_hsl(c0, c1, t),
        ColorSpace::Hsv       => interpolate_hsv(c0, c1, t),
        ColorSpace::Cmyk      => interpolate_cmyk(c0, c1, t),
    }
}

/// Hue interpolation com wrap shorter (CSS standard; vanilla paridade
/// literal portada de `mix_iter` linha 1126-1136).
fn interpolate_hue_shorter(h0: f32, h1: f32, t: f32) -> f32 {
    let diff = h1 - h0;
    let wrapped_h1 = if diff.abs() > 180.0 {
        if diff > 0.0 { h1 - 360.0 } else { h1 + 360.0 }
    } else {
        h1
    };
    (h0 + (wrapped_h1 - h0) * t).rem_euclid(360.0)
}

/// Lerp linear simples.
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// sRGB → linear (gamma 2.2 inversa). Espelho privado de `srgb_to_linear`
/// já existente, mantido aqui por encapsulamento — helper P270.
#[inline]
fn srgb_to_linear_local(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// sRGB componentes (extracted; lossless para `Color::Srgb`).
fn to_srgb_components(c: Color) -> (f32, f32, f32, f32) {
    c.to_rgba_f32()  // todas variants convertem para sRGB nativo
}

/// Linear RGB componentes (extracted; via sRGB → linear inverso).
fn to_linear_rgb_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::LinearRgb { r, g, b, a } => (r, g, b, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            (srgb_to_linear_local(r), srgb_to_linear_local(g),
             srgb_to_linear_local(b), a)
        }
    }
}

/// Luma componente (Rec 709 luminance) + alpha.
fn to_luma_components(c: Color) -> (f32, f32) {
    match c {
        Color::Luma { l, a } => (l, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            // Rec 709 luminance (paridade cristalino `Color::Luma {l, l, l, a}` to_rgba).
            let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            (l, a)
        }
    }
}

/// Oklch componentes (l, c, h, alpha) — via Oklab cartesiano → polar.
fn to_oklch_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklch { l, c, h, alpha } => (l, c, h, alpha),
        _ => {
            let (l, a, b, alpha) = color_to_oklab_with_alpha(c);
            let chroma = (a * a + b * b).sqrt();
            let h = b.atan2(a).to_degrees().rem_euclid(360.0);
            (l, chroma, h, alpha)
        }
    }
}

/// HSL componentes (h, s, l, alpha).
fn to_hsl_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Hsl { h, s, l, a } => (h, s, l, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            let l = (max + min) * 0.5;
            let s = if delta == 0.0 {
                0.0
            } else {
                delta / (1.0 - (2.0 * l - 1.0).abs()).max(1e-6)
            };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta).rem_euclid(6.0))
            } else if max == g {
                60.0 * ((b - r) / delta + 2.0)
            } else {
                60.0 * ((r - g) / delta + 4.0)
            };
            (h.rem_euclid(360.0), s.clamp(0.0, 1.0), l.clamp(0.0, 1.0), a)
        }
    }
}

/// HSV componentes (h, s, v, alpha).
fn to_hsv_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Hsv { h, s, v, a } => (h, s, v, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            let v = max;
            let s = if max == 0.0 { 0.0 } else { delta / max };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta).rem_euclid(6.0))
            } else if max == g {
                60.0 * ((b - r) / delta + 2.0)
            } else {
                60.0 * ((r - g) / delta + 4.0)
            };
            (h.rem_euclid(360.0), s.clamp(0.0, 1.0), v.clamp(0.0, 1.0), a)
        }
    }
}

/// CMYK componentes (c, m, y, k).
fn to_cmyk_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Cmyk { c, m, y, k } => (c, m, y, k),
        _ => {
            let (r, g, b, _) = c.to_rgba_f32();
            let k = 1.0 - r.max(g).max(b);
            if k >= 1.0 - 1e-6 {
                (0.0, 0.0, 0.0, 1.0)
            } else {
                let denom = 1.0 - k;
                let cc = (1.0 - r - k) / denom;
                let mm = (1.0 - g - k) / denom;
                let yy = (1.0 - b - k) / denom;
                (cc.clamp(0.0, 1.0), mm.clamp(0.0, 1.0),
                 yy.clamp(0.0, 1.0), k.clamp(0.0, 1.0))
            }
        }
    }
}

/// Interpolação sRGB componentwise.
fn interpolate_srgb(c0: Color, c1: Color, t: f32) -> Color {
    let (r0, g0, b0, a0) = to_srgb_components(c0);
    let (r1, g1, b1, a1) = to_srgb_components(c1);
    Color::srgb_f32(lerp(r0, r1, t), lerp(g0, g1, t), lerp(b0, b1, t), lerp(a0, a1, t))
}

/// Interpolação linear RGB componentwise.
fn interpolate_linear_rgb(c0: Color, c1: Color, t: f32) -> Color {
    let (r0, g0, b0, a0) = to_linear_rgb_components(c0);
    let (r1, g1, b1, a1) = to_linear_rgb_components(c1);
    Color::linear_rgb(lerp(r0, r1, t), lerp(g0, g1, t), lerp(b0, b1, t), lerp(a0, a1, t))
}

/// Interpolação Luma (grayscale; alpha preservado).
fn interpolate_luma(c0: Color, c1: Color, t: f32) -> Color {
    let (l0, a0) = to_luma_components(c0);
    let (l1, a1) = to_luma_components(c1);
    // Color::luma só aceita l (alpha=1.0 implícito); construct via Color::Luma directo.
    Color::Luma { l: lerp(l0, l1, t), a: lerp(a0, a1, t) }
}

/// Interpolação Oklch (polar; hue-wrap shorter).
fn interpolate_oklch(c0: Color, c1: Color, t: f32) -> Color {
    let (l0, ch0, h0, a0) = to_oklch_components(c0);
    let (l1, ch1, h1, a1) = to_oklch_components(c1);
    Color::oklch(
        lerp(l0, l1, t),
        lerp(ch0, ch1, t),
        interpolate_hue_shorter(h0, h1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação HSL (polar; hue-wrap shorter).
fn interpolate_hsl(c0: Color, c1: Color, t: f32) -> Color {
    let (h0, s0, l0, a0) = to_hsl_components(c0);
    let (h1, s1, l1, a1) = to_hsl_components(c1);
    Color::hsl(
        interpolate_hue_shorter(h0, h1, t),
        lerp(s0, s1, t),
        lerp(l0, l1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação HSV (polar; hue-wrap shorter).
fn interpolate_hsv(c0: Color, c1: Color, t: f32) -> Color {
    let (h0, s0, v0, a0) = to_hsv_components(c0);
    let (h1, s1, v1, a1) = to_hsv_components(c1);
    Color::hsv(
        interpolate_hue_shorter(h0, h1, t),
        lerp(s0, s1, t),
        lerp(v0, v1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação CMYK componentwise.
fn interpolate_cmyk(c0: Color, c1: Color, t: f32) -> Color {
    let (c0_, m0, y0, k0) = to_cmyk_components(c0);
    let (c1_, m1, y1, k1) = to_cmyk_components(c1);
    Color::cmyk(lerp(c0_, c1_, t), lerp(m0, m1, t), lerp(y0, y1, t), lerp(k0, k1, t))
}

/// Radial gradient — paridade vanilla `RadialGradient` subset
/// (per ADR-0088 P264).
///
/// **3 campos materializados**: stops + center + radius.
/// Scope-outs (per ADR-0088): focal_center, focal_radius,
/// space (Oklab fixo), relative (bbox-local), anti_alias (true).
///
/// **PDF render Radial fallback Solid** (first_stop_color) até
/// P265 dedicado materializar `/ShadingType 3` real.
#[derive(Debug, Clone, PartialEq)]
pub struct Radial {
    pub stops:  Arc<[GradientStop]>,
    pub center: crate::entities::axes::Axes<Ratio>,
    pub radius: Ratio,
    /// P269 — focal_center activado per ADR-0088 §"Anotação cumulativa P269".
    /// Default via construtor `Gradient::radial(...)` = `center`.
    pub focal_center: crate::entities::axes::Axes<Ratio>,
    /// P269 — focal_radius activado per ADR-0088 §"Anotação cumulativa P269".
    /// Default via construtor `Gradient::radial(...)` = `Ratio(0.0)`.
    pub focal_radius: Ratio,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::radial(...)` = `ColorSpace::Oklab`.
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field. `None` = Auto =
    /// `Self_` default; preserva P264/P265/P269 bit-exact.
    pub relative: Option<RelativeTo>,
}

impl Radial {
    /// Auto-spacing paridade `Linear::effective_offsets` (P262).
    pub fn effective_offsets(&self) -> Vec<f32> {
        let n = self.stops.len();
        if n == 0 { return Vec::new(); }
        if n == 1 {
            return vec![self.stops[0].offset.map(|r| r.0 as f32).unwrap_or(0.0)];
        }

        let mut offs: Vec<Option<f32>> = self.stops.iter()
            .map(|s| s.offset.map(|r| r.0 as f32))
            .collect();

        if offs[0].is_none() { offs[0] = Some(0.0); }
        if offs[n - 1].is_none() { offs[n - 1] = Some(1.0); }

        let mut result = vec![0.0_f32; n];
        let mut i = 0;
        while i < n {
            if let Some(v) = offs[i] {
                result[i] = v;
                i += 1;
                continue;
            }
            let mut j = i;
            while j < n && offs[j].is_none() { j += 1; }
            let prev = result[i - 1];
            let next = offs[j].unwrap();
            let gap = j - i + 1;
            for k in 0..gap {
                result[i + k] = prev + (next - prev) * ((k + 1) as f32) / (gap as f32);
            }
            i = j;
        }
        result
    }

    /// Amostragem 1D em t ∈ [0, 1] (paridade `Linear::sample`).
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P264/P269
    /// behavior bit-exact.
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let offs = self.effective_offsets();
        let n = self.stops.len();
        if n == 0 { return Color::rgb(0, 0, 0); }
        if n == 1 { return self.stops[0].color; }

        for i in 0..(n - 1) {
            let o0 = offs[i];
            let o1 = offs[i + 1];
            if t >= o0 && t <= o1 {
                let local_t = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                return interpolate_in_space(
                    self.stops[i].color, self.stops[i + 1].color, local_t, self.space);
            }
        }
        if t <= offs[0] { self.stops[0].color } else { self.stops[n - 1].color }
    }
}

/// Conic gradient — paridade vanilla `ConicGradient` subset
/// (per ADR-0089 P267).
///
/// **3 campos materializados**: stops + center + angle.
/// Scope-outs (per ADR-0089): space (Oklab fixo), relative
/// (bbox-local), anti_alias (true). **Sem `focal_*`** — não
/// existe em ConicGradient vanilla (exclusivo Radial).
///
/// **PDF render Conic fallback Solid** (first_stop_color) até
/// P268 dedicado materializar shading real.
#[derive(Debug, Clone, PartialEq)]
pub struct Conic {
    pub stops:  Arc<[GradientStop]>,
    pub center: crate::entities::axes::Axes<Ratio>,
    pub angle:  Angle,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::conic(...)` = `ColorSpace::Oklab`.
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field. `None` = Auto =
    /// `Self_` default; preserva P267/P272 bit-exact (Coons RGB N=stops*4
    /// + Coons CMYK N=stops).
    pub relative: Option<RelativeTo>,
}

impl Conic {
    /// Auto-spacing paridade `Linear::effective_offsets` (P262)
    /// e `Radial::effective_offsets` (P264).
    pub fn effective_offsets(&self) -> Vec<f32> {
        let n = self.stops.len();
        if n == 0 { return Vec::new(); }
        if n == 1 {
            return vec![self.stops[0].offset.map(|r| r.0 as f32).unwrap_or(0.0)];
        }

        let mut offs: Vec<Option<f32>> = self.stops.iter()
            .map(|s| s.offset.map(|r| r.0 as f32))
            .collect();

        if offs[0].is_none() { offs[0] = Some(0.0); }
        if offs[n - 1].is_none() { offs[n - 1] = Some(1.0); }

        let mut result = vec![0.0_f32; n];
        let mut i = 0;
        while i < n {
            if let Some(v) = offs[i] {
                result[i] = v;
                i += 1;
                continue;
            }
            let mut j = i;
            while j < n && offs[j].is_none() { j += 1; }
            let prev = result[i - 1];
            let next = offs[j].unwrap();
            let gap = j - i + 1;
            for k in 0..gap {
                result[i + k] = prev + (next - prev) * ((k + 1) as f32) / (gap as f32);
            }
            i = j;
        }
        result
    }

    /// Amostragem 1D em t ∈ [0, 1] (paridade `Linear::sample` +
    /// `Radial::sample`).
    ///
    /// Para Conic, `t` representa a fração da circumferência
    /// (0 = ângulo inicial; 1 = volta completa).
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P267
    /// behavior bit-exact.
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let offs = self.effective_offsets();
        let n = self.stops.len();
        if n == 0 { return Color::rgb(0, 0, 0); }
        if n == 1 { return self.stops[0].color; }

        for i in 0..(n - 1) {
            let o0 = offs[i];
            let o1 = offs[i + 1];
            if t >= o0 && t <= o1 {
                let local_t = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                return interpolate_in_space(
                    self.stops[i].color, self.stops[i + 1].color, local_t, self.space);
            }
        }
        if t <= offs[0] { self.stops[0].color } else { self.stops[n - 1].color }
    }
}

/// Gradient — enum tagged paridade vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),   // P264 — descomentado per ADR-0088
    Conic(Arc<Conic>),     // P267 — descomentado per ADR-0089 (cluster 3/3 completo)
}

impl Gradient {
    /// Construtor Linear (P262; ADR-0087).
    ///
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P262
    /// behavior bit-exact).
    pub fn linear(
        stops: impl Into<Arc<[GradientStop]>>,
        angle: Angle,
    ) -> Self {
        Gradient::Linear(Arc::new(Linear {
            stops: stops.into(),
            angle,
            space: ColorSpace::Oklab,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Construtor Linear com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR — §ColorSpace runtime activado).
    pub fn linear_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        angle: Angle,
        space: ColorSpace,
    ) -> Self {
        Gradient::Linear(Arc::new(Linear {
            stops: stops.into(),
            angle,
            space,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Construtor Radial (P264; ADR-0088).
    ///
    /// **P269**: defaults focal_center = center; focal_radius = 0.
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P264/P269
    /// behavior bit-exact).
    pub fn radial(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center: center,         // P269 default
            focal_radius: Ratio(0.0),     // P269 default
            space: ColorSpace::Oklab,     // P270 default
            relative: None,               // P273 — Auto (Self_).
        }))
    }

    /// Construtor Radial com focal_* explícito (P269).
    /// Default `space: Oklab` (P270).
    pub fn radial_with_focal(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
        focal_center: crate::entities::axes::Axes<Ratio>,
        focal_radius: Ratio,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center,
            focal_radius,
            space: ColorSpace::Oklab,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Construtor Radial com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR). Defaults focal=(center, 0) preserva P264/P269.
    pub fn radial_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
        space: ColorSpace,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center: center,
            focal_radius: Ratio(0.0),
            space,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Construtor Conic (P267; ADR-0089).
    ///
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P267
    /// behavior bit-exact).
    pub fn conic(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        angle: Angle,
    ) -> Self {
        Gradient::Conic(Arc::new(Conic {
            stops: stops.into(),
            center,
            angle,
            space: ColorSpace::Oklab,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Construtor Conic com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR).
    pub fn conic_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        angle: Angle,
        space: ColorSpace,
    ) -> Self {
        Gradient::Conic(Arc::new(Conic {
            stops: stops.into(),
            center,
            angle,
            space,
            relative: None,  // P273 — Auto (Self_).
        }))
    }

    /// Retorna primeira cor do primeiro stop. Usado como
    /// fallback para `Paint::to_color()` quando consumer
    /// precisa de Color literal (Solid path).
    pub fn first_stop_color(&self) -> Color {
        match self {
            Gradient::Linear(l) => l.stops.first()
                .map(|s| s.color)
                .unwrap_or(Color::rgb(0, 0, 0)),
            Gradient::Radial(r) => r.stops.first()
                .map(|s| s.color)
                .unwrap_or(Color::rgb(0, 0, 0)),
            Gradient::Conic(c) => c.stops.first()
                .map(|s| s.color)
                .unwrap_or(Color::rgb(0, 0, 0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_stop_new_com_offset() {
        let s = GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.5));
        assert_eq!(s.color, Color::rgb(255, 0, 0));
        assert_eq!(s.offset, Some(Ratio(0.5)));
    }

    #[test]
    fn gradient_stop_unspaced() {
        let s = GradientStop::unspaced(Color::rgb(0, 255, 0));
        assert_eq!(s.color, Color::rgb(0, 255, 0));
        assert_eq!(s.offset, None);
    }

    #[test]
    fn gradient_linear_construcao_2_stops() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        if let Gradient::Linear(l) = &g {
            assert_eq!(l.stops.len(), 2);
            assert_eq!(l.angle.to_rad(), 0.0);
        } else {
            panic!("esperado Gradient::Linear");
        }
    }

    #[test]
    fn gradient_first_stop_color() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(100, 50, 25), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(100, 50, 25));
    }

    #[test]
    fn linear_effective_offsets_explicit() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.5)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let offs = l.effective_offsets();
        assert_eq!(offs, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn linear_effective_offsets_auto_spacing_all_none() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let offs = l.effective_offsets();
        // 3 stops igualmente espaçados: 0.0 / 0.5 / 1.0
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn linear_effective_offsets_auto_spacing_middle_explicit() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.7)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let offs = l.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.7).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn linear_sample_extremos_returns_stops() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        // Amostragem nos extremos deve retornar cores dos stops
        // (após Oklab roundtrip — pequena tolerância).
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, _, _) = c1.to_rgba_f32();
        // Vermelho stop @ 0 → r próximo de 1.0
        assert!(r0 > 0.9, "sample(0.0) deve retornar próximo de vermelho, r={}", r0);
        // Azul stop @ 1 → r próximo de 0.0
        assert!(r1 < 0.1, "sample(1.0) deve retornar próximo de azul, r={}", r1);
    }

    #[test]
    fn linear_sample_meio_interpola() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        // Sample em 0.5 deve ser mistura entre vermelho e azul.
        let c_meio = l.sample(0.5);
        let (r, _, b, _) = c_meio.to_rgba_f32();
        // Em Oklab → sRGB, a mistura tem componentes r e b
        // ambos > 0 mas nenhum dominante.
        assert!(r > 0.05 && r < 0.95, "sample(0.5) r intermediate: {}", r);
        assert!(b > 0.05 && b < 0.95, "sample(0.5) b intermediate: {}", b);
    }

    #[test]
    fn gradient_clone_arc_o1() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        let g2 = g.clone();
        // Arc clones share storage.
        if let (Gradient::Linear(l1), Gradient::Linear(l2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(l1, l2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn gradient_partial_eq() {
        let g1 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        let g2 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        let g3 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn linear_effective_offsets_1_stop() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(100, 100, 100), Ratio(0.3)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_eq!(l.effective_offsets(), vec![0.3]);
    }

    #[test]
    fn linear_sample_clamp_above_1() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        // t > 1.0 deve clamp.
        let c = l.sample(1.5);
        let c_ref = l.sample(1.0);
        assert_eq!(c, c_ref);
    }

    // ── P264 (ADR-0088 Gradient Radial-only) ───────────────────────────

    use crate::entities::axes::Axes;

    #[test]
    fn p264_radial_construcao_2_stops() {
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.stops.len(), 2);
            assert_eq!(r.center.x, Ratio(0.5));
            assert_eq!(r.center.y, Ratio(0.5));
            assert_eq!(r.radius, Ratio(0.5));
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p264_radial_first_stop_color() {
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(100, 200, 50), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(100, 200, 50));
    }

    #[test]
    fn p264_radial_clone_arc_o1() {
        let g = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g2 = g.clone();
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(r1, r2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn p264_radial_partial_eq() {
        let g1 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g2 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g3 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn p264_radial_effective_offsets_auto_spacing() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let offs = r.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn p264_radial_sample_extremos() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        // Amostragem nos extremos: r próximo de vermelho/azul.
        let c0 = r.sample(0.0);
        let c1 = r.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0) ≈ vermelho, r={}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ azul, r={}, b={}", r1, b1);
    }

    #[test]
    fn p264_radial_sample_clamp_above_1() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let c = r.sample(1.5);
        let c_ref = r.sample(1.0);
        assert_eq!(c, c_ref);
    }

    #[test]
    fn p264_gradient_radial_to_paint_via_from() {
        use crate::entities::paint::Paint;
        let g = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let p: Paint = g.into();
        assert!(matches!(p, Paint::Gradient(Gradient::Radial(_))));
    }

    #[test]
    fn p264_radial_center_non_default() {
        let center = Axes::new(Ratio(0.25), Ratio(0.75));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.4),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_eq!(r.center.x, Ratio(0.25));
        assert_eq!(r.center.y, Ratio(0.75));
        assert_eq!(r.radius, Ratio(0.4));
    }

    // ── P269 (ADR-0088 §focal_* revogado parcialmente) — Radial focal_center + focal_radius

    #[test]
    fn p269_radial_construcao_default_focal_preserva_p264() {
        // Gradient::radial(...) sem focal args → focal=(center, 0)
        // → bytes /Coords idênticos P265.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            center,
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.focal_center, center, "default focal_center = center");
            assert_eq!(r.focal_radius, Ratio(0.0), "default focal_radius = 0");
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p269_gradient_radial_with_focal_construct() {
        let g = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),  // center
            Ratio(0.5),                          // radius
            Axes::new(Ratio(0.3), Ratio(0.4)),  // focal_center
            Ratio(0.1),                          // focal_radius
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.focal_center, Axes::new(Ratio(0.3), Ratio(0.4)));
            assert_eq!(r.focal_radius, Ratio(0.1));
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p269_radial_focal_struct_pub_fields() {
        // focal_center + focal_radius são pub fields acessíveis directamente.
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.2), Ratio(0.3)),
            focal_radius: Ratio(0.05),
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_eq!(r.focal_center.x, Ratio(0.2));
        assert_eq!(r.focal_center.y, Ratio(0.3));
        assert_eq!(r.focal_radius, Ratio(0.05));
    }

    #[test]
    fn p269_radial_partial_eq_focal_distingue() {
        // Radials com focal diferente NÃO devem comparar igual.
        let stops = Arc::from(vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]);
        let r1 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let r2 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.3), Ratio(0.4)),  // diff
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_ne!(r1, r2, "focal_center diferente → PartialEq false");
    }

    #[test]
    fn p269_radial_partial_eq_focal_radius_distingue() {
        let stops = Arc::from(vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]);
        let r1 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let r2 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.1),  // diff,
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_ne!(r1, r2, "focal_radius diferente → PartialEq false");
    }

    #[test]
    fn p269_radial_clone_arc_o1_focal_preservado() {
        let g = Gradient::radial_with_focal(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.3), Ratio(0.4)),
            Ratio(0.05),
        );
        let g2 = g.clone();
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g, &g2) {
            assert_eq!(r1.focal_center, r2.focal_center);
            assert_eq!(r1.focal_radius, r2.focal_radius);
            // Arc::ptr_eq garante O(1) clone (mesmo allocation interno).
            assert!(Arc::ptr_eq(r1, r2), "clone deve preservar Arc identity");
        } else {
            panic!("esperado Gradient::Radial em ambos");
        }
    }

    #[test]
    fn p269_radial_focal_radius_zero_default_idempotente() {
        // 2 construções via Gradient::radial(...) produzem focal_radius=0.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g1 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            center, Ratio(0.5),
        );
        let g2 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 0, 255), Ratio(0.0))],
            center, Ratio(0.5),
        );
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g1, &g2) {
            assert_eq!(r1.focal_radius, Ratio(0.0));
            assert_eq!(r2.focal_radius, Ratio(0.0));
            assert_eq!(r1.focal_radius, r2.focal_radius);
        }
    }

    #[test]
    fn p269_radial_sample_inalterado_por_focal_default() {
        // sample(t) é 1D em cristalino — não usa focal. Verificar
        // que defaults focal não afectam saídas P264.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g_default = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            center, Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g_default {
            // sample(0.0) ≈ vermelho; sample(1.0) ≈ azul; idêntico P264.
            let c0 = r.sample(0.0);
            let c1 = r.sample(1.0);
            let (r0, _, _, _) = c0.to_rgba_f32();
            let (r1, _, b1, _) = c1.to_rgba_f32();
            assert!(r0 > 0.9, "sample(0.0) ≈ red preservado P264");
            assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue preservado P264");
        }
    }

    #[test]
    fn p269_radial_sample_inalterado_com_focal_explicit() {
        // sample(t) é 1D — focal arbitrário NÃO afecta output sample.
        // Esta é uma propriedade arquitectural cristalino vs vanilla
        // (vanilla tem sample_at(x,y) que usa focal; cristalino só tem
        // sample(t) 1D).
        let g_focal = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.2), Ratio(0.3)),  // focal arbitrário
            Ratio(0.1),
        );
        if let Gradient::Radial(r) = &g_focal {
            let c0 = r.sample(0.0);
            let c1 = r.sample(1.0);
            let (r0, _, _, _) = c0.to_rgba_f32();
            let (r1, _, b1, _) = c1.to_rgba_f32();
            // sample(t) ignora focal — endpoints idênticos default.
            assert!(r0 > 0.9, "sample(0.0) ≈ red ignora focal");
            assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue ignora focal");
        }
    }

    #[test]
    fn p269_radial_first_stop_color_inalterado_focal() {
        // first_stop_color é independente de focal.
        let g = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(123, 45, 67), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.1), Ratio(0.2)),
            Ratio(0.05),
        );
        let c = g.first_stop_color();
        let (r, _g, _b, _) = c.to_rgba_f32();
        assert!((r * 255.0 - 123.0).abs() < 1.5, "first_stop_color ignora focal");
    }

    // ── P267 (ADR-0089 Gradient Conic-only) ────────────────────────────

    #[test]
    fn p267_conic_construcao_2_stops() {
        let g = Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        if let Gradient::Conic(c) = &g {
            assert_eq!(c.stops.len(), 2);
            assert_eq!(c.center.x, Ratio(0.5));
            assert_eq!(c.angle.to_rad(), 0.0);
        } else {
            panic!("esperado Gradient::Conic");
        }
    }

    #[test]
    fn p267_conic_first_stop_color() {
        let g = Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(200, 100, 50), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(200, 100, 50));
    }

    #[test]
    fn p267_conic_clone_arc_o1() {
        let g = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g2 = g.clone();
        if let (Gradient::Conic(c1), Gradient::Conic(c2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(c1, c2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn p267_conic_partial_eq() {
        let g1 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g2 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g3 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(90.0),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn p267_conic_effective_offsets_auto_spacing() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let offs = c.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn p267_conic_sample_extremos() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let c0 = c.sample(0.0);
        let c1 = c.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0) ≈ vermelho, r={}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ azul, r={}, b={}", r1, b1);
    }

    #[test]
    fn p267_conic_sample_clamp_above_1() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        let c_clamp = c.sample(1.5);
        let c_ref = c.sample(1.0);
        assert_eq!(c_clamp, c_ref);
    }

    #[test]
    fn p267_gradient_conic_to_paint_via_from() {
        use crate::entities::paint::Paint;
        let g = Gradient::conic(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let p: Paint = g.into();
        assert!(matches!(p, Paint::Gradient(Gradient::Conic(_))));
    }

    #[test]
    fn p267_conic_angle_non_default() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.25), Ratio(0.75)),
            angle: Angle::deg(90.0),
            space: ColorSpace::Oklab,
            relative: None,
        };
        assert_eq!(c.center.x, Ratio(0.25));
        assert_eq!(c.center.y, Ratio(0.75));
        assert!((c.angle.to_rad() - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    }

    // ── P270 (ADR-0091 EM VIGOR — ColorSpace runtime cross-variant) ──

    fn red_blue_stops() -> Vec<GradientStop> {
        vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]
    }

    // ── Hue-wrap shorter helper (4 tests) ──

    #[test]
    fn p270_hue_shorter_no_wrap() {
        // diff < 180°: caminho directo.
        // h0=0, h1=90, t=0.5 → 45.
        let h = interpolate_hue_shorter(0.0, 90.0, 0.5);
        assert!((h - 45.0).abs() < 1e-3, "no-wrap: got {}", h);
    }

    #[test]
    fn p270_hue_shorter_wrap_positive_to_negative() {
        // diff > 180° positivo → wrap pelo lado negativo.
        // h0=10, h1=350, diff=340; wrap: h1=350-360=-10; t=0.5 → 0.
        let h = interpolate_hue_shorter(10.0, 350.0, 0.5);
        // Resultado esperado: caminho curto via 0°; (10 + (-10-10)*0.5)=0 mod 360 = 0.
        assert!((h - 0.0).abs() < 1e-3 || (h - 360.0).abs() < 1e-3,
            "wrap positive: got {}", h);
    }

    #[test]
    fn p270_hue_shorter_wrap_negative_to_positive() {
        // diff < -180° → wrap pelo lado positivo.
        // h0=350, h1=10, diff=-340; wrap: h1=10+360=370; t=0.5 → 360 mod 360 = 0.
        let h = interpolate_hue_shorter(350.0, 10.0, 0.5);
        assert!((h - 0.0).abs() < 1e-3 || (h - 360.0).abs() < 1e-3,
            "wrap negative: got {}", h);
    }

    #[test]
    fn p270_hue_shorter_exactly_180() {
        // Edge case diff == 180° → wrap fica em sentido positivo
        // (CSS default; implementação cristalina: condição estrita > 180° não dispara
        // para exactly 180°, então caminho directo é usado).
        let h = interpolate_hue_shorter(0.0, 180.0, 0.5);
        assert!((h - 90.0).abs() < 1e-3, "exactly_180: got {}", h);
    }

    // ── L1 sample multi-space — 8 spaces × 3 variants = 24 tests ──

    fn make_linear_with_space(space: ColorSpace) -> Linear {
        Linear {
            stops: Arc::from(red_blue_stops()),
            angle: Angle::rad(0.0),
            space,
            relative: None,
        }
    }
    fn make_radial_with_space(space: ColorSpace) -> Radial {
        Radial {
            stops: Arc::from(red_blue_stops()),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space,
            relative: None,
        }
    }
    fn make_conic_with_space(space: ColorSpace) -> Conic {
        Conic {
            stops: Arc::from(red_blue_stops()),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::rad(0.0),
            space,
            relative: None,
        }
    }

    // Linear

    #[test]
    fn p270_linear_sample_oklab_preserva_p262() {
        // Default Oklab — preserva P262 (red↔blue endpoints).
        let l = make_linear_with_space(ColorSpace::Oklab);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0).r ≈ 1.0; got {}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue; got r={}, b={}", r1, b1);
    }

    #[test]
    fn p270_linear_sample_srgb_endpoints() {
        let l = make_linear_with_space(ColorSpace::Srgb);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_linear_sample_linear_rgb_endpoints() {
        let l = make_linear_with_space(ColorSpace::LinearRgb);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_linear_sample_luma_endpoints() {
        let l = make_linear_with_space(ColorSpace::Luma);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        // red→blue em Luma → grayscale: ambos endpoints próximos a 0 (red=0.21 luma; blue=0.07).
        // Confirma que sample produz cores válidas (no panic).
        let _ = (c0.to_rgba_f32(), c1.to_rgba_f32());
    }

    #[test]
    fn p270_linear_sample_oklch_shorter_hue_endpoints() {
        let l = make_linear_with_space(ColorSpace::Oklch);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7, "Oklch sample(0.0) ≈ red; r={}", r0);
        assert!(b1 > 0.7, "Oklch sample(1.0) ≈ blue; b={}", b1);
    }

    #[test]
    fn p270_linear_sample_hsl_endpoints() {
        let l = make_linear_with_space(ColorSpace::Hsl);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    #[test]
    fn p270_linear_sample_hsv_endpoints() {
        let l = make_linear_with_space(ColorSpace::Hsv);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    #[test]
    fn p270_linear_sample_cmyk_endpoints() {
        let l = make_linear_with_space(ColorSpace::Cmyk);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        // CMYK red ≈ (0,1,1,0) → rgb≈(1,0,0); CMYK blue ≈ (1,1,0,0) → rgb≈(0,0,1).
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    // Radial

    #[test]
    fn p270_radial_sample_oklab_preserva_p264() {
        let r = make_radial_with_space(ColorSpace::Oklab);
        let c0 = r.sample(0.0);
        let c1 = r.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_radial_sample_srgb() {
        let r = make_radial_with_space(ColorSpace::Srgb);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_linear_rgb() {
        let r = make_radial_with_space(ColorSpace::LinearRgb);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_luma() {
        let r = make_radial_with_space(ColorSpace::Luma);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_oklch() {
        let r = make_radial_with_space(ColorSpace::Oklch);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_hsl() {
        let r = make_radial_with_space(ColorSpace::Hsl);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_hsv() {
        let r = make_radial_with_space(ColorSpace::Hsv);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_cmyk() {
        let r = make_radial_with_space(ColorSpace::Cmyk);
        let _ = r.sample(0.5);
    }

    // Conic

    #[test]
    fn p270_conic_sample_oklab_preserva_p267() {
        let c = make_conic_with_space(ColorSpace::Oklab);
        let c0 = c.sample(0.0);
        let c1 = c.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_conic_sample_srgb() {
        let c = make_conic_with_space(ColorSpace::Srgb);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_linear_rgb() {
        let c = make_conic_with_space(ColorSpace::LinearRgb);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_luma() {
        let c = make_conic_with_space(ColorSpace::Luma);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_oklch() {
        let c = make_conic_with_space(ColorSpace::Oklch);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_hsl() {
        let c = make_conic_with_space(ColorSpace::Hsl);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_hsv() {
        let c = make_conic_with_space(ColorSpace::Hsv);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_cmyk() {
        let c = make_conic_with_space(ColorSpace::Cmyk);
        let _ = c.sample(0.5);
    }

    // Construtores defaults preservam bit-exact P262/P264/P267

    #[test]
    fn p270_linear_default_construtor_space_oklab() {
        let g = Gradient::linear(red_blue_stops(), Angle::rad(0.0));
        if let Gradient::Linear(l) = g {
            assert_eq!(l.space, ColorSpace::Oklab,
                "Gradient::linear default deve ser ColorSpace::Oklab");
        } else { panic!("expected Linear"); }
    }

    #[test]
    fn p270_radial_default_construtor_space_oklab() {
        let g = Gradient::radial(
            red_blue_stops(),
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = g {
            assert_eq!(r.space, ColorSpace::Oklab);
        } else { panic!("expected Radial"); }
    }

    #[test]
    fn p270_conic_default_construtor_space_oklab() {
        let g = Gradient::conic(
            red_blue_stops(),
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::rad(0.0),
        );
        if let Gradient::Conic(c) = g {
            assert_eq!(c.space, ColorSpace::Oklab);
        } else { panic!("expected Conic"); }
    }

    #[test]
    fn p270_linear_with_space_explicit() {
        let g = Gradient::linear_with_space(
            red_blue_stops(),
            Angle::rad(0.0),
            ColorSpace::Hsl,
        );
        if let Gradient::Linear(l) = g {
            assert_eq!(l.space, ColorSpace::Hsl);
        } else { panic!("expected Linear"); }
    }
}
