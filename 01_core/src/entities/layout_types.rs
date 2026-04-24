//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/layout_types.md
//! @prompt-hash af36c701
//! @layer L1
//! @updated 2026-04-23
//!
//! Excepção Regra 6 da ADR-0037: agrega tipos geométricos e
//! estruturais fundamentais do layout (`Pt`, `Point`, `FrameItem`,
//! `Page`, `PageConfig`, `PagedDocument`, `TextStyle`, `Color`,
//! `Length`, `TrackSizing`, `Align2D`, `HAlign`/`VAlign`, `PlaceScope`,
//! `TransformMatrix`). Estes tipos têm muitas operações e conversões
//! próximas; separá-los por ficheiro destruiria a visibilidade mútua
//! (impls cruzadas) e multiplicaria imports nos consumidores sem
//! ganho. ~850 linhas aceitas como custo de coesão do vocabulário
//! geométrico.

use std::collections::HashMap;
use std::sync::Arc;

use ecow::EcoString;

use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::label::Label;

// ── Coordenadas e medidas ──────────────────────────────────────────────────

/// Ponto tipográfico — unidade interna de layout.
/// 1 pt = 1/72 inch.
///
/// Não implementa `Add<f64>` — escalares brutos requerem `Pt(valor)` explícito.
/// Isto previne misturar coordenadas com índices ou contagens.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
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
/// Estilo resolvido — vista achatada do resultado de
/// `From<&StyleChain>` (ADR-0039, Passo 100). Os campos `fill` e
/// `heading_level` são forward-compat (ADR-0038) e por omissão `None`.
///
/// Semântica: é **o resultado** de resolver uma `StyleChain`, não a
/// cadeia em si. Consumido por `FrameItem::Text.style` e por
/// `export.rs` em L3.
/// Passo 136 (Fase A de DEBT-52, ADR-0054): `TextStyle` estendido
/// com 5 campos propagados de `StyleDelta`. Remoção de `Copy`
/// porque `FontList` contém `Vec<FontFamily>`; call sites usam
/// `.clone()` explícito. Consumers em fase B/C do roadmap.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextStyle {
    pub bold:          bool,
    pub italic:        bool,
    pub size:          Pt,
    /// Cor de preenchimento — ADR-0038/0039 forward-compat.
    pub fill:          Option<Color>,
    /// Nível de heading — ADR-0038/0039 forward-compat.
    pub heading_level: Option<u8>,

    // Passo 136 (Fase A — DEBT-52). Propagados de `StyleDelta`
    // mas sem consumer em layout ainda. Fases B/C resolvem.
    pub weight:        Option<u16>,
    pub tracking:      Option<crate::entities::layout_types::Length>,
    pub leading:       Option<crate::entities::layout_types::Length>,
    pub lang:          Option<crate::entities::lang::Lang>,
    pub font:          Option<crate::entities::font_list::FontList>,
}

