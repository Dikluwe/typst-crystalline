"""Apply the probe to a disposable copy of the current product sources."""
from pathlib import Path
import shutil
import sys

here = Path(__file__).resolve().parent
root = Path(sys.argv[1])
pipeline = root / "03_infra/src/pipeline"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


guard = pipeline / "context_stabilization.rs"
replace(guard, "struct LegacyWorldGuard<'a> {", "pub(super) struct LegacyWorldGuard<'a> {\n    watch_probe: bool,")
replace(guard, "Self { inner, source: None,", "Self { watch_probe: false, inner, source: None,")
replace(guard, "    fn observed_source(&self)", """    pub(super) fn for_watch_probe(inner: &'a dyn World, source: &'a Source) -> Self {
        Self { watch_probe: true, ..Self::with_source(inner, source) }
    }
    fn observed_source(&self)""")
replace(guard, "    fn sealed(&self) -> bool", "    pub(super) fn sealed(&self) -> bool")
replace(guard, "Option<Source> { self.inner.immutable_source(id) }", """Option<Source> {
        if self.watch_probe && self.source.is_none_or(|source| source.id() != id) {
            self.poison();
        }
        self.inner.immutable_source(id)
    }""")
replace(guard, "SysInputs { self.poison(); self.inner.inputs() }", """SysInputs {
        if !self.watch_probe { self.poison(); }
        self.inner.inputs()
    }""")

pdf = pipeline / "pdf.rs"
replace(pdf, "    let (doc_result, mut warnings) = super::paged::compile_paged_in_session(\n        world, source, full_error, features, timings, session,\n    );\n    let doc_result = doc_result.map(|compiled| compiled.document);", """    let (doc_result, mut warnings) =
        watch_probe_paged(world, source, full_error, features, timings, session);""")
replace(pdf, "    // **P1468/P1469** — resolve `pages`", """    if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
        let mut labels = doc.extracted_label_pages.iter().collect::<Vec<_>>();
        labels.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));
        let coordinates: Vec<_> = labels.iter().map(|(label, page)| {
            (label.0.as_str(), **page, doc.extracted_label_positions.get(label))
        }).collect();
        eprintln!("watch-positions {} {:?}", doc.extracted_positions.len(), coordinates);
    }
    // **P1468/P1469** — resolve `pages`""")
replace(pdf, "pub struct PdfCompileSession {", """pub struct PdfCompileSession {
    watch_probe: std::sync::Mutex<Option<WatchProbeEntry>>,""")
shutil.copyfile(here / "references_500.typ", pipeline / "watch_probe_fixture.typ")
replace(pdf, "struct PdfCompileRequest<'a> {", (here / "watch_probe.inc.rs").read_text() + "\nstruct PdfCompileRequest<'a> {")

# Count memo outcomes and generations in L3 without adding I/O to L1.
running = pipeline / "context_stabilization/page_running.rs"
replace(running, "            let outcome = if !replay && self.bundle_html_targets.is_none() {", """            let trace_probe = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
            let before = trace_probe.then(|| self.running.physical.stats());
            let outcome = if !replay && self.bundle_html_targets.is_none() {""")
replace(running, "            match &outcome {", """            if let Some(before) = before {
                let after = self.running.physical.stats();
                eprintln!("watch-memo gen={} replay={} hits={} builds={} refusals={} {:?}",
                    self.running.generation, replay, after.0 - before.0,
                    after.1 - before.1, after.2 - before.2, self.running.physical.refusals());
            }
            match &outcome {""")
