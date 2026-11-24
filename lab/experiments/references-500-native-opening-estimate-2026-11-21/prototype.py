"""Use live font edges and layout defaults in the provisional ordinal model."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-one-opening-traversal-2026-11-20"
runpy.run_path(str(prior / "prototype.py"))
pipeline = Path(sys.argv[1]) / "03_infra/src/pipeline"
pdf = pipeline / "pdf.rs"
running = pipeline / "context_stabilization/page_running.rs"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


replace(pdf, """pub(super) fn lab_predict_outline_ordinals(source: &Source, count: usize) -> Option<Vec<usize>> {
    let (prefix, _) = watch_probe_tail(source);""",
"""pub(super) fn lab_predict_outline_ordinals(
    source: &Source,
    count: usize,
    native: Option<&impl typst_core::compiler::layout::FontMetrics>,
) -> Option<Vec<usize>> {
    let (prefix, _) = watch_probe_tail(source);
    let mut has_outline_title = false;""")
replace(pdf, """        } else if node.kind() == SyntaxKind::Heading {
            if awaiting_loop || node.children().find(|c| c.kind() == SyntaxKind::HeadingMarker)?""",
"""        } else if let Some(args) = inert_call(node, "outline") {
            has_outline_title = args.is_empty();
        } else if node.kind() == SyntaxKind::Heading {
            if awaiting_loop || node.children().find(|c| c.kind() == SyntaxKind::HeadingMarker)?""")
replace(pdf, """    // Calibration at the default face and styles, in points. The actual
    // compiler must independently re-observe all these page numbers.
    let body_step = 20.438;
    let outline_step = 14.388;
    let usable = height - 2.0 * width.min(height) / 8.4;
    let slots = (usable / body_step).floor() as usize;
    let first = (count as f64 * outline_step / body_step).floor() as usize;
    if slots == 0 || first >= slots { return None; }
    let section_slots = repetitions.checked_add(1)?;""",
"""    use typst_core::compiler::layout::vanilla_defaults::{PAR_LEADING, PAR_SPACING};
    use typst_core::entities::layout_types::{PageConfig, TextStyle};
    use typst_core::entities::style_chain::StyleChain;

    let page = PageConfig { width, height, ..PageConfig::default() };
    let usable = height - 2.0 * page.auto_margin();
    let (body_step, outline_step, title_cost, section_overhead) = if let Some(metrics) = native {
        let body = TextStyle::from(&StyleChain::default_chain());
        // The experiment's source grammar admits only level-1 headings.
        // Layout's level-1 heading scale and margins are 1.4, 1.8 and 0.75 em.
        let heading = TextStyle { bold: true, heading_level: Some(1),
            size: body.size * 1.4, ..body.clone() };
        let (top, bottom) = metrics.text_edges(body.size, &body);
        let (heading_top, heading_bottom) = metrics.text_edges(heading.size, &heading);
        let spacing = body.size.0 * PAR_SPACING;
        let leading = body.size.0 * PAR_LEADING;
        let body_step = top.0 - bottom.0 + spacing;
        let outline_step = top.0 - bottom.0 + leading;
        let title_cost = if has_outline_title {
            heading_top.0 - heading_bottom.0 + body.size.0 * 0.75 - top.0
        } else { 0.0 };
        let section_transition = body.size.0 * 1.8 - spacing
            + heading_top.0 - heading_bottom.0 + leading;
        if !body_step.is_finite() || body_step <= 0.0
            || !outline_step.is_finite() || outline_step <= 0.0
            || !section_transition.is_finite() || section_transition < 0.0
        {
            return None;
        }
        let overhead = (section_transition / body_step).round() as usize;
        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
            eprintln!("lab-outline-native-step body={} outline={} title={} overhead={}",
                body_step, outline_step, title_cost, overhead);
        }
        (body_step, outline_step, title_cost, overhead)
    } else {
        (20.438, 14.388, 0.0, 1)
    };
    let slots = (usable / body_step).floor() as usize;
    let first = ((count as f64 * outline_step + title_cost) / body_step).floor() as usize;
    if slots == 0 || first >= slots { return None; }
    let section_slots = repetitions.checked_add(section_overhead)?;""")
replace(running, '''                            "model" => super::super::pdf::lab_predict_outline_ordinals(self.source, count),''',
'''                            "model" => super::super::pdf::lab_predict_outline_ordinals(
                                self.source, count, None::<&FallbackFontMetrics<'_>>),
                            "metrics" => super::super::pdf::lab_predict_outline_ordinals(
                                self.source, count, Some(&self.metrics)),''')
