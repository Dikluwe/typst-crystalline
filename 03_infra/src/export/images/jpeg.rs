//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/jpeg.md
//! @prompt-hash e4ae0d49
//! @layer L3
//! @updated 2026-09-26

/// Lê o marcador SOF0 (0xC0) ou SOF2 (0xC2) do cabeçalho JPEG para determinar
/// o número de componentes de cor (1, 3 ou 4).
///
/// Usado por `jpeg_color_space` e `jpeg_is_rgb`.
fn jpeg_components(data: &[u8]) -> u8 {
    let mut i = 2usize; // saltar SOI (FF D8)
    while i + 3 < data.len() {
        if data[i] != 0xFF {
            break;
        }
        let marker = data[i + 1];
        let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;

        if marker == 0xC0 || marker == 0xC2 {
            // SOF: offset i+9 é o número de componentes de cor
            if i + 9 < data.len() {
                return data[i + 9];
            }
            break;
        }

        // SOS (0xDA) inicia os dados comprimidos — parar antes de entrar neles.
        if marker == 0xDA {
            break;
        }

        if len < 2 {
            break;
        }
        i += 2 + len;
    }
    3 // fallback RGB
}

/// Lê o marcador SOF0 (0xC0) ou SOF2 (0xC2) do cabeçalho JPEG para determinar
/// o ColorSpace correcto para o dicionário do XObject (DEBT-29).
///
/// Um JPEG com ColorSpace errado produz lixo visual (Grayscale renderizado como
/// RGB monocromático) ou é recusado por alguns leitores PDF (CMYK).
/// O fallback "/DeviceRGB" cobre a maioria dos JPEGs de câmara.
pub(in crate::export) fn jpeg_color_space(data: &[u8]) -> &'static str {
    match jpeg_components(data) {
        1 => "/DeviceGray",
        4 => "/DeviceCMYK",
        _ => "/DeviceRGB",
    }
}

/// **P777** — verdadeiro se o JPEG for RGB (3 componentes).
pub(in crate::export) fn jpeg_is_rgb(data: &[u8]) -> bool {
    jpeg_components(data) == 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpeg_sof_channels_and_truncated_headers() {
        for marker in [0xc0, 0xc2] {
            for (channels, space, rgb) in [
                (1, "/DeviceGray", false),
                (3, "/DeviceRGB", true),
                (4, "/DeviceCMYK", false),
            ] {
                let data = [0xff, 0xd8, 0xff, marker, 0, 11, 8, 0, 1, 0, 1, channels];
                assert_eq!(jpeg_color_space(&data), space);
                assert_eq!(jpeg_is_rgb(&data), rgb);
                for end in 0..data.len() {
                    assert_eq!(jpeg_color_space(&data[..end]), "/DeviceRGB");
                }
            }
        }
    }
}
