//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/document_format_options.md
//! @prompt-hash 1aeee6c9
//! @layer L1
//! @updated 2026-09-21

use std::num::NonZeroUsize;

/// Opções documentais do export SVG.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SvgFormatOptions {
    pub pretty: bool,
}

/// Opções documentais do export PNG.
#[derive(Debug, Clone, PartialEq)]
pub struct PngFormatOptions {
    pub ppi: f64,
}

impl Default for PngFormatOptions {
    fn default() -> Self {
        Self { ppi: 144.0 }
    }
}

/// Opções documentais do export HTML.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HtmlFormatOptions {
    pub pretty: bool,
}

/// Estado de tagging do export PDF: três estados da linguagem, não um
/// `Option<bool>` de conveniência.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PdfTagged {
    #[default]
    Auto,
    Explicit(bool),
}

/// **P1468** — um intervalo inclusivo de páginas físicas (1-indexed).
///
/// Espelha `typst-library/src/layout/page.rs:801-808` do vanilla ratificado.
/// Limite `None` de um lado é "aberto" (sem restrição desse lado).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRange {
    pub start: Option<NonZeroUsize>,
    pub end: Option<NonZeroUsize>,
}

impl PageRange {
    /// Página única (`start == end == nr`), espelhando `PageRange::at` do
    /// vanilla (`typst-library/src/layout/page.rs:857-859`).
    pub fn at(nr: NonZeroUsize) -> Self {
        Self { start: Some(nr), end: Some(nr) }
    }

    pub fn includes(&self, page: NonZeroUsize) -> bool {
        match (self.start, self.end) {
            (Some(s), Some(e)) => (s..=e).contains(&page),
            (Some(s), None) => page >= s,
            (None, Some(e)) => page <= e,
            (None, None) => true,
        }
    }
}

/// **P1468** — lista de intervalos de páginas a exportar.
///
/// Não normaliza, deduplica nem ordena a lista interna (ver
/// `entities/document_format_options.md` P1468) — a semântica de "conjunto
/// físico único e ordenado" emerge exclusivamente de quem consulta
/// `includes_page` iterando as páginas físicas em ordem crescente
/// (`infra/export/builder.md`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PageRanges(Vec<PageRange>);

impl PageRanges {
    pub fn new(ranges: Vec<PageRange>) -> Self {
        Self(ranges)
    }

    pub fn includes_page(&self, page: NonZeroUsize) -> bool {
        self.0.iter().any(|r| r.includes(page))
    }
}

/// **P1469** (escopo reduzido — contrato de linguagem) — versão declarada do
/// header `%PDF-{version}`. Espelha `krilla::configure::version::PdfVersion`
/// (`~/.cargo/git/checkouts/krilla-*/crates/krilla/src/configure/version.rs:13-25`),
/// só os cinco valores que o cast documental/CLI aceita.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PdfVersion {
    V1_4,
    V1_5,
    V1_6,
    #[default]
    V1_7,
    V2_0,
}

impl PdfVersion {
    /// Nome bruto usado no header `%PDF-{version}` e no cast documental/CLI
    /// (`"1.4"`..`"2.0"`) — distinto do texto de diagnóstico `"PDF 1.4"` que
    /// `compiler/eval/rules.rs` constrói a partir deste valor.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::V1_4 => "1.4",
            Self::V1_5 => "1.5",
            Self::V1_6 => "1.6",
            Self::V1_7 => "1.7",
            Self::V2_0 => "2.0",
        }
    }

    /// Posição relativa entre os cinco valores, usada internamente para a
    /// interseção de range de `PdfStandards::new` (`min`/`max`). Não deriva
    /// `Ord` no tipo público porque o contrato de P1469 só lista
    /// `PartialEq, Eq, Hash` — este helper evita ampliar esse contrato.
    fn rank(self) -> u8 {
        match self {
            Self::V1_4 => 0,
            Self::V1_5 => 1,
            Self::V1_6 => 2,
            Self::V1_7 => 3,
            Self::V2_0 => 4,
        }
    }
}

/// **P1469** (escopo reduzido) — perfil PDF/A escolhido. Espelha
/// `krilla::configure::validate::Archival` (só os onze nomes que o cast
/// documental/CLI aceita).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfArchival {
    A1a,
    A1b,
    A2a,
    A2b,
    A2u,
    A3a,
    A3b,
    A3u,
    A4,
    A4f,
    A4e,
}

