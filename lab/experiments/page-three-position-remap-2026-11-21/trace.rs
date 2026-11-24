
impl PageRunningTrace {
    fn lab_suffix_indices(&self) -> Vec<usize> {
        let events = &self.data.admissions;
        let mut suffix = events.iter().enumerate()
            .filter_map(|(index, event)| (event.page.get() >= 3).then_some(index))
            .collect::<Vec<_>>();
        suffix.sort_by(|&a, &b| {
            let (a, b) = (&events[a], &events[b]);
            (a.page, a.section.order(), &a.causal_order)
                .cmp(&(b.page, b.section.order(), &b.causal_order))
        });
        suffix
    }

    pub(super) fn lab_remap_suffix_positions(
        &mut self,
        old: &Self,
    ) -> (Option<SealedPositions>, [u64; 14]) {
        let mut receipt = [0; 14];
        receipt[1] = old.data.pages.total_pages().map_or(0, |n| n.get()) as u64;
        receipt[2] = self.data.pages.total_pages().map_or(0, |n| n.get()) as u64;
        receipt[3] = old.data.positions.len() as u64;
        receipt[4] = self.data.positions.len() as u64;
        receipt[12] = old.data.positions.lab_position_count_on_or_after(3) as u64;
        receipt[13] = self.data.positions.lab_position_count_on_or_after(3) as u64;
        let result = self.lab_try_suffix_positions(old, &mut receipt);
        match result {
            Ok(positions) => (Some(positions), receipt),
            Err(reason) => {
                receipt[0] = reason;
                (None, receipt)
            }
        }
    }

    fn lab_try_suffix_positions(
        &mut self,
        old: &Self,
        receipt: &mut [u64; 14],
    ) -> Result<SealedPositions, u64> {
        use std::collections::HashMap;

        fn insert_pair<K: Copy + Eq + std::hash::Hash>(
            forward: &mut HashMap<K, K>,
            reverse: &mut HashMap<K, K>,
            from: K,
            to: K,
        ) -> bool {
            if forward.get(&from).is_some_and(|&mapped| mapped != to)
                || reverse.get(&to).is_some_and(|&mapped| mapped != from)
            {
                return false;
            }
            forward.insert(from, to);
            reverse.insert(to, from);
            true
        }

        if !old.data.complete || !self.data.complete
            || !old.data.requests.is_empty() || !self.data.requests.is_empty()
            || !old.data.inactive_repeat_bodies.is_empty()
            || !self.data.inactive_repeat_bodies.is_empty()
        {
            return Err(1);
        }
        if !old.data.pages.same_scalar_snapshot(&self.data.pages) {
            return Err(2);
        }
        let old_indices = old.lab_suffix_indices();
        let new_indices = self.lab_suffix_indices();
        receipt[5] = old_indices.len() as u64;
        receipt[6] = new_indices.len() as u64;
        if old_indices.len() != new_indices.len() || old_indices.is_empty() {
            return Err(3);
        }
        let (mut packets, mut reverse_packets) = (HashMap::new(), HashMap::new());
        let (mut locations, mut reverse_locations) = (HashMap::new(), HashMap::new());
        let mut rebound = Vec::with_capacity(new_indices.len());
        for (old_index, new_index) in old_indices.into_iter().zip(new_indices) {
            let a = &old.data.admissions[old_index];
            let b = &self.data.admissions[new_index];
            if !Arc::ptr_eq(&a.plan, &old.data.source)
                || !Arc::ptr_eq(&b.plan, &self.data.source)
                || a.page != b.page || a.section != b.section || a.phase != b.phase
                || a.deferred_anchor || b.deferred_anchor
            {
                return Err(4);
            }
            let (Some(from), Some(to)) = (a.plan.node(a.packet), b.plan.node(b.packet)) else {
                return Err(5);
            };
            if from.role != to.role || from.label != to.label || from.lang != to.lang
                || from.original != to.original || from.math != to.math
                || from.location.is_some() != to.location.is_some()
                || from.auxiliary_locations.len() != to.auxiliary_locations.len()
                || from.conditional_ref.is_some() != to.conditional_ref.is_some()
                || std::mem::discriminant(&from.content) != std::mem::discriminant(&to.content)
                || !insert_pair(&mut packets, &mut reverse_packets, a.packet, b.packet)
            {
                return Err(5);
            }
            let old_locations = from.location.iter().chain(&from.auxiliary_locations);
            let new_locations = to.location.iter().chain(&to.auxiliary_locations);
            for (&from, &to) in old_locations.zip(new_locations) {
                if !insert_pair(&mut locations, &mut reverse_locations, from, to) {
                    return Err(6);
                }
            }
            receipt[7] += u64::from(a.packet != b.packet);
            rebound.push((new_index, Admission {
                plan: Arc::clone(&self.data.source),
                packet: b.packet,
                phase: a.phase,
                page: a.page,
                section: a.section,
                causal_order: b.causal_order.clone(),
                deferred_anchor: b.deferred_anchor,
            }));
        }

        receipt[8] = locations.len() as u64;
        receipt[9] = locations.iter().filter(|(a, b)| a != b).count() as u64;
        let mut replacements = Vec::new();
        for (from, to) in locations {
            let old_position = old.data.positions.position_of(from);
            let new_position = self.data.positions.position_of(to);
            match (old_position, new_position) {
                (Some(a), Some(b)) if a.page.get() < 3 && a == b => {}
                (Some(a), Some(b)) if a.page.get() >= 3 && a == b => {
                    replacements.push((to, a));
                }
                (None, None) => {}
                _ => return Err(7),
            }
        }
        receipt[10] = replacements.len() as u64;
        if replacements.is_empty() || receipt[10] > receipt[4] {
            return Err(8);
        }
        let positions = self.data.positions.lab_position_overlay(&replacements).ok_or(9_u64)?;

        for (index, admission) in rebound {
            self.data.admissions[index] = admission;
            receipt[11] += 1;
        }
        self.data.positions = positions.clone();
        Ok(positions)
    }
}

