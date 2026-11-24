"""Resume at the corrected outline packet using a checkpoint from projection."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-retained-break-projection-2026-11-21"
runpy.run_path(str(prior / "prototype.py"))
root = Path(sys.argv[1])
layout = root / "01_core/src/compiler/layout"
running = root / "03_infra/src/pipeline/context_stabilization/page_running.rs"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


module = layout / "mod.rs"
replace(module, "    pub(super) lab_prefix_only: bool,",
"""    pub(super) lab_capture_checkpoint: bool,
    pub(super) lab_outline_checkpoint: Option<Box<lab_retain::OutlineCheckpoint>>,""")
replace(module, "            lab_prefix_only: false,",
"""            lab_capture_checkpoint: false,
            lab_outline_checkpoint: None,""")

replace(layout / "sequence.rs", """        if layouter.lab_prefix_only && layouter.lab_outline_end.is_some() {
            break;
        }
""", "")

requests = layout / "page_running_requests.rs"
replace(requests, """        if self.lab_prefix_only && self.lab_outline_end.is_some() {
            return;
        }
        self.admit_footnote_block_frontier(content);""",
"""        if self.lab_capture_checkpoint && matches!(content, Content::Outline(_)) {
            self.lab_record_outline_checkpoint(&cursor, body_only);
        }
        self.admit_footnote_block_frontier(content);""")

retain = layout / "lab_retain.rs"
replace(retain, "use page_running_requests::{Cursor, PlannedPass, TraceSection};",
"""use page_running_requests::{Cursor, TraceSection};""")
start = "// This lays out only the path from document setup through the outline."
stop = "pub(super) fn splice"
text = retain.read_text()
before, tail = text.split(start, 1)
_, after = tail.split(stop, 1)
retain.write_text(before + (here / "checkpoint.rs").read_text() + "\n" + stop + after)
replace(retain, """    sealed: &SealedPageRunning,
) -> Result<PageRunningDocument, &'static str> {""",
"""    sealed: &SealedPageRunning,
    checkpoint: Option<Box<OutlineCheckpoint>>,
) -> Result<PageRunningDocument, &'static str> {""")
replace(retain, """    let (mut prefix, new_end) = layout_prefix(body, input, metrics, sizer, sealed).ok_or("prefix layout")?;""",
"""    let checkpoint = checkpoint.ok_or("outline checkpoint unavailable")?;
    let (mut prefix, new_end) =
        layout_from_checkpoint(checkpoint, body, input, metrics, sizer, sealed)
            .ok_or("checkpoint replay extent")?;""")

convergence = layout / "convergence.rs"
replace(convergence, "inputs, journal, false, false)", "inputs, journal, false, false, None)")
replace(convergence, """    lab_capture_outline: bool,
) -> PageRunningPassOutcome""",
"""    lab_capture_outline: bool,
    mut checkpoint_out: Option<&mut Option<Box<super::lab_retain::OutlineCheckpoint>>>,
) -> PageRunningPassOutcome""")
replace(convergence, "        l.lab_capture_outline = lab_capture_outline;",
"""        l.lab_capture_outline = lab_capture_outline;
        l.lab_capture_checkpoint = checkpoint_out.is_some();""")
replace(convergence, "        let outline_end = l.lab_outline_end.take();",
"""        let outline_end = l.lab_outline_end.take();
        if let Some(slot) = checkpoint_out.as_deref_mut() {
            *slot = l.lab_outline_checkpoint.take();
        }""")

memo = layout / "physical_memo.rs"
replace(memo, "inputs, None, true, false,", "inputs, None, true, false, None,")
replace(memo, """pub struct LabReplayWitness {
    journal:""",
"""pub struct LabReplayWitness {
    checkpoint: Option<Box<super::lab_retain::OutlineCheckpoint>>,
    journal:""")
replace(memo, """    let witness = LabReplayWitness {
        journal:""",
"""    let mut witness = LabReplayWitness {
        checkpoint: None,
        journal:""")
replace(memo, "inputs, Some(&witness.journal), false, true).into_result()?;",
        "inputs, Some(&witness.journal), false, true, Some(&mut witness.checkpoint)).into_result()?;")
replace(memo, """    metrics: M, sizer: S, sealed: &super::SealedPageRunning,
) -> Result<PageRunningDocument, &'static str> {
    super::lab_retain::splice(retained, body, input, metrics, sizer, sealed)
}""",
"""    metrics: M, sizer: S, sealed: &super::SealedPageRunning, witness: &mut LabReplayWitness,
) -> Result<PageRunningDocument, &'static str> {
    super::lab_retain::splice(retained, body, input, metrics, sizer, sealed, witness.checkpoint.take())
}""")
replace(running, "let (retained, witness) = physical_memo::lab_retain", "let (retained, mut witness) = physical_memo::lab_retain")
replace(running, """        let retained = physical_memo::lab_splice(retained, &body, &actual, self.metrics.clone(),
            ImageSizeImageSizer, &sealed)?;""",
"""        let retained = physical_memo::lab_splice(retained, &body, &actual, self.metrics.clone(),
            ImageSizeImageSizer, &sealed, &mut witness)?;""")


with requests.open("a") as output:
    output.write("""
impl PlannedPass {
    pub(super) fn lab_rebase_for_outline(
        &mut self, body: &PageRunningBody, sealed: &SealedPageRunning,
    ) -> Option<()> {
        if self.invalid || !self.consumed.is_empty() || !sealed.resolutions().is_empty()
            || self.slots.iter().any(Option::is_some)
            || !self.trace.data.requests.is_empty()
            || !self.trace.data.inactive_repeat_bodies.is_empty()
        {
            return None;
        }
        for admission in &mut self.trace.data.admissions {
            if !Arc::ptr_eq(&admission.plan, &self.source) {
                return None;
            }
            admission.plan = body.plan.clone();
        }
        self.trace.data.source = body.plan.clone();
        self.source = body.plan.clone();
        self.store = Some(sealed.clone());
        self.cursor = None;
        Some(())
    }
}
""")
