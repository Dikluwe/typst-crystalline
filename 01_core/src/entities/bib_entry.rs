//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/bib_entry.md
//! @prompt-hash 5a2c0ebd
//! @layer L1
//! @updated 2026-04-27
//!
//! Tipo entity `BibEntry` (entrada bibliográfica minimal).
//! Adicionado no Passo 159A (Model Bibliography + Cite par acoplado,
//! ADR-0060 §"Decisão 2" Fase 2 sub-passo 1) como suporte a
//! `Content::Bibliography { entries: Vec<BibEntry>, ... }`.
//!
//! Subset minimal de campos vanilla per ADR-0054 graded:
//! 4 fields universais (key/author/title/year). Fields adicionais
//! vanilla (volume/journal/publisher/url/doi/etc.) **diferidos**
//! para refinos futuros NÃO reservados (extensível sem breaking
//! change via adição de `Option<String>` fields).
//!
//! Estilo paralelo ao vanilla `hayagriva::Entry` mas com subset
//! extremamente reduzido — input cristalino é literal, sem parsing
//! externo. Refino futuro candidato a integração hayagriva via
//! ADR-0062 promovida (não reservada per política P158).

/// Entrada bibliográfica minimal — vanilla `hayagriva::Entry`
/// reduzido a 4 fields universais.
///
/// **Subset minimal** per ADR-0054 graded e diagnóstico P159A
/// §1.3. Input cristalino é literal — sem hayagriva, sem CSL.
///
/// Campos:
/// - `key`: identificador único (paridade vanilla `Label`).
///   Usado por `Content::Cite { key, ... }` para referenciar.
/// - `author`: campo universal em todas as styles bibliográficas.
/// - `title`: campo universal idem.
/// - `year`: campo universal; `u32` para anos positivos
///   (aceita 0 para "no year").
///
/// `Clone`/`Debug`/`PartialEq` derivados — entry é dados puros
/// sem métodos próprios neste passo. Refinos futuros (e.g.
/// formatação CSL) virão como métodos ou trait separada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    pub key:    String,
    pub author: String,
    pub title:  String,
    pub year:   u32,
}

impl BibEntry {
    /// Constrói `BibEntry` com todos os 4 fields per padrão de
    /// construtores cristalinos (parâmetros explícitos vs default).
    pub fn new(
        key:    impl Into<String>,
        author: impl Into<String>,
        title:  impl Into<String>,
        year:   u32,
    ) -> Self {
        Self {
            key:    key.into(),
            author: author.into(),
            title:  title.into(),
            year,
        }
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
        // Debug derivado deve incluir os 4 fields legíveis.
        assert!(dbg.contains("key"));
        assert!(dbg.contains("author"));
        assert!(dbg.contains("title"));
        assert!(dbg.contains("year"));
    }
}
