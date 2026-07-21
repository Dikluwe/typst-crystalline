//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images.md
//! @prompt-hash 11940dcb
//! @layer L3
//! @updated 2026-05-19
//!
//! Suporte a imagens no exporter PDF — detecção de formato (JPEG/PNG),
//! compressão Zlib, deduplicação via `Arc::as_ptr`, e XObject builders
//! por formato.
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Reusado por:
//! - `mod.rs::PdfBuilder` (scan_all_images, emit_image_xobjects).
//! - `stream/draw.rs` (xobject_resources_for_page, image emit).
//!
//! Conteúdo bit-exact pré e pós migração.

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::sync::Arc;

use flate2::write::ZlibEncoder;
use flate2::Compression;

pub(super) use typst_core::entities::image_format::{detect_image_format, ImageFormat};
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

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
pub(super) fn srgb_icc_profile_bytes() -> &'static [u8] {
    SRGB_ICC_PROFILE
}

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
pub(super) fn jpeg_color_space(data: &[u8]) -> &'static str {
    match jpeg_components(data) {
        1 => "/DeviceGray",
        4 => "/DeviceCMYK",
        _ => "/DeviceRGB",
    }
}

/// **P777** — verdadeiro se o JPEG for RGB (3 componentes).
pub(super) fn jpeg_is_rgb(data: &[u8]) -> bool {
    jpeg_components(data) == 3
}

/// Dados de imagem PNG prontos para emissão como XObject(s) num PDF.
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

