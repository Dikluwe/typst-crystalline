
#[derive(Debug, Clone, Copy)]
struct LabPlanMark {
    packet: usize,
    slots: usize,
    environments: usize,
}

#[derive(Debug, Clone)]
struct LabPlanBlock {
    donor: Arc<SourcePlan>,
    anchor: PacketId,
    end: usize,
    tail: Vec<PlanNode>,
}

#[inline(never)]
fn lab_block_receipt(reused: usize, reason: usize) {
    std::hint::black_box((reused, reason));
}

fn lab_block_content(old: &Content, current: &Content) -> Result<(), usize> {
    if lab_same_leaf(old, current) {
        return Ok(());
    }
    match (old, current) {
        (Content::Source(a), Content::Source(b)) if a.span == b.span => {
            lab_block_content(&a.body, &b.body)
        }
        (Content::Source(_), Content::Source(_)) => Err(11),
        (Content::Located(a), Content::Located(b)) if a.location == b.location => {
            lab_block_content(&a.body, &b.body)
        }
        (Content::Located(_), Content::Located(_)) => Err(12),
        (Content::Realized(a), Content::Realized(b)) => {
            lab_block_content(&a.original, &b.original)?;
            lab_block_content(&a.visual, &b.visual)
        }
        (Content::Styled(a), Content::Styled(b)) if a.styles == b.styles => {
            lab_block_content(&a.body, &b.body)
        }
        (Content::Styled(_), Content::Styled(_)) => Err(13),
        (Content::Sequence(a), Content::Sequence(b)) if a.len() == b.len() => {
            a.iter().zip(b.iter()).try_for_each(|(a, b)| lab_block_content(a, b))
        }
        (Content::Sequence(_), Content::Sequence(_)) => Err(14),
        (Content::Par { body: a }, Content::Par { body: b }) => lab_block_content(a, b),
        (Content::Label(a), Content::Label(b)) if a.name == b.name && a.auto == b.auto => {
            lab_block_content(&a.body, &b.body)
        }
        (Content::Heading(a), Content::Heading(b))
            if a.level == b.level && a.outlined == b.outlined
                && a.bookmarked == b.bookmarked && a.set_fields == b.set_fields =>
        {
            lab_block_content(&a.body, &b.body)
        }
        (Content::Ref(a), Content::Ref(b)) if a.name == b.name && a.form == b.form => {
            match (&a.supplement, &b.supplement) {
                (None, None) => Ok(()),
                (Some(a), Some(b)) => lab_block_content(a, b),
                _ => Err(15),
            }
        }
        (Content::Link(a), Content::Link(b)) if a.url == b.url => {
            lab_block_content(&a.body, &b.body)
        }
        (Content::ContextBlock(a), Content::ContextBlock(b)) if a.id == b.id => {
            use crate::compiler::realize::reference_supplements::is_reference_supplement_producer;
            use crate::entities::{func::FuncRepr, value::Value};
            if !is_reference_supplement_producer(a) || !is_reference_supplement_producer(b) {
                return Err(16);
            }
            let (FuncRepr::With(a), FuncRepr::With(b)) = (a.closure.repr(), b.closure.repr()) else {
                return Err(16);
            };
            if a.1.span != b.1.span {
                return Err(16);
            }
            let ([Value::Content(a)], [Value::Content(b)]) =
                (a.1.items.as_slice(), b.1.items.as_slice()) else { return Err(16) };
            lab_block_content(a, b)
        }
        (Content::ContextBlock(_), Content::ContextBlock(_)) => Err(17),
        (Content::Text(_), Content::Text(_)) => Err(18),
        _ => Err(19),
    }
}

