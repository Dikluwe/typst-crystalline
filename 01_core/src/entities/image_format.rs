//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/image-format.md
//! @prompt-hash b1586899
//! @layer L1
//! @updated 2026-07-17
//!
//! Detecção de formato de imagem por assinatura binária (magic bytes).
//! Movido de `03_infra/src/export/images.rs` (P772p) — pura, sem I/O,
//! reutilizável tanto por `native_image` (validação em avaliação) como
//! pelo exportador PDF (sem duplicar a lógica entre camadas).

/// Formato de imagem detectado a partir dos bytes crus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Unknown,
}

/// Detecta o formato pela assinatura binária dos primeiros bytes.
/// Não decodifica a imagem — só compara a assinatura.
pub fn detect_image_format(data: &[u8]) -> ImageFormat {
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        ImageFormat::Jpeg
    } else if data.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        ImageFormat::Png
    } else {
        ImageFormat::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detecta_jpeg() {
        assert_eq!(detect_image_format(&[0xFF, 0xD8, 0xFF, 0xE0]), ImageFormat::Jpeg);
    }

    #[test]
    fn detecta_png() {
        assert_eq!(
            detect_image_format(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0, 0]),
            ImageFormat::Png
        );
    }

    #[test]
    fn desconhecido_para_bytes_arbitrarios() {
        assert_eq!(detect_image_format(b"garbage-not-an-image"), ImageFormat::Unknown);
    }

    #[test]
    fn desconhecido_para_vazio() {
        assert_eq!(detect_image_format(&[]), ImageFormat::Unknown);
    }
}
