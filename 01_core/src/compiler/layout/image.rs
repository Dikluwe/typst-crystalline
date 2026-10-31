//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/image.md
//! @prompt-hash 49df88d1
//! @layer L1
//! @updated 2026-09-27

use std::sync::Arc;

use crate::entities::elements::image::ImageElem;
use crate::entities::image_sizer::ImageSizer;
#[cfg(test)]
use crate::entities::layout_types::Length;
use crate::entities::layout_types::{FrameItem, Point, Pt, Rect};
use crate::entities::value::Value;

use super::{FontMetrics, Layouter};

/// **P769** — espaçamento por defeito de um bloco de imagem, equivalente ao
/// `BlockElem::spacing` por defeito do vanilla (`Em::new(1.2)`).
const IMAGE_BLOCK_SPACING_EM: f64 = 1.2;

/// DPI padrão para conversão px → pt (P770/P773).
/// Vanilla usa `Image::DEFAULT_DPI = 72.0`; sem metadados de DPI, 1 px = 1 pt.
const DEFAULT_DPI: f64 = 72.0;

/// Dimensões finais de uma imagem para o layouter, em pontos.
pub struct ImageDimensions {
    /// Dimensões da transformação da imagem (pode exceder o target em cover).
    pub width_pt: f64,
    pub height_pt: f64,
    /// Dimensões do rectângulo target pedido pelo utilizador.
    /// `None` quando apenas um eixo ou nenhum é fornecido.
    pub target_width: Option<f64>,
    pub target_height: Option<f64>,
    /// Dimensões reais em píxeis, lidas do cabeçalho da imagem via sizer.
    /// `None` se o sizer retornou `None` (formato desconhecido — fallback usado).
    /// Retornadas para evitar uma segunda chamada a `sizer.size()` no layouter (DEBT-28).
    pub intrinsic_width: Option<u32>,
    pub intrinsic_height: Option<u32>,
    /// Valor EXIF Orientation (1-8). O exportador PDF aplica a transformação
    /// visual via matriz `cm` (P776); aqui usamos apenas para trocar as
    /// dimensões de layout quando a orientação implica rotação 90°/270°.
    pub orientation: u32,
}

/// Calcula as dimensões finais de uma imagem.
///
/// 1. Lê dimensões intrínsecas em píxeis via `sizer`.
/// 2. Converte para pontos usando DPI real (EXIF/JFIF/pHYs) ou fallback 72.0.
/// 3. Aplica overrides do utilizador e `fit` (cover/contain/stretch),
///    preservando o aspect ratio quando apropriado.
///
/// Se `sizer` não conseguir ler os bytes, usa fallback 100×100 pt.
pub fn calculate_dimensions(
    data: &[u8],
    user_width: Option<&Value>,
    user_height: Option<&Value>,
    fit: &str,
    sizer: &dyn ImageSizer,
) -> ImageDimensions {
    let intrinsic = sizer.size(data); // única leitura do cabeçalho (DEBT-28)
    let orientation = sizer.orientation(data).unwrap_or(1).clamp(1, 8);
    let dpi = sizer.dpi(data).unwrap_or(DEFAULT_DPI);
    let px_to_pt = 72.0 / dpi;

    // P776 — orientações 5-8 implicam rotação 90°/270°; o layout usa dimensões
    // trocadas, como o `new_size` do vanilla em `exif_transform`. As dimensões
    // originais são preservadas para o XObject PDF.
    let (layout_w_px, layout_h_px) = match intrinsic {
        Some((pw, ph)) if (5..=8).contains(&orientation) => (ph, pw),
        Some((pw, ph)) => (pw, ph),
        None => (0, 0),
    };

    let (intrinsic_w_pt, intrinsic_h_pt) = if layout_w_px == 0 && layout_h_px == 0 {
        (100.0, 100.0)
    } else {
        (layout_w_px as f64 * px_to_pt, layout_h_px as f64 * px_to_pt)
    };

    let aspect = if intrinsic_h_pt > 0.0 { intrinsic_w_pt / intrinsic_h_pt } else { 1.0 };

    let req_w = user_width.and_then(extract_pt);
    let req_h = user_height.and_then(extract_pt);

    let (width_pt, height_pt, target_width, target_height) = match (req_w, req_h) {
        (Some(w), Some(h)) => {
            let (img_w, img_h) = apply_fit(w, h, aspect, fit);
            (img_w, img_h, Some(w), Some(h))
        }
        (Some(w), None) => (w, w / aspect, None, None),
        (None, Some(h)) => (h * aspect, h, None, None),
        (None, None) => (intrinsic_w_pt, intrinsic_h_pt, None, None),
    };

    ImageDimensions {
        width_pt,
        height_pt,
        target_width,
        target_height,
        intrinsic_width: intrinsic.map(|(w, _)| w),
        intrinsic_height: intrinsic.map(|(_, h)| h),
        orientation,
    }
}

