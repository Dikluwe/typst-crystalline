"""Keep the validated, corrected product in the ordinary physical memo."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here / "prototype-unretained.py"))
root = Path(sys.argv[1])
memo = root / "01_core/src/compiler/layout/physical_memo.rs"
pipeline = root / "03_infra/src/pipeline"
running = pipeline / "context_stabilization/page_running.rs"


def replace(path, old, new):
    source = path.read_text()
    assert source.count(old) == 1, (path, old)
    path.write_text(source.replace(old, new))


replace(memo, """#[inline(never)]
pub fn lab_retain<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody, input: TagIntrospector, metrics: M, sizer: S,
    sealed: &super::SealedPageRunning,
) -> Result<PageRunningDocument, super::PageRunningRequests> {
    let inputs = PageRunningLayoutInputs { math: None, par_line: None, running: Some(sealed) };
    super::convergence::run_planned_lab(body, input, metrics, sizer, 11.0,
        inputs, None, false, true).into_result()
}""", """pub struct LabReplayWitness {
    journal: Journal,
    epoch: Option<MetricEpoch>,
    pages: PageStore,
}

#[inline(never)]
pub fn lab_retain<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody, input: TagIntrospector, metrics: M, sizer: S,
    sealed: &super::SealedPageRunning,
) -> Result<(PageRunningDocument, LabReplayWitness), super::PageRunningRequests> {
    let inputs = PageRunningLayoutInputs { math: None, par_line: None, running: Some(sealed) };
    let witness = LabReplayWitness {
        journal: Journal::default(), epoch: metrics.physical_epoch(), pages: input.page_store.clone(),
    };
    let recorded = providers::RecordingMetrics {
        inner: metrics, journal: witness.journal.clone(), immutable: witness.epoch.is_some(),
    };
    let images = providers::RecordingSizer { inner: sizer, journal: witness.journal.clone() };
    let product = super::convergence::run_planned_lab(body, input, recorded, images, 11.0,
        inputs, Some(&witness.journal), false, true).into_result()?;
    Ok((product, witness))
}

// The prefix has already been laid out with the actual input. The remaining
// frames were laid out under the recorded reads, revalidated here.
pub fn lab_remember(
    witness: LabReplayWitness, body: &PageRunningBody, index: &TagIntrospector,
    metrics: &dyn FontMetrics, product: PageRunningDocument, session: &PhysicalLayoutSession,
) -> Result<Arc<PageRunningDocument>, &'static str> {
    if !same_epoch(&witness.epoch, &metrics.physical_epoch())
        || !witness.journal.page_inputs_match(&witness.pages, &index.page_store)
        || !witness.journal.validates(metrics, index)
        || !same_epoch(&witness.epoch, &metrics.physical_epoch())
        || !document_bounded(&product.document)
    {
        return Err("recorded physical reads or output limits");
    }
    let product = Arc::new(product);
    session.0.lock().unwrap().entry = Some(Entry {
        epoch: witness.epoch, source: body.plan.clone(), pages: index.page_store.clone(),
        font_size: 11.0_f64.to_bits(), running: true,
        journal: witness.journal, product: Arc::clone(&product),
    });
    Ok(product)
}""")
replace(running, """        let retained = physical_memo::lab_retain(&body, input.as_ref().clone(),""",
        """        let (retained, witness) = physical_memo::lab_retain(&body, input.as_ref().clone(),""")
replace(running, """        let step = PhysicalStep { body, target: EvalTarget::Paged,
            running: Some((retained.trace, sealed)),
            document: None, warnings: Vec::new() };
        let (index, _, warnings) = self.index_physical_steps(std::slice::from_ref(&step), input)
            .map_err(|_| "corrected index")?;""",
"""        let mut step = PhysicalStep { body, target: EvalTarget::Paged,
            running: Some((retained.trace, sealed)),
            document: None, warnings: Vec::new() };
        let (index, _, warnings) = self.index_physical_steps(std::slice::from_ref(&step), input)
            .map_err(|_| "corrected index")?;""")
replace(running, """        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
            eprintln!("lab-retain accepted");
        }
        self.running.lab_replay_input = Some(index.clone());
        Ok(PhysicalCandidate { index, document: Some(PhysicalDocument::Owned(retained.document)),
            complete: true, warnings })""",
"""        let (trace, _) = step.running.take().unwrap();
        let retained = PageRunningDocument { document: retained.document, trace };
        let product = physical_memo::lab_remember(witness, &step.body, &index,
            &self.metrics, retained, &self.running.physical)?;
        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
            eprintln!("lab-retain accepted");
        }
        self.running.lab_replay_input = Some(index.clone());
        Ok(PhysicalCandidate { index, document: Some(PhysicalDocument::Shared(product)),
            complete: true, warnings })""")
