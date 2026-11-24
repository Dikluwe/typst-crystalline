"""Compile the exact page-transition predicate with an independent matrix."""
from pathlib import Path
import re
import subprocess
import tempfile

here = Path(__file__).resolve().parent
file = here / "source/01_core/src/compiler/layout/paragraph_emit.rs"
code = re.search(r"fn lab_suspend_emitter\(.*?\n}", file.read_text(), re.S).group()
tests = """
#[test]
fn commits_one_page_and_only_at_the_last_line() {
    for page in [1, 2, 3, 9, 44] {
        assert!(lab_suspend_emitter(true, page - 1, page, 2, 3, true));
        assert_eq!(lab_suspend_emitter(true, page - 1, page, 2, 3, false), page == 2);
        assert!(!lab_suspend_emitter(true, page, page, 2, 3, true));
        assert!(!lab_suspend_emitter(true, page - 1, page, 1, 3, true));
        assert!(!lab_suspend_emitter(false, page - 1, page, 2, 3, true));
    }
    assert!(!lab_suspend_emitter(true, 4, 6, 0, 1, true));
    assert!(!lab_suspend_emitter(true, usize::MAX, 0, 0, 1, true));
}
"""
with tempfile.TemporaryDirectory(prefix="adaptive-predicate-") as tmp:
    p = Path(tmp)
    (p / "boundary.rs").write_text(code + tests)
    subprocess.run(["rustc", "--test", str(p / "boundary.rs"), "-o", str(p / "test")], check=True)
    subprocess.run([str(p / "test")], check=True)
