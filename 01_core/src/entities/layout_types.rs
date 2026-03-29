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
    // Variantes futuras — NÃO implementar sem ADR:
    // Shape { pos: Point, geometry: Geometry },
    // Image { pos: Point, size: Size, data: Bytes },
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
}
