//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/xobjects.md
//! @prompt-hash e5e8cf06
//! @layer L3
//! @updated 2026-09-26

use super::PdfImagePayload;

pub(in crate::export) fn build_jpeg_xobject(
    data: &[u8],
    iw: u32,
    ih: u32,
    color_space: &str,
    icc_profile_id: Option<usize>,
) -> Vec<u8> {
    let len = data.len();
    let cs_entry = match icc_profile_id {
        Some(id) => format!("/ColorSpace [/ICCBased {id} 0 R]"),
        None => format!("/ColorSpace {color_space}"),
    };
    let header = format!(
        "<< /Type /XObject /Subtype /Image \
           /Width {iw} /Height {ih} \
           {cs_entry} /BitsPerComponent 8 \
           /Filter /DCTDecode /Length {len} >>\nstream\n"
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(data);
    obj.extend_from_slice(b"\nendstream");
    obj
}

/// Stream XObject para o canal alpha de um PNG (/DeviceGray, /FlateDecode).
pub(in crate::export) fn build_png_smask_xobject(
    w: u32,
    h: u32,
    alpha_compressed: &[u8],
) -> Vec<u8> {
    let len = alpha_compressed.len();
    let header = format!(
        "<< /Type /XObject /Subtype /Image \
           /Width {w} /Height {h} \
           /ColorSpace /DeviceGray /BitsPerComponent 8 \
           /Filter /FlateDecode /Length {len} >>\nstream\n"
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(alpha_compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
}

/// Stream XObject para o canal RGB de um PNG (/DeviceRGB, /FlateDecode).
/// Referencia o /SMask pelo seu ID se a imagem tiver transparência.
pub(in crate::export) fn build_png_rgb_xobject(
    payload: &PdfImagePayload,
    smask_obj_id: Option<usize>,
) -> Vec<u8> {
    let len = payload.rgb_data_compressed.len();
    let smask_entry = match smask_obj_id {
        Some(id) => format!("/SMask {id} 0 R "),
        None => String::new(),
    };
    let header = format!(
        "<< /Type /XObject /Subtype /Image \
           /Width {w} /Height {h} \
           /ColorSpace {cs} /BitsPerComponent 8 \
           {smask_entry}/Filter /FlateDecode /Length {len} >>\nstream\n",
        w = payload.width,
        h = payload.height,
        cs = payload.color_space,
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(&payload.rgb_data_compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xobjects_preserve_binary_streams_and_pdf_references() {
        let bytes = [0xff, 0, 0x80, 13, 10];
        let jpeg = build_jpeg_xobject(&bytes, 7, 9, "/DeviceCMYK", None);
        assert!(jpeg.ends_with(&[&bytes[..], b"\nendstream"].concat()));
        let header = String::from_utf8_lossy(&jpeg[..jpeg.len() - bytes.len() - 10]);
        assert!(header.contains("/Width 7 /Height 9"));
        assert!(header.contains("/ColorSpace /DeviceCMYK"));
        assert!(header.contains("/Length 5"));
        let icc = build_jpeg_xobject(&bytes, 7, 9, "/DeviceRGB", Some(6));
        assert!(String::from_utf8_lossy(&icc).contains("/ColorSpace [/ICCBased 6 0 R]"));
        let payload = PdfImagePayload {
            width: 7,
            height: 9,
            color_space: "/DeviceRGB",
            rgb_data_compressed: bytes.to_vec(),
            alpha_data_compressed: Some(bytes.to_vec()),
        };
        let rgb = build_png_rgb_xobject(&payload, Some(12));
        assert!(rgb.ends_with(&[&bytes[..], b"\nendstream"].concat()));
        assert!(String::from_utf8_lossy(&rgb).contains("/SMask 12 0 R"));
        assert!(!String::from_utf8_lossy(&build_png_rgb_xobject(&payload, None))
            .contains("/SMask"));
        let mask = build_png_smask_xobject(7, 9, &bytes);
        assert!(String::from_utf8_lossy(&mask).contains("/ColorSpace /DeviceGray"));
        assert!(mask.ends_with(&[&bytes[..], b"\nendstream"].concat()));
    }
}
