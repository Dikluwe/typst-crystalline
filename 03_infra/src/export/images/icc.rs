//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/icc.md
//! @prompt-hash 6a8e1553
//! @layer L3
//! @updated 2026-09-26

use super::compress_zlib;
use typst_core::entities::document_format_options::PdfVersion;

/// **P777** — perfil ICC sRGB compacto (480 bytes, compatível com lcms2).
/// Replicado do vanilla/krilla para JPEGs RGB em PDF.
const SRGB_ICC_PROFILE: &[u8] = &[
    0, 0, 1, 224, 108, 99, 109, 115, 4, 32, 0, 0, 109, 110, 116, 114, 82, 71, 66, 32, 88,
    89, 90, 32, 7, 226, 0, 3, 0, 20, 0, 9, 0, 14, 0, 29, 97, 99, 115, 112, 77, 83, 70,
    84, 0, 0, 0, 0, 115, 97, 119, 115, 99, 116, 114, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 246, 214, 0, 1, 0, 0, 0, 0, 211, 45, 104, 97, 110, 100, 121, 233, 191,
    86, 90, 62, 1, 182, 131, 35, 133, 85, 70, 247, 79, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 100, 101, 115, 99,
    0, 0, 0, 252, 0, 0, 0, 36, 99, 112, 114, 116, 0, 0, 1, 32, 0, 0, 0, 34, 119, 116,
    112, 116, 0, 0, 1, 68, 0, 0, 0, 20, 99, 104, 97, 100, 0, 0, 1, 88, 0, 0, 0, 44, 114,
    88, 89, 90, 0, 0, 1, 132, 0, 0, 0, 20, 103, 88, 89, 90, 0, 0, 1, 152, 0, 0, 0, 20,
    98, 88, 89, 90, 0, 0, 1, 172, 0, 0, 0, 20, 114, 84, 82, 67, 0, 0, 1, 192, 0, 0, 0,
    32, 103, 84, 82, 67, 0, 0, 1, 192, 0, 0, 0, 32, 98, 84, 82, 67, 0, 0, 1, 192, 0, 0,
    0, 32, 109, 108, 117, 99, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 12, 101, 110, 85, 83, 0,
    0, 0, 8, 0, 0, 0, 28, 0, 115, 0, 82, 0, 71, 0, 66, 109, 108, 117, 99, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 12, 101, 110, 85, 83, 0, 0, 0, 6, 0, 0, 0, 28, 0, 67, 0, 67, 0, 48,
    0, 0, 88, 89, 90, 32, 0, 0, 0, 0, 0, 0, 246, 214, 0, 1, 0, 0, 0, 0, 211, 45, 115,
    102, 51, 50, 0, 0, 0, 0, 0, 1, 12, 63, 0, 0, 5, 221, 255, 255, 243, 38, 0, 0, 7, 144,
    0, 0, 253, 146, 255, 255, 251, 161, 255, 255, 253, 162, 0, 0, 3, 220, 0, 0, 192, 113,
    88, 89, 90, 32, 0, 0, 0, 0, 0, 0, 111, 160, 0, 0, 56, 242, 0, 0, 3, 143, 88, 89, 90,
    32, 0, 0, 0, 0, 0, 0, 98, 150, 0, 0, 183, 137, 0, 0, 24, 218, 88, 89, 90, 32, 0, 0,
    0, 0, 0, 0, 36, 160, 0, 0, 15, 133, 0, 0, 182, 196, 112, 97, 114, 97, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 2, 102, 105, 0, 0, 242, 167, 0, 0, 13, 89, 0, 0, 19, 208, 0, 0, 10, 91,
];

/// **P777** — devolve os bytes do perfil ICC sRGB partilhado.
pub(in crate::export) fn srgb_icc_profile_bytes() -> &'static [u8] {
    SRGB_ICC_PROFILE
}

