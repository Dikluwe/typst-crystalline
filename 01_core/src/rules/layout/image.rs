//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout-image.md
//! @prompt-hash ab5f39c1
//! @layer L1
//! @updated 2026-07-15

use std::sync::Arc;

use crate::entities::elements::image::ImageElem;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt};
use crate::entities::value::Value;

use super::{FontMetrics, Layouter};

/// **P769** — espaçamento por defeito de um bloco de imagem, equivalente ao
/// `BlockElem::spacing` por defeito do vanilla (`Em::new(1.2)`).
const IMAGE_BLOCK_SPACING_EM: f64 = 1.2;

/// Densidade padrão para conversão px → pt.
/// 96 DPI: 1 pt = 1/72 inch; 1 px = 1/96 inch → 1 px = 72/96 pt = 0.75 pt.
const PX_TO_PT: f64 = 0.75;

/// Dimensões finais de uma imagem para o layouter, em pontos.
pub struct ImageDimensions {
    pub width_pt:         f64,
    pub height_pt:        f64,
    /// Dimensões reais em píxeis, lidas do cabeçalho da imagem via sizer.
    /// `None` se o sizer retornou `None` (formato desconhecido — fallback usado).
    /// Retornadas para evitar uma segunda chamada a `sizer.size()` no layouter (DEBT-28).
    pub intrinsic_width:  Option<u32>,
    pub intrinsic_height: Option<u32>,
}

/// Calcula as dimensões finais de uma imagem.
///
/// 1. Lê dimensões intrínsecas em píxeis via `sizer`.
/// 2. Converte para pontos (96 DPI).
/// 3. Aplica overrides do utilizador preservando o aspect ratio se apenas
///    um dos valores for fornecido.
///
/// Se `sizer` não conseguir ler os bytes, usa fallback 100×100 pt.
pub fn calculate_dimensions(
    data:        &[u8],
    user_width:  Option<&Value>,
    user_height: Option<&Value>,
    sizer:       &dyn ImageSizer,
) -> ImageDimensions {
    let intrinsic = sizer.size(data); // única leitura do cabeçalho (DEBT-28)

    let (intrinsic_w_pt, intrinsic_h_pt) = match intrinsic {
        Some((pw, ph)) => (pw as f64 * PX_TO_PT, ph as f64 * PX_TO_PT),
        None           => (100.0, 100.0),
    };

    let aspect = if intrinsic_h_pt > 0.0 {
        intrinsic_w_pt / intrinsic_h_pt
    } else {
        1.0
    };

    let req_w = user_width.and_then(extract_pt);
    let req_h = user_height.and_then(extract_pt);

    let (width_pt, height_pt) = match (req_w, req_h) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None)    => (w, w / aspect),
        (None, Some(h))    => (h * aspect, h),
        (None, None)       => (intrinsic_w_pt, intrinsic_h_pt),
    };

    ImageDimensions {
        width_pt,
        height_pt,
        intrinsic_width:  intrinsic.map(|(w, _)| w),
        intrinsic_height: intrinsic.map(|(_, h)| h),
    }
}

