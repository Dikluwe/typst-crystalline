//! `FrameDTO` — representação neutra de `PagedDocument` para
//! comparação de paridade P3 (layout).
//!
//! Materializado no Passo 150. Conversões:
//! - `from_cristalino(&typst_core::PagedDocument) -> FrameDTO`.
//! - `from_vanilla(...)` — **stub** nesta iteração; integração
//!   vanilla é DEBT-53 (Passo 150 entrega cristalino-only
//!   baseline).
//!
//! Modos de comparação (per `typst-paridade-definicoes.md` §P3):
//! - `text_content` — extracção de texto por página; comparação
//!   exacta.
//! - `structural` — contagem de `FrameItem`s por tipo por página.
//! - `geometric` — posições; **experimental** (não conta para %
//!   agregada porque cristalino usa `FixedMetrics` enquanto
//!   vanilla usa `FontBookMetrics` — divergência estrutural).

use typst_core::entities::layout_types::{FrameItem, PagedDocument};

#[derive(Debug, Clone, PartialEq)]
pub struct FrameDTO {
    pub pages: Vec<PageDTO>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageDTO {
    /// Texto extraído (concatenação de todos os `FrameItem::Text.text`).
    pub text: String,
    /// Tipo de cada item, em ordem (modo structural).
    pub items: Vec<ItemDTO>,
    /// Posições `(x, y)` em pt para cada item, em ordem (modo geometric).
    pub item_positions: Vec<(f64, f64)>,
    /// Largura da página em pt.
    pub width: f64,
    /// Altura da página em pt.
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemDTO {
    Text,
    Group,
    Glyph,
    Line,
    Image,
    Shape,
    /// Catch-all para variantes vanilla-only (ex: itens de
    /// vanilla que cristalino não tem).
    Other(String),
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutTolerance {
    pub text_content: bool,
    pub structural: bool,
    pub geometric_pt: f64,
    /// Marca o modo `geometric` como experimental — o
    /// resultado é registado em números brutos mas **não conta
    /// para a % agregada** do relatório.
    pub geometric_experimental: bool,
}

impl Default for LayoutTolerance {
    fn default() -> Self {
        Self {
            text_content: true,
            structural: true,
            geometric_pt: 5.0,
            geometric_experimental: true,
        }
    }
}

impl FrameDTO {
    /// Conversão a partir do `PagedDocument` cristalino.
    pub fn from_cristalino(doc: &PagedDocument) -> Self {
        let pages = doc.pages.iter().map(|p| {
            let mut text = String::new();
            let mut items = Vec::new();
            let mut item_positions = Vec::new();
            for item in &p.items {
                let (kind, pos) = classify_item(item);
                items.push(kind);
                item_positions.push(pos);
                if let FrameItem::Text { text: t, .. } = item {
                    text.push_str(t.as_str());
                }
            }
            PageDTO { text, items, item_positions, width: p.width, height: p.height }
        }).collect();
        Self { pages }
    }

    /// Conversão a partir do `PagedDocument` vanilla.
    ///
    /// **Stub**: integração vanilla é DEBT-53. Devolve
    /// `FrameDTO` vazio; testes que dependem desta função
    /// recebem fallback.
    pub fn from_vanilla_stub() -> Self {
        Self { pages: Vec::new() }
    }

    /// Compara dois DTOs respeitando os modos da tolerância.
    pub fn compare(&self, other: &Self, t: LayoutTolerance) -> Vec<ModeResult> {
        let mut results = Vec::new();

        if t.text_content {
            let crist_text: Vec<&str> = self.pages.iter().map(|p| p.text.as_str()).collect();
            let other_text: Vec<&str> = other.pages.iter().map(|p| p.text.as_str()).collect();
            let passed = crist_text == other_text;
            let divergent_pages = if passed {
                Vec::new()
            } else {
                crist_text.iter().zip(other_text.iter())
                    .enumerate()
                    .filter_map(|(i, (a, b))| if a != b { Some(i) } else { None })
                    .collect()
            };
            results.push(ModeResult::TextContent { passed, divergent_pages });
        }

        if t.structural {
            let mut mismatches = Vec::new();
            let pages_match = self.pages.len() == other.pages.len();
            if !pages_match {
                mismatches.push(StructuralMismatch::PageCount {
                    cristalino: self.pages.len(),
                    vanilla:    other.pages.len(),
                });
            }
            for (i, (a, b)) in self.pages.iter().zip(other.pages.iter()).enumerate() {
                if a.items != b.items {
                    mismatches.push(StructuralMismatch::PageItems {
                        page: i,
                        cristalino: a.items.clone(),
                        vanilla:    b.items.clone(),
                    });
                }
            }
            results.push(ModeResult::Structural {
                passed: mismatches.is_empty(),
                mismatches,
            });
        }

        // Geometric — experimental, sempre executado se a tolerância o permite.
        let mut max_dx = 0.0_f64;
        let mut max_dy = 0.0_f64;
        let mut sum_dx = 0.0_f64;
        let mut sum_dy = 0.0_f64;
        let mut count = 0usize;
        for (a, b) in self.pages.iter().zip(other.pages.iter()) {
            for (pa, pb) in a.item_positions.iter().zip(b.item_positions.iter()) {
                let dx = (pa.0 - pb.0).abs();
                let dy = (pa.1 - pb.1).abs();
                if dx > max_dx { max_dx = dx; }
                if dy > max_dy { max_dy = dy; }
                sum_dx += dx;
                sum_dy += dy;
                count += 1;
            }
        }
        let (mean_dx, mean_dy) = if count == 0 {
            (0.0, 0.0)
        } else {
            (sum_dx / count as f64, sum_dy / count as f64)
        };
        results.push(ModeResult::Geometric {
            experimental: t.geometric_experimental,
            max_dx, max_dy, mean_dx, mean_dy,
            within_tolerance: count > 0
                && max_dx <= t.geometric_pt
                && max_dy <= t.geometric_pt,
            sample_count: count,
        });

        results
    }
}

fn classify_item(item: &FrameItem) -> (ItemDTO, (f64, f64)) {
    match item {
        FrameItem::Text  { pos, .. } => (ItemDTO::Text,  (pos.x.val(), pos.y.val())),
        FrameItem::Group { pos, .. } => (ItemDTO::Group, (pos.x.val(), pos.y.val())),
        FrameItem::Glyph { pos, .. } => (ItemDTO::Glyph, (pos.x.val(), pos.y.val())),
        FrameItem::Line  { start, .. } => (ItemDTO::Line, (start.x.val(), start.y.val())),
        FrameItem::Image { pos, .. } => (ItemDTO::Image, (pos.x.val(), pos.y.val())),
        FrameItem::Shape { pos, .. } => (ItemDTO::Shape, (pos.x.val(), pos.y.val())),
    }
}

#[derive(Debug)]
pub enum ModeResult {
    TextContent { passed: bool, divergent_pages: Vec<usize> },
    Structural  { passed: bool, mismatches: Vec<StructuralMismatch> },
    Geometric   {
        experimental: bool,
        max_dx:  f64,
        max_dy:  f64,
        mean_dx: f64,
        mean_dy: f64,
        within_tolerance: bool,
        sample_count: usize,
    },
}

#[derive(Debug, Clone)]
pub enum StructuralMismatch {
    PageCount { cristalino: usize, vanilla: usize },
    PageItems { page: usize, cristalino: Vec<ItemDTO>, vanilla: Vec<ItemDTO> },
}