/// Aplica `fit` a um rectângulo target de dimensões explicitamente fornecidas.
///
/// - `"stretch"` → usa `(target_w, target_h)` exactamente.
/// - `"cover"` / `"contain"` → ajusta para preencher o target preservando
///   o aspect ratio, replicando `typst-layout/src/image.rs`.
fn apply_fit(target_w: f64, target_h: f64, aspect: f64, fit: &str) -> (f64, f64) {
    if fit == "stretch" {
        return (target_w, target_h);
    }

    let target_aspect = if target_h > 0.0 { target_w / target_h } else { aspect };
    let wide = aspect > target_aspect;

    // Para "cover", preenche o target: a dimensão dominante é a que evita
    // bordas vazias. Para "contain", encaixa dentro: a dimensão dominante é
    // a que evita cortar. A condição `wide == (fit == "contain")` cobre
    // ambos os casos de forma simétrica.
    if wide == (fit == "contain") {
        (target_w, target_w / aspect)
    } else {
        (target_h * aspect, target_h)
    }
}

fn extract_pt(val: &Value) -> Option<f64> {
    match val {
        Value::Float(f) => Some(*f),
        Value::Length(l) => Some(l.abs.to_pt()),
        // No axis in this bounded projection; source validation is caller-owned.
        Value::None
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Str(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => None,
    }
}

fn block_margin<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
    edge: &str,
) -> (f64, bool) {
    let font = layouter.style.size.val();
    for key in [edge, "block.spacing"] {
        if let Some(Value::Length(length)) = layouter.chain.custom(key) {
            return (length.resolve_pt(font), true);
        }
        if layouter.chain.custom(key) == Some(&Value::Auto) {
            break;
        }
    }
    if let Some(Value::Length(length)) = layouter.chain.custom("par.spacing") {
        return (length.resolve_pt(font), false);
    }
    (font * IMAGE_BLOCK_SPACING_EM, false)
}

