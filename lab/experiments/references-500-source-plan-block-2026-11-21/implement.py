p='core/compiler/introspect/page_running'
r='core/compiler/layout/page_running_requests'
s='core/compiler/realize/context_occurrences'

def modify(source):
    source=replace(source, '    page_frontiers: Vec<bool>,','''    lab_suffix: Option<LabPlanBlock>,
    lab_marks: std::collections::HashMap<PacketId, Vec<LabPlanMark>>,
    lab_slots: Option<Vec<(LocatorSeed, Location)>>,
    page_frontiers: Vec<bool>,''')
    source=replace(source, '        self.nodes.get(id.0)','''        self.nodes.get(id.0).or_else(|| {
            let block = self.lab_suffix.as_ref()?;
            if id.0 < block.end {
                block.donor.node(id)
            } else {
                block.tail.get(id.0 - block.end)
            }
        })
    }

    pub(in crate::compiler) fn plan_nodes(&self) -> impl Iterator<Item = &PlanNode> {
        self.nodes.iter().chain(self.lab_suffix.iter().flat_map(|block| {
            block.donor.nodes[self.nodes.len()..block.end].iter().chain(&block.tail)
        }))
    }

    fn plan_environments(&self) -> impl Iterator<Item = &PlanEnvironment> {
        self.environments.iter().chain(self.lab_suffix.iter().flat_map(|block| {
            block.donor.environments[self.environments.len()..].iter()
        }))''')
    source=replace(source, '        let &(start, end) = self.child_ranges.get(parent.0)?;', '''        if parent.0 >= self.nodes.len() {
            let block = self.lab_suffix.as_ref()?;
            if parent.0 >= block.end {
                return None;
            }
            return block.donor.child_query(parent, edge, visit);
        }
        let &(start, end) = self.child_ranges.get(parent.0)?;''')
    source=replace(source, '        self.page_frontiers.get(packet.0).copied().unwrap_or(false)', '''        if let Some(&value) = self.page_frontiers.get(packet.0) {
            return value;
        }
        self.lab_suffix.as_ref().is_some_and(|block| {
            self.page_frontiers[block.anchor.0] && if packet.0 < block.end {
                block.donor.page_frontier(packet)
            } else {
                packet.0 - block.end < block.tail.len()
            }
        })''')
    source=replace(source, '        while let Some(current) = pending.pop() {\n            let Some(node)', '''        while let Some(current) = pending.pop() {
            if current.0 >= self.nodes.len() {
                continue;
            }
            let Some(node)''')
    source=replace(source, '        self.environments.get(id.0)','''        self.environments.get(id.0)
            .or_else(|| self.lab_suffix.as_ref()?.donor.environment(id))''')
    source=source.replace('plan.nodes.iter().enumerate()', 'plan.plan_nodes().enumerate()')
    source=source.replace('self.nodes.iter().any(|node|', 'self.plan_nodes().any(|node|')
    source=source.replace('self.environments.iter().enumerate().find', 'self.plan_environments().enumerate().find')
    source=source.replace('for (index, _) in plan.plan_nodes().enumerate() {', 'for (index, node) in plan.plan_nodes().enumerate() {')
    source=source.replace('invalid_plan(plan.nodes[index].span)', 'invalid_plan(node.span)')
    source=replace(source, "    lab_prior: Option<(&'a SourcePlan, usize)>,", '''    lab_prior: Option<(&'a SourcePlan, usize)>,
    lab_donor: Option<Arc<SourcePlan>>,
    lab_suffix: Option<LabPlanBlock>,
    lab_marks: std::collections::HashMap<PacketId, Vec<LabPlanMark>>,
    lab_terminal: bool,
    lab_enabled: bool,''')
    source=replace(source, '        let mut children: Vec<PlanChild> = Vec::new();', '''        if self.lab_enabled && self.lab_terminal && !self.original
            && self.conditional_ref.is_none() && !math && role == PlanRole::Canonical
            && label.is_none() && forced.is_none()
        {
            if let Content::Sequence(parts) = content {
                if parts.len() >= 256 {
                    self.lab_terminal_sequence(id, parts, &PlanBuildContext {
                        environment, source_parent: parent, span,
                        label: None, forced: None, role, math,
                    })?;
                    return Ok(id);
                }
            }
        }
        let mut children: Vec<PlanChild> = Vec::new();''')
    source=replace(source, '''        for PlanChild {
            edge,
            content: child,
            environment,
            label,
            forced,
            role,
        } in children
        {''', '''        let child_count = children.len();
        for (index, PlanChild {
            edge,
            content: child,
            environment,
            label,
            forced,
            role,
        }) in children.into_iter().enumerate()
        {
            let terminal = self.lab_terminal;
            self.lab_terminal &= index + 1 == child_count;''')
    source=replace(source, '            self.conditional_ref = previous_conditional;', '''            self.lab_terminal = terminal;
            self.conditional_ref = previous_conditional;''')
    source=replace(source, '    lab_build_body(content, ctx, chain, allocate, document, None)', '    lab_build_body(content, ctx, chain, allocate, document, None, false)')
    source=replace(source, "    prior: Option<(&'a SourcePlan, usize)>,\n) -> SourceResult<PageRunningBody> {", '''    witness: Option<&'a (Arc<SourcePlan>, usize)>,
    lab_enabled: bool,
) -> SourceResult<PageRunningBody> {
    let prior = witness.map(|(plan, first)| (plan.as_ref(), *first));''')
    source=replace(source, '        lab_prior: prior,\n        lab_reused: 0,', '''        lab_prior: prior,
        lab_donor: witness.map(|(plan, _)| plan.clone()),
        lab_suffix: None,
        lab_marks: std::collections::HashMap::new(),
        lab_terminal: true,
        lab_enabled,
        lab_reused: 0,''')
    source=replace(source, '''    let mut plan = SourcePlan {
        root,
        nodes: builder.nodes,''', '''    let lab_slots = if lab_enabled && builder.lab_suffix.is_none() {
        crate::compiler::realize::context_occurrences::lab_consumed_index_slots(builder.ctx)
    } else {
        None
    };
    let mut plan = SourcePlan {
        root,
        nodes: builder.nodes,
        lab_suffix: builder.lab_suffix,
        lab_marks: builder.lab_marks,
        lab_slots,''')
    source=replace(source, '''    let prior = witness.as_ref().map(|(plan, first)| (plan.as_ref(), *first));
    lab_build_body(content, ctx, &StyleChain::default_chain(), true, true, prior)''', '''    lab_build_body(content, ctx, &StyleChain::default_chain(), true, true, witness.as_ref(), true)''')
    return source+(exp/'block.rs').read_text()
edit(p,modify)
edit(r,lambda t:replace(t,'plan.nodes.iter().enumerate().all','plan.plan_nodes().enumerate().all'))
edit(s,lambda t:t+'''

pub(in crate::compiler) fn lab_index_slot_cursor(ctx: &EvalContext) -> Option<usize> {
    Some(ctx.context_occurrence_slots.as_ref()?.cursor)
}
''')
