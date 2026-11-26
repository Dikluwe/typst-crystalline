//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world/fonts.md
//! @prompt-hash 87740ce0
//! @layer L3
//! @updated 2026-10-07

use super::SystemWorld;
use crate::fonts::FontSlot;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use typst_core::contracts::world::World;
use typst_core::entities::font_book::{Coverage, FontBook, FontInfo, FontVariant};
use typst_core::entities::font_list::FontNamePattern;
use typst_core::entities::source::Source;
use typst_core::entities::syntax_kind::SyntaxKind;
use typst_core::entities::world_types::Font;

/// Paired immutable leading assets. Construction is empty and acquisition is lazy.
#[derive(Default)]
pub(super) struct EmbeddedPrefix {
    epoch: Arc<()>,
    catalog: OnceLock<(Vec<FontSlot>, FontBook)>,
}
impl EmbeddedPrefix {
    fn get(&self) -> &(Vec<FontSlot>, FontBook) {
        self.catalog.get_or_init(|| {
            let sets = crate::embedded_fonts::load_embedded_fonts();
            (sets.text_slots, sets.text_book)
        })
    }
}

impl SystemWorld {
    /// No catalog acquisition: full demand irreversibly closes this World scope.
    pub(super) fn world_font_query_epoch(&self) -> Option<Arc<()>> {
        if self.font_catalog.get().is_some() {
            return None;
        }
        Some(self.font_prefix.as_ref()?.epoch.clone())
    }

    /// Builder: associa slots de fontes ao world e popula o `FontBook`.
    ///
    /// **P839** — slots cuja extracção de `FontInfo` falha são descartados
    /// (`pair_slots_with_book`): book e slots ficam sempre alinhados por
    /// índice, como no vanilla.
    pub fn with_fonts(mut self, font_slots: Vec<FontSlot>) -> Self {
        let (slots, book) = crate::fonts::pair_slots_with_book(font_slots);
        self.install_font_catalog((slots, book));
        self
    }

    /// Builder: schedules system discovery for the first catalog request.
    ///
    /// Usa `fontdb` para carregar as fontes do sistema operativo.
    /// Comportamento defensivo: se nenhuma fonte for encontrada, o
    /// `FontBook` fica vazio mas o `SystemWorld` continua funcional.
    pub fn with_system_fonts(mut self) -> Self {
        self.font_prefix = None;
        self.font_catalog = OnceLock::new();
        self.font_loader = Box::new(|| Arc::new(crate::fontdb::load_system_fonts()));
        self.coverage_cache.get_mut().unwrap().clear();
        self
    }

    /// Builder: carrega as fontes embutidas do vanilla (P753/P754).
    ///
    /// Usa `typst-assets` para obter os bytes em tempo de compilação.
    /// Isto garante que `Libertinus Serif` e as fontes de math/code do
    /// vanilla estejam sempre disponíveis.
    pub fn with_embedded_fonts(mut self) -> Self {
        let sets = crate::embedded_fonts::load_embedded_fonts();
        let mut slots = sets.text_slots;
        slots.extend(sets.math_code_slots);
        let mut book = sets.text_book;
        for info in sets.math_code_book.infos() {
            book.push(info.clone());
        }
        self.install_font_catalog((slots, book));
        self
    }

    /// Builder: schedules embedded, system and project discovery on demand.
    ///
    /// A ordem final é: texto embutido → sistema → math/code embutido →
    /// projecto. Isto garante que fontes especializadas do sistema (ex.:
    /// Noto Sans Devanagari) sejam preferidas no fallback de texto normal
    /// sobre fontes math/code embutidas que podem ter cobertura parcial
    /// ou glifos incorrectos para scripts não latinos (P754).
    pub fn with_fonts_and_system(mut self, font_paths: &[PathBuf]) -> Self {
        self.font_prefix = Some(Arc::new(EmbeddedPrefix::default()));
        let project_paths = font_paths.to_vec();
        self.font_catalog = OnceLock::new();
        self.font_loader = Box::new(move || Arc::new(fresh_catalog(&project_paths)));
        self.coverage_cache.get_mut().unwrap().clear();
        self
    }