/// Layout de `image(...)` (atomização ADR-0109 P378): resolve dimensões via
/// `calculate_dimensions`, garante linha/página, emite o `FrameItem::Image` e
/// avança o cursor vertical. Content-preserving — era inline no `layout_content`.
///
/// **P769** — `Content::Image` comporta-se como bloco no fluxo principal,
/// replicando `BlockElem::single_layouter` do vanilla. Quando sucede texto
/// não-bloco, ancora a base da imagem em `baseline + above` e estende-a para
/// cima; caso contrário mantém o modelo de bloco (`base = cursor_y − cap_height`).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &ImageElem,
) {
    super::h_space::close_paragraph(layouter);
    // A fresh block starts at a geometric top, including root Place.
    // Legacy baseline-origin subframes arrive with the pending flag cleared.
    let fresh_block = layouter.initial_baseline_pending
        && layouter
            .regions
            .current
            .current_line
            .iter()
            .all(super::html_frame::is_context_marker);
    if fresh_block {
        layouter.initial_baseline_pending = false;
    } else if layouter.initial_baseline_pending {
        let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        layouter.regions.current.cursor_y += top;
        layouter.initial_baseline_pending = false;
    }
    let dims = calculate_dimensions(
        &e.data.0, // &[u8] via PtrEqArc → Arc → deref
        e.width.as_deref(),
        e.height.as_deref(),
        &e.fit,
        &layouter.sizer,
    );

    // P771 — altura que conta para overflow de página e avanço de cursor:
    // a do target quando ambos os eixos são fornecidos; caso contrário, a da
    // transformação da imagem.
    let used_height = dims.target_height.unwrap_or(dims.height_pt);

    // **P769** — guardar a baseline da linha que vai ser descarregada.
    // Após `flush_line`, o cursor aponta para a baseline da *próxima* linha;
    // para ancorar a imagem na grelha de linhas do parágrafo actual, usamos
    // a baseline *antes* do avanço.
    let baseline_before_flush = layouter.regions.current.cursor_y;
    layouter.flush_line();
    let cursor_after_flush = layouter.regions.current.cursor_y;
    let had_text_line = cursor_after_flush.0 > baseline_before_flush.0 + 1e-6;

    // **P769** — protocolo de bloco no fluxo principal.
    let in_main_flow = !layouter.is_sub_frame;
    let (above_pt, above_explicit) = block_margin(layouter, "block.above");
    let (below_pt, below_explicit) = block_margin(layouter, "block.below");
    let cap_height =
        if fresh_block || !in_main_flow || layouter.block_chain_active || had_text_line {
            Pt(0.0)
        } else {
            layouter.metrics.cap_height(layouter.style.size, &layouter.style)
        };

    if in_main_flow {
        // A imagem de bloco alinha-se à margem esquerda do contentor.
        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    }

    // The previous frame publishes its physical end; below is still pending.
    let candidate_base = if !in_main_flow {
        layouter.regions.current.cursor_y
    } else if had_text_line {
        baseline_before_flush + Pt(above_pt)
    } else if layouter.block_chain_active {
        let gap = if above_explicit && layouter.prev_margin_is_parbreak {
            above_pt
        } else if !above_explicit && !layouter.prev_margin_is_parbreak {
            layouter.prev_block_below_pending
        } else {
            layouter.prev_block_below_pending.max(above_pt)
        };
        Pt(layouter.prev_line_baseline + layouter.prev_block_equation_descent + gap)
    } else {
        layouter.regions.current.cursor_y - cap_height
    };
    let page_bottom =
        layouter.regions.current.height - layouter.page_config.margin.bottom;
    let image_base = if in_main_flow && candidate_base.0 + used_height > page_bottom {
        layouter.new_page();
        let top = if layouter.initial_baseline_pending {
            Pt(0.0)
        } else {
            layouter.metrics.text_edges(layouter.style.size, &layouter.style).0
        };
        layouter.regions.current.cursor_y - top
    } else {
        candidate_base
    };

    // Centralização da transformação da imagem dentro do target (P771).
    let (target_w, target_h, clip_rect) = match (dims.target_width, dims.target_height) {
        (Some(tw), Some(th)) => {
            let clip = if e.fit == "cover" {
                Some(Rect {
                    x: layouter.regions.current.cursor_x,
                    y: image_base,
                    w: Pt(tw),
                    h: Pt(th),
                })
            } else {
                None
            };
            (tw, th, clip)
        }
        _ => (dims.width_pt, dims.height_pt, None),
    };

    layouter.record_flow_width_at(layouter.regions.current.cursor_x.0, target_w);
    let pos = Point {
        // rationale: P1064 Classe 1A — centragem de imagem em frame ((target_w - dims.w) / 2.0)
        x: Pt(layouter.regions.current.cursor_x.0 + (target_w - dims.width_pt) / 2.0),
        // rationale: P1064 Classe 1A — centragem vertical de imagem em frame ((target_h - dims.h) / 2.0)
        y: Pt(image_base.0 + (target_h - dims.height_pt) / 2.0),
    };

    // DEBT-28 encerrado: intrinsic_width/height vêm de calculate_dimensions.
    let intrinsic_w = dims.intrinsic_width.unwrap_or(100);
    let intrinsic_h = dims.intrinsic_height.unwrap_or(100);

    layouter.regions.current.current_items.push(FrameItem::Image {
        pos,
        alt: e.alt.clone(),
        data: Arc::clone(&e.data.0), // .0 acede ao Arc interno de PtrEqArc
        width: Pt(dims.width_pt),
        height: Pt(dims.height_pt),
        intrinsic_width: intrinsic_w,
        intrinsic_height: intrinsic_h,
        clip_rect,
        orientation: dims.orientation,
    });

    // Publish the settled target end, excluding the prepared lower margin.
    let end = image_base.0 + used_height;
    if in_main_flow {
        layouter.regions.current.cursor_y = Pt(end + below_pt);
        layouter.prev_line_baseline = end;
        layouter.last_block_descent_y = Some(end);
        layouter.prev_block_equation_descent = 0.0;
        layouter.prev_block_below_pending = below_pt;
        layouter.prev_margin_is_parbreak = !below_explicit;
        layouter.block_chain_active = true;
    } else {
        layouter.regions.current.cursor_y += Pt(used_height);
    }
    layouter.record_flow_baseline(None);
    layouter.record_flow_placement_end(end);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::image_sizer::NullImageSizer;

    #[test]
    fn dimensoes_fallback_quando_sizer_retorna_none() {
        let dims = calculate_dimensions(&[], None, None, "cover", &NullImageSizer);
        assert_eq!(dims.width_pt, 100.0);
        assert_eq!(dims.height_pt, 100.0);
    }

    #[test]
    fn dimensoes_intrinsecas_sem_overrides() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // P770 — 72 DPI padrão: 400 px = 400 pt; 300 px = 300 pt.
        let dims = calculate_dimensions(&[], None, None, "cover", &MockSizer);
        assert_eq!(dims.width_pt, 400.0);
        assert_eq!(dims.height_pt, 300.0);
    }

    #[test]
    fn override_width_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // Forçar width = 120pt → height = 120 / (4/3) = 90pt
        let w = Value::Float(120.0);
        let dims = calculate_dimensions(&[], Some(&w), None, "cover", &MockSizer);
        assert_eq!(dims.width_pt, 120.0);
        assert_eq!(dims.height_pt, 90.0);
        // P771 — só um eixo fornecido: não há target nem clip.
        assert_eq!(dims.target_width, None);
        assert_eq!(dims.target_height, None);
    }

    #[test]
    fn override_height_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // Forçar height = 90pt → width = 90 * (4/3) = 120pt
        let h = Value::Float(90.0);
        let dims = calculate_dimensions(&[], None, Some(&h), "cover", &MockSizer);
        assert_eq!(dims.width_pt, 120.0);
        assert_eq!(dims.height_pt, 90.0);
    }

    #[test]
    fn ambos_overrides_stretch_forca_dimensoes() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        let w = Value::Float(50.0);
        let h = Value::Float(50.0);
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), "stretch", &MockSizer);
        assert_eq!(dims.width_pt, 50.0);
        assert_eq!(dims.height_pt, 50.0);
        // P771 — target é preservado para avanço de cursor e clip.
        assert_eq!(dims.target_width, Some(50.0));
        assert_eq!(dims.target_height, Some(50.0));
    }

    #[test]
    fn ambos_overrides_cover_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // Imagem 4:3, target 50×50 (aspecto 1). Cover → preenche o target:
        // wide (4/3 > 1) → height = 50, width = 50 * (4/3) = 66.666...
        let w = Value::Float(50.0);
        let h = Value::Float(50.0);
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), "cover", &MockSizer);
        assert!((dims.width_pt - 200.0 / 3.0).abs() < 1e-9);
        assert_eq!(dims.height_pt, 50.0);
        assert_eq!(dims.target_width, Some(50.0));
        assert_eq!(dims.target_height, Some(50.0));
    }

    #[test]
    fn ambos_overrides_contain_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // Imagem 4:3, target 50×50. Contain → encaixa dentro do target:
        // wide (4/3 > 1) → width = 50, height = 50 / (4/3) = 37.5.
        let w = Value::Float(50.0);
        let h = Value::Float(50.0);
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), "contain", &MockSizer);
        assert_eq!(dims.width_pt, 50.0);
        assert_eq!(dims.height_pt, 37.5);
        assert_eq!(dims.target_width, Some(50.0));
        assert_eq!(dims.target_height, Some(50.0));
    }

    #[test]
    fn fit_padrao_cover_quando_ambos_fornecidos() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((100, 80))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                None
            }
        }
        // Replicação exacta do caso P770: tiny.png 100×80 (aspect 1.25),
        // target 2cm × 1.5cm (≈ 56.693 × 42.520 pt, aspect 1.333).
        // Cover → wide=false (1.25 < 1.333) → width = target_w,
        // height = target_w / 1.25.
        let w = Value::Length(crate::entities::layout_types::Length::cm(2.0));
        let h = Value::Length(crate::entities::layout_types::Length::cm(1.5));
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), "cover", &MockSizer);
        assert!((dims.width_pt - 56.6929).abs() < 0.001);
        assert!((dims.height_pt - 45.3543).abs() < 0.001);
        assert!((dims.target_width.unwrap() - 56.6929).abs() < 0.001);
        assert!((dims.target_height.unwrap() - 42.5197).abs() < 0.001);
    }

    #[test]
    fn calculate_dimensions_retorna_intrinsic() {
        // NullImageSizer retorna None — campos intrinsic devem ser None.
        let dims = calculate_dimensions(
            &[0xFF, 0xD8, 0xFF, 0x00],
            None,
            None,
            "cover",
            &NullImageSizer,
        );
        assert_eq!(dims.intrinsic_width, None);
        assert_eq!(dims.intrinsic_height, None);

        // Sizer com dimensões reais — campos devem ser preenchidos.
        struct FixedSizer;
        impl ImageSizer for FixedSizer {
            fn size(&self, _data: &[u8]) -> Option<(u32, u32)> {
                Some((800, 600))
            }
            fn dpi(&self, _data: &[u8]) -> Option<f64> {
                None
            }
            fn orientation(&self, _data: &[u8]) -> Option<u32> {
                None
            }
        }
        let dims2 = calculate_dimensions(&[], None, None, "cover", &FixedSizer);
        assert_eq!(dims2.intrinsic_width, Some(800));
        assert_eq!(dims2.intrinsic_height, Some(600));
    }
}

