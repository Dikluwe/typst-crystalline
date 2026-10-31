//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/scan.md
//! @prompt-hash 35b14166
//! @layer L3
//! @updated 2026-10-26

use super::jpeg::jpeg_is_rgb;
use super::png::process_png_for_pdf;
use super::{detect_image_format, ImageFormat, ImageRef, ImageXObject, PdfImagePayload};
use crate::export::tiling::walk_resources as walk_frame_items;
mod reader;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

// P876 — cache thread-local de `PdfImagePayload` processados, indexado pelo
// ponteiro do `Arc<Vec<u8>>` original. Como `PagedDocument` mantém os Arcs
// vivos durante o export, o endereço é uma chave estável e segura.
thread_local! {
    static PAYLOAD_CACHE: RefCell<HashMap<usize, PdfImagePayload>> =
        RefCell::new(HashMap::new());
}

struct ImageScanCursor {
    next_id: usize,
    counter: usize,
}

/// **P833 (#18, GRAVE)** — valida TODAS as imagens do documento antes do
/// export, falhando a compilação com a mensagem do vanilla
/// (`failed to decode image ({detalhe})`) quando alguma não descodifica.
///
/// Antes desta função, uma imagem com assinatura válida mas corrompida era
/// omitida no export com um aviso retornado ao caller e PDF sem a imagem.
/// O vanilla descodifica toda a imagem raster em avaliação
/// (`RasterImage::new` → `format_image_error`, `visualize/image/raster.rs:448`)
/// e rejeita o documento. O cristalino não descodifica em L1 (whitelist de
/// deps); a validação acontece aqui, em L3, no pipeline antes do export —
/// o erro sobe como `SourceDiagnostic` com span detached (nuance registada:
/// sem a posição do `#image(...)`, que o vanilla aponta).
///
/// JPEG é verificado por descodificação completa (o export embute o JPEG cru
/// sem descodificar — sem esta verificação um JPEG corrompido produzia um
/// PDF inválido em silêncio). GIF/WebP (P833/#17) seguem o mesmo caminho de
/// descodificação do PNG.
pub(crate) fn validate_document_images(doc: &PagedDocument) -> Result<(), String> {
    struct ImageValidator {
        result: Result<(), String>,
        validated: HashSet<usize>,
    }
    impl FrameVisitor for ImageValidator {
        fn visit_image(&mut self, item: &FrameItem) {
            if self.result.is_err() {
                return;
            }
            if let FrameItem::Image { data, .. } = item {
                // The document keeps every Arc alive for this invocation.
                let identity = Arc::as_ptr(data) as usize;
                if self.validated.contains(&identity) {
                    return;
                }
                if let Err(e) = validate_image_data(data) {
                    self.result = Err(e);
                } else {
                    self.validated.insert(identity);
                }
            }
        }
    }
    let mut validator = ImageValidator { result: Ok(()), validated: HashSet::new() };
    for page in &doc.pages {
        for items in [&page.background, &page.items, &page.foreground] {
            walk_frame_items(&mut validator, items);
        }
        if validator.result.is_err() {
            return validator.result;
        }
    }
    Ok(())
}

/// Valida uma imagem por descodificação completa, com a mensagem no formato
/// do vanilla (`failed to decode image ({detalhe})` — `format_image_error`).
fn validate_image_data(data: &[u8]) -> Result<(), String> {
    #[cfg(test)]
    tests::record_validation_call();
    match detect_image_format(data) {
        ImageFormat::Jpeg => {
            use image::ImageDecoder;
            image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(data))
                .and_then(|mut decoder| {
                    // Match load_from_memory's limits before decoding any pixels.
                    let mut limits = image::io::Limits::default();
                    limits.reserve(decoder.total_bytes())?;
                    decoder.set_limits(limits)?;
                    if jpeg_scanline_candidate(data)
                        && if decoder.color_type() == image::ColorType::L8 {
                            jpeg_gray_native_complete(data)
                                || jpeg_scanline_complete::<true>(data)
                        } else {
                            jpeg_scanline_complete::<false>(data)
                        }
                    {
                        return Ok(());
                    }
                    // In image 0.24 this eagerly decodes, without read_image's extra copy.
                    #[allow(deprecated)]
                    let pixels = decoder.into_reader()?;
                    drop(pixels);
                    Ok(())
                })
                .map_err(|e| format!("failed to decode image ({e})"))
        }
        ImageFormat::Png | ImageFormat::Gif | ImageFormat::WebP => {
            process_png_for_pdf(data)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"))
        }
        // Unknown: a avaliação (`native_image`) já rejeita com
        // "unknown image format" antes de chegar aqui — nada a validar.
        ImageFormat::Unknown => Ok(()),
    }
}