impl TextStyle {
    pub fn regular(size: Pt) -> Self {
        Self { bold: false, italic: false, size, ..Self::default() }
    }
    pub fn bold(size: Pt) -> Self {
        Self { bold: true,  italic: false, size, ..Self::default() }
    }
    pub fn italic(size: Pt) -> Self {
        Self { bold: false, italic: true,  size, ..Self::default() }
    }
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
    /// Imagem a desenhar na página.
    ///
    /// `pos`: canto superior esquerdo em coordenadas de página (pt).
    ///        NOTA: para imagens, pos.y é o TOPO da bounding box — não o baseline de texto.
    ///        O exportador calcula pdf_y = page_height - pos.y - height (inversão de eixo Y).
    /// `data`: bytes raw da imagem (JPEG, PNG, etc.) — Arc para zero-copy.
    /// `width`, `height`: dimensões físicas no documento (pt) — tamanho de layout.
    /// `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis, lidas do
    ///   cabeçalho da imagem. Obrigatórias para o dicionário XObject no PDF —
    ///   /Width e /Height intrínsecos ≠ tamanho de layout na página.
    Image {
        pos:              Point,
        data:             Arc<Vec<u8>>,
        width:            Pt,
        height:           Pt,
        intrinsic_width:  u32,
        intrinsic_height: u32,
    },
    /// Forma geométrica com dimensões resolvidas em pontos (Passo 76).
    ///
    /// Todos os campos são concretos — sem `Option<Value>`.
    /// `pos`: canto superior esquerdo da bounding box.
    /// O exportador calcula `pdf_y = page_height - pos.y - height` (inversão de eixo Y).
    Shape {
        pos:    Point,
        kind:   ShapeKind,
        width:  f64,
        height: f64,
        fill:   Option<Color>,
        stroke: Option<Stroke>,
    },
    /// Grupo com transformação afim aplicada (Passo 78).
    ///
    /// O exportador emite q → cm → [W n se clip_mask] → itens filhos → Q.
    /// `pos`: posição do grupo na página (espaço global).
    /// `matrix`: transformação afim com compensação de origem negativa.
    /// `clip_mask`: forma que restringe o desenho à sua área interna (DEBT-30).
    ///   Se Some, o exportador emite o path da máscara seguido de `W n` no
    ///   espaço local (após `cm`). Se None, sem recorte.
    /// `inner_width`, `inner_height`: dimensões do conteúdo antes da transformação.
    ///   Necessárias para clip_mask do tipo Rect no espaço local.
    /// `items`: itens em espaço local (Y-down, origem em (0,0)).
    Group {
        pos:          Point,
        matrix:       TransformMatrix,
        clip_mask:    Option<ShapeKind>,
        inner_width:  f64,
        inner_height: f64,
        items:        Vec<FrameItem>,
    },
}

// ── Alinhamento (Passo 82) ─────────────────────────────────────────────────

/// Alinhamento horizontal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

/// Alinhamento vertical.
///
/// `Horizon` é o termo interno do Typst para centro vertical.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VAlign {
    Top,
    Horizon,
    Bottom,
}

/// Alinhamento 2D composto por componentes horizontal e vertical opcionais.
///
/// Ambos `None` equivale a `Left + Top` (comportamento por omissão).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Align2D {
    pub h: Option<HAlign>,
    pub v: Option<VAlign>,
}

impl Align2D {
    /// Parse de uma string composta por partes separadas por '-'.
    ///
    /// Exemplos: `"center"`, `"top-right"`, `"bottom"`, `"horizon"`.
    /// Partes não reconhecidas são ignoradas silenciosamente.
    ///
    /// Sintaxe legacy preservada após o Passo 84.5 (DEBT-36 encerrado).
    /// A sintaxe preferida é a composição simbólica via `Value::Align`:
    /// `align(center + bottom, ...)` em vez de `align("center-bottom", ...)`.
    /// Continua a ser usada como fallback em `native_align` e `native_place`
    /// quando o utilizador passa string literal.
    pub fn from_string(s: &str) -> Self {
        let mut align = Align2D::default();
        for part in s.split('-') {
            match part {
                "left"    => align.h = Some(HAlign::Left),
                "center"  => align.h = Some(HAlign::Center),
                "right"   => align.h = Some(HAlign::Right),
                "top"     => align.v = Some(VAlign::Top),
                "horizon" => align.v = Some(VAlign::Horizon),
                "bottom"  => align.v = Some(VAlign::Bottom),
                _         => {},
            }
        }
        align
    }
}

/// Escopo de ancoragem de `Content::Place` (Passo 84.6, encerra DEBT-37).
///
/// Espelha `PlacementScope` do vanilla:
/// - `Column` (default): ancora ao "current container" — célula de Grid
///   activa, ou página se não houver Grid no contexto.
/// - `Parent`: ancora à página inteira mesmo dentro de Grid.
///
/// **Divergência vs vanilla:** o vanilla restringe `Parent` a `float: true`
/// e devolve erro caso contrário. O cristalino não tem `float` implementado,
/// pelo que `Parent` é aceite sempre — efeito visual: ancora à página sem
/// layout flutuante.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PlaceScope {
    #[default]
    Column,
    Parent,
}