impl PdfArchival {
    /// Nome canônico de exibição (`"PDF/A-1a"`, ...), espelhando
    /// `Archival::as_str` (`krilla/src/configure/validate.rs:977-991`).
    /// `pub` (não `pub(crate)`): consumido por `compiler/eval/rules.rs`
    /// (mesma crate), mas também por `infra/pipeline.rs` e
    /// `04_wiring/src/main.rs` (crates separadas) para construir os
    /// mesmos textos de diagnóstico medidos contra o vanilla ratificado.
    pub fn label(self) -> &'static str {
        match self {
            Self::A1a => "PDF/A-1a",
            Self::A1b => "PDF/A-1b",
            Self::A2a => "PDF/A-2a",
            Self::A2b => "PDF/A-2b",
            Self::A2u => "PDF/A-2u",
            Self::A3a => "PDF/A-3a",
            Self::A3b => "PDF/A-3b",
            Self::A3u => "PDF/A-3u",
            Self::A4 => "PDF/A-4",
            Self::A4f => "PDF/A-4f",
            Self::A4e => "PDF/A-4e",
        }
    }

    /// Tabela min/max medida em `krilla/src/configure/validate.rs:993-1013`.
    pub(crate) fn min_version(self) -> PdfVersion {
        match self {
            Self::A1a | Self::A1b => PdfVersion::V1_4,
            Self::A2a | Self::A2b | Self::A2u | Self::A3a | Self::A3b | Self::A3u => {
                PdfVersion::V1_4
            }
            Self::A4 | Self::A4f | Self::A4e => PdfVersion::V2_0,
        }
    }

    pub(crate) fn max_version(self) -> PdfVersion {
        match self {
            Self::A1a | Self::A1b => PdfVersion::V1_4,
            Self::A2a | Self::A2b | Self::A2u | Self::A3a | Self::A3b | Self::A3u => {
                PdfVersion::V1_7
            }
            Self::A4 | Self::A4f | Self::A4e => PdfVersion::V2_0,
        }
    }
}

/// **P1469** (escopo reduzido) — perfil PDF/UA escolhido. Espelha
/// `krilla::configure::validate::Accessibility` (só `UA1`, único nome que o
/// cast documental/CLI aceita hoje).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfAccessibility {
    Ua1,
}

impl PdfAccessibility {
    /// `pub` pelos mesmos motivos de `PdfArchival::label`.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ua1 => "PDF/UA-1",
        }
    }

    /// Tabela min/max medida em `krilla/src/configure/validate.rs:1187-1201`.
    pub(crate) fn min_version(self) -> PdfVersion {
        match self {
            Self::Ua1 => PdfVersion::V1_4,
        }
    }

    pub(crate) fn max_version(self) -> PdfVersion {
        match self {
            Self::Ua1 => PdfVersion::V1_7,
        }
    }
}

/// Um dos 17 nomes brutos aceites pelo cast documental/CLI, antes de
/// categorizados em versão/archival/accessibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfStandard {
    V1_4,
    V1_5,
    V1_6,
    V1_7,
    V2_0,
    A1a,
    A1b,
    A2a,
    A2b,
    A2u,
    A3a,
    A3b,
    A3u,
    A4,
    A4f,
    A4e,
    Ua1,
}

/// Eixo em que um `PdfStandard` bruto se classifica — mesma classificação de
/// `typst-pdf/src/format.rs:613-632` do vanilla ratificado.
pub(crate) enum PdfStandardAxis {
    Version(PdfVersion),
    Archival(PdfArchival),
    Accessibility(PdfAccessibility),
}

