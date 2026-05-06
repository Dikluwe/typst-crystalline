//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/bib_entry.md
//! @prompt-hash 5a2c0ebd
//! @layer L1
//! @updated 2026-04-27
//!
//! Tipo entity `BibEntry` (entrada bibliográfica minimal extendida).
//! Adicionado no Passo 159A (Model Bibliography + Cite par acoplado,
//! ADR-0060 §"Decisão 2" Fase 2 sub-passo 1) como suporte a
//! `Content::Bibliography { entries: Vec<BibEntry>, ... }`.
//! Extendido no Passo 159D com 4 fields universais comuns
//! (volume/pages/journal/publisher) + builder pattern (Opção C
//! diagnóstico §8). Extendido no Passo 159E com par natural
//! url/doi. Extendido no Passo 159G com 6 fields restantes
//! comuns hayagriva (editor/series/note/isbn/location/
//! organization) — **subpadrão #16 atinge N=3** "refino tipo
//! entity sem alteração ao variant Content" (limiar formalização
//! N=3-4).
//!
//! Subset minimal extendido per ADR-0054 graded:
//! - 4 fields obrigatórios (key/author/title/year — P159A).
//! - 4 fields opcionais comuns (volume/pages/journal/publisher
//!   — P159D).
//! - 2 fields opcionais identificadores digitais (url/doi —
//!   P159E).
//! - 6 fields opcionais restantes (editor/series/note/isbn/
//!   location/organization — P159G).
//! Total **16 fields** (cobertura ~70-75% hayagriva universais).
//! Restantes vanilla (booktitle/address/chapter/type/institution/
//! etc.) **diferidos** para refinos futuros NÃO reservados.
//!
//! Estilo paralelo ao vanilla `hayagriva::Entry` mas com subset
//! extremamente reduzido — input cristalino é literal, sem parsing
//! externo. Refino futuro candidato a integração hayagriva via
//! ADR-0062 promovida (não reservada per política P158).

/// Entrada bibliográfica minimal extendida — vanilla
/// `hayagriva::Entry` reduzido a 4+4 fields universais.
///
/// **Subset minimal extendido** per ADR-0054 graded e diagnósticos
/// P159A §1.3 + P159D §1. Input cristalino é literal — sem
/// hayagriva, sem CSL.
///
/// **Campos obrigatórios** (P159A):
/// - `key`: identificador único (paridade vanilla `Label`).
///   Usado por `Content::Cite { key, ... }` para referenciar.
/// - `author`: campo universal em todas as styles bibliográficas.
/// - `title`: campo universal idem.
/// - `year`: campo universal; `u32` para anos positivos
///   (aceita 0 para "no year").
///
/// **Campos opcionais comuns** (P159D — selecção universal per
/// ADR-0065 critério #2):
/// - `volume`: universal em journals/proceedings/livros multi-volume.
/// - `pages`: universal em qualquer publicação com paginação.
/// - `journal`: universal em artigos de journal.
/// - `publisher`: universal em livros/tech reports/manuals.
///
/// **Campos opcionais identificadores digitais** (P159E — par
/// natural identificado em P159D §9):
/// - `url`: identificador digital genérico; plaintext literal
///   (sem URL parsing/validation per ADR-0054 graded).
/// - `doi`: Digital Object Identifier; plaintext literal com
///   prefixo `doi:` no render (sem regex validation).
///
/// **Campos opcionais restantes comuns** (P159G — segunda
/// metade hayagriva universais):
/// - `editor`: editor(es) da publicação; render `(Ed. {editor})`.
/// - `series`: série/colecção; render `({series})`.
/// - `note`: nota auxiliar; render `[{note}]`.
/// - `isbn`: ISBN sem validation; render `isbn:{isbn}`.
/// - `location`: cidade/país; render `{location}: {publisher}`.
/// - `organization`: instituição publicadora (substitutivo a
///   publisher quando publisher ausente).
///
/// `Clone`/`Debug`/`PartialEq` derivados — entry é dados puros.
/// Refinos futuros (mais fields, formatação CSL, hyperlinks,
/// tipos estruturados como `Vec<Person>` editor) virão como
/// métodos ou trait separada.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BibEntry {
    pub key:          String,
    pub author:       String,
    pub title:        String,
    pub year:         u32,
    // Passo 159D — fields opcionais comuns.
    pub volume:       Option<String>,
    pub pages:        Option<String>,
    pub journal:      Option<String>,
    pub publisher:    Option<String>,
    // Passo 159E — fields opcionais identificadores digitais.
    pub url:          Option<String>,
    pub doi:          Option<String>,
    // Passo 159G — fields opcionais restantes comuns.
    pub editor:       Option<String>,
    pub series:       Option<String>,
    pub note:         Option<String>,
    pub isbn:         Option<String>,
    pub location:     Option<String>,
    pub organization: Option<String>,
}

