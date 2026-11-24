p='core/compiler/introspect/page_running'
q='core/compiler/layout/paragraph_session'
r='core/compiler/layout/page_running_requests'
f='core/compiler/layout/finish'
s='core/compiler/realize/context_occurrences'
i='infra/pipeline/context_stabilization/page_running'

def snip(name):
    return (exp / f'{name}.rs').read_text()

def plan(txt):
    txt=replace(txt,'''struct PlanBuilder<'a> {
    ctx: &'a mut EvalContext,''','''struct PlanBuilder<'a> {
    ctx: &'a mut EvalContext,
    lab_prior: Option<(&'a SourcePlan, usize)>,
    lab_reused: usize,''')
    txt=replace(txt,'''        let PlanBuildContext {
            environment,''','''        if self.lab_prior.is_some() {
            if let Some(packet) = self.lab_reuse_text_packets(content, &context) {
                return Ok(packet);
            }
        }
        let PlanBuildContext {
            environment,''')
    txt=replace(txt,'''    let allocation_owner =
        crate::compiler::realize::context_occurrences::allocation_owner(ctx);''','''    lab_build_body(content, ctx, chain, allocate, document, None)
}

fn lab_build_body<'a>(
    content: &Content,
    ctx: &'a mut EvalContext,
    chain: &StyleChain,
    allocate: bool,
    document: bool,
    prior: Option<(&'a SourcePlan, usize)>,
) -> SourceResult<PageRunningBody> {
    let allocation_owner =
        crate::compiler::realize::context_occurrences::allocation_owner(ctx);''')
    txt=replace(txt,'''    let mut builder = PlanBuilder {
        ctx,
        allocate,''','''    let mut builder = PlanBuilder {
        ctx,
        lab_prior: prior,
        lab_reused: 0,
        allocate,''')
    txt=replace(txt,'''    let mut plan = SourcePlan {
        root,
        nodes: builder.nodes,''','''    if prior.is_some() {
        lab_plan_reuse_receipt(builder.lab_reused);
    }
    let mut plan = SourcePlan {
        root,
        nodes: builder.nodes,''')
    return txt+snip('plan')+snip('index')
edit(p,plan)
def session(txt):
    txt=replace(txt,'''    pub(super) lab_three_retained: Arc<Mutex<Option<Arc<super::physical_memo::LabThreeRetained>>>>,''','''    pub(super) lab_three_retained: Arc<Mutex<Option<Arc<super::physical_memo::LabThreeRetained>>>>,
    pub(in crate::compiler) lab_previous_index: Arc<Mutex<Option<Arc<crate::compiler::introspect::page_running::LabSemanticSnapshot>>>>,''')
    return txt+'''
impl ParagraphSession {
    pub(in crate::compiler) fn lab_previous_source_suffix(
        &self,
    ) -> Option<(Arc<crate::compiler::introspect::page_running::SourcePlan>, usize)> {
        self.lab_three_retained.lock().unwrap().as_ref()?.product.trace.lab_source_suffix()
    }
}
'''
edit(q,session)
def trace(txt):
    txt=replace(txt,'''    pub(super) lab_three_reused: bool,''','''    pub(super) lab_three_reused: bool,
    pub(in crate::compiler) lab_index_prior: Option<Arc<SourcePlan>>,''')
    txt=replace(txt,'''                lab_three_reused: false,''','''                lab_three_reused: false,
                lab_index_prior: None,''')
    return txt+'''
impl PageRunningTrace {
    pub(super) fn lab_source_suffix(&self) -> Option<(Arc<SourcePlan>, usize)> {
        let frontier = self.data.admissions.iter().filter(|event|
            event.page.get() >= 3 && event.phase == PlanPhase::Root
                && Arc::ptr_eq(&event.plan, &self.data.source)
        ).map(|event| event.packet.0).min()?;
        (self.data.complete && self.data.lab_three_cut.is_some() && frontier > 0)
            .then(|| (self.data.source.clone(), frontier))
    }
}
'''
edit(r,trace)
edit(f,lambda txt:replace(txt,'''        pass.lab_install_three_splice(replay.splice);''','''        pass.lab_install_three_splice(replay.splice);
        pass.trace.data.lab_index_prior = Some(old.trace.data.source.clone());'''))