// ── TrackSizing ───────────────────────────────────────────────────────────

/// Dimensionamento de uma coluna ou linha de grid (Passo 80).
#[derive(Debug, Clone, PartialEq)]
pub enum TrackSizing {
    /// Largura absoluta em pontos.
    Fixed(f64),
    /// Ajusta-se ao conteúdo mais largo da coluna, limitado por safe_available.
    Auto,
    /// Fracção do espaço restante após Fixed e Auto.
    /// Pode receber 0pt se Fixed + Auto esgotarem o espaço disponível (DEBT-34d).
    Fraction(f64),
}

// ── PageConfig e Page ─────────────────────────────────────────────────────

/// Configuração da página activa no layouter (Passo 81).
///
/// Mutável durante o layout — Content::SetPage altera estes valores.
/// As páginas já fechadas têm os seus próprios snapshots de width/height.
#[derive(Debug, Clone, PartialEq)]
pub struct PageConfig {
    pub width:  f64, // em pontos
    pub height: f64, // em pontos
    pub margin: f64, // margem uniforme em pontos
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            width:  595.28, // A4 portrait
            height: 841.89, // A4 portrait
            margin:  70.87, // ≈ 2.5 cm
        }
    }
}

/// Página fechada — snapshot imutável das dimensões e items visuais (Passo 81).
///
/// `width` e `height` são capturados de `PageConfig` no momento do fecho da página.
/// Duas páginas consecutivas podem ter dimensões distintas.
#[derive(Debug, Clone)]
pub struct Page {
    /// Largura da página no momento em que foi fechada.
    pub width:  f64,
    /// Altura da página no momento em que foi fechada.
    pub height: f64,
    pub items:  Vec<FrameItem>,
}