impl BibEntry {
    /// Constrói `BibEntry` com 4 fields obrigatórios; fields
    /// opcionais default `None`. Backwards compat preservada
    /// (P159A signature inalterada).
    pub fn new(
        key:    impl Into<String>,
        author: impl Into<String>,
        title:  impl Into<String>,
        year:   u32,
    ) -> Self {
        Self {
            key:          key.into(),
            author:       author.into(),
            title:        title.into(),
            year,
            volume:       None,
            pages:        None,
            journal:      None,
            publisher:    None,
            url:          None,
            doi:          None,
            editor:       None,
            series:       None,
            note:         None,
            isbn:         None,
            location:     None,
            organization: None,
        }
    }

    // ── Builder pattern P159D (Opção C diagnóstico §8) ─────────

    /// Builder fluente — adiciona `volume`. Consome `self`,
    /// devolve `Self` (paridade idiomática Rust builder).
    pub fn with_volume(mut self, v: impl Into<String>) -> Self {
        self.volume = Some(v.into());
        self
    }

    /// Builder fluente — adiciona `pages`.
    pub fn with_pages(mut self, p: impl Into<String>) -> Self {
        self.pages = Some(p.into());
        self
    }

    /// Builder fluente — adiciona `journal`.
    pub fn with_journal(mut self, j: impl Into<String>) -> Self {
        self.journal = Some(j.into());
        self
    }

    /// Builder fluente — adiciona `publisher`.
    pub fn with_publisher(mut self, pb: impl Into<String>) -> Self {
        self.publisher = Some(pb.into());
        self
    }

    // ── Builder pattern P159E (par natural url/doi) ────────────

    /// Builder fluente — adiciona `url` (plaintext literal).
    pub fn with_url(mut self, u: impl Into<String>) -> Self {
        self.url = Some(u.into());
        self
    }

    /// Builder fluente — adiciona `doi` (plaintext literal;
    /// render aplica prefixo `doi:` em layout).
    pub fn with_doi(mut self, d: impl Into<String>) -> Self {
        self.doi = Some(d.into());
        self
    }

    // ── Builder pattern P159G (6 fields restantes comuns) ──────

    /// Builder fluente — adiciona `editor` (render `(Ed. {editor})`).
    pub fn with_editor(mut self, e: impl Into<String>) -> Self {
        self.editor = Some(e.into());
        self
    }

    /// Builder fluente — adiciona `series` (render `({series})`).
    pub fn with_series(mut self, s: impl Into<String>) -> Self {
        self.series = Some(s.into());
        self
    }

    /// Builder fluente — adiciona `note` (render `[{note}]`).
    pub fn with_note(mut self, n: impl Into<String>) -> Self {
        self.note = Some(n.into());
        self
    }

    /// Builder fluente — adiciona `isbn` (render `isbn:{isbn}`;
    /// sem validation per ADR-0054 graded).
    pub fn with_isbn(mut self, i: impl Into<String>) -> Self {
        self.isbn = Some(i.into());
        self
    }

