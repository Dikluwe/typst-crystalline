"""Retain full frames and run the planner only over the corrected outline prefix."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-real-break-projection-2026-11-21"
runpy.run_path(str(prior / "prototype.py"))
root = Path(sys.argv[1])
layout = root / "01_core/src/compiler/layout"
pipeline = root / "03_infra/src/pipeline"
running = pipeline / "context_stabilization/page_running.rs"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


(layout / "lab_retain.rs").write_text((here / "lab_retain.rs").read_text())
replace(layout/"mod.rs", "pub mod vanilla_defaults;", "pub mod vanilla_defaults;\nmod lab_retain;")
replace(layout/"mod.rs", "    pub(super) lab_geometry_only: bool,",
"""    pub(super) lab_geometry_only: bool,
    pub(super) lab_capture_outline: bool,
    pub(super) lab_prefix_only: bool,
    pub(super) lab_outline_end: Option<lab_retain::FlowEnd>,""")
replace(layout/"mod.rs", "            lab_geometry_only: false,",
"""            lab_geometry_only: false,
            lab_capture_outline: false,
            lab_prefix_only: false,
            lab_outline_end: None,""")
replace(layout/"outline.rs", """        layouter.block_chain_active = true;
        return;
    }
    // P200B""",
"""        layouter.block_chain_active = true;
        layouter.lab_record_outline_end();
        return;
    }
    // P200B""")
replace(layout/"sequence.rs", """    while let Some(part) = iter.next() {
        let tail""",
"""    while let Some(part) = iter.next() {
        if layouter.lab_prefix_only && layouter.lab_outline_end.is_some() {
            break;
        }
        let tail""")
requests = layout/"page_running_requests.rs"
replace(requests, "    pub(in crate::compiler) lab_geometry_lines: usize,",
"""    pub(in crate::compiler) lab_geometry_lines: usize,
    pub(in crate::compiler) lab_outline_end: Option<super::lab_retain::FlowEnd>,""")
replace(requests, "                lab_geometry_lines: 0,", "                lab_geometry_lines: 0,\n                lab_outline_end: None,")
replace(requests, """        self.admit_footnote_block_frontier(content);
        let note_atomic""",
"""        if self.lab_prefix_only && self.lab_outline_end.is_some() {
            return;
        }
        self.admit_footnote_block_frontier(content);
        let note_atomic""")
with requests.open("a") as out:
    out.write("""
impl PageRunningTrace {
    pub(super) fn lab_rebind_prefix(&mut self, body: &PageRunningBody) -> Result<(), &'static str> {
        let old = &self.data.source;
        let new = &body.plan;
        if !self.data.complete || !self.data.requests.is_empty()
            || old.root != new.root || old.allocation_start != new.allocation_start
            || old.document_boundary != new.document_boundary
            || !matches!((&old.allocation_owner, &new.allocation_owner),
                (Some(a), Some(b)) if Arc::ptr_eq(a,b))
            || old.environments.len() != new.environments.len()
            || old.nodes.len() != new.nodes.len()
            || !old.environments.iter().zip(&new.environments).all(|(a,b)| a.parent == b.parent && a.origin == b.origin)
            || !old.nodes.iter().zip(&new.nodes).all(|(a,b)|
                a.children == b.children && a.source_parent == b.source_parent && a.location == b.location
                && a.auxiliary_locations == b.auxiliary_locations && a.role == b.role
                && a.environment == b.environment && a.span == b.span && a.label == b.label
                && a.lang == b.lang && a.conditional_ref == b.conditional_ref && a.original == b.original
                && a.math == b.math && std::mem::discriminant(&a.content) == std::mem::discriminant(&b.content))
            || self.data.admissions.iter().any(|event| !Arc::ptr_eq(&event.plan, old))
            || self.data.inactive_repeat_bodies.iter().any(|proof| !Arc::ptr_eq(&proof.plan, old))
        {
            return Err("plan topology and locations");
        }
        for admission in &mut self.data.admissions {
            admission.plan = new.clone();
        }
        for proof in &mut self.data.inactive_repeat_bodies {
            proof.plan = new.clone();
        }
        self.data.source = new.clone();
        self.repeat_activity_valid().then_some(()).ok_or("repeat activity")
    }
}
""")
sealed = root/"01_core/src/entities/sealed_positions.rs"
replace(sealed, """    pub fn is_empty(&self) -> bool {""",
"""    pub fn lab_exact_subset(&self, other: &Self) -> bool {
        self.positions.iter().all(|(location, position)| other.positions.get(location) == Some(position))
    }

    pub fn is_empty(&self) -> bool {""")
conv = layout/"convergence.rs"
replace(conv, "inputs, journal, false)","inputs, journal, false, false)")
replace(conv, "    lab_geometry_only: bool,","    lab_geometry_only: bool,\n    lab_capture_outline: bool,")
replace(conv, "(journal.is_some() || lab_geometry_only)", "(journal.is_some() || lab_geometry_only || lab_capture_outline)")
replace(conv, "        l.lab_geometry_only = lab_geometry_only;", "        l.lab_geometry_only = lab_geometry_only;\n        l.lab_capture_outline = lab_capture_outline;")
replace(conv, "        let compact_lines = l.lab_geometry_lines;", "        let compact_lines = l.lab_geometry_lines;\n        let outline_end = l.lab_outline_end.take();")
replace(conv, "        running.trace.data.lab_geometry_lines = compact_lines;", """        running.trace.data.lab_geometry_lines = compact_lines;
        running.trace.data.lab_outline_end = outline_end;""")
