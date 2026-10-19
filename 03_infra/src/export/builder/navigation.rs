//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/navigation.md
//! @prompt-hash 962df1f2
//! @layer L3
//! @updated 2026-09-26

use super::serialization::{
    escape_pdf_literal, insert_pdf_dict_entry, pdf_indirect_ref_after, pdf_name,
    utf16be_hex_string,
};
use super::PdfBuilder;
use crate::export::pdf_defaults;
use ecow::EcoString;
use typst_core::entities::document_format_options::PageRanges;
use typst_core::entities::document_format_options::PdfAccessibility;
use typst_core::entities::document_format_options::PdfArchival;
use typst_core::entities::elements::pdf_attach::AttachedFileRelationship;
use typst_core::entities::frame_visitor::walk_frame_items;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::LinkTarget;
use typst_core::entities::layout_types::PagedDocument;
use typst_core::entities::layout_types::Point;
use typst_core::entities::layout_types::Size;

/// **P1476** — famílias PDF/A que suportam anexos embutidos conformes.
/// ISO 19005-3 introduziu embedded files em PDF/A-3 (A3a/A3b/A3u); A4f/A4e
/// são o equivalente em PDF/A-4. A2*/A4 (sem `f`/`e`) e A1* não suportam
/// embedded files conformes no ISO 19005 correspondente (não medido para
/// eles — typst-passo-1476.md §3.1). Reusado por `emit_attachments` para
/// gate de `/AFRelationship`/`/AF`.
fn archival_supports_embedded_files(archival: Option<PdfArchival>) -> bool {
    matches!(
        archival,
        Some(
            PdfArchival::A3a
                | PdfArchival::A3b
                | PdfArchival::A3u
                | PdfArchival::A4f
                | PdfArchival::A4e
        )
    )
}

/// **P1476** — mapeia `AttachedFileRelationship` do domínio para o nome
/// `/AFRelationship` da Filespec PDF. Mapeamento 1:1 confirmado por
/// medição contra o vanilla ratificado (typst-passo-1476.md §2.3): os
/// quatro nomes de variante do enum do domínio são idênticos aos quatro
/// valores PDF nomeados; `None` do domínio (sem `relationship:` explícito
/// em `#pdf.attach`) não tem variante própria e mapeia para o quinto
/// valor PDF `/Unspecified`.
fn af_relationship_name(relationship: Option<&AttachedFileRelationship>) -> &'static str {
    match relationship {
        Some(r) => af_relationship_variant_name(r),
        None => "Unspecified",
    }
}

/// Parte exaustiva sobre as quatro variantes do domínio, isolada da
/// resolução de `None` acima — evita que o `match` de `af_relationship_name`
/// combine a ausência (`Option::None`, não é variante do domínio) com a
/// cobertura nominal de `AttachedFileRelationship` (V16 exige exaustividade
/// só sobre as variantes reais do enum de domínio).
fn af_relationship_variant_name(relationship: &AttachedFileRelationship) -> &'static str {
    match relationship {
        AttachedFileRelationship::Source => "Source",
        AttachedFileRelationship::Data => "Data",
        AttachedFileRelationship::Alternative => "Alternative",
        AttachedFileRelationship::Supplement => "Supplement",
    }
}

/// Devolve o operando literal PDF completo, incluindo os delimitadores.
/// Mantém a URI o mais intacta possível; escapa apenas `(`, `)`, `\` e
/// caracteres de controlo (0x00–0x1f, 0x7f) por compatibilidade com leitores.
fn escape_pdf_uri(uri: &str) -> String {
    escape_pdf_literal(uri)
}

/// A mesma string PDF identifica a chave da árvore e o operando GoTo.
fn pdf_destination_string(name: &str) -> String {
    utf16be_hex_string(name)
}

