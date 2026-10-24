//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder.md
//! @prompt-hash dcbfbc2c
//! @layer L3
//! @updated 2026-09-29
//!
//! `PdfBuilder` — orquestrador L3 que constrói o ficheiro PDF
//! agregando objects, xref, trailer. Três caminhos: Helvetica
//! (Type1 fallback), CIDFont (Identity-H), Multifont.
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.
//!
//! Depende de `super::` para os submódulos extraídos
//! (fonts, gradients, images, stream).

use super::build_icc_profile_stream;
use super::build_page_stream;
use super::pattern_resources_for_page;
use super::pdf_defaults;
use super::scan_all_gradients;
use super::scan_all_images;
use super::srgb_icc_profile_for_version;
use super::xobject_resources_for_page;
use super::PageContext;
use super::PdfExport;
use super::PdfSerializationMode;
use super::StreamMode;
use ttf_parser::Face;
use typst_core::entities::document_format_options::PageRanges;
use typst_core::entities::document_format_options::PdfAccessibility;
use typst_core::entities::document_format_options::PdfStandards;
use typst_core::entities::layout_types::PagedDocument;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

/// PDF canvas minimum used by the pinned vanilla writer for empty/tiny pages.
const MIN_PDF_CANVAS_PT: f64 = 3.0;

fn pdf_canvas_extent(extent: f64) -> f64 {
    extent.max(MIN_PDF_CANVAS_PT)
}

mod fonts;
mod metadata;
mod navigation;
mod resources;
mod serialization;
mod streams;

use self::navigation::PageIndexConverter;
use self::resources::{colorspace_resource_entry, has_rgb_jpeg};

#[inline]
fn format_dim(val: f64) -> String {
    let s = format!("{val:.4}");
    if s.ends_with("00") {
        format!("{val:.2}")
    } else {
        s
    }
}

// ── Builder ────────────────────────────────────────────────────────────────

pub(super) struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,
    warnings: Vec<SourceDiagnostic>,
    /// P518 — tempo acumulado em `subset_font_with_mapping` (ms).
    subset_ms: f64,
    /// **P536** — object ID do dicionário `/Info`, ou `None` se não houver
    /// metadados para emitir.
    info_id: Option<usize>,
    /// **P611** — object ID do stream `/Metadata` (XMP), ou `None` se ainda
    /// não emitido.
    xmp_id: Option<usize>,
    /// **P617** — `DocumentID` externo de 16 bytes. Quando `Some`, sobrepõe
    /// o valor aleatório/fixo por defeito. `InstanceID` nunca é fixado.
    document_id: Option<[u8; 16]>,
    /// **P1475** — par `(InstanceID, DocumentID)` em base64, calculado por
    /// `emit_xmp_metadata` e reutilizado por `serialize()` para o trailer
    /// `/ID [ (instance) (document) ]` — universal, com ou sem `standard`.
    /// `None` só no caminho teórico em que `emit_xmp_metadata` não foi
    /// chamada antes de `serialize()` (não acontece hoje).
    xmp_ids: Option<(String, String)>,
    /// **P956** — modo de emissão dos content streams de texto (ADR-0126),
    /// propagado aos `PageContext` e aos recursos de página (`/ColorSpace`
    /// + ICC sempre embutido em verbose).
    stream_mode: StreamMode,
    /// **P1360** — compressão física somente dos streams de página.
    serialization: PdfSerializationMode,
    /// **P1140.6** — estrutura lógica PDF, ortogonal a `stream_mode`.
    pdf_tags: super::PdfTags,
    /// P1481: annotation id e StructParent key por página física.
    ua1_link_annots: Vec<Vec<(usize, usize)>>,
    /// **P980** — modo oráculo de paridade de operador (diagnóstico;
    /// flag `--oracle-pdf`). Aplica as transformações de
    /// `export/oracle.rs` aos content streams. `false` no caminho normal.
    oracle: bool,
    /// **P1468** — seleção de páginas a exportar (`None` = todas, o
    /// caminho anterior a P1468, byte-idêntico). Consultado via
    /// `PageIndexConverter` dentro de `build_helvetica`/`build_cidfont`/
    /// `build_multifont`.
    pages: Option<PageRanges>,
    /// **P1469** (escopo reduzido — contrato de linguagem) — conjunto de
    /// standards PDF resolvido; `PdfStandards::default()` (versão `1.7`,
    /// sem validators) preserva o header anterior a P1469, byte-idêntico.
    /// Consultado só em `serialize()` para o header `%PDF-{version}` — não
    /// altera xref, tipos de objeto nem o resto da estrutura do PDF.
    standard: PdfStandards,
}

impl PdfBuilder {
    pub(super) fn new() -> Self {
        Self {
            objects: Vec::new(),
            warnings: Vec::new(),
            subset_ms: 0.0,
            info_id: None,
            xmp_id: None,
            document_id: None,
            xmp_ids: None,
            stream_mode: StreamMode::default(),
            serialization: PdfSerializationMode::default(),
            pdf_tags: super::PdfTags::default(),
            ua1_link_annots: Vec::new(),
            oracle: false,
            pages: None,
            standard: PdfStandards::default(),
        }
    }