memo = layout/"physical_memo.rs"
replace(memo, "inputs, None, true,", "inputs, None, true, false,")
replace(memo, "/// Returns a shared physical result for read-only stabilization.", """#[inline(never)]
pub fn lab_retain<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody, input: TagIntrospector, metrics: M, sizer: S,
    sealed: &super::SealedPageRunning,
) -> Result<PageRunningDocument, super::PageRunningRequests> {
    let inputs = PageRunningLayoutInputs { math: None, par_line: None, running: Some(sealed) };
    super::convergence::run_planned_lab(body, input, metrics, sizer, 11.0,
        inputs, None, false, true).into_result()
}

pub fn lab_splice<M: FontMetrics, S: ImageSizer>(
    retained: PageRunningDocument, body: &PageRunningBody, input: &TagIntrospector,
    metrics: M, sizer: S, sealed: &super::SealedPageRunning,
) -> Result<PageRunningDocument, &'static str> {
    super::lab_retain::splice(retained, body, input, metrics, sizer, sealed)
}

/// Returns a shared physical result for read-only stabilization.""")
replace(running, "    physical: PhysicalLayoutSession,", """    physical: PhysicalLayoutSession,
    pub(super) lab_replay_input: Option<TagIntrospector>,""")
replace(running, """        self.prepare_outline_candidate(content, input)?;
        if input.page_store.is_empty()""",
"""        self.running.lab_replay_input = None;
        self.prepare_outline_candidate(content, input)?;
        if input.page_store.is_empty() && input.positions.is_empty()
            && !input.outlined_heading_locations.is_empty()
            && matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain"))
            && self.world.library().global.is_empty() && self.running.outline.roots.is_empty()
            && self.nodes.iter().filter(|node| node.active).all(|node|
                typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(&node.elem))
            && super::super::pdf::lab_closed_outline_source(self.source)
        {
            match self.lab_try_retain(content, input) {
                Ok(candidate) => return Ok(candidate),
                Err(reason) => {
                    if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                        eprintln!("lab-retain refused: {}", reason);
                    }
                    self.prepare_outline_candidate(content, input)?;
                }
            }
        }
        if input.page_store.is_empty()""")
replace(running, "    fn physical_bundle_candidate(", """    fn lab_try_retain(
        &mut self, content: &Content, input: &Arc<TagIntrospector>,
    ) -> Result<PhysicalCandidate, &'static str> {
        let source = self.outline_candidate_content(content);
        self.prepare_running_body(&source).map_err(|_| "provisional source plan")?;
        let body = self.running.body.as_ref().unwrap().clone();
        let sealed = self.current_running_store().map_err(|_| "running store")?;
        if !sealed.resolutions().is_empty() {
            return Err("running content");
        }
        let retained = physical_memo::lab_retain(&body, input.as_ref().clone(),
            self.metrics.clone(), ImageSizeImageSizer, &sealed).map_err(|_| "retained layout")?;
        let mut step = PhysicalStep { body, target: EvalTarget::Paged,
            running: Some((retained.trace, sealed)),
            document: None, warnings: Vec::new() };
        let (index, _, warnings) = self.index_physical_steps(std::slice::from_ref(&step), input)
            .map_err(|_| "retained index")?;
        if !warnings.is_empty() { return Err("retained index warnings"); }
        let (trace, sealed) = step.running.take().unwrap();
        let retained = PageRunningDocument { document: retained.document, trace };
        let actual = Arc::new(index);
        self.prepare_outline_candidate(content, &actual).map_err(|_| "corrected outline")?;
        if !self.running.outline.roots.is_empty() { return Err("corrected outline roots"); }
        let source = self.outline_candidate_content(content);
        self.prepare_running_body(&source).map_err(|_| "corrected source plan")?;
        let body = self.running.body.as_ref().unwrap().clone();
        let retained = physical_memo::lab_splice(retained, &body, &actual, self.metrics.clone(),
            ImageSizeImageSizer, &sealed)?;
        let step = PhysicalStep { body, target: EvalTarget::Paged,
            running: Some((retained.trace, sealed)),
            document: None, warnings: Vec::new() };
        let (index, _, warnings) = self.index_physical_steps(std::slice::from_ref(&step), input)
            .map_err(|_| "corrected index")?;
        if !warnings.is_empty() || !matches!(self.outline_reads_valid(&index), Ok((true,true))) {
            return Err("corrected index reads");
        }
        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
            eprintln!("lab-retain accepted");
        }
        self.running.lab_replay_input = Some(index.clone());
        Ok(PhysicalCandidate { index, document: Some(PhysicalDocument::Owned(retained.document)),
            complete: true, warnings })
    }

    fn physical_bundle_candidate(""")
replace(pipeline/"context_stabilization.rs", """                    pagination_stability(input, &candidate)
                } else {""",
"""                    pagination_stability(self.running.lab_replay_input.as_ref().unwrap_or(input.as_ref()), &candidate)
                } else {""")
replace(running, """            let close_probe = input.page_store.is_empty()
                && !input.positions.is_empty()""",
"""            let close_probe = input.page_store.is_empty()
                && (!input.positions.is_empty()
                    || matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain")))""")
