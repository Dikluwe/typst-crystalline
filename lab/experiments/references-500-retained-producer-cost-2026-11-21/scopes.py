"""Exclusive work classes derived from the causal caller, not callee names alone."""

TEKT = (
    "eval_initial", "eval_rest", "execute", "validate_reads", "validation_engine", "substitute_source",
    "substitute_other", "validate_other", "context_index", "context_other", "physical", "shape",
    "pdf", "other",
)
VANILLA = ("eval", "realization", "physical_index", "layout_rest", "pdf", "other")


def category(arm, stack):
    def has(*needle):
        return any(part in frame for part in needle for frame in stack)

    if arm == "vanilla":
        if has("::PagedIntrospector::new"):
            return "physical_index"
        if has("typst_realize::realize"):
            return "realization"
        if has("typst_eval::eval"):
            return "eval"
        if has("typst_layout::pages::layout_document"):
            return "layout_rest"
        if has("typst_pdf::pdf", "krilla::"):
            return "pdf"
        return "other"

    if has("::eval_to_module_in_session"):
        return "eval_initial" if has("::realize::realize_document") else "eval_rest"
    if has("::Session::execute", "::reference_supplements::produce"):
        return "execute"
    if has("[producer-replay-engine]"):
        return "validate_reads" if has("::context_reads_valid_for") else "validation_engine"
    if has("::context_reads_valid_for", "::Session::with_replay_engine"):
        return "validate_other"
    if has("::Session::content", "::Session::body") and not has("::outline_state_content"):
        return "substitute_source"
    if has("::substitute_context_blocks", "::substitution::map_", "::outline_state_content"):
        return "substitute_other"
    if has("::Session::realize") and has("::introspect_with_introspector", "::introspect::walk::"):
        return "context_index"
    if has("::physical_memo::lab_retain", "::convergence::run_planned", "::layout_planned_root",
           "::compiler::layout::physical_memo::layout_with", "::prepare_outline_candidate",
           "::index_physical_steps", "::prepare_running_body"):
        return "physical"
    if has("::context_stabilization::"):
        return "context_other"
    if has("::shaper::traversal::shape_document_with_faces", "::layout_bidi::", "::pre_bidi::",
           "::shaper::reconciliation::"):
        return "shape"
    if has("::compile_to_pdf_bytes_impl"):
        return "pdf"
    return "other"
