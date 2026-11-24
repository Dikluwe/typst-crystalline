p='core/compiler/layout/page_running_requests'
def update(s):
    a=s.index('    pub(super) fn lab_prepare_three_splice(')
    b=s.index('    pub(super) fn lab_install_three_splice',a)
    body=s[a:b]
    body=replace(body,'''        let source = &self.source;
        let mut suffix = Vec::new();''','''        use crate::compiler::introspect::page_running::{PlanNode, PlanRole};
        let inert_leaf = |node: &PlanNode| {
            node.role == PlanRole::Canonical && node.children.is_empty()
                && node.location.is_none() && node.label.is_none()
                && node.auxiliary_locations.is_empty() && node.conditional_ref.is_none()
                && !node.original && !node.math
                && matches!(node.content, Content::Empty | Content::Space
                    | Content::SpaceFromNewline | Content::Parbreak)
        };
        let source = &self.source;
        let last_page = old.data.pages.total_pages().ok_or(8_u64)?;
        let mut suffix = Vec::new();''')
    body=replace(body,'''                || std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)
            {
                return Err(9);
            }''','''                || std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)
            {
                if old_event.page == last_page && inert_leaf(a) && inert_leaf(b)
                    && [PlanPhase::Root, PlanPhase::PostChildren].into_iter().all(|phase|
                        !old.data.source.required(packet, phase) && !source.required(packet, phase))
                {
                    continue;
                }
                return Err(9);
            }''')
    return s[:a]+body+s[b:]
edit(p,update)
