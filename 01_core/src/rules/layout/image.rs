//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout-image.md
//! @prompt-hash 9a038555
//! @layer L1
//! @updated 2026-04-19

use crate::entities::image_sizer::ImageSizer;
use crate::entities::value::Value;

/// Densidade padrão para conversão px → pt.
/// 96 DPI: 1 pt = 1/72 inch; 1 px = 1/96 inch → 1 px = 72/96 pt = 0.75 pt.
const PX_TO_PT: f64 = 0.75;

/// Dimensões finais de uma imagem para o layouter, em pontos.
pub struct ImageDimensions {
    pub width_pt:  f64,
    pub height_pt: f64,
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
    let (intrinsic_w_pt, intrinsic_h_pt) = match sizer.size(data) {
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

    ImageDimensions { width_pt, height_pt }
}

fn extract_pt(val: &Value) -> Option<f64> {
    match val {
        Value::Float(f)  => Some(*f),
        Value::Length(l) => Some(l.abs.to_pt()),
        _ => None,
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
}