fn page_label_dict(page: &typst_core::entities::layout_types::Page) -> Option<String> {
    let pattern = page.numbering.as_ref()?.as_pattern()?;
    let (token_index, token) = pattern
        .char_indices()
        .find(|(_, ch)| typst_core::entities::counter_format::is_numbering_token(*ch))?;
    let prefix = &pattern[..token_index];
    let suffix = &pattern[token_index + token.len_utf8()..];
    let number = page.logical_number;

    // PDF tem estilos nativos somente para estes sistemas sem sufixo.
    // Os limites de romano/alfabético seguem o formatter de linguagem:
    // fora deles, o estilo nativo comunicaria outro rótulo.
    let style = if suffix.is_empty() && number > 0 {
        match token {
            '1' => Some("D"),
            'i' if number <= 3999 => Some("r"),
            'I' if number <= 3999 => Some("R"),
            'a' if number <= 26 => Some("a"),
            'A' if number <= 26 => Some("A"),
            _ => None,
        }
    } else {
        None
    };
    if let Some(style) = style {
        let prefix_entry = if prefix.is_empty() {
            String::new()
        } else {
            format!(" /P {}", utf16be_hex_string(prefix))
        };
        return Some(format!(
            "<< /Type /PageLabel /S /{style}{prefix_entry} /St {number} >>"
        ));
    }

    let text = typst_core::entities::counter_format::format_counter(&[number], pattern)?;
    Some(format!("<< /Type /PageLabel /P {} >>", utf16be_hex_string(&text)))
}

/// Recolhe `FrameItem::Link` de uma lista de items, incluindo links aninhados
/// dentro de `Group`.
fn collect_links(items: &[FrameItem], out: &mut Vec<(LinkTarget, Point, Size)>) {
    struct LinkCollector<'a>(&'a mut Vec<(LinkTarget, Point, Size)>);
    impl<'a> FrameVisitor for LinkCollector<'a> {
        fn visit_link(&mut self, item: &FrameItem) {
            if let FrameItem::Link { target, pos, size, .. } = item {
                self.0.push((target.clone(), *pos, *size));
            }
        }
    }
    let mut collector = LinkCollector(out);
    walk_frame_items(&mut collector, items);
}

/// **P1468** — mapeia índice físico de página (0-indexed) para o novo
/// índice sequencial das páginas **incluídas** por uma seleção `pages:
/// Option<PageRanges>` (`#set format.pdf(pages: ...)` ou `--pages`).
/// Páginas excluídas não entram no mapa (`new_index` devolve `None`). Sem
/// seleção (`ranges == None`), o mapeamento é a identidade e
/// `exported_count() == total_pages` — comportamento byte-idêntico ao
/// anterior a P1468. A "ordenação"/"deduplicação" observável da seleção
/// emerge exclusivamente desta iteração em ordem física crescente, nunca de
/// normalizar a lista de `PageRange` guardada em `PageRanges` (ver
/// `entities/document_format_options.md` P1468).
pub(super) struct PageIndexConverter {
    /// Índice físico antigo (0-indexed) -> novo índice (0-indexed), `None`
    /// se a página foi excluída da seleção.
    mapping: Vec<Option<usize>>,
    exported_count: usize,
}

impl PageIndexConverter {
    pub(super) fn new(total_pages: usize, ranges: Option<&PageRanges>) -> Self {
        let Some(ranges) = ranges else {
            return Self {
                mapping: (0..total_pages).map(Some).collect(),
                exported_count: total_pages,
            };
        };

        let mut mapping = Vec::with_capacity(total_pages);
        let mut next = 0usize;
        for old_index in 0..total_pages {
            // Página física 1-indexed (`old_index` é 0-indexed).
            let page = std::num::NonZeroUsize::new(old_index + 1)
                .expect("old_index + 1 nunca é zero");
            if ranges.includes_page(page) {
                mapping.push(Some(next));
                next += 1;
            } else {
                mapping.push(None);
            }
        }
        Self { mapping, exported_count: next }
    }

    /// Número de páginas efetivamente exportadas (`n`/`/Count` filtrado).
    pub(super) fn exported_count(&self) -> usize {
        self.exported_count
    }

    /// `None` se a página (0-indexed) foi excluída da seleção.
    pub(super) fn new_index(&self, old_index: usize) -> Option<usize> {
        self.mapping.get(old_index).copied().flatten()
    }
}