/// Admit only the single-scan baseline subset covered by the compatibility gate.
/// Rejection selects the legacy decoder; this does not validate the entropy data.
fn jpeg_scanline_candidate(data: &[u8]) -> bool {
    if !data.starts_with(&[0xff, 0xd8]) {
        return false;
    }
    let mut pos = 2usize;
    let mut components = None;
    while data.get(pos) == Some(&0xff) {
        let Some(&marker) = data.get(pos + 1) else {
            return false;
        };
        if matches!(marker, 0xd8 | 0xd9 | 0x00 | 0x01 | 0xd0..=0xd7) {
            return false;
        }
        let Some(length) = data.get(pos + 2..pos + 4) else {
            return false;
        };
        let length = usize::from(u16::from_be_bytes([length[0], length[1]]));
        let Some(end) = pos
            .checked_add(2 + length)
            .filter(|end| length >= 2 && *end <= data.len())
        else {
            return false;
        };
        if marker == 0xc0 {
            if components.is_some() || length < 8 || data.get(pos + 4) != Some(&8) {
                return false;
            }
            let n = data[pos + 9];
            if !matches!(n, 1 | 3) || length != 8 + 3 * usize::from(n) {
                return false;
            }
            components = Some(n);
        } else if marker == 0xdb {
            if length < 67 {
                return false;
            }
            let mut q = pos + 4;
            while q < end {
                if data[q] >> 4 != 0 || data[q] & 15 > 3 {
                    return false;
                }
                let Some(values) = data.get(q + 1..q + 65).filter(|_| q + 65 <= end)
                else {
                    return false;
                };
                if values.contains(&0) {
                    return false;
                }
                q += 65;
            }
        } else if marker == 0xc4 {
            let mut h = pos + 4;
            while h < end {
                let selector = data[h];
                h += 1;
                if selector >> 4 > 1 || selector & 15 > 1 {
                    return false;
                }
                let Some(counts) = data.get(h..h + 16).filter(|_| h + 16 <= end) else {
                    return false;
                };
                let n: usize = counts.iter().map(|x| usize::from(*x)).sum();
                h += 16;
                let Some(values) = data.get(h..h + n).filter(|_| h + n <= end) else {
                    return false;
                };
                if n == 0
                    || values.iter().any(|v| {
                        if selector >> 4 == 0 {
                            *v > 11
                        } else {
                            v & 15 > 10 || (v & 15 == 0 && !matches!(*v, 0 | 0xf0))
                        }
                    })
                {
                    return false;
                }
                h += n;
            }
        } else if marker == 0xda {
            let Some(n) = components else {
                return false;
            };
            if length != 6 + 2 * usize::from(n)
                || data.get(pos + 4) != Some(&n)
                || data.get(end - 3..end) != Some(&[0, 63, 0])
            {
                return false;
            }
            return jpeg_entropy_markers_valid(&data[end..]);
        } else if !matches!(marker, 0xdb | 0xdd | 0xe0..=0xef | 0xfe)
            || (components.is_some() && marker != 0xdd)
        {
            return false;
        }
        pos = end;
    }
    false
}

