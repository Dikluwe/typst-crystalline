
#[derive(Debug, Clone)]
pub(super) struct LabThreeCut {
    pub(super) boundary: super::physical_memo::LabThreeBoundary,
    admissions: usize,
    order: usize,
}

pub(super) struct LabThreeSplice {
    prefix: usize,
    suffix: Vec<Admission>,
}

impl PlannedPass {
    pub(super) fn lab_three_cut(
        &self, boundary: super::physical_memo::LabThreeBoundary,
    ) -> LabThreeCut {
        LabThreeCut {
            boundary, admissions: self.trace.data.admissions.len(), order: self.next_causal_order,
        }
    }

    pub(super) fn lab_prepare_three_splice(
        &self,
        old: &PageRunningTrace,
        current: &LabThreeCut,
    ) -> Result<LabThreeSplice, u64> {
        let previous = old.data.lab_three_cut.as_ref().ok_or(5_u64)?;
        if previous.boundary != current.boundary {
            return Err(6);
        }
        if self.invalid || !self.trace.data.requests.is_empty() || !old.data.requests.is_empty()
            || !self.trace.data.inactive_repeat_bodies.is_empty()
            || !old.data.inactive_repeat_bodies.is_empty()
            || !old.data.complete
            || self.slots.iter().any(Option::is_some)
            || !self.consumed.is_empty()
        {
            return Err(7);
        }
        let source = &self.source;
        let mut suffix = Vec::new();
        for old_event in old.data.admissions.get(previous.admissions..).ok_or(8_u64)? {
            if !Arc::ptr_eq(&old_event.plan, &old.data.source)
                || old_event.deferred_anchor || old_event.section != TraceSection::Body
            {
                return Err(8);
            }
            let packet = old_event.packet;
            let a = old.data.source.node(packet).ok_or(8_u64)?;
            let Some(b) = source.node(packet) else {
                if old.data.source.required(packet, old_event.phase) || a.location.is_some() {
                    return Err(9);
                }
                continue;
            };
            if a.location != b.location || a.auxiliary_locations != b.auxiliary_locations
                || a.role != b.role || a.label != b.label || a.lang != b.lang
                || a.source_parent != b.source_parent || a.environment != b.environment
                || a.conditional_ref != b.conditional_ref || a.original != b.original
                || a.math != b.math
                || std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)
            {
                return Err(9);
            }
            let [old_order] = old_event.causal_order.as_slice() else { return Err(8) };
            let order = old_order.checked_sub(previous.order)
                .and_then(|offset| current.order.checked_add(offset)).ok_or(8_u64)?;
            let mut rebound = old_event.clone();
            rebound.plan = Arc::clone(source);
            rebound.causal_order = vec![order];
            suffix.push(rebound);
        }
        if suffix.is_empty() {
            return Err(8);
        }
        Ok(LabThreeSplice { prefix: current.admissions, suffix })
    }

    pub(super) fn lab_install_three_splice(&mut self, splice: LabThreeSplice) {
        self.trace.data.admissions.truncate(splice.prefix);
        self.trace.data.admissions.extend(splice.suffix);
        self.trace.data.lab_three_reused = true;
    }
}
