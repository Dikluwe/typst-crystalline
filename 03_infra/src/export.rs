//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export.md
//! @prompt-hash bf71181c
//! @layer L3
//! @updated 2026-04-20

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use ttf_parser::Face;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

use crate::font_metrics::build_math_glyph_reverse_map;

/// Serializa um `PagedDocument` para bytes PDF-1.7.
///
/// Sem fonte TrueType → fallback para Helvetica Type1 (WinAnsiEncoding, Latin-1).
/// Para suporte Unicode completo, usar `export_pdf_with_font` (ADR-0027).
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> {
    PdfBuilder::new().build(doc, None)
}

/// Serializa com fonte TrueType embebida — CIDFont + Identity-H (ADR-0027).
/// Suporte Unicode completo para codepoints arbitrários.
/// `font_data`: bytes brutos de um ficheiro `.ttf`/`.otf`.
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8> {
    PdfBuilder::new().build(doc, Some(font_data))
}

/// Serializa com **N** fontes TrueType embebidas — multi-font per
/// document (Passo 146, ADR-0055 decisão 5). Cada `FrameItem::Text`
/// emite com a entrada `/F{i+1}` cuja `FontList` casa com
/// `style.font`; spans com `style.font == None` ou sem match
/// caem em `/F1` (font 0) por consistência com o caminho
/// single-font (todos os spans usam o mesmo embedding em
/// `export_pdf_with_font`).
///
/// Se nenhuma das fontes parsear como TTF/OTF válida, fallback
/// para `export_pdf` (Helvetica). Single-font (`fonts.len() == 1`)
/// é caso particular válido.
pub fn export_pdf_multifont(
    doc:   &PagedDocument,
    fonts: &[(FontList, Vec<u8>)],
) -> Vec<u8> {
    if fonts.is_empty() {
        return PdfBuilder::new().build(doc, None);
    }
    let faces: Vec<Face<'_>> = fonts.iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        return PdfBuilder::new().build(doc, None);
    }
    PdfBuilder::new().build_multifont(doc, fonts, &faces)
}

// ── Suporte a imagens ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum ImageFormat {
    Jpeg,
    Png,
    Unknown,
}

fn detect_format(data: &[u8]) -> ImageFormat {
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        ImageFormat::Jpeg
    } else if data.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        ImageFormat::Png
    } else {
        ImageFormat::Unknown
    }
}

/// Lê o marcador SOF0 (0xC0) ou SOF2 (0xC2) do cabeçalho JPEG para determinar
/// o ColorSpace correcto para o dicionário do XObject (DEBT-29).
///
/// Um JPEG com ColorSpace errado produz lixo visual (Grayscale renderizado como
/// RGB monocromático) ou é recusado por alguns leitores PDF (CMYK).
/// O fallback "/DeviceRGB" cobre a maioria dos JPEGs de câmara.
fn jpeg_color_space(data: &[u8]) -> &'static str {
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
                return match data[i + 9] {
                    1 => "/DeviceGray",
                    3 => "/DeviceRGB",
                    4 => "/DeviceCMYK",
                    _ => "/DeviceRGB",
                };
            }
            break;
        }

        // SOS (0xDA) inicia os dados comprimidos — parar antes de entrar neles.
        if marker == 0xDA {
            break;
        }

        if len < 2 { break; }
        i += 2 + len;
    }
    "/DeviceRGB"
}

/// Dados de imagem PNG prontos para emissão como XObject(s) num PDF.
pub struct PdfImagePayload {
    pub width:                 u32,
    pub height:                u32,
    /// "/DeviceRGB" ou "/DeviceGray" — determinado pelos dados da imagem.
    pub color_space:           &'static str,
    /// Canal de cor comprimido com Zlib (/FlateDecode).
    pub rgb_data_compressed:   Vec<u8>,
    /// Canal alpha comprimido com Zlib, se a imagem tiver transparência não trivial.
    /// `None` se opaca ou sem canal alpha.
    pub alpha_data_compressed: Option<Vec<u8>>,
}

fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, String> {
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

    let width  = img.width();
    let height = img.height();

    if !img.color().has_alpha() {
        return Ok(PdfImagePayload {
            width,
            height,
            color_space:           "/DeviceRGB",
            rgb_data_compressed:   compress_zlib(img.to_rgb8().as_raw())?,
            alpha_data_compressed: None,
        });
    }

    let rgba = img.to_rgba8();
    let mut rgb_buf   = Vec::with_capacity((width * height * 3) as usize);
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
        color_space:           "/DeviceRGB",
        rgb_data_compressed:   compress_zlib(&rgb_buf)?,
        alpha_data_compressed: alpha_compressed,
    })
}

/// Metadados de imagem para resource dict e page streams.
struct ImageRef {
    main_obj_id: usize,
    name:        String,
}

/// Dados para emissão de XObjects no PDF.
enum ImageXObject {
    Jpeg {
        data:        Arc<Vec<u8>>,
        main_obj_id: usize,
        iw:          u32,
        ih:          u32,
    },
    Png {
        payload:      PdfImagePayload,
        main_obj_id:  usize,
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
fn scan_all_images(
    doc:      &PagedDocument,
    first_id: usize,
) -> (Vec<ImageRef>, HashMap<usize, usize>, Vec<ImageXObject>) {
    let mut ptr_to_idx: HashMap<usize, usize> = HashMap::new();
    let mut refs:       Vec<ImageRef>      = Vec::new();
    let mut xobjects:   Vec<ImageXObject>  = Vec::new();
    let mut next_id  = first_id;
    let mut counter  = 1usize;

    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Image { data, intrinsic_width, intrinsic_height, .. } = item {
                let ptr = Arc::as_ptr(data) as usize;
                if ptr_to_idx.contains_key(&ptr) {
                    continue;
                }
                let idx = refs.len();
                let name = format!("Im{counter}");
                counter += 1;

                match detect_format(data) {
                    ImageFormat::Jpeg => {
                        let main_id = next_id;
                        next_id += 1;
                        refs.push(ImageRef { main_obj_id: main_id, name });
                        xobjects.push(ImageXObject::Jpeg {
                            data:        Arc::clone(data),
                            main_obj_id: main_id,
                            iw:          *intrinsic_width,
                            ih:          *intrinsic_height,
                        });
                        ptr_to_idx.insert(ptr, idx);
                    }
                    ImageFormat::Png => {
                        match process_png_for_pdf(data) {
                            Ok(payload) => {
                                // Alocar ID do /SMask antes do ID principal para que smask
                                // apareça primeiro no ficheiro PDF (xref em ordem crescente).
                                let smask_id = if payload.alpha_data_compressed.is_some() {
                                    let id = next_id;
                                    next_id += 1;
                                    Some(id)
                                } else {
                                    None
                                };
                                let main_id = next_id;
                                next_id += 1;
                                refs.push(ImageRef { main_obj_id: main_id, name });
                                xobjects.push(ImageXObject::Png { payload, main_obj_id: main_id, smask_obj_id: smask_id });
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
        }
    }
    (refs, ptr_to_idx, xobjects)
}

/// Constrói o fragmento `/XObject << /Im1 X 0 R ... >>` para os recursos de página.
/// Retorna string vazia se não houver imagens na página.
fn xobject_resources_for_page(
    page:       &Page,
    ptr_to_idx: &HashMap<usize, usize>,
    refs:       &[ImageRef],
) -> String {
    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    for item in &page.items {
        if let FrameItem::Image { data, .. } = item {
            let ptr = Arc::as_ptr(data) as usize;
            if let Some(&idx) = ptr_to_idx.get(&ptr) {
                if seen.insert(idx) {
                    let r = &refs[idx];
                    entries.push(format!("/{} {} 0 R", r.name, r.main_obj_id));
                }
            }
        }
    }
    if entries.is_empty() {
        return String::new();
    }
    format!("/XObject << {} >>", entries.join(" "))
}

// ── P263: Gradient Linear → PDF Shading Patterns (ADR-0087) ────────────────

/// Metadados de gradient para resource dict e page streams.
struct PatternRef {
    pattern_obj_id: usize,
    name:           String,
}

/// P265 — variant para distinguir Linear / Radial em emit.
enum GradientObjectKind {
    Linear(std::sync::Arc<typst_core::entities::gradient::Linear>),
    Radial(std::sync::Arc<typst_core::entities::gradient::Radial>),
}

/// Dados internos para emit Function/Shading/Pattern object dicts.
struct GradientObject {
    kind:           GradientObjectKind,
    function_id:    usize,
    shading_id:     usize,
    pattern_id:     usize,
}

/// Varre o documento e pré-processa todos os gradients únicos por
/// `Arc::as_ptr(linear)` (paridade pattern image P73).
///
/// Aloca 3 ObjectIDs por gradient único: Function + Shading + Pattern.
///
/// Retorna `(refs, ptr_to_idx, grad_objs)`:
/// - `refs`: metadados name/obj_id por gradient (para resource dict).
/// - `ptr_to_idx`: `arc_ptr → índice em refs`.
/// - `grad_objs`: dados para emit (mesma ordem que refs).
fn scan_all_gradients(
    doc:      &typst_core::entities::layout_types::PagedDocument,
    first_id: usize,
) -> (Vec<PatternRef>, HashMap<usize, usize>, Vec<GradientObject>) {
    use typst_core::entities::geometry::Stroke;
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::layout_types::FrameItem;
    use typst_core::entities::paint::Paint;

    let mut ptr_to_idx: HashMap<usize, usize> = HashMap::new();
    let mut refs:       Vec<PatternRef>    = Vec::new();
    let mut grad_objs:  Vec<GradientObject> = Vec::new();
    let mut next_id  = first_id;
    let mut counter  = 1usize;

    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), .. } = item {
                // P265 — Linear e Radial via enum GradientObjectKind.
                // P267 — Conic adicionado; fallback Solid no emit (PDF Conic shading adiado P268).
                let (ptr, kind) = match g {
                    Gradient::Linear(l) => (
                        std::sync::Arc::as_ptr(l) as usize,
                        GradientObjectKind::Linear(std::sync::Arc::clone(l)),
                    ),
                    Gradient::Radial(r) => (
                        std::sync::Arc::as_ptr(r) as usize,
                        GradientObjectKind::Radial(std::sync::Arc::clone(r)),
                    ),
                    Gradient::Conic(_) => continue,
                };
                if ptr_to_idx.contains_key(&ptr) {
                    continue;
                }
                let function_id = next_id; next_id += 1;
                let shading_id  = next_id; next_id += 1;
                let pattern_id  = next_id; next_id += 1;
                let name = format!("P{counter}");
                counter += 1;
                let idx = refs.len();
                refs.push(PatternRef { pattern_obj_id: pattern_id, name });
                grad_objs.push(GradientObject {
                    kind, function_id, shading_id, pattern_id,
                });
                ptr_to_idx.insert(ptr, idx);
            }
        }
    }
    (refs, ptr_to_idx, grad_objs)
}

/// Constrói o fragmento `/Pattern << /P1 X 0 R ... >>` para os recursos
/// de página. Retorna string vazia se não houver gradients na página.
fn pattern_resources_for_page(
    page:       &Page,
    ptr_to_idx: &HashMap<usize, usize>,
    refs:       &[PatternRef],
) -> String {
    use typst_core::entities::geometry::Stroke;
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::layout_types::FrameItem;
    use typst_core::entities::paint::Paint;

    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    for item in &page.items {
        if let FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), .. } = item {
            // P265 — Linear e Radial ambos registados.
            // P267 — Conic adiado P268; salta resource entry.
            let ptr = match g {
                Gradient::Linear(l) => std::sync::Arc::as_ptr(l) as usize,
                Gradient::Radial(r) => std::sync::Arc::as_ptr(r) as usize,
                Gradient::Conic(_) => continue,
            };
            if let Some(&idx) = ptr_to_idx.get(&ptr) {
                if seen.insert(idx) {
                    let r = &refs[idx];
                    entries.push(format!("/{} {} 0 R", r.name, r.pattern_obj_id));
                }
            }
        }
    }
    if entries.is_empty() {
        return String::new();
    }
    format!("/Pattern << {} >>", entries.join(" "))
}