impl PdfStandard {
    pub(crate) fn axis(self) -> PdfStandardAxis {
        match self {
            Self::V1_4 => PdfStandardAxis::Version(PdfVersion::V1_4),
            Self::V1_5 => PdfStandardAxis::Version(PdfVersion::V1_5),
            Self::V1_6 => PdfStandardAxis::Version(PdfVersion::V1_6),
            Self::V1_7 => PdfStandardAxis::Version(PdfVersion::V1_7),
            Self::V2_0 => PdfStandardAxis::Version(PdfVersion::V2_0),
            Self::A1a => PdfStandardAxis::Archival(PdfArchival::A1a),
            Self::A1b => PdfStandardAxis::Archival(PdfArchival::A1b),
            Self::A2a => PdfStandardAxis::Archival(PdfArchival::A2a),
            Self::A2b => PdfStandardAxis::Archival(PdfArchival::A2b),
            Self::A2u => PdfStandardAxis::Archival(PdfArchival::A2u),
            Self::A3a => PdfStandardAxis::Archival(PdfArchival::A3a),
            Self::A3b => PdfStandardAxis::Archival(PdfArchival::A3b),
            Self::A3u => PdfStandardAxis::Archival(PdfArchival::A3u),
            Self::A4 => PdfStandardAxis::Archival(PdfArchival::A4),
            Self::A4f => PdfStandardAxis::Archival(PdfArchival::A4f),
            Self::A4e => PdfStandardAxis::Archival(PdfArchival::A4e),
            Self::Ua1 => PdfStandardAxis::Accessibility(PdfAccessibility::Ua1),
        }
    }
}

/// **P1469** — os 17 nomes brutos aceites pelo cast documental/CLI, na
/// mesma ordem medida em `typst-pdf/src/format.rs:727-...`/clap
/// `--pdf-standard` do vanilla ratificado. Fonte única de verdade para
/// nome↔variante, reutilizada por `compiler/eval/rules.rs` (mensagens de
/// diagnóstico) e `compiler/layout/format_options.rs` (reconstrução a
/// partir do array já validado).
pub(crate) const PDF_STANDARD_NAMES: &[(&str, PdfStandard)] = &[
    ("1.4", PdfStandard::V1_4),
    ("1.5", PdfStandard::V1_5),
    ("1.6", PdfStandard::V1_6),
    ("1.7", PdfStandard::V1_7),
    ("2.0", PdfStandard::V2_0),
    ("a-1b", PdfStandard::A1b),
    ("a-1a", PdfStandard::A1a),
    ("a-2b", PdfStandard::A2b),
    ("a-2u", PdfStandard::A2u),
    ("a-2a", PdfStandard::A2a),
    ("a-3b", PdfStandard::A3b),
    ("a-3u", PdfStandard::A3u),
    ("a-3a", PdfStandard::A3a),
    ("a-4", PdfStandard::A4),
    ("a-4f", PdfStandard::A4f),
    ("a-4e", PdfStandard::A4e),
    ("ua-1", PdfStandard::Ua1),
];

/// Procura o nome bruto correspondente na tabela canônica.
pub(crate) fn pdf_standard_from_name(name: &str) -> Option<PdfStandard> {
    PDF_STANDARD_NAMES.iter().find(|(n, _)| *n == name).map(|(_, s)| *s)
}

/// Conjunto validado e resolvido de standards PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PdfStandards {
    pub version: PdfVersion,
    pub archival: Option<PdfArchival>,
    pub accessibility: Option<PdfAccessibility>,
}

impl Default for PdfStandards {
    fn default() -> Self {
        Self {
            version: PdfVersion::V1_7,
            archival: None,
            accessibility: None,
        }
    }
}

/// Um validador nomeado (archival ou accessibility) para compor mensagens
/// que citam "todos os validators envolvidos" (`NoOverlappingRange`/
/// `VersionOutOfRange` com mais de um validator no conjunto).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfValidator {
    Archival(PdfArchival),
    Accessibility(PdfAccessibility),
}

/// Motivo estruturado de rejeição; o consumer (`compiler/eval/rules.rs`)
/// formata a mensagem final exata a partir destes dados — este owner não
/// formata texto de diagnóstico (fronteira igual à de outros carriers L1).
/// `validators`, quando presente, preserva a ordem de citação medida
/// (mesma ordem em que os hints devem ser emitidos — arquival antes de
/// accessibility, únicas duas famílias possíveis).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfStandardsConflict {
    ConflictingVersions(PdfVersion, PdfVersion),
    MultipleArchival,
    MultipleAccessibility,
    /// Nenhuma versão comum entre os validators do conjunto (sem versão
    /// explícita envolvida — no máximo dois nomes, já que só há uma
    /// família archival e uma accessibility).
    NoOverlappingRange {
        validators: Vec<PdfValidator>,
    },
    /// Versão explícita fora do range de um ou mais validators do conjunto.
    VersionOutOfRange {
        version: PdfVersion,
        validators: Vec<PdfValidator>,
    },
}