/// Check byte stuffing and terminal EOI only; Huffman/IDCT decoding stays separate.
#[inline(never)]
fn jpeg_entropy_markers_valid(data: &[u8]) -> bool {
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    {
        use std::arch::x86_64::{
            _mm_andnot_si128, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8,
            _mm_set1_epi8, _mm_setzero_si128,
        };
        if !data.ends_with(&[0xff, 0xd9]) {
            return false;
        }
        let body_end = data.len() - 2;
        let mut i = 0;
        // SAFETY: cfg guarantees SSE2. The loop maintains i <= body_end and
        // checks at least 16 body bytes. Both 16-byte loads (at i and i+1)
        // fit inside data, including the terminal EOI lookahead. loadu does
        // not require alignment. No pointer escapes this input's lifetime.
        unsafe {
            let ff = _mm_set1_epi8(-1);
            let zero = _mm_setzero_si128();
            while body_end - i >= 16 {
                let current = _mm_loadu_si128(data.as_ptr().add(i).cast());
                let next = _mm_loadu_si128(data.as_ptr().add(i + 1).cast());
                let bad = _mm_andnot_si128(
                    _mm_cmpeq_epi8(next, zero),
                    _mm_cmpeq_epi8(current, ff),
                );
                if _mm_movemask_epi8(bad) != 0 {
                    return false;
                }
                i += 16;
            }
        }
        #[cfg(test)]
        let mut scalar_probes = 0;
        while i < body_end {
            #[cfg(test)]
            {
                scalar_probes += 1;
            }
            if data[i] == 0xff && data[i + 1] != 0 {
                #[cfg(test)]
                tests::record_scalar_entropy_probes(scalar_probes);
                return false;
            }
            i += 1;
        }
        #[cfg(test)]
        tests::record_scalar_entropy_probes(scalar_probes);
        true
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
    {
        let mut i = 0;
        #[cfg(test)]
        let mut scalar_probes = 0;
        while let Some(delta) = data[i..].iter().position(|b| {
            #[cfg(test)]
            {
                scalar_probes += 1;
            }
            *b == 0xff
        }) {
            i += delta;
            match data.get(i + 1) {
                Some(0) => i += 2,
                Some(0xd9) => {
                    #[cfg(test)]
                    tests::record_scalar_entropy_probes(scalar_probes);
                    return i + 2 == data.len();
                }
                _ => {
                    #[cfg(test)]
                    tests::record_scalar_entropy_probes(scalar_probes);
                    return false;
                }
            }
        }
        #[cfg(test)]
        tests::record_scalar_entropy_probes(scalar_probes);
        false
    }
}

/// Try native reconstructed stripes for an admitted one-component baseline JPEG.
fn jpeg_gray_native_complete(data: &[u8]) -> bool {
    use zune_core::{colorspace::ColorSpace, options::DecoderOptions};
    use zune_jpeg::{JpegDecoder, RawImcuRowStatus};
    let options = DecoderOptions::default()
        .set_strict_mode(true)
        .set_max_width(65535)
        .set_max_height(65535)
        .jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder =
        JpegDecoder::new_with_options(reader::BorrowedJpegReader::new(data), options);
    if decoder.decode_headers().is_err() {
        return false;
    }
    let mut raw = decoder.raw_output();
    if raw.num_components() != Some(1) {
        return false;
    }
    let Some(layout) = raw.layout() else {
        return false;
    };
    let plane = layout[0];
    if plane.horizontal_sampling_factor != 1
        || plane.vertical_sampling_factor != 1
        || plane.width == 0
        || plane.height == 0
    {
        return false;
    }
    let Some(stripe_bytes) = plane.width.checked_mul(8) else {
        return false;
    };
    let mut stripe = vec![0; stripe_bytes];
    let mut remaining = plane.height;
    for _ in 0..=plane.height.div_ceil(8) {
        match raw.decode_next_imcu_row(&mut [&mut stripe], &[plane.width]) {
            Ok(RawImcuRowStatus::RowReady { rows_written }) => {
                if remaining == 0 {
                    return false;
                }
                if rows_written[0] != remaining.min(8) {
                    return false;
                }
                remaining -= rows_written[0];
            }
            Ok(RawImcuRowStatus::Complete) => return remaining == 0,
            _ => return false,
        }
    }
    false
}

/// Decode and discard all converted stripes; incomplete input is never success.
fn jpeg_scanline_complete<const GRAYSCALE: bool>(data: &[u8]) -> bool {
    use zune_core::{
        bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions,
    };
    use zune_jpeg::{JpegDecoder, ScanlineStatus};
    let options = DecoderOptions::default()
        .set_strict_mode(true)
        .set_max_width(65535)
        .set_max_height(65535);
    // Validation discards the decoded stripes; do not expand a gray channel
    // into RGB. Keep color inputs on their existing complete RGB path.
    let options = if GRAYSCALE {
        options.jpeg_set_out_colorspace(ColorSpace::Luma)
    } else {
        options
    };
    let mut decoder = JpegDecoder::new_with_options(ZCursor::new(data), options);
    let mut rows = decoder.scanline_output();
    matches!(rows.start(), Ok(ScanlineStatus::Ready))
        && matches!(rows.finish(), Ok(ScanlineStatus::Complete))
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
pub(in crate::export) fn scan_all_images(
    doc: &PagedDocument,
    first_id: usize,
    icc_profile_id: Option<usize>,
    warnings: &mut Vec<SourceDiagnostic>,
) -> (Vec<ImageRef>, HashMap<usize, usize>, Vec<ImageXObject>) {
    // P279 — helper recursivo (scope creep análogo P273.10 §A.7 para scan_all_gradients).
    // Bug latent pré-existente: scan_all_images iterava apenas page.items top-level;
    // Images dentro de Groups (via Content::Transform / Block clip / etc.) não eram
    // registadas. Sem fix, Image arm em draw_item_local (P279) teria nada a lookup.
    struct ImageScanner<'a> {
        ptr_to_idx: &'a mut HashMap<usize, usize>,
        refs: &'a mut Vec<ImageRef>,
        xobjects: &'a mut Vec<ImageXObject>,
        cursor: &'a mut ImageScanCursor,
        warnings: &'a mut Vec<SourceDiagnostic>,
        icc_profile_id: Option<usize>,
    }
    impl<'a> FrameVisitor for ImageScanner<'a> {
        fn visit_image(&mut self, item: &FrameItem) {
            process_image_item(
                item,
                self.ptr_to_idx,
                self.refs,
                self.xobjects,
                self.cursor,
                self.warnings,
                self.icc_profile_id,
            );
        }
    }

    let mut ptr_to_idx: HashMap<usize, usize> = HashMap::new();
    let mut refs: Vec<ImageRef> = Vec::new();
    let mut xobjects: Vec<ImageXObject> = Vec::new();
    let mut cursor = ImageScanCursor { next_id: first_id, counter: 1 };
    let mut scanner = ImageScanner {
        ptr_to_idx: &mut ptr_to_idx,
        refs: &mut refs,
        xobjects: &mut xobjects,
        cursor: &mut cursor,
        warnings,
        icc_profile_id,
    };

    for page in &doc.pages {
        for items in [&page.background, &page.items, &page.foreground] {
            walk_frame_items(&mut scanner, items);
        }
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
    cursor: &mut ImageScanCursor,
    warnings: &mut Vec<SourceDiagnostic>,
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
    let name = format!("Im{}", cursor.counter);
    cursor.counter += 1;

    match detect_image_format(data) {
        ImageFormat::Jpeg => {
            let main_id = cursor.next_id;
            cursor.next_id += 1;
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
        ImageFormat::Png | ImageFormat::Gif | ImageFormat::WebP => {
            // P876 — cache de payload por ponteiro Arc. A deduplicação via
            // `ptr_to_idx` já evita XObjects duplicados; o cache evita
            // reprocessar/decomprimir a mesma imagem raster mais do que uma vez.
            let payload: Result<PdfImagePayload, String> = PAYLOAD_CACHE.with(|c| {
                if let Some(p) = c.borrow().get(&ptr).cloned() {
                    return Ok(p);
                }
                let p = process_png_for_pdf(data)?;
                c.borrow_mut().insert(ptr, p.clone());
                Ok(p)
            });
            match payload {
                Ok(payload) => {
                    // Alocar ID do /SMask antes do ID principal para que smask
                    // apareça primeiro no ficheiro PDF (xref em ordem crescente).
                    let smask_id = if payload.alpha_data_compressed.is_some() {
                        let id = cursor.next_id;
                        cursor.next_id += 1;
                        Some(id)
                    } else {
                        None
                    };
                    let main_id = cursor.next_id;
                    cursor.next_id += 1;
                    refs.push(ImageRef { main_obj_id: main_id, name });
                    xobjects.push(ImageXObject::Png {
                        payload,
                        main_obj_id: main_id,
                        smask_obj_id: smask_id,
                    });
                    ptr_to_idx.insert(ptr, idx);
                }
                Err(e) => {
                    // P833 (#18) — defensivo: em produção este ponto é
                    // inalcançável porque `validate_document_images` (pipeline)
                    // falha a compilação ANTES do export. Se algum caller da
                    // API pública de export saltar a validação, a imagem é
                    // omitida com aviso estruturado para o caller.
                    warnings.push(SourceDiagnostic::warning(
                        Span::detached(),
                        format!("failed to decode image ({e}) — imagem omitida"),
                    ));
                    // Não inserir em ptr_to_idx — imagem ignorada nas páginas.
                }
            }
        }
        ImageFormat::Unknown => {
            warnings.push(SourceDiagnostic::warning(
                Span::detached(),
                "Formato de imagem desconhecido — imagem omitida",
            ));
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
pub(in crate::export) fn xobject_resources_for_page(
    page: &Page,
    ptr_to_idx: &HashMap<usize, usize>,
    refs: &[ImageRef],
) -> String {
    struct PageImageResourceCollector<'a> {
        ptr_to_idx: &'a HashMap<usize, usize>,
        refs: &'a [ImageRef],
        seen: &'a mut BTreeSet<usize>,
        entries: &'a mut Vec<String>,
    }
    impl<'a> FrameVisitor for PageImageResourceCollector<'a> {
        fn visit_image(&mut self, item: &FrameItem) {
            if let FrameItem::Image { data, .. } = item {
                let ptr = Arc::as_ptr(data) as usize;
                if let Some(&idx) = self.ptr_to_idx.get(&ptr) {
                    if self.seen.insert(idx) {
                        let r = &self.refs[idx];
                        self.entries.push(format!("/{} {} 0 R", r.name, r.main_obj_id));
                    }
                }
            }
        }
    }

    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    let mut collector = PageImageResourceCollector {
        ptr_to_idx,
        refs,
        seen: &mut seen,
        entries: &mut entries,
    };
    for items in [&page.background, &page.items, &page.foreground] {
        walk_frame_items(&mut collector, items);
    }
    if entries.is_empty() {
        return String::new();
    }
    format!("/XObject << {} >>", entries.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::content::Content;
    use typst_core::entities::layout_types::{Point, Pt, TransformMatrix};

    thread_local! {
        static VALIDATION_CALLS: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
        static SCALAR_ENTROPY_PROBES: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
    }
    pub(super) fn record_scalar_entropy_probes(probes: usize) {
        SCALAR_ENTROPY_PROBES.with(|count| {
            if let Some(n) = count.get() {
                count.set(Some(n + probes));
            }
        });
    }
    pub(super) fn record_validation_call() {
        VALIDATION_CALLS.with(|calls| {
            if let Some(n) = calls.get() {
                calls.set(Some(n + 1));
            }
        });
    }

    #[cfg(target_os = "linux")]
    mod allocation_probe {
        use std::alloc::{GlobalAlloc, Layout, System};
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

        pub(super) static ACTIVE: AtomicBool = AtomicBool::new(false);
        pub(super) static FULL_RGB: AtomicUsize = AtomicUsize::new(0);
        pub(super) static TARGET_SIZE: AtomicUsize = AtomicUsize::new(6000 * 4000 * 3);

        struct Probe;
        #[global_allocator]
        static ALLOCATOR: Probe = Probe;

        fn record(size: usize) {
            if ACTIVE.load(Ordering::Relaxed)
                && size == TARGET_SIZE.load(Ordering::Relaxed)
            {
                FULL_RGB.fetch_add(1, Ordering::Relaxed);
            }
        }

        // SAFETY: every operation forwards the original arguments to System.
        // The counters do not allocate and never access the allocated memory.
        unsafe impl GlobalAlloc for Probe {
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                record(layout.size());
                unsafe { System.alloc(layout) }
            }
            unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
                record(layout.size());
                unsafe { System.alloc_zeroed(layout) }
            }
            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                unsafe { System.dealloc(ptr, layout) }
            }
            unsafe fn realloc(
                &self,
                ptr: *mut u8,
                layout: Layout,
                new_size: usize,
            ) -> *mut u8 {
                record(new_size);
                unsafe { System.realloc(ptr, layout, new_size) }
            }
        }
    }

    fn encoded_jpeg(width: u32, height: u32, color: image::ColorType) -> Vec<u8> {
        let pixels =
            vec![
                127;
                width as usize * height as usize * color.bytes_per_pixel() as usize
            ];
        let mut bytes = Vec::new();
        image::codecs::jpeg::JpegEncoder::new(&mut bytes)
            .encode(&pixels, width, height, color)
            .unwrap();
        bytes
    }

    #[test]
    fn jpeg_validation_preserves_full_decode_errors_and_limits() {
        for color in [image::ColorType::Rgb8, image::ColorType::L8] {
            let valid = encoded_jpeg(64, 48, color);
            assert!(validate_image_data(&valid).is_ok());
            for end in [3, 20, valid.len() / 2, valid.len() - 5] {
                let data = &valid[..end];
                let previous = image::load_from_memory(data)
                    .map(|_| ())
                    .map_err(|e| format!("failed to decode image ({e})"));
                assert_eq!(validate_image_data(data), previous, "truncated at {end}");
            }
            assert!(validate_image_data(&valid[..valid.len() / 2]).is_err());
            let mut oversized = valid;
            let sof = oversized.windows(2).position(|w| w == [0xff, 0xc0]).unwrap();
            oversized[sof + 5..sof + 9].fill(0xff);
            let previous = image::load_from_memory(&oversized)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"));
            assert!(previous.is_err());
            assert_eq!(validate_image_data(&oversized), previous);
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn jpeg_grayscale_avoids_rgb_stripe() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let fixture = std::env::temp_dir()
            .join(format!("tekt-jpeg-gray-{}-{stamp}.jpg", std::process::id()));
        std::fs::write(&fixture, encoded_jpeg(1009, 73, image::ColorType::L8)).unwrap();
        let test = format!(
            "{}::jpeg_grayscale_allocation_child",
            module_path!().split_once("::").unwrap().1
        );
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", &test, "--nocapture"])
            .env("TEKT_JPEG_GRAY_FIXTURE", &fixture)
            .output();
        std::fs::remove_file(&fixture).unwrap();
        let output = output.unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn jpeg_grayscale_allocation_child() {
        let Some(fixture) = std::env::var_os("TEKT_JPEG_GRAY_FIXTURE") else {
            return;
        };
        let bytes = std::fs::read(fixture).unwrap();
        let mut counts = Vec::new();
        // This odd width distinguishes converted/native output from padded
        // component storage. The last pass guards a full gray destination.
        for size in [1009 * 9 * 3, 1009 * 9, 1009 * 8, 1009 * 73] {
            allocation_probe::TARGET_SIZE
                .store(size, std::sync::atomic::Ordering::SeqCst);
            allocation_probe::FULL_RGB.store(0, std::sync::atomic::Ordering::SeqCst);
            allocation_probe::ACTIVE.store(true, std::sync::atomic::Ordering::SeqCst);
            let result = validate_image_data(&bytes);
            allocation_probe::ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
            result.unwrap();
            counts.push(
                allocation_probe::FULL_RGB.load(std::sync::atomic::Ordering::SeqCst),
            );
        }
        println!(
            "Grayscale allocations [RGB9, converted9, native8, full frame]: {counts:?}"
        );
        assert_eq!(counts[0], 0, "discarded RGB stripe allocated for grayscale");
        assert_eq!(counts[1], 0, "discarded converted stripe allocated for grayscale");
        assert!(counts[2] > 0, "expected native one-channel reconstructed stripe");
        assert_eq!(counts[3], 0, "full grayscale destination allocated");
    }

    #[test]
    fn jpeg_grayscale_noise_preserves_complete_validation_and_entropy_errors() {
        let (width, height) = (257, 73);
        let mut seed = 0x591a_7823_u32;
        let pixels: Vec<u8> = (0..width * height)
            .map(|_| {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                (seed >> 24) as u8
            })
            .collect();
        let mut valid = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut valid, 90)
            .encode(&pixels, width, height, image::ColorType::L8)
            .unwrap();
        assert!(jpeg_scanline_candidate(&valid));
        assert!(jpeg_gray_native_complete(&valid));
        validate_image_data(&valid).unwrap();
        for end in [valid.len() / 2, valid.len() - 50, valid.len() - 5] {
            let mut cut = valid[..end].to_vec();
            cut.extend_from_slice(&[0xff, 0xd9]);
            let legacy = image::load_from_memory(&cut)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"));
            assert_eq!(validate_image_data(&cut), legacy, "noise entropy cut {end}");
        }
    }

    #[test]
    fn jpeg_grayscale_gradient_preserves_entropy_errors() {
        let (width, height) = (257, 65);
        let pixels: Vec<u8> = (0..height)
            .flat_map(|y| (0..width).map(move |x| ((x * 3 + y * 11) % 256) as u8))
            .collect();
        let mut valid = Vec::new();
        image::codecs::jpeg::JpegEncoder::new(&mut valid)
            .encode(&pixels, width, height, image::ColorType::L8)
            .unwrap();
        validate_image_data(&valid).unwrap();
        for end in [valid.len() / 2, valid.len() - 50, valid.len() - 5] {
            let mut data = valid[..end].to_vec();
            data.extend_from_slice(&[0xff, 0xd9]);
            let legacy = image::load_from_memory(&data)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"));
            println!("Gradient entropy cut {end}/{}: legacy {legacy:?}", valid.len());
            assert_eq!(validate_image_data(&data), legacy);
        }
    }

    #[test]
    fn jpeg_grayscale_unusual_sampling_preserves_results() {
        let valid = encoded_jpeg(64, 49, image::ColorType::L8);
        let sof = valid.windows(2).position(|w| w == [0xff, 0xc0]).unwrap();
        for sampling in [0x11, 0x21, 0x12, 0x22, 0x44, 0x01, 0x10, 0xff] {
            let mut data = valid.clone();
            data[sof + 11] = sampling;
            let legacy = image::load_from_memory(&data)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"));
            assert_eq!(validate_image_data(&data), legacy, "sampling {sampling:x}");
        }
    }

    #[test]
    fn jpeg_scanline_guard_preserves_marker_boundaries() {
        let valid = encoded_jpeg(32, 24, image::ColorType::L8);
        let sos = valid.windows(2).position(|w| w == [0xff, 0xda]).unwrap();
        let end =
            sos + 2 + usize::from(u16::from_be_bytes([valid[sos + 2], valid[sos + 3]]));
        for gap in [0, 1, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128] {
            let mut data = valid[..end].to_vec();
            data.resize(end + gap, 1);
            data.extend_from_slice(&[0xff, 0, 1, 0xff, 0, 0xff, 0xd9]);
            assert!(jpeg_scanline_candidate(&data), "stuffing after {gap} bytes");
            let mut trailing = data.clone();
            trailing.push(0);
            assert!(!jpeg_scanline_candidate(&trailing));
            data[end + gap + 1] = 0xd0;
            assert!(!jpeg_scanline_candidate(&data));
        }
    }

    fn marker_test_header() -> Vec<u8> {
        let mut data = encoded_jpeg(32, 24, image::ColorType::L8);
        let sos = data.windows(2).position(|w| w == [0xff, 0xda]).unwrap();
        let end =
            sos + 2 + usize::from(u16::from_be_bytes([data[sos + 2], data[sos + 3]]));
        data.truncate(end);
        data
    }

    fn received_entropy_framing(data: &[u8]) -> bool {
        let mut i = 0;
        while let Some(delta) = data[i..].iter().position(|b| *b == 0xff) {
            i += delta;
            match data.get(i + 1) {
                Some(0) => i += 2,
                Some(0xd9) => return i + 2 == data.len(),
                _ => return false,
            }
        }
        false
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    fn jpeg_marker_guard_bounds_scalar_tail() {
        let mut data = marker_test_header();
        for _ in 0..8192 {
            data.extend_from_slice(&[0xff, 0]);
        }
        data.extend_from_slice(&[0xff, 0xd9]);
        SCALAR_ENTROPY_PROBES.with(|n| n.set(Some(0)));
        let eligible = jpeg_scanline_candidate(&data);
        let probes = SCALAR_ENTROPY_PROBES.with(|n| n.replace(None).unwrap());
        assert!(eligible);
        println!("Scalar entropy comparisons: {probes}");
        assert!(probes <= 15, "scalar walk was not restricted to the final tail");
    }

    #[test]
    fn jpeg_marker_guard_matches_all_byte_pairs() {
        let header = marker_test_header();
        let mut data = header.clone();
        data.resize(header.len() + 32, 1);
        data.extend_from_slice(&[0xff, 0xd9]);
        for a in 0..=255 {
            for b in 0..=255 {
                data[header.len() + 15] = a;
                data[header.len() + 16] = b;
                assert_eq!(
                    jpeg_scanline_candidate(&data),
                    received_entropy_framing(&data[header.len()..]),
                    "pair {a:02x}/{b:02x} across the batch boundary"
                );
            }
        }
    }

    #[test]
    fn jpeg_marker_guard_matches_random_payloads_and_alignments() {
        let header = marker_test_header();
        let mut seed = 20261027_u64;
        for case in 0..4096 {
            let offset = (case / 129) % 32;
            let mut data = vec![0x7e; offset];
            data.extend_from_slice(&header);
            for _ in 0..case % 129 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                data.push((seed >> 32) as u8);
            }
            data.extend_from_slice(&[0xff, 0xd9]);
            if case % 7 == 0 {
                data.pop();
            }
            assert_eq!(
                jpeg_scanline_candidate(&data[offset..]),
                received_entropy_framing(&data[offset + header.len()..]),
                "payload {case}, alignment offset {offset}"
            );
        }
    }

    #[test]
    fn jpeg_grayscale_boundaries_and_corrupt_tails() {
        for (width, height) in [(1, 1), (7, 9), (8, 8), (9, 17), (1009, 73)] {
            let valid = encoded_jpeg(width, height, image::ColorType::L8);
            assert!(jpeg_scanline_candidate(&valid));
            assert!(validate_image_data(&valid).is_ok());
            let mut appended = valid.clone();
            appended.extend_from_slice(&[0xff, 0xdb, 0, 3, 0]);
            for data in [&valid[..valid.len() / 2], &valid[..valid.len() - 5], &appended]
            {
                let legacy = image::load_from_memory(data)
                    .map(|_| ())
                    .map_err(|e| format!("failed to decode image ({e})"));
                assert_eq!(validate_image_data(data), legacy, "{width}x{height}");
            }
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn jpeg_validation_avoids_duplicate_pixel_buffer() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let fixture = std::env::temp_dir()
            .join(format!("tekt-jpeg-allocation-{}-{stamp}.jpg", std::process::id()));
        std::fs::write(&fixture, encoded_jpeg(6000, 4000, image::ColorType::Rgb8))
            .unwrap();
        let test = format!(
            "{}::jpeg_validation_allocation_child",
            module_path!().split_once("::").unwrap().1
        );
        let mut outputs = Vec::new();
        for stream in [false, true] {
            let mut command =
                std::process::Command::new(std::env::current_exe().unwrap());
            command
                .arg("--exact")
                .arg(&test)
                .arg("--nocapture")
                .env("TEKT_JPEG_ALLOCATION_FIXTURE", &fixture)
                .env_remove("TEKT_JPEG_STREAM_EXPECT_ZERO")
                .env("RAYON_NUM_THREADS", "1");
            if stream {
                command.env("TEKT_JPEG_STREAM_EXPECT_ZERO", "1");
            }
            outputs.push(command.output());
        }
        std::fs::remove_file(&fixture).unwrap();
        for output in outputs {
            let output = output.unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn jpeg_validation_allocation_child() {
        let Some(fixture) = std::env::var_os("TEKT_JPEG_ALLOCATION_FIXTURE") else {
            return;
        };
        let bytes = std::fs::read(fixture).unwrap();
        allocation_probe::FULL_RGB.store(0, std::sync::atomic::Ordering::SeqCst);
        allocation_probe::ACTIVE.store(true, std::sync::atomic::Ordering::SeqCst);
        let result = validate_image_data(&bytes);
        allocation_probe::ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
        let allocations =
            allocation_probe::FULL_RGB.load(std::sync::atomic::Ordering::SeqCst);
        println!("Full RGB-size validation allocations: {allocations}");
        result.unwrap();
        assert!(
            allocations <= 1,
            "validation allocated a duplicate full RGB destination"
        );
        if std::env::var_os("TEKT_JPEG_STREAM_EXPECT_ZERO").is_some() {
            assert_eq!(
                allocations, 0,
                "scanline validation allocated a full RGB destination"
            );
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn shared_image_validation_once_per_call() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let fixture = std::env::temp_dir()
            .join(format!("tekt-image-sharing-{}-{stamp}.jpg", std::process::id()));
        std::fs::write(&fixture, encoded_jpeg(1600, 1200, image::ColorType::Rgb8))
            .unwrap();
        let test = format!(
            "{}::shared_image_validation_child",
            module_path!().split_once("::").unwrap().1
        );
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--exact")
            .arg(test)
            .arg("--nocapture")
            .env("TEKT_SHARED_IMAGE_FIXTURE", &fixture)
            .env("RAYON_NUM_THREADS", "1")
            .output();
        std::fs::remove_file(&fixture).unwrap();
        let output = result.unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn shared_image_validation_child() {
        use typst_core::entities::frame_paint::{FramePaint, FrameStroke, TilingFrame};
        use typst_core::entities::geometry::{FillRule, ShapeKind};
        use typst_core::entities::gradient::RelativeTo;
        use typst_core::entities::layout_types::{Angle, Frame, Size};

        let Some(fixture) = std::env::var_os("TEKT_SHARED_IMAGE_FIXTURE") else {
            return;
        };
        let shared = Arc::new(std::fs::read(fixture).unwrap());
        let mut frame = Frame::new(Size { width: Pt(1.0), height: Pt(1.0) });
        frame.items.push(image_item(shared.clone()));
        let paint = FramePaint::Tiling(Arc::new(TilingFrame {
            frame,
            spacing: Size { width: Pt(0.0), height: Pt(0.0) },
            offset: Point::ZERO,
            angle: Angle::deg(0.0),
            relative: RelativeTo::Self_,
        }));
        let shape = FrameItem::Shape {
            pos: Point::ZERO,
            kind: ShapeKind::Rect,
            width: 1.0,
            height: 1.0,
            fill: Some(paint.clone()),
            stroke: Some(FrameStroke { paint, ..Default::default() }),
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        };
        let mut doc = document(vec![group(vec![image_item(shared.clone())]), shape]);
        doc.pages[0].background.push(image_item(shared.clone()));
        doc.pages[0].foreground.push(image_item(shared.clone()));
        let mut page = doc.pages[0].clone();
        page.background.clear();
        page.foreground.clear();
        page.items =
            vec![image_item(shared.clone()), group(vec![image_item(shared.clone())])];
        doc.pages.push(page);
        let count = |doc: &PagedDocument, calls| {
            VALIDATION_CALLS.with(|n| n.set(Some(0)));
            let result = (0..calls).try_for_each(|_| validate_document_images(doc));
            let count = VALIDATION_CALLS.with(|n| n.replace(None).unwrap());
            result.unwrap();
            count
        };
        let once = count(&doc, 1);
        println!("Seven shared occurrences, including page roots and fill/stroke tilings: {once} decodes");
        assert_eq!(once, 1);
        let distinct = Arc::new(shared.as_ref().clone());
        let distinct_doc = document(vec![
            image_item(shared.clone()),
            image_item(shared),
            image_item(distinct),
        ]);
        let identities = count(&distinct_doc, 1);
        println!("Two distinct Arcs with equal bytes: {identities} decodes");
        assert_eq!(identities, 2);
        let repeated = count(&doc, 2);
        println!("Two calls with shared images: {repeated} decodes");
        assert_eq!(repeated, 2);
    }

    #[test]
    fn shared_image_validation_preserves_first_error_and_fresh_calls() {
        let valid = encoded_jpeg(32, 24, image::ColorType::Rgb8);
        let good = Arc::new(valid.clone());
        let bad = Arc::new(valid[..20].to_vec());
        let mut later = valid.clone();
        let sof = later.windows(2).position(|w| w == [0xff, 0xc0]).unwrap();
        later[sof + 5..sof + 9].fill(0xff);
        let later = Arc::new(later);
        let expected = validate_image_data(&bad).unwrap_err();
        assert_ne!(expected, validate_image_data(&later).unwrap_err());
        let doc = document(vec![
            image_item(good.clone()),
            group(vec![image_item(good), image_item(bad), image_item(later)]),
        ]);
        assert_eq!(validate_document_images(&doc), Err(expected));
        let mut doc = document(vec![image_item(Arc::new(valid.clone()))]);
        validate_document_images(&doc).unwrap();
        let FrameItem::Image { data, .. } = &mut doc.pages[0].items[0] else {
            unreachable!()
        };
        let pointer = Arc::as_ptr(data);
        Arc::get_mut(data).unwrap().truncate(20);
        assert_eq!(Arc::as_ptr(data), pointer);
        assert!(validate_document_images(&doc).is_err());
        let FrameItem::Image { data, .. } = &mut doc.pages[0].items[0] else {
            unreachable!()
        };
        *Arc::get_mut(data).unwrap() = valid;
        assert_eq!(Arc::as_ptr(data), pointer);
        validate_document_images(&doc).unwrap();
    }

    #[test]
    fn jpeg_scanline_preserves_progressive_and_trailing_table_errors() {
        let valid = include_bytes!("../../../fixtures/jpeg-validation/progressive.jpg");
        validate_image_data(valid).unwrap();
        let mut corrupt = valid.to_vec();
        corrupt[2136] ^= 2;
        let expected = image::load_from_memory(&corrupt)
            .map(|_| ())
            .map_err(|e| format!("failed to decode image ({e})"));
        assert!(expected.is_err());
        assert_eq!(validate_image_data(&corrupt), expected);
        let mut trailing_table = encoded_jpeg(32, 24, image::ColorType::Rgb8);
        trailing_table.truncate(trailing_table.len() - 2);
        trailing_table.extend_from_slice(&[0xff, 0xdb, 0, 3, 0xc0, 0xff, 0xd9]);
        let expected = image::load_from_memory(&trailing_table)
            .map(|_| ())
            .map_err(|e| format!("failed to decode image ({e})"));
        assert!(expected.is_err());
        assert_eq!(validate_image_data(&trailing_table), expected);
    }

    #[test]
    fn jpeg_scanline_admission_is_conservative() {
        let baseline = encoded_jpeg(32, 24, image::ColorType::Rgb8);
        assert!(jpeg_scanline_candidate(&baseline));
        assert!(jpeg_scanline_candidate(&encoded_jpeg(32, 24, image::ColorType::L8)));
        assert!(!jpeg_scanline_candidate(include_bytes!(
            "../../../fixtures/jpeg-validation/progressive.jpg"
        )));
        let mut trailing = baseline.clone();
        trailing.push(0);
        assert!(!jpeg_scanline_candidate(&trailing));
        validate_image_data(&trailing).unwrap();
        let mut restart = baseline.clone();
        restart.splice(restart.len() - 2..restart.len() - 2, [0xff, 0xd0]);
        assert!(!jpeg_scanline_candidate(&restart));
        let mut bad_dc = baseline;
        let dht = bad_dc.windows(2).position(|w| w == [0xff, 0xc4]).unwrap();
        bad_dc[dht + 21] = 15;
        assert!(!jpeg_scanline_candidate(&bad_dc));
    }

    #[test]
    fn jpeg_scanline_external_compatibility_corpus() {
        let Some(root) = std::env::var_os("TEKT_JPEG_COMPAT_CORPUS") else {
            return;
        };
        let mut files: Vec<_> = std::fs::read_dir(root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().and_then(|v| v.to_str()) == Some("jpg"))
            .collect();
        files.sort();
        for path in &files {
            let data = std::fs::read(path).unwrap();
            if detect_image_format(&data) != ImageFormat::Jpeg {
                continue;
            }
            let previous = image::load_from_memory(&data)
                .map(|_| ())
                .map_err(|e| format!("failed to decode image ({e})"));
            assert_eq!(validate_image_data(&data), previous, "{}", path.display());
        }
        println!(
            "Productive validator matched legacy acceptance/errors on {} corpus files",
            files.len()
        );
    }

    fn image_item(data: Arc<Vec<u8>>) -> FrameItem {
        FrameItem::Image {
            alt: None,
            pos: Point::ZERO,
            data,
            width: Pt(1.0),
            height: Pt(1.0),
            intrinsic_width: 1,
            intrinsic_height: 1,
            clip_rect: None,
            orientation: 1,
        }
    }
    fn document(items: Vec<FrameItem>) -> PagedDocument {
        PagedDocument::new(vec![Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            logical_number: 1,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }])
    }
    fn group(items: Vec<FrameItem>) -> FrameItem {
        FrameItem::Group {
            paint_root: false,
            pos: Point::ZERO,
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 1.0,
            inner_height: 1.0,
            items,
        }
    }

    #[test]
    fn scan_deduplicates_arc_identity_through_nested_groups() {
        let shared = Arc::new(vec![0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, 0, 1, 0, 1, 3]);
        let distinct = Arc::new(shared.as_ref().clone());
        let doc = document(vec![
            image_item(shared.clone()),
            group(vec![group(vec![image_item(shared.clone())]), image_item(distinct)]),
        ]);
        let mut warnings = Vec::new();
        let (refs, pointers, objects) = scan_all_images(&doc, 40, Some(9), &mut warnings);
        assert!(warnings.is_empty());
        assert_eq!((refs.len(), pointers.len(), objects.len()), (2, 2, 2));
        assert_eq!((refs[0].main_obj_id, refs[1].main_obj_id), (40, 41));
        assert!(matches!(objects[0], ImageXObject::Jpeg { icc_profile_id: Some(9), .. }));
        assert_eq!(
            xobject_resources_for_page(&doc.pages[0], &pointers, &refs),
            "/XObject << /Im1 40 0 R /Im2 41 0 R >>"
        );
    }

    #[test]
    fn scan_allocates_mask_before_rgb_and_validates_nested_images() {
        // Keep payload cache isolated in this thread; no cache policy change.
        PAYLOAD_CACHE.with(|cache| cache.borrow_mut().clear());
        let image = image::RgbaImage::from_raw(1, 1, vec![20, 40, 60, 128]).unwrap();
        let mut encoded = std::io::Cursor::new(Vec::new());
        image.write_to(&mut encoded, image::ImageFormat::Png).unwrap();
        let data = Arc::new(encoded.into_inner());
        let doc = document(vec![group(vec![image_item(data.clone()), image_item(data)])]);
        validate_document_images(&doc).unwrap();
        let mut warnings = Vec::new();
        let (refs, pointers, objects) = scan_all_images(&doc, 10, None, &mut warnings);
        assert!(warnings.is_empty());
        assert_eq!((refs.len(), pointers.len(), objects.len()), (1, 1, 1));
        assert!(matches!(
            objects[0],
            ImageXObject::Png { main_obj_id: 11, smask_obj_id: Some(10), .. }
        ));
        let broken = document(vec![group(vec![image_item(Arc::new(
            b"\x89PNG\r\n\x1a\ninvalid".to_vec(),
        ))])]);
        assert!(validate_document_images(&broken)
            .unwrap_err()
            .starts_with("failed to decode image ("));
        PAYLOAD_CACHE.with(|cache| cache.borrow_mut().clear());
    }

    #[test]
    fn public_pdf_export_returns_image_omission_warnings() {
        PAYLOAD_CACHE.with(|cache| cache.borrow_mut().clear());

        let unknown = document(vec![image_item(Arc::new(b"not an image".to_vec()))]);
        let unknown_export =
            crate::export::export_pdf(&unknown, crate::export::StreamMode::Verbose);
        assert!(unknown_export.output.starts_with(b"%PDF-1.7"));
        assert_eq!(unknown_export.warnings.len(), 1);
        assert_eq!(
            unknown_export.warnings[0].message,
            "Formato de imagem desconhecido — imagem omitida",
        );

        let malformed =
            document(vec![image_item(Arc::new(b"\x89PNG\r\n\x1a\ninvalid".to_vec()))]);
        let malformed_export =
            crate::export::export_pdf(&malformed, crate::export::StreamMode::Verbose);
        assert!(malformed_export.output.starts_with(b"%PDF-1.7"));
        assert_eq!(malformed_export.warnings.len(), 1);
        assert!(malformed_export.warnings[0]
            .message
            .starts_with("failed to decode image ("));
        assert!(malformed_export.warnings[0].message.ends_with("— imagem omitida"));

        PAYLOAD_CACHE.with(|cache| cache.borrow_mut().clear());
    }
}