    /// Builder: like `with_fonts_and_system`, leaving out system or embedded
    /// fonts when they are off, in the same order.
    pub fn with_font_selection(self, font_paths: &[PathBuf], system: bool, embedded: bool) -> Self {
        if system && embedded {
            return self.with_fonts_and_system(font_paths);
        }
        let mut world = self;
        let project_paths = font_paths.to_vec();
        world.font_prefix = None;
        world.font_catalog = OnceLock::new();
        world.font_loader = Box::new(move || {
            let sets = if embedded {
                crate::embedded_fonts::load_embedded_fonts()
            } else {
                crate::embedded_fonts::EmbeddedFontSets {
                    text_slots: Vec::new(),
                    text_book: FontBook::new(),
                    math_code_slots: Vec::new(),
                    math_code_book: FontBook::new(),
                }
            };
            let system_fonts = if system {
                crate::fontdb::load_system_fonts()
            } else {
                (Vec::new(), FontBook::new())
            };
            let project = crate::fonts::discover_fonts(&project_paths);
            Arc::new(combine_catalog(sets, system_fonts, project))
        });
        world.coverage_cache.get_mut().unwrap().clear();
        world
    }

    fn install_font_catalog(&mut self, catalog: (Vec<FontSlot>, FontBook)) {
        self.font_prefix = None;
        self.font_catalog = OnceLock::from(Arc::new(catalog));
        self.font_loader = Box::new(|| Arc::new((Vec::new(), FontBook::new())));
        self.coverage_cache.get_mut().unwrap().clear();
    }

    pub(super) fn font_catalog(&self) -> &(Vec<FontSlot>, FontBook) {
        self.font_catalog.get_or_init(|| (self.font_loader)()).as_ref()
    }

    /// **P927/P938** — pré-carrega a coverage das fontes do sistema se o
    /// documento contiver carateres que nenhuma fonte embutida cobre.
    ///
    /// Percorre a árvore de sintaxe do source (texto literal, texto matemático
    /// e blocos raw) e, no primeiro caractere não coberto pelas fontes
    /// embutidas, dispara `candidates_for_char`. Isso mantém o caminho caro
    /// lazy para casos que realmente precisam de fallback do sistema, mas
    /// evita abrir centenas de ficheiros de fonte em documentos latinos
    /// simples.
    ///
    /// Texto que só existe depois de `eval` (`context`, interpolações, dados
    /// de `read()`, etc.) não é visto aqui — esses casos continuam a usar o
    /// caminho lazy original, sem regressão face ao comportamento actual.
    pub fn preload_coverage_if_needed(&self, source: &Source) {
        let base = self.embedded_coverage_union();
        for node in Self::source_text_nodes(source.root()) {
            for c in node.as_str().chars() {
                if !base.contains(c as u32) {
                    // Dispara o scan lazy de todas as fontes do sistema.
                    let _ = self.candidates_for_char(c);
                    return;
                }
            }
        }
    }

    /// **P927/P938** — devolve a união das coberturas de todas as fontes
    /// embutidas. Usada como proxy conservador para a cobertura da fonte
    /// primária.
    fn embedded_coverage_union(&self) -> Coverage {
        let mut codepoints = Vec::new();
        for slot in &self.font_catalog().0 {
            // Apenas fontes embutidas (caminho marcador). Fontes do sistema e
            // de projecto são ignoradas de propósito — não as queremos abrir
            // só para decidir se precisamos de as abrir.
            if slot.path.as_os_str() != std::ffi::OsStr::new("<embedded>") {
                continue;
            }
            let Some(data) = slot.source_bytes() else { continue };
            let Ok(face) = ttf_parser::Face::parse(&data, slot.index) else { continue };
            let face_cov = crate::fonts::extract_coverage(&face);
            codepoints.extend(face_cov.iter());
        }
        Coverage::from_codepoints(codepoints)
    }

