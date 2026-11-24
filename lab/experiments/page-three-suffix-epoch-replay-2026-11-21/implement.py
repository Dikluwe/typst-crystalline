m='core/compiler/layout/metrics'
p='core/compiler/layout/physical_memo'
l='core/compiler/layout/mod'
edit(m,lambda s:replace(s,
    '    pub fn same(&self, other: &Self) -> bool {',
    '''    pub(super) fn lab_same_dependency(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.dependency, &other.dependency)
    }

    pub fn same(&self, other: &Self) -> bool {'''))
def memo(s):
    s=replace(s,'''        let refuse = if prior.is_none() { 1 } else { 2 };
        let previous = prior.filter(|entry| {
            epoch.as_ref().is_some_and(|epoch| entry.epoch.same(epoch))
                && entry.journal.page_inputs_match(&entry.pages, &input.page_store)
                && entry.journal.validates(&metrics, &input)
        });''','''        let mut refuse = 1;
        let previous = prior.filter(|entry| {
            if !epoch.as_ref().is_some_and(|epoch| entry.epoch.lab_same_dependency(epoch)) {
                refuse = 2;
                return false;
            }
            if !entry.journal.page_inputs_match(&entry.pages, &input.page_store) {
                refuse = 14;
                return false;
            }
            if !entry.journal.validates(&metrics, &input) {
                refuse = 15;
                return false;
            }
            true
        });''')
    return replace(s,'metrics.physical_epoch().is_some_and(|epoch| prior.epoch.same(&epoch))',
                    'metrics.physical_epoch().is_some_and(|epoch| prior.epoch.lab_same_dependency(&epoch))')
edit(p,memo)
edit(l,lambda s:replace(replace(s,'    pub(super) lab_three_attempt:','    lab_three_attempt:'),
                                '    pub(super) lab_three_replay:','    lab_three_replay:'))