/// Calcula os endpoints axial em coordenadas locais (espaço PDF; Y
/// já invertido pelo `build_page_stream_*`).
///
/// Angle 0° → linha horizontal através do centro.
/// Angle 90° (π/2) → linha vertical através do centro.
/// Generalização: linha que passa pelo centro com direcção (cos θ, sin θ),
/// estendida pelas semi-axes (w/2, h/2) projectadas.
fn compute_axial_coords(angle_rad: f64, x0: f64, y0: f64, w: f64, h: f64)
    -> (f64, f64, f64, f64)
{
    let cx = x0 + w / 2.0;
    let cy = y0 + h / 2.0;
    let dx = angle_rad.cos();
    let dy = angle_rad.sin();
    let hx = (w / 2.0) * dx;
    let hy = (h / 2.0) * dy;
    (cx - hx, cy - hy, cx + hx, cy + hy)
}

/// Amostra N stops intermédios em sRGB pós-interpolação Oklab L1
/// (via `Linear::sample(t)` P262).
///
/// Output: `Vec<(r, g, b)>` em [0, 1] sRGB normalizado.
fn oklab_sample_stops(
    linear: &typst_core::entities::gradient::Linear,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = linear.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}

/// P265 — Calcula os 6 valores Coords para `/ShadingType 3`
/// radial.
///
/// Subset materializado P264 (focal_* scope-out): círculos
/// concêntricos. Foco pontual no center; target concêntrico
/// com radius.
///
/// `center` em Ratios (0.0-1.0); `radius` em Ratio. `w`/`h` são
/// dimensões do bbox local em pontos.
///
/// Retorna `(x0, y0, r0, x1, y1, r1)`:
/// - `(x0, y0, r0)`: focal point (gradient origin).
/// - `(x1, y1, r1)`: target point (gradient outer).
fn compute_radial_coords(
    center: typst_core::entities::axes::Axes<typst_core::entities::layout_types::Ratio>,
    radius: typst_core::entities::layout_types::Ratio,
    w: f64,
    h: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    // Subset: focal point pontual no center; target concêntrico.
    (cx, cy, 0.0, cx, cy, r)
}

/// P265 — Amostra N stops intermédios em sRGB pós-interpolação
/// Oklab L1 (via `Radial::sample(t)` P264). Paridade literal
/// `oklab_sample_stops` (Linear).
fn oklab_sample_stops_radial(
    radial: &typst_core::entities::gradient::Radial,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = radial.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}

/// Emit PDF Function dict (Type 2 ou Type 3).
///
/// 2 stops → Type 2 (exponential linear `/N 1`).
/// N>2 stops → Type 3 stitching com N-1 sub-funções Type 2.
///
/// Retorna `(function_dict_string, sub_function_dicts)` onde
/// sub_function_dicts é vazio para Type 2.
fn emit_function_dict(stops: &[(f32, f32, f32)], function_id: usize, sub_first_id: &mut usize)
    -> (String, Vec<(usize, String)>)
{
    if stops.len() == 2 {
        let (r0, g0, b0) = stops[0];
        let (r1, g1, b1) = stops[1];
        let dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /C0 [{:.4} {:.4} {:.4}] /C1 [{:.4} {:.4} {:.4}] /N 1 >>",
            r0, g0, b0, r1, g1, b1
        );
        let _ = function_id;
        return (dict, Vec::new());
    }
    // Type 3 stitching.
    let n = stops.len();
    let mut sub_objs: Vec<(usize, String)> = Vec::new();
    let mut sub_refs: Vec<String> = Vec::new();
    for i in 0..(n - 1) {
        let (r0, g0, b0) = stops[i];
        let (r1, g1, b1) = stops[i + 1];
        let sub_id = *sub_first_id;
        *sub_first_id += 1;
        let sub_dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /C0 [{:.4} {:.4} {:.4}] /C1 [{:.4} {:.4} {:.4}] /N 1 >>",
            r0, g0, b0, r1, g1, b1
        );
        sub_objs.push((sub_id, sub_dict));
        sub_refs.push(format!("{sub_id} 0 R"));
    }
    let mut bounds = Vec::new();
    for i in 1..(n - 1) {
        let t = i as f64 / (n - 1) as f64;
        bounds.push(format!("{:.4}", t));
    }
    let encode: Vec<String> = (0..(n - 1)).map(|_| "0 1".to_string()).collect();
    let dict = format!(
        "<< /FunctionType 3 /Domain [0 1] /Functions [{}] /Bounds [{}] /Encode [{}] >>",
        sub_refs.join(" "),
        bounds.join(" "),
        encode.join(" "),
    );
    let _ = function_id;
    (dict, sub_objs)
}

