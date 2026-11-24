"""Install a source-order first index into the three-traversal lab source."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
prior = here.parent / "references-500-opening-physical-2026-11-20"
runpy.run_path(str(prior / "prototype.py"))
pipeline = Path(sys.argv[1]) / "03_infra/src/pipeline"
running = pipeline / "context_stabilization/page_running.rs"

old = """        self.prepare_outline_candidate(content, input)?;
        let mut callbacks = LayoutCallbacks::default();
        let discovery ="""
new = """        self.prepare_outline_candidate(content, input)?;
        if input.page_store.is_empty()
            && input.headings_for_toc.is_empty()
            && std::env::var_os("TEKT_LAB_SOURCE_ONLY_FIRST_INDEX").is_some()
            && std::env::var_os("TEKT_LAB_SKIP_PROVISIONAL_REPLAY").is_some()
            && self.world.library().global.is_empty()
            && self.nodes.iter().filter(|node| node.active).all(|node|
                typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(&node.elem))
            && self.running.outline.roots.is_empty()
            && super::super::pdf::lab_closed_outline_source(self.source)
        {
            let source = self.outline_candidate_content(content);
            self.prepare_running_body(&source)?;
            // Reuse the source-only scheduler for a provisional index. Runtime
            // remains paged and this index deliberately has no page geometry.
            let provisional = PhysicalStep {
                body: self.running.body.as_ref().unwrap().clone(),
                target: EvalTarget::Html,
                running: None,
                document: None,
                warnings: Vec::new(),
            };
            if let Ok((index, _, warnings)) =
                self.index_physical_steps(std::slice::from_ref(&provisional), input)
            {
                if warnings.is_empty()
                    && !index.headings_for_toc.is_empty()
                    && matches!(self.outline_reads_valid(&index), Ok((false, true)))
                {
                    if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                        eprintln!("lab-source-only-first-index headings={}", index.headings_for_toc.len());
                    }
                    return Ok(PhysicalCandidate {
                        index,
                        complete: false,
                        document: None,
                        warnings,
                    });
                }
            }
        }
        let mut callbacks = LayoutCallbacks::default();
        let discovery ="""
source = running.read_text()
assert source.count(old) == 1
running.write_text(source.replace(old, new))

# Sort the complete positions map in the Python observer without exposing a new
# L1 method or introducing work in the nominal measurement.
pdf = pipeline / "pdf.rs"
source = pdf.read_text()
anchor = '        eprintln!("watch-positions {} {:?}", doc.extracted_positions.len(), coordinates);'
assert source.count(anchor) == 1
pdf.write_text(source.replace(anchor, anchor +
    '\n        eprintln!("lab-sealed-positions {:?}", doc.extracted_positions);'))
