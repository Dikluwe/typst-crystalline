"""Retain initial realization while disabling only certified Ref producers."""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
if not str(root).startswith("/dev/shm/"):
    raise ValueError("Only a RAM checkout may be instrumented.")


def edit(file, before, after):
    path = root / file
    text = path.read_text()
    assert text.count(before) == 1, (file, before)
    path.write_text(text.replace(before, after, 1))


evaluate = "01_core/src/compiler/eval/mod.rs"
realize = "01_core/src/compiler/realize/mod.rs"
refs = "01_core/src/compiler/realize/reference_supplements.rs"
paged = "03_infra/src/pipeline/paged.rs"

edit(evaluate, "    subtree_memo: Option<std::rc::Rc<RefCell<subtree_session::Memo>>>,",
     """    subtree_memo: Option<std::rc::Rc<RefCell<subtree_session::Memo>>>,
    pub(in crate::compiler) lab_no_heading_ref_producers: bool,""")
edit(evaluate, "            subtree_memo: None,", """            subtree_memo: None,
            lab_no_heading_ref_producers: false,""")

old = """            if has_recipes
                || crate::compiler::realize::bare_math::needs_preparation(&content)
                || crate::compiler::realize::figure::needs_preparation(&content)
                || crate::compiler::realize::page_running::needs_capture(&content)
                || (target == EvalTarget::Paged
                    && crate::compiler::realize::reference_supplements::needs_preparation(
                        &content,
                    )
                    && !crate::compiler::realize::reference_supplements::only_numbered_heading_targets(
                        &content,
                    ))
            {"""
new = """            let other_preparation = has_recipes
                || crate::compiler::realize::bare_math::needs_preparation(&content)
                || crate::compiler::realize::figure::needs_preparation(&content)
                || crate::compiler::realize::page_running::needs_capture(&content);
            let ref_preparation = !other_preparation && target == EvalTarget::Paged
                && crate::compiler::realize::reference_supplements::needs_preparation(&content);
            if other_preparation || ref_preparation {
                let lab_no_heading_ref_producers = ref_preparation
                    && crate::compiler::realize::reference_supplements::only_numbered_heading_targets(&content);"""
edit(evaluate, old, new)
edit(evaluate, """                let (result, warnings) = crate::compiler::realize::realize_document(
                    &content, world, source, registry, &previous, full_error, target,
                    features,
                );""",
     """                let (result, warnings) = crate::compiler::realize::realize_document_heading_cost(
                    &content, world, source, registry, &previous, full_error, target,
                    features, lab_no_heading_ref_producers,
                );""")
edit(realize, """    let mut ctx = EvalContext::new();
    ctx.full_error = full_error;
    ctx.target = target;
    ctx.features = features;
    ctx.introspector = previous.clone();""",
     """    realize_document_heading_cost(
        content, world, source, _registry, previous, full_error, target, features, false,
    )
}

#[inline(never)]
pub(in crate::compiler) fn realize_document_heading_cost(
    content: &Content,
    world: &dyn World,
    source: &Source,
    _registry: &ElementRegistry,
    previous: &TagIntrospector,
    full_error: bool,
    target: EvalTarget,
    features: Features,
    lab_no_heading_ref_producers: bool,
) -> (SourceResult<RealizedDocument>, Vec<SourceDiagnostic>) {
    let mut ctx = EvalContext::new();
    ctx.lab_no_heading_ref_producers = lab_no_heading_ref_producers;
    ctx.full_error = full_error;
    ctx.target = target;
    ctx.features = features;
    ctx.introspector = previous.clone();""")
edit(refs, """    if ctx.target == EvalTarget::Html || reference.form != RefForm::Normal {
        return Ok(content.clone());""",
     """    if ctx.lab_no_heading_ref_producers || ctx.target == EvalTarget::Html
        || reference.form != RefForm::Normal
    {
        return Ok(content.clone());""")
edit(paged, '''        eprintln!("lab-heading-ref initial={}", module.realization_introspector().is_some());''',
     '''        let index = module.realization_introspector();
        eprintln!("lab-heading-ref initial={}", index.is_some());
        eprintln!("lab-heading-cost context-roots={}", index.map_or(0, |i| i.context_block_locations.len()));''')
