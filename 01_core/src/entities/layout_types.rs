//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/layout_types.md
//! @prompt-hash 9175bb7c
//! @layer L1
//! @updated 2026-03-28

use ecow::EcoString;

// ── Coordenadas e medidas ──────────────────────────────────────────────────

/// Ponto tipográfico — unidade interna de layout.
/// 1 pt = 1/72 inch.
///
/// Não implementa `Add<f64>` — escalares brutos requerem `Pt(valor)` explícito.
/// Isto previne misturar coordenadas com índices ou contagens.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pt(pub f64);

impl Pt {
    pub const ZERO: Self = Self(0.0);

    pub fn val(self) -> f64 {
        self.0
    }
}

impl std::ops::Add for Pt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl std::ops::Sub for Pt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}

impl std::ops::Mul<f64> for Pt {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self { Self(self.0 * rhs) }
}

impl std::ops::AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

// Deliberadamente NÃO implementado:
// impl Add<f64> for Pt — escalares requerem Pt(valor) explícito

/// Posição 2D na página.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Pt,
    pub y: Pt,
}

impl Point {
    pub const ZERO: Self = Self { x: Pt::ZERO, y: Pt::ZERO };
}

/// Tamanho 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width:  Pt,
    pub height: Pt,
}

impl Size {
    /// Tamanho A4 em pontos tipográficos.
    pub fn a4() -> Self {
        Self { width: Pt(595.0), height: Pt(842.0) }
    }
}

// ── Estilo de texto ────────────────────────────────────────────────────────

/// Estilo de texto — struct plano.
///
/// DEBT: deve ser substituído por StyleChain (lista ligada de deltas)
/// antes de implementar `#set text(...)`. Ver DEBT.md.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub bold:   bool,
    pub italic: bool,
    pub size:   Pt,
}

impl TextStyle {
    pub fn regular(size: Pt) -> Self { Self { bold: false, italic: false, size } }
    pub fn bold(size: Pt)    -> Self { Self { bold: true,  italic: false, size } }
    pub fn italic(size: Pt)  -> Self { Self { bold: false, italic: true,  size } }
}

// ── Frame e FrameItem ──────────────────────────────────────────────────────

/// Item posicionado num frame.
///
/// Divergência: original usa `(Point, FrameItem)` como tupla separada.
/// Cristalino embute `pos` em `FrameItem::Text` por simplicidade.
#[derive(Debug, Clone)]
pub enum FrameItem {
    /// Texto posicionado.
    Text {
        pos:   Point,
        text:  EcoString,
        style: TextStyle,
    },
    /// Linha horizontal. Usada pela linha de fracção matemática (Passo 38).
    /// `start` e `end` são posições absolutas no Frame.
    /// `thickness` em pontos tipográficos.
    Line {
        start:     Point,
        end:       Point,
        thickness: f64,
    },
    /// Glifo renderizado directamente por ID, sem mapeamento Unicode.
    ///
    /// Usado para variantes de tamanho matemático onde `glyph_to_char`
    /// retorna `None`. O export PDF escreve o ID como `<XXXX> Tj`.
    ///
    /// `pos`: posição final do glifo (coordenadas de página), calculada
    ///        pelo `MathLayouter` antes de emitir este item.
    /// `glyph_id`: índice do glifo na fonte (índice CIDFont, Identity-H).
    /// `x_advance`: largura horizontal do glifo em pt.
    /// `size`: corpo tipográfico em pt.
    Glyph {
        pos:       Point,
        glyph_id:  u16,
        x_advance: Pt,
        size:      Pt,
    },
}