    /// **P1468** — fixa a seleção de páginas a exportar. `None` (omisso por
    /// omissão) preserva o comportamento anterior (todas as páginas).
    pub(super) fn with_pages(mut self, pages: Option<PageRanges>) -> Self {
        self.pages = pages;
        self
    }

    /// **P1469** (escopo reduzido — contrato de linguagem) — fixa o
    /// conjunto de standards PDF resolvido. Só afeta o header
    /// `%PDF-{version}` em `serialize()`; o resto do corpo do PDF é
    /// idêntico independentemente do valor.
    pub(super) fn with_standard(mut self, standard: PdfStandards) -> Self {
        self.standard = standard;
        self
    }

    /// **P980** — activa o modo oráculo (transformações de paridade de
    /// operador nos content streams — diagnóstico, `--oracle-pdf`).
    pub(super) fn with_oracle(mut self, oracle: bool) -> Self {
        self.oracle = oracle;
        self
    }

    /// **P617** — fixa o `DocumentID` usado no pacote XMP.
    pub(super) fn with_document_id(mut self, id: Option<[u8; 16]>) -> Self {
        self.document_id = id;
        self
    }

    /// **P956** — fixa o modo de emissão de texto (verbose/compact).
    pub(super) fn with_stream_mode(mut self, mode: StreamMode) -> Self {
        self.stream_mode = mode;
        self
    }

    pub(super) fn with_serialization(mut self, mode: PdfSerializationMode) -> Self {
        self.serialization = mode;
        self
    }

    pub(super) fn with_pdf_tags(mut self, tags: super::PdfTags) -> Self {
        self.pdf_tags = tags;
        self
    }

    pub(super) fn warn(&mut self, message: impl Into<String>) {
        self.warnings
            .push(SourceDiagnostic::warning(Span::detached(), message));
    }

    pub(super) fn build(
        self,
        doc: &PagedDocument,
        font_data: Option<&[u8]>,
    ) -> PdfExport<(Vec<u8>, f64)> {
        // **P811** — documento sem páginas (conteúdo vazio: ficheiro vazio,
        // `$ $`, ` `): o vanilla emite **1 página em branco** A4 (medido).
        // Sem isto, os caminhos de build declaravam `/Kids [3 0 R] /Count 1`
        // mas o loop de emissão iterava 0 páginas — o objecto de página
        // nunca existia e o PDF saía inválido (`Kid object is wrong type
        // (null)`, medido com pdfinfo). Ponto único: todos os exports
        // públicos passam por aqui.
        let blank_page = typst_core::entities::layout_types::Page {
            width: pdf_defaults::A4_DEFAULT_WIDTH,
            height: pdf_defaults::A4_DEFAULT_HEIGHT,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![],
        };
        let blank_doc;
        let doc = if doc.pages.is_empty() {
            blank_doc = {
                let mut preserved = doc.clone();
                preserved.pages.push(blank_page);
                preserved
            };
            &blank_doc
        } else {
            doc
        };
        let bound = super::font_binding::bind_document(doc, &[]);
        if !bound.is_empty() {
            if let [((_, variant, variations), data)] = bound.as_ref() {
                let needs_instance = Face::parse(data, 0).ok().is_some_and(|face| {
                    let axes = crate::font_variant::merge_explicit_variations(
                        crate::font_variant::axis_variations_for_font_variant(variant),
                        variations,
                    );
                    face.variation_axes()
                        .into_iter()
                        .any(|axis| axes.iter().any(|v| v.tag == axis.tag))
                });
                if !needs_instance {
                    if let Ok(face) = Face::parse(data, 0) {
                        return self.build_cidfont(doc, &face, data);
                    }
                }
            }
            let faces: Vec<_> = bound
                .iter()
                .filter_map(|(_, data)| Face::parse(data, 0).ok())
                .collect();
            if faces.len() == bound.len() {
                return self.build_multifont(doc, &bound, &faces);
            }
        }
        if let Some(data) = font_data {
            if let Ok(face) = Face::parse(data, 0) {
                return self.build_cidfont(doc, &face, data);
            }
        }
        self.build_helvetica(doc)
    }

    // ── Caminho Helvetica (fallback, Type1 sem embedding) ─────────────────

