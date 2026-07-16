//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/image-sizer.md
//! @prompt-hash d6ffa02a
//! @layer L3
//! @updated 2026-04-19

use std::io::Cursor;

use typst_core::entities::image_sizer::ImageSizer;

/// Implementação de ImageSizer usando a crate imagesize.
/// imagesize lê apenas o cabeçalho do ficheiro — não descodifica píxeis.
#[derive(Clone, Copy)]
pub struct ImageSizeImageSizer;

impl ImageSizer for ImageSizeImageSizer {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)> {
        imagesize::blob_size(data)
            .ok()
            .map(|s| (s.width as u32, s.height as u32))
    }

    fn dpi(&self, data: &[u8]) -> Option<f64> {
        determine_dpi(data)
    }

    fn orientation(&self, data: &[u8]) -> Option<u32> {
        exif_orientation(data)
    }
}

/// Determina o DPI da imagem a partir dos seus metadados.
/// Prioridade: EXIF > JFIF APP0 > PNG pHYs.
fn determine_dpi(data: &[u8]) -> Option<f64> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        // PNG: EXIF (eXIf) tem prioridade sobre pHYs.
        exif_dpi(data).or_else(|| png_phys_dpi(data))
    } else if data.starts_with(b"\xff\xd8") {
        // JPEG: EXIF (APP1) tem prioridade sobre JFIF APP0.
        exif_dpi(data).or_else(|| jfif_dpi(data))
    } else {
        None
    }
}

/// Lê o DPI do chunk `pHYs` de um PNG.
///
/// Estrutura do chunk:
/// - 4 bytes: pixels por unidade X
/// - 4 bytes: pixels por unidade Y
/// - 1 byte: unidade (1 = metro)
fn png_phys_dpi(data: &[u8]) -> Option<f64> {
    // O chunk pHYs começa após a assinatura PNG (8 bytes) e o chunk IHDR.
    let mut i = 8;
    while i + 12 <= data.len() {
        let len = u32::from_be_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
        let chunk_type = &data[i + 4..i + 8];
        let chunk_data_start = i + 8;
        let chunk_data_end = chunk_data_start + len;

        if chunk_data_end + 4 > data.len() {
            break;
        }

        if chunk_type == b"pHYs" && len == 9 {
            let ppu_x = u32::from_be_bytes([
                data[chunk_data_start],
                data[chunk_data_start + 1],
                data[chunk_data_start + 2],
                data[chunk_data_start + 3],
            ]);
            let unit = data[chunk_data_start + 8];
            if unit == 1 {
                // pixels por metro → DPI: 1 metro = 39.3701 polegadas.
                return Some(ppu_x as f64 * 0.0254);
            } else if unit == 2 {
                // pixels por centímetro → DPI.
                return Some(ppu_x as f64 * 2.54);
            }
        }

        // Avança para o próximo chunk (len + type + data + CRC).
        i = chunk_data_end + 4;
    }
    None
}

/// Lê o DPI do segmento JFIF APP0 de um JPEG.
///
/// Estrutura do segmento APP0 (após o marker 0xFFE0):
/// - 2 bytes: comprimento (incluindo estes 2 bytes)
/// - 5 bytes: identificador "JFIF\0"
/// - 2 bytes: versão
/// - 1 byte: unidades (0 = nenhuma, 1 = DPI, 2 = DPCM)
/// - 2 bytes: densidade X
/// - 2 bytes: densidade Y
fn jfif_dpi(data: &[u8]) -> Option<f64> {
    let mut i = 2; // após a assinatura SOI 0xFFD8
    while i + 4 <= data.len() {
        if data[i] != 0xFF || data[i + 1] == 0x00 || data[i + 1] == 0xFF {
            i += 1;
            continue;
        }

        let marker = data[i + 1];
        if marker == 0xD9 || marker == 0xD8 {
            // EOI ou SOI — fim ou reinício; seguro em frente.
            i += 2;
            continue;
        }

        if i + 4 > data.len() {
            break;
        }
        let seg_len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
        let seg_end = i + 2 + seg_len;
        if seg_end > data.len() {
            break;
        }

        if marker == 0xE0 {
            let seg_data = &data[i + 4..seg_end];
            // APP0 JFIF: identifier[5] + version[2] + units[1] + Xdensity[2] + Ydensity[2]
            if seg_data.starts_with(b"JFIF\0") && seg_data.len() >= 12 {
                let units = seg_data[7];
                let x_density = u16::from_be_bytes([seg_data[8], seg_data[9]]);
                if units == 1 && x_density > 0 {
                    return Some(x_density as f64);
                }
            }
        }

        i = seg_end;
    }
    None
}

