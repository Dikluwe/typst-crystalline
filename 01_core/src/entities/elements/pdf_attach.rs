//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/pdf_attach.md
//! @prompt-hash e1b72398
//! @layer L1
//! @updated 2026-08-30

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachedFileRelationship {
    Source,
    Data,
    Alternative,
    Supplement,
}

impl AttachedFileRelationship {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "source" => Self::Source,
            "data" => Self::Data,
            "alternative" => Self::Alternative,
            "supplement" => Self::Supplement,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PdfAttachElem {
    pub path: EcoString,
    pub data: Arc<Vec<u8>>,
    pub relationship: Option<AttachedFileRelationship>,
    pub mime_type: Option<EcoString>,
    pub description: Option<EcoString>,
}

impl Element for PdfAttachElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::PdfAttach(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::PdfAttach(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attachment() -> PdfAttachElem {
        PdfAttachElem {
            path: "proof.bin".into(),
            data: Arc::new(vec![0, 65, 10]),
            relationship: Some(AttachedFileRelationship::Data),
            mime_type: Some("application/octet-stream".into()),
            description: Some("proof".into()),
        }
    }

    #[test]
    fn p1286_relationship_parse_e_fechado() {
        assert_eq!(
            AttachedFileRelationship::parse("source"),
            Some(AttachedFileRelationship::Source)
        );
        assert_eq!(
            AttachedFileRelationship::parse("data"),
            Some(AttachedFileRelationship::Data)
        );
        assert_eq!(
            AttachedFileRelationship::parse("alternative"),
            Some(AttachedFileRelationship::Alternative)
        );
        assert_eq!(
            AttachedFileRelationship::parse("supplement"),
            Some(AttachedFileRelationship::Supplement)
        );
        assert_eq!(AttachedFileRelationship::parse("unspecified"), None);
    }

    #[test]
    fn p1286_attachment_e_invisivel_mas_nao_podavel() {
        let elem = attachment();
        assert_eq!(elem.plain_text(), "");
        assert!(!elem.is_empty());
    }

    #[test]
    fn p1286_attachment_map_text_preserva_os_cinco_campos() {
        let expected = attachment();
        let mapped = expected.map_text(&mut |text| text.to_uppercase());
        let Content::PdfAttach(mapped) = mapped else {
            panic!("map_text deve preservar o carrier PdfAttach");
        };
        assert_eq!(*mapped, expected);
    }
}