/// **P1477** — perfil ICC sRGB v2 (736 bytes), usado pelo vanilla/krilla
/// para `PdfVersion::Pdf14|Pdf15|Pdf16` (`krilla/src/configure/version.rs:
/// 54-61`). Mesma fonte/licença CC0 1.0 do perfil v4 acima
/// (`icc/sRGB-v2-magic.icc`, Compact-ICC-Profiles), sha256
/// `af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4`.
const SRGB_V2_ICC_PROFILE: &[u8] = &[
    0, 0, 2, 224, 108, 99, 109, 115, 2, 16, 0, 0, 109, 110, 116, 114, 82, 71, 66, 32, 88,
    89, 90, 32, 7, 226, 0, 3, 0, 20, 0, 9, 0, 14, 0, 29, 97, 99, 115, 112, 77, 83, 70,
    84, 0, 0, 0, 0, 115, 97, 119, 115, 99, 116, 114, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 246, 214, 0, 1, 0, 0, 0, 0, 211, 45, 104, 97, 110, 100, 147, 178, 52,
    169, 14, 176, 34, 138, 152, 253, 154, 175, 163, 103, 137, 155, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 100, 101,
    115, 99, 0, 0, 0, 240, 0, 0, 0, 95, 99, 112, 114, 116, 0, 0, 1, 12, 0, 0, 0, 12, 119,
    116, 112, 116, 0, 0, 1, 24, 0, 0, 0, 20, 114, 88, 89, 90, 0, 0, 1, 44, 0, 0, 0, 20,
    103, 88, 89, 90, 0, 0, 1, 64, 0, 0, 0, 20, 98, 88, 89, 90, 0, 0, 1, 84, 0, 0, 0, 20,
    114, 84, 82, 67, 0, 0, 1, 104, 0, 0, 1, 120, 103, 84, 82, 67, 0, 0, 1, 104, 0, 0, 1,
    120, 98, 84, 82, 67, 0, 0, 1, 104, 0, 0, 1, 120, 100, 101, 115, 99, 0, 0, 0, 0, 0, 0,
    0, 5, 115, 82, 71, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 101, 120, 116, 0, 0,
    0, 0, 67, 67, 48, 0, 88, 89, 90, 32, 0, 0, 0, 0, 0, 0, 243, 84, 0, 1, 0, 0, 0, 1, 22,
    201, 88, 89, 90, 32, 0, 0, 0, 0, 0, 0, 111, 160, 0, 0, 56, 242, 0, 0, 3, 143, 88, 89,
    90, 32, 0, 0, 0, 0, 0, 0, 98, 150, 0, 0, 183, 137, 0, 0, 24, 218, 88, 89, 90, 32, 0,
    0, 0, 0, 0, 0, 36, 160, 0, 0, 15, 133, 0, 0, 182, 196, 99, 117, 114, 118, 0, 0, 0, 0,
    0, 0, 0, 182, 0, 0, 0, 28, 0, 56, 0, 84, 0, 112, 0, 140, 0, 168, 0, 196, 0, 225, 1,
    0, 1, 34, 1, 70, 1, 109, 1, 149, 1, 193, 1, 240, 2, 32, 2, 85, 2, 139, 2, 196, 3, 1,
    3, 63, 3, 130, 3, 198, 4, 14, 4, 89, 4, 167, 4, 249, 5, 76, 5, 164, 5, 254, 6, 92, 6,
    190, 7, 33, 7, 138, 7, 244, 8, 99, 8, 213, 9, 73, 9, 195, 10, 63, 10, 191, 11, 66,
    11, 201, 12, 84, 12, 225, 13, 116, 14, 9, 14, 162, 15, 64, 15, 224, 16, 133, 17, 45,
    17, 218, 18, 138, 19, 62, 19, 246, 20, 178, 21, 113, 22, 54, 22, 253, 23, 202, 24,
    153, 25, 110, 26, 70, 27, 34, 28, 3, 28, 231, 29, 208, 30, 189, 31, 174, 32, 164, 33,
    158, 34, 156, 35, 159, 36, 165, 37, 177, 38, 192, 39, 213, 40, 237, 42, 10, 43, 43,
    44, 81, 45, 124, 46, 170, 47, 222, 49, 22, 50, 82, 51, 148, 52, 217, 54, 36, 55, 115,
    56, 198, 58, 32, 59, 124, 60, 223, 62, 69, 63, 176, 65, 33, 66, 150, 68, 16, 69, 143,
    71, 18, 72, 155, 74, 40, 75, 187, 77, 81, 78, 238, 80, 143, 82, 53, 83, 224, 85, 144,
    87, 69, 89, 0, 90, 190, 92, 132, 94, 76, 96, 27, 97, 239, 99, 199, 101, 166, 103,
    137, 105, 113, 107, 95, 109, 81, 111, 74, 113, 70, 115, 74, 117, 81, 119, 94, 121,
    113, 123, 136, 125, 166, 127, 200, 129, 240, 132, 30, 134, 80, 136, 137, 138, 197,
    141, 9, 143, 81, 145, 159, 147, 243, 150, 75, 152, 171, 155, 14, 157, 120, 159, 231,
    162, 91, 164, 214, 167, 86, 169, 219, 172, 103, 174, 247, 177, 143, 180, 42, 182,
    204, 185, 116, 188, 33, 190, 213, 193, 141, 196, 76, 199, 16, 201, 218, 204, 171,
    207, 127, 210, 92, 213, 61, 216, 36, 219, 18, 222, 4, 224, 254, 227, 252, 231, 1,
    234, 12, 237, 28, 240, 52, 243, 80, 246, 115, 249, 155, 252, 202, 255, 255,
];