    /// **P927/P938** — percorre recursivamente a árvore de sintaxe e devolve
    /// os nós que contêm texto renderizado.
    fn source_text_nodes(
        node: &typst_core::entities::syntax_node::SyntaxNode,
    ) -> Vec<typst_core::entities::syntax_text::SyntaxText> {
        let mut result = Vec::new();
        let mut stack = vec![node];
        while let Some(node) = stack.pop() {
            match node.kind() {
                SyntaxKind::Text | SyntaxKind::MathText | SyntaxKind::RawTrimmed => {
                    result.push(node.text());
                }
                _ => {
                    stack.extend(node.children());
                }
            }
        }
        result
    }

    pub(super) fn world_select_font(
        &self,
        pattern: &FontNamePattern,
        variant: &FontVariant,
    ) -> Option<usize> {
        if matches!(pattern, FontNamePattern::Literal(_)) {
            if let Some(prefix) = &self.font_prefix {
                let book = &prefix.get().1;
                if let Some(index) = book.select_pattern(pattern, variant) {
                    let selected = book.infos()[index].variant;
                    if selected.weight.distance(variant.weight) == 0
                        && selected.style.distance(variant.style) == 0
                    {
                        return Some(index);
                    }
                }
            }
        }
        self.font_catalog().1.select_pattern(pattern, variant)
    }

    pub(super) fn world_font_info(&self, index: usize) -> Option<&FontInfo> {
        if let Some(prefix) = &self.font_prefix {
            if let Some(info) = prefix.get().1.infos().get(index) {
                return Some(info);
            }
        }
        self.font_catalog().1.infos().get(index)
    }

    pub(super) fn world_font(&self, index: usize) -> Option<Font> {
        if let Some(prefix) = &self.font_prefix {
            if let Some(slot) = prefix.get().0.get(index) {
                return slot.get();
            }
        }
        self.font_catalog().0.get(index)?.get()
    }

    /// **P938/P1410** — cobertura Unicode exacta híbrida por slot.
    ///
    /// Fontes do sistema chegam com coverage materializada pelo `fontdb`;
    /// somente slots ainda vazios abrem seus bytes e usam o cache lazy. Isso
    /// impede que uma consulta mapeie todos os ficheiros de fontes instaladas.
    pub(super) fn world_candidates_for_char(&self, c: char) -> Vec<usize> {
        let codepoint = c as u32;
        let (slots, book) = self.font_catalog();
        let mut cache = self.coverage_cache.lock().unwrap();
        let mut result = Vec::new();
        for (idx, slot) in slots.iter().enumerate() {
            let precomputed = &book.infos()[idx].coverage;
            let coverage = if precomputed.is_empty() {
                cache.entry(idx).or_insert_with(|| {
                    slot.source_bytes()
                        .map(|data| {
                            ttf_parser::Face::parse(&data, slot.index)
                                .map(|face| crate::fonts::extract_coverage(&face))
                                .unwrap_or_else(|_| Coverage::new())
                        })
                        .unwrap_or_else(Coverage::new)
                })
            } else {
                precomputed
            };
            if coverage.contains(codepoint) {
                result.push(idx);
            }
        }
        result
    }
}

pub(super) fn fresh_catalog(paths: &[PathBuf]) -> (Vec<FontSlot>, FontBook) {
    let sets = crate::embedded_fonts::load_embedded_fonts();
    let system = crate::fontdb::load_system_fonts();
    let project = crate::fonts::discover_fonts(paths);
    combine_catalog(sets, system, project)
}
pub(super) fn prepared_catalog(
    prepared: crate::fontdb::generation::PreparedDiscovery,
) -> (Vec<FontSlot>, FontBook) {
    let sets = crate::embedded_fonts::load_embedded_fonts();
    let system = crate::fontdb::catalog_from_database(&prepared.db);
    combine_catalog(sets, system, prepared.project_slots)
}
fn combine_catalog(
    sets: crate::embedded_fonts::EmbeddedFontSets,
    system: (Vec<FontSlot>, FontBook),
    project: Vec<FontSlot>,
) -> (Vec<FontSlot>, FontBook) {
    let mut slots = sets.text_slots;
    let mut book = sets.text_book;
    slots.extend(system.0);
    for info in system.1.infos() {
        book.push(info.clone());
    }
    slots.extend(sets.math_code_slots);
    for info in sets.math_code_book.infos() {
        book.push(info.clone());
    }
    let (project_slots, project_book) = crate::fonts::pair_slots_with_book(project);
    slots.extend(project_slots);
    for info in project_book.infos() {
        book.push(info.clone());
    }
    (slots, book)
}
impl SystemWorld {
    /// Captures a session policy without acquiring fonts until first demand.
    pub fn with_font_catalog_session(
        mut self,
        session: Arc<super::FontCatalogSession>,
        paths: &[PathBuf],
    ) -> Self {
        let paths = paths.to_vec();
        self.font_prefix = Some(Arc::clone(&session.prefix));
        self.font_catalog = OnceLock::new();
        self.font_loader = Box::new(move || session.catalog(&paths));
        self.coverage_cache.get_mut().unwrap().clear();
        self
    }
}