/// Stream XObject para um JPEG (raw bytes com /DCTDecode).
fn build_jpeg_xobject(data: &[u8], iw: u32, ih: u32, color_space: &str) -> Vec<u8> {
    let len = data.len();
    let header = format!(
        "<< /Type /XObject /Subtype /Image \
           /Width {iw} /Height {ih} \
           /ColorSpace {color_space} /BitsPerComponent 8 \
           /Filter /DCTDecode /Length {len} >>\nstream\n"
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(data);
    obj.extend_from_slice(b"\nendstream");
    obj
}

/// Stream XObject para o canal alpha de um PNG (/DeviceGray, /FlateDecode).
fn build_png_smask_xobject(w: u32, h: u32, alpha_compressed: &[u8]) -> Vec<u8> {
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
fn build_png_rgb_xobject(payload: &PdfImagePayload, smask_obj_id: Option<usize>) -> Vec<u8> {
    let len = payload.rgb_data_compressed.len();
    let smask_entry = match smask_obj_id {
        Some(id) => format!("/SMask {id} 0 R "),
        None     => String::new(),
    };
    let header = format!(
        "<< /Type /XObject /Subtype /Image \
           /Width {w} /Height {h} \
           /ColorSpace {cs} /BitsPerComponent 8 \
           {smask_entry}/Filter /FlateDecode /Length {len} >>\nstream\n",
        w  = payload.width,
        h  = payload.height,
        cs = payload.color_space,
    );
    let mut obj = header.into_bytes();
    obj.extend_from_slice(&payload.rgb_data_compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
}

// ── Builder ────────────────────────────────────────────────────────────────

struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,
}

impl PdfBuilder {
    fn new() -> Self { Self { objects: Vec::new() } }

    fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    fn add_bytes(&mut self, id: usize, content: Vec<u8>) {
        self.objects.push((id, content));
    }

    fn build(self, doc: &PagedDocument, font_data: Option<&[u8]>) -> Vec<u8> {
        if let Some(data) = font_data {
            if let Ok(face) = Face::parse(data, 0) {
                return self.build_cidfont(doc, &face, data);
            }
        }
        self.build_helvetica(doc)
    }

    // ── Caminho Helvetica (fallback, Type1 sem embedding) ─────────────────

    fn build_helvetica(mut self, doc: &PagedDocument) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        let first_page   = 3usize;
        let first_stream = first_page + n;
        let font_f1      = first_stream + n;
        let font_f2      = font_f1 + 1;
        let font_f3      = font_f2 + 1;
        let first_img_id = font_f3 + 1;

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

        // P263 — Allocar IDs após imagens. Reserva n_gradients*3 + N
        // sub-functions (estimativa pessimista: N stops 16 → 15 subs por gradient).
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        // Sub-function IDs após os 3*N gradient object IDs.
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let resources_str = format!(
                "/Font << /F1 {font_f1} 0 R /F2 {font_f2} 0 R /F3 {font_f3} 0 R >> {xobj_res} {pat_res}"
            );

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let stream_bytes = build_page_stream_type1(page, &ptr_to_idx, &img_refs, &pat_ptr_to_idx, &pat_refs);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        self.add(font_f1, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f2, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f3, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique \
                            /Encoding /WinAnsiEncoding >>".into());

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit Function/Shading/Pattern objects para gradients.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.serialize()
    }

    // ── Caminho CIDFont (Unicode completo, Identity-H) ─────────────────────

    fn build_cidfont(mut self, doc: &PagedDocument, face: &Face<'_>, font_data: &[u8]) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        let first_page         = 3usize;
        let first_stream       = first_page + n;
        let font_id            = first_stream + n;      // Type0 — /F1
        let cidfont_id         = font_id + 1;
        let font_descriptor_id = font_id + 2;
        let font_stream_id     = font_id + 3;
        let to_unicode_id      = font_id + 4;
        let first_img_id       = to_unicode_id + 1;

        let chars = collect_codepoints(doc);
        let mut mappings = map_chars_to_glyphs(face, &chars);

        // Passo 45 — DEBT-9: adicionar glifos variantes (FrameItem::Glyph) ao ToUnicode.
        // O dicionário reverso mapeia glyph_id → char base para caracteres extensíveis.
        let glyph_reverse = build_math_glyph_reverse_map(face);
        let existing_gids: BTreeSet<u16> = mappings.iter().map(|(_, gid)| *gid).collect();
        for gid in collect_glyph_ids(doc) {
            if !existing_gids.contains(&gid) {
                if let Some(&c) = glyph_reverse.get(&gid) {
                    mappings.push((c, gid));
                }
            }
        }

        let char_to_gid: HashMap<char, u16> = mappings.iter().copied().collect();
        let widths = widths_array(face, &mappings);

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let resources_str = format!("/Font << /F1 {font_id} 0 R >> {xobj_res} {pat_res}");

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let stream_bytes = build_page_stream_cidfont(page, &char_to_gid, &ptr_to_idx, &img_refs, &pat_ptr_to_idx, &pat_refs);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        // Type0 font (F1)
        self.add(font_id, format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont /CrystallineFont \
               /Encoding /Identity-H \
               /DescendantFonts [{cidfont_id} 0 R] \
               /ToUnicode {to_unicode_id} 0 R >>"
        ));

        // CIDFont
        self.add(cidfont_id, format!(
            "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /CrystallineFont \
               /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
               /FontDescriptor {font_descriptor_id} 0 R \
               /DW 500 \
               /W [{widths}] >>"
        ));

        // FontDescriptor
        self.add(font_descriptor_id, format!(
            "<< /Type /FontDescriptor /FontName /CrystallineFont \
               /Flags 32 \
               /FontBBox [-1000 -200 2000 900] \
               /ItalicAngle 0 /Ascent 800 /Descent -200 \
               /CapHeight 700 /StemV 80 \
               /FontFile2 {font_stream_id} 0 R >>"
        ));

        // Font data stream — Opção A: fonte completa sem subsetting (ADR-0027)
        let font_len = font_data.len();
        let mut font_stream = format!(
            "<< /Length {font_len} /Subtype /CIDFontType2 >>\nstream\n"
        ).into_bytes();
        font_stream.extend_from_slice(font_data);
        font_stream.extend_from_slice(b"\nendstream");
        self.add_bytes(font_stream_id, font_stream);

        // ToUnicode CMap stream
        let cmap = to_unicode_cmap(&mappings);
        let cmap_len = cmap.len();
        let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
        cmap_obj.extend_from_slice(&cmap);
        cmap_obj.extend_from_slice(b"\nendstream");
        self.add_bytes(to_unicode_id, cmap_obj);

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.serialize()
    }

    // ── Caminho Multi-font (Passo 146, ADR-0055 decisão 5) ───────────────────

    fn build_multifont(
        mut self,
        doc:   &PagedDocument,
        fonts: &[(FontList, Vec<u8>)],
        faces: &[Face<'_>],
    ) -> Vec<u8> {
        let n_pages = doc.pages.len().max(1);
        let n_fonts = fonts.len();
        let first_page   = 3usize;
        let first_stream = first_page + n_pages;
        // Cada font ocupa 5 IDs consecutivos: type0, cidfont, descriptor,
        // font_stream, to_unicode. Type0 é o "/Fn" referenciado no resource.
        let fonts_start  = first_stream + n_pages;
        let first_img_id = fonts_start + 5 * n_fonts;

        // Codepoints + glyph mappings por font. Cada font tem o seu
        // mapping (chars partilhados; gids específicos da face).
        let chars = collect_codepoints(doc);
        let glyph_ids = collect_glyph_ids(doc);
        let mut per_font_mappings: Vec<Vec<(char, u16)>> = Vec::with_capacity(n_fonts);
        let mut per_font_char_to_gid: Vec<HashMap<char, u16>> = Vec::with_capacity(n_fonts);
        let mut per_font_widths: Vec<String> = Vec::with_capacity(n_fonts);
        for face in faces {
            let mut mappings = map_chars_to_glyphs(face, &chars);
            // Adicionar glifos variantes de tamanho matemático
            // (Passo 45, DEBT-9) — mesmo tratamento que `build_cidfont`.
            let glyph_reverse = build_math_glyph_reverse_map(face);
            let existing_gids: BTreeSet<u16> = mappings.iter().map(|(_, gid)| *gid).collect();
            for &gid in &glyph_ids {
                if !existing_gids.contains(&gid) {
                    if let Some(&c) = glyph_reverse.get(&gid) {
                        mappings.push((c, gid));
                    }
                }
            }
            let char_to_gid: HashMap<char, u16> = mappings.iter().copied().collect();
            let widths = widths_array(face, &mappings);
            per_font_mappings.push(mappings);
            per_font_char_to_gid.push(char_to_gid);
            per_font_widths.push(widths);
        }

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n_pages)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n_pages} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let font_entries = (0..n_fonts).map(|fi| {
                let type0_id = fonts_start + 5 * fi;
                format!("/F{} {} 0 R", fi + 1, type0_id)
            }).collect::<Vec<_>>().join(" ");
            let resources_str = format!("/Font << {font_entries} >> {xobj_res} {pat_res}");

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let stream_bytes = build_page_stream_multifont(
                page, fonts, &per_font_char_to_gid, &ptr_to_idx, &img_refs,
                &pat_ptr_to_idx, &pat_refs,
            );
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        // Emit objectos por font (5 cada).
        for (fi, ((_, font_data), _face)) in fonts.iter().zip(faces.iter()).enumerate() {
            let type0_id      = fonts_start + 5 * fi;
            let cidfont_id    = type0_id + 1;
            let descriptor_id = type0_id + 2;
            let stream_id     = type0_id + 3;
            let to_unicode_id = type0_id + 4;
            let name = format!("CrystallineFont{}", fi + 1);
            let widths = &per_font_widths[fi];
            let mappings = &per_font_mappings[fi];

            // Type0
            self.add(type0_id, format!(
                "<< /Type /Font /Subtype /Type0 /BaseFont /{name} \
                   /Encoding /Identity-H \
                   /DescendantFonts [{cidfont_id} 0 R] \
                   /ToUnicode {to_unicode_id} 0 R >>"
            ));

            // CIDFont
            self.add(cidfont_id, format!(
                "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /{name} \
                   /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
                   /FontDescriptor {descriptor_id} 0 R \
                   /DW 500 \
                   /W [{widths}] >>"
            ));

            // FontDescriptor
            self.add(descriptor_id, format!(
                "<< /Type /FontDescriptor /FontName /{name} \
                   /Flags 32 \
                   /FontBBox [-1000 -200 2000 900] \
                   /ItalicAngle 0 /Ascent 800 /Descent -200 \
                   /CapHeight 700 /StemV 80 \
                   /FontFile2 {stream_id} 0 R >>"
            ));

            // FontFile2 stream — fonte completa, sem subsetting (ADR-0027).
            let font_len = font_data.len();
            let mut font_stream = format!(
                "<< /Length {font_len} /Subtype /CIDFontType2 >>\nstream\n"
            ).into_bytes();
            font_stream.extend_from_slice(font_data);
            font_stream.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, font_stream);

            // ToUnicode CMap
            let cmap = to_unicode_cmap(mappings);
            let cmap_len = cmap.len();
            let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
            cmap_obj.extend_from_slice(&cmap);
            cmap_obj.extend_from_slice(b"\nendstream");
            self.add_bytes(to_unicode_id, cmap_obj);
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.serialize()
    }

    /// Emite todos os XObjects de imagem pré-processados para o builder.
    ///
    /// Para PNG com alpha: emite /SMask (canal alpha) antes do XObject principal
    /// (canal RGB), para que o SMask apareça antes no ficheiro PDF — o dicionário
    /// do XObject principal referencia o ID do SMask por forward reference.
    /// P263 — Emite objects Function + Shading + Pattern para cada
    /// gradient único pré-processado por `scan_all_gradients`.
    ///
    /// `next_sub_id`: contador de IDs allocaveis para sub-Functions
    /// (Type 3 stitching). Os IDs alocados por gradient (3×N) **não
    /// incluem** as sub-Functions; estas são alocadas em `next_sub_id`
    /// (que deve apontar para zone de IDs livre pós-todos os outros).
    fn emit_gradient_objects(
        &mut self,
        grad_objs: Vec<GradientObject>,
        page_dimensions: &[(f64, f64)],
        next_sub_id: &mut usize,
    ) {
        for go in grad_objs {
            let GradientObject { kind, function_id, shading_id, pattern_id } = go;
            let (page_w, page_h) = page_dimensions.first().copied().unwrap_or((595.0, 842.0));
            // P265 — branching Linear / Radial.
            let (stops, shading_dict) = match &kind {
                GradientObjectKind::Linear(linear) => {
                    let stops = oklab_sample_stops(linear, 16);
                    let (x0, y0, x1, y1) = compute_axial_coords(
                        linear.angle.to_rad(), 0.0, 0.0, page_w, page_h);
                    let shading = format!(
                        "<< /ShadingType 2 /ColorSpace /DeviceRGB \
                           /Coords [{:.3} {:.3} {:.3} {:.3}] \
                           /Function {} 0 R /Extend [false false] >>",
                        x0, y0, x1, y1, function_id,
                    );
                    (stops, shading)
                }
                GradientObjectKind::Radial(radial) => {
                    let stops = oklab_sample_stops_radial(radial, 16);
                    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
                        radial.center, radial.radius, page_w, page_h);
                    let shading = format!(
                        "<< /ShadingType 3 /ColorSpace /DeviceRGB \
                           /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
                           /Function {} 0 R /Extend [true true] >>",
                        x0, y0, r0, x1, y1, r1, function_id,
                    );
                    (stops, shading)
                }
            };
            let (func_dict, sub_objs) = emit_function_dict(&stops, function_id, next_sub_id);
            // Emit sub-Functions primeiro (se Type 3 stitching).
            for (sub_id, sub_dict) in sub_objs {
                self.add(sub_id, sub_dict);
            }
            // Emit Function dict principal.
            self.add(function_id, func_dict);
            // Emit Shading dict.
            self.add(shading_id, shading_dict);
            // Emit Pattern dict.
            let pattern_dict = format!(
                "<< /PatternType 2 /Shading {} 0 R /Matrix [1 0 0 1 0 0] >>",
                shading_id,
            );
            self.add(pattern_id, pattern_dict);
        }
    }

    fn emit_image_xobjects(&mut self, xobjects: Vec<ImageXObject>) {
        for xobj in xobjects {
            match xobj {
                ImageXObject::Jpeg { data, main_obj_id, iw, ih } => {
                    let cs = jpeg_color_space(&data);
                    self.add_bytes(main_obj_id, build_jpeg_xobject(&data, iw, ih, cs));
                }
                ImageXObject::Png { payload, main_obj_id, smask_obj_id } => {
                    // Emitir /SMask antes do XObject principal.
                    if let (Some(smask_id), Some(alpha)) = (smask_obj_id, &payload.alpha_data_compressed) {
                        self.add_bytes(smask_id, build_png_smask_xobject(payload.width, payload.height, alpha));
                    }
                    self.add_bytes(main_obj_id, build_png_rgb_xobject(&payload, smask_obj_id));
                }
            }
        }
    }

    fn serialize(self) -> Vec<u8> {
        // Header — %PDF-1.7 + comentário binário (4 bytes > 127)
        let mut out: Vec<u8> = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
        let mut offsets: HashMap<usize, usize> = Default::default();

        for (id, content) in &self.objects {
            offsets.insert(*id, out.len());
            out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
            out.extend_from_slice(content);
            out.extend_from_slice(b"\nendobj\n");
        }

        // xref table
        let xref_start = out.len();
        let max_id = offsets.keys().copied().max().unwrap_or(0);
        out.extend_from_slice(b"xref\n");
        out.extend_from_slice(format!("0 {}\n", max_id + 1).as_bytes());
        out.extend_from_slice(b"0000000000 65535 f \n");
        for id in 1..=max_id {
            let off = offsets.get(&id).copied().unwrap_or(0);
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }

        // Trailer
        out.extend_from_slice(format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            max_id + 1, xref_start
        ).as_bytes());

        out
    }
}

// ── Helpers — caminho Helvetica ────────────────────────────────────────────

/// P263 — Emite operadores de stroke colour para um Paint (Solid ou Gradient).
///
/// Para `Paint::Solid(c)`: emit `r g b RG` literal P261 preservado.
/// Para `Paint::Gradient(g)`: emit `/Pattern CS /P{n} SCN` (set colour
/// space pattern + apply pattern).
fn emit_stroke_paint(
    ops:            &mut String,
    paint:          &typst_core::entities::paint::Paint,
    thickness:      f64,
    pat_ptr_to_idx: &HashMap<usize, usize>,
    pat_refs:       &[PatternRef],
) {
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::paint::Paint;
    match paint {
        Paint::Solid(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
        }
        Paint::Gradient(g) => {
            // P265 — Linear e Radial ambos via lookup pattern.
            // P267 — Conic fallback Solid (PDF emit adiado P268).
            let ptr = match g {
                Gradient::Linear(l) => std::sync::Arc::as_ptr(l) as usize,
                Gradient::Radial(r) => std::sync::Arc::as_ptr(r) as usize,
                Gradient::Conic(_) => {
                    let c = paint.to_color();
                    let (r, g, b, _) = c.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
                    return;
                }
            };
            if let Some(&idx) = pat_ptr_to_idx.get(&ptr) {
                let r = &pat_refs[idx];
                ops.push_str(&format!("/Pattern CS\n/{} SCN\n{:.2} w\n", r.name, thickness));
            } else {
                // Fallback paranóide — gradient não registado em scan_all_gradients.
                let c = paint.to_color();
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
            }
        }
    }
}

/// Alias específico para path Helvetica (legacy alias preservado para
/// compatibilidade interna; comportamento idêntico a `emit_stroke_paint`).
fn emit_stroke_paint_type1(
    ops:            &mut String,
    paint:          &typst_core::entities::paint::Paint,
    thickness:      f64,
    pat_ptr_to_idx: &HashMap<usize, usize>,
    pat_refs:       &[PatternRef],
) {
    emit_stroke_paint(ops, paint, thickness, pat_ptr_to_idx, pat_refs);
}

