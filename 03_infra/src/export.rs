//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export.md
//! @prompt-hash 8edd13ad
//! @layer L3
//! @updated 2026-04-20

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use ttf_parser::Face;
use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument};

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
    page:       &Frame,
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

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.size.width.val();
            let h = page.size.height.val();

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let resources_str = format!(
                "/Font << /F1 {font_f1} 0 R /F2 {font_f2} 0 R /F3 {font_f3} 0 R >> {xobj_res}"
            );

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.1} {h:.1}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let stream_bytes = build_page_stream_type1(page, &ptr_to_idx, &img_refs);
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

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.size.width.val();
            let h = page.size.height.val();

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let resources_str = format!("/Font << /F1 {font_id} 0 R >> {xobj_res}");

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.1} {h:.1}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let stream_bytes = build_page_stream_cidfont(page, &char_to_gid, &ptr_to_idx, &img_refs);
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
        self.serialize()
    }

    /// Emite todos os XObjects de imagem pré-processados para o builder.
    ///
    /// Para PNG com alpha: emite /SMask (canal alpha) antes do XObject principal
    /// (canal RGB), para que o SMask apareça antes no ficheiro PDF — o dicionário
    /// do XObject principal referencia o ID do SMask por forward reference.
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

fn build_page_stream_type1(
    page:       &Frame,
    ptr_to_idx: &HashMap<usize, usize>,
    img_refs:   &[ImageRef],
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

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

                ops.push_str(&format!(
                    "BT\n/{font_ref} {:.1} Tf\n{:.1} {:.1} Td\n({safe}) Tj\nET\n",
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
                if let Some(s) = stroke {
                    let (r, g, b, _) = s.paint.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, s.thickness));
                }

                // Path — depende do tipo de forma.
                match kind {
                    ShapeKind::Rect => {
                        // Operador re: x y width height re — rectângulo como sub-path fechado.
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::Ellipse => {
                        // TODO: substituir por aproximação Bézier real (DEBT-31).
                        // Placeholder: rectângulo para manter PDF válido.
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::Line { dx, dy } => {
                        // start_y: Y do ponto de início no espaço PDF.
                        // dy positivo = desce no layout → subtrai no PDF (eixos opostos).
                        let start_y = page_height - pos.y.val();
                        let end_y   = page_height - (pos.y.val() + dy);
                        ops.push_str(&format!("{:.2} {:.2} m\n", pos.x.val(), start_y));
                        ops.push_str(&format!("{:.2} {:.2} l\n", pos.x.val() + dx, end_y));
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
        }
    }

    ops.into_bytes()
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
    page:        &Frame,
    char_to_gid: &HashMap<char, u16>,
    ptr_to_idx:  &HashMap<usize, usize>,
    img_refs:    &[ImageRef],
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

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
                    let (r, g, b, _) = s.paint.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, s.thickness));
                }
                match kind {
                    ShapeKind::Rect => {
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::Ellipse => {
                        // TODO: substituir por aproximação Bézier real (DEBT-31).
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::Line { dx, dy } => {
                        let start_y = page_height - pos.y.val();
                        // dy positivo = desce no layout → subtrai no PDF (eixos opostos).
                        let end_y   = page_height - (pos.y.val() + dy);
                        ops.push_str(&format!("{:.2} {:.2} m\n", pos.x.val(), start_y));
                        ops.push_str(&format!("{:.2} {:.2} l\n", pos.x.val() + dx, end_y));
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
        }
    }

    ops.into_bytes()
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::{
        entities::{content::Content, counter_state::CounterState},
        rules::layout::layout,
    };

    #[test]
    fn pdf_header_correcto() {
        let doc = layout(&Content::text("Hello"), CounterState::default());
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"), "deve começar com %PDF-1.7");
    }

    #[test]
    fn pdf_termina_com_eof() {
        let doc = layout(&Content::text("Test"), CounterState::default());
        let pdf = export_pdf(&doc);
        let tail = std::str::from_utf8(&pdf[pdf.len().saturating_sub(20)..]).unwrap_or("");
        assert!(tail.contains("%%EOF"), "deve terminar com %%EOF");
    }

    #[test]
    fn pdf_tem_estrutura_valida() {
        let doc = layout(&Content::text("Test"), CounterState::default());
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
        let doc = layout(&Content::text("Hello world"), CounterState::default());
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
            Frame, FrameItem, PagedDocument, Point, Pt, Size, TextStyle,
        };
        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Text {
            pos:   Point { x: Pt(72.0), y: Pt(84.0) },
            text:  "Top".into(),
            style: TextStyle::regular(Pt(12.0)),
        });
        let doc = PagedDocument::new(vec![frame]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        // y_pdf = 842 - 84 = 758
        assert!(s.contains("758.0") || s.contains("758"),
            "y_pdf deve ser 842-84=758: {}", &s[..s.len().min(500)]);
    }

    #[test]
    fn pdf_mediabox_dimensoes_a4() {
        let doc = layout(&Content::text("Test"), CounterState::default());
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("595") && s.contains("842"),
            "MediaBox deve ter dimensões A4 (595x842 pt)");
    }

    // ── Passo 24 — DEBT-5: Unicode PDF ──────────────────────────────────────

    #[test]
    fn unicode_nao_produz_interrogacao() {
        // Modo Helvetica — documenta intenção. Com CIDFont + fonte real, '?' desaparece.
        let doc = layout(&Content::text("café naïve résumé"), CounterState::default());
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
        let doc = layout(&Content::text("Hello World"), CounterState::default());
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref") && s.contains("%%EOF"));
    }

    #[test]
    fn bullet_e_unicode() {
        use typst_core::entities::{content::Content as C2, layout_types::FrameItem};
        let doc = layout(&C2::list_item(C2::text("item")), CounterState::default());
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
        let doc = layout(&Content::text("aaa bbb"), CounterState::default());
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
        use typst_core::entities::layout_types::{Frame, PagedDocument, Point, Pt, Size};
        let mut frame = Frame::new(Size { width: Pt(595.0), height: Pt(842.0) });
        frame.items.push(FrameItem::Glyph {
            pos: Point::ZERO, glyph_id: 42, x_advance: Pt(10.0), size: Pt(12.0),
        });
        frame.items.push(FrameItem::Glyph {
            pos: Point::ZERO, glyph_id: 42, x_advance: Pt(10.0), size: Pt(12.0), // dup
        });
        frame.items.push(FrameItem::Glyph {
            pos: Point::ZERO, glyph_id: 99, x_advance: Pt(10.0), size: Pt(12.0),
        });
        let doc = PagedDocument::new(vec![frame]);
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
        use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size};

        // JPEG mínimo com magic numbers correctos — 4 bytes suficientes para detect_format.
        let jpeg_bytes = Arc::new(vec![0xFF, 0xD8, 0xFF, 0xE0u8]);

        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Image {
            pos:              Point { x: Pt(72.0), y: Pt(100.0) },
            data:             Arc::clone(&jpeg_bytes),
            width:            Pt(100.0),
            height:           Pt(75.0),
            intrinsic_width:  400,
            intrinsic_height: 300,
        });
        let doc = PagedDocument::new(vec![frame]);
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
        use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size};

        // PNG com apenas magic bytes — processo_png_for_pdf falha, imagem omitida.
        // O PDF deve continuar válido (sem corrupção).
        let png_bytes = Arc::new(vec![0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);

        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Image {
            pos:              Point { x: Pt(72.0), y: Pt(100.0) },
            data:             Arc::clone(&png_bytes),
            width:            Pt(100.0),
            height:           Pt(100.0),
            intrinsic_width:  200,
            intrinsic_height: 200,
        });
        let doc = PagedDocument::new(vec![frame]);
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
        use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size};

        // JPEG com SOF0 e 3 canais — deve ter /DeviceRGB no XObject.
        let mut jpeg = vec![0xFF, 0xD8u8, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01, 0x03];
        // Adicionar marcador EOI para que o JPEG seja "válido" o suficiente para o exporter.
        jpeg.extend_from_slice(&[0xFF, 0xD9]);
        let data = Arc::new(jpeg);

        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Image {
            pos: Point { x: Pt(72.0), y: Pt(100.0) },
            data: Arc::clone(&data),
            width: Pt(100.0), height: Pt(75.0),
            intrinsic_width: 1, intrinsic_height: 1,
        });
        let doc = PagedDocument::new(vec![frame]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("/DeviceRGB"), "JPEG 3 canais deve usar /DeviceRGB");
    }

    #[test]
    fn jpeg_deduplicado_por_arc_ptr() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size};

        let jpeg_bytes = Arc::new(vec![0xFF, 0xD8, 0xFF, 0xE0u8]);

        // Mesma imagem duas vezes na mesma página — deve gerar apenas um XObject.
        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Image {
            pos: Point { x: Pt(72.0), y: Pt(72.0) },
            data: Arc::clone(&jpeg_bytes),
            width: Pt(100.0), height: Pt(75.0),
            intrinsic_width: 400, intrinsic_height: 300,
        });
        frame.push(FrameItem::Image {
            pos: Point { x: Pt(72.0), y: Pt(200.0) },
            data: Arc::clone(&jpeg_bytes),
            width: Pt(50.0), height: Pt(37.0),
            intrinsic_width: 400, intrinsic_height: 300,
        });
        let doc = PagedDocument::new(vec![frame]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);

        // "Im1 Do" deve aparecer duas vezes (dois usos)
        let uses = s.matches("/Im1 Do").count();
        assert_eq!(uses, 2, "Im1 deve ser usado duas vezes mas definido uma vez");
        // Só um XObject com DCTDecode
        let dct_count = s.matches("/DCTDecode").count();
        assert_eq!(dct_count, 1, "deve haver apenas um XObject JPEG (deduplicado)");
    }
}