impl PdfStandards {
    /// Valida uma lista de nomes brutos e produz o conjunto resolvido, na
    /// mesma ordem/regras medidas em `krilla::configure::ConfigurationBuilder`
    /// (`~/.cargo/git/checkouts/krilla-*/crates/krilla/src/configure/mod.rs:37-97`)
    /// e `typst-pdf/src/format.rs:570-674` do vanilla ratificado.
    pub fn new(
        names: impl IntoIterator<Item = PdfStandard>,
    ) -> Result<Self, PdfStandardsConflict> {
        let mut version: Option<PdfVersion> = None;
        let mut archival: Option<PdfArchival> = None;
        let mut accessibility: Option<PdfAccessibility> = None;

        for name in names {
            match name.axis() {
                PdfStandardAxis::Version(v) => {
                    if let Some(prev) = version {
                        return Err(PdfStandardsConflict::ConflictingVersions(prev, v));
                    }
                    version = Some(v);
                }
                PdfStandardAxis::Archival(a) => {
                    if archival.is_some() {
                        return Err(PdfStandardsConflict::MultipleArchival);
                    }
                    archival = Some(a);
                }
                PdfStandardAxis::Accessibility(ua) => {
                    if accessibility.is_some() {
                        return Err(PdfStandardsConflict::MultipleAccessibility);
                    }
                    accessibility = Some(ua);
                }
            }
        }

        let has_validators = archival.is_some() || accessibility.is_some();
        let min_range = [
            archival.map(PdfArchival::min_version),
            accessibility.map(PdfAccessibility::min_version),
        ]
        .into_iter()
        .flatten()
        .max_by_key(|v| v.rank())
        .unwrap_or(PdfVersion::V1_4);
        let max_range = [
            archival.map(PdfArchival::max_version),
            accessibility.map(PdfAccessibility::max_version),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|v| v.rank())
        .unwrap_or(PdfVersion::V2_0);

        // Citação em ordem: archival antes de accessibility (mesma ordem
        // medida nas mensagens "X and Y", `entities/document_format_options.md`
        // P1469, retificação de resolução de interseção vazia/versão fora
        // do range com mais de um validator no conjunto).
        let cited_validators = || -> Vec<PdfValidator> {
            let mut validators = Vec::with_capacity(2);
            if let Some(a) = archival {
                validators.push(PdfValidator::Archival(a));
            }
            if let Some(ua) = accessibility {
                validators.push(PdfValidator::Accessibility(ua));
            }
            validators
        };

        if has_validators && min_range.rank() > max_range.rank() {
            return Err(PdfStandardsConflict::NoOverlappingRange {
                validators: cited_validators(),
            });
        }

        let resolved_version = match version {
            Some(v) => {
                if has_validators
                    && (v.rank() < min_range.rank() || v.rank() > max_range.rank())
                {
                    return Err(PdfStandardsConflict::VersionOutOfRange {
                        version: v,
                        validators: cited_validators(),
                    });
                }
                v
            }
            None if has_validators => max_range,
            None => PdfVersion::V1_7,
        };

        Ok(Self { version: resolved_version, archival, accessibility })
    }
}

/// Opções documentais do export PDF.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PdfFormatOptions {
    pub tagged: PdfTagged,
    pub pretty: bool,
    /// **P1468** — seleção de páginas a exportar (`#set format.pdf(pages:
    /// ...)`). `None` = todas as páginas (default).
    pub pages: Option<PageRanges>,
    /// **P1469** (escopo reduzido — contrato de linguagem) — conjunto de
    /// standards PDF (`#set format.pdf(standard: ...)`). `None` = documento
    /// não decidiu; distinto de `Some(PdfStandards::default())`, alcançável
    /// via array vazio explícito.
    pub standard: Option<PdfStandards>,
}