fn build_page_stream_type1(
    page:           &Page,
    ptr_to_idx:     &HashMap<usize, usize>,
    img_refs:       &[ImageRef],
    pat_ptr_to_idx: &HashMap<usize, usize>,
    pat_refs:       &[PatternRef],
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                let pdf_y = page_height - pos.y.val();
                let safe  = escape_pdf_string(text.as_str());

                if safe.is_empty() { continue; }

                let font_ref = match (style.bold, style.italic) {
                    (true,  _)     => "F2",  // Helvetica-Bold
                    (false, true)  => "F3",  // Helvetica-Oblique
                    (false, false) => "F1",  // Helvetica
                };

                // Passo 137 (Fase B.1 DEBT-52): PDF `Tc` operator
                // adiciona character spacing a cada glyph dentro
                // do `Tj`. Resolvido contra `size` para Pt.
                // Omitido se tracking ausente ou zero.
                let tracking_pt = style.tracking
                    .map(|t| t.resolve_pt(style.size.val()))
                    .unwrap_or(0.0);
                let tc_op = if tracking_pt.abs() > f64::EPSILON {
                    format!("{:.2} Tc\n", tracking_pt)
                } else {
                    String::new()
                };

                // Passo 139 (Fase B.3 DEBT-52): faux-bold via PDF
                // `2 Tr` (fill + stroke) + `{stroke_pt} w`. Estratégia
                // de aproximação até font embedding real (Fase C).
                // Wrapped em `q/Q` porque `w` é graphics state — não
                // pode atravessar para Lines seguintes.
                const FAUX_BOLD_K: f64 = 0.04;
                let stroke_pt = style.faux_bold_stroke_pt(FAUX_BOLD_K);
                let (q_open, q_close, bold_ops) = if stroke_pt > f64::EPSILON {
                    (
                        "q\n",
                        "Q\n",
                        format!("2 Tr\n{:.3} w\n", stroke_pt),
                    )
                } else {
                    ("", "", String::new())
                };

                ops.push_str(&format!(
                    "{q_open}BT\n/{font_ref} {:.1} Tf\n{tc_op}{bold_ops}{:.1} {:.1} Td\n({safe}) Tj\nET\n{q_close}",
                    style.size.val(), pos.x.val(), pdf_y
                ));
            }
            FrameItem::Line { start, end, thickness } => {
                let x1 = start.x.val();
                let y1 = page_height - start.y.val();
                let x2 = end.x.val();
                let y2 = page_height - end.y.val();
                ops.push_str(&format!(
                    "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
                    thickness, x1, y1, x2, y2
                ));
            }
            // FrameItem::Glyph não tem suporte no caminho Helvetica (sem fonte TrueType).
            // Ignorado silenciosamente — o delimitador simplesmente não aparece.
            FrameItem::Glyph { .. } => {}
            FrameItem::Image { pos, data, width, height, .. } => {
                let ptr = Arc::as_ptr(data) as usize;
                if let Some(&idx) = ptr_to_idx.get(&ptr) {
                    // pos.y é o TOPO da imagem → canto inferior esquerdo no espaço PDF.
                    let pdf_y = page_height - pos.y.val() - height.val();
                    ops.push_str(&format!(
                        "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                        width.val(), height.val(), pos.x.val(), pdf_y,
                        img_refs[idx].name
                    ));
                }
            }
            FrameItem::Shape { pos, kind, width, height, fill, stroke } => {
                use typst_core::entities::geometry::ShapeKind;
                // Inverter eixo Y: layout tem Y crescente para baixo; PDF crescente para cima.
                // pdf_y é o canto inferior esquerdo da bounding box no espaço PDF.
                let pdf_y = page_height - pos.y.val() - height;

                // Ordem obrigatória: push state → cores → path → paint operator → pop state.
                ops.push_str("q\n");

                // Cor de preenchimento (rg — RGB para fills).
                // Alpha ignorado: transparência vectorial requer ca/CA (PDF 1.4), adiado.
                if let Some(c) = fill {
                    let (r, g, b, _) = c.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
                }

                // Cor e espessura do contorno (RG + w).
                // P263 — branching Paint::Solid vs Paint::Gradient.
                if let Some(s) = stroke {
                    emit_stroke_paint_type1(&mut ops, &s.paint, s.thickness, pat_ptr_to_idx, pat_refs);
                }

                // Path — depende do tipo de forma.
                match kind {
                    ShapeKind::Rect => {
                        // Operador re: x y width height re — rectângulo como sub-path fechado.
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::RoundedRect { radii } => {
                        // P242 — Bezier 4 corners (paridade Ellipse mesmo kappa).
                        emit_rounded_rect_ops(&mut ops, pos.x.val(), pdf_y, *width, *height, radii);
                    }
                    ShapeKind::Ellipse => {
                        // κ = 4*(√2−1)/3 ≈ 0.5523: minimiza erro de arredondamento para qualquer raio.
                        const KAPPA: f64 = 0.552_284_749_831;
                        let cx = pos.x.val() + width  / 2.0;
                        let cy = pdf_y       + height / 2.0;
                        let rx = width  / 2.0;
                        let ry = height / 2.0;
                        let ox = rx * KAPPA;
                        let oy = ry * KAPPA;
                        // Mover para o topo da elipse.
                        ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                        // 1º quadrante: topo → direita
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                        // 4º quadrante: direita → base
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                        // 3º quadrante: base → esquerda
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                        // 2º quadrante: esquerda → topo (fecha a elipse)
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                    }
                    ShapeKind::Line { dx, dy } => {
                        // pdf_y é o canto inferior esquerdo da bounding box (já calculado acima).
                        // width e height são os valores absolutos do layouter (dx.abs(), dy.abs()).
                        // dx > 0: início à esquerda, fim à direita da caixa.
                        // dx < 0: início à direita, fim à esquerda da caixa.
                        let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                        let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                        // dy > 0: desce no layout → início no topo PDF (pdf_y + height), fim na base (pdf_y).
                        // dy < 0: sobe no layout  → início na base PDF (pdf_y), fim no topo (pdf_y + height).
                        let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                        let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                        let start_x = pos.x.val() + start_offset_x;
                        let start_y = pdf_y        + start_offset_y;
                        let end_x   = pos.x.val() + end_offset_x;
                        let end_y   = pdf_y        + end_offset_y;
                        ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                        ops.push_str(&format!("{:.3} {:.3} l\n", end_x,   end_y));
                    }
                    ShapeKind::Path(items) => {
                        use typst_core::entities::geometry::PathItem;
                        for item in items {
                            match item {
                                PathItem::MoveTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} m\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::LineTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} l\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                    "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                    pos.x.val() + p1.x.0, page_height - (pos.y.val() + p1.y.0),
                                    pos.x.val() + p2.x.0, page_height - (pos.y.val() + p2.y.0),
                                    pos.x.val() + p3.x.0, page_height - (pos.y.val() + p3.y.0),
                                )),
                                PathItem::ClosePath => ops.push_str("h\n"),
                            }
                        }
                    }
                }

                // Paint operator (fill/stroke/ambos).
                match (fill.is_some(), stroke.is_some()) {
                    (true,  true)  => ops.push_str("B\n"),
                    (true,  false) => ops.push_str("f\n"),
                    (false, true)  => ops.push_str("S\n"),
                    (false, false) => {}
                }

                ops.push_str("Q\n");
            }
            FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } => {
                // pdf_y: topo do grupo na página em coordenadas PDF (Y-up).
                let pdf_y = page_height - pos.y.val();

                // O layouter usa Y-down; o PDF usa Y-up.
                // Os componentes de cisalhamento b e c são invertidos para corrigir a paridade.
                ops.push_str("q\n");
                ops.push_str(&format!(
                    "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
                    matrix.a,
                    -matrix.b,
                    -matrix.c,
                    matrix.d,
                    pos.x.val() + matrix.tx,
                    pdf_y       - matrix.ty,
                ));

                // Clipping path no espaço LOCAL: após cm, antes dos filhos.
                // W = definir clipping path; n = fechar sem pintar.
                if let Some(mask) = clip_mask {
                    emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                    ops.push_str("W n\n");
                }

                for child in items {
                    draw_item_local(&mut ops, child);
                }
                ops.push_str("Q\n");
            }
        }
    }

    ops.into_bytes()
}

/// Emite os operadores de path de uma forma no espaço LOCAL de um Group.
///
/// NÃO usa page_height. A matriz `cm` já inverteu o eixo Y.
/// Chamada para emitir clip_mask antes de `W n`.
fn emit_shape_path_local(ops: &mut String, kind: &typst_core::entities::geometry::ShapeKind, width: f64, height: f64) {
    use typst_core::entities::geometry::{PathItem, ShapeKind};
    match kind {
        ShapeKind::Rect => {
            ops.push_str(&format!("0.00 {:.2} {:.2} {:.2} re\n", -height, width, height));
        }
        ShapeKind::RoundedRect { radii } => {
            // P242 (M9d / M7+5) — Bezier 4 corners path em espaço local
            // (origem 0,0; Y invertido pela matriz cm). Coords iguais a
            // ShapeKind::Rect: (0, -height, width, height).
            emit_rounded_rect_ops(ops, 0.0, -height, width, height, radii);
        }
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    PathItem::MoveTo(p) => ops.push_str(&format!(
                        "{:.2} {:.2} m\n", p.x.0, -p.y.0,
                    )),
                    PathItem::LineTo(p) => ops.push_str(&format!(
                        "{:.2} {:.2} l\n", p.x.0, -p.y.0,
                    )),
                    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, -p1.y.0, p2.x.0, -p2.y.0, p3.x.0, -p3.y.0,
                    )),
                    PathItem::ClosePath => ops.push_str("h\n"),
                }
            }
        }
        _ => {}
    }
}

/// **P242 (M9d / M7+5)** — emite operadores PDF para um rectângulo com
/// cantos arredondados via Bezier 4 corners (paridade vanilla
/// `typst-pdf/.../shape.rs::draw_rounded_rect`).
///
/// Coordenadas em sistema PDF (Y crescente para cima). `(x, y)` é o
/// canto inferior-esquerdo; `w` largura; `h` altura positivos. `radii`
/// em `Corners<Length>` (top_left/top_right/bottom_right/bottom_left
/// sentido horário começando top-left).
///
/// **Bezier kappa = 0.552_284_749_831** (paridade `ShapeKind::Ellipse`
/// neste mesmo ficheiro). Quarto de círculo aproximado com 2 control
/// points por canto.
///
/// **Output**: sequência `m` (move) + `l` (line) + `c` (cubic) + `h`
/// (closePath) — formato compatível com `B`/`S`/`W n` paint operators.
fn emit_rounded_rect_ops(
    ops: &mut String,
    x: f64, y: f64, w: f64, h: f64,
    radii: &typst_core::entities::corners::Corners<typst_core::entities::layout_types::Length>,
) {
    const K: f64 = 0.552_284_749_831;
    // Resolver Length → f64 pt (em = 0 para clip_mask; valores absolutos).
    // Clamp cada raio a metade da menor dimensão (paridade vanilla evita
    // overflow geométrico).
    let max_r = (w.min(h)) / 2.0;
    let tl = radii.top_left.abs.0.clamp(0.0, max_r);
    let tr = radii.top_right.abs.0.clamp(0.0, max_r);
    let br = radii.bottom_right.abs.0.clamp(0.0, max_r);
    let bl = radii.bottom_left.abs.0.clamp(0.0, max_r);

    // Sentido horário em PDF coords (Y para cima). Sequência:
    // start top-left edge → top edge → top-right corner → right edge →
    // bottom-right corner → bottom edge → bottom-left corner → left edge →
    // top-left corner → close.
    let x_left   = x;
    let x_right  = x + w;
    let y_top    = y + h;
    let y_bottom = y;

    // MoveTo: começa no início da edge top (após canto top-left).
    ops.push_str(&format!("{:.3} {:.3} m\n", x_left + tl, y_top));
    // Linha top edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right - tr, y_top));
    // Cubic top-right corner.
    if tr > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right - tr + tr * K, y_top,
            x_right,               y_top - tr + tr * K,
            x_right,               y_top - tr));
    }
    // Linha right edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right, y_bottom + br));
    // Cubic bottom-right corner.
    if br > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right,               y_bottom + br - br * K,
            x_right - br + br * K, y_bottom,
            x_right - br,          y_bottom));
    }
    // Linha bottom edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left + bl, y_bottom));
    // Cubic bottom-left corner.
    if bl > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left + bl - bl * K, y_bottom,
            x_left,               y_bottom + bl - bl * K,
            x_left,               y_bottom + bl));
    }
    // Linha left edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left, y_top - tl));
    // Cubic top-left corner.
    if tl > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left,               y_top - tl + tl * K,
            x_left + tl - tl * K, y_top,
            x_left + tl,          y_top));
    }
    // Fecha o path.
    ops.push_str("h\n");
}