/// Lê o DPI de metadados EXIF.
///
/// O EXIF pode estar em:
/// - JPEG: segmento APP1 (marker 0xFFE1) com identificador "Exif\0\0".
/// - PNG: chunk `eXIf`.
fn exif_dpi(data: &[u8]) -> Option<f64> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        // Procura chunk eXIf.
        let mut i = 8;
        while i + 12 <= data.len() {
            let len = u32::from_be_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
            let chunk_type = &data[i + 4..i + 8];
            let chunk_data_start = i + 8;
            let chunk_data_end = chunk_data_start + len;
            if chunk_data_end + 4 > data.len() {
                break;
            }
            if chunk_type == b"eXIf" {
                let tiff = &data[chunk_data_start..chunk_data_end];
                if let Some(dpi) = parse_tiff_dpi(tiff) {
                    return Some(dpi);
                }
            }
            i = chunk_data_end + 4;
        }
    } else if data.starts_with(b"\xff\xd8") {
        // Procura segmento APP1.
        let mut i = 2;
        while i + 4 <= data.len() {
            if data[i] != 0xFF || data[i + 1] == 0x00 || data[i + 1] == 0xFF {
                i += 1;
                continue;
            }

            let marker = data[i + 1];
            if marker == 0xD9 || marker == 0xD8 {
                i += 2;
                continue;
            }

            let seg_len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
            let seg_end = i + 2 + seg_len;
            if seg_end > data.len() {
                break;
            }

            if marker == 0xE1 {
                let seg_data = &data[i + 4..seg_end];
                if seg_data.starts_with(b"Exif\0\0") && seg_data.len() >= 8 {
                    let tiff = &seg_data[6..];
                    if let Some(dpi) = parse_tiff_dpi(tiff) {
                        return Some(dpi);
                    }
                }
            }

            i = seg_end;
        }
    }
    None
}

/// Lê a orientação EXIF (tag 0x0112) dos metadados da imagem.
///
/// O EXIF pode estar em:
/// - JPEG: segmento APP1 (marker 0xFFE1) com identificador "Exif\0\0".
/// - PNG: chunk `eXIf`.
fn exif_orientation(data: &[u8]) -> Option<u32> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        let mut i = 8;
        while i + 12 <= data.len() {
            let len = u32::from_be_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
            let chunk_type = &data[i + 4..i + 8];
            let chunk_data_start = i + 8;
            let chunk_data_end = chunk_data_start + len;
            if chunk_data_end + 4 > data.len() {
                break;
            }
            if chunk_type == b"eXIf" {
                let tiff = &data[chunk_data_start..chunk_data_end];
                if let Some(orientation) = parse_tiff_orientation(tiff) {
                    return Some(orientation);
                }
            }
            i = chunk_data_end + 4;
        }
    } else if data.starts_with(b"\xff\xd8") {
        let mut i = 2;
        while i + 4 <= data.len() {
            if data[i] != 0xFF || data[i + 1] == 0x00 || data[i + 1] == 0xFF {
                i += 1;
                continue;
            }

            let marker = data[i + 1];
            if marker == 0xD9 || marker == 0xD8 {
                i += 2;
                continue;
            }

            let seg_len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
            let seg_end = i + 2 + seg_len;
            if seg_end > data.len() {
                break;
            }

            if marker == 0xE1 {
                let seg_data = &data[i + 4..seg_end];
                if seg_data.starts_with(b"Exif\0\0") && seg_data.len() >= 8 {
                    let tiff = &seg_data[6..];
                    if let Some(orientation) = parse_tiff_orientation(tiff) {
                        return Some(orientation);
                    }
                }
            }

            i = seg_end;
        }
    }
    None
}