    fn build_helvetica(mut self, doc: &PagedDocument) -> PdfExport<(Vec<u8>, f64)> {
        // **P1468** — `n`/`/Count` refletem só as páginas incluídas pela
        // seleção (`self.pages`); sem seleção, `pic` é a identidade e `n`
        // permanece `doc.pages.len()` (byte-idêntico ao caminho anterior).
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());
        // `doc.pages` nunca está vazio aqui (`build()` já substitui pelo
        // documento em branco de 1 página antes de chamar este caminho), logo
        // sem seleção `pic.exported_count() == doc.pages.len() >= 1` sempre —
        // preserva o `n.max(1)` anterior sem reintroduzir o `.max(1)`
        // explícito (que produziria um `/Kids` com objecto nunca emitido se
        // a seleção excluir todas as páginas).
        let n = pic.exported_count();
        let first_page = 3usize;
        let first_stream = first_page + n;
        let font_f1 = first_stream + n;
        let font_f2 = font_f1 + 1;
        let font_f3 = font_f2 + 1;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        // **P956** — em modo verbose o ICC é embutido SEMPRE (o `/c0` dos
        // blocos de texto aponta para ele), alocado DEPOIS dos objectos de
        // fonte, na mesma posição da convenção P777. Compact: inalterado.
        let verbose = self.stream_mode == StreamMode::Verbose;
        let needs_icc = has_rgb_jpeg(doc) || verbose;
        let icc_profile_id = if needs_icc { Some(font_f3 + 1) } else { None };
        let first_img_id = font_f3 + 1 + if needs_icc { 1 } else { 0 };

        let (img_refs, ptr_to_idx, img_xobjects) =
            scan_all_images(doc, first_img_id, icc_profile_id, &mut self.warnings);

        // P263 — Allocar IDs após imagens. Reserva n_gradients*3 + N
        // sub-functions (estimativa pessimista: N stops 16 → 15 subs por gradient).
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) =
            scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        // Sub-function IDs após os 3*N gradient object IDs.
        let mut next_sub_id = first_grad_id + n_grads * 3;
        let tilings = crate::export::pdf_tiling::Patterns::scan(doc, &mut next_sub_id);

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            // **P1468** — página excluída pela seleção: não emitir objecto
            // de página nem stream (comportamento idêntico ao anterior
            // quando `self.pages` é `None`, já que `pic` é a identidade).
            let Some(new_i) = pic.new_index(i) else { continue };
            let page_id = first_page + new_i;
            let stream_id = first_stream + new_i;
            let w = pdf_canvas_extent(page.canvas_width());
            let h = pdf_canvas_extent(page.canvas_height());

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res = tilings.resources(
                &pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs),
                i,
            );
            // **P956** — verbose: `/c0` (sRGB ICCBased) nos recursos da página.
            let cs_res = colorspace_resource_entry(verbose, icc_profile_id, page);
            let resources_str = format!(
                "/Font << /F1 {font_f1} 0 R /F2 {font_f2} 0 R /F3 {font_f3} 0 R >> {xobj_res} {pat_res}{cs_res}"
            );

            let trim_box = if page.bleed.is_zero() {
                String::new()
            } else {
                format!(
                    "/TrimBox [{} {} {} {}] ",
                    format_dim(page.bleed.left),
                    format_dim(page.bleed.bottom),
                    format_dim(page.bleed.left + page.width),
                    format_dim(page.bleed.bottom + page.height),
                )
            };
            self.add(
                page_id,
                format!(
                    "<< /Type /Page /Parent 2 0 R \
/MediaBox [0 0 {} {}] \
                   {trim_box}/Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>",
                    format_dim(w),
                    format_dim(h)
                ),
            );

            let mut ctx = PageContext::type1(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                self.stream_mode,
            )
            .with_pdf_tags(
                self.pdf_tags,
                page,
                matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)),
            )
            .with_oracle(self.oracle);
            ctx.tiling_names = Some(&tilings.names[i]);
            for (id, bytes) in tilings.emit(i, &mut ctx, &resources_str) {
                self.add_bytes(id, bytes);
            }
            let stream_bytes = self.maybe_oracle(build_page_stream(page, &ctx));
            // P884 — content stream comprimido com FlateDecode quando rentável.
            let stream_object = self.serialize_page_stream(&stream_bytes);
            self.add_bytes(stream_id, stream_object);
        }

        self.add(
            font_f1,
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica \
                            /Encoding /WinAnsiEncoding >>"
                .into(),
        );
        self.add(
            font_f2,
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold \
                            /Encoding /WinAnsiEncoding >>"
                .into(),
        );
        self.add(
            font_f3,
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique \
                            /Encoding /WinAnsiEncoding >>"
                .into(),
        );

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        // **P1477** — perfil selecionado pela versão efetiva do PDF.
        if let Some(id) = icc_profile_id {
            self.add_bytes(
                id,
                build_icc_profile_stream(srgb_icc_profile_for_version(
                    self.standard.version,
                )),
            );
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit Function/Shading/Pattern objects para gradients.
        let page_dimensions: Vec<(f64, f64)> =
            doc.pages.iter().map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(
            grad_objs,
            &page_dimensions,
            &mut next_sub_id,
            icc_profile_id,
        );

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_attachments(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_pdfa_output_intent();
        self.emit_xmp_metadata(doc);
        self.emit_tag_structure(doc);
        self.emit_pdfua_viewer_preferences();
        self.emit_page_labels(doc);
        let subset_ms = self.subset_ms;
        let warnings = std::mem::take(&mut self.warnings);
        PdfExport { output: (self.serialize(), subset_ms), warnings }
    }
}
