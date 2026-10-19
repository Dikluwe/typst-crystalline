//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/bib_csl/plan.md
//! @prompt-hash cd7aeebd
//! @layer L1
//! @updated 2026-10-18

use crate::entities::content::Content;

#[derive(Debug)]
pub(in crate::compiler::layout) struct Entry {
    pub(in crate::compiler::layout) prefix: Option<Content>,
    pub(in crate::compiler::layout) body: Content,
}

#[derive(Debug)]
pub(in crate::compiler::layout) enum BibliographyPlan {
    Columns(Vec<Entry>),
    Hanging(Vec<Entry>),
    Plain(Vec<Entry>),
}

impl BibliographyPlan {
    pub(super) fn from_rendered(bib: &hayagriva::RenderedBibliography) -> Self {
        let entries: Vec<_> = bib
            .items
            .iter()
            .map(|item| Entry {
                prefix: item.first_field.as_ref().map(super::elem_child_to_content),
                body: super::elem_children_to_content(&item.content),
            })
            .collect();
        if entries.iter().any(|entry| entry.prefix.is_some()) {
            Self::Columns(entries)
        } else if bib.hanging_indent {
            Self::Hanging(entries)
        } else {
            Self::Plain(entries)
        }
    }

    pub(super) fn legacy_content(&self) -> Content {
        let entries = match self {
            Self::Columns(entries) | Self::Hanging(entries) | Self::Plain(entries) => {
                entries
            }
        };
        let mut items = Vec::with_capacity(entries.len().saturating_mul(2));
        for entry in entries {
            let mut line = Vec::new();
            if let Some(prefix) = &entry.prefix {
                line.push(prefix.clone());
                line.push(Content::text(" "));
            }
            line.push(entry.body.clone());
            items.push(Content::sequence(line));
            items.push(Content::linebreak());
        }
        Content::sequence(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hayagriva::{
        BibliographyItem, ElemChild, ElemChildren, Formatted, Formatting,
        RenderedBibliography,
    };

    fn rendered(prefix: Option<&str>, hanging: bool) -> RenderedBibliography {
        let text = |text: &str| {
            ElemChild::Text(Formatted {
                text: text.into(),
                formatting: Formatting::default(),
            })
        };
        RenderedBibliography {
            hanging_indent: hanging,
            second_field_align: None,
            line_spacing: std::num::NonZeroI16::new(1).unwrap(),
            entry_spacing: 0,
            items: vec![BibliographyItem {
                key: "entry".into(),
                first_field: prefix.map(text),
                content: ElemChildren(vec![text("BODY")]),
            }],
        }
    }

    #[test]
    fn a2_structure_mode_preserves_hanging_and_prefix_presence() {
        assert!(matches!(
            BibliographyPlan::from_rendered(&rendered(None, false)),
            BibliographyPlan::Plain(_)
        ));
        assert!(matches!(
            BibliographyPlan::from_rendered(&rendered(None, true)),
            BibliographyPlan::Hanging(_)
        ));
        for prefix in ["", "[1]"] {
            assert!(matches!(
                BibliographyPlan::from_rendered(&rendered(Some(prefix), true)),
                BibliographyPlan::Columns(_)
            ));
        }
    }

    #[test]
    fn a2_structure_legacy_projection_keeps_prefix_and_linebreak() {
        let plan = BibliographyPlan::from_rendered(&rendered(Some("[1]"), false));
        let expected = Content::sequence(vec![
            Content::sequence(vec![
                Content::text("[1]"),
                Content::text(" "),
                Content::text("BODY"),
            ]),
            Content::linebreak(),
        ]);
        assert_eq!(plan.legacy_content(), expected);
    }

    #[test]
    fn a2_structure_legacy_projection_does_not_invent_missing_prefix_space() {
        for hanging in [false, true] {
            let plan = BibliographyPlan::from_rendered(&rendered(None, hanging));
            assert_eq!(
                plan.legacy_content(),
                Content::sequence(vec![Content::text("BODY"), Content::linebreak()])
            );
        }
    }

    #[test]
    fn a2_structure_empty_plan_keeps_empty_legacy_content() {
        let mut empty = rendered(None, false);
        empty.items.clear();
        let plan = BibliographyPlan::from_rendered(&empty);
        assert!(
            matches!(plan, BibliographyPlan::Plain(ref entries) if entries.is_empty())
        );
        assert!(plan.legacy_content().is_empty());
    }
}