/// **P1477** — devolve os bytes do perfil ICC sRGB v2 (736 bytes).
pub(in crate::export) fn srgb_v2_icc_profile_bytes() -> &'static [u8] {
    SRGB_V2_ICC_PROFILE
}

/// **P1477** — seleciona o perfil ICC sRGB pela versão efetiva do PDF,
/// espelhando `krilla/src/configure/version.rs:54-61`: `V1_4|V1_5|V1_6`
/// usam o perfil v2 (736 bytes), `V1_7|V2_0` usam o v4 já vendorizado
/// (P777, 480 bytes). `match` exaustivo — nenhuma variante nova de
/// `PdfVersion` pode cair em wildcard sem decisão explícita.
pub(in crate::export) fn srgb_icc_profile_for_version(
    version: PdfVersion,
) -> &'static [u8] {
    match version {
        PdfVersion::V1_4 | PdfVersion::V1_5 | PdfVersion::V1_6 => {
            srgb_v2_icc_profile_bytes()
        }
        PdfVersion::V1_7 | PdfVersion::V2_0 => srgb_icc_profile_bytes(),
    }
}
/// **P777** — constrói o stream `/ICCBased` para o perfil sRGB partilhado.
/// Inclui `/Range [0 1 0 1 0 1]` para replicar o vanilla/krilla e garantir
/// interpretação determinística dos componentes RGB.
pub(in crate::export) fn build_icc_profile_stream(icc_data: &[u8]) -> Vec<u8> {
    let compressed = compress_zlib(icc_data).unwrap_or_else(|_| icc_data.to_vec());
    let len = compressed.len();
    let header = format!(
        "<< /Length {len} /N 3 /Range [0 1 0 1 0 1] /Filter /FlateDecode >>\nstream\n"
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(&compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn icc_versions_and_profile_headers_are_preserved() {
        let v2 = srgb_v2_icc_profile_bytes();
        assert_eq!(u32::from_be_bytes(v2[..4].try_into().unwrap()) as usize, v2.len());
        let v4 = srgb_icc_profile_bytes();
        assert_eq!(u32::from_be_bytes(v4[..4].try_into().unwrap()) as usize, v4.len());
        for version in [PdfVersion::V1_4, PdfVersion::V1_5, PdfVersion::V1_6] {
            assert_eq!(srgb_icc_profile_for_version(version), v2);
        }
        for version in [PdfVersion::V1_7, PdfVersion::V2_0] {
            assert_eq!(srgb_icc_profile_for_version(version), srgb_icc_profile_bytes());
        }
        assert_eq!(&v2[36..40], b"acsp");
        assert_eq!(v2[8], 2);
        assert_eq!(srgb_icc_profile_bytes()[8], 4);
    }

    #[test]
    fn icc_stream_roundtrips_profile_bytes() {
        let profile = srgb_icc_profile_bytes();
        let object = build_icc_profile_stream(profile);
        let start = object.windows(8).position(|w| w == b"\nstream\n").unwrap() + 8;
        let compressed = &object[start..object.len() - 10];
        let header = std::str::from_utf8(&object[..start]).unwrap();
        assert!(header.contains("/N 3 /Range [0 1 0 1 0 1]"));
        assert!(header.contains(&format!("/Length {} ", compressed.len())));
        let mut decoded = Vec::new();
        flate2::read::ZlibDecoder::new(compressed)
            .read_to_end(&mut decoded)
            .unwrap();
        assert_eq!(decoded, profile);
    }
}