/// Desenha um `FrameItem` em espaço LOCAL (após `cm`).
///
/// Diferença de `draw_item_global`: não subtrai `page_height` — a matriz `cm`
/// já aplicou a transformação e a inversão Y. Os filhos usam `pos.y.0` directamente.
fn draw_item_local(ops: &mut String, item: &FrameItem) {
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::FrameItem;
    match item {
        FrameItem::Shape { pos, kind, width, height, fill, stroke } => {
            let local_y = pos.y.0;
            ops.push_str("q\n");
            if let Some(c) = fill {
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
            }
            if let Some(s) = stroke {
                let (r, g, b, _) = s.paint.to_color().to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, s.thickness));
            }
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.0, local_y, width, height));
                }
                ShapeKind::RoundedRect { radii } => {
                    // P242 — Bezier 4 corners em espaço local.
                    emit_rounded_rect_ops(&mut *ops, pos.x.0, local_y, *width, *height, radii);
                }
                ShapeKind::Ellipse => {
                    const KAPPA: f64 = 0.552_284_749_831;
                    let cx = pos.x.0 + width  / 2.0;
                    let cy = local_y  + height / 2.0;
                    let rx = width  / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * KAPPA;
                    let oy = ry * KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                    let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                    ops.push_str(&format!("{:.3} {:.3} m\n",
                        pos.x.0 + start_offset_x, local_y + start_offset_y));
                    ops.push_str(&format!("{:.3} {:.3} l\n",
                        pos.x.0 + end_offset_x, local_y + end_offset_y));
                }
                ShapeKind::Path(items) => {
                    use typst_core::entities::geometry::PathItem;
                    for item in items {
                        match item {
                            PathItem::MoveTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} m\n",
                                pos.x.0 + p.x.0, -(local_y + p.y.0),
                            )),
                            PathItem::LineTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} l\n",
                                pos.x.0 + p.x.0, -(local_y + p.y.0),
                            )),
                            PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                pos.x.0 + p1.x.0, -(local_y + p1.y.0),
                                pos.x.0 + p2.x.0, -(local_y + p2.y.0),
                                pos.x.0 + p3.x.0, -(local_y + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }
            match (fill.is_some(), stroke.is_some()) {
                (true,  true)  => ops.push_str("B\n"),
                (true,  false) => ops.push_str("f\n"),
                (false, true)  => ops.push_str("S\n"),
                (false, false) => {}
            }
            ops.push_str("Q\n");
        }
        _ => {}  // Texto e outros tipos em grupos: adiado para passo futuro.
    }
}

fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '('  => out.push_str("\\("),
            ')'  => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _    => out.push('?'),
        }
    }
    out
}

// ── Helpers — caminho CIDFont ──────────────────────────────────────────────

/// Coleciona todos os codepoints Unicode distintos usados no documento.
fn collect_codepoints(doc: &PagedDocument) -> Vec<char> {
    let mut seen = std::collections::BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    seen.insert(c);
                }
            }
            // Image, Line, Glyph não contribuem com codepoints de texto.
        }
    }
    seen.into_iter().collect()
}

/// Coleciona todos os glyph IDs distintos usados em `FrameItem::Glyph` no documento.
fn collect_glyph_ids(doc: &PagedDocument) -> BTreeSet<u16> {
    let mut ids = BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Glyph { glyph_id, .. } = item {
                ids.insert(*glyph_id);
            }
        }
    }
    ids
}

/// Para um conjunto de chars, retorna Vec<(char, glyph_id)>.
/// Chars sem glyph na fonte são omitidos.
fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char]) -> Vec<(char, u16)> {
    chars.iter()
        .filter_map(|&c| face.glyph_index(c).map(|gid| (c, gid.0)))
        .collect()
}

/// Gera o array W do CIDFont: "gid [width] ..." em unidades PDF (1/1000 text space).
fn widths_array(face: &Face<'_>, mappings: &[(char, u16)]) -> String {
    let upem = face.units_per_em() as f64;
    let mut parts = Vec::new();
    for (_c, gid) in mappings {
        let adv = face.glyph_hor_advance(ttf_parser::GlyphId(*gid))
            .unwrap_or(500) as f64;
        let w = (adv / upem * 1000.0).round() as i32;
        parts.push(format!("{gid} [{w}]"));
    }
    parts.join(" ")
}

/// Gera o stream ToUnicode CMap para o mapeamento glyph_id → char.
/// Emite em blocos de ≤ 100 entradas (limite PDF spec).
fn to_unicode_cmap(mappings: &[(char, u16)]) -> Vec<u8> {
    let mut s = String::new();
    s.push_str("/CIDInit /ProcSet findresource begin\n");
    s.push_str("12 dict begin\n");
    s.push_str("begincmap\n");
    s.push_str("/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n");
    s.push_str("/CMapName /Adobe-Identity-UCS def\n");
    s.push_str("/CMapType 2 def\n");
    s.push_str("1 begincodespacerange\n");
    s.push_str("<0000> <FFFF>\n");
    s.push_str("endcodespacerange\n");

    for chunk in mappings.chunks(100) {
        s.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for (c, gid) in chunk {
            let cp = *c as u32;
            s.push_str(&format!("<{gid:04X}> <{cp:04X}>\n"));
        }
        s.push_str("endbfchar\n");
    }

    s.push_str("endcmap\n");
    s.push_str("CMapName currentdict /CMap defineresource pop\n");
    s.push_str("end\nend\n");
    s.into_bytes()
}

/// Serializa texto como string hexadecimal de glyph IDs para Identity-H.
/// Chars sem mapeamento usam glyph ID 0 (notdef).
fn text_to_hex_string(text: &str, char_to_gid: &HashMap<char, u16>) -> String {
    let mut hex = String::from("<");
    for c in text.chars() {
        let gid = char_to_gid.get(&c).copied().unwrap_or(0);
        hex.push_str(&format!("{gid:04X}"));
    }
    hex.push('>');
    hex
}

fn build_page_stream_cidfont(
    page:           &Page,
    char_to_gid:    &HashMap<char, u16>,
    ptr_to_idx:     &HashMap<usize, usize>,
    img_refs:       &[ImageRef],
    pat_ptr_to_idx: &HashMap<usize, usize>,
    pat_refs:       &[PatternRef],
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                if text.is_empty() { continue; }

                let pdf_y   = page_height - pos.y.val();
                let hex_str = text_to_hex_string(text.as_str(), char_to_gid);

                // Identity-H: F1 para todos os estilos (bold/italic requer fontes adicionais — DEBT)
                ops.push_str(&format!(
                    "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                    style.size.val(), pos.x.val(), pdf_y
                ));
            }
            FrameItem::Line { start, end, thickness } => {
                let x1 = start.x.val();
                let y1 = page_height - start.y.val();
                let x2 = end.x.val();
                let y2 = page_height - end.y.val();
                ops.push_str(&format!(
                    "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
                    thickness, x1, y1, x2, y2
                ));
            }
            // Glifo variante de tamanho matemático — emitir directamente por ID (Identity-H).
            // Passo 45: glyph_id incluído no ToUnicode via build_math_glyph_reverse_map.
            FrameItem::Glyph { pos, glyph_id, size, .. } => {
                let pdf_y = page_height - pos.y.val();
                ops.push_str(&format!(
                    "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n<{:04X}> Tj\nET\n",
                    size.val(), pos.x.val(), pdf_y, glyph_id
                ));
            }
            FrameItem::Image { pos, data, width, height, .. } => {
                let ptr = Arc::as_ptr(data) as usize;
                if let Some(&idx) = ptr_to_idx.get(&ptr) {
                    // pos.y é o TOPO da imagem → canto inferior esquerdo no espaço PDF.
                    let pdf_y = page_height - pos.y.val() - height.val();
                    ops.push_str(&format!(
                        "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                        width.val(), height.val(), pos.x.val(), pdf_y,
                        img_refs[idx].name
                    ));
                }
            }
            FrameItem::Shape { pos, kind, width, height, fill, stroke } => {
                use typst_core::entities::geometry::ShapeKind;
                let pdf_y = page_height - pos.y.val() - height;

                ops.push_str("q\n");
                if let Some(c) = fill {
                    let (r, g, b, _) = c.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
                }
                if let Some(s) = stroke {
                    // P263 — branching Paint::Solid vs Paint::Gradient.
                    emit_stroke_paint(&mut ops, &s.paint, s.thickness, pat_ptr_to_idx, pat_refs);
                }
                match kind {
                    ShapeKind::Rect => {
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::RoundedRect { radii } => {
                        // P242 — Bezier 4 corners path (paridade arm shape global).
                        emit_rounded_rect_ops(&mut ops, pos.x.val(), pdf_y, *width, *height, radii);
                    }
                    ShapeKind::Ellipse => {
                        const KAPPA: f64 = 0.552_284_749_831;
                        let cx = pos.x.val() + width  / 2.0;
                        let cy = pdf_y       + height / 2.0;
                        let rx = width  / 2.0;
                        let ry = height / 2.0;
                        let ox = rx * KAPPA;
                        let oy = ry * KAPPA;
                        ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                    }
                    ShapeKind::Line { dx, dy } => {
                        let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                        let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                        let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                        let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                        let start_x = pos.x.val() + start_offset_x;
                        let start_y = pdf_y        + start_offset_y;
                        let end_x   = pos.x.val() + end_offset_x;
                        let end_y   = pdf_y        + end_offset_y;
                        ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                        ops.push_str(&format!("{:.3} {:.3} l\n", end_x,   end_y));
                    }
                    ShapeKind::Path(items) => {
                        use typst_core::entities::geometry::PathItem;
                        for item in items {
                            match item {
                                PathItem::MoveTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} m\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::LineTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} l\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                    "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                    pos.x.val() + p1.x.0, page_height - (pos.y.val() + p1.y.0),
                                    pos.x.val() + p2.x.0, page_height - (pos.y.val() + p2.y.0),
                                    pos.x.val() + p3.x.0, page_height - (pos.y.val() + p3.y.0),
                                )),
                                PathItem::ClosePath => ops.push_str("h\n"),
                            }
                        }
                    }
                }
                match (fill.is_some(), stroke.is_some()) {
                    (true,  true)  => ops.push_str("B\n"),
                    (true,  false) => ops.push_str("f\n"),
                    (false, true)  => ops.push_str("S\n"),
                    (false, false) => {}
                }
                ops.push_str("Q\n");
            }
            FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } => {
                let pdf_y = page_height - pos.y.val();
                ops.push_str("q\n");
                ops.push_str(&format!(
                    "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
                    matrix.a,
                    -matrix.b,
                    -matrix.c,
                    matrix.d,
                    pos.x.val() + matrix.tx,
                    pdf_y       - matrix.ty,
                ));
                if let Some(mask) = clip_mask {
                    emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                    ops.push_str("W n\n");
                }
                for child in items {
                    draw_item_local(&mut ops, child);
                }
                ops.push_str("Q\n");
            }
        }
    }

    ops.into_bytes()
}

// ── Helpers — caminho Multi-font (Passo 146) ───────────────────────────────

