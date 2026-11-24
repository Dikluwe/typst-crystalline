from pathlib import Path
import sys,re,hashlib,json,datetime
s=Path(sys.argv[1]); p=s/'03_infra/src/pipeline.rs'
t=p.read_text()
a=t.index('fn stable_heading_substitution_body('); b=t.index('\nfn substitute_context_blocks(',a)
proof=t[a:b].replace('stable_heading_substitution_body','lab_substitution_inert').replace('depth > 8 || *nodes >= 128','depth > 32 || *nodes >= 512')
# Same budget discipline, with wrappers whose exact substitution semantics have been inspected.
variants=['Source','Located','Realized','ShowRecipe','Styled']
for v in variants:
    proof=proof.replace(f'            Content::{v}(..)\n', '').replace(f'            | Content::{v}(..)\n','')
proof=proof.replace('            | Content::Symbol(..)','            Content::Symbol(..)',1)
anchor='            Content::Strong(element) => visit(&element.body, depth + 1, nodes, payload),'
assert anchor in proof
proof=proof.replace(anchor,'''            Content::Source(e) => visit(&e.body, depth + 1, nodes, payload),
            Content::Located(e) => visit(&e.body, depth + 1, nodes, payload),
            Content::Realized(e) => visit(&e.visual, depth + 1, nodes, payload),
            Content::ShowRecipe(e) => visit(&e.body, depth + 1, nodes, payload),
            Content::Styled(e) => {
                typst_core::compiler::realize::reference_supplements::prepared_reference_supplement(content).is_none()
                    && visit(&e.body, depth + 1, nodes, payload)
            }
'''+anchor)
proof=proof.replace('            | Content::Heading(..)\n','').replace('            Content::Symbol(..)','            Content::Heading(..)\n            | Content::Symbol(..)',1)
# Keep return on its own symbol for a separate debugger census, no counter in nominal execution.
proof+='\n#[inline(never)]\nfn lab_preserve_substitution(content: Content) -> Content { content }\n'
t=t[:b]+'\n'+proof+t[b:]
anchor='''    macro_rules! body {
        ($variant:ident, $elem:expr) => {{'''
assert t.count(anchor)==1
t=t.replace(anchor,'''    if lab_substitution_inert(&content) {
        return lab_preserve_substitution(content);
    }
'''+anchor)
p.write_text(t)
