def snippet(name):
    return (exp/f'{name}.rs').read_text()

def layout(name,f):
    edit('core/compiler/layout/'+name,f)

def change(name,old,new):
    layout(name,lambda s:replace(s,old,new))

layout('mod',lambda s: replace(replace(s,
    '    pub(super) lab_geometry_lines: usize,',
    '''    pub(super) lab_geometry_lines: usize,
    pub(super) lab_three_attempt: Option<&'a physical_memo::LabThreeAttempt>,
    pub(super) lab_three_replay: Option<physical_memo::LabThreeReplay>,'''),
    '            lab_geometry_lines: 0,',
    '''            lab_geometry_lines: 0,
            lab_three_attempt: None,
            lab_three_replay: None,'''))
change('paragraph_session',
       '        Arc<Mutex<Option<Arc<super::page_running_requests::PageRunningDocument>>>>,',
       '''        Arc<Mutex<Option<Arc<super::page_running_requests::PageRunningDocument>>>>,
    pub(super) lab_three_retained: Arc<Mutex<Option<Arc<super::physical_memo::LabThreeRetained>>>>,''')
layout('paragraph_emit',lambda s:s+snippet('emit'))
layout('paragraph_plan',lambda s:s+'''
impl ParagraphCheckpoint {
    pub(super) fn lab_three_line(&self) -> paragraph_emit::LabThreeLine {
        self.emitter.lab_three_line()
    }
}
''')
change('sequence','''                        lab_resume_sequence(layouter, SequenceCheckpoint {''',
'''                        if lab_three_cut(layouter, parts, route.as_ref(), part_index, end,
                            text_segment_start, owns_text_segment, &paragraph)
                        {
                            return;
                        }
                        lab_resume_sequence(layouter, SequenceCheckpoint {''')
change('sequence','''        if let Some((group, parbreak)) = item_group_state {''','''        if layouter.lab_three_replay.is_some() {
            return;
        }
        if let Some((group, parbreak)) = item_group_state {''')
layout('sequence',lambda s:s+snippet('cut'))

change('page_running_requests',
       '    pub(in crate::compiler) lab_outline_end: Option<super::lab_retain::FlowEnd>,',
       '''    pub(in crate::compiler) lab_outline_end: Option<super::lab_retain::FlowEnd>,
    pub(super) lab_three_cut: Option<LabThreeCut>,
    pub(super) lab_three_reused: bool,''')
change('page_running_requests','                lab_outline_end: None,',
       '''                lab_outline_end: None,
                lab_three_cut: None,
                lab_three_reused: false,''')
layout('page_running_requests',lambda s:s+snippet('trace'))
edit('core/entities/sealed_positions',lambda s:s+'''
impl SealedPositions {
    pub fn lab_entries(&self) -> impl Iterator<Item = (Location, Position)> + '_ {
        self.positions.iter().map(|(&location, &position)| (location, position))
    }
}
''')
change('finish',
       '        // P842 (#38)',
       '''        if let Some(replay) = self.lab_three_replay.take() {
            return self.lab_finish_three(replay);
        }
        // P842 (#38)''')
layout('finish',lambda s:s+snippet('finish'))

change('convergence',
       'inputs, journal, false, false, None)',
       'inputs, journal, false, false, None, None)')
change('convergence',
       '    mut checkpoint_out: Option<&mut Option<Box<super::lab_retain::OutlineCheckpoint>>>,',
       '''    mut checkpoint_out: Option<&mut Option<Box<super::lab_retain::OutlineCheckpoint>>>,
    lab_three: Option<&super::physical_memo::LabThreeAttempt>,''')
change('convergence','        l.lab_capture_outline = lab_capture_outline;',
       '''        l.lab_capture_outline = lab_capture_outline;
        l.lab_three_attempt = lab_three;''')
change('convergence','        let compact_lines = l.lab_geometry_lines;',
       '''        let cut = l.lab_three_replay.is_some();
        let compact_lines = l.lab_geometry_lines;''')