#[cfg(test)]
mod lab_page_three_remap_tests {
    use super::*;
    use crate::compiler::introspect::page_running::{PlanEnvironment, PlanNode, PlanRole, EnvironmentOrigin};
    use crate::entities::{
        layout_types::{Point, Pt}, location::Location, locator::Locator, position::Position,
    };
    use std::collections::HashMap;

    fn trace(packet: usize, location: Location, y: f64) -> PageRunningTrace {
        let node = PlanNode {
            content: Content::Empty,
            children: vec![],
            location: None,
            source_parent: None,
            environment: StyleEnvironmentId(0),
            lang: None,
            label: None,
            span: Span::detached(),
            role: PlanRole::Canonical,
            auxiliary_locations: vec![],
            conditional_ref: None,
            original: false,
            math: false,
        };
        let mut nodes = vec![node; packet + 1];
        nodes[packet].location = Some(location);
        let plan = Arc::new(SourcePlan::test_source_plan(
            PacketId(0), nodes, vec![PlanEnvironment {
                parent: None, origin: EnvironmentOrigin::Root, chain: StyleChain::empty(),
            }],
            Locator::new().checkpoint(), Some(Arc::new(())), None,
        ));
        let mut trace = PageRunningTrace::new(plan.clone());
        let page = NonZeroUsize::new(3).unwrap();
        trace.data.complete = true;
        trace.data.pages = PageStore::from_runtime(page, vec![None; 3], vec![Content::Empty; 3]);
        trace.data.positions = SealedPositions::from_runtime(HashMap::from([(
            location, Position { page, point: Point { x: Pt(20.0), y: Pt(y) } },
        )]));
        trace.data.admissions.push(Admission {
            plan, packet: PacketId(packet), phase: PlanPhase::Root, page,
            section: TraceSection::Body, causal_order: vec![packet], deferred_anchor: false,
        });
        trace
    }

    #[test]
    fn changed_packet_and_location_rebind_with_the_current_causal_order() {
        let original = Location::from_raw(101);
        let current = Location::from_raw(102);
        let old = trace(1, original, 48.0);
        let mut new = trace(2, current, 48.0);
        let (positions, receipt) = new.lab_remap_suffix_positions(&old);
        assert_eq!(receipt, [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        let positions = positions.unwrap();
        assert_eq!(positions.position_of(current), old.data.positions.position_of(original));
        assert!(positions.position_of(original).is_none());
        let event = &new.data.admissions[0];
        assert_eq!(event.packet, PacketId(2));
        assert_eq!(event.causal_order, vec![2]);
        assert!(Arc::ptr_eq(&event.plan, &new.data.source));
    }

    #[test]
    fn same_page_count_with_shifted_coordinates_refuses_the_transaction() {
        let current = Location::from_raw(102);
        let old = trace(1, Location::from_raw(101), 48.0);
        let mut new = trace(2, current, 50.0);
        let witness = new.data.positions.position_of(current);
        let (positions, receipt) = new.lab_remap_suffix_positions(&old);
        assert!(positions.is_none());
        assert_eq!((receipt[0], receipt[11]), (7, 0));
        assert_eq!(new.data.positions.position_of(current), witness);
    }
}