impl PlanBuilder<'_> {
    fn lab_terminal_sequence(
        &mut self,
        anchor: PacketId,
        parts: &[Content],
        context: &PlanBuildContext,
    ) -> SourceResult<()> {
        use crate::compiler::realize::context_occurrences::{
            lab_index_slot_cursor, lab_replay_index_slots,
        };
        let PlanBuildContext { environment, source_parent, span, .. } = *context;
        let donor = self.lab_prior.and_then(|(donor, frontier)| {
            let proof = (|| {
                let old = donor.node(anchor).ok_or(20_usize)?;
                let Content::Sequence(prior_parts) = &old.content else { return Err(20) };
                if donor.lab_suffix.is_some() || !donor.page_frontier(anchor)
                    || old.children.len() != prior_parts.len()
                    || old.environment != environment || old.source_parent != source_parent
                    || old.span != span || old.role != PlanRole::Canonical || old.original
                    || old.math || old.conditional_ref.is_some()
                {
                    return Err(21);
                }
                if !donor.environment(environment).ok_or(22_usize)?.chain.raw_deltas()
                    .eq(self.environments[environment.0].chain.raw_deltas())
                {
                    return Err(22);
                }
                let common_len = parts.len().min(prior_parts.len());
                let shared_end = if parts.len() == prior_parts.len() {
                    common_len
                } else {
                    common_len.saturating_sub(8)
                };
                if parts.len().abs_diff(prior_parts.len()) > 1
                    || !parts[shared_end..].iter().chain(&prior_parts[shared_end..])
                        .all(|part| lab_same_leaf(part, part))
                {
                    return Err(23);
                }
                let index = old.children.partition_point(|(_, packet)| packet.0 < frontier);
                let (edge, first) = *old.children.get(index).ok_or(24_usize)?;
                if edge != PlanEdge::SequencePart(index) || shared_end.saturating_sub(index) < 128 {
                    return Err(24);
                }
                let mark = donor.lab_marks.get(&anchor).and_then(|marks| marks.get(index)).ok_or(25_usize)?;
                if mark.packet != first.0 {
                    return Err(25);
                }
                let end = old.children.get(shared_end).map_or(donor.nodes.len(), |(_, packet)| packet.0);
                Ok((donor, old, prior_parts, index, *mark, shared_end, end))
            })();
            match proof {
                Ok(value) => Some(value),
                Err(reason) => { lab_block_receipt(0, reason); None }
            }
        });

        let mut marks = Vec::with_capacity(parts.len());
        for (index, part) in parts.iter().enumerate() {
            let cursor = lab_index_slot_cursor(self.ctx);
            if let Some((donor, old, prior_parts, start, mark, shared_end, end)) = donor {
                if index == start {
                    let check = (|| {
                        if self.nodes.len() != mark.packet
                            || self.environments.len() != mark.environments || cursor != Some(mark.slots)
                        {
                            return Err(1);
                        }
                        let slots = donor.lab_slots.as_deref()
                            .and_then(|slots| slots.get(mark.slots..)).ok_or(2_usize)?;
                        if old.children[index..shared_end].iter().enumerate()
                            .any(|(offset, (edge, _))| *edge != PlanEdge::SequencePart(index + offset))
                        {
                            return Err(3);
                        }
                        prior_parts[index..shared_end].iter().zip(&parts[index..shared_end])
                            .try_for_each(|(a, b)| lab_block_content(a, b))?;
                        if !lab_replay_index_slots(self.ctx, slots) {
                            return Err(4);
                        }
                        Ok(())
                    })();
                    match check {
                        Ok(()) => {
                            let reused = end - mark.packet;
                            let children = &mut self.nodes[anchor.0].children;
                            children.extend_from_slice(&old.children[index..shared_end]);
                            let tail = parts[shared_end..].iter().enumerate().map(|(offset, part)| {
                                children.push((PlanEdge::SequencePart(shared_end + offset), PacketId(end + offset)));
                                PlanNode {
                                    content: part.clone(), children: Vec::new(), location: None,
                                    source_parent, environment, lang: donor.nodes[anchor.0].lang,
                                    label: None, span, role: PlanRole::Canonical,
                                    auxiliary_locations: Vec::new(), conditional_ref: None,
                                    original: false, math: false,
                                }
                            }).collect();
                            self.lab_suffix = Some(LabPlanBlock {
                                donor: self.lab_donor.as_ref().unwrap().clone(),
                                anchor, end, tail,
                            });
                            lab_block_receipt(reused, 0);
                            return Ok(());
                        }
                        Err(reason) => lab_block_receipt(0, reason),
                    }
                }
            }
            marks.push(LabPlanMark {
                packet: self.nodes.len(),
                slots: cursor.unwrap_or(usize::MAX),
                environments: self.environments.len(),
            });
            let terminal = self.lab_terminal;
            self.lab_terminal &= index + 1 == parts.len();
            let packet = self.build(part, PlanBuildContext {
                environment, source_parent, span, label: None, forced: None,
                role: PlanRole::Canonical, math: false,
            })?;
            self.lab_terminal = terminal;
            self.nodes[anchor.0].children.push((PlanEdge::SequencePart(index), packet));
        }
        self.lab_marks.insert(anchor, marks);
        Ok(())
    }
}