#[cfg(test)]
mod initial_nontext_block_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::layout_types::{TextEdge, TextStyle};
    use comemo::Track;

    struct NoTypography;
    impl FontMetrics for NoTypography {
        fn advance(&self, _: &str, _: Pt, _: &TextStyle) -> Pt {
            panic!("unexpected text advance")
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("unexpected vertical metrics")
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            panic!("unexpected cap height")
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("unexpected text edges")
        }
    }

    fn check<M: FontMetrics>(metrics: M, edge: Option<TextEdge>, subframe: bool) {
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
        let mut layouter = Layouter::new(metrics, NullImageSizer, 12.0, intr_dyn.track());
        layouter.style.top_edge = edge;
        layouter.is_sub_frame = subframe;
        layouter.initial_baseline_pending = !subframe;
        let origin = layouter.regions.current.cursor_y.0;
        let elem = crate::entities::elements::image::ImageElem {
            path: String::new(),
            alt: None,
            data: crate::entities::ptr_eq_arc::PtrEqArc(Arc::new(vec![])),
            width: Some(Box::new(Value::Float(80.0))),
            height: Some(Box::new(Value::Float(40.0))),
            fit: "cover".into(),
        };
        super::layout(&mut layouter, &elem);
        let y = match &layouter.regions.current.current_items[0] {
            FrameItem::Image { clip_rect: Some(rect), .. } => rect.y.0,
            other => panic!("unexpected first block {other:?}"),
        };
        assert!((y - origin).abs() < 1e-9, "block top {y} != physical origin {origin}");
        assert!(!layouter.initial_baseline_pending);
        assert_eq!(layouter.block_chain_active, !subframe);
    }

    #[test]
    fn initial_nontext_block_top_is_independent_of_text_edge() {
        for edge in [
            None,
            Some(TextEdge::Length(Length::pt(0.0))),
            Some(TextEdge::Length(Length::pt(20.0))),
        ] {
            check(FixedMetrics, edge, false);
        }
    }

    #[test]
    fn initial_nontext_block_does_not_acquire_typography() {
        check(NoTypography, None, false);
    }
    #[test]
    fn ordinary_subframe_block_does_not_query_unused_typography() {
        check(NoTypography, None, true);
    }
}

