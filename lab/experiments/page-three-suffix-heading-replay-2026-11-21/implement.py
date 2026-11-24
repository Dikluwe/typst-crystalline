p='core/compiler/layout/physical_memo'
i='core/compiler/layout/physical_memo/introspection'
def memo(s):
    s=replace(s,'''struct JournalState {
    page_reads:''','''struct JournalState {
    lab_three_reads: bool,
    page_reads:''')
    s=replace(s,'''    fn decline(&self, reason: &'static str) {''','''    fn lab_three_reads(&self) -> bool {
        self.0.lock().unwrap().lab_three_reads
    }
    fn decline(&self, reason: &'static str) {''')
    return replace(s,'''    let mut witness = LabReplayWitness {
        three, checkpoint: None,
        journal: Journal::default(),''','''    let journal = Journal::default();
    journal.0.lock().unwrap().lab_three_reads = certify_three;
    let mut witness = LabReplayWitness {
        three, checkpoint: None,
        journal,''')
edit(p,memo)
def reflection(s):
    s=replace(s,'''            if heading_body_is_closed(&heading.body) {
                let expected = std::sync::Arc::downgrade(heading);''','''            if heading_body_is_closed(&heading.body) && self.journal.lab_three_reads() {
                let expected = heading.as_ref().clone();
                self.journal.index(17 * 1024, move |input| {
                    matches!(input.element_at(location),
                        Some(crate::entities::content::Content::Heading(current))
                            if lab_same_heading(&expected, current))
                });
            } else if heading_body_is_closed(&heading.body) {
                let expected = std::sync::Arc::downgrade(heading);''')
    s=replace(s,'''    #[test]
    fn same_heading_owner_validates_and_unread_data_is_ignored()''','''    #[test]
    fn lab_three_heading_compares_closed_content_across_owned_instances() {
        let h = heading(Content::sequence(vec![Content::text("A"), Content::Space, Content::text("B")]));
        let old = index(h.clone());
        let journal = Journal::default();
        journal.0.lock().unwrap().lab_three_reads = true;
        let r = RecordingIntrospector { input: &old, journal: &journal };
        r.element_at(Location::from_raw(1)).expect("heading");
        assert!(journal.validates(&FixedMetrics, &index(Arc::new((*h).clone()))));
        let newline = h.with_body(Content::sequence(vec![Content::text("A"), Content::SpaceFromNewline, Content::text("B")]));
        assert!(!journal.validates(&FixedMetrics, &index(Arc::new(newline))));
        let mut flags = (*h).clone();
        flags.set_fields = 0;
        assert!(!journal.validates(&FixedMetrics, &index(Arc::new(flags))));
        let changed = h.with_body(Content::text("different"));
        assert!(!journal.validates(&FixedMetrics, &index(Arc::new(changed))));
        assert!(!journal.validates(&FixedMetrics, &TagIntrospector::empty()));
    }

    #[test]
    fn same_heading_owner_validates_and_unread_data_is_ignored()''')
    return s + (exp/'heading.rs').read_text()
edit(i,reflection)
