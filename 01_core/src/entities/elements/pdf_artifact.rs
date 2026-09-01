//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/pdf_artifact.md
//! @prompt-hash 3ee69eba
//! @layer L1
//! @updated 2026-08-30

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
    Header,
    Footer,
    Watermark,
    PageNumber,
    LineNumber,
    Redaction,
    Bates,
    Page,
    PaginationOther,
    Layout,
    Background,
    Other,
}

impl ArtifactKind {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "header" => Self::Header,
            "footer" => Self::Footer,
            "watermark" => Self::Watermark,
            "page-number" => Self::PageNumber,
            "line-number" => Self::LineNumber,
            "redaction" => Self::Redaction,
            "bates" => Self::Bates,
            "page" => Self::Page,
            "pagination-other" => Self::PaginationOther,
            "layout" => Self::Layout,
            "background" => Self::Background,
            "other" => Self::Other,
            _ => return None,
        })
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Footer => "footer",
            Self::Watermark => "watermark",
            Self::PageNumber => "page-number",
            Self::LineNumber => "line-number",
            Self::Redaction => "redaction",
            Self::Bates => "bates",
            Self::Page => "page",
            Self::PaginationOther => "pagination-other",
            Self::Layout => "layout",
            Self::Background => "background",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PdfArtifactElem {
    pub kind: ArtifactKind,
    pub body: Content,
}

impl Element for PdfArtifactElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::PdfArtifact(Arc::new(Self {
            kind: self.kind,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::PdfArtifact(Arc::new(Self {
            kind: self.kind,
            body: self.body.map_text(transform),
        }))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "kind" => Some(Value::Str(self.kind.as_str().into())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1286_artifact_kind_roundtrip_cobre_dominio_fechado() {
        let names = [
            "header",
            "footer",
            "watermark",
            "page-number",
            "line-number",
            "redaction",
            "bates",
            "page",
            "pagination-other",
            "layout",
            "background",
            "other",
        ];
        for name in names {
            let kind = ArtifactKind::parse(name).expect("kind P1286 válido");
            assert_eq!(kind.as_str(), name);
        }
        assert_eq!(ArtifactKind::parse("structure"), None);
        assert_eq!(ArtifactKind::Other.as_str(), "other");
    }

    #[test]
    fn p1286_artifact_map_text_preserva_kind_e_morfologia() {
        let elem = PdfArtifactElem {
            kind: ArtifactKind::Header,
            body: Content::text("before"),
        };
        let mapped = elem.map_text(&mut |text| text.replace("before", "after"));
        let Content::PdfArtifact(mapped) = mapped else {
            panic!("map_text deve preservar o carrier PdfArtifact");
        };
        assert_eq!(mapped.kind, ArtifactKind::Header);
        assert_eq!(mapped.plain_text(), "after");
        assert_eq!(mapped.get_field("kind"), Some(Value::Str("header".into())));
    }
}