edit(s,lambda txt:txt+snip('slots'))
def infra(txt):
    txt=replace(txt, 'use std::num::NonZeroUsize;', 'use std::num::NonZeroUsize;\nuse typst_core::compiler::layout::FontMetrics;')
    txt=replace(txt,'''    fn prepare_running_body(&mut self, content: &Content) -> SourceResult<()> {
        let mut versions''','''    fn prepare_running_body(&mut self, content: &Content) -> SourceResult<()> {
        self.prepare_running_body_with_suffix(content, None)
    }

    fn prepare_running_body_with_suffix(
        &mut self, content: &Content,
        suffix: Option<&typst_core::compiler::layout::paragraph_session::ParagraphSession>,
    ) -> SourceResult<()> {
        let mut versions''')
    txt=replace(txt,'''                prepare_page_running_body(content, ctx)
            });
        self.running.body_frame''','''                if let Some(session) = suffix {
                    typst_core::compiler::introspect::page_running::lab_prepare_suffix_source_plan(content, ctx, session)
                } else {
                    prepare_page_running_body(content, ctx)
                }
            });
        self.running.body_frame''')
    anchor='''    fn index_physical_steps(
        &mut self,
        steps: &[PhysicalStep],
        input: &TagIntrospector,
    ) -> SourceResult<(
        TagIntrospector,
        Vec<typst_core::entities::locator::LocatorSeed>,
        Vec<SourceDiagnostic>,
    )> {
        let inputs: Vec<_> = steps'''
    txt=replace(txt,anchor,'''    fn index_physical_steps(
        &mut self, steps: &[PhysicalStep], input: &TagIntrospector,
    ) -> SourceResult<(
        TagIntrospector, Vec<typst_core::entities::locator::LocatorSeed>, Vec<SourceDiagnostic>,
    )> {
        self.index_physical_steps_with_suffix(steps, input, None)
    }

    fn index_physical_steps_with_suffix(
        &mut self,
        steps: &[PhysicalStep],
        input: &TagIntrospector,
        suffix: Option<&typst_core::compiler::layout::paragraph_session::ParagraphSession>,
    ) -> SourceResult<(
        TagIntrospector,
        Vec<typst_core::entities::locator::LocatorSeed>,
        Vec<SourceDiagnostic>,
    )> {
        let inputs: Vec<_> = steps''')
    txt=replace(txt,'''                    introspect_with_page_running(&inputs, features, ctx, engine)
                })''','''                    if let Some(session) = suffix {
                        typst_core::compiler::introspect::page_running::lab_introspect_suffix_index(
                            &inputs, features, ctx, engine, session)
                    } else {
                        introspect_with_page_running(&inputs, features, ctx, engine)
                    }
                })''')
    a=txt.index('    fn lab_try_retain(')
    b=txt.index('    fn physical_bundle_candidate(',a)
    body=txt[a:b]
    body=replace(body,'''        self.prepare_running_body(&source).map_err(|_| "provisional source plan")?;''','''        let closed = lab_page_three_fixture(self.source.text());
        let suffix = closed.then(|| self.metrics.paragraph_session()).flatten();
        self.prepare_running_body_with_suffix(&source, suffix.as_ref())
            .map_err(|_| "provisional source plan")?;''')
    body=replace(body,'''            lab_page_three_fixture(self.source.text())).map_err(|_| "retained layout")?;''','''            closed).map_err(|_| "retained layout")?;''')
    body=replace(body,'''        let (index, _, warnings) = self.index_physical_steps(std::slice::from_ref(&step), input)
            .map_err(|_| "retained index")?;''','''        let (index, _, warnings) = self.index_physical_steps_with_suffix(
            std::slice::from_ref(&step), input, suffix.as_ref())
            .map_err(|_| "retained index")?;''')
    return txt[:a]+body+txt[b:]
edit(i,infra)
