//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/document_info.md
//! @prompt-hash 88f901ac
//! @layer L1
//! @updated 2026-07-02
//!
//! **P536** — metadados do documento definidos por `#set document(...)`.
//! Transporte eval → pipeline → exportador PDF (`/Info`).

use ecow::EcoString;

/// Metadados de nível documento: título, autor, palavras-chave.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentInfo {
    /// Título do documento.
    pub title: Option<EcoString>,
    /// Autor do documento (string simples ou array de autores convertido).
    pub author: Option<EcoString>,
    /// Palavras-chave do documento.
    pub keywords: Option<EcoString>,
}

impl DocumentInfo {
    /// Cria metadados vazios.
    pub fn empty() -> Self {
        Self::default()
    }

    /// True se nenhum campo está preenchido.
    pub fn is_empty(&self) -> bool {
        self.title.is_none() && self.author.is_none() && self.keywords.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_document_info_is_empty() {
        let info = DocumentInfo::empty();
        assert!(info.is_empty());
        assert!(info.title.is_none());
        assert!(info.author.is_none());
        assert!(info.keywords.is_none());
    }

    #[test]
    fn document_info_with_title_is_not_empty() {
        let info = DocumentInfo {
            title: Some(EcoString::from("Título")),
            ..DocumentInfo::empty()
        };
        assert!(!info.is_empty());
        assert_eq!(info.title.as_deref(), Some("Título"));
    }
}
