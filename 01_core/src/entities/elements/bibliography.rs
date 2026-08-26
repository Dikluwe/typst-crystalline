//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/bibliography.md
//! @prompt-hash 3ff92fc6
//! @layer L1
//! @updated 2026-06-23
//!
//! `BibliographyElem` — Lote 10 P325 (por largura). `bibliography(entries, title)`.
//! **Locatável** (P181C) — absorve `extract_payload`. Contentor: recurse no title.
//!
//! **P429 (DEBT-63)** — `BibliographyElem` tornou-se um struct puro: sem
//! `resolved_style`. A identidade do elemento é dada só pelos dados de
//! entrada (`entries`, `path`, `title`, `style`, `locale`), pelo que deriva
//! `PartialEq`, `Eq` e `Hash`. O style CSL resolvido em eval time viaja numa
//! tabela lateral em `BibStore`, indexada por `BibliographyElem::style_key()`.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use ecow::EcoString;

use crate::entities::bib_entry::BibEntry;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Lista de referências (`entries`) com `title` opcional.
///
/// **P429**: struct puro — `PartialEq`/`Hash` derivados; `Eq` manual
/// porque `Content` não implementa `Eq`. O style resolvido não é
/// armazenado aqui; transporta-se via `BibStore`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BibliographyElem {
    pub entries: Vec<BibEntry>,
    /// **P419** — Path do ficheiro `.bib`/`.yaml`/`.json`. Quando `Some`, as
    /// `entries` são carregadas em eval time a partir deste path.
    pub path: Option<EcoString>,
    pub title: Option<Content>,
    /// **P418** — CSL style: nome built-in (ex: `"ieee"`, `"apa"`) ou path `.csl`.
    pub style: Option<EcoString>,
    /// **P418** — CSL locale override (ex: `"en-US"`, `"pt-PT"`). `None` usa locale do style.
    pub locale: Option<EcoString>,
}

impl BibliographyElem {
    /// **P429** — chave determinística para lookup do style resolvido no
    /// `BibStore`. Baseia-se no hash derivado de todos os campos de entrada,
    /// pelo que elementos iguais produzem a mesma chave.
    pub fn style_key(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Eq for BibliographyElem {}

impl Element for BibliographyElem {
    fn plain_text(&self) -> String {
        // Paridade content.rs:1810: title + cada entry formatada.
        let mut out = String::new();
        if let Some(t) = &self.title {
            out.push_str(&t.plain_text());
            out.push('\n');
        }
        for e in &self.entries {
            out.push_str(&format!(
                "[{}] {}. {} ({}).\n",
                e.key, e.author, e.title, e.year
            ));
        }
        out
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.title.is_none()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Bibliography(Arc::new(BibliographyElem {
            entries: self.entries.clone(),
            path: self.path.clone(),
            title: self.title.as_ref().map(|t| t.map_content(transform)).transpose()?,
            style: self.style.clone(),
            locale: self.locale.clone(),
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Bibliography(Arc::new(BibliographyElem {
            entries: self.entries.clone(),
            path: self.path.clone(),
            title: self.title.as_ref().map(|t| t.map_text(transform)),
            style: self.style.clone(),
            locale: self.locale.clone(),
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Bibliography)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Bibliography { entries: self.entries.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry() -> BibEntry {
        BibEntry::new("smith2024", "Smith", "Title", 2024)
    }

    fn ex() -> BibliographyElem {
        BibliographyElem {
            entries: vec![entry()],
            path: None,
            title: None,
            style: None,
            locale: None,
        }
    }

    #[test]
    fn plain_text_formata_entries() {
        assert_eq!(ex().plain_text(), "[smith2024] Smith. Title (2024).\n");
    }

    #[test]
    fn is_empty_so_sem_entries_e_sem_title() {
        assert!(!ex().is_empty());
        assert!(BibliographyElem {
            entries: vec![],
            path: None,
            title: None,
            style: None,
            locale: None,
        }
        .is_empty());
        assert!(!BibliographyElem {
            entries: vec![],
            path: None,
            title: Some(Content::text("Refs")),
            style: None,
            locale: None,
        }
        .is_empty());
    }

    #[test]
    fn map_content_recurse_title_preserva_entries() {
        let b = BibliographyElem {
            entries: vec![entry()],
            path: None,
            title: Some(Content::text("a")),
            style: None,
            locale: None,
        };
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match b.map_content(&mut f).unwrap() {
            Content::Bibliography(e) => {
                assert_eq!(e.entries.len(), 1);
                assert!(
                    matches!(e.title.as_ref().unwrap(), Content::Text(s) if s.as_str() == "Z")
                );
            }
            _ => panic!("esperado Bibliography"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Bibliography));
        assert_eq!(
            ex().to_payload(),
            Some(ElementPayload::Bibliography { entries: vec![entry()] })
        );
    }

    fn h(e: &BibliographyElem) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(
            h(&ex()),
            h(&BibliographyElem {
                entries: vec![],
                path: None,
                title: None,
                style: None,
                locale: None,
            })
        );
    }

    #[test]
    fn style_e_locale_participam_de_eq_e_hash() {
        let a = BibliographyElem {
            entries: vec![],
            path: None,
            title: None,
            style: Some("ieee".into()),
            locale: None,
        };
        let b = BibliographyElem {
            entries: vec![],
            path: None,
            title: None,
            style: Some("apa".into()),
            locale: None,
        };
        let c = BibliographyElem {
            entries: vec![],
            path: None,
            title: None,
            style: Some("ieee".into()),
            locale: Some("en-US".into()),
        };
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(h(&a), h(&b));
    }

    #[test]
    fn style_key_e_deterministica_e_igual_para_elementos_iguais() {
        let a = BibliographyElem {
            entries: vec![entry()],
            path: None,
            title: Some(Content::text("Refs")),
            style: Some("ieee".into()),
            locale: Some("en-US".into()),
        };
        let b = BibliographyElem {
            entries: vec![entry()],
            path: None,
            title: Some(Content::text("Refs")),
            style: Some("ieee".into()),
            locale: Some("en-US".into()),
        };
        assert_eq!(a, b);
        assert_eq!(a.style_key(), b.style_key());
        assert_ne!(a.style_key(), ex().style_key());
    }
}
