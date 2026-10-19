
#[cfg(test)]
mod circle_origin_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::shape::ShapeOrigin;
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::Length;
    use crate::entities::value::Value;

    #[test]
    fn circle_origin_observation_compares_constructor_identity_and_existing_fields() {
        let Content::Shape(geometry) = Content::shape(
            ShapeKind::Ellipse,
            Some(Box::new(Value::Length(Length::pt(4.0)))),
            Some(Box::new(Value::Length(Length::pt(4.0)))),
            None,
            None,
        ) else { panic!("shape") };
        let mut circle = geometry.as_ref().clone();
        circle.origin = ShapeOrigin::Circle;
        assert_eq!(circle.observation_relation(&circle.clone()), ObservationRelation::Same);
        assert_eq!(circle.observation_relation(geometry.as_ref()), ObservationRelation::Different);
        assert_eq!(geometry.as_ref().observation_relation(&circle), ObservationRelation::Different);
        let mut different_radius = circle.clone();
        different_radius.width = Some(Box::new(Value::Length(Length::pt(6.0))));
        different_radius.height = different_radius.width.clone();
        assert_eq!(circle.observation_relation(&different_radius), ObservationRelation::Different);
    }
}
