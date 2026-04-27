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
//! url/doi (subpadrão #16 N=1→2 "refino tipo entity sem alteração
//! ao variant Content").
//!
//! Subset minimal extendido per ADR-0054 graded:
//! - 4 fields obrigatórios (key/author/title/year — P159A).
//! - 4 fields opcionais comuns (volume/pages/journal/publisher
//!   — P159D).
//! - 2 fields opcionais identificadores digitais (url/doi —
//!   P159E).
//! Outros fields vanilla (editor/series/note/isbn/location/
//! organization/etc.) **diferidos** para refinos futuros NÃO
//! reservados.
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
/// `Clone`/`Debug`/`PartialEq` derivados — entry é dados puros.
/// Refinos futuros (mais fields, formatação CSL, hyperlinks)
/// virão como métodos ou trait separada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    pub key:       String,
    pub author:    String,
    pub title:     String,
    pub year:      u32,
    // Passo 159D — fields opcionais comuns.
    pub volume:    Option<String>,
    pub pages:     Option<String>,
    pub journal:   Option<String>,
    pub publisher: Option<String>,
    // Passo 159E — fields opcionais identificadores digitais.
    pub url:       Option<String>,
    pub doi:       Option<String>,
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
            key:       key.into(),
            author:    author.into(),
            title:     title.into(),
            year,
            volume:    None,
            pages:     None,
            journal:   None,
            publisher: None,
            url:       None,
            doi:       None,
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
}
