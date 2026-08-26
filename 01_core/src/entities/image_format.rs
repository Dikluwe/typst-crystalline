//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/image-format.md
//! @prompt-hash f7ce1d7d
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
    /// **P833** (#17) — GIF (frame estático, como o vanilla).
    Gif,
    /// **P833** (#17) — WebP.
    WebP,
    Unknown,
}

/// Detecta o formato pela assinatura binária dos primeiros bytes.
/// Não decodifica a imagem — só compara a assinatura.
pub fn detect_image_format(data: &[u8]) -> ImageFormat {
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        ImageFormat::Jpeg
    } else if data.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        ImageFormat::Png
    } else if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        ImageFormat::Gif
    } else if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP" {
        ImageFormat::WebP
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
    fn detecta_gif() {
        // GIF87a e GIF89a (P833, #17).
        assert_eq!(detect_image_format(b"GIF87a\x01\x00"), ImageFormat::Gif);
        assert_eq!(detect_image_format(b"GIF89a\x02\x00"), ImageFormat::Gif);
    }

    #[test]
    fn detecta_webp() {
        // RIFF + "WEBP" no offset 8 (P833, #17).
        let webp: &[u8] = &[82, 73, 70, 70, 28, 0, 0, 0, 87, 69, 66, 80, 86, 80, 56, 76];
        assert_eq!(detect_image_format(webp), ImageFormat::WebP);
        // RIFF sem "WEBP" não é WebP.
        assert_eq!(
            detect_image_format(b"RIFF\x04\x00\x00\x00WAVE"),
            ImageFormat::Unknown
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