impl PdfBuilder {
    /// **P424/P463** — Emite annotations `/Subtype /Link` para cada
    /// `FrameItem::Link` do documento: `/A /URI` para URLs externos e
    /// `/A /GoTo` para destinos internos. As annotations são adicionadas como
    /// objetos após todos os recursos e referenciadas pelo `/Annots` de cada
    /// página. Coordenadas convertidas de Y-down (layout) para Y-up (PDF).
    pub(super) fn emit_link_annotations(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        // **P1468** — `pic` decide tanto a origem (páginas excluídas não têm
        // annotations, a página inteira já não é emitida) quanto o destino
        // (`LinkTarget::Destination` para uma página excluída é omitido por
        // completo — comportamento 2 medido no vanilla,
        // `typst-pdf/src/link.rs:191-209`).
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());

        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let ua1 = matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1));
        let mut link_key = doc.pages.len();
        self.ua1_link_annots = vec![Vec::new(); doc.pages.len()];

        // Coletar links por página (coordenadas globais de página).
        let mut per_page: Vec<Vec<(LinkTarget, Point, Size)>> =
            Vec::with_capacity(doc.pages.len());
        for page in &doc.pages {
            let mut links = Vec::new();
            collect_links(&page.items, &mut links);
            per_page.push(links);
        }

        let mut page_annotation_ids: Vec<Vec<usize>> =
            vec![Vec::new(); pic.exported_count()];
        for (page_idx, links) in per_page.iter().enumerate() {
            // Página excluída da seleção: a página inteira não é exportada.
            let Some(new_idx) = pic.new_index(page_idx) else { continue };
            let page_h = doc.pages[page_idx].height;
            for (target, pos, size) in links {
                // **P1468** — destino de página excluída: omitir a
                // annotation inteira (não emitir o objecto nem referenciá-lo
                // em `/Annots`).
                if let LinkTarget::Destination(label) = target {
                    if let Some(&target_page) = doc.extracted_label_pages.get(label) {
                        let excluded =
                            target_page == 0 || pic.new_index(target_page - 1).is_none();
                        if excluded {
                            continue;
                        }
                    }
                }

                let annot_id = next_id;
                next_id += 1;
                let struct_parent = if ua1 {
                    let key = link_key;
                    link_key += 1;
                    self.ua1_link_annots[page_idx].push((annot_id, key));
                    Some(key)
                } else {
                    None
                };

                let x0 = pos.x.val();
                let y0 = page_h - pos.y.val() - size.height.val();
                let x1 = x0 + size.width.val();
                let y1 = y0 + size.height.val();

                let action = match target {
                    LinkTarget::Url(url) => {
                        let escaped_url = escape_pdf_uri(url.as_str());
                        format!("<< /Type /Action /S /URI /URI {escaped_url} >>")
                    }
                    LinkTarget::Destination(label) => {
                        let name = pdf_destination_string(&label.0);
                        format!("<< /Type /Action /S /GoTo /D {name} >>")
                    }
                };

                let ua1_entries = struct_parent
                    .map(|key| {
                        let description = match target {
                            LinkTarget::Url(url) => url.as_str(),
                            LinkTarget::Destination(label) => label.0.as_str(),
                        };
                        format!(
                            " /Contents {} /StructParent {key} /F 4",
                            escape_pdf_literal(description)
                        )
                    })
                    .unwrap_or_default();
                self.add(
                    annot_id,
                    format!(
                        "<< /Type /Annot /Subtype /Link \
                       /Rect [{x0:.2} {y0:.2} {x1:.2} {y1:.2}] \
                       /Border [0 0 0] \
                       /A {action}{ua1_entries} >>"
                    ),
                );
                page_annotation_ids[new_idx].push(annot_id);
            }
        }

        // Reescrever os dicionários das páginas (já reindexadas) que
        // possuem annotations.
        for (new_idx, ids) in page_annotation_ids.iter().enumerate() {
            if ids.is_empty() {
                continue;
            }
            let page_id = FIRST_PAGE_ID + new_idx;
            let refs =
                ids.iter().map(|id| format!("{id} 0 R")).collect::<Vec<_>>().join(" ");
            if let Some((_, content)) =
                self.objects.iter_mut().find(|(id, _)| *id == page_id)
            {
                let s = String::from_utf8_lossy(content);
                if let Some(idx) = s.rfind(">>") {
                    let mut new = s[..idx].to_string();
                    new.push_str(&format!(" /Annots [{refs}]"));
                    if ua1 {
                        new.push_str(" /Tabs /S");
                    }
                    new.push_str(&s[idx..]);
                    *content = new.into_bytes();
                }
            }
        }
    }

    /// **P460** — Emite o dicionário `/Names /Dests` no catalog (objecto 1)
    /// para cada label registado no `PagedDocument`.
    ///
    /// Cada destino usa `/XYZ x y null` com coordenadas PDF (y-up). O catalog
    /// original `<< /Type /Catalog /Pages 2 0 R >>` é editado in-place.
    pub(super) fn emit_named_destinations(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        if doc.extracted_label_pages.is_empty() {
            return;
        }

        // **P1468** — label cuja página foi validamente excluída pela
        // seleção: a entrada não é criada (comportamento 3 medido no
        // vanilla, `typst-pdf/src/convert.rs:858-872`). Distinto do
        // fallback abaixo (número de página que nunca existiu).
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());

        // Juntar labels comuns a label_pages e label_positions.
        let mut entries: Vec<(EcoString, usize, Point)> = Vec::new();
        for (label, &page) in &doc.extracted_label_pages {
            let pos = doc
                .extracted_label_positions
                .get(label)
                .copied()
                .unwrap_or(Point::ZERO);
            entries.push((label.0.clone().into(), page, pos));
        }
        // UTF-16BE keys share a BOM. Code-unit order equals unsigned byte
        // order and keeps prefix keys before their extensions.
        entries.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
        if entries.is_empty() {
            return;
        }

        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let names_id = next_id;
        next_id += 1;
        let dests_id = next_id;

        // Root leaf name tree: alternating string keys and destinations.
        let mut dests_dict = String::from("<< /Names [");
        for (name, page, pos) in entries {
            let page_idx = page.saturating_sub(1);
            let page_ref = if page_idx >= doc.pages.len() {
                // Fallback existente: número de página que nunca existiu no
                // array `doc.pages` (caso distinto de exclusão pela
                // seleção). Aponta para a última página efetivamente
                // exportada.
                format!("{} 0 R", FIRST_PAGE_ID + pic.exported_count().saturating_sub(1))
            } else if let Some(new_idx) = pic.new_index(page_idx) {
                format!("{} 0 R", FIRST_PAGE_ID + new_idx)
            } else {
                // **P1468** — página validamente excluída pela seleção: não
                // adicionar a entrada ao dicionário `/Dests`.
                continue;
            };
            let page_h = doc
                .pages
                .get(page_idx)
                .map(|p| p.height)
                .unwrap_or(pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED);
            let pdf_y = page_h - pos.y.val();
            let escaped_name = pdf_destination_string(&name);
            dests_dict.push_str(&format!(
                "{} [{} /XYZ {:.2} {:.2} null] ",
                escaped_name,
                page_ref,
                pos.x.val(),
                pdf_y
            ));
        }
        dests_dict.push_str("] >>");

        self.add(dests_id, dests_dict);
        self.add(names_id, format!("<< /Dests {dests_id} 0 R >>"));

        // Editar objecto 1 (catalog) para incluir /Names.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Names {names_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
    }

    /// P1286 — materializa os carriers `pdf.attach` como EmbeddedFiles.
    ///
    /// **P1476** — quando `archival_supports_embedded_files` está ativo
    /// (`A3a/A3b/A3u/A4f/A4e`), cada Filespec ganha `/AFRelationship`
    /// (mapeado de `attachment.relationship` via `af_relationship_name`) e
    /// o catálogo ganha `/AF` com uma referência por Filespec processada.
    /// Fora desse gate, comportamento preservado byte-a-byte (P1286).
    pub(super) fn emit_attachments(&mut self, doc: &PagedDocument) {
        // **P1478** — A4f exige `/EmbeddedFiles` mesmo sem nenhum anexo
        // (ISO 19005-4:2020 6.9 teste 5; fonte pinada krilla
        // `configure/validate.rs:269-273,859-871`: só `Archival::A4F` tem
        // `requires_embedded_files_when_empty() == true` — A1/A2/A3/A4/A4E
        // são `false`). Gate distinto de `archival_supports_embedded_files`
        // (esse inclui A3*/A4e, criaria árvores vazias onde o vanilla não
        // as emite). Com `attachments` vazio e o gate ativo, o resto da
        // função já produz `<< /Names [] >>` e mantém `/AF` ausente
        // (`filespec_ids` continua vazio) sem nenhuma outra mudança.
        let requires_empty_embedded_files =
            matches!(self.standard.archival, Some(PdfArchival::A4f));
        if doc.attachments.is_empty() && !requires_empty_embedded_files {
            return;
        }

        let mut attachments = doc.attachments.clone();
        attachments.sort_by(|a, b| a.path.cmp(&b.path));
        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let mut names = String::from("<< /Names [");
        let af_gate = archival_supports_embedded_files(self.standard.archival);
        let mut filespec_ids: Vec<usize> = Vec::new();

        for attachment in attachments {
            let embedded_id = next_id;
            let filespec_id = next_id + 1;
            next_id += 2;

            let subtype = attachment
                .mime_type
                .as_deref()
                .map(|mime| format!(" /Subtype {}", pdf_name(mime)))
                .unwrap_or_default();
            let mut stream = format!(
                "<< /Type /EmbeddedFile{subtype} /Length {} >>\nstream\n",
                attachment.data.len()
            )
            .into_bytes();
            stream.extend_from_slice(attachment.data.as_slice());
            stream.extend_from_slice(b"\nendstream");
            self.add_bytes(embedded_id, stream);

            let filename = attachment.path.as_str();
            let description = attachment
                .description
                .as_deref()
                .map(|value| format!(" /Desc {}", escape_pdf_literal(value)))
                .unwrap_or_default();
            // **P1476** — gated a `archival_supports_embedded_files`;
            // mapeamento via `af_relationship_name` (medição §2.3).
            let af_relationship = if af_gate {
                format!(
                    " /AFRelationship /{}",
                    af_relationship_name(attachment.relationship.as_ref())
                )
            } else {
                String::new()
            };
            self.add(
                filespec_id,
                format!(
                    "<< /Type /Filespec /F {} /UF {} /EF << /F {embedded_id} 0 R >>{description}{af_relationship} >>",
                    escape_pdf_literal(filename),
                    utf16be_hex_string(filename),
                ),
            );
            names.push_str(&format!(
                "{} {filespec_id} 0 R ",
                escape_pdf_literal(filename)
            ));
            filespec_ids.push(filespec_id);
        }

        names.push_str("] >>");
        let embedded_names_id = next_id;
        self.add(embedded_names_id, names);

        let existing_names_id = self
            .objects
            .iter()
            .find(|(id, _)| *id == 1)
            .and_then(|(_, content)| pdf_indirect_ref_after(content, "/Names"));
        if let Some(names_id) = existing_names_id {
            if let Some((_, content)) =
                self.objects.iter_mut().find(|(id, _)| *id == names_id)
            {
                insert_pdf_dict_entry(
                    content,
                    &format!(" /EmbeddedFiles {embedded_names_id} 0 R"),
                );
            }
        } else {
            let names_id = embedded_names_id + 1;
            self.add(names_id, format!("<< /EmbeddedFiles {embedded_names_id} 0 R >>"));
            if let Some((_, catalog)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
                insert_pdf_dict_entry(catalog, &format!(" /Names {names_id} 0 R"));
            }
        }

        // **P1476** — `/AF` no catálogo: array com uma referência por
        // Filespec processada, mesmo gate de `/AFRelationship`. Replicado
        // por completude estrutural/paridade com o vanilla (não
        // comprovadamente exigido pela suíte veraPDF medida — §2.7), custo
        // marginal zero (mesmo `filespec_id` já calculado acima).
        if af_gate && !filespec_ids.is_empty() {
            let af_refs = filespec_ids
                .iter()
                .map(|id| format!("{id} 0 R"))
                .collect::<Vec<_>>()
                .join(" ");
            if let Some((_, catalog)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
                insert_pdf_dict_entry(catalog, &format!(" /AF [{af_refs}]"));
            }
        }
    }

    /// **P535** — Emite a árvore `/Outlines` no catálogo (objecto 1) a partir
    /// dos headings registados em `PagedDocument::extracted_headings`.
    ///
    /// Cada bookmark aponta para a página e posição `(x, y-up)` do heading via
    /// `extracted_label_pages` / `extracted_label_positions`. O aninhamento usa
    /// `/Parent`, `/Prev`, `/Next`, `/First` e `/Last`.
    pub(super) fn emit_outlines(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        if doc.extracted_headings.is_empty() {
            return;
        }

        // **P1468** — divergência deliberada do vanilla (não o bug de sinal
        // invertido de `typst-pdf/src/outline.rs:25-31`, ver
        // `infra/export/builder.md` P1468 §4 retificado): um heading cuja
        // página está incluída permanece na árvore; um heading cuja página
        // foi validamente excluída é omitido, e os seus descendentes são
        // reparentados ao ancestral incluído mais próximo (ou promovidos a
        // top-level).
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());

        struct Node {
            id: usize,
            title: String,
            page_ref: String,
            x: f64,
            y: f64,
            parent: Option<usize>,
            prev: Option<usize>,
            next: Option<usize>,
            first_child: Option<usize>,
            last_child: Option<usize>,
            child_count: usize,
        }

        let mut nodes: Vec<Node> = Vec::new();
        let mut stack: Vec<usize> = Vec::new();

        for (label, _number, body, level) in &doc.extracted_headings {
            let page = doc.extracted_label_pages.get(label).copied().unwrap_or(1);
            let page_idx = page.saturating_sub(1);

            let page_ref = if page_idx >= doc.pages.len() {
                // Fallback existente: número de página que nunca existiu no
                // array `doc.pages` (caso distinto de exclusão pela
                // seleção) — mantém o comportamento anterior a P1468.
                Some(format!(
                    "{} 0 R",
                    FIRST_PAGE_ID + pic.exported_count().saturating_sub(1)
                ))
            } else {
                pic.new_index(page_idx)
                    .map(|new_idx| format!("{} 0 R", FIRST_PAGE_ID + new_idx))
            };
            // **P1468** — página validamente excluída: não criar nó para
            // este heading nem empilhar; a pilha (`stack`) fica inalterada,
            // reparentando automaticamente qualquer descendente seguinte.
            let Some(page_ref) = page_ref else { continue };

            let pos = doc
                .extracted_label_positions
                .get(label)
                .copied()
                .unwrap_or(Point::ZERO);
            let page_h = doc
                .pages
                .get(page_idx)
                .map(|p| p.height)
                .unwrap_or(pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED);
            let pdf_y = page_h - pos.y.val();
            let title = body.plain_text();

            // Fechar níveis até ao pai correcto.
            while stack.len() >= *level {
                stack.pop();
            }
            let parent = stack.last().copied();
            let node_idx = nodes.len();

            // Irmão anterior: último filho do pai, ou último top-level.
            let prev = if let Some(p) = parent {
                nodes[p].last_child
            } else {
                nodes.iter().rposition(|n| n.parent.is_none())
            };

            nodes.push(Node {
                id: 0, // preenchido depois
                title,
                page_ref,
                x: pos.x.val(),
                y: pdf_y,
                parent,
                prev,
                next: None,
                first_child: None,
                last_child: None,
                child_count: 0,
            });

            if let Some(p) = parent {
                if nodes[p].first_child.is_none() {
                    nodes[p].first_child = Some(node_idx);
                }
                nodes[p].last_child = Some(node_idx);
                nodes[p].child_count += 1;
            }
            if let Some(prev_idx) = prev {
                nodes[prev_idx].next = Some(node_idx);
            }

            stack.push(node_idx);
        }

        // Alocar object IDs após todos os objectos existentes.
        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let root_id = next_id;
        next_id += 1;
        let first_node_id = next_id;
        for (i, node) in nodes.iter_mut().enumerate() {
            node.id = first_node_id + i;
        }

        // Emitir cada item de outline.
        for node in &nodes {
            let title_hex = utf16be_hex_string(&node.title);
            let parent_ref = node
                .parent
                .map(|p| format!("{} 0 R", nodes[p].id))
                .unwrap_or_else(|| format!("{root_id} 0 R"));
            let mut dict = format!(
                "<< /Title {title_hex} /Parent {parent_ref} /Dest [{} /XYZ {:.2} {:.2} null]",
                node.page_ref, node.x, node.y
            );
            if let Some(prev_idx) = node.prev {
                dict.push_str(&format!(" /Prev {} 0 R", nodes[prev_idx].id));
            }
            if let Some(next_idx) = node.next {
                dict.push_str(&format!(" /Next {} 0 R", nodes[next_idx].id));
            }
            if let Some(first_idx) = node.first_child {
                dict.push_str(&format!(" /First {} 0 R", nodes[first_idx].id));
            }
            if let Some(last_idx) = node.last_child {
                dict.push_str(&format!(" /Last {} 0 R", nodes[last_idx].id));
            }
            // P602 — /Count com sinal negativo para entradas com filhos
            // (fechadas por defeito), seguindo o vanilla 0.15.0.
            if node.child_count > 0 {
                dict.push_str(&format!(" /Count -{}", node.child_count));
            }
            dict.push_str(" >>");
            self.add(node.id, dict);
        }

        let first_top =
            nodes.iter().find(|n| n.parent.is_none()).map(|n| n.id).unwrap_or(0);
        let last_top =
            nodes.iter().rfind(|n| n.parent.is_none()).map(|n| n.id).unwrap_or(0);
        // P602 — /Count na raiz é o número de bookmarks de topo (sem pai),
        // positivo (abertos por defeito).
        let top_count = nodes.iter().filter(|n| n.parent.is_none()).count();
        self.add(root_id, format!(
            "<< /Type /Outlines /First {first_top} 0 R /Last {last_top} 0 R /Count {top_count} >>"
        ));

        // Editar objecto 1 (catalog) para incluir /Outlines.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Outlines {root_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
    }

    /// **P1486** — number tree de rótulos da sequência exportada. O índice
    /// da tree é o novo índice PDF, mas o valor vem da página Typst original.
    pub(super) fn emit_page_labels(&mut self, doc: &PagedDocument) {
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());
        let skipped = pic.exported_count() < doc.pages.len();
        let labels: Vec<(usize, String)> = doc
            .pages
            .iter()
            .enumerate()
            .filter_map(|(old_index, page)| {
                let new_index = pic.new_index(old_index)?;
                let dict = page_label_dict(page).or_else(|| {
                    skipped.then(|| {
                        format!("<< /Type /PageLabel /S /D /St {} >>", old_index + 1)
                    })
                })?;
                Some((new_index, dict))
            })
            .collect();
        if labels.is_empty() {
            return;
        }

        // Todos os outros objetos (incluindo tagging, PDF/A e PDF/UA) já foram
        // emitidos. Alocar após o máximo real evita colisões com ranges vazios
        // e com reservas de IDs dos outros domínios.
        let tree_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let mut nums = Vec::with_capacity(labels.len());
        for (entry, (new_index, dict)) in labels.into_iter().enumerate() {
            let label_id = tree_id + entry + 1;
            nums.push(format!("{new_index} {label_id} 0 R"));
            self.add(label_id, dict);
        }
        self.add(tree_id, format!("<< /Nums [{}] >>", nums.join(" ")));
        if let Some((_, catalog)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            insert_pdf_dict_entry(catalog, &format!(" /PageLabels {tree_id} 0 R"));
        }
    }
}