#[cfg(test)]
mod v16_d_tests {
    use super::*;
    use crate::entities::image_sizer::NullImageSizer;

    #[test]
    fn v16_d_image_axis_schema_keeps_intrinsic_and_fit_controls() {
        for invalid in [
            Value::Auto,
            Value::None,
            Value::Bool(true),
            Value::Int(30),
            Value::Str("30pt".into()),
            Value::Array(vec![]),
            Value::Content(crate::entities::content::Content::Empty),
        ] {
            assert_eq!(extract_pt(&invalid), None);
            let dims = calculate_dimensions(
                &[],
                Some(&invalid),
                Some(&Value::Length(Length::pt(20.0))),
                "cover",
                &NullImageSizer,
            );
            assert_eq!((dims.width_pt, dims.height_pt), (20.0, 20.0));
            assert!(dims.target_width.is_none());
        }
        assert_eq!(extract_pt(&Value::Float(30.0)), Some(30.0));
        assert_eq!(extract_pt(&Value::Length(Length::pt(30.0))), Some(30.0));
        let dims = calculate_dimensions(
            &[],
            Some(&Value::Length(Length::pt(40.0))),
            Some(&Value::Length(Length::pt(20.0))),
            "cover",
            &NullImageSizer,
        );
        assert_eq!((dims.target_width, dims.target_height), (Some(40.0), Some(20.0)));
        assert_eq!((dims.width_pt, dims.height_pt), (40.0, 40.0));
    }
}