/// Descodifica um PNG e prepara os dados para emissão como XObject(s) num PDF.
///
/// **Sem alpha**: converte para RGB8, comprime os bytes planos com Zlib.
/// **Com alpha**: separa os canais RGB e A, comprime ambos separadamente.
///   Se o canal A for totalmente opaco (todos 255), descarta-o — um /SMask
///   com alpha uniforme não tem efeito visual e aumenta o PDF desnecessariamente.
pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String> {
    let img = image::load_from_memory(raw_data)
        .map_err(|e| format!("Falha ao descodificar imagem: {}", e))?;

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

/// Varre o documento e pré-processa todas as imagens únicas (JPEG e PNG).
///
/// A deduplicação usa `Arc::as_ptr(data) as usize` como chave — seguro porque
/// `PagedDocument` mantém todos os Arcs vivos durante `export_pdf`, impedindo
/// que o alocador reutilize os mesmos endereços.
///
/// Retorna `(refs, ptr_to_idx, xobjects)`:
/// - `refs`: metadados name/obj_id por imagem (para resource dict e page stream)
/// - `ptr_to_idx`: `arc_ptr → índice em refs`
/// - `xobjects`: dados para emissão de XObjects (na mesma ordem que refs)
pub(super) fn scan_all_images(
    doc: &PagedDocument,
    first_id: usize,
    icc_profile_id: Option<usize>,
) -> (Vec<ImageRef>, HashMap<usize, usize>, Vec<ImageXObject>) {
    // P279 — helper recursivo (scope creep análogo P273.10 §A.7 para scan_all_gradients).
    // Bug latent pré-existente: scan_all_images iterava apenas page.items top-level;
    // Images dentro de Groups (via Content::Transform / Block clip / etc.) não eram
    // registadas. Sem fix, Image arm em draw_item_local (P279) teria nada a lookup.
    fn walk(
        items: &[FrameItem],
        ptr_to_idx: &mut HashMap<usize, usize>,
        refs: &mut Vec<ImageRef>,
        xobjects: &mut Vec<ImageXObject>,
        next_id: &mut usize,
        counter: &mut usize,
        icc_profile_id: Option<usize>,
    ) {
        for item in items {
            match item {
                FrameItem::Image {
                    data: _,
                    intrinsic_width: _,
                    intrinsic_height: _,
                    ..
                } => {
                    process_image_item(
                        item,
                        ptr_to_idx,
                        refs,
                        xobjects,
                        next_id,
                        counter,
                        icc_profile_id,
                    );
                }
                FrameItem::Group { items: child_items, .. }
                | FrameItem::Link { items: child_items, .. } => {
                    walk(
                        child_items,
                        ptr_to_idx,
                        refs,
                        xobjects,
                        next_id,
                        counter,
                        icc_profile_id,
                    );
                }
                _ => {}
            }
        }
    }

    let mut ptr_to_idx: HashMap<usize, usize> = HashMap::new();
    let mut refs: Vec<ImageRef> = Vec::new();
    let mut xobjects: Vec<ImageXObject> = Vec::new();
    let mut next_id = first_id;
    let mut counter = 1usize;

    for page in &doc.pages {
        walk(
            &page.items,
            &mut ptr_to_idx,
            &mut refs,
            &mut xobjects,
            &mut next_id,
            &mut counter,
            icc_profile_id,
        );
    }
    (refs, ptr_to_idx, xobjects)
}

/// **P279 — Helper privado**: processa um único `FrameItem::Image` (detecta
/// formato JPEG/PNG, aloca ObjectIDs, regista em refs/xobjects/ptr_to_idx).
/// Extraído do corpo de `scan_all_images` para permitir recursão em
/// `walk` sem duplicar lógica.
fn process_image_item(
    item: &FrameItem,
    ptr_to_idx: &mut HashMap<usize, usize>,
    refs: &mut Vec<ImageRef>,
    xobjects: &mut Vec<ImageXObject>,
    next_id: &mut usize,
    counter: &mut usize,
    icc_profile_id: Option<usize>,
) {
    let FrameItem::Image { data, intrinsic_width, intrinsic_height, .. } = item else {
        return;
    };
    let ptr = Arc::as_ptr(data) as usize;
    if ptr_to_idx.contains_key(&ptr) {
        return;
    }
    let idx = refs.len();
    let name = format!("Im{}", *counter);
    *counter += 1;

    match detect_image_format(data) {
        ImageFormat::Jpeg => {
            let main_id = *next_id;
            *next_id += 1;
            let image_icc = if jpeg_is_rgb(data) { icc_profile_id } else { None };
            refs.push(ImageRef { main_obj_id: main_id, name });
            xobjects.push(ImageXObject::Jpeg {
                data: Arc::clone(data),
                main_obj_id: main_id,
                iw: *intrinsic_width,
                ih: *intrinsic_height,
                icc_profile_id: image_icc,
            });
            ptr_to_idx.insert(ptr, idx);
        }
        ImageFormat::Png => {
            match process_png_for_pdf(data) {
                Ok(payload) => {
                    // Alocar ID do /SMask antes do ID principal para que smask
                    // apareça primeiro no ficheiro PDF (xref em ordem crescente).
                    let smask_id = if payload.alpha_data_compressed.is_some() {
                        let id = *next_id;
                        *next_id += 1;
                        Some(id)
                    } else {
                        None
                    };
                    let main_id = *next_id;
                    *next_id += 1;
                    refs.push(ImageRef { main_obj_id: main_id, name });
                    xobjects.push(ImageXObject::Png {
                        payload,
                        main_obj_id: main_id,
                        smask_obj_id: smask_id,
                    });
                    ptr_to_idx.insert(ptr, idx);
                }
                Err(e) => {
                    eprintln!("PNG inválido — imagem omitida: {}", e);
                    // Não inserir em ptr_to_idx — imagem ignorada nas páginas.
                }
            }
        }
        ImageFormat::Unknown => {
            eprintln!("Formato de imagem desconhecido — imagem omitida");
        }
    }
}

/// Constrói o fragmento `/XObject << /Im1 X 0 R ... >>` para os recursos de página.
/// Retorna string vazia se não houver imagens na página.
///
/// **P279** — recursivo em Groups (análogo à recursão de `scan_all_images`):
/// Images dentro de Groups (Block clip / Transform) também têm de aparecer
/// no `/XObject` dict da página, senão o `/Im1 Do` emitido em `draw_item_local`
/// fica órfão e o PDF reader não consegue resolver a referência.
pub(super) fn xobject_resources_for_page(
    page: &Page,
    ptr_to_idx: &HashMap<usize, usize>,
    refs: &[ImageRef],
) -> String {
    fn walk(
        items: &[FrameItem],
        ptr_to_idx: &HashMap<usize, usize>,
        refs: &[ImageRef],
        seen: &mut BTreeSet<usize>,
        entries: &mut Vec<String>,
    ) {
        for item in items {
            match item {
                FrameItem::Image { data, .. } => {
                    let ptr = Arc::as_ptr(data) as usize;
                    if let Some(&idx) = ptr_to_idx.get(&ptr) {
                        if seen.insert(idx) {
                            let r = &refs[idx];
                            entries.push(format!("/{} {} 0 R", r.name, r.main_obj_id));
                        }
                    }
                }
                FrameItem::Group { items: child_items, .. } => {
                    walk(child_items, ptr_to_idx, refs, seen, entries);
                }
                _ => {}
            }
        }
    }

    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    walk(&page.items, ptr_to_idx, refs, &mut seen, &mut entries);
    if entries.is_empty() {
        return String::new();
    }
    format!("/XObject << {} >>", entries.join(" "))
}

// ── XObject builders por formato ────────────────────────────────────────────

/// **P777** — constrói o stream `/ICCBased` para o perfil sRGB partilhado.
/// Inclui `/Range [0 1 0 1 0 1]` para replicar o vanilla/krilla e garantir
/// interpretação determinística dos componentes RGB.
pub(super) fn build_icc_profile_stream(icc_data: &[u8]) -> Vec<u8> {
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

pub(super) fn build_jpeg_xobject(
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
pub(super) fn build_png_smask_xobject(
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
pub(super) fn build_png_rgb_xobject(
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