/// Faz parsing de um bloco TIFF (little ou big endian) e devolve a tag
/// Orientation (0x0112), se existir.
fn parse_tiff_orientation(tiff: &[u8]) -> Option<u32> {
    if tiff.len() < 8 {
        return None;
    }

    let (_le, u16_, u32_): (bool, fn(&[u8]) -> u16, fn(&[u8]) -> u32) =
        match &tiff[0..2] {
            b"II" => (true, |b| u16::from_le_bytes([b[0], b[1]]), |b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            }),
            b"MM" => (false, |b| u16::from_be_bytes([b[0], b[1]]), |b| {
                u32::from_be_bytes([b[0], b[1], b[2], b[3]])
            }),
            _ => return None,
        };

    let magic = u16_(&tiff[2..4]);
    if magic != 42 {
        return None;
    }

    let ifd_offset = u32_(&tiff[4..8]) as usize;
    read_ifd_orientation(tiff, ifd_offset, u16_, u32_)
}

/// Lê a tag Orientation (0x0112) de um IFD TIFF.
fn read_ifd_orientation(
    tiff: &[u8],
    offset: usize,
    u16_: fn(&[u8]) -> u16,
    u32_: fn(&[u8]) -> u32,
) -> Option<u32> {
    if offset + 2 > tiff.len() {
        return None;
    }

    let num_entries = u16_(&tiff[offset..offset + 2]) as usize;
    let mut entry_offset = offset + 2;
    for _ in 0..num_entries {
        if entry_offset + 12 > tiff.len() {
            break;
        }

        let tag = u16_(&tiff[entry_offset..entry_offset + 2]);
        let type_ = u16_(&tiff[entry_offset + 2..entry_offset + 4]);
        let count = u32_(&tiff[entry_offset + 4..entry_offset + 8]);
        let value_bytes = &tiff[entry_offset + 8..entry_offset + 12];

        if tag == 0x0112 && count == 1 {
            let orientation = match type_ {
                1 => value_bytes[0] as u32, // BYTE
                3 => u16_(value_bytes) as u32, // SHORT
                4 => u32_(value_bytes), // LONG
                _ => return None,
            };
            if (1..=8).contains(&orientation) {
                return Some(orientation);
            }
        }

        entry_offset += 12;
    }

    None
}

/// Faz parsing de um bloco TIFF (little ou big endian) e devolve o DPI de
/// XResolution (tag 0x011A), se existir.
fn parse_tiff_dpi(tiff: &[u8]) -> Option<f64> {
    if tiff.len() < 8 {
        return None;
    }

    let (_le, u16_, u32_): (bool, fn(&[u8]) -> u16, fn(&[u8]) -> u32) =
        match &tiff[0..2] {
            b"II" => (true, |b| u16::from_le_bytes([b[0], b[1]]), |b| {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            }),
            b"MM" => (false, |b| u16::from_be_bytes([b[0], b[1]]), |b| {
                u32::from_be_bytes([b[0], b[1], b[2], b[3]])
            }),
            _ => return None,
        };

    let magic = u16_(&tiff[2..4]);
    if magic != 42 {
        return None;
    }

    let ifd_offset = u32_(&tiff[4..8]) as usize;
    read_ifd_dpi(tiff, ifd_offset, u16_, u32_)
}

/// Lê a tag XResolution (0x011A) de um IFD TIFF.
fn read_ifd_dpi(
    tiff: &[u8],
    offset: usize,
    u16_: fn(&[u8]) -> u16,
    u32_: fn(&[u8]) -> u32,
) -> Option<f64> {
    if offset + 2 > tiff.len() {
        return None;
    }

    let num_entries = u16_(&tiff[offset..offset + 2]) as usize;
    let mut entry_offset = offset + 2;
    for _ in 0..num_entries {
        if entry_offset + 12 > tiff.len() {
            break;
        }

        let tag = u16_(&tiff[entry_offset..entry_offset + 2]);
        let type_ = u16_(&tiff[entry_offset + 2..entry_offset + 4]);
        let count = u32_(&tiff[entry_offset + 4..entry_offset + 8]);
        let value_offset = u32_(&tiff[entry_offset + 8..entry_offset + 12]) as usize;

        if tag == 0x011A && (type_ == 5 || type_ == 10) && count == 1 {
            let rational_offset = if value_offset + 8 <= tiff.len() {
                value_offset
            } else {
                // Valor inline: os 4 bytes inferiores do campo value/offset.
                entry_offset + 8
            };
            if rational_offset + 8 <= tiff.len() {
                let num = u32_(&tiff[rational_offset..rational_offset + 4]);
                let den = u32_(&tiff[rational_offset + 4..rational_offset + 8]);
                if den > 0 {
                    return Some((num as f64) / (den as f64));
                }
            }
        }

        entry_offset += 12;
    }

    None
}