#[cfg(test)]
mod selective_font_foundation_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use typst_core::entities::{
        font_book::{FontVariant, FontWeight},
        font_list::FontNamePattern,
    };
    fn pattern(name: &str) -> FontNamePattern {
        FontNamePattern::Literal(name.into())
    }
    fn observed() -> (SystemWorld, Arc<AtomicUsize>) {
        observed_source("")
    }
    fn observed_source(text: &str) -> (SystemWorld, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        let count = calls.clone();
        let mut world =
            SystemWorld::for_stdin(".", text.into()).with_fonts_and_system(&[]);
        world.font_loader = Box::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            let sets = crate::embedded_fonts::load_embedded_fonts();
            let mut slots = sets.text_slots;
            let mut book = sets.text_book;
            let mut suffix = book.infos()[0].clone();
            suffix.variant.weight = FontWeight(100);
            let data = slots[0].get().unwrap();
            slots.push(FontSlot::new_embedded(
                "<fixture-suffix>".into(),
                data.as_slice().to_vec(),
            ));
            book.push(suffix);
            slots.extend(sets.math_code_slots);
            for info in sets.math_code_book.infos() {
                book.push(info.clone());
            }
            Arc::new((slots, book))
        });
        (world, calls)
    }
    #[test]
    fn prefix_selection_metadata_and_bytes_do_not_acquire_mutable_catalog() {
        let (world, calls) = observed();
        let request = pattern("Libertinus Serif");
        let idx = world.select_font(&request, &FontVariant::default()).unwrap();
        let info = world.font_info(idx).unwrap().clone();
        let bytes = world.font(idx).unwrap();
        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "prefix query must not acquire mutable catalog"
        );
        assert!(world.font_catalog.get().is_none());
        assert_eq!(
            world.book().select_pattern(&request, &FontVariant::default()),
            Some(idx)
        );
        assert_eq!(world.book().infos()[idx], info);
        assert_eq!(world.font(idx).unwrap().as_slice(), bytes.as_slice());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn nonminimum_absent_and_nonprefix_queries_use_complete_catalog() {
        let (world, calls) = observed();
        let req = FontVariant { weight: FontWeight(100), ..Default::default() };
        let idx = world.select_font(&pattern("Libertinus Serif"), &req).unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            Some(idx),
            world.book().select_pattern(&pattern("Libertinus Serif"), &req)
        );
        assert_eq!(world.font_info(idx).unwrap().variant.weight, FontWeight(100));
        for family in ["Lab Missing", "New Computer Modern Math", "DejaVu Sans Mono"] {
            let (world, calls) = observed();
            let got = world.select_font(&pattern(family), &FontVariant::default());
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            assert_eq!(
                got,
                world.book().select_pattern(&pattern(family), &FontVariant::default())
            );
        }
        let (world, calls) = observed();
        assert!(world.font_info(usize::MAX).is_none());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn replacement_policies_discard_old_prefix() {
        let (world, _) = observed();
        assert!(world
            .select_font(&pattern("Libertinus Serif"), &FontVariant::default())
            .is_some());
        let world = world.with_fonts(vec![]);
        assert!(world
            .select_font(&pattern("Libertinus Serif"), &FontVariant::default())
            .is_none());
        assert!(world.font_info(0).is_none());
        assert!(world.font(0).is_none());
        let (world, _) = observed();
        let mut world = world.with_system_fonts();
        let calls = Arc::new(AtomicUsize::new(0));
        let count = calls.clone();
        world.font_loader = Box::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            Arc::new((vec![], FontBook::new()))
        });
        assert!(world
            .select_font(&pattern("Libertinus Serif"), &FontVariant::default())
            .is_none());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn concurrent_prefix_queries_keep_one_complete_catalog_boundary() {
        let (world, calls) = observed();
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let world = &world;
                scope.spawn(move || {
                    let i = world
                        .select_font(
                            &pattern("Libertinus Serif"),
                            &FontVariant::default(),
                        )
                        .unwrap();
                    assert!(world.font(i).is_some());
                    assert!(world.font_info(i).is_some());
                });
            }
        });
        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "concurrent prefix queries must stay lazy"
        );
        let _ = world.book();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn control_primary_measurement_does_not_require_control_cmap() {
        use typst_core::compiler::layout::FontMetrics;
        use typst_core::entities::{
            font_list::FontList,
            layout_types::{Pt, TextStyle},
        };
        for text in [
            "\u{2060}",
            "\u{200e}",
            "\u{200f}",
            "\u{61c}",
            "\u{202a}\u{202c}",
            "\u{2066}\u{2069}",
            "\u{2060}\u{200e}\u{2060}",
        ] {
            let (world, calls) = observed();
            let mut style = TextStyle::regular(Pt(11.0));
            style.font = Some(FontList::single("Libertinus Serif".into()));
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            assert!(metrics.advance_shaped(text, style.size, &style).is_some());
            assert_eq!(
                calls.load(Ordering::SeqCst),
                0,
                "control-only selection must not acquire the full catalog: {text:?}"
            );
        }
    }
    #[test]
    fn control_primary_outline_pdf_keeps_catalog_lazy() {
        let text = "#set page(width:400pt,height:600pt)\n#outline()\nAlpha\n#for i in range(3) [= Heading #i\n#lorem(10)\n#pagebreak()]";
        let (world, calls) = observed_source(text);
        let source = world.source(world.main()).unwrap();
        let (result, warnings) = crate::pipeline::compile_to_pdf_bytes(
            &world,
            &source,
            crate::export::StreamMode::Verbose,
            crate::export::PdfTags::Disabled,
        );
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(result.unwrap().starts_with(b"%PDF"));
        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "outline control must not acquire unused mutable catalog"
        );
        assert!(world.font_catalog.get().is_none());
    }
    #[test]
    fn control_primary_mixed_text_keeps_longest_run_refusal() {
        use typst_core::compiler::layout::FontMetrics;
        use typst_core::entities::{
            font_list::FontList,
            layout_types::{Pt, TextStyle},
        };
        let (world, calls) = observed();
        let slot = world
            .select_font(&pattern("Libertinus Serif"), &FontVariant::default())
            .unwrap();
        let font = world.font(slot).unwrap();
        assert!(
            ttf_parser::Face::parse(font.as_slice(), 0)
                .unwrap()
                .glyph_index('\u{2060}')
                .is_none(),
            "fixture must lack control cmap"
        );
        let mut style = TextStyle::regular(Pt(11.0));
        style.font = Some(FontList::single("Libertinus Serif".into()));
        let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
        assert!(metrics.advance_shaped("A\u{2060}B", style.size, &style).is_some());
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "mixed control input must retain the full longest-run route"
        );
    }
    #[test]
    fn control_primary_nonminimum_variant_still_acquires_catalog() {
        use typst_core::compiler::layout::FontMetrics;
        use typst_core::entities::{
            font_list::FontList,
            layout_types::{Pt, TextStyle},
        };
        let (world, calls) = observed();
        let mut style = TextStyle::regular(Pt(11.0));
        style.font = Some(FontList::single("Libertinus Serif".into()));
        style.weight = Some(500);
        let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
        assert!(metrics.advance_shaped("\u{2060}", style.size, &style).is_some());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn whole_pdf_embedded_text_never_acquires_full_catalog() {
        let text="#set text(font: \"Libertinus Serif\")\nAlpha beta gamma.\n\nDelta epsilon.\n";
        let (world, calls) = observed_source(text);
        let source = world.source(world.main()).unwrap();
        let (result, warnings) = crate::pipeline::compile_to_pdf_bytes(
            &world,
            &source,
            crate::export::StreamMode::Verbose,
            crate::export::PdfTags::Disabled,
        );
        assert!(warnings.is_empty(), "{warnings:?}");
        let pdf = result.unwrap();
        assert!(pdf.starts_with(b"%PDF"));
        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "full pipeline must not acquire unused mutable catalog"
        );
        assert!(world.font_catalog.get().is_none());
    }
    #[test]
    fn regex_uses_full_catalog_even_for_a_minimum_prefix_match() {
        let (world, calls) = observed();
        let pattern = FontNamePattern::Regex(
            typst_core::entities::regex::Regex::new("Libertinus Serif").unwrap(),
        );
        let got = world.select_font(&pattern, &FontVariant::default());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(got, world.book().select_pattern(&pattern, &FontVariant::default()));
    }
    #[test]
    fn session_shares_only_immutable_prefix_across_fresh_worlds() {
        let session = Arc::new(super::super::FontCatalogSession::new());
        let a =
            SystemWorld::for_eval(".").with_font_catalog_session(session.clone(), &[]);
        let b = SystemWorld::for_eval(".").with_font_catalog_session(session, &[]);
        let request = pattern("Libertinus Serif");
        let i = a.select_font(&request, &FontVariant::default()).unwrap();
        assert_eq!(b.select_font(&request, &FontVariant::default()), Some(i));
        let x = a.font(i).unwrap();
        let y = b.font(i).unwrap();
        assert_eq!(x.as_slice(), y.as_slice());
        assert_eq!(
            x.as_slice().as_ptr(),
            y.as_slice().as_ptr(),
            "immutable prefix bytes are session-owned"
        );
        assert!(a.font_catalog.get().is_none() && b.font_catalog.get().is_none());
        let a = a.with_fonts(vec![]);
        assert!(a.font_info(i).is_none());
        assert!(b.font_info(i).is_some());
        assert!(b.font(i).is_some());
    }
}