fn extract_pt(val: &Value) -> Option<f64> {
    match val {
        Value::Float(f)  => Some(*f),
        Value::Length(l) => Some(l.abs.to_pt()),
        _ => None,
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
    e:        &ImageElem,
) {
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar a primeira imagem real.
    layouter.ensure_initial_baseline();
    let dims = calculate_dimensions(
        &e.data.0,  // &[u8] via PtrEqArc → Arc → deref
        e.width.as_deref(),
        e.height.as_deref(),
        &layouter.sizer,
    );

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
    if layouter.regions.current.cursor_y.0 + dims.height_pt > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }

    // P748/P750 — no fluxo principal o cursor_y representa a baseline do
    // texto. O `pos.y` de `FrameItem::Image` é a base da imagem em
    // coordenadas do Layouter (Y crescente para cima a partir da base da
    // página); a imagem estende-se para cima pela sua altura.
    //
    // **P769** — quando a imagem é a primeira depois de texto não-bloco,
    // o vanilla ancora a *base* da imagem em `baseline + above`; o topo da
    // imagem fica em `baseline + above + height`. Quando a imagem sucede
    // outro bloco, ou quando é a primeira de uma Sequence sem texto antes,
    // mantém-se o modelo de bloco: base da imagem em `cursor_y − cap_height`
    // (topo da linha anterior, estendendo-se para cima).
    let image_base = if !in_main_flow {
        layouter.regions.current.cursor_y
    } else if layouter.block_chain_active {
        layouter.regions.current.cursor_y - cap_height
    } else if had_text_line {
        baseline_before_flush + Pt(above_pt)
    } else {
        layouter.regions.current.cursor_y - cap_height
    };
    let pos = Point { x: layouter.regions.current.cursor_x, y: image_base };

    // DEBT-28 encerrado: intrinsic_width/height vêm de calculate_dimensions.
    let intrinsic_w = dims.intrinsic_width.unwrap_or(100);
    let intrinsic_h = dims.intrinsic_height.unwrap_or(100);

    layouter.regions.current.current_items.push(FrameItem::Image {
        pos,
        data:             Arc::clone(&e.data.0), // .0 acede ao Arc interno de PtrEqArc
        width:            Pt(dims.width_pt),
        height:           Pt(dims.height_pt),
        intrinsic_width:  intrinsic_w,
        intrinsic_height: intrinsic_h,
    });

    // **P769** — avanço do cursor conforme o tipo de ancoragem. Após
    // imagem-a-seguir-a-texto, a próxima baseline fica no topo da imagem +
    // `below + cap_height`; após imagem-a-seguir-a-bloco, ou imagem isolada,
    // mantém-se o comportamento de bloco (base da imagem + `below`).
    if in_main_flow {
        if layouter.block_chain_active {
            layouter.regions.current.cursor_y = image_base + Pt(dims.height_pt + below_pt);
        } else if had_text_line {
            layouter.regions.current.cursor_y = image_base + Pt(dims.height_pt + below_pt + cap_height.0);
        } else {
            layouter.regions.current.cursor_y = image_base + Pt(dims.height_pt + below_pt);
        }
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    } else {
        layouter.regions.current.cursor_y += Pt(dims.height_pt);
    }

    if layouter.regions.current.cursor_y.0 > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::image_sizer::NullImageSizer;

    #[test]
    fn dimensoes_fallback_quando_sizer_retorna_none() {
        let dims = calculate_dimensions(&[], None, None, &NullImageSizer);
        assert_eq!(dims.width_pt,  100.0);
        assert_eq!(dims.height_pt, 100.0);
    }

    #[test]
    fn dimensoes_intrinsecas_sem_overrides() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> { Some((400, 300)) }
        }
        // 400 * 0.75 = 300pt; 300 * 0.75 = 225pt
        let dims = calculate_dimensions(&[], None, None, &MockSizer);
        assert_eq!(dims.width_pt,  300.0);
        assert_eq!(dims.height_pt, 225.0);
    }

    #[test]
    fn override_width_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> { Some((400, 300)) }
        }
        // Forçar width = 120pt → height = 120 / (4/3) = 90pt
        let w = Value::Float(120.0);
        let dims = calculate_dimensions(&[], Some(&w), None, &MockSizer);
        assert_eq!(dims.width_pt,  120.0);
        assert_eq!(dims.height_pt,  90.0);
    }

    #[test]
    fn override_height_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> { Some((400, 300)) }
        }
        // Forçar height = 90pt → width = 90 * (4/3) = 120pt
        let h = Value::Float(90.0);
        let dims = calculate_dimensions(&[], None, Some(&h), &MockSizer);
        assert_eq!(dims.width_pt,  120.0);
        assert_eq!(dims.height_pt,  90.0);
    }

    #[test]
    fn ambos_overrides_forcam_dimensoes() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> { Some((400, 300)) }
        }
        let w = Value::Float(50.0);
        let h = Value::Float(50.0);
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), &MockSizer);
        assert_eq!(dims.width_pt, 50.0);
        assert_eq!(dims.height_pt, 50.0);
    }

    #[test]
    fn calculate_dimensions_retorna_intrinsic() {
        // NullImageSizer retorna None — campos intrinsic devem ser None.
        let dims = calculate_dimensions(
            &[0xFF, 0xD8, 0xFF, 0x00],
            None,
            None,
            &NullImageSizer,
        );
        assert_eq!(dims.intrinsic_width,  None);
        assert_eq!(dims.intrinsic_height, None);

        // Sizer com dimensões reais — campos devem ser preenchidos.
        struct FixedSizer;
        impl ImageSizer for FixedSizer {
            fn size(&self, _data: &[u8]) -> Option<(u32, u32)> { Some((800, 600)) }
        }
        let dims2 = calculate_dimensions(&[], None, None, &FixedSizer);
        assert_eq!(dims2.intrinsic_width,  Some(800));
        assert_eq!(dims2.intrinsic_height, Some(600));
    }
}
