
fn lab_same_heading(
    expected: &crate::entities::elements::heading::HeadingElem,
    current: &crate::entities::elements::heading::HeadingElem,
) -> bool {
    use crate::entities::content::Content;
    fn same(a: &Content, b: &Content) -> bool {
        if let (Content::Text(a), Content::Text(b)) = (a,b) {
            return a == b;
        }
        if let (Content::Sequence(a), Content::Sequence(b)) = (a,b) {
            return a.len() == b.len() && a.iter().zip(b.iter()).all(|(a,b)| same(a,b));
        }
        if let (Content::Strong(a), Content::Strong(b)) = (a,b) {
            return same(&a.body, &b.body);
        }
        if let (Content::Emph(a), Content::Emph(b)) = (a,b) {
            return same(&a.body, &b.body);
        }
        matches!((a,b), (Content::Empty, Content::Empty) | (Content::Space, Content::Space)
            | (Content::SpaceFromNewline, Content::SpaceFromNewline)
            | (Content::Parbreak, Content::Parbreak))
    }
    expected.level == current.level && expected.outlined == current.outlined
        && expected.bookmarked == current.bookmarked && expected.set_fields == current.set_fields
        && heading_body_is_closed(&current.body) && same(&expected.body, &current.body)
}
