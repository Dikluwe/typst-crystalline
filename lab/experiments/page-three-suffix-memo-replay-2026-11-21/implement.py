p='core/compiler/layout/physical_memo'
def compose(s):
    s=replace(s,'''    journal: Journal,
    product: Arc<PageRunningDocument>,
}''','''    journal: Journal,
    suffix_reads: Option<(Journal, PageStore)>,
    product: Arc<PageRunningDocument>,
}''')
    s=replace(s,'''    if product.trace.data.lab_three_reused {
        let prior = witness.three.and_then(|attempt| attempt.previous).ok_or("missing suffix witness")?;''',
                '''    let suffix_reads = if product.trace.data.lab_three_reused {
        let prior = witness.three.as_ref().and_then(|attempt| attempt.previous.as_ref())
            .ok_or("missing suffix witness")?;''')
    s=replace(s,'''        return Ok(Arc::new(product));
    }
    let product = Arc::new(product);
    if let Some(paragraphs) = metrics.paragraph_session() {''','''        Some((prior.journal.clone(), prior.pages.clone()))
    } else {
        None
    };
    let product = Arc::new(product);
    if let Some(paragraphs) = metrics.paragraph_session().filter(|_| suffix_reads.is_none()) {''')
    s=replace(s,'''        journal: witness.journal, product: Arc::clone(&product),''','''        journal: witness.journal, suffix_reads, product: Arc::clone(&product),''')
    s=replace(s,'''                && entry.journal.validates(&metrics, &input)
                && same_epoch''','''                && entry.journal.validates(&metrics, &input)
                && entry.suffix_reads.as_ref().is_none_or(|(journal, pages)|
                    journal.page_inputs_match(pages, &input.page_store) && journal.validates(&metrics, &input))
                && same_epoch''')
    s=replace(s,'''                    journal,
                    product,
                });''','''                    journal,
                    suffix_reads: None,
                    product,
                });''')
    return s
edit(p,compose)
