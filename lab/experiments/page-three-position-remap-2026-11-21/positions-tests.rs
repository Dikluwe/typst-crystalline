
#[cfg(test)]
mod lab_page_three_overlay_tests {
    use super::*;
    use crate::entities::layout_types::{Point, Pt};
    use std::num::NonZeroUsize;

    #[test]
    fn remapped_overlay_preserves_unrelated_positions_and_refuses_stale_values() {
        let a = Location::from_raw(12);
        let b = Location::from_raw(13);
        let absent = Location::from_raw(14);
        let value = Position {
            page: NonZeroUsize::new(3).unwrap(),
            point: Point { x: Pt(45.0), y: Pt(90.0) },
        };
        let snapshot = SealedPositions::from_runtime(HashMap::from([(a, value), (b, value)]));
        let joined = snapshot.lab_position_overlay(&[(a, value)]).unwrap();
        assert_eq!(joined.position_of(a), Some(value));
        assert_eq!(joined.position_of(b), Some(value));
        let stale = Position { page: NonZeroUsize::new(4).unwrap(), ..value };
        assert!(snapshot.lab_position_overlay(&[(b, value), (a, stale)]).is_none());
        assert!(snapshot.lab_position_overlay(&[(absent, value)]).is_none());
        assert!(snapshot.lab_position_overlay(&[(a, value), (a, value)]).is_none());
        assert_eq!(snapshot.position_of(a), Some(value));
    }
}
