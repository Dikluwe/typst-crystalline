//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout-image.md
//! @prompt-hash 41cbb718
//! @layer L1
//! @updated 2026-07-15

use std::sync::Arc;

use crate::entities::elements::image::ImageElem;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt, Rect};
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
        _ => None, // neutro: N16[β] — Value não-dimensionável retorna None na extracção de pontos (projeção de tipo)
    }
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
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar a primeira imagem real.
    layouter.ensure_initial_baseline();
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
    let font = layouter.style.size.val();
    let above_pt = Length::em(IMAGE_BLOCK_SPACING_EM).resolve_pt(font);
    let below_pt = Length::em(IMAGE_BLOCK_SPACING_EM).resolve_pt(font);
    let cap_height = layouter.metrics.cap_height(layouter.style.size, &layouter.style);

    if in_main_flow {
        // Colapso de margem com o bloco anterior (P250):
        // `max(prev.below, curr.above)`; o primeiro bloco de uma Sequence
        // não leva above (`block_chain_active == false`).
        let gap = if layouter.block_chain_active {
            layouter.prev_block_below_pending.max(above_pt)
        } else {
            0.0
        };
        let advance = (gap - layouter.prev_block_below_pending).max(0.0);
        layouter.regions.current.cursor_y += Pt(advance);
        layouter.prev_block_below_pending = 0.0;

        // A imagem de bloco alinha-se à margem esquerda do contentor.
        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    }

    // Verificar se a imagem cabe na página actual.
    if layouter.regions.current.cursor_y.0 + used_height
        > layouter.regions.current.height - layouter.page_config.margin.bottom
    {
        layouter.new_page();
    }

    // P748/P750 — no fluxo principal o cursor_y representa a baseline do
    // texto. Quando existe target (ambos width e height fornecidos), o
    // ancoramento e o avanço do cursor usam o rectângulo target; a imagem
    // transformada é centralizada dentro desse target. Quando não existe
    // target, usam-se as dimensões da transformação.
    //
    // **P769** — quando a imagem é a primeira depois de texto não-bloco,
    // o vanilla ancora a *base do target* em `baseline + above`; o topo do
    // target fica em `baseline + above + target_height`. Quando a imagem
    // sucede outro bloco, ou quando é a primeira de uma Sequence sem texto
    // antes, mantém-se o modelo de bloco: base do target em `cursor_y − cap_height`.
    let image_base = if !in_main_flow {
        layouter.regions.current.cursor_y
    } else if layouter.block_chain_active {
        layouter.regions.current.cursor_y - cap_height
    } else if had_text_line {
        baseline_before_flush + Pt(above_pt)
    } else {
        layouter.regions.current.cursor_y - cap_height
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
        data: Arc::clone(&e.data.0), // .0 acede ao Arc interno de PtrEqArc
        width: Pt(dims.width_pt),
        height: Pt(dims.height_pt),
        intrinsic_width: intrinsic_w,
        intrinsic_height: intrinsic_h,
        clip_rect,
        orientation: dims.orientation,
    });

    // **P769/P771** — avanço do cursor conforme o tipo de ancoragem, usando
    // a altura do target quando existe. Após imagem-a-seguir-a-texto, a
    // próxima baseline fica no topo do target + `below + cap_height`; após
    // imagem-a-seguir-a-bloco, ou imagem isolada, mantém-se o comportamento
    // de bloco (base do target + `below`).
    if in_main_flow {
        if layouter.block_chain_active {
            layouter.regions.current.cursor_y = image_base + Pt(used_height + below_pt);
        } else if had_text_line {
            layouter.regions.current.cursor_y =
                image_base + Pt(used_height + below_pt + cap_height.0);
        } else {
            layouter.regions.current.cursor_y = image_base + Pt(used_height + below_pt);
        }
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    } else {
        layouter.regions.current.cursor_y += Pt(used_height);
    }

    if layouter.regions.current.cursor_y.0
        > layouter.regions.current.height - layouter.page_config.margin.bottom
    {
        layouter.new_page();
    }
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
