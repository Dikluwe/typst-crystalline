//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/scan.md
//! @prompt-hash f520227a
//! @layer L3
//! @updated 2026-09-26

use super::jpeg::jpeg_is_rgb;
use super::png::process_png_for_pdf;
use super::{detect_image_format, ImageFormat, ImageRef, ImageXObject, PdfImagePayload};
use crate::export::tiling::walk_resources as walk_frame_items;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

// P876 — cache thread-local de `PdfImagePayload` processados, indexado pelo
// ponteiro do `Arc<Vec<u8>>` original. Como `PagedDocument` mantém os Arcs
// vivos durante o export, o endereço é uma chave estável e segura.
thread_local! {
    static PAYLOAD_CACHE: RefCell<HashMap<usize, PdfImagePayload>> =
        RefCell::new(HashMap::new());
}

/// **P833 (#18, GRAVE)** — valida TODAS as imagens do documento antes do
/// export, falhando a compilação com a mensagem do vanilla
/// (`failed to decode image ({detalhe})`) quando alguma não descodifica.
///
/// Antes desta função, uma imagem com assinatura válida mas corrompida era
/// omitida silenciosamente no export (`eprintln!` interno, exit 0 e PDF sem
/// a imagem). O vanilla descodifica toda a imagem raster em avaliação
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
    struct ImageValidator(Result<(), String>);
    impl FrameVisitor for ImageValidator {
        fn visit_image(&mut self, item: &FrameItem) {
            if self.0.is_err() {
                return;
            }
            if let FrameItem::Image { data, .. } = item {
                if let Err(e) = validate_image_data(data) {
                    self.0 = Err(e);
                }
            }
        }
    }
    let mut validator = ImageValidator(Ok(()));
    for page in &doc.pages {
        for items in [&page.background, &page.items, &page.foreground] {
            walk_frame_items(&mut validator, items);
        }
        if validator.0.is_err() {
            return validator.0;
        }
    }
    Ok(())
}

/// Valida uma imagem por descodificação completa, com a mensagem no formato
/// do vanilla (`failed to decode image ({detalhe})` — `format_image_error`).
fn validate_image_data(data: &[u8]) -> Result<(), String> {
    match detect_image_format(data) {
        ImageFormat::Jpeg => image::load_from_memory(data)
            .map(|_| ())
            .map_err(|e| format!("failed to decode image ({e})")),
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
) -> (Vec<ImageRef>, HashMap<usize, usize>, Vec<ImageXObject>) {
    // P279 — helper recursivo (scope creep análogo P273.10 §A.7 para scan_all_gradients).
    // Bug latent pré-existente: scan_all_images iterava apenas page.items top-level;
    // Images dentro de Groups (via Content::Transform / Block clip / etc.) não eram
    // registadas. Sem fix, Image arm em draw_item_local (P279) teria nada a lookup.
    struct ImageScanner<'a> {
        ptr_to_idx: &'a mut HashMap<usize, usize>,
        refs: &'a mut Vec<ImageRef>,
        xobjects: &'a mut Vec<ImageXObject>,
        next_id: &'a mut usize,
        counter: &'a mut usize,
        icc_profile_id: Option<usize>,
    }
    impl<'a> FrameVisitor for ImageScanner<'a> {
        fn visit_image(&mut self, item: &FrameItem) {
            process_image_item(
                item,
                self.ptr_to_idx,
                self.refs,
                self.xobjects,
                self.next_id,
                self.counter,
                self.icc_profile_id,
            );
        }
    }

    let mut ptr_to_idx: HashMap<usize, usize> = HashMap::new();
    let mut refs: Vec<ImageRef> = Vec::new();
    let mut xobjects: Vec<ImageXObject> = Vec::new();
    let mut next_id = first_id;
    let mut counter = 1usize;
    let mut scanner = ImageScanner {
        ptr_to_idx: &mut ptr_to_idx,
        refs: &mut refs,
        xobjects: &mut xobjects,
        next_id: &mut next_id,
        counter: &mut counter,
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
                    // P833 (#18) — defensivo: em produção este ponto é
                    // inalcançável porque `validate_document_images` (pipeline)
                    // falha a compilação ANTES do export. Se algum caller da
                    // API pública de export saltar a validação, a imagem é
                    // omitida com aviso em stderr (comportamento legado).
                    eprintln!("failed to decode image ({}) — imagem omitida", e);
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
        let (refs, pointers, objects) = scan_all_images(&doc, 40, Some(9));
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
        let (refs, pointers, objects) = scan_all_images(&doc, 10, None);
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
}
