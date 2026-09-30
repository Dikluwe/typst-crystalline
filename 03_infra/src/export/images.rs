//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images.md
//! @prompt-hash 8cd14566
//! @layer L3
//! @updated 2026-09-26

//! Composição e tipos de imagens PDF; famílias proprietárias em images/.

use flate2::{write::ZlibEncoder, Compression};
use std::io::Write;
use std::sync::Arc;

mod icc;
mod jpeg;
mod png;
mod scan;
mod xobjects;

pub(super) use icc::{build_icc_profile_stream, srgb_icc_profile_for_version};
#[allow(unused_imports)] // Preserve the existing export-private getter paths.
pub(super) use icc::{srgb_icc_profile_bytes, srgb_v2_icc_profile_bytes};
pub(super) use jpeg::{jpeg_color_space, jpeg_is_rgb};
pub use png::process_png_for_pdf;
pub(crate) use scan::validate_document_images;
pub(super) use scan::{scan_all_images, xobject_resources_for_page};
pub(super) use typst_core::entities::image_format::{detect_image_format, ImageFormat};
pub(super) use xobjects::{
    build_jpeg_xobject, build_png_rgb_xobject, build_png_smask_xobject,
};

/// Dados de imagem PNG prontos para emissão como XObject(s) num PDF.
#[derive(Clone)]
pub struct PdfImagePayload {
    pub width: u32,
    pub height: u32,
    /// "/DeviceRGB" ou "/DeviceGray" — determinado pelos dados da imagem.
    pub color_space: &'static str,
    /// Canal de cor comprimido com Zlib (/FlateDecode).
    pub rgb_data_compressed: Vec<u8>,
    /// Canal alpha comprimido com Zlib, se a imagem tiver transparência não trivial.
    /// `None` se opaca ou sem canal alpha.
    pub alpha_data_compressed: Option<Vec<u8>>,
}

pub(super) fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data).map_err(|e| e.to_string())?;
    enc.finish().map_err(|e| e.to_string())
}

/// Metadados de imagem para resource dict e page streams.
pub(crate) struct ImageRef {
    pub(super) main_obj_id: usize,
    pub(super) name: String,
}
/// Dados para emissão de XObjects no PDF.
pub(super) enum ImageXObject {
    Jpeg {
        data: Arc<Vec<u8>>,
        main_obj_id: usize,
        iw: u32,
        ih: u32,
        /// **P777** — ID do perfil ICC sRGB partilhado (`None` para grayscale/CMYK).
        icc_profile_id: Option<usize>,
    },
    Png {
        payload: PdfImagePayload,
        main_obj_id: usize,
        smask_obj_id: Option<usize>,
    },
}