#[cfg(test)]
mod image_flow_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        content::Content,
        elements::h_space::{HSpaceElem, Spacing},
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins, PagedDocument},
    };
    use comemo::Track;

    pub(super) fn make(
        intr: &dyn Introspector,
        margin: f64,
        height: f64,
    ) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 11.0, intr.track());
        l.page_config.width = 400.0;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(margin);
        l.page_config.margin_spec = PageMarginSpec::uniform(margin);
        l.regions.current.width = 400.0;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(margin);
        l.regions.current.line_start_x = Pt(margin);
        l.regions.current.cursor_y = Pt(margin);
        l
    }
    pub(super) fn photo(height: f64) -> Content {
        Content::Image(Arc::new(ImageElem {
            path: String::new(),
            alt: None,
            data: crate::entities::ptr_eq_arc::PtrEqArc(Arc::new(vec![])),
            width: Some(Box::new(Value::Float(30.0))),
            height: Some(Box::new(Value::Float(height))),
            fit: "stretch".into(),
        }))
    }
    pub(super) fn horizontal(amount: f64) -> Content {
        Content::HSpace(Arc::new(HSpaceElem {
            amount: Spacing::Absolute(Length::pt(amount)),
            weak: false,
            weak_explicit: false,
        }))
    }
    pub(super) fn positions(doc: &PagedDocument) -> Vec<Vec<f64>> {
        doc.pages
            .iter()
            .map(|page| {
                page.items
                    .iter()
                    .filter_map(|item| {
                        if let FrameItem::Image { pos, .. } = item {
                            Some(pos.y.0)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }
    pub(super) fn assert_ys(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 0.001, "image top {a} != {b}");
        }
    }
    #[test]
    fn image_flow_repeated_blocks_publish_physical_progress() {
        let intr = TagIntrospector::empty();
        for (margin, h) in [(5.0, 20.0), (30.0, 20.0), (5.0, 40.0)] {
            let mut l = make(&intr, margin, 400.0);
            for _ in 0..3 {
                l.layout_content(&photo(h));
            }
            let doc = l.finish();
            assert_eq!(doc.pages.len(), 1);
            assert_ys(
                &positions(&doc)[0],
                &[margin, margin + h + 13.2, margin + 2.0 * (h + 13.2)],
            );
        }
    }
    #[test]
    fn image_flow_horizontal_only_paragraph_keeps_zero_presence() {
        let intr = TagIntrospector::empty();
        for amount in [0.0, 1.0, -1.0] {
            let mut l = make(&intr, 5.0, 400.0);
            for _ in 0..3 {
                l.layout_content(&photo(20.0));
                l.layout_content(&horizontal(amount));
            }
            let doc = l.finish();
            assert_ys(&positions(&doc)[0], &[5.0, 51.4, 97.8]);
            assert_eq!(doc.plain_text(), "");
        }
    }
    #[test]
    fn image_flow_sixteen_horizontal_paragraphs_break_eight_per_page() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        for _ in 0..16 {
            l.layout_content(&photo(20.0));
            l.layout_content(&horizontal(1.0));
        }
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 2);
        for page in positions(&doc) {
            let expected: Vec<_> = (0..8).map(|i| 5.0 + f64::from(i) * 46.4).collect();
            assert_ys(&page, &expected);
        }
    }
    #[test]
    fn image_flow_exact_fit_does_not_page_for_pending_margin() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 30.0);
        l.layout_content(&photo(20.0));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        assert_ys(&positions(&doc)[0], &[5.0]);
    }
    #[test]
    fn image_flow_consecutive_horizontal_producers_are_one_paragraph() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        l.layout_content(&photo(20.0));
        for _ in 0..3 {
            l.layout_content(&horizontal(1.0));
        }
        l.layout_content(&photo(20.0));
        assert_ys(&positions(&l.finish())[0], &[5.0, 51.4]);
    }
    #[test]
    fn image_flow_pending_horizontal_paragraph_survives_rollback_once() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        l.layout_content(&photo(20.0));
        l.layout_content(&horizontal(1.0));
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.flush_line();
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.layout_content(&photo(20.0));
        assert_ys(&positions(&l.finish())[0], &[5.0, 51.4]);
    }
    #[test]
    fn image_flow_horizontal_custom_paragraph_spacing_is_used_once() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(30.0)));
        l.layout_content(&photo(20.0));
        l.layout_content(&horizontal(1.0));
        l.layout_content(&photo(20.0));
        assert_ys(&positions(&l.finish())[0], &[5.0, 85.0]);
    }
}