change('convergence','        running.trace.data.lab_outline_end = outline_end;',
       '''        running.trace.data.lab_outline_end = outline_end;
        if cut && !running.consumed_all() {
            return PageRunningPassOutcome::pending(PageRunningRequests {
                callbacks: LayoutCallbackRequests { math: vec![], par_line: vec![] },
                running: vec![],
                trace: running.trace,
            });
        }''')

change('physical_memo','inputs, None, true, false, None,','inputs, None, true, false, None, None,')
change('physical_memo','''pub struct LabReplayWitness {
    checkpoint:''','''pub struct LabReplayWitness {
    three: Option<LabThreeAttempt>,
    checkpoint:''')
change('physical_memo','''    sealed: &super::SealedPageRunning,
) -> Result<(PageRunningDocument, LabReplayWitness),''','''    sealed: &super::SealedPageRunning, certify_three: bool,
) -> Result<(PageRunningDocument, LabReplayWitness),''')
change('physical_memo','''    let mut witness = LabReplayWitness {
        checkpoint: None,''','''    let epoch = metrics.physical_epoch();
    let three = certify_three.then(|| {
        let prior = metrics.paragraph_session()
            .and_then(|session| session.lab_three_retained.lock().unwrap().clone());
        let refuse = if prior.is_none() { 1 } else { 2 };
        let previous = prior.filter(|entry| {
            epoch.as_ref().is_some_and(|epoch| entry.epoch.same(epoch))
                && entry.journal.page_inputs_match(&entry.pages, &input.page_store)
                && entry.journal.validates(&metrics, &input)
        });
        LabThreeAttempt { previous, refuse }
    });
    let mut witness = LabReplayWitness {
        three, checkpoint: None,''')
change('physical_memo',
       '''inputs, Some(&witness.journal), false, true, Some(&mut witness.checkpoint)).into_result()?;
    if let Some(previous) = previous {''',
       '''inputs, Some(&witness.journal), false, true, Some(&mut witness.checkpoint),
        witness.three.as_ref()).into_result()?;
    if let Some(previous) = previous.filter(|_| !product.trace.data.lab_three_reused) {''')
change('physical_memo','''    let product = Arc::new(product);
    if let Some(paragraphs) = metrics.paragraph_session() {
        *paragraphs.lab_previous_physical.lock().unwrap() = Some(Arc::clone(&product));
    }
    session.0.lock().unwrap().entry = Some(Entry {''','''    if product.trace.data.lab_three_reused {
        let prior = witness.three.and_then(|attempt| attempt.previous).ok_or("missing suffix witness")?;
        if !metrics.physical_epoch().is_some_and(|epoch| prior.epoch.same(&epoch))
            || !prior.journal.page_inputs_match(&prior.pages, &index.page_store)
            || !prior.journal.validates(metrics, index)
        {
            return Err("retained suffix dependency changed");
        }
        return Ok(Arc::new(product));
    }
    let product = Arc::new(product);
    if let Some(paragraphs) = metrics.paragraph_session() {
        *paragraphs.lab_previous_physical.lock().unwrap() = Some(Arc::clone(&product));
        if let Some(epoch) = witness.epoch.clone().filter(|_| product.trace.data.lab_three_cut.is_some()) {
            let retained = LabThreeRetained {
                product: Arc::clone(&product), epoch, journal: witness.journal.clone(),
                pages: witness.pages.clone(),
            };
            *paragraphs.lab_three_retained.lock().unwrap() = Some(Arc::new(retained));
        }
    }
    session.0.lock().unwrap().entry = Some(Entry {''')
layout('physical_memo',lambda s:s+snippet('memo'))
edit('infra/pipeline/context_stabilization/page_running',lambda s:replace(s,
    'self.metrics.clone(), ImageSizeImageSizer, &sealed).map_err(|_| "retained layout")?;',
    '''self.metrics.clone(), ImageSizeImageSizer, &sealed,
            lab_page_three_fixture(self.source.text())).map_err(|_| "retained layout")?;''')
    +snippet('fixture'))
