"""Add the memo experiment only to the disposable stable-CID source copy."""
from pathlib import Path
import runpy
import sys

d = Path(__file__).resolve().parent
runpy.run_path(str(d.parent/"references-500-visible-stable-cids-2026-11-21/prototype.py"))
root = Path(sys.argv[1])
core = root/"01_core/src/compiler/eval/subtree_session.rs"
infra = root/"03_infra/src/pipeline.rs"

text = core.read_text().replace("const MAX_ENTRIES: usize = 16;\n", "")
a, b = text.index("struct Entry {"), text.index("fn spans(")
text = text[:a]+(d/"memo.inc.rs").read_text()+"\n"+text[b:]
anchor = "    pub(super) fn bind(&self, ctx: &mut EvalContext) {"
assert text.count(anchor) == 1
text = text.replace(anchor, """    pub fn lab_begin(&self, preserve: bool, budget: usize) {
        self.memo.borrow_mut().begin(preserve, budget);
    }

    pub fn lab_finish(&self) -> LabReport {
        let mut memo = self.memo.borrow_mut();
        memo.finish();
        memo.report
    }

"""+anchor)
assert text.count("assert_eq!(session.stats().2, MAX_ENTRIES);") == 1
text = text.replace("assert_eq!(session.stats().2, MAX_ENTRIES);",
                    "assert_eq!(session.stats().2, 40);")
core.write_text(text)

text = infra.read_text()
anchor = "        session.evaluate(world, source, full_error, target, features)"
assert text.count(anchor) == 1
text = text.replace(anchor, """        let protect = std::env::var_os("TEKT_LAB_MEMO_PROTECT").is_some();
        let budget = std::env::var("TEKT_LAB_MEMO_BYTES").ok()
            .and_then(|text| text.parse().ok()).unwrap_or(8 * 1024 * 1024);
        session.lab_begin(protect, budget);
        let result = session.evaluate(world, source, full_error, target, features);
        let stats = session.lab_finish();
        if std::env::var_os("TEKT_LAB_MEMO_TRACE").is_some() {
            eprintln!("lab-eval-memo {stats:?}");
        }
        result""")
infra.write_text(text)