/// Aplica a rotação EXIF aos pixels da imagem e devolve os novos bytes.
///
/// Se a imagem não tiver tag `Orientation` não-padrão, ou se o formato não for
/// JPEG/PNG, retorna `None` — o chamador deve usar os bytes originais.
///
/// O mapeamento 1-8 replica `apply_rotation` do vanilla
/// (`typst_library::visualize::image::raster`).
pub fn apply_exif_rotation(data: &[u8]) -> Option<Vec<u8>> {
    let orientation = exif_orientation(data)?;
    if orientation == 1 {
        return None;
    }

    let format = if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        image::ImageFormat::Png
    } else if data.starts_with(b"\xff\xd8") {
        image::ImageFormat::Jpeg
    } else {
        return None;
    };

    let mut img = image::load_from_memory_with_format(data, format).ok()?;

    use image::imageops as ops;
    match orientation {
        2 => ops::flip_horizontal_in_place(&mut img),
        3 => ops::rotate180_in_place(&mut img),
        4 => ops::flip_vertical_in_place(&mut img),
        5 => {
            ops::flip_horizontal_in_place(&mut img);
            img = img.rotate270();
        }
        6 => img = img.rotate90(),
        7 => {
            ops::flip_horizontal_in_place(&mut img);
            img = img.rotate90();
        }
        8 => img = img.rotate270(),
        _ => return None,
    }

    let mut out = Vec::new();
    let output_format = match format {
        image::ImageFormat::Png => image::ImageOutputFormat::Png,
        image::ImageFormat::Jpeg => image::ImageOutputFormat::Jpeg(95),
        _ => return None,
    };
    img.write_to(&mut Cursor::new(&mut out), output_format).ok()?;
    Some(out)
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

    #[test]
    fn png_1x1_sem_dpi() {
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
        assert_eq!(ImageSizeImageSizer.dpi(png_1x1), None);
    }

    #[test]
    fn png_1x1_com_phys_300dpi() {
        // PNG 1×1 px com chunk pHYs a 300 DPI (11811 pixels/metro).
        let png_1x1_dpi300: &[u8] = &[
            137, 80, 78, 71, 13, 10, 26, 10,
            0, 0, 0, 13, 73, 72, 68, 82,
            0, 0, 0, 1, 0, 0, 0, 1,
            8, 6, 0, 0, 0, 31, 21, 196, 137,
            // pHYs — 11811 ppm, unit = 1 (metro) → 300 DPI
            0, 0, 0, 9, 112, 72, 89, 115,
            0, 0, 46, 35, 0, 0, 46, 35, 1,
            120, 165, 63, 118,
            0, 0, 0, 11, 73, 68, 65, 84,
            8, 215, 99, 96, 0, 2, 0, 0,
            5, 0, 1, 226, 38, 5, 155,
            0, 0, 0, 0, 73, 69, 78, 68,
            174, 66, 96, 130,
        ];
        let dpi = ImageSizeImageSizer.dpi(png_1x1_dpi300);
        assert!(dpi.is_some(), "pHYs deve ser detectado");
        assert!((dpi.unwrap() - 300.0).abs() < 0.01, "DPI deve ser ~300, foi {:?}", dpi);
    }

    #[test]
    fn jpeg_jfif_300dpi_inline() {
        // SOI + APP0 JFIF com 300 DPI. O parser de JFIF lê apenas o APP0.
        let jpeg: &[u8] = &[
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, 0x00, 0x10, // APP0, length 16
            0x4A, 0x46, 0x49, 0x46, 0x00, // "JFIF\0"
            0x01, 0x01, // version 1.1
            0x01, // units = DPI
            0x01, 0x2C, // Xdensity = 300
            0x01, 0x2C, // Ydensity = 300
            0x00, 0x00, // thumbnail 0×0
        ];
        let dpi = ImageSizeImageSizer.dpi(jpeg);
        assert!(dpi.is_some(), "JFIF APP0 deve ser detectado");
        assert!((dpi.unwrap() - 300.0).abs() < 0.01, "DPI deve ser ~300, foi {:?}", dpi);
    }

    #[test]
    fn imagens_reais_300dpi_se_existirem() {
        let paths = ["/tmp/p773-dpi300.png", "/tmp/p773-dpi300.jpg"];
        for path in &paths {
            if !std::path::Path::new(path).exists() {
                continue;
            }
            let data = std::fs::read(path).expect("ler imagem de teste");
            let dpi = ImageSizeImageSizer.dpi(&data);
            assert!(dpi.is_some(), "{}: DPI deve ser detectado", path);
            assert!(
                (dpi.unwrap() - 300.0).abs() < 1.0,
                "{}: DPI deve ser ~300, foi {:?}",
                path,
                dpi
            );
        }
    }

    #[test]
    fn exif_dpi_inline_le() {
        // TIFF little-endian mínimo com XResolution = 300/1.
        // Header: II, magic 42, offset do IFD = 8.
        // IFD: 1 entry (tag 0x011A, type RATIONAL, count 1, offset 26).
        // Next IFD = 0.
        // Valor rational em offset 26: num=300, den=1.
        let tiff: &[u8] = &[
            0x49, 0x49, // II
            0x2A, 0x00, // magic 42
            0x08, 0x00, 0x00, 0x00, // offset IFD = 8
            // IFD
            0x01, 0x00, // 1 entry
            // entry: tag 0x011A, type 5, count 1, offset 26
            0x1A, 0x01, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x00, 0x00,
            // next IFD = 0
            0x00, 0x00, 0x00, 0x00,
            // valor rational: 300 / 1
            0x2C, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];
        assert_eq!(parse_tiff_dpi(tiff), Some(300.0));
    }

    #[test]
    fn exif_dpi_inline_be() {
        // TIFF big-endian mínimo com XResolution = 300/1.
        let tiff: &[u8] = &[
            0x4D, 0x4D, // MM
            0x00, 0x2A, // magic 42
            0x00, 0x00, 0x00, 0x08, // offset IFD = 8
            // IFD
            0x00, 0x01, // 1 entry
            // entry: tag 0x011A, type 5, count 1, offset 26
            0x01, 0x1A, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1A,
            // next IFD = 0
            0x00, 0x00, 0x00, 0x00,
            // valor rational: 300 / 1
            0x00, 0x00, 0x01, 0x2C, 0x00, 0x00, 0x00, 0x01,
        ];
        assert_eq!(parse_tiff_dpi(tiff), Some(300.0));
    }

    #[test]
    fn exif_orientation_inline() {
        // TIFF LE com Orientation = 6 (rotate 90 CW).
        let tiff: &[u8] = &[
            0x49, 0x49,
            0x2A, 0x00,
            0x08, 0x00, 0x00, 0x00,
            0x01, 0x00,
            // entry: tag 0x0112, type 3 (SHORT), count 1, value 6
            0x12, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(parse_tiff_orientation(tiff), Some(6));

        // TIFF BE com Orientation = 8.
        let tiff_be: &[u8] = &[
            0x4D, 0x4D,
            0x00, 0x2A,
            0x00, 0x00, 0x00, 0x08,
            0x00, 0x01,
            // entry: tag 0x0112, type 3, count 1, value 8
            0x01, 0x12, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(parse_tiff_orientation(tiff_be), Some(8));
    }

    #[test]
    fn apply_exif_rotation_orient1_retorna_none() {
        // Orientação 1 → sem transformação.
        let data = std::fs::read("/tmp/p774-base.jpg").unwrap_or_default();
        if data.is_empty() {
            return;
        }
        assert_eq!(apply_exif_rotation(&data), None);
    }

    #[test]
    fn apply_exif_rotation_real_se_existir() {
        let path = "/tmp/p774-orient6.jpg";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).expect("ler imagem de teste");
        assert_eq!(ImageSizeImageSizer.orientation(&data), Some(6));

        let rotated = apply_exif_rotation(&data).expect("deve rodar orient6");
        // Após rotação, as dimensões devem estar trocadas.
        let (w, h) = ImageSizeImageSizer.size(&rotated).expect("tamanho após rotação");
        assert_eq!((w, h), (100, 200), "orient6: 200×100 → 100×200");
    }
}
