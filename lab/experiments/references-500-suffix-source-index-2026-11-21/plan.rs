
fn lab_same_leaf(a: &Content, b: &Content) -> bool {
    if let (Content::Text(a), Content::Text(b)) = (a, b) {
        return a == b;
    }
    matches!((a,b),
        (Content::Empty, Content::Empty) | (Content::Space, Content::Space)
        | (Content::SpaceFromNewline, Content::SpaceFromNewline)
        | (Content::Parbreak, Content::Parbreak))
}

impl PlanBuilder<'_> {
    fn lab_reuse_text_packets(
        &mut self,
        content: &Content,
        context: &PlanBuildContext,
    ) -> Option<PacketId> {
        let (old, first) = self.lab_prior?;
        let ordinal = self.nodes.len();
        if ordinal < first || context.role != PlanRole::Canonical
            || self.original || self.conditional_ref.is_some() || context.math
            || context.label.is_some() || context.forced.is_some()
        {
            return None;
        }
        let id = PacketId(ordinal);
        let prior = old.node(id)?;
        let lang = self.environments[context.environment.0].chain.lang();
        let eligible = |node: &PlanNode| {
            node.role == PlanRole::Canonical && !node.original && !node.math
                && node.conditional_ref.is_none() && node.label.is_none()
                && node.location.is_none() && node.auxiliary_locations.is_empty()
                && node.environment == context.environment && node.source_parent == context.source_parent
                && node.lang == lang
        };
        if !eligible(prior) {
            return None;
        }
        let own = content.source_span();
        let span = if own.is_detached() { context.span } else { own };
        let inner = if let (Content::Source(a), Content::Source(b)) = (content, &prior.content) {
            let child = PacketId(ordinal + 1);
            if prior.children.as_slice() != [(PlanEdge::SourceBody, child)] {
                return None;
            }
            let leaf = old.node(child)?;
            if !eligible(leaf) || !leaf.children.is_empty()
                || !lab_same_leaf(&a.body, &b.body) || !lab_same_leaf(&a.body, &leaf.content)
            {
                return None;
            }
            Some(leaf)
        } else {
            if !prior.children.is_empty() || !lab_same_leaf(content, &prior.content) {
                return None;
            }
            None
        };
        let mut node = prior.clone();
        node.span = span;
        node.content = content.clone();
        self.nodes.push(node);
        self.lab_reused += 1;
        if let Some(inner) = inner {
            let mut inner = inner.clone();
            inner.span = span;
            self.nodes.push(inner);
            self.lab_reused += 1;
        }
        Some(id)
    }
}

#[inline(never)]
fn lab_plan_reuse_receipt(count: usize) {
    std::hint::black_box(count);
}

/// Closed-fixture experiment, retaining only rebindable text packets.
pub fn lab_prepare_suffix_source_plan(
    content: &Content,
    ctx: &mut EvalContext,
    session: &crate::compiler::layout::paragraph_session::ParagraphSession,
) -> SourceResult<PageRunningBody> {
    if ctx.context_occurrence_slots.is_none() {
        return Err(invalid_plan(content.source_span()));
    }
    let witness = session.lab_previous_source_suffix();
    let prior = witness.as_ref().map(|(plan, first)| (plan.as_ref(), *first));
    lab_build_body(content, ctx, &StyleChain::default_chain(), true, true, prior)
}