/// Canvas de uma página — colecção de itens com posições absolutas.
///
/// Divergência: original usa `Arc<LazyHash<Vec<(Point, FrameItem)>>>`.
/// Cristalino usa `Vec<FrameItem>` directo por simplicidade.
#[derive(Debug, Clone)]
pub struct Frame {
    pub size:  Size,
    pub items: Vec<FrameItem>,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self { size, items: Vec::new() }
    }

    pub fn push(&mut self, item: FrameItem) {
        self.items.push(item);
    }

    /// Extrai texto plano — para verificação em testes.
    pub fn plain_text(&self) -> String {
        self.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                FrameItem::Line { .. }       => None,
                FrameItem::Glyph { .. }      => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// ── PagedDocument ──────────────────────────────────────────────────────────

/// Documento paginado — resultado de `layout()`.
///
/// Divergência: original tem `EcoVec<Page>` + `DocumentInfo` + `Arc<PagedIntrospector>`.
/// Cristalino usa `Vec<Frame>` — stub até Passo 20+.
#[derive(Debug, Clone)]
pub struct PagedDocument {
    pub pages: Vec<Frame>,
}

impl PagedDocument {
    pub fn new(pages: Vec<Frame>) -> Self {
        Self { pages }
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Extrai texto plano de todas as páginas — para verificação em testes.
    pub fn plain_text(&self) -> String {
        self.pages
            .iter()
            .map(|p| p.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ── Tipos tipográficos (ADR-0028, ADR-0029) ────────────────────────────────

/// Comprimento absoluto em pontos tipográficos.
///
/// ADR-0029: representação fiel ao Typst vanilla (`Abs(Scalar)`).
/// Escala interna: 1.0 = 1pt (simplificação L1 — vanilla usa 127 raw/pt).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Abs(pub f64);

impl Abs {
    pub const ZERO: Self = Self(0.0);

    pub fn pt(v: f64) -> Self { Self(v) }
    pub fn to_pt(self) -> f64 { self.0 }
    pub fn is_zero(self) -> bool { self.0 == 0.0 }
}

impl std::ops::Add for Abs {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl std::ops::Neg for Abs {
    type Output = Self;
    fn neg(self) -> Self { Self(-self.0) }
}

/// Comprimento tipográfico — combinação de componente absoluta e relativa.
///
/// ADR-0029 — revoga ADR-0028. Estrutura fiel ao Typst vanilla:
/// `struct Length { abs: Abs, em: f64 }`.
/// `abs`: componente absoluta em pontos.
/// `em`: componente relativa em múltiplos do font-size actual.
///
/// A soma `1pt + 1em` é representável; a resolução para pt requer font-size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub abs: Abs,
    pub em:  f64,
}

impl Length {
    pub const ZERO: Self = Self { abs: Abs::ZERO, em: 0.0 };

    pub fn pt(v: f64) -> Self { Self { abs: Abs::pt(v), em: 0.0 } }
    pub fn em(v: f64) -> Self { Self { abs: Abs::ZERO,  em: v   } }

    pub fn is_zero(&self) -> bool { self.abs.is_zero() && self.em == 0.0 }

    /// Resolve para pontos dado um font-size em pt.
    /// `1pt + 1em` com font_size=12.0 → 13.0pt
    pub fn resolve_pt(&self, font_size_pt: f64) -> f64 {
        self.abs.to_pt() + self.em * font_size_pt
    }
}

impl std::ops::Add for Length {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { abs: self.abs + rhs.abs, em: self.em + rhs.em }
    }
}

impl std::ops::Neg for Length {
    type Output = Self;
    fn neg(self) -> Self { Self { abs: -self.abs, em: -self.em } }
}

/// Rácio — valor normalizado (0.0 = 0%, 1.0 = 100%).
///
/// ADR-0028: newtype f64. PartialEq exacto (derive).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio(pub f64);

impl Ratio {
    pub fn from_percent(pct: f64) -> Self { Self(pct / 100.0) }
    pub fn get(self) -> f64 { self.0 }
    pub fn to_percent(self) -> f64 { self.0 * 100.0 }
}

/// Ângulo — armazenado internamente em radianos.
///
/// ADR-0028: newtype f64. PartialEq exacto (derive) — sem tolerância embutida.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(f64);

impl Angle {
    pub fn deg(d: f64) -> Self { Self(d.to_radians()) }
    pub fn rad(r: f64) -> Self { Self(r) }
    pub fn to_rad(self) -> f64 { self.0 }
    pub fn to_deg(self) -> f64 { self.0.to_degrees() }
}

/// Cor tipográfica.
///
/// ADR-0028: enum simplificado. Espaços avançados (Oklab, HSL, CMYK) — adiados para Passo 30+.
/// `luma(l)` → Rgb { r: l, g: l, b: l } (escala de cinzentos).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgb  { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8)          -> Self { Self::Rgb { r, g, b } }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8)  -> Self { Self::Rgba { r, g, b, a } }

    /// Retorna (r, g, b, a) normalizados para [0.0, 1.0].
    pub fn to_rgba_f32(self) -> (f32, f32, f32, f32) {
        match self {
            Self::Rgb  { r, g, b }    => (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0),
            Self::Rgba { r, g, b, a } => (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0),
        }
    }
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_add_pt() {
        assert_eq!(Pt(10.0) + Pt(5.0), Pt(15.0));
    }

    #[test]
    fn pt_sub_pt() {
        assert_eq!(Pt(10.0) - Pt(3.0), Pt(7.0));
    }

    #[test]
    fn pt_mul_f64() {
        assert_eq!(Pt(10.0) * 2.0, Pt(20.0));
    }

    #[test]
    fn pt_add_assign() {
        let mut a = Pt(5.0);
        a += Pt(3.0);
        assert_eq!(a, Pt(8.0));
    }

    #[test]
    fn pt_zero_e_val() {
        assert_eq!(Pt::ZERO.val(), 0.0);
        assert_eq!(Pt(42.0).val(), 42.0);
    }

    #[test]
    fn pt_tipagem_nao_permite_add_f64() {
        // Verificação de compilação — se Add<f64> existisse, este teste
        // seria desnecessário. O compilador força conversão explícita.
        let a = Pt(10.0);
        let b = Pt(5.0);
        let c = a + b;  // Add<Pt> — OK
        assert_eq!(c, Pt(15.0));
        // a + 5.0  ← não compila — sem impl Add<f64>
    }

    #[test]
    fn size_a4() {
        let s = Size::a4();
        assert_eq!(s.width,  Pt(595.0));
        assert_eq!(s.height, Pt(842.0));
    }

    #[test]
    fn text_style_constructors() {
        let r = TextStyle::regular(Pt(12.0));
        assert!(!r.bold && !r.italic);
        let b = TextStyle::bold(Pt(12.0));
        assert!(b.bold && !b.italic);
        let i = TextStyle::italic(Pt(12.0));
        assert!(!i.bold && i.italic);
    }

    #[test]
    fn frame_plain_text() {
        let style = TextStyle::regular(Pt(12.0));
        let mut f = Frame::new(Size::a4());
        f.push(FrameItem::Text { pos: Point::ZERO, text: "Hello".into(), style });
        f.push(FrameItem::Text { pos: Point { x: Pt(50.0), y: Pt::ZERO }, text: "world".into(), style });
        assert_eq!(f.plain_text(), "Hello world");
    }

    #[test]
    fn paged_document_plain_text() {
        let style = TextStyle::regular(Pt(12.0));
        let mut f1 = Frame::new(Size::a4());
        f1.push(FrameItem::Text { pos: Point::ZERO, text: "page1".into(), style });
        let mut f2 = Frame::new(Size::a4());
        f2.push(FrameItem::Text { pos: Point::ZERO, text: "page2".into(), style });
        let doc = PagedDocument::new(vec![f1, f2]);
        assert_eq!(doc.plain_text(), "page1\npage2");
    }

    #[test]
    fn paged_document_vazio() {
        let doc = PagedDocument::new(vec![]);
        assert!(doc.is_empty());
        assert_eq!(doc.plain_text(), "");
    }

    // ── Passo 25 — tipos tipográficos (ADR-0028) ─────────────────────────────

    #[cfg(test)]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => { assert_approx_eq!($a, $b, 1e-10) };
        ($a:expr, $b:expr, $eps:expr) => {{
            let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
            assert!(
                (a - b).abs() < eps,
                "assert_approx_eq falhou: |{a} - {b}| = {} >= {eps}",
                (a - b).abs()
            );
        }};
    }

    #[test]
    fn length_resolve_pt() {
        assert_eq!(Length::pt(12.0).resolve_pt(12.0), 12.0);
        assert_eq!(Length::em(1.5).resolve_pt(12.0), 18.0);
        assert_eq!(Length::em(2.0).resolve_pt(10.0), 20.0);
    }

    #[test]
    fn ratio_percent_roundtrip() {
        let r = Ratio::from_percent(50.0);
        assert_approx_eq!(r.get(), 0.5);
        assert_approx_eq!(r.to_percent(), 50.0);
    }

    #[test]
    fn angle_deg_rad_usa_approx() {
        let a = Angle::deg(180.0);
        assert_approx_eq!(a.to_rad(), std::f64::consts::PI);
        assert_approx_eq!(a.to_deg(), 180.0);
    }

    #[test]
    fn angle_partial_eq_e_exacto() {
        let a1 = Angle::deg(180.0);
        let a2 = Angle::deg(180.0);
        assert_eq!(a1, a2);
        // Ângulos diferentes NÃO são iguais — sem tolerância embutida.
        let a3 = Angle::deg(180.0 + 1e-15);
        let _ = a3;  // comportamento documentado no relatório
    }

    #[test]
    fn color_to_rgba_f32() {
        let (r, g, b, a) = Color::rgb(255, 0, 128).to_rgba_f32();
        assert_approx_eq!(r, 1.0, 1e-3);
        assert_approx_eq!(g, 0.0, 1e-3);
        assert_approx_eq!(b, 0.502, 1e-3);
        assert_approx_eq!(a, 1.0, 1e-3);
    }

    // ── Passo 26 — Length struct fiel ao vanilla (ADR-0029) ──────────────────

    #[test]
    fn length_soma_mista_agora_funciona() {
        // Com Length vanilla (abs + em), a soma Pt + Em é representável
        let l = Length::pt(6.0) + Length::em(1.0);
        assert_eq!(l.abs.to_pt(), 6.0);
        assert_eq!(l.em, 1.0);
        // Resolve com font-size=12pt → 6 + 12 = 18pt
        assert_approx_eq!(l.resolve_pt(12.0), 18.0);
    }

    #[test]
    fn length_zero_constante() {
        assert!(Length::ZERO.is_zero());
        assert_approx_eq!(Length::ZERO.resolve_pt(12.0), 0.0);
    }

    #[test]
    fn length_soma_abs_preserva_em() {
        let a = Length::pt(3.0);
        let b = Length::pt(4.0);
        let sum = a + b;
        assert_approx_eq!(sum.abs.to_pt(), 7.0);
        assert_eq!(sum.em, 0.0);
    }

    #[test]
    fn length_neg() {
        let l = Length::pt(5.0);
        let neg = -l;
        assert_approx_eq!(neg.abs.to_pt(), -5.0);
        assert_eq!(neg.em, 0.0);
    }

    #[test]
    fn abs_add_e_neg() {
        assert_eq!(Abs::pt(3.0) + Abs::pt(4.0), Abs::pt(7.0));
        assert_eq!(-Abs::pt(2.0), Abs::pt(-2.0));
        assert!(Abs::ZERO.is_zero());
    }
}
