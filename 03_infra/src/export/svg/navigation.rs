//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/navigation.md
//! @prompt-hash 4e55e946
//! @layer L3
//! @updated 2026-09-26

use super::serialization::fmt_num;
use super::{render_item, SvgRenderSession};
use typst_core::entities::label::Label;
use typst_core::entities::layout_types::{FrameItem, LinkTarget, Point, Size};
use xmlwriter::XmlWriter;

/// Registro imutável de destinos internos pertencentes à página SVG atual.
#[derive(Debug, Clone, Default)]
pub struct SvgDestinationContext {
    entries: Vec<SvgDestination>,
}

#[derive(Debug, Clone)]
struct SvgDestination {
    label: Label,
    point: Point,
    id: String,
}

impl SvgDestinationContext {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Constrói identidades locais por ordem semântica de label, nunca por
    /// posição, endereço ou ordem incidental do mapa de origem.
    pub fn from_destinations(
        destinations: impl IntoIterator<Item = (Label, Point)>,
    ) -> Self {
        let mut values: Vec<(Label, Point)> = destinations
            .into_iter()
            .filter(|(_, point)| point.x.0.is_finite() && point.y.0.is_finite())
            .collect();
        values.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));

        let mut entries: Vec<SvgDestination> = Vec::new();
        for (label, point) in values {
            if let Some(previous) = entries.last() {
                if previous.label == label {
                    // O registry produtivo é único por Label. Duplicatas não
                    // acrescentam nós; conflito não é promovido pelo exporter.
                    continue;
                }
            }
            let id = format!("crystalline-destination-{}", entries.len());
            entries.push(SvgDestination { label, point, id });
        }
        Self { entries }
    }

    pub fn id_for(&self, label: &Label) -> Option<&str> {
        self.entries
            .binary_search_by(|entry| entry.label.0.cmp(&label.0))
            .ok()
            .map(|index| self.entries[index].id.as_str())
    }

    pub(super) fn write_nodes(&self, xml: &mut XmlWriter) {
        for entry in &self.entries {
            xml.start_element("g");
            xml.write_attribute("id", &entry.id);
            xml.write_attribute("data-crystalline-destination", &entry.label.0);
            xml.write_attribute(
                "transform",
                &format!(
                    "translate({} {})",
                    fmt_num(entry.point.x.0),
                    fmt_num(entry.point.y.0)
                ),
            );
            xml.end_element();
        }
    }
}
pub(super) fn render_link(
    xml: &mut XmlWriter,
    target: &LinkTarget,
    items: &[FrameItem],
    pos: &Point,
    size: &Size,
    session: &mut SvgRenderSession<'_>,
) {
    xml.start_element("a");
    match target {
        LinkTarget::Url(url) => {
            xml.write_attribute("xlink:href", url.as_str());
        }
        LinkTarget::Destination(label) => {
            if let Some(id) = session.destinations.id_for(label) {
                xml.write_attribute("xlink:href", &format!("#{id}"));
            } else {
                xml.write_attribute(
                    "data-crystalline-link-fallback",
                    "destination-unknown",
                );
            }
        }
    }
    xml.start_element("rect");
    xml.write_attribute("x", &fmt_num(pos.x.0));
    xml.write_attribute("y", &fmt_num(pos.y.0));
    xml.write_attribute("width", &fmt_num(size.width.0));
    xml.write_attribute("height", &fmt_num(size.height.0));
    xml.write_attribute("fill", "transparent");
    xml.write_attribute("data-crystalline-link-hit-area", "true");
    xml.end_element();
    for item in items {
        render_item(xml, item, session);
    }
    xml.end_element();
}
