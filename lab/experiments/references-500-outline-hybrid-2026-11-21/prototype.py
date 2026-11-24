"""Validate native ordinals against retained physical geometry before publishing."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-outline-checkpoint-2026-11-21"
runpy.run_path(str(prior / "prototype.py"))
root = Path(sys.argv[1])
layout = root / "01_core/src/compiler/layout"
running = root / "03_infra/src/pipeline/context_stabilization/page_running.rs"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


replace(running,
    '|| matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain")))',
    '|| matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain" | "hybrid")))')

replace(running, """        if input.page_store.is_empty() && input.positions.is_empty()
            && !input.outlined_heading_locations.is_empty()
            && matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain"))""",
"""        let hybrid = matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("hybrid"));
        if input.page_store.is_empty()
            && !input.outlined_heading_locations.is_empty()
            && (hybrid || (input.positions.is_empty()
                && matches!(std::env::var("TEKT_LAB_OUTLINE_ORDINALS").as_deref(), Ok("retain"))))""")
replace(running, "match self.lab_try_retain(content, input)", "match self.lab_try_retain(content, input, hybrid)")
replace(running, '''                            "metrics" => super::super::pdf::lab_predict_outline_ordinals(''',
                 '''                            "metrics" | "hybrid" => super::super::pdf::lab_predict_outline_ordinals(''')
replace(running, """        &mut self, content: &Content, input: &Arc<TagIntrospector>,
    ) -> Result<PhysicalCandidate, &'static str> {""",
"""        &mut self, content: &Content, input: &Arc<TagIntrospector>, hybrid: bool,
    ) -> Result<PhysicalCandidate, &'static str> {""")
replace(running, "let (retained, mut witness) = physical_memo::lab_retain", "let (mut retained, mut witness) = physical_memo::lab_retain")
replace(running, """        let (trace, sealed) = step.running.take().unwrap();
        let retained = PageRunningDocument { document: retained.document, trace };
        let actual = Arc::new(index);""",
"""        let (trace, sealed) = step.running.take().unwrap();
        // The estimator is only a hypothesis. Each guessed ordinal must exist
        // in the freshly laid-out index and all outline reads must be stable.
        let proved_native = hybrid && input.outlined_heading_locations.iter().all(|&location| {
            input.positions.position_of(location).is_some_and(|predicted| {
                index.positions.position_of(location).is_some_and(|actual| actual.page == predicted.page)
            })
        }) && matches!(self.outline_reads_valid(&index), Ok((true, true)));
        if proved_native && physical_memo::lab_finish_native(&mut retained.document) {
            let product = physical_memo::lab_remember(witness, &step.body, &index,
                &self.metrics, PageRunningDocument { document: retained.document, trace },
                &self.running.physical)?;
            if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                eprintln!("lab-hybrid native");
            }
            self.running.lab_replay_input = Some(index.clone());
            return Ok(PhysicalCandidate { index, document: Some(PhysicalDocument::Shared(product)),
                complete: true, warnings });
        }
        if hybrid && std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
            eprintln!("lab-hybrid checkpoint attempt");
        }
        let retained = PageRunningDocument { document: retained.document, trace };
        let actual = Arc::new(index);""")
replace(layout / "physical_memo.rs", "pub fn lab_splice<M:",
"""#[inline(never)]
pub fn lab_finish_native(doc: &mut PagedDocument) -> bool {
    super::lab_retain::remove_native_marker(doc)
}

pub fn lab_splice<M:""")
replace(layout / "lab_retain.rs", "pub(super) fn splice<M:",
"""pub(super) fn remove_native_marker(doc: &mut PagedDocument) -> bool {
    let Some(first) = doc.pages.first_mut() else { return false };
    let Some(marker) = prefix_end(first) else { return false };
    first.items.remove(marker);
    true
}

pub(super) fn splice<M:""")