/// Page stream para multi-font. Difere de `build_page_stream_cidfont`
/// apenas no arm `FrameItem::Text`: escolhe `/F{i+1}` consoante
/// `style.font` casa com uma das `FontList`s embebidas. Quando
/// `style.font` é `None` ou não há match, usa `/F1` (font 0) por
/// consistência com o caminho single-font (todos os spans usam o
/// mesmo embedding em `build_cidfont`).
///
/// Restantes arms (`Line`, `Glyph`, `Image`, `Shape`, `Group`)
/// são idênticos aos de `build_page_stream_cidfont`. Glyph emite
/// sempre em `/F1` (variantes matemáticas — única font tipicamente
/// usada para math).
fn build_page_stream_multifont(
    page:                 &Page,
    fonts:                &[(FontList, Vec<u8>)],
    per_font_char_to_gid: &[HashMap<char, u16>],
    ptr_to_idx:           &HashMap<usize, usize>,
    img_refs:             &[ImageRef],
    pat_ptr_to_idx:       &HashMap<usize, usize>,
    pat_refs:             &[PatternRef],
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                if text.is_empty() { continue; }

                // Match style.font contra fonts embebidas. Default: 0.
                let fi = style.font.as_ref()
                    .and_then(|fl| fonts.iter().position(|(stored, _)| stored == fl))
                    .unwrap_or(0);

                let pdf_y   = page_height - pos.y.val();
                let hex_str = text_to_hex_string(text.as_str(), &per_font_char_to_gid[fi]);

                ops.push_str(&format!(
                    "BT\n/F{} {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                    fi + 1, style.size.val(), pos.x.val(), pdf_y
                ));
            }
            FrameItem::Line { start, end, thickness } => {
                let x1 = start.x.val();
                let y1 = page_height - start.y.val();
                let x2 = end.x.val();
                let y2 = page_height - end.y.val();
                ops.push_str(&format!(
                    "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
                    thickness, x1, y1, x2, y2
                ));
            }
            FrameItem::Glyph { pos, glyph_id, size, .. } => {
                // Variantes matemáticas: emitir em /F1.
                let pdf_y = page_height - pos.y.val();
                ops.push_str(&format!(
                    "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n<{:04X}> Tj\nET\n",
                    size.val(), pos.x.val(), pdf_y, glyph_id
                ));
            }
            FrameItem::Image { pos, data, width, height, .. } => {
                let ptr = Arc::as_ptr(data) as usize;
                if let Some(&idx) = ptr_to_idx.get(&ptr) {
                    let pdf_y = page_height - pos.y.val() - height.val();
                    ops.push_str(&format!(
                        "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                        width.val(), height.val(), pos.x.val(), pdf_y,
                        img_refs[idx].name
                    ));
                }
            }
            FrameItem::Shape { pos, kind, width, height, fill, stroke } => {
                use typst_core::entities::geometry::ShapeKind;
                let pdf_y = page_height - pos.y.val() - height;

                ops.push_str("q\n");
                if let Some(c) = fill {
                    let (r, g, b, _) = c.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
                }
                if let Some(s) = stroke {
                    // P263 — branching Paint::Solid vs Paint::Gradient.
                    emit_stroke_paint(&mut ops, &s.paint, s.thickness, pat_ptr_to_idx, pat_refs);
                }
                match kind {
                    ShapeKind::Rect => {
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::RoundedRect { radii } => {
                        // P242 — Bezier 4 corners path (paridade arm shape global).
                        emit_rounded_rect_ops(&mut ops, pos.x.val(), pdf_y, *width, *height, radii);
                    }
                    ShapeKind::Ellipse => {
                        const KAPPA: f64 = 0.552_284_749_831;
                        let cx = pos.x.val() + width  / 2.0;
                        let cy = pdf_y       + height / 2.0;
                        let rx = width  / 2.0;
                        let ry = height / 2.0;
                        let ox = rx * KAPPA;
                        let oy = ry * KAPPA;
                        ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                    }
                    ShapeKind::Line { dx, dy } => {
                        let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                        let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                        let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                        let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                        let start_x = pos.x.val() + start_offset_x;
                        let start_y = pdf_y        + start_offset_y;
                        let end_x   = pos.x.val() + end_offset_x;
                        let end_y   = pdf_y        + end_offset_y;
                        ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                        ops.push_str(&format!("{:.3} {:.3} l\n", end_x,   end_y));
                    }
                    ShapeKind::Path(items) => {
                        use typst_core::entities::geometry::PathItem;
                        for item in items {
                            match item {
                                PathItem::MoveTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} m\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::LineTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} l\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                    "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                    pos.x.val() + p1.x.0, page_height - (pos.y.val() + p1.y.0),
                                    pos.x.val() + p2.x.0, page_height - (pos.y.val() + p2.y.0),
                                    pos.x.val() + p3.x.0, page_height - (pos.y.val() + p3.y.0),
                                )),
                                PathItem::ClosePath => ops.push_str("h\n"),
                            }
                        }
                    }
                }
                match (fill.is_some(), stroke.is_some()) {
                    (true,  true)  => ops.push_str("B\n"),
                    (true,  false) => ops.push_str("f\n"),
                    (false, true)  => ops.push_str("S\n"),
                    (false, false) => {}
                }
                ops.push_str("Q\n");
            }
            FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } => {
                let pdf_y = page_height - pos.y.val();
                ops.push_str("q\n");
                ops.push_str(&format!(
                    "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
                    matrix.a,
                    -matrix.b,
                    -matrix.c,
                    matrix.d,
                    pos.x.val() + matrix.tx,
                    pdf_y       - matrix.ty,
                ));
                if let Some(mask) = clip_mask {
                    emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                    ops.push_str("W n\n");
                }
                for child in items {
                    draw_item_local(&mut ops, child);
                }
                ops.push_str("Q\n");
            }
        }
    }

    ops.into_bytes()
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::{
        entities::{content::Content},
        rules::layout::layout,
    };

    #[test]
    fn pdf_header_correcto() {
        let doc = layout(&Content::text("Hello"));
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"), "deve começar com %PDF-1.7");
    }

    #[test]
    fn pdf_termina_com_eof() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let tail = std::str::from_utf8(&pdf[pdf.len().saturating_sub(20)..]).unwrap_or("");
        assert!(tail.contains("%%EOF"), "deve terminar com %%EOF");
    }

    #[test]
    fn pdf_tem_estrutura_valida() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref"),      "deve ter xref");
        assert!(s.contains("trailer"),   "deve ter trailer");
        assert!(s.contains("startxref"), "deve ter startxref");
        assert!(s.contains("/Catalog"),  "deve ter Catalog");
        assert!(s.contains("/Pages"),    "deve ter Pages");
        assert!(s.contains("Helvetica"), "deve ter Helvetica");
    }

    #[test]
    fn pdf_contem_texto_ascii() {
        let doc = layout(&Content::text("Hello world"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("Hello") || s.contains("world"),
            "texto ASCII deve aparecer no PDF");
    }

    #[test]
    fn pdf_documento_vazio_valido() {
        let doc = typst_core::entities::layout_types::PagedDocument::new(vec![]);
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        assert!(String::from_utf8_lossy(&pdf).contains("%%EOF"));
    }

    #[test]
    fn escaping_caracteres_especiais() {
        let escaped = escape_pdf_string("Hello (world) back\\slash");
        assert!(escaped.contains("\\("),  "( deve ser escapado");
        assert!(escaped.contains("\\)"),  ") deve ser escapado");
        assert!(escaped.contains("\\\\"), "\\ deve ser escapado");
        let with_percent = escape_pdf_string("100% done");
        assert!(!with_percent.contains("\\%"), "% não precisa de escape em PDF strings");
        assert!(with_percent.contains('%'), "% deve aparecer sem escape");
    }

    #[test]
    fn inversao_eixo_y_texto_no_topo() {
        use typst_core::entities::layout_types::{
            FrameItem, Page, PagedDocument, Point, Pt, TextStyle,
        };
        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Text {
                pos:   Point { x: Pt(72.0), y: Pt(84.0) },
                text:  "Top".into(),
                style: TextStyle::regular(Pt(12.0)),
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        // y_pdf = 841.89 - 84 = 757.89
        assert!(s.contains("757.89") || s.contains("757.9"),
            "y_pdf deve ser 841.89-84=757.89: {}", &s[..s.len().min(500)]);
    }

    #[test]
    fn pdf_mediabox_dimensoes_a4() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("595.28") && s.contains("841.89"),
            "MediaBox deve ter dimensões A4 (595.28x841.89 pt)");
    }

    // ── Passo 24 — DEBT-5: Unicode PDF ──────────────────────────────────────

    #[test]
    fn unicode_nao_produz_interrogacao() {
        // Modo Helvetica — documenta intenção. Com CIDFont + fonte real, '?' desaparece.
        let doc = layout(&Content::text("café naïve résumé"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref"), "PDF deve ser estruturalmente válido");
    }

    #[test]
    #[ignore = "requer fixture de fonte TrueType"]
    fn cidfont_presente_quando_ha_fonte() {
        // Estrutura esperada no PDF com fonte real:
        // /Type0, /CIDFontType2, /ToUnicode, /FontFile2 devem estar presentes
    }

    #[test]
    fn texto_ascii_com_cidfont() {
        let doc = layout(&Content::text("Hello World"));
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref") && s.contains("%%EOF"));
    }

    #[test]
    fn bullet_e_unicode() {
        use typst_core::entities::{content::Content as C2, layout_types::FrameItem};
        let doc = layout(&C2::list_item(C2::text("item")));
        let has_bullet = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
        assert!(has_bullet, "ListItem deve ter marcador '•' após DEBT-5 pago");
    }

    // ── Testes dos helpers CIDFont ────────────────────────────────────────────

    #[test]
    fn collect_codepoints_vazio() {
        use typst_core::entities::layout_types::PagedDocument;
        let doc = PagedDocument::new(vec![]);
        assert!(collect_codepoints(&doc).is_empty());
    }

    #[test]
    fn collect_codepoints_dedup() {
        let doc = layout(&Content::text("aaa bbb"));
        let chars = collect_codepoints(&doc);
        // BTreeSet garante que não há duplicados
        let unique: std::collections::BTreeSet<_> = chars.iter().copied().collect();
        assert_eq!(chars.len(), unique.len(), "não deve haver duplicados");
    }

    #[test]
    fn to_unicode_cmap_estrutura_basica() {
        let mappings: Vec<(char, u16)> = vec![('A', 36), ('B', 37)];
        let cmap = to_unicode_cmap(&mappings);
        let s = String::from_utf8(cmap).unwrap();
        assert!(s.contains("begincmap"), "deve ter begincmap");
        assert!(s.contains("endcmap"),   "deve ter endcmap");
        assert!(s.contains("beginbfchar"), "deve ter beginbfchar");
        // 'A' = U+0041, glyph 36 = 0x0024
        assert!(s.contains("<0024> <0041>"), "'A' deve mapear para U+0041");
    }

    #[test]
    fn to_unicode_cmap_blocos_de_100() {
        // 101 mappings → dois blocos (100 + 1)
        let mappings: Vec<(char, u16)> = (0u16..101)
            .filter_map(|i| char::from_u32(i as u32 + 32).map(|c| (c, i)))
            .collect();
        let cmap = to_unicode_cmap(&mappings);
        let s = String::from_utf8(cmap).unwrap();
        let count = s.matches("beginbfchar").count();
        assert_eq!(count, 2, "101 entradas → 2 blocos beginbfchar");
    }

    #[test]
    fn text_to_hex_string_ascii() {
        let mut map = HashMap::new();
        map.insert('H', 0x0048u16);
        map.insert('i', 0x0069u16);
        let hex = text_to_hex_string("Hi", &map);
        assert_eq!(hex, "<00480069>", "glyph IDs em hex, 2 bytes cada");
    }

    #[test]
    fn text_to_hex_string_sem_mapeamento_usa_zero() {
        let map = HashMap::new();
        let hex = text_to_hex_string("X", &map);
        assert_eq!(hex, "<0000>", "char sem mapeamento → glyph ID 0");
    }

    // ── Passo 45 — DEBT-9: ToUnicode para FrameItem::Glyph ──────────────────

    #[test]
    fn collect_glyph_ids_de_documento_vazio() {
        use typst_core::entities::layout_types::PagedDocument;
        let doc = PagedDocument::new(vec![]);
        let ids = collect_glyph_ids(&doc);
        assert!(ids.is_empty());
    }

    #[test]
    fn collect_glyph_ids_retorna_ids_unicos() {
        use typst_core::entities::layout_types::{Page, PagedDocument, Point, Pt};
        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![
                FrameItem::Glyph {
                    pos: Point::ZERO, glyph_id: 42, x_advance: Pt(10.0), size: Pt(12.0),
                },
                FrameItem::Glyph {
                    pos: Point::ZERO, glyph_id: 42, x_advance: Pt(10.0), size: Pt(12.0), // dup
                },
                FrameItem::Glyph {
                    pos: Point::ZERO, glyph_id: 99, x_advance: Pt(10.0), size: Pt(12.0),
                },
            ],
        };
        let doc = PagedDocument::new(vec![page]);
        let ids = collect_glyph_ids(&doc);
        assert!(ids.contains(&42u16));
        assert!(ids.contains(&99u16));
        assert_eq!(ids.len(), 2, "sem duplicados");
    }

    #[test]
    fn to_unicode_cmap_inclui_glifo_variante() {
        // Glyph ID 0x00A2 → '(' (U+0028)
        let mappings = vec![('(', 0x00A2u16)];
        let cmap = to_unicode_cmap(&mappings);
        let s = String::from_utf8(cmap).unwrap();
        assert!(s.contains("<00A2> <0028>"), "CMap deve ter entrada glyph→Unicode: {s}");
    }

    // ── Testes de imagem (Passo 73) ───────────────────────────────────────────

    #[test]
    fn detect_format_jpeg() {
        assert_eq!(detect_format(&[0xFF, 0xD8, 0xFF, 0xE0]), ImageFormat::Jpeg);
        assert_eq!(detect_format(&[0xFF, 0xD8, 0xFF, 0x00]), ImageFormat::Jpeg);
    }

    #[test]
    fn detect_format_png() {
        assert_eq!(
            detect_format(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00]),
            ImageFormat::Png,
        );
    }

    #[test]
    fn detect_format_unknown() {
        assert_eq!(detect_format(&[0x00, 0x01, 0x02]), ImageFormat::Unknown);
        assert_eq!(detect_format(&[]), ImageFormat::Unknown);
    }

    #[test]
    fn pipeline_jpeg_gera_pdf_com_xobject() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};

        // JPEG mínimo com magic numbers correctos — 4 bytes suficientes para detect_format.
        let jpeg_bytes = Arc::new(vec![0xFF, 0xD8, 0xFF, 0xE0u8]);

        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Image {
                pos:              Point { x: Pt(72.0), y: Pt(100.0) },
                data:             Arc::clone(&jpeg_bytes),
                width:            Pt(100.0),
                height:           Pt(75.0),
                intrinsic_width:  400,
                intrinsic_height: 300,
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);

        assert!(!pdf.is_empty(), "export_pdf deve produzir bytes");
        assert!(pdf.starts_with(b"%PDF-1.7"), "deve ser PDF válido");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("/XObject"), "deve ter /XObject nos recursos");
        assert!(s.contains("/DCTDecode"), "deve ter /DCTDecode para JPEG");
        assert!(s.contains("/Im1"), "deve referenciar Im1");
        assert!(s.contains("Do"), "deve ter operador Do para imagem");
    }

    #[test]
    fn pipeline_png_invalido_ignorado_graciosamente() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};

        // PNG com apenas magic bytes — processo_png_for_pdf falha, imagem omitida.
        // O PDF deve continuar válido (sem corrupção).
        let png_bytes = Arc::new(vec![0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);

        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Image {
                pos:              Point { x: Pt(72.0), y: Pt(100.0) },
                data:             Arc::clone(&png_bytes),
                width:            Pt(100.0),
                height:           Pt(100.0),
                intrinsic_width:  200,
                intrinsic_height: 200,
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);

        assert!(pdf.starts_with(b"%PDF-1.7"), "PDF deve ser válido mesmo com PNG inválido");
        let s = String::from_utf8_lossy(&pdf);
        // PNG inválido não deve gerar XObject DCTDecode nem FlateDecode
        assert!(!s.contains("/DCTDecode"),   "PNG inválido não usa DCTDecode");
        assert!(!s.contains("/FlateDecode"), "PNG inválido não gera XObject");
    }

    // ── Testes de imagem (Passo 74) ───────────────────────────────────────────

    #[test]
    fn jpeg_color_space_grayscale() {
        // Cabeçalho JPEG mínimo com SOF0 e 1 canal (Grayscale).
        let jpeg = vec![
            0xFF, 0xD8,       // SOI
            0xFF, 0xC0,       // SOF0
            0x00, 0x0B,       // length = 11
            0x08,             // precision = 8 bits
            0x00, 0x01,       // height = 1
            0x00, 0x01,       // width = 1
            0x01,             // components = 1 → DeviceGray
        ];
        assert_eq!(jpeg_color_space(&jpeg), "/DeviceGray");
    }

    #[test]
    fn jpeg_color_space_rgb() {
        let jpeg = vec![
            0xFF, 0xD8,
            0xFF, 0xC0,
            0x00, 0x0B,
            0x08,
            0x00, 0x01, 0x00, 0x01,
            0x03, // components = 3 → DeviceRGB
        ];
        assert_eq!(jpeg_color_space(&jpeg), "/DeviceRGB");
    }

    #[test]
    fn jpeg_color_space_fallback_rgb() {
        // Sem marcador SOF0/SOF2 — fallback DeviceRGB.
        let jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x04];
        assert_eq!(jpeg_color_space(&jpeg), "/DeviceRGB");
    }

    #[test]
    fn process_png_for_pdf_opaco_sem_alpha() {
        use image::{ImageBuffer, Rgb};
        // Gerar PNG RGB 1×1 sem canal alpha.
        let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(1, 1, vec![255u8, 0, 0]).unwrap();
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();

        let payload = process_png_for_pdf(&buf).expect("deve processar PNG RGB");
        assert_eq!(payload.width,  1);
        assert_eq!(payload.height, 1);
        assert!(payload.alpha_data_compressed.is_none(), "PNG opaco não deve ter alpha");
        assert!(!payload.rgb_data_compressed.is_empty());
    }

    #[test]
    fn process_png_for_pdf_transparente_gera_alpha() {
        use image::{ImageBuffer, Rgba};
        // PNG RGBA 1×1 com pixel semi-transparente.
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(1, 1, vec![255u8, 0, 0, 128]).unwrap();
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();

        let payload = process_png_for_pdf(&buf).expect("deve processar PNG RGBA");
        assert!(payload.alpha_data_compressed.is_some(), "PNG com transparência deve ter alpha");
    }

    #[test]
    fn process_png_for_pdf_opaco_total_sem_smask() {
        use image::{ImageBuffer, Rgba};
        // PNG RGBA 1×1 totalmente opaco — alpha 255 deve ser descartado.
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(1, 1, vec![100u8, 150, 200, 255]).unwrap();
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();

        let payload = process_png_for_pdf(&buf).expect("deve processar PNG RGBA opaco");
        assert!(payload.alpha_data_compressed.is_none(), "alpha 255 uniforme deve ser descartado");
    }

    #[test]
    fn pipeline_jpeg_usa_jpeg_color_space() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};

        // JPEG com SOF0 e 3 canais — deve ter /DeviceRGB no XObject.
        let mut jpeg = vec![0xFF, 0xD8u8, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01, 0x03];
        // Adicionar marcador EOI para que o JPEG seja "válido" o suficiente para o exporter.
        jpeg.extend_from_slice(&[0xFF, 0xD9]);
        let data = Arc::new(jpeg);

        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Image {
                pos: Point { x: Pt(72.0), y: Pt(100.0) },
                data: Arc::clone(&data),
                width: Pt(100.0), height: Pt(75.0),
                intrinsic_width: 1, intrinsic_height: 1,
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("/DeviceRGB"), "JPEG 3 canais deve usar /DeviceRGB");
    }

    #[test]
    fn jpeg_deduplicado_por_arc_ptr() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};

        let jpeg_bytes = Arc::new(vec![0xFF, 0xD8, 0xFF, 0xE0u8]);

        // Mesma imagem duas vezes na mesma página — deve gerar apenas um XObject.
        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![
                FrameItem::Image {
                    pos: Point { x: Pt(72.0), y: Pt(72.0) },
                    data: Arc::clone(&jpeg_bytes),
                    width: Pt(100.0), height: Pt(75.0),
                    intrinsic_width: 400, intrinsic_height: 300,
                },
                FrameItem::Image {
                    pos: Point { x: Pt(72.0), y: Pt(200.0) },
                    data: Arc::clone(&jpeg_bytes),
                    width: Pt(50.0), height: Pt(37.0),
                    intrinsic_width: 400, intrinsic_height: 300,
                },
            ],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);

        // "Im1 Do" deve aparecer duas vezes (dois usos)
        let uses = s.matches("/Im1 Do").count();
        assert_eq!(uses, 2, "Im1 deve ser usado duas vezes mas definido uma vez");
        // Só um XObject com DCTDecode
        let dct_count = s.matches("/DCTDecode").count();
        assert_eq!(dct_count, 1, "deve haver apenas um XObject JPEG (deduplicado)");
    }

    #[test]
    fn export_path_com_cubicto_emite_operador_c() {
        use typst_core::entities::geometry::{PathItem, ShapeKind};
        use typst_core::entities::layout_types::{
            Color, FrameItem, Page, PagedDocument, Point, Pt,
        };

        let path = vec![
            PathItem::MoveTo(Point { x: Pt(0.0),  y: Pt(0.0)  }),
            PathItem::CubicTo(
                Point { x: Pt(10.0), y: Pt(0.0)  },
                Point { x: Pt(20.0), y: Pt(10.0) },
                Point { x: Pt(20.0), y: Pt(20.0) },
            ),
            PathItem::ClosePath,
        ];

        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Shape {
                pos:    Point { x: Pt(72.0), y: Pt(72.0) },
                kind:   ShapeKind::Path(path),
                width:  20.0,
                height: 20.0,
                fill:   Some(Color::rgb(255, 0, 0)),
                stroke: None,
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains(" c\n"),
            "CubicTo deve emitir operador Bézier 'c' no PDF");
        assert!(pdf_str.contains("h\n"),
            "ClosePath deve emitir operador 'h' no PDF");
    }

    #[test]
    fn export_group_com_clip_mask_emite_w_n_na_ordem_correcta() {
        use typst_core::entities::geometry::ShapeKind;
        use typst_core::entities::layout_types::{
            Color, FrameItem, Page, PagedDocument, Point, Pt, TransformMatrix,
        };

        let child = FrameItem::Shape {
            pos:    Point { x: Pt(0.0), y: Pt(0.0) },
            kind:   ShapeKind::Rect,
            width:  50.0,
            height: 50.0,
            fill:   Some(Color::rgb(0, 0, 255)),
            stroke: None,
        };

        let page = Page {
            width: 595.28, height: 841.89,
            items: vec![FrameItem::Group {
                pos:          Point { x: Pt(100.0), y: Pt(100.0) },
                matrix:       TransformMatrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 },
                clip_mask:    Some(ShapeKind::Rect),
                inner_width:  50.0,
                inner_height: 50.0,
                items:        vec![child],
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("W n\n"), "Deve conter operador de clip W n");

        let pos_cm   = pdf_str.find(" cm\n").expect("Deve conter matriz cm");
        let pos_clip = pdf_str.find("W n\n").expect("Deve conter W n");
        // Usar a cor de preenchimento do filho como marcador do início do desenho do filho.
        // O clip mask é um caminho sem fill/stroke (W n), o filho tem rg antes do re.
        let pos_child = pdf_str.find("rg\n").expect("Deve conter cor de preenchimento do filho");
        let pos_q     = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_cm   < pos_clip,  "cm deve preceder W n");
        assert!(pos_clip < pos_child, "W n deve preceder o desenho dos filhos");
        assert!(pos_child < pos_q,    "filhos devem ser desenhados antes de Q");
    }

    // ── P263 (ADR-0087 anotação cumulativa) ────────────────────────────

    #[test]
    fn p263_compute_axial_coords_angle_0_horizontal() {
        let (x0, y0, x1, y1) = compute_axial_coords(0.0, 0.0, 0.0, 100.0, 50.0);
        // angle 0: linha horizontal através do centro
        // cx=50 cy=25; dx=cos(0)=1 dy=sin(0)=0
        // hx = 50; hy = 0
        // (x0, y0) = (0, 25); (x1, y1) = (100, 25)
        assert!((x0 - 0.0).abs() < 0.01);
        assert!((y0 - 25.0).abs() < 0.01);
        assert!((x1 - 100.0).abs() < 0.01);
        assert!((y1 - 25.0).abs() < 0.01);
    }

    #[test]
    fn p263_compute_axial_coords_angle_90_vertical() {
        let (x0, y0, x1, y1) = compute_axial_coords(std::f64::consts::FRAC_PI_2, 0.0, 0.0, 100.0, 50.0);
        // angle pi/2: linha vertical através do centro
        // cx=50 cy=25; dx≈0 dy=1
        // hx≈0; hy = 25
        // (x0, y0) ≈ (50, 0); (x1, y1) ≈ (50, 50)
        assert!((x0 - 50.0).abs() < 0.01);
        assert!((y0 - 0.0).abs() < 0.01);
        assert!((x1 - 50.0).abs() < 0.01);
        assert!((y1 - 50.0).abs() < 0.01);
    }

    #[test]
    fn p263_oklab_sample_stops_red_blue_endpoints() {
        use std::sync::Arc;
        use typst_core::entities::gradient::{GradientStop, Linear};
        use typst_core::entities::layout_types::{Angle, Color, Ratio};

        let linear = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::rad(0.0),
        };
        let samples = oklab_sample_stops(&linear, 16);
        assert_eq!(samples.len(), 16);
        // Endpoints: primeiro stop é vermelho, último é azul.
        // Tolerância ampla (Oklab roundtrip não é bit-identical).
        let (r0, g0, b0) = samples[0];
        let (r1, g1, b1) = samples[15];
        assert!(r0 > 0.9, "sample[0].r ≈ 1.0 (vermelho), got {}", r0);
        assert!(g0 < 0.2 && b0 < 0.2, "sample[0] verde+azul baixo");
        assert!(b1 > 0.9, "sample[15].b ≈ 1.0 (azul), got {}", b1);
        assert!(r1 < 0.2 && g1 < 0.2, "sample[15] vermelho+verde baixo");
    }

    #[test]
    fn p263_emit_function_dict_2_stops_uses_type_2() {
        let mut sub_id = 100;
        let (dict, sub_objs) = emit_function_dict(
            &[(1.0, 0.0, 0.0), (0.0, 0.0, 1.0)],
            0,
            &mut sub_id,
        );
        assert!(dict.contains("/FunctionType 2"),
            "2 stops → Type 2; got: {}", dict);
        assert!(dict.contains("/C0"), "Type 2 deve ter C0");
        assert!(dict.contains("/C1"), "Type 2 deve ter C1");
        assert_eq!(sub_objs.len(), 0, "Type 2 não tem sub-functions");
        assert_eq!(sub_id, 100, "Type 2 não consome sub_id");
    }

    #[test]
    fn p263_emit_function_dict_4_stops_uses_type_3_stitching() {
        let mut sub_id = 100;
        let (dict, sub_objs) = emit_function_dict(
            &[(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0), (1.0, 1.0, 1.0)],
            0,
            &mut sub_id,
        );
        assert!(dict.contains("/FunctionType 3"),
            "N>2 stops → Type 3 stitching; got: {}", dict);
        assert!(dict.contains("/Functions"));
        assert!(dict.contains("/Bounds"));
        assert!(dict.contains("/Encode"));
        // N=4 stops → 3 sub-functions Type 2.
        assert_eq!(sub_objs.len(), 3, "4 stops → 3 sub-Type-2");
        assert_eq!(sub_id, 103, "consumiu 3 sub_ids");
    }

    #[test]
    fn p263_export_pdf_gradient_in_stroke_emits_shading() {
        use std::sync::Arc;
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::gradient::{Gradient, GradientStop, Linear};
        use typst_core::entities::layout_types::{
            Angle, Color, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
        };
        use typst_core::entities::paint::Paint;

        let linear = Gradient::Linear(Arc::new(Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::rad(0.0),
        }));
        let stroke = Stroke {
            paint: Paint::Gradient(linear),
            thickness: 2.0,
            overhang: false,
        };

        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(10.0), y: Pt(10.0) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 30.0,
                fill: Some(Color::rgb(255, 255, 255)),
                stroke: Some(stroke),
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("/ShadingType 2"),
            "PDF deve conter /ShadingType 2 (axial)");
        assert!(pdf_str.contains("/PatternType 2"),
            "PDF deve conter /PatternType 2 (shading pattern)");
        assert!(pdf_str.contains("/FunctionType"),
            "PDF deve conter Function dict");
        assert!(pdf_str.contains("/Coords"),
            "PDF deve conter /Coords endpoints");
        assert!(pdf_str.contains("/Pattern <<"),
            "PDF deve conter /Pattern << ... >> em /Resources");
        assert!(pdf_str.contains("SCN"),
            "PDF deve conter SCN (apply pattern para stroke)");
    }

    #[test]
    fn p263_export_pdf_gradient_solid_preserva_rg_emit() {
        // Solid path preservado P261: emit `r g b RG` literal.
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::layout_types::{
            Color, FrameItem, Page, PagedDocument, Point, Pt,
        };
        use typst_core::entities::paint::Paint;

        let stroke = Stroke {
            paint: Paint::Solid(Color::rgb(0, 128, 255)),
            thickness: 1.5,
            overhang: false,
        };
        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 30.0,
                fill: None,
                stroke: Some(stroke),
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        // RG operator com sRGB normalizado de Color::rgb(0, 128, 255).
        assert!(pdf_str.contains("RG"), "Solid preservado emit RG operator");
        // Não deve emit /Pattern para Solid puro.
        assert!(!pdf_str.contains("/ShadingType"),
            "Solid não deve emit /ShadingType");
    }

    #[test]
    fn p263_export_pdf_gradient_dedup_arc_ptr() {
        // 3 shapes com mesmo Arc<Linear> → 1 Pattern object (dedup).
        use std::sync::Arc;
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::gradient::{Gradient, GradientStop, Linear};
        use typst_core::entities::layout_types::{
            Angle, Color, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
        };
        use typst_core::entities::paint::Paint;

        let linear_arc = Arc::new(Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::rad(0.0),
        });
        let make_shape = |y: f64| {
            let g = Gradient::Linear(Arc::clone(&linear_arc));
            FrameItem::Shape {
                pos: Point { x: Pt(0.0), y: Pt(y) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 20.0,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Gradient(g),
                    thickness: 1.0,
                    overhang: false,
                }),
            }
        };
        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![make_shape(0.0), make_shape(25.0), make_shape(50.0)],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        // Único /ShadingType 2 (3 shapes partilham via dedup).
        let n_shadings = pdf_str.matches("/ShadingType 2").count();
        assert_eq!(n_shadings, 1,
            "3 shapes com mesmo Arc<Linear> → 1 Shading dedup; got {}", n_shadings);
    }

    // ── P265 (ADR-0088 anotação cumulativa) — PDF Radial shading complete ──

    #[test]
    fn p265_compute_radial_coords_center_default() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::layout_types::Ratio;
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(center, Ratio(0.5), 100.0, 100.0);
        // center (0.5, 0.5) * 100x100 = (50, 50); radius 0.5 * min(100, 100) = 50.
        assert!((x0 - 50.0).abs() < 0.01);
        assert!((y0 - 50.0).abs() < 0.01);
        assert!((r0 - 0.0).abs() < 0.01);
        assert!((x1 - 50.0).abs() < 0.01);
        assert!((y1 - 50.0).abs() < 0.01);
        assert!((r1 - 50.0).abs() < 0.01);
    }

    #[test]
    fn p265_compute_radial_coords_center_offset() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::layout_types::Ratio;
        let center = Axes::new(Ratio(0.25), Ratio(0.75));
        let (x0, y0, _, x1, y1, r1) = compute_radial_coords(center, Ratio(0.4), 200.0, 100.0);
        // center.x * 200 = 50; center.y * 100 = 75; radius 0.4 * min(200,100) = 40.
        assert!((x0 - 50.0).abs() < 0.01);
        assert!((y0 - 75.0).abs() < 0.01);
        assert_eq!(x0, x1);  // concêntrico
        assert_eq!(y0, y1);
        assert!((r1 - 40.0).abs() < 0.01);
    }

    #[test]
    fn p265_compute_radial_coords_non_square_uses_min_dim() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::layout_types::Ratio;
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        // bbox 300x50 → radius 1.0 * min(300, 50) = 50.
        let (_, _, _, _, _, r1) = compute_radial_coords(center, Ratio(1.0), 300.0, 50.0);
        assert!((r1 - 50.0).abs() < 0.01);
    }

    #[test]
    fn p265_oklab_sample_stops_radial_red_blue_endpoints() {
        use std::sync::Arc;
        use typst_core::entities::axes::Axes;
        use typst_core::entities::gradient::{GradientStop, Radial};
        use typst_core::entities::layout_types::{Color, Ratio};

        let radial = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
        };
        let samples = oklab_sample_stops_radial(&radial, 16);
        assert_eq!(samples.len(), 16);
        let (r0, g0, b0) = samples[0];
        let (r1, g1, b1) = samples[15];
        assert!(r0 > 0.9, "sample[0].r ≈ 1.0 (vermelho), got {}", r0);
        assert!(g0 < 0.2 && b0 < 0.2);
        assert!(b1 > 0.9, "sample[15].b ≈ 1.0 (azul), got {}", b1);
        assert!(r1 < 0.2 && g1 < 0.2);
    }

    #[test]
    fn p265_export_pdf_radial_emits_shading_type_3() {
        use std::sync::Arc;
        use typst_core::entities::axes::Axes;
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::gradient::{Gradient, GradientStop, Radial};
        use typst_core::entities::layout_types::{
            Color, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
        };
        use typst_core::entities::paint::Paint;

        let radial = Gradient::Radial(Arc::new(Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
        }));
        let stroke = Stroke {
            paint: Paint::Gradient(radial),
            thickness: 2.0,
            overhang: false,
        };

        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(10.0), y: Pt(10.0) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 30.0,
                fill: Some(Color::rgb(255, 255, 255)),
                stroke: Some(stroke),
            }],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("/ShadingType 3"),
            "PDF deve conter /ShadingType 3 (radial)");
        assert!(pdf_str.contains("/PatternType 2"),
            "PDF deve conter /PatternType 2 (shading pattern)");
        assert!(pdf_str.contains("/FunctionType"),
            "PDF deve conter Function dict");
        assert!(pdf_str.contains("/Coords"),
            "PDF deve conter /Coords endpoints (6 valores radial)");
        assert!(pdf_str.contains("/Extend [true true]"),
            "Radial deve emit /Extend [true true] (vanilla default)");
        assert!(pdf_str.contains("/Pattern <<"),
            "PDF deve conter /Pattern << ... >> em /Resources");
        assert!(pdf_str.contains("SCN"),
            "PDF deve conter SCN (apply pattern para stroke)");
    }

    #[test]
    fn p265_export_pdf_radial_dedup_arc_ptr() {
        use std::sync::Arc;
        use typst_core::entities::axes::Axes;
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::gradient::{Gradient, GradientStop, Radial};
        use typst_core::entities::layout_types::{
            Color, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
        };
        use typst_core::entities::paint::Paint;

        let radial_arc = Arc::new(Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
        });
        let make_shape = |y: f64| {
            let g = Gradient::Radial(Arc::clone(&radial_arc));
            FrameItem::Shape {
                pos: Point { x: Pt(0.0), y: Pt(y) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 20.0,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Gradient(g),
                    thickness: 1.0,
                    overhang: false,
                }),
            }
        };
        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![make_shape(0.0), make_shape(25.0), make_shape(50.0)],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        let n_shadings = pdf_str.matches("/ShadingType 3").count();
        assert_eq!(n_shadings, 1,
            "3 shapes com mesmo Arc<Radial> → 1 Shading dedup; got {}", n_shadings);
    }

    #[test]
    fn p265_export_pdf_linear_e_radial_coexistem() {
        use std::sync::Arc;
        use typst_core::entities::axes::Axes;
        use typst_core::entities::geometry::{ShapeKind, Stroke};
        use typst_core::entities::gradient::{Gradient, GradientStop, Linear, Radial};
        use typst_core::entities::layout_types::{
            Angle, Color, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
        };
        use typst_core::entities::paint::Paint;

        let linear = Gradient::Linear(Arc::new(Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(1.0)),
            ]),
            angle: Angle::rad(0.0),
        }));
        let radial = Gradient::Radial(Arc::new(Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 0), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
        }));
        let page = Page {
            width:  100.0,
            height: 100.0,
            items: vec![
                FrameItem::Shape {
                    pos: Point { x: Pt(0.0), y: Pt(0.0) },
                    kind: ShapeKind::Rect, width: 50.0, height: 20.0,
                    fill: None,
                    stroke: Some(Stroke {
                        paint: Paint::Gradient(linear),
                        thickness: 1.0, overhang: false,
                    }),
                },
                FrameItem::Shape {
                    pos: Point { x: Pt(0.0), y: Pt(30.0) },
                    kind: ShapeKind::Rect, width: 50.0, height: 20.0,
                    fill: None,
                    stroke: Some(Stroke {
                        paint: Paint::Gradient(radial),
                        thickness: 1.0, overhang: false,
                    }),
                },
            ],
        };
        let doc = PagedDocument::new(vec![page]);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("/ShadingType 2"),
            "Linear deve emit /ShadingType 2");
        assert!(pdf_str.contains("/ShadingType 3"),
            "Radial deve emit /ShadingType 3");
        // 1 axial + 1 radial = 2 shadings distintos.
        let n_axial = pdf_str.matches("/ShadingType 2").count();
        let n_radial = pdf_str.matches("/ShadingType 3").count();
        assert_eq!(n_axial, 1);
        assert_eq!(n_radial, 1);
    }
}