#[cfg(test)]
mod metric_epoch_tests {
    use super::*;
    use typst_core::contracts::world::World;
    #[test]
    fn metric_epoch_embedded_prefix_is_lazy_and_full_book_revokes() {
        let world = SystemWorld::for_eval(".").with_fonts_and_system(&[]);
        let first = world.font_query_epoch().expect("immutable prefix should certify");
        assert!(world.font_catalog.get().is_none());
        assert!(world.font_prefix.as_ref().unwrap().catalog.get().is_none());
        let index = world
            .select_font(
                &FontNamePattern::Literal("Libertinus Serif".into()),
                &FontVariant::default(),
            )
            .unwrap();
        assert!(matches!(world.font(index), Some(Font::Vec(_))));
        assert!(world.font_info(index).is_some());
        assert!(Arc::ptr_eq(&first, &world.font_query_epoch().unwrap()));
        let _ = world.book();
        assert!(world.font_query_epoch().is_none());
    }
    #[test]
    fn metric_epoch_missing_query_and_replaced_builder_revoke() {
        let world = SystemWorld::for_eval(".").with_fonts_and_system(&[]);
        assert!(world.font_query_epoch().is_some());
        assert!(world
            .select_font(
                &FontNamePattern::Literal("epoch-refuter-absent-family".into()),
                &FontVariant::default()
            )
            .is_none());
        assert!(world.font_query_epoch().is_none());
        let world = SystemWorld::for_eval(".")
            .with_fonts_and_system(&[])
            .with_fonts(vec![]);
        assert!(world.font_query_epoch().is_none());
    }
}
