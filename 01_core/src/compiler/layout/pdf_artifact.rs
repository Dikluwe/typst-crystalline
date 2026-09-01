//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/pdf_artifact.md
//! @prompt-hash b8125413
//! @layer L1
//! @updated 2026-08-30

use ecow::EcoString;

use crate::entities::elements::pdf_artifact::{ArtifactKind, PdfArtifactElem};
use crate::entities::layout_types::{FrameItem, SemanticKind, SemanticPlacement};

use super::{FontMetrics, ImageSizer, Layouter};

fn marker(kind: ArtifactKind, label: EcoString) -> FrameItem {
    FrameItem::Semantic {
        kind: SemanticKind::Artifact(kind),
        placement: SemanticPlacement::Inline,
        alt: Some(label),
        items: Vec::new(),
    }
}

fn is_marker(item: &FrameItem, label: &str) -> bool {
    matches!(
        item,
        FrameItem::Semantic {
            kind: SemanticKind::Artifact(_),
            alt: Some(candidate),
            items,
            ..
        } if items.is_empty() && candidate.as_str() == label
    )
}

fn wrap_vector(
    items: &mut Vec<FrameItem>,
    start: &str,
    end: &str,
    kind: ArtifactKind,
    active: &mut bool,
) {
    let mut output = Vec::with_capacity(items.len());
    let mut fragment = Vec::new();
    for item in std::mem::take(items) {
        if is_marker(&item, start) {
            *active = true;
            continue;
        }
        if is_marker(&item, end) {
            if !fragment.is_empty() {
                output.push(FrameItem::Semantic {
                    kind: SemanticKind::Artifact(kind),
                    placement: SemanticPlacement::Inline,
                    alt: None,
                    items: std::mem::take(&mut fragment),
                });
            }
            *active = false;
            continue;
        }
        if *active {
            fragment.push(item);
        } else {
            output.push(item);
        }
    }
    if !fragment.is_empty() {
        output.push(FrameItem::Semantic {
            kind: SemanticKind::Artifact(kind),
            placement: SemanticPlacement::Inline,
            alt: None,
            items: fragment,
        });
    }
    *items = output;
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &PdfArtifactElem,
) {
    let id = layouter.artifact_marker_id;
    layouter.artifact_marker_id += 1;
    let start: EcoString = format!("\u{0}p1286-artifact-start-{id}").into();
    let end: EcoString = format!("\u{0}p1286-artifact-end-{id}").into();

    if layouter.regions.current.current_line.is_empty() {
        layouter
            .regions
            .current
            .current_items
            .push(marker(elem.kind, start.clone()));
    } else {
        layouter
            .regions
            .current
            .current_line
            .push(marker(elem.kind, start.clone()));
    }
    layouter.layout_content(&elem.body);
    if layouter.regions.current.current_line.is_empty() {
        layouter
            .regions
            .current
            .current_items
            .push(marker(elem.kind, end.clone()));
    } else {
        layouter
            .regions
            .current
            .current_line
            .push(marker(elem.kind, end.clone()));
    }

    let mut active = false;
    for page in &mut layouter.pages {
        wrap_vector(&mut page.items, &start, &end, elem.kind, &mut active);
    }
    wrap_vector(
        &mut layouter.regions.current.current_items,
        &start,
        &end,
        elem.kind,
        &mut active,
    );
    wrap_vector(
        &mut layouter.regions.current.current_line,
        &start,
        &end,
        elem.kind,
        &mut active,
    );
}