#[cfg(test)]
mod image_flow_isolation_controls {
    use super::image_flow_regressions::{assert_ys, horizontal, make, photo, positions};
    use crate::compiler::layout::sub_frame::SubLayoutRegion;
    use crate::entities::{content::Content, introspector::TagIntrospector};

    #[test]
    fn image_flow_horizontal_cause_isolated_from_both_subframe_paths() {
        let intr = TagIntrospector::empty();
        for inline in [false, true] {
            let mut l = make(&intr, 5.0, 400.0);
            l.layout_content(&photo(20.0));
            l.layout_content(&horizontal(1.0));
            // Inline capture may legitimately prepare the parent's text baseline.
            // A horizontal-only child tests pending-cause isolation independently.
            let body = horizontal(2.0);
            let chain = l.chain.clone();
            l.chain = l.chain.push_custom(
                "par.spacing",
                crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(30.0),
                ),
            );
            let region = SubLayoutRegion {
                origin_x: 0.0,
                width: if inline { 0.0 } else { 100.0 },
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            };
            if inline {
                l.layout_sub_frame_inline_planned(&body, region, None);
            } else {
                l.layout_sub_frame(&body, region);
            }
            l.chain = chain;
            l.layout_content(&photo(20.0));
            assert_ys(&positions(&l.finish())[0], &[5.0, 51.4]);
        }
    }
    #[test]
    fn image_flow_real_text_absorbs_horizontal_paragraph_cause() {
        let intr = TagIntrospector::empty();
        let mut images = Vec::new();
        for with_h in [false, true] {
            let mut l = make(&intr, 5.0, 400.0);
            l.layout_content(&photo(20.0));
            if with_h {
                l.layout_content(&horizontal(1.0));
            }
            l.layout_content(&Content::text("X"));
            l.layout_content(&photo(20.0));
            let doc = l.finish();
            assert_eq!(doc.plain_text(), "X");
            images.push(positions(&doc));
        }
        assert_eq!(images[0], images[1]);
    }
    #[test]
    fn image_flow_block_margin_zero_auto_and_precedence() {
        use crate::entities::{layout_types::Length, value::Value};
        let intr = TagIntrospector::empty();
        for (par, block, above, below, with_h, expected) in [
            (Some(0.0), None, None, None, false, [5.0, 25.0, 45.0]),
            (Some(30.0), Some(0.0), None, None, true, [5.0, 25.0, 45.0]),
            (None, None, Some(8.0), Some(3.0), false, [5.0, 33.0, 61.0]),
            (None, None, Some(8.0), Some(3.0), true, [5.0, 36.0, 67.0]),
        ] {
            let mut l = make(&intr, 5.0, 400.0);
            for (key, value) in [
                ("par.spacing", par),
                ("block.spacing", block),
                ("block.above", above),
                ("block.below", below),
            ] {
                if let Some(v) = value {
                    l.chain = l.chain.push_custom(key, Value::Length(Length::pt(v)));
                }
            }
            for _ in 0..3 {
                l.layout_content(&photo(20.0));
                if with_h {
                    l.layout_content(&horizontal(1.0));
                }
            }
            assert_ys(&positions(&l.finish())[0], &expected);
        }
    }
    #[test]
    fn image_flow_horizontal_parbreak_does_not_duplicate_boundary() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        l.layout_content(&photo(20.0));
        l.layout_content(&horizontal(0.0));
        l.layout_content(&Content::Parbreak);
        l.layout_content(&photo(20.0));
        assert_ys(&positions(&l.finish())[0], &[5.0, 51.4]);
    }
}

