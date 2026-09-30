//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/png.md
//! @prompt-hash fab3d5a8
//! @layer L3
//! @updated 2026-09-26

use super::{compress_zlib, PdfImagePayload};

/// Descodifica uma imagem raster (PNG, GIF, WebP) e prepara os dados para
/// emissão como XObject(s) num PDF.
///
/// **Sem alpha**: converte para RGB8, comprime os bytes planos com Zlib.
/// **Com alpha**: separa os canais RGB e A, comprime ambos separadamente.
///   Se o canal A for totalmente opaco (todos 255), descarta-o — um /SMask
///   com alpha uniforme não tem efeito visual e aumenta o PDF desnecessariamente.
///
/// O nome é histórico (P833/#17): a função sempre foi formato-genérica
/// (`image::load_from_memory`); GIF/WebP passaram a ser suportados quando as
/// features `gif`/`webp` da crate `image` foram activadas. GIF fica estático
/// no primeiro frame (paridade vanilla).
///
/// Erro: devolve o detalhe cru do decoder — o caller embrulha no formato do
/// vanilla (`failed to decode image ({detalhe})`, P833/#18).
pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String> {
    let img = image::load_from_memory(raw_data).map_err(|e| e.to_string())?;

    let width = img.width();
    let height = img.height();

    if !img.color().has_alpha() {
        return Ok(PdfImagePayload {
            width,
            height,
            color_space: "/DeviceRGB",
            rgb_data_compressed: compress_zlib(img.to_rgb8().as_raw())?,
            alpha_data_compressed: None,
        });
    }

    let rgba = img.to_rgba8();
    let mut rgb_buf = Vec::with_capacity((width * height * 3) as usize);
    let mut alpha_buf = Vec::with_capacity((width * height) as usize);

    for pixel in rgba.pixels() {
        rgb_buf.push(pixel[0]);
        rgb_buf.push(pixel[1]);
        rgb_buf.push(pixel[2]);
        alpha_buf.push(pixel[3]);
    }

    let alpha_compressed = if alpha_buf.iter().all(|&a| a == 255) {
        None // totalmente opaco — /SMask redundante
    } else {
        Some(compress_zlib(&alpha_buf)?)
    };

    Ok(PdfImagePayload {
        width,
        height,
        color_space: "/DeviceRGB",
        rgb_data_compressed: compress_zlib(&rgb_buf)?,
        alpha_data_compressed: alpha_compressed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn png_payload_preserves_rgb_and_alpha_channels() {
        for alpha in [0, 128, 255] {
            let image =
                image::RgbaImage::from_raw(1, 1, vec![17, 89, 201, alpha]).unwrap();
            let mut encoded = Cursor::new(Vec::new());
            image.write_to(&mut encoded, image::ImageFormat::Png).unwrap();
            let payload = process_png_for_pdf(encoded.get_ref()).unwrap();
            assert_eq!(
                (payload.width, payload.height, payload.color_space),
                (1, 1, "/DeviceRGB")
            );
            let mut rgb = Vec::new();
            flate2::read::ZlibDecoder::new(payload.rgb_data_compressed.as_slice())
                .read_to_end(&mut rgb)
                .unwrap();
            assert_eq!(rgb, [17, 89, 201]);
            if alpha == 255 {
                assert!(payload.alpha_data_compressed.is_none());
            } else {
                let mut decoded = Vec::new();
                flate2::read::ZlibDecoder::new(
                    payload.alpha_data_compressed.unwrap().as_slice(),
                )
                .read_to_end(&mut decoded)
                .unwrap();
                assert_eq!(decoded, [alpha]);
            }
        }
        assert!(process_png_for_pdf(b"not an image").is_err());
    }
}
