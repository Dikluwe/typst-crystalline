"""Apply an isolated admission experiment only to the tmpfs checkout."""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
if not str(root).startswith("/dev/shm/"):
    raise ValueError("This experiment requires the RAM checkout.")
d = Path(__file__).resolve().parent

def patch(file, before, after):
    path = root / file
    text = path.read_text()
    assert text.count(before) == 1, (file, before)
    path.write_text(text.replace(before, after, 1))

patch("01_core/src/compiler/realize/reference_supplements.rs",
      "fn token(\n", (d / "proof.rs").read_text() + "\nfn token(\n")
patch("01_core/src/compiler/eval/mod.rs",
      """                    && crate::compiler::realize::reference_supplements::needs_preparation(
                        &content,
                    ))""",
      """                    && crate::compiler::realize::reference_supplements::needs_preparation(
                        &content,
                    )
                    && !crate::compiler::realize::reference_supplements::only_numbered_heading_targets(
                        &content,
                    ))""")
patch("03_infra/src/pipeline/paged.rs",
      "    let content = match module.content() {",
      """    if let Ok(mode) = std::env::var("TEKT_LAB_HEADING_REF_TRACE") {
        eprintln!("lab-heading-ref initial={}", module.realization_introspector().is_some());
        if mode == "tree" {
            eprintln!("lab-heading-ref tree={:?}", module.introspection_content());
        }
    }
    let content = match module.content() {""")