    /// Builder fluente — adiciona `location` (render
    /// `{location}:` antes de publisher).
    pub fn with_location(mut self, l: impl Into<String>) -> Self {
        self.location = Some(l.into());
        self
    }

    /// Builder fluente — adiciona `organization` (substitutivo
    /// a publisher quando publisher ausente).
    pub fn with_organization(mut self, o: impl Into<String>) -> Self {
        self.organization = Some(o.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bib_entry_constructor_preserves_fields() {
        let e = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024);
        assert_eq!(e.key,    "smith2024");
        assert_eq!(e.author, "Smith, J.");
        assert_eq!(e.title,  "On Crystal Math");
        assert_eq!(e.year,   2024);
    }

    #[test]
    fn bib_entry_partial_eq_cobre_4_fields() {
        let mk = || BibEntry::new("k", "A", "T", 2024);
        assert_eq!(mk(), mk());
        // Cada field divergente quebra equivalência.
        assert_ne!(mk(), BibEntry::new("k2", "A", "T", 2024));
        assert_ne!(mk(), BibEntry::new("k", "A2", "T", 2024));
        assert_ne!(mk(), BibEntry::new("k", "A", "T2", 2024));
        assert_ne!(mk(), BibEntry::new("k", "A", "T", 2025));
    }

    #[test]
    fn bib_entry_debug_formatting_inclui_fields() {
        let e = BibEntry::new("k", "A", "T", 2024);
        let dbg = format!("{:?}", e);
        // Debug derivado deve incluir os 4 fields obrigatórios legíveis.
        assert!(dbg.contains("key"));
        assert!(dbg.contains("author"));
        assert!(dbg.contains("title"));
        assert!(dbg.contains("year"));
    }

    // ── Passo 159D — fields opcionais + builder pattern ─────────

    #[test]
    fn bib_entry_new_default_optional_fields_none() {
        // Backwards compat P159A: new() com 4 args produz fields
        // novos default None.
        let e = BibEntry::new("k", "A", "T", 2024);
        assert!(e.volume.is_none());
        assert!(e.pages.is_none());
        assert!(e.journal.is_none());
        assert!(e.publisher.is_none());
    }

    #[test]
    fn bib_entry_builder_pattern_fluente() {
        let e = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_volume("12")
            .with_pages("1-10")
            .with_journal("Nature Communications")
            .with_publisher("ACM");
        assert_eq!(e.volume.as_deref(),    Some("12"));
        assert_eq!(e.pages.as_deref(),     Some("1-10"));
        assert_eq!(e.journal.as_deref(),   Some("Nature Communications"));
        assert_eq!(e.publisher.as_deref(), Some("ACM"));
        // Fields obrigatórios preservados.
        assert_eq!(e.key,    "smith2024");
        assert_eq!(e.author, "Smith, J.");
        assert_eq!(e.title,  "On Crystal Math");
        assert_eq!(e.year,   2024);
    }

    #[test]
    fn bib_entry_partial_eq_cobre_8_fields() {
        let mk = || BibEntry::new("k", "A", "T", 2024)
            .with_volume("1")
            .with_pages("10-20")
            .with_journal("J")
            .with_publisher("P");
        assert_eq!(mk(), mk());
        // Cada field opcional divergente quebra equivalência.
        assert_ne!(mk(), mk().with_volume("2"));
        assert_ne!(mk(), mk().with_pages("21-30"));
        assert_ne!(mk(), mk().with_journal("J2"));
        assert_ne!(mk(), mk().with_publisher("P2"));
        // Field opcional ausente vs presente quebra.
        let sem_volume = BibEntry::new("k", "A", "T", 2024)
            .with_pages("10-20")
            .with_journal("J")
            .with_publisher("P");
        assert_ne!(mk(), sem_volume);
    }

    // ── Passo 159E — par natural url/doi ────────────────────────

    #[test]
    fn bib_entry_new_default_url_doi_none() {
        // Backwards compat P159A+P159D: new() com 4 args produz
        // url/doi default None.
        let e = BibEntry::new("k", "A", "T", 2024);
        assert!(e.url.is_none());
        assert!(e.doi.is_none());
    }

    #[test]
    fn bib_entry_builder_url_doi() {
        let e = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_url("https://example.com/paper")
            .with_doi("10.1234/abc");
        assert_eq!(e.url.as_deref(), Some("https://example.com/paper"));
        assert_eq!(e.doi.as_deref(), Some("10.1234/abc"));
        // Outros fields permanecem default None.
        assert!(e.volume.is_none());
        assert!(e.pages.is_none());
    }

    #[test]
    fn bib_entry_partial_eq_cobre_10_fields() {
        let mk = || BibEntry::new("k", "A", "T", 2024)
            .with_volume("1")
            .with_pages("10-20")
            .with_journal("J")
            .with_publisher("P")
            .with_url("https://x.com")
            .with_doi("10.1/a");
        assert_eq!(mk(), mk());
        // Cada novo field opcional divergente quebra equivalência.
        assert_ne!(mk(), mk().with_url("https://y.com"));
        assert_ne!(mk(), mk().with_doi("10.2/b"));
        // Field opcional ausente vs presente quebra.
        let sem_url = BibEntry::new("k", "A", "T", 2024)
            .with_volume("1")
            .with_pages("10-20")
            .with_journal("J")
            .with_publisher("P")
            .with_doi("10.1/a");
        assert_ne!(mk(), sem_url);
    }

    // ── Passo 159G — 6 fields restantes comuns hayagriva ────────

    #[test]
    fn bib_entry_new_default_p159g_fields_none() {
        // Backwards compat P159A+P159D+P159E: new() com 4 args
        // produz 6 fields novos default None.
        let e = BibEntry::new("k", "A", "T", 2024);
        assert!(e.editor.is_none());
        assert!(e.series.is_none());
        assert!(e.note.is_none());
        assert!(e.isbn.is_none());
        assert!(e.location.is_none());
        assert!(e.organization.is_none());
    }

    #[test]
    fn bib_entry_builder_p159g_fields() {
        let e = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_editor("Doe, A.")
            .with_series("Crystal Studies")
            .with_note("See also Smith 2023")
            .with_isbn("978-0-1234-5678-9")
            .with_location("New York")
            .with_organization("ACM");
        assert_eq!(e.editor.as_deref(),       Some("Doe, A."));
        assert_eq!(e.series.as_deref(),       Some("Crystal Studies"));
        assert_eq!(e.note.as_deref(),         Some("See also Smith 2023"));
        assert_eq!(e.isbn.as_deref(),         Some("978-0-1234-5678-9"));
        assert_eq!(e.location.as_deref(),     Some("New York"));
        assert_eq!(e.organization.as_deref(), Some("ACM"));
    }

    #[test]
    fn bib_entry_partial_eq_cobre_16_fields() {
        let mk = || BibEntry::new("k", "A", "T", 2024)
            .with_volume("1")
            .with_url("https://x.com")
            .with_doi("10.1/a")
            .with_editor("Ed1")
            .with_series("S1")
            .with_isbn("978-0-1");
        assert_eq!(mk(), mk());
        // Cada novo field opcional divergente quebra equivalência.
        assert_ne!(mk(), mk().with_editor("Ed2"));
        assert_ne!(mk(), mk().with_series("S2"));
        assert_ne!(mk(), mk().with_isbn("978-0-2"));
    }

    #[test]
    fn bib_entry_builder_subset_p159g() {
        // Builder pattern combina subset (só editor + isbn) sem
        // outros fields P159G.
        let e = BibEntry::new("k", "A", "T", 2024)
            .with_editor("Ed1")
            .with_isbn("978-0-1");
        assert_eq!(e.editor.as_deref(), Some("Ed1"));
        assert_eq!(e.isbn.as_deref(),   Some("978-0-1"));
        // Outros P159G fields permanecem None.
        assert!(e.series.is_none());
        assert!(e.note.is_none());
        assert!(e.location.is_none());
        assert!(e.organization.is_none());
    }
}
