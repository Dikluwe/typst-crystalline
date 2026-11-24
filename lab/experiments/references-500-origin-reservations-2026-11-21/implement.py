owner='core/compiler/introspect/page_running'

def assemble(text):
    first='    lab_suffix: Option<LabPlanBlock>,\n    lab_marks:'
    plan=text.index('pub(in crate::compiler) struct SourcePlan')
    compiler=text.index("struct PlanBuilder<'a>")
    top=replace(text[plan:compiler], first, '''    lab_suffix: Option<LabPlanBlock>,
    lab_origin_table: Option<LabOriginTable>,
    lab_marks:''')
    bottom=replace(text[compiler:], first, '''    lab_suffix: Option<LabPlanBlock>,
    lab_origin_proof: Option<LabOriginProof>,
    lab_marks:''')
    text=text[:plan]+top+bottom
    text=replace(text, '''fn reserve_source(content: &Content, ctx: &mut EvalContext) -> SourceResult<()> {
    let _ = content.map_content_pre(&mut |node| {
        match node {''','''#[inline(never)]
fn reserve_source(
    content: &Content,
    facts: &mut LabOriginFacts,
    scan: &mut LabOriginScan<'_>,
) -> SourceResult<()> {
    let _ = content.map_content_pre(&mut |node| {
        if let Content::Sequence(parts) = node {
            if scan.sequence(parts, facts)? {
                return Ok(Some(node.clone()));
            }
        }
        match node {''')
    text=replace(text,'ctx.realization_locator.observe(located.location);','facts.observe(located.location);')
    text=replace(text,'ctx.next_context_id = ctx.next_context_id.max(block.id.saturating_add(1));',
                 'facts.context_floor = facts.context_floor.max(block.id.saturating_add(1));')
    text=replace(text,'ctx.next_context_id = ctx.next_context_id.max(recipe.context_id_floor);',
                 'facts.context_floor = facts.context_floor.max(recipe.context_id_floor);')
    for node in ('&located.body', '&realized.original', '&realized.visual', 'supplement'):
        text=replace(text, f'reserve_source({node}, ctx)?',f'reserve_source({node}, facts, scan)?')
    assert text.count('reserve_source(body, ctx)?')==2
    text=text.replace('reserve_source(body, ctx)?','reserve_source(body, facts, scan)?')
    text=replace(text, '    reserve_source(content, ctx)?;', '''    let mut origin_scan = LabOriginScan {
        donor: prior.and_then(|(plan, frontier)| LabOriginDonor::new(plan, frontier)),
        table: None,
        proof: None,
        capture: lab_enabled && prior.is_none(),
    };
    let mut facts = LabOriginFacts::default();
    reserve_source(content, &mut facts, &mut origin_scan)?;
    facts.apply(ctx);
    if lab_enabled {
        lab_reserve_receipt(
            origin_scan.proof.as_ref().map_or(0, |proof| proof.end - proof.start),
            facts.context_floor,
            facts.location.map_or(0, |location| location.as_u128() + 1),
        );
    }''')
    text=replace(text, '        lab_suffix: None,\n        lab_marks:', '''        lab_suffix: None,
        lab_origin_proof: origin_scan.proof,
        lab_marks:''')
    text=replace(text, '        lab_suffix: builder.lab_suffix,\n        lab_marks:', '''        lab_suffix: builder.lab_suffix,
        lab_origin_table: origin_scan.table,
        lab_marks:''')
    text=replace(text, '''        parts: &[Content],
        context: &PlanBuildContext,''','''        parts: &Arc<[Content]>,
        context: &PlanBuildContext,''')
    start=text.index('                let common_len = parts.len().min(prior_parts.len());')
    end=text.index('                let index = old.children',start)
    text=text[:start]+'''                let shared_end = lab_shared_part_end(prior_parts, parts)?;
'''+text[end:]
    text=replace(text, '''                        prior_parts[index..shared_end].iter().zip(&parts[index..shared_end])
                            .try_for_each(|(a, b)| lab_block_content(a, b))?;''','''                        if !self.lab_origin_proof.as_ref().is_some_and(|proof|
                            proof.covers(parts, anchor, index, shared_end))
                        {
                            prior_parts[index..shared_end].iter().zip(&parts[index..shared_end])
                                .try_for_each(|(a, b)| lab_block_content(a, b))?;
                        }''')
    return text+(exp/'reservations.rs').read_text()

edit(owner,assemble)
