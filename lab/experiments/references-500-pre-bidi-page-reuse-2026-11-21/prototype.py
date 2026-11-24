"""Insert the pre-BiDi page experiment into the disposable source copy."""
from pathlib import Path
import sys

root = Path(sys.argv[1])
pipeline = root / "03_infra/src/pipeline"
pdf = pipeline / "pdf.rs"
paged = pipeline / "paged.rs"

def replace_once(path, before, after):
    source = path.read_text()
    assert source.count(before) == 1, (path, before)
    path.write_text(source.replace(before, after))

replace_once(pdf, "pub struct PdfCompileSession {", """pub struct PdfCompileSession {
    pub(super) pre_bidi: super::paged::pre_bidi::Session,""")
replace_once(paged, "\nuse super::*;\n", "\nuse super::*;\npub(super) mod pre_bidi;\n")
source = paged.read_text()
a = source.index("    let bidi_registry = crate::bidi_context::Registry::new(&doc);")
b = source.index("    let t5 = Instant::now();", a)
passes = source[a:b]
assert passes.count("    let doc = match ") == 3 and passes.endswith("    };\n")
source = source[:a] + """    let finish_pages = |doc: typst_core::entities::layout_types::PagedDocument| {
""" + "".join("    " + line for line in passes.splitlines(keepends=True)) + """        doc
    };
    let doc = match session {
        Some(session) => {
            let metrics_route = if ordinary_layout_metrics.is_some() {
                1
            } else if selected_layout_metrics.is_some() {
                2
            } else {
                3
            };
            session.pre_bidi.process(world, features, metrics_route, doc, finish_pages)
        }
        None => finish_pages(doc),
    };
""" + source[b:]
anchor = "    if let Err(msg) = crate::export::validate_document_images(&doc) {"
assert source.count(anchor) == 1
source = source.replace(anchor, anchor + """
        if let Some(session) = session {
            session.pre_bidi.clear();
        }""")
paged.write_text(source)
module = pipeline / "paged/pre_bidi.rs"
module.parent.mkdir(exist_ok=True)
module.write_text(Path(__file__).with_name("pre_bidi.rs").read_text())