#[cfg(test)]
mod uri_tests {
    use super::*;
    use typst_core::{
        compiler::layout::layout,
        entities::{content::Content, elements::link::LinkElem},
    };

    fn emitted_action(url: &str) -> String {
        let doc = layout(&Content::Link(
            LinkElem { url: url.into(), body: Content::text("LINK") }.into(),
        ));
        let mut builder = PdfBuilder::new();
        builder.emit_link_annotations(&doc);
        builder
            .objects
            .iter()
            .map(|(_, bytes)| String::from_utf8_lossy(bytes))
            .find(|body| body.contains("/Subtype /Link"))
            .expect("link annotation")
            .into_owned()
    }

    #[test]
    fn uri_annotation_has_one_literal_operand() {
        let action = emitted_action("https://example.org");
        assert!(action.contains("/URI (https://example.org) >>"), "{action}");
    }

    #[test]
    fn uri_annotation_escapes_destination_without_extra_delimiters() {
        let action = emitted_action("https://example.org/a(b)\\c\n");
        assert!(
            action.contains(r"/URI (https://example.org/a\(b\)\\c\012) >>"),
            "{action}"
        );
    }
}

#[cfg(test)]
mod destination_tests {
    use super::*;
    use std::num::NonZeroUsize;
    use typst_core::{
        compiler::layout::layout,
        entities::{
            content::Content, document_format_options::PageRange, label::Label,
            layout_types::Pt,
        },
    };