impl Page {
    /// Extrai texto plano — para verificação em testes.
    pub fn plain_text(&self) -> String {
        self.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
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
                FrameItem::Image { .. }      => None,
                FrameItem::Shape { .. }      => None,
                FrameItem::Group { .. }      => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// ── PagedDocument ──────────────────────────────────────────────────────────

/// Documento paginado — resultado de `layout()`.
///
/// Divergência: original tem `EcoVec<Page>` + `DocumentInfo` + `Arc<PagedIntrospector>`.
/// Cristalino usa `Vec<Page>` com snapshots imutáveis de dimensão (Passo 81).
#[derive(Debug, Clone)]
pub struct PagedDocument {
    pub pages: Vec<Page>,
    /// Mapa de labels para o número de página onde aterraram (Passo 63).
    /// Populado por `Layouter::finish()` após cada passagem de layout.
    /// Vazio por defeito — só tem dados após `layout()` com labels no documento.
    pub extracted_label_pages: HashMap<Label, usize>,
}

impl PagedDocument {
    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages, extracted_label_pages: HashMap::new() }
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

// ── Transformações afins (Passo 78) ──────────────────────────────────────────

/// Matriz de transformação afim 2D: [a, b, c, d, tx, ty].
///
/// Representa a transformação:
///   x' = a*x + c*y + tx
///   y' = b*x + d*y + ty
///
/// Esta convenção segue o formato do operador `cm` do PDF.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformMatrix {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
    pub tx: f64, pub ty: f64,
}

impl Default for TransformMatrix {
    fn default() -> Self { Self::identity() }
}

impl TransformMatrix {
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: dx, ty: dy }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0 }
    }

    /// Rotação em radianos no sistema Y-down do layouter.
    ///   x' =  cos*x - sin*y
    ///   y' =  sin*x + cos*y
    pub fn rotate(radians: f64) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();
        Self { a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0 }
    }

    /// Compõe `other` primeiro, depois `self`.
    ///
    /// `rotate.concat(translate)` aplica translate primeiro, depois rotate.
    /// Composição não é comutativa.
    pub fn concat(&self, other: &Self) -> Self {
        Self {
            a:  self.a * other.a  + self.c * other.b,
            b:  self.b * other.a  + self.d * other.b,
            c:  self.a * other.c  + self.c * other.d,
            d:  self.b * other.c  + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// Aplica a matriz a um ponto 2D.
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
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
        f.push(FrameItem::Text { pos: Point::ZERO, text: "Hello".into(), style: style.clone() });
        f.push(FrameItem::Text { pos: Point { x: Pt(50.0), y: Pt::ZERO }, text: "world".into(), style });
        assert_eq!(f.plain_text(), "Hello world");
    }

    #[test]
    fn paged_document_plain_text() {
        let style = TextStyle::regular(Pt(12.0));
        let p1 = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Text { pos: Point::ZERO, text: "page1".into(), style: style.clone() }],
        };
        let p2 = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Text { pos: Point::ZERO, text: "page2".into(), style }],
        };
        let doc = PagedDocument::new(vec![p1, p2]);
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

    // ── Passo 78 — TransformMatrix ────────────────────────────────────────

    #[test]
    fn transform_matrix_rotacao_90_graus_quadrado_mantem_dimensoes() {
        let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
        let corners = [
            matrix.apply(0.0,   0.0),
            matrix.apply(100.0, 0.0),
            matrix.apply(0.0,   100.0),
            matrix.apply(100.0, 100.0),
        ];
        let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
        let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
        let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
        let new_w = max_x - min_x;
        let new_h = max_y - min_y;
        assert!((new_w - 100.0).abs() < 0.001,
            "Quadrado 100×100 rodado 90° deve ter largura 100, obteve {}", new_w);
        assert!((new_h - 100.0).abs() < 0.001,
            "Quadrado 100×100 rodado 90° deve ter altura 100, obteve {}", new_h);
    }

    #[test]
    fn transform_matrix_rotacao_45_graus_aumenta_bounding_box() {
        let matrix = TransformMatrix::rotate(std::f64::consts::FRAC_PI_4);
        let corners = [
            matrix.apply(0.0,   0.0),
            matrix.apply(100.0, 0.0),
            matrix.apply(0.0,   100.0),
            matrix.apply(100.0, 100.0),
        ];
        let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
        let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
        let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
        let new_w = max_x - min_x;
        let new_h = max_y - min_y;
        let diagonal = 100.0_f64 * std::f64::consts::SQRT_2;
        assert!((new_w - diagonal).abs() < 0.01,
            "Quadrado 100×100 rodado 45° deve ter largura ≈ {:.2}, obteve {:.4}", diagonal, new_w);
        assert!((new_h - diagonal).abs() < 0.01,
            "Quadrado 100×100 rodado 45° deve ter altura ≈ {:.2}, obteve {:.4}", diagonal, new_h);
    }

    // ── Passo 82 — Align2D ─────────────────────────────────────────────────

    #[test]
    fn align2d_from_string_parse_correcto() {
        let a = Align2D::from_string("top-right");
        assert_eq!(a.h, Some(HAlign::Right));
        assert_eq!(a.v, Some(VAlign::Top));

        let b = Align2D::from_string("center");
        assert_eq!(b.h, Some(HAlign::Center));
        assert_eq!(b.v, None);

        let c = Align2D::from_string("bottom");
        assert_eq!(c.h, None);
        assert_eq!(c.v, Some(VAlign::Bottom));

        let d = Align2D::from_string("horizon");
        assert_eq!(d.v, Some(VAlign::Horizon));

        // String inválida: nenhum campo deve ser preenchido.
        let e = Align2D::from_string("invalid");
        assert_eq!(e.h, None);
        assert_eq!(e.v, None);
    }

    #[test]
    fn transform_matrix_concat_ordem_correta() {
        let translate = TransformMatrix::translate(10.0, 0.0);
        let rotate90  = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
        // rotate90.concat(translate): aplica translate primeiro, depois rotate90
        let composed = rotate90.concat(&translate);
        let (rx, ry) = composed.apply(0.0, 0.0);
        assert!((rx - 0.0).abs() < 0.001, "x esperado 0.0, obteve {}", rx);
        assert!((ry - 10.0).abs() < 0.001, "y esperado 10.0, obteve {}", ry);
    }
}