#[cfg(test)]
mod image_flow_lexical_controls {
    use super::image_flow_regressions::{assert_ys, horizontal, make, photo, positions};
    use crate::entities::{
        introspector::TagIntrospector, layout_types::Length, style_chain::StyleDelta,
    };

    #[test]
    fn image_flow_horizontal_paragraph_uses_common_lexical_scope() {
        let intr = TagIntrospector::empty();
        for mode in 0..4 {
            let mut l = make(&intr, 5.0, 400.0);
            let outer = l.chain.clone();
            for _ in 0..3 {
                l.layout_content(&photo(20.0));
                let styled = outer.push(StyleDelta {
                    size: Some(Length::pt(20.0)),
                    ..StyleDelta::empty()
                });
                l.chain = styled.clone();
                l.layout_content(&horizontal(1.0));
                if mode > 0 {
                    l.chain = match mode {
                        1 => outer.clone(),
                        2 => outer.push(StyleDelta {
                            size: Some(Length::pt(20.0)),
                            ..StyleDelta::empty()
                        }),
                        _ => styled,
                    };
                    l.layout_content(&horizontal(1.0));
                }
                l.chain = outer.clone();
            }
            let expected = if mode == 1 || mode == 2 {
                [5.0, 51.4, 97.8]
            } else {
                [5.0, 73.0, 141.0]
            };
            assert_ys(&positions(&l.finish())[0], &expected);
        }
    }
}

#[cfg(test)]
mod image_flow_terminal_controls {
    use super::image_flow_regressions::{assert_ys, horizontal, make, photo, positions};
    use crate::entities::introspector::TagIntrospector;

    #[test]
    fn image_flow_terminal_horizontal_paragraph_keeps_required_successor() {
        let intr = TagIntrospector::empty();
        for height in [370.0, 380.0, 385.0, 390.0] {
            for with_h in [false, true] {
                let mut l = make(&intr, 5.0, 400.0);
                l.layout_content(&photo(height));
                if with_h {
                    l.layout_content(&horizontal(1.0));
                }
                let doc = l.finish();
                let expected = if with_h && height > 370.0 { 2 } else { 1 };
                assert_eq!(doc.pages.len(), expected, "height={height}, h={with_h}");
                assert_eq!(doc.plain_text(), if expected == 2 { "\n" } else { "" });
                assert_ys(&positions(&doc)[0], &[5.0]);
                if expected == 2 {
                    assert!(positions(&doc)[1].is_empty());
                }
            }
        }
    }

    #[test]
    fn image_flow_moved_horizontal_paragraph_keeps_gap_before_next_image() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, 5.0, 400.0);
        l.layout_content(&photo(380.0));
        l.layout_content(&horizontal(1.0));
        l.layout_content(&photo(20.0));
        let ys = positions(&l.finish());
        assert_eq!(ys.len(), 2);
        assert_ys(&ys[0], &[5.0]);
        assert_ys(&ys[1], &[18.2]);
    }
}
