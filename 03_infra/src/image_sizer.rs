//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/image-sizer.md
//! @prompt-hash a6443c5a
//! @layer L3
//! @updated 2026-04-19

use typst_core::entities::image_sizer::ImageSizer;

/// Implementação de ImageSizer usando a crate imagesize.
/// imagesize lê apenas o cabeçalho do ficheiro — não descodifica píxeis.
pub struct ImageSizeImageSizer;

impl ImageSizer for ImageSizeImageSizer {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)> {
        imagesize::blob_size(data)
            .ok()
            .map(|s| (s.width as u32, s.height as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_sizer_le_cabecalho_png_1x1() {
        // PNG 1×1 px transparente — bytes do cabeçalho suficientes para imagesize
        let png_1x1: &[u8] = &[
            137, 80, 78, 71, 13, 10, 26, 10,
            0, 0, 0, 13, 73, 72, 68, 82,
            0, 0, 0, 1, 0, 0, 0, 1,
            8, 6, 0, 0, 0, 31, 21, 196, 137,
            0, 0, 0, 11, 73, 68, 65, 84,
            8, 215, 99, 96, 0, 2, 0, 0,
            5, 0, 1, 226, 38, 5, 155,
            0, 0, 0, 0, 73, 69, 78, 68,
            174, 66, 96, 130,
        ];

        let sizer = ImageSizeImageSizer;
        let result = sizer.size(png_1x1);
        assert_eq!(result, Some((1, 1)),
            "imagesize deve ler cabeçalho PNG 1×1: {:?}", result);
    }
}
