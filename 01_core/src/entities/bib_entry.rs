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
//! (volume/pages/journal/publisher) per ADR-0065 critério #2 +
//! builder pattern (Opção C diagnóstico §8).
//!
//! Subset minimal extendido per ADR-0054 graded:
//! - 4 fields obrigatórios (key/author/title/year — P159A).
//! - 4 fields opcionais (volume/pages/journal/publisher — P159D).
//! Outros fields vanilla (url/doi/editor/series/note/isbn/location/
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
/// **Campos opcionais** (P159D — selecção universal per
/// ADR-0065 critério #2):
/// - `volume`: universal em journals/proceedings/livros multi-volume.
/// - `pages`: universal em qualquer publicação com paginação.
/// - `journal`: universal em artigos de journal.
/// - `publisher`: universal em livros/tech reports/manuals.
///
/// `Clone`/`Debug`/`PartialEq` derivados — entry é dados puros.
/// Refinos futuros (mais fields, formatação CSL) virão como
/// métodos ou trait separada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    pub key:       String,
    pub author:    String,
    pub title:     String,
    pub year:      u32,
    // Passo 159D — fields opcionais.
    pub volume:    Option<String>,
    pub pages:     Option<String>,
    pub journal:   Option<String>,
    pub publisher: Option<String>,
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
}
