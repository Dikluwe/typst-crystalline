
const LAB_PAGE_BREAK: &str = "#pagebreak()\n";
const LAB_MAX_PAGES: usize = 32;

fn lab_page_inputs(text: &str) -> Option<(&str, Vec<&str>)> {
    let mut lines = text.split_inclusive('\n');
    let page = lines.next()?;
    let font = lines.next()?;
    if !matches!(page,
        "#set page(width: 400pt, height: 600pt, margin: 20pt)\n"
        | "#set page(width: 360pt, height: 600pt, margin: 20pt)\n")
        || !matches!(font,
            "#set text(font: \"Libertinus Serif\")\n"
            | "#set text(font: \"New Computer Modern\")\n")
    {
        return None;
    }
    let (header, body) = text.split_at(page.len() + font.len());
    let mut pages = Vec::new();
    for body in body.split(LAB_PAGE_BREAK) {
        if pages.len() >= LAB_MAX_PAGES || body.len() > 2240 {
            return None;
        }
        let paragraphs = body.strip_suffix("\n\n")?;
        for (index, paragraph) in paragraphs.split("\n\n").enumerate() {
            if index >= 20 || paragraph.len() > 110
                || !paragraph.as_bytes().first()?.is_ascii_alphabetic()
                || !paragraph.ends_with('.')
                || !paragraph.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b' ' | b'.' | b','))
            {
                return None;
            }
        }
        pages.push(body);
    }
    (pages.len() >= 2).then_some((header, pages))
}

pub(super) struct LabPageMemo {
    epoch: Arc<()>,
    root: typst_core::entities::file_id::FileId,
    features: typst_core::entities::compiler_features::Features,
    header: String,
    language: Option<typst_core::entities::lang::Lang>,
    pages: Vec<LabPageProduct>,
}
#[derive(Clone)]
struct LabPageProduct {
    input: Arc<str>,
    page: Arc<typst_core::entities::layout_types::Page>,
}

fn lab_closed_page_output(doc: &PagedDocument, count: usize) -> bool {
    doc.pages.len() == count
        && doc.document_info.is_empty()
        && doc.attachments.is_empty()
        && doc.format_options == Default::default()
        && doc.extracted_label_pages.is_empty()
        && doc.extracted_label_positions.is_empty()
        && doc.extracted_positions.is_empty()
        && doc.extracted_figure_page_numbers.is_empty()
        && doc.extracted_table_page_numbers.is_empty()
        && doc.extracted_headings.is_empty()
        && doc.layout_warnings.is_empty()
        && doc.layout_errors.is_empty()
        && doc.pages.iter().enumerate().all(|(index, page)| {
            page.logical_number == index + 1 && page.numbering.is_none()
                && page.background.is_empty() && page.foreground.is_empty()
                && !page.items.is_empty()
        })
}

#[inline(never)]
fn lab_page_replay(
    old: &LabPageMemo,
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    header: &str,
    pages: &[&str],
    epoch: &Arc<()>,
) -> Option<(PagedCompilation, LabPageMemo, usize)> {
    if old.root != source.id() || old.features != features || old.header != header
        || !Arc::ptr_eq(&old.epoch, epoch) || old.pages.len() != pages.len()
    {
        return None;
    }
    if !old.pages.iter().zip(pages).any(|(before, after)| before.input.as_ref() == *after) {
        return None;
    }
    let mut products = Vec::with_capacity(pages.len());
    let mut hits = 0;
    for (ordinal, (before, &after)) in old.pages.iter().zip(pages).enumerate() {
        if before.input.as_ref() == after {
            hits += 1;
            products.push(before.clone());
            continue;
        }

        let mut text = header.to_owned();
        if ordinal != 0 {
            // Restore the entry class after a material page with the same
            // fixed style and margins, including the committed baseline.
            text.push_str("Warm boundary paragraph.\n\n");
            text.push_str(LAB_PAGE_BREAK);
        }
        text.push_str(after);
        let segment = Source::new(source.id(), text).ok()?;
        let (fresh, diagnostics) = compile_paged_native(
            world, &segment, full_error, features, &mut Timings::default(), None,
        );
        let mut fresh = fresh.ok()?.document;
        if !diagnostics.is_empty() || fresh.document_language != old.language
            || !lab_closed_page_output(&fresh, if ordinal == 0 { 1 } else { 2 })
        {
            return None;
        }
        let mut page = fresh.pages.pop()?;
        page.logical_number = ordinal + 1;
        products.push(LabPageProduct { input: Arc::from(after), page: Arc::new(page) });
    }
    if !world.font_query_epoch().is_some_and(|now| Arc::ptr_eq(&now, epoch)) {
        return None;
    }
    let mut document = PagedDocument::new(
        products.iter().map(|entry| (*entry.page).clone()).collect(),
    );
    document.document_language = old.language;
    let next = LabPageMemo {
        epoch: epoch.clone(),
        root: source.id(),
        features,
        header: header.to_owned(),
        language: old.language,
        pages: products,
    };
    Some((PagedCompilation {
        document,
        introspector: typst_core::entities::introspector::TagIntrospector::empty(),
    }, next, hits))
}

pub(super) fn compile_paged_in_session(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
    session: Option<&super::pdf::PdfCompileSession>,
) -> (SourceResult<PagedCompilation>, Vec<SourceDiagnostic>) {
    let trace = std::env::var_os("TEKT_LAB_PAGE_TRACE").is_some();
    if let Some(session) = session.filter(|_| std::env::var_os("TEKT_LAB_PAGE_REPLAY").is_some()) {
        if let (Some((header, pages)), Some(epoch)) =
            (lab_page_inputs(source.text()), world.font_query_epoch())
        {
            if world.library().global.is_empty() {
                let start = Instant::now();
                let mut memo = session.lab_pages.lock().unwrap();
                if let Some((compiled, next, hits)) = memo.as_ref().and_then(|old| {
                    lab_page_replay(old, world, source, full_error, features, header, &pages, &epoch)
                }) {
                    timings.layout_ms = duration_ms(start.elapsed());
                    timings.total_ms = timings.layout_ms;
                    if trace { eprintln!("lab-page-replay hits={hits}/{}", pages.len()); }
                    *memo = Some(next);
                    return (Ok(compiled), Vec::new());
                }

                let (compiled, diagnostics) = compile_paged_native(
                    world, source, full_error, features, timings, Some(session),
                );
                *memo = compiled.as_ref().ok().filter(|c| {
                    diagnostics.is_empty() && lab_closed_page_output(&c.document, pages.len())
                        && world.font_query_epoch().is_some_and(|now| Arc::ptr_eq(&now, &epoch))
                }).map(|c| LabPageMemo {
                    epoch,
                    root: source.id(),
                    features,
                    header: header.to_owned(),
                    language: c.document.document_language,
                    pages: pages.iter().zip(&c.document.pages).map(|(&input, page)| LabPageProduct {
                        input: Arc::from(input),
                        page: Arc::new(page.clone()),
                    }).collect(),
                });
                if trace { eprintln!("lab-page-replay native seeded={}", memo.is_some()); }
                return (compiled, diagnostics);
            }
        }
        if trace { eprintln!("lab-page-replay refused: input or dependencies"); }
    }
    compile_paged_native(world, source, full_error, features, timings, session)
}
