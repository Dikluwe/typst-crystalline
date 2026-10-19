//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world/fonts.md
//! @prompt-hash 1b429cde
//! @layer L3
//! @updated 2026-10-07

use super::SystemWorld;
use crate::fonts::FontSlot;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use typst_core::contracts::world::World;
use typst_core::entities::font_book::{Coverage, FontBook};
use typst_core::entities::source::Source;
use typst_core::entities::syntax_kind::SyntaxKind;
use typst_core::entities::world_types::Font;

impl SystemWorld {
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
        let project_paths = font_paths.to_vec();
        self.font_catalog = OnceLock::new();
        self.font_loader = Box::new(move || Arc::new(fresh_catalog(&project_paths)));
        self.coverage_cache.get_mut().unwrap().clear();
        self
    }

    fn install_font_catalog(&mut self, catalog: (Vec<FontSlot>, FontBook)) {
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

    pub(super) fn world_font(&self, index: usize) -> Option<Font> {
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
        self.font_catalog = OnceLock::new();
        self.font_loader = Box::new(move || session.catalog(&paths));
        self.coverage_cache.get_mut().unwrap().clear();
        self
    }
}