    fn document(labels: &[(&str, usize)]) -> PagedDocument {
        let mut doc = layout(&Content::text("BODY"));
        doc.pages = vec![doc.pages[0].clone(); 3];
        for &(name, page) in labels {
            doc.extracted_label_pages.insert(Label(name.into()), page);
            doc.extracted_label_positions
                .insert(Label(name.into()), Point { x: Pt(12.0), y: Pt(20.0) });
        }
        doc
    }
    fn builder() -> PdfBuilder {
        let mut b = PdfBuilder::new();
        b.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());
        b.add(2, "<< /Type /Pages >>".into());
        for id in 3..=5 {
            b.add(id, "<< /Type /Page >>".into());
        }
        b
    }
    fn bodies(b: &PdfBuilder) -> Vec<String> {
        b.objects
            .iter()
            .map(|(_, bytes)| String::from_utf8(bytes.clone()).unwrap())
            .collect()
    }

    #[test]
    fn named_destination_tree_uses_strings_sorted_by_encoded_bytes() {
        let doc = document(&[("\u{e000}", 1), ("aa", 1), ("😀", 1), ("a", 1)]);
        let mut b = builder();
        b.emit_named_destinations(&doc);
        let strings = bodies(&b);
        let tree = strings
            .iter()
            .find(|body| body.starts_with("<< /Names ["))
            .expect("root leaf name tree");
        let keys = ["<FEFF0061>", "<FEFF00610061>", "<FEFFD83DDE00>", "<FEFFE000>"];
        let mut previous = None;
        for key in keys {
            let at = tree
                .find(&format!("{key} [3 0 R /XYZ 12.00 "))
                .expect("encoded key and page destination");
            if let Some(before) = previous {
                assert!(before < at, "binary key order: {tree}");
            }
            previous = Some(at);
            assert_eq!(tree.matches(key).count(), 1);
        }
        assert!(tree.ends_with("] >>"));
    }

    #[test]
    fn named_destination_action_matches_unicode_and_delimiter_safe_key() {
        let mut doc = document(&[("é/😀", 1)]);
        doc.pages[0].items.push(FrameItem::Link {
            target: LinkTarget::Destination(Label("é/😀".into())),
            items: vec![],
            pos: Point::ZERO,
            size: Size { width: Pt(10.0), height: Pt(10.0) },
        });
        let mut b = builder();
        b.emit_named_destinations(&doc);
        b.emit_link_annotations(&doc);
        let text = bodies(&b).join("\n");
        assert!(text.contains("/Names [<FEFF00E9002FD83DDE00> [3 0 R /XYZ"), "{text}");
        assert!(text.contains("/S /GoTo /D <FEFF00E9002FD83DDE00> >>"), "{text}");
    }

    #[test]
    fn named_destination_selection_omits_removed_page_and_remaps_retained_page() {
        let doc = document(&[("gone", 1), ("kept", 2)]);
        let ranges = PageRanges::new(vec![PageRange::at(NonZeroUsize::new(2).unwrap())]);
        let mut b = builder().with_pages(Some(ranges));
        b.emit_named_destinations(&doc);
        let text = bodies(&b).join("\n");
        assert!(
            text.contains("/Names [<FEFF006B006500700074> [3 0 R /XYZ 12.00 "),
            "{text}"
        );
        assert!(!text.contains("0067006F006E0065"), "excluded target remains: {text}");
    }
}
