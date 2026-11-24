#[derive(Clone, Copy, Debug, Default)]
pub struct LabReport {
    pub hits: usize,
    pub misses: usize,
    pub stores: usize,
    pub deferred: usize,
    pub evicted_during: usize,
    pub evicted_after: usize,
    pub peak: usize,
    pub charged: usize,
    pub entries: usize,
}

struct Entry {
    key: Key,
    value: Value,
    protected: bool,
}
pub(super) struct Memo {
    entries: Vec<Entry>,
    charged: usize,
    hits: usize,
    misses: usize,
    budget: usize,
    preserve: bool,
    reserve: usize,
    report: LabReport,
}

impl Default for Memo {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            charged: 0,
            hits: 0,
            misses: 0,
            budget: MAX_TOTAL,
            preserve: false,
            reserve: 0,
            report: LabReport::default(),
        }
    }
}

impl Memo {
    fn remove(&mut self, index: usize) {
        self.charged -= self.entries.remove(index).key.charge;
    }

    fn begin(&mut self, preserve: bool, budget: usize) {
        self.preserve = preserve;
        self.budget = budget.min(MAX_TOTAL);
        self.reserve = 0;
        self.report = LabReport::default();
        while self.charged > self.budget {
            self.remove(0);
            self.report.evicted_after += 1;
        }
        self.report.peak = self.charged;
        if preserve {
            for entry in &mut self.entries {
                entry.protected = true;
            }
        }
    }

    fn finish(&mut self) {
        // The deferred value has already been dropped. Free space for a
        // future admission only after the current scan has finished.
        let target = self.budget - self.reserve;
        while self.charged > target {
            let index = self.entries.iter().position(|entry| entry.protected).unwrap_or(0);
            self.remove(index);
            self.report.evicted_after += 1;
        }
        self.report.charged = self.charged;
        self.report.entries = self.entries.len();
    }

    pub(super) fn lookup(&mut self, key: &Key) -> Option<Value> {
        if let Some(index) = self.entries.iter().position(|entry| entry.key.matches(key)) {
            let mut entry = self.entries.remove(index);
            let value = entry.value.clone();
            entry.protected = false;
            self.entries.push(entry);
            self.hits += 1;
            self.report.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            self.report.misses += 1;
            None
        }
    }

    pub(super) fn store(&mut self, key: Key, value: Value) {
        if key.charge > self.budget {
            return;
        }
        let available = self.budget - key.charge;
        if self.preserve && self.charged > available {
            let evictable: usize = self.entries.iter().filter(|entry| !entry.protected)
                .map(|entry| entry.key.charge).sum();
            if evictable < self.charged - available {
                self.reserve = self.reserve.max(key.charge);
                self.report.deferred += 1;
                return;
            }
        }
        while self.charged > available {
            let index = if self.preserve {
                self.entries.iter().position(|entry| !entry.protected).unwrap()
            } else {
                0
            };
            self.remove(index);
            self.report.evicted_during += 1;
        }
        self.charged += key.charge;
        self.entries.push(Entry { key, value, protected: false });
        self.report.stores += 1;
        self.report.peak = self.report.peak.max(self.charged);
    }
}
