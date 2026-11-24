"""Install a gated omission probe into the prior PDF reuse lab candidate."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
root = Path(sys.argv[1])
base = here.parent / "references-500-pdf-export-reuse-2026-11-20"
runpy.run_path(str(base / "prototype.py"))
pipeline = root / "03_infra/src/pipeline"
pdf = pipeline / "pdf.rs"
running = pipeline / "context_stabilization/page_running.rs"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


replace(pdf, "fn inert_source(source: &Source) -> bool {",
        """pub(super) fn lab_closed_outline_source(source: &Source) -> bool {
    if !inert_source(source) {
        return false;
    }
    let (prefix, _) = watch_probe_tail(source);
    let mut outlines = 0;
    for (index, node) in prefix.iter().enumerate() {
        if inert_call(node, "outline").is_some() {
            // The one read of heading metadata occurs before body material.
            if prefix[..index].iter().any(|part| matches!(part.kind(),
                SyntaxKind::Heading | SyntaxKind::Ref | SyntaxKind::Text | SyntaxKind::ForLoop))
            {
                return false;
            }
            outlines += 1;
        }
    }
    outlines == 1
}

fn inert_source(source: &Source) -> bool {""")
replace(running, """        let replay_input = Arc::new(index);
        let step =
            self.physical_step(content, &replay_input, &mut callbacks, true, None)?;""",
        """        if input.page_store.is_empty()
            && std::env::var_os("TEKT_LAB_SKIP_PROVISIONAL_REPLAY").is_some()
            && self.world.library().global.is_empty()
            && callbacks.passes == 1
            && callbacks.closed
            && callbacks.math.is_none()
            && callbacks.par_line.is_none()
            && callbacks.warnings.is_empty()
            && discovery.running.as_ref().is_some_and(|(_, sealed)| sealed.resolutions().is_empty())
            && super::super::pdf::lab_closed_outline_source(self.source)
            && matches!(self.outline_reads_valid(&index), Ok((false, true)))
        {
            if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
                eprintln!("lab-provisional-replay skipped");
            }
            return Ok(PhysicalCandidate {
                index,
                complete: false,
                document: None,
                warnings: Vec::new(),
            });
        }
        let replay_input = Arc::new(index);
        let step =
            self.physical_step(content, &replay_input, &mut callbacks, true, None)?;""")