/// Opções de formato resolvidas pela cascata do documento.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
    pub pdf: PdfFormatOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1462_defaults_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.svg.pretty);
        assert!(!first.html.pretty);
        second.svg.pretty = true;
        assert_ne!(first, second);
        assert!(!first.svg.pretty);
        assert!(!first.html.pretty);
    }

    #[test]
    fn p1465_html_default_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.html.pretty);
        second.html.pretty = true;
        assert_ne!(first, second);
        assert!(!first.html.pretty);
    }

    #[test]
    fn p1467_pdf_default_is_auto_tagged_and_compact_pretty() {
        let options = PdfFormatOptions::default();

        assert_eq!(options.tagged, PdfTagged::Auto);
        assert!(!options.pretty);
    }

    #[test]
    fn p1467_document_default_pdf_is_auto_tagged_and_compact_pretty() {
        let document = DocumentFormatOptions::default();

        assert_eq!(document.pdf.tagged, PdfTagged::Auto);
        assert!(!document.pdf.pretty);
    }

    #[test]
    fn p1467_pdf_tagged_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert_eq!(first.pdf.tagged, PdfTagged::Auto);
        second.pdf.tagged = PdfTagged::Explicit(true);
        assert_ne!(first, second);
        assert_eq!(first.pdf.tagged, PdfTagged::Auto);
    }

    #[test]
    fn p1467_pdf_pretty_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.pdf.pretty);
        second.pdf.pretty = true;
        assert_ne!(first, second);
        assert!(!first.pdf.pretty);
    }

    #[test]
    fn p1467_pdf_tagged_variants_are_pairwise_distinct() {
        assert_ne!(PdfTagged::Explicit(true), PdfTagged::Explicit(false));
        assert_ne!(PdfTagged::Explicit(true), PdfTagged::Auto);
        assert_ne!(PdfTagged::Explicit(false), PdfTagged::Auto);
        assert_eq!(PdfTagged::Explicit(true), PdfTagged::Explicit(true));
        assert_eq!(PdfTagged::Auto, PdfTagged::Auto);
    }

    #[test]
    fn p1467_pdf_field_is_orthogonal_to_svg_png_html() {
        let baseline = DocumentFormatOptions::default();
        let mut only_pdf_changed = baseline.clone();
        only_pdf_changed.pdf.tagged = PdfTagged::Explicit(false);
        only_pdf_changed.pdf.pretty = true;

        assert_eq!(only_pdf_changed.svg, baseline.svg);
        assert_eq!(only_pdf_changed.png, baseline.png);
        assert_eq!(only_pdf_changed.html, baseline.html);
        assert_ne!(only_pdf_changed.pdf, baseline.pdf);

        let mut only_others_changed = baseline.clone();
        only_others_changed.svg.pretty = true;
        only_others_changed.html.pretty = true;
        only_others_changed.png.ppi = 300.0;

        assert_eq!(only_others_changed.pdf, baseline.pdf);
        assert_eq!(only_others_changed.pdf.tagged, PdfTagged::Auto);
        assert!(!only_others_changed.pdf.pretty);
    }

    // ── P1468 (RED — Agente B) — `PageRange`/`PageRanges` e
    // `PdfFormatOptions::pages`. Nenhum dos três existe neste baseline
    // (`0b322a926dd2f86b49aabfeb0d2f287ce06aaace`); este módulo falha por
    // erro de compilação até o carrier P1468 (mesmo gate ADR-0127) ser
    // materializado por outro agente em paralelo.
    use std::num::NonZeroUsize;

    fn page(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    fn closed(start: usize, end: usize) -> PageRange {
        PageRange { start: Some(page(start)), end: Some(page(end)) }
    }

    fn open_at_start(end: usize) -> PageRange {
        PageRange { start: None, end: Some(page(end)) }
    }

    fn open_at_end(start: usize) -> PageRange {
        PageRange { start: Some(page(start)), end: None }
    }

    fn universal() -> PageRange {
        PageRange { start: None, end: None }
    }

    #[test]
    fn p1468_page_range_includes_closed_range() {
        let range = closed(2, 4);
        assert!(!range.includes(page(1)));
        assert!(range.includes(page(2)));
        assert!(range.includes(page(3)));
        assert!(range.includes(page(4)));
        assert!(!range.includes(page(5)));
    }

    #[test]
    fn p1468_page_range_includes_open_at_start() {
        // Aberto no início (sem `start`): cobre 1..=end.
        let range = open_at_start(3);
        assert!(range.includes(page(1)));
        assert!(range.includes(page(3)));
        assert!(!range.includes(page(4)));
    }

    #[test]
    fn p1468_page_range_includes_open_at_end() {
        // Aberto no fim (sem `end`): cobre start..
        let range = open_at_end(3);
        assert!(!range.includes(page(2)));
        assert!(range.includes(page(3)));
        assert!(range.includes(page(1000)));
    }

    #[test]
    fn p1468_page_range_includes_universal_range() {
        let range = universal();
        assert!(range.includes(page(1)));
        assert!(range.includes(page(9999)));
    }

    #[test]
    fn p1468_page_ranges_includes_page_with_overlapping_and_disjoint_ranges() {
        let ranges = PageRanges::new(vec![closed(1, 2), closed(2, 4), closed(10, 10)]);
        assert!(ranges.includes_page(page(1)));
        assert!(ranges.includes_page(page(2)));
        assert!(ranges.includes_page(page(3)));
        assert!(ranges.includes_page(page(4)));
        assert!(!ranges.includes_page(page(5)));
        assert!(!ranges.includes_page(page(9)));
        assert!(ranges.includes_page(page(10)));
        assert!(!ranges.includes_page(page(11)));
    }

    #[test]
    fn p1468_page_ranges_default_is_empty_and_pdf_default_pages_is_none() {
        let ranges = PageRanges::default();
        assert!(!ranges.includes_page(page(1)));

        let pdf = PdfFormatOptions::default();
        assert_eq!(pdf.pages, None);

        let document = DocumentFormatOptions::default();
        assert_eq!(document.pdf.pages, None);
    }

    #[test]
    fn p1468_page_ranges_does_not_normalize_dedupe_or_sort() {
        let a = closed(3, 3);
        let b = closed(1, 1);
        // Mesmos ranges, ordens diferentes: a lista interna não é
        // normalizada/ordenada (ver `entities/document_format_options.md`
        // P1468), logo a igualdade derivada é sensível à ordem.
        assert_ne!(PageRanges::new(vec![a, b]), PageRanges::new(vec![b, a]));

        // Duplicatas exatas não são deduplicadas: `includes_page` continua
        // correto porque é um `.any()`, não porque a lista foi limpa.
        let duplicated = PageRanges::new(vec![closed(5, 5), closed(5, 5)]);
        assert!(duplicated.includes_page(page(5)));
        assert!(!duplicated.includes_page(page(6)));
    }

    #[test]
    fn p1468_page_range_and_page_ranges_derive_eq_and_clone_are_independent() {
        let first = PdfFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert_eq!(first.pages, None);
        second.pages = Some(PageRanges::new(vec![closed(1, 1)]));
        assert_ne!(first, second);
        assert_eq!(first.pages, None);

        assert_eq!(closed(1, 2), closed(1, 2));
        assert_ne!(closed(1, 2), closed(1, 3));
        assert_ne!(closed(1, 2), open_at_end(1));
    }

    #[test]
    fn p1468_pdf_pages_field_is_orthogonal_to_tagged_pretty_svg_png_html() {
        let baseline = DocumentFormatOptions::default();
        let mut only_pages_changed = baseline.clone();
        only_pages_changed.pdf.pages = Some(PageRanges::new(vec![closed(2, 3)]));

        assert_eq!(only_pages_changed.svg, baseline.svg);
        assert_eq!(only_pages_changed.png, baseline.png);
        assert_eq!(only_pages_changed.html, baseline.html);
        assert_eq!(only_pages_changed.pdf.tagged, baseline.pdf.tagged);
        assert_eq!(only_pages_changed.pdf.pretty, baseline.pdf.pretty);
        assert_ne!(only_pages_changed.pdf.pages, baseline.pdf.pages);

        let mut only_others_changed = baseline.clone();
        only_others_changed.pdf.tagged = PdfTagged::Explicit(true);
        only_others_changed.pdf.pretty = true;
        only_others_changed.svg.pretty = true;

        assert_eq!(only_others_changed.pdf.pages, baseline.pdf.pages);
    }

    // ── P1469 (RED — Agente B) — `PdfStandard`/`PdfStandards`/
    // `PdfStandardsConflict` e `PdfFormatOptions::standard` (contrato de
    // linguagem, escopo reduzido — ver a nota no topo da secção P1469
    // deste L0). Nenhum destes tipos existe neste baseline
    // (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`); este módulo falha por
    // erro de compilação até o carrier P1469 (mesmo gate ADR-0127) ser
    // materializado por outro agente em paralelo.

    /// Standard "puro de versão" (sem archival/accessibility) para usar
    /// como segundo elemento de um conjunto, testando uma versão
    /// explícita ao lado de um perfil archival/accessibility.
    fn version_standard(version: PdfVersion) -> PdfStandard {
        match version {
            PdfVersion::V1_4 => PdfStandard::V1_4,
            PdfVersion::V1_5 => PdfStandard::V1_5,
            PdfVersion::V1_6 => PdfStandard::V1_6,
            PdfVersion::V1_7 => PdfStandard::V1_7,
            PdfVersion::V2_0 => PdfStandard::V2_0,
        }
    }

    #[test]
    fn p1469_conflict_two_versions_is_rejected() {
        let err = PdfStandards::new([PdfStandard::V1_4, PdfStandard::V1_7]).unwrap_err();
        assert_eq!(
            err,
            PdfStandardsConflict::ConflictingVersions(PdfVersion::V1_4, PdfVersion::V1_7)
        );
    }

    #[test]
    fn p1469_conflict_two_archival_is_rejected() {
        let err = PdfStandards::new([PdfStandard::A1b, PdfStandard::A2b]).unwrap_err();
        assert_eq!(err, PdfStandardsConflict::MultipleArchival);
    }

    #[test]
    fn p1469_conflict_two_accessibility_is_rejected() {
        // Só existe um nome de acessibilidade (`ua-1`); repeti-lo no
        // conjunto é a única forma de produzir este conflito.
        let err = PdfStandards::new([PdfStandard::Ua1, PdfStandard::Ua1]).unwrap_err();
        assert_eq!(err, PdfStandardsConflict::MultipleAccessibility);
    }

    #[test]
    fn p1469_conflict_no_overlapping_range_between_validators_is_rejected() {
        // `ua-1` (range 1.4..=1.7) e `a-4` (range 2.0..=2.0) não têm
        // interseção — conflito estrutural distinto de `VersionOutOfRange`
        // (que é sobre uma versão explícita, não sobre dois validators
        // incompatíveis entre si).
        let err = PdfStandards::new([PdfStandard::Ua1, PdfStandard::A4]).unwrap_err();
        assert_eq!(
            err,
            PdfStandardsConflict::NoOverlappingRange {
                validators: vec![
                    PdfValidator::Archival(PdfArchival::A4),
                    PdfValidator::Accessibility(PdfAccessibility::Ua1),
                ],
            }
        );
    }

    #[test]
    fn p1469_conflict_explicit_version_out_of_range_is_rejected() {
        let err = PdfStandards::new([PdfStandard::A1b, PdfStandard::V1_7]).unwrap_err();
        assert_eq!(
            err,
            PdfStandardsConflict::VersionOutOfRange {
                version: PdfVersion::V1_7,
                validators: vec![PdfValidator::Archival(PdfArchival::A1b)],
            }
        );

        let err = PdfStandards::new([PdfStandard::A4, PdfStandard::V1_7]).unwrap_err();
        assert_eq!(
            err,
            PdfStandardsConflict::VersionOutOfRange {
                version: PdfVersion::V1_7,
                validators: vec![PdfValidator::Archival(PdfArchival::A4)],
            }
        );
    }

    #[test]
    fn p1469_effective_version_explicit_within_range_is_used() {
        let standards = PdfStandards::new([PdfStandard::A2b, PdfStandard::V1_5]).unwrap();
        assert_eq!(standards.version, PdfVersion::V1_5);
        assert_eq!(standards.archival, Some(PdfArchival::A2b));
        assert_eq!(standards.accessibility, None);
    }

    #[test]
    fn p1469_effective_version_absent_with_validators_uses_range_maximum() {
        let a2b = PdfStandards::new([PdfStandard::A2b]).unwrap();
        assert_eq!(a2b.version, PdfVersion::V1_7, "a-2b sozinho -> efetivo 1.7");

        let a1b = PdfStandards::new([PdfStandard::A1b]).unwrap();
        assert_eq!(a1b.version, PdfVersion::V1_4, "a-1b sozinho -> efetivo 1.4");

        let a4 = PdfStandards::new([PdfStandard::A4]).unwrap();
        assert_eq!(a4.version, PdfVersion::V2_0, "a-4 sozinho -> efetivo 2.0");
    }

    #[test]
    fn p1469_effective_version_absent_without_validators_uses_default_1_7() {
        let standards = PdfStandards::new(Vec::<PdfStandard>::new()).unwrap();
        assert_eq!(standards, PdfStandards::default());
        assert_eq!(standards.version, PdfVersion::V1_7);
        assert_eq!(standards.archival, None);
        assert_eq!(standards.accessibility, None);
    }

    #[test]
    fn p1469_min_max_table_per_profile_is_measured_correctly() {
        // Tabela medida em `entities/document_format_options.md` P1469
        // (`krilla/src/configure/validate.rs:993-1013,1187-1201`). Para
        // cada perfil: sozinho resolve ao máximo do range; explícito no
        // mínimo e no máximo do range é aceite.
        let cases: &[(PdfStandard, PdfVersion, PdfVersion)] = &[
            (PdfStandard::A1a, PdfVersion::V1_4, PdfVersion::V1_4),
            (PdfStandard::A1b, PdfVersion::V1_4, PdfVersion::V1_4),
            (PdfStandard::A2a, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A2b, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A2u, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A3a, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A3b, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A3u, PdfVersion::V1_4, PdfVersion::V1_7),
            (PdfStandard::A4, PdfVersion::V2_0, PdfVersion::V2_0),
            (PdfStandard::A4f, PdfVersion::V2_0, PdfVersion::V2_0),
            (PdfStandard::A4e, PdfVersion::V2_0, PdfVersion::V2_0),
            (PdfStandard::Ua1, PdfVersion::V1_4, PdfVersion::V1_7),
        ];
        assert_eq!(cases.len(), 12, "12 perfis archival/accessibility no pin");

        for (profile, min, max) in cases.iter().copied() {
            let alone = PdfStandards::new([profile]).unwrap();
            assert_eq!(alone.version, max, "{profile:?}: ausente -> máximo do range");

            let at_min = PdfStandards::new([profile, version_standard(min)]).unwrap();
            assert_eq!(at_min.version, min, "{profile:?}: versão mínima aceite");

            let at_max = PdfStandards::new([profile, version_standard(max)]).unwrap();
            assert_eq!(at_max.version, max, "{profile:?}: versão máxima aceite");
        }
    }

    #[test]
    fn p1469_pdf_standards_default_is_1_7_no_archival_no_accessibility() {
        let standards = PdfStandards::default();
        assert_eq!(standards.version, PdfVersion::V1_7);
        assert_eq!(standards.archival, None);
        assert_eq!(standards.accessibility, None);
    }

    #[test]
    fn p1469_pdf_standards_eq_hash_and_clone_are_value_semantics() {
        use std::collections::HashSet;

        let a = PdfStandards::new([PdfStandard::A2b]).unwrap();
        let b = a;
        assert_eq!(a, b);

        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));

        let different = PdfStandards::new([PdfStandard::A1b]).unwrap();
        assert_ne!(a, different);
        assert!(!set.contains(&different));
    }

    #[test]
    fn p1469_pdf_format_options_default_standard_is_none() {
        let options = PdfFormatOptions::default();
        assert_eq!(options.standard, None);

        let document = DocumentFormatOptions::default();
        assert_eq!(document.pdf.standard, None);
    }

    #[test]
    fn p1469_pdf_standard_field_is_orthogonal_to_pages_tagged_pretty_svg_png_html() {
        let baseline = DocumentFormatOptions::default();
        let mut only_standard_changed = baseline.clone();
        only_standard_changed.pdf.standard =
            Some(PdfStandards::new([PdfStandard::A2b]).unwrap());

        assert_eq!(only_standard_changed.svg, baseline.svg);
        assert_eq!(only_standard_changed.png, baseline.png);
        assert_eq!(only_standard_changed.html, baseline.html);
        assert_eq!(only_standard_changed.pdf.tagged, baseline.pdf.tagged);
        assert_eq!(only_standard_changed.pdf.pretty, baseline.pdf.pretty);
        assert_eq!(only_standard_changed.pdf.pages, baseline.pdf.pages);
        assert_ne!(only_standard_changed.pdf.standard, baseline.pdf.standard);

        let mut only_others_changed = baseline.clone();
        only_others_changed.pdf.tagged = PdfTagged::Explicit(true);
        only_others_changed.pdf.pretty = true;
        only_others_changed.pdf.pages = Some(PageRanges::new(vec![closed(1, 1)]));
        only_others_changed.svg.pretty = true;

        assert_eq!(only_others_changed.pdf.standard, baseline.pdf.standard);
    }
}
