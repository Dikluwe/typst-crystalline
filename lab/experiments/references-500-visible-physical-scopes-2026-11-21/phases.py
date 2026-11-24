"""Exclusive CPU attribution within and around the single physical visit."""

PHYSICAL = (
    "paragraph_prepare", "paragraph_emit", "root_outline_links", "root_bidi_inputs",
    "root_navigation", "root_pagination", "root_other", "finish_links", "finish_positions",
    "finish_other", "physical_pass_other",
)
PAGED = (
    "outline_preparation", "source_plan", "physical_indexes", "physical_memo",
    "reuse_pages", "bidi", "shaping", "line_reconciliation", "paged_other",
)
TEKT = (*PHYSICAL, *PAGED, "eval", "pdf", "other")
VANILLA = (
    "document_realization", "nested_realization", "paragraph_prepare", "paragraph_break",
    "paragraph_finalize", "page_flow_other", "physical_indexes", "layout_other",
    "eval", "pdf", "other",
)


def classify(arm, stack):
    def has(*tokens):
        return any(token in frame for token in tokens for frame in stack)

    if arm == "vanilla":
        if has("::PagedIntrospector::new"):
            return "physical_indexes"
        if has("typst_realize::realize"):
            if stack.count("typst_realize::realize") > 1 or has("typst_layout::flow::", "typst_layout::inline::"):
                return "nested_realization"
            return "document_realization"
        if has("typst_layout::pages::layout_document"):
            if has("typst_layout::inline::prepare::", "typst_layout::inline::collect::",
                   "typst_layout::inline::shaping::"):
                return "paragraph_prepare"
            if has("typst_layout::inline::linebreak::"):
                return "paragraph_break"
            if has("typst_layout::inline::finalize::"):
                return "paragraph_finalize"
            if has("typst_layout::pages::run::", "typst_layout::flow::"):
                return "page_flow_other"
            return "layout_other"
        if has("typst_eval::eval"):
            return "eval"
        if has("typst_pdf::", "krilla::"):
            return "pdf"
        return "other"

    if has("::eval_to_module_in_session"):
        return "eval"
    if has("::convergence::run_planned", "::physical_memo::lab_retain"):
        if has("::finish_with_running"):
            if has("::link_capture::"):
                return "finish_links"
            if has("::position_marker::"):
                return "finish_positions"
            return "finish_other"
        if has("::layout_planned_root", "::layout_planned_cursor"):
            if has("::outline::layout_", "::link_capture::"):
                return "root_outline_links"
            if has("::bidi_origin::prepare", "::bidi_origin::collect", "::bidi_origin::capture"):
                return "root_bidi_inputs"
            if has("::page_commit::", "::new_page", "::page_break::", "::page_transition::",
                   "::realize_numbering", "::page_running::explicit_layers"):
                return "root_pagination"
            if has("::paragraph_emit::", "::line_commit::", "::line_flush::",
                   "::word_layout::", "::text::layout_run"):
                return "paragraph_emit"
            if has("::paragraph_session::", "::paragraph_plan::", "::paragraph_select",
                   "::paragraph_hyphen::", "::text_measure::"):
                return "paragraph_prepare"
            if has("::admit_planned_phase", "::position_marker::", "::record_packet_position",
                   "::SourcePlan::child", "::planned_child_cursor", "::certifies_pure_source_part"):
                return "root_navigation"
            return "root_other"
        return "physical_pass_other"

    if has("typst_infra::layout_bidi::", "typst_infra::bidi_context::Registry::"):
        return "bidi"
    if has("::shaper::reconciliation::", "::fix_line_positions"):
        return "line_reconciliation"
    if has("::shape_document_with_faces", "::shape_document_with_context"):
        return "shaping"
    if has("::pre_bidi::"):
        return "reuse_pages"
    if has("::prepare_outline_candidate", "::outline_reads_valid"):
        return "outline_preparation"
    if has("::prepare_running_body", "::prepare_page_running_body"):
        return "source_plan"
    if has("::index_physical_steps"):
        return "physical_indexes"
    if has("::physical_memo::", "::validate_page_inputs"):
        return "physical_memo"
    if has("::compile_paged_in_session"):
        return "paged_other"
    if has("::compile_to_pdf_bytes_impl", "::export::"):
        return "pdf"
    return "other"
