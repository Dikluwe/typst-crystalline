"""Install calibrated ordinal prediction into the two-traversal lab source."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-two-opening-traversals-2026-11-20"
runpy.run_path(str(prior / "prototype.py"))
pipeline = Path(sys.argv[1]) / "03_infra/src/pipeline"


def replace(path, old, new):
    source = path.read_text()
    assert source.count(old) == 1, (path, old)
    path.write_text(source.replace(old, new))


pdf = pipeline / "pdf.rs"
replace(pdf, "fn inert_source(source: &Source) -> bool {", """// This is a calibrated predictor, not a source of accepted page geometry.
pub(super) fn lab_predict_outline_ordinals(source: &Source, count: usize) -> Option<Vec<usize>> {
    let (prefix, _) = watch_probe_tail(source);
    let mut size = None;
    let mut per_section = None;
    let mut sections = 0usize;
    let mut awaiting_loop = false;
    for node in prefix {
        if node.kind() == SyntaxKind::SetRule && inert_set(node) == Some(true) {
            let args = inert_args(inert_children(node).nth(2)?)?;
            let mut width = None;
            let mut height = None;
            for arg in args {
                let (name, value) = inert_named(arg)?;
                let raw = value.text();
                let points = raw.as_str().strip_suffix("pt")?.parse::<f64>().ok()?;
                if inert_identifier(name, "width") { width = Some(points); }
                if inert_identifier(name, "height") { height = Some(points); }
            }
            size = Some((width?, height?));
        } else if node.kind() == SyntaxKind::Heading {
            if awaiting_loop || node.children().find(|c| c.kind() == SyntaxKind::HeadingMarker)?
                .text().as_str() != "="
            {
                return None;
            }
            sections += 1;
            awaiting_loop = true;
        } else if node.kind() == SyntaxKind::ForLoop {
            if !awaiting_loop { return None; }
            let args = inert_call(inert_children(node).nth(3)?, "range")?;
            let [arg] = args.as_slice() else { return None; };
            let repetitions = arg.text().as_str().parse::<usize>().ok()?;
            if per_section.is_some_and(|known| known != repetitions) { return None; }
            per_section = Some(repetitions);
            awaiting_loop = false;
        }
    }
    if awaiting_loop || sections != count || !(2..=48).contains(&count) { return None; }
    let (width, height) = size?;
    let repetitions = per_section?;
    // Calibration at the default face and styles, in points. The actual
    // compiler must independently re-observe all these page numbers.
    let body_step = 20.438;
    let outline_step = 14.388;
    let usable = height - 2.0 * width.min(height) / 8.4;
    let slots = (usable / body_step).floor() as usize;
    let first = (count as f64 * outline_step / body_step).floor() as usize;
    if slots == 0 || first >= slots { return None; }
    let section_slots = repetitions.checked_add(1)?;
    Some((0..count).map(|ordinal| 1 + (first + ordinal * section_slots) / slots).collect())
}

fn inert_source(source: &Source) -> bool {""")

running = pipeline / "context_stabilization/page_running.rs"
replace(running, """                    return Ok(PhysicalCandidate {
                        index,
                        complete: false,""", """                    let mut index = index;
                    if let Ok(mode) = std::env::var("TEKT_LAB_OUTLINE_ORDINALS") {
                        let count = index.outlined_heading_locations.len();
                        let pages: Option<Vec<usize>> = match mode.as_str() {
                            "model" => super::super::pdf::lab_predict_outline_ordinals(self.source, count),
                            "witness" => std::env::var("TEKT_LAB_OUTLINE_WITNESS").ok()
                                .and_then(|s| s.split(',').map(|v| v.parse().ok()).collect()),
                            _ => None,
                        };
                        if let Some(pages) = pages.filter(|p| p.len() == count
                            && p.iter().all(|&page| (1..=512).contains(&page)))
                        {
                            let positions = index.outlined_heading_locations.iter().zip(&pages)
                                .map(|(&location, &page)| {
                                    (location, typst_core::entities::position::Position {
                                        page: std::num::NonZeroUsize::new(page).unwrap(),
                                        point: typst_core::entities::layout_types::Point {
                                            x: typst_core::entities::layout_types::Pt(0.0),
                                            y: typst_core::entities::layout_types::Pt(0.0),
                                        },
                                    })
                                }).collect();
                            index.inject_positions(
                                typst_core::entities::sealed_positions::SealedPositions::from_runtime(positions));
                            if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                                eprintln!("lab-outline-predict mode={} {:?}", mode, pages);
                            }
                        }
                    }
                    return Ok(PhysicalCandidate {
                        index,
                        complete: false,""")

# A source-closed probe can discard the outline-width reconnaissance after
# replaying it in the ordinary first frame with the actual derived width.
replace(running, """            let mut focal_frame = state
                .focal_frame
                .take()
                .unwrap_or_else(|| self.occurrences.as_mut().unwrap().frame());
            let focal = self.outline_execution(&mut focal_frame, content, &focal_input);
            state.focal_frame = Some(focal_frame);
            let product = focal.result.clone();
            state.executions.push(focal);""",
"""            let close_probe = input.page_store.is_empty()
                && !input.positions.is_empty()
                && std::env::var_os("TEKT_LAB_OUTLINE_ORDINALS").is_some()
                && state.executions.iter().all(|execution| execution.warnings.is_empty())
                && self.world.library().global.is_empty()
                && super::super::pdf::lab_closed_outline_source(self.source);
            let mut focal_frame = if close_probe {
                state.first_frame.take().unwrap()
            } else {
                state.focal_frame.take()
                    .unwrap_or_else(|| self.occurrences.as_mut().unwrap().frame())
            };
            let focal = self.outline_execution(&mut focal_frame, content, &focal_input);
            let product = focal.result.clone();
            if close_probe {
                state.first_frame = Some(focal_frame);
                if focal.warnings.is_empty() && product.is_ok() {
                    state.retired.append(&mut state.executions);
                }
            } else {
                state.focal_frame = Some(focal_frame);
            }
            state.executions.push(focal);""")
replace(running, """        let (index, _, _) =
            self.index_physical_steps(std::slice::from_ref(&discovery), input)?;
        if input.page_store.is_empty()""",
"""        let (index, _, _) =
            self.index_physical_steps(std::slice::from_ref(&discovery), input)?;
        if input.page_store.is_empty() && !input.positions.is_empty()
            && std::env::var_os("TEKT_LAB_OUTLINE_ORDINALS").is_some()
            && std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some()
        {
            let headings = &input.outlined_heading_locations;
            let equal = headings.iter().filter(|&&location| {
                input.positions.position_of(location).map(|p| p.page)
                    == index.positions.position_of(location).map(|p| p.page)
            }).count();
            eprintln!("lab-outline-predict validation={}/{} outline={:?}",
                equal, headings.len(), self.outline_reads_valid(&index));
        }
        if input.page_store.is_empty()""")
