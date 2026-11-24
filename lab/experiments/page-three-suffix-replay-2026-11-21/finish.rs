
impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    fn lab_finish_three(
        mut self,
        replay: physical_memo::LabThreeReplay,
    ) -> (PagedDocument, Option<page_running_requests::PlannedPass>) {
        use physical_memo::lab_three_cut_receipt;
        use crate::entities::sealed_positions::SealedPositions;
        const PREFIX: usize = 2;
        // Only the freshly painted prefix contains live markers.
        self.resolve_links();
        self.resolve_position_markers();
        let mut pass = self.page_running_pass.take().expect("three-page cut has plan");
        let old = &replay.previous.product;
        let prior = &old.document;
        self.runtime.positions.retain(|_, p| p.page.get() <= PREFIX);
        let positions_ok = prior.extracted_positions.lab_entries()
            .filter(|(_, p)| p.page.get() <= PREFIX)
            .all(|(loc, p)| self.runtime.positions.get(&loc) == Some(&p))
            && self.runtime.positions.iter().all(|(&loc, &p)|
                prior.extracted_positions.position_of(loc) == Some(p));
        let labels_ok = self.runtime.label_pages.iter().filter(|(_, &p)| p <= PREFIX)
            .all(|(key, page)| prior.extracted_label_pages.get(key) == Some(page)
                && prior.extracted_label_positions.get(key) == self.runtime.label_positions.get(key))
            && prior.extracted_label_pages.iter().filter(|(_, &p)| p <= PREFIX)
                .all(|(key, page)| self.runtime.label_pages.get(key) == Some(page)
                    && prior.extracted_label_positions.get(key) == self.runtime.label_positions.get(key));
        if self.pages.len() != PREFIX || prior.pages.len() <= PREFIX
            || !positions_ok || !labels_ok || prior.document_language != self.document_language
            || !prior.layout_errors.is_empty() || !prior.layout_warnings.is_empty()
            || !self.layout_errors.is_empty() || !self.layout_warnings.is_empty()
            || !prior.attachments.is_empty() || !self.attachments.is_empty()
            || !prior.extracted_figure_page_numbers.is_empty()
            || !prior.extracted_table_page_numbers.is_empty()
        {
            lab_three_cut_receipt(if !positions_ok { 11 } else if !labels_ok { 12 } else { 13 }, self.pages.len(), 0);
            pass.invalid = true;
            return (PagedDocument::new(self.pages), Some(pass));
        }
        self.runtime.positions.extend(prior.extracted_positions.lab_entries()
            .filter(|(_, p)| p.page.get() > PREFIX));
        let count = prior.pages.len() - PREFIX;
        self.pages.extend_from_slice(&prior.pages[PREFIX..]);
        let mut doc = PagedDocument::new(self.pages);
        doc.document_language = self.document_language;
        doc.extracted_label_pages = prior.extracted_label_pages.clone();
        doc.extracted_label_positions = prior.extracted_label_positions.clone();
        doc.extracted_positions = SealedPositions::from_runtime(self.runtime.positions);
        pass.lab_install_three_splice(replay.splice);
        pass.trace.data.pages = old.trace.data.pages.clone();
        pass.trace.data.positions = doc.extracted_positions.clone();
        lab_three_cut_receipt(0, PREFIX, count);
        (doc, Some(pass))
    }
}
