"""Geometry-only paint compression with unchanged native break selection."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-native-opening-estimate-2026-11-21"
runpy.run_path(str(prior / "prototype.py"))
root = Path(sys.argv[1])
layout = root / "01_core/src/compiler/layout"
running = root / "03_infra/src/pipeline/context_stabilization/page_running.rs"


def replace(path, old, new):
    source = path.read_text()
    assert source.count(old) == 1, (path, old)
    path.write_text(source.replace(old, new))


replace(layout / "mod.rs", "    pub(super) style: TextStyle,", """    pub(super) style: TextStyle,
    pub(super) lab_geometry_only: bool,
    pub(super) lab_geometry_lines: usize,
    pub(super) lab_line_right: Option<f64>,""")
replace(layout / "mod.rs", "            style: TextStyle::from(&StyleChain::default_chain()),",
"""            style: TextStyle::from(&StyleChain::default_chain()),
            lab_geometry_only: false,
            lab_geometry_lines: 0,
            lab_line_right: None,""")
replace(layout / "page_running_requests.rs", "pub(in crate::compiler) struct PageRunningTraceData {",
"""pub(in crate::compiler) struct PageRunningTraceData {
    pub(in crate::compiler) lab_geometry_lines: usize,""")
replace(layout / "page_running_requests.rs", "            data: Box::new(PageRunningTraceData {",
"""            data: Box::new(PageRunningTraceData {
                lab_geometry_lines: 0,""")

convergence = layout / "convergence.rs"
signature = """pub(super) fn run_planned<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
    journal: Option<&super::physical_memo::Journal>,
) -> PageRunningPassOutcome {"""
replace(convergence, signature, signature + """
    run_planned_lab(body, introspector, metrics, sizer, font_size, inputs, journal, false)
}

""" + signature.replace("fn run_planned<", "fn run_planned_lab<").replace(
    "    journal: Option<&super::physical_memo::Journal>,",
    "    journal: Option<&super::physical_memo::Journal>,\n    lab_geometry_only: bool,"))
replace(convergence, "let observe_prior_label_pages = attempt == 0 && journal.is_some();",
        "let observe_prior_label_pages = attempt == 0 && (journal.is_some() || lab_geometry_only);")
replace(convergence, """        l.prior_label_pages_observed =
            observe_prior_label_pages.then_some(&prior_label_pages_observed);""",
"""        l.prior_label_pages_observed =
            observe_prior_label_pages.then_some(&prior_label_pages_observed);
        l.lab_geometry_only = lab_geometry_only;""")
replace(convergence, """        let (doc, running) = l.finish_with_running();
        let mut running =
            running.expect("planned pass remains installed until terminal composition");""",
"""        let compact_lines = l.lab_geometry_lines;
        let (doc, running) = l.finish_with_running();
        let mut running =
            running.expect("planned pass remains installed until terminal composition");
        running.trace.data.lab_geometry_lines = compact_lines;""")

memo = layout / "physical_memo.rs"
replace(memo, "/// Returns a shared physical result for read-only stabilization.", """/// Lab-only projection. Its miniature document is never returned to the caller.
#[inline(never)]
pub fn lab_project<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    input: TagIntrospector,
    metrics: M,
    sizer: S,
    sealed: &super::SealedPageRunning,
) -> Result<(super::PageRunningTrace, usize), super::PageRunningRequests> {
    let inputs = PageRunningLayoutInputs { math: None, par_line: None, running: Some(sealed) };
    let product = super::convergence::run_planned_lab(
        body, input, metrics, sizer, 11.0, inputs, None, true,
    ).into_result()?;
    let compacted = product.trace.data.lab_geometry_lines;
    Ok((product.trace, compacted))
}

/// Returns a shared physical result for read-only stabilization.""")

line_commit = layout / "line_commit.rs"
replace(line_commit, """        if let Some(right) = tail_right {
            self.record_flow_width_at(right, 0.0);""",
"""        if let Some(right) = self.lab_line_right.take().or(tail_right) {
            self.record_flow_width_at(right, 0.0);""")

emit = layout / "paragraph_emit.rs"
replace(emit, """    let m = measure(p, line);
    let original_delta""",
"""    let m = measure(p, line);
    if l.lab_geometry_only
        && !justify && !continued && line.start == 0 && line.end == p.words.len()
        && !l.is_sub_frame && l.link_capture.active.is_empty()
        && l.regions.current.current_line.is_empty()
        && p.trailing.is_empty() && p.right_overhang == 0.0 && m.width > 0.0 && m.width <= width
    {
        if let Some(first) = p.words.first().and_then(|w| w.fragments.first()) {
            let style = &first.style;
            if style.tracking.is_none() && style.highlight.is_none()
                && !style.superscript && !style.subscript
                && p.words.iter().all(|w| w.fragments.iter().all(|f| f.style == *style
                        && f.replacement.is_none() && f.text.is_ascii())
                    && w.before.iter().all(|g| g.style == *style))
            {
                super::word_layout::prepared(l, "x", Pt(m.width), style, None);
                l.lab_line_right = Some(l.regions.current.cursor_x.0);
                l.lab_geometry_lines += 1;
                return None;
            }
        }
    }
    let original_delta""")

replace(running, """        let mut callbacks = LayoutCallbacks::default();
        let discovery =
            self.physical_step(content, input, &mut callbacks, false, None)?;""",
"""        let mut projected_input = None;
        if input.page_store.is_empty()
            && input.positions.is_empty()
            && !input.outlined_heading_locations.is_empty()
            && matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("projection"))
            && self.world.library().global.is_empty()
            && self.running.outline.roots.is_empty()
            && self.nodes.iter().filter(|node| node.active).all(|node|
                typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(&node.elem))
            && super::super::pdf::lab_closed_outline_source(self.source)
        {
            let source = self.outline_candidate_content(content);
            self.prepare_running_body(&source)?;
            let body = self.running.body.as_ref().unwrap().clone();
            let sealed = self.current_running_store()?;
            if sealed.resolutions().is_empty() {
                if let Ok((trace, compact_lines)) = physical_memo::lab_project(
                    &body, input.as_ref().clone(), self.metrics.clone(), ImageSizeImageSizer,
                    &sealed,
                ) {
                    let step = PhysicalStep { body, target: EvalTarget::Paged,
                        running: Some((trace, sealed)), document: None, warnings: Vec::new() };
                    if let Ok((index, _, warnings)) =
                        self.index_physical_steps(std::slice::from_ref(&step), input)
                    {
                        let ordinals = input.outlined_heading_locations.iter().map(|&location| {
                            let page = index.positions.position_of(location)?.page;
                            Some((location, typst_core::entities::position::Position {
                                page, point: typst_core::entities::layout_types::Point {
                                    x: typst_core::entities::layout_types::Pt(0.0),
                                    y: typst_core::entities::layout_types::Pt(0.0),
                                },
                            }))
                        }).collect::<Option<HashMap<_, _>>>();
                        if let Some(positions) = ordinals.filter(|_| warnings.is_empty()) {
                            if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                                let pages: Vec<_> = input.outlined_heading_locations.iter()
                                    .map(|loc| positions[loc].page.get()).collect();
                                eprintln!("lab-break-projection compact-lines={} {:?}", compact_lines, pages);
                            }
                            let mut provisional = input.as_ref().clone();
                            provisional.inject_positions(
                                typst_core::entities::sealed_positions::SealedPositions::from_runtime(positions));
                            let provisional = Arc::new(provisional);
                            self.prepare_outline_candidate(content, &provisional)?;
                            projected_input = Some(provisional);
                        }
                    }
                }
            }
        }
        let input = projected_input.as_ref().unwrap_or(input);
        let mut callbacks = LayoutCallbacks::default();
        let discovery =
            self.physical_step(content, input, &mut callbacks, false, None)?;""")
