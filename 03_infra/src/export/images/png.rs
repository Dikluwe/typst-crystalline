//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/png.md
//! @prompt-hash 1c9dda55
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
    let img = decode_raster_image(raw_data)?;

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

// The existing image 0.24 WebP decoder changes decoded colors for valid lossy
// payloads. Keep the other codecs and share one corrected WebP route with PNG.
pub(in crate::export) fn decode_raster_image(
    raw_data: &[u8],
) -> Result<image::DynamicImage, String> {
    if super::detect_image_format(raw_data) != super::ImageFormat::WebP {
        return image::load_from_memory(raw_data).map_err(|error| error.to_string());
    }
    let mut decoder = image_webp::WebPDecoder::new(std::io::Cursor::new(raw_data))
        .map_err(|error| error.to_string())?;
    let mut limits = image::io::Limits::default();
    if let Some(maximum) = limits.max_alloc {
        decoder.set_memory_limit(usize::try_from(maximum).unwrap_or(usize::MAX));
    }
    let (width, height) = decoder.dimensions();
    let alpha = decoder.has_alpha();
    let required =
        decoder.output_buffer_size().ok_or("WebP image buffer is too large")?;
    limits.reserve(required as u64).map_err(|error| error.to_string())?;
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(required)
        .map_err(|error| error.to_string())?;
    pixels.resize(required, 0);
    decoder.read_image(&mut pixels).map_err(|error| error.to_string())?;
    if alpha {
        image::RgbaImage::from_raw(width, height, pixels)
            .map(image::DynamicImage::ImageRgba8)
            .ok_or_else(|| "invalid WebP RGBA buffer".to_string())
    } else {
        image::RgbImage::from_raw(width, height, pixels)
            .map(image::DynamicImage::ImageRgb8)
            .ok_or_else(|| "invalid WebP RGB buffer".to_string())
    }
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

#[cfg(test)]
mod raster_bytes_webp_tests {
    use super::*;
    use std::io::Read;
    const WEBP_LOSSY: &[u8] = &[
        82, 73, 70, 70, 144, 0, 0, 0, 87, 69, 66, 80, 86, 80, 56, 88, 10, 0, 0, 0, 16, 0,
        0, 0, 15, 0, 0, 7, 0, 0, 65, 76, 80, 72, 23, 0, 0, 0, 1, 15, 240, 192, 255, 136,
        136, 160, 32, 109, 3, 102, 254, 77, 255, 121, 136, 232, 255, 4, 224, 115, 1, 0,
        86, 80, 56, 32, 82, 0, 0, 0, 144, 3, 0, 157, 1, 42, 16, 0, 8, 0, 1, 64, 38, 37,
        168, 2, 116, 186, 1, 38, 3, 248, 7, 226, 5, 223, 253, 216, 7, 244, 0, 95, 26, 7,
        168, 230, 192, 0, 254, 246, 137, 223, 128, 37, 251, 122, 31, 188, 37, 143, 254,
        102, 233, 188, 20, 63, 252, 142, 120, 224, 15, 255, 75, 131, 255, 77, 155, 255,
        164, 243, 88, 190, 15, 230, 53, 190, 185, 114, 99, 128, 0, 0,
    ];
    #[test]
    fn raster_bytes_lossy_webp_preserves_rgb_and_alpha() {
        let payload = process_png_for_pdf(WEBP_LOSSY).unwrap();
        assert_eq!((payload.width, payload.height), (16, 8));
        let mut rgb = Vec::new();
        flate2::read::ZlibDecoder::new(payload.rgb_data_compressed.as_slice())
            .read_to_end(&mut rgb)
            .unwrap();
        let mut alpha = Vec::new();
        flate2::read::ZlibDecoder::new(
            payload
                .alpha_data_compressed
                .as_ref()
                .expect("alpha preserved")
                .as_slice(),
        )
        .read_to_end(&mut alpha)
        .unwrap();
        for ((x, y), expected) in [(2, 2), (14, 2), (2, 6), (14, 6)].into_iter().zip([
            [255, 1, 5],
            [0, 255, 1],
            [2, 0, 255],
            [255, 254, 2],
        ]) {
            let i = (y * 16 + x) * 3;
            for channel in 0..3 {
                assert!(
                    (rgb[i + channel] as i16 - expected[channel]).abs() <= 3,
                    "decoded sample {:?} expected {:?}",
                    &rgb[i..i + 3],
                    expected
                );
            }
        }
        assert_eq!(alpha[2 * 16 + 2], 255);
        assert_eq!(alpha[6 * 16 + 14], 128);
    }
}
