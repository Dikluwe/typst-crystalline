
pub(super) struct LabThreeRetained {
    pub(super) product: Arc<PageRunningDocument>,
    epoch: MetricEpoch,
    journal: Journal,
    pages: PageStore,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct LabThreeBoundary {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) text_segment_start: usize,
    pub(super) owns_text_segment: bool,
    pub(super) packet: crate::compiler::introspect::page_running::PacketId,
    pub(super) line: super::paragraph_emit::LabThreeLine,
    pub(super) page: PageConfig,
    pub(super) scalars: [u64; 11],
    pub(super) flags: [bool; 5],
}

pub(super) struct LabThreeAttempt {
    pub(super) previous: Option<Arc<LabThreeRetained>>,
    pub(super) refuse: u64,
}

pub(super) struct LabThreeReplay {
    pub(super) previous: Arc<LabThreeRetained>,
    pub(super) splice: super::page_running_requests::LabThreeSplice,
}

#[inline(never)]
pub(super) fn lab_three_cut_receipt(reason: u64, prefix_pages: usize, reused_pages: usize) {
    std::hint::black_box((reason, prefix_pages, reused_pages));
}
