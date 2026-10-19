
    #[test]
    fn circle_origin_native_and_generic_constructor_invariants() {
        use crate::entities::elements::shape::ShapeOrigin;
        let mut inputs = vec![Args::positional(vec![])];
        inputs.extend([Value::Auto, Value::Int(2), Value::Float(2.0), Value::Length(Length::em(2.0)), Value::Length(Length::pt(2.0) + Length::em(1.0))].map(named));
        for (index, args) in inputs.into_iter().enumerate() {
            let Content::Shape(shape) = content(call(native_circle, args).unwrap()) else { panic!("shape") };
            assert_eq!(shape.origin, ShapeOrigin::Circle);
            assert_eq!(shape.kind, ShapeKind::Ellipse);
            assert_eq!(shape.width, shape.height);
            if index == 0 { assert!(shape.width.is_none()); }
            else { assert!(matches!(shape.width.as_deref(), Some(Value::Length(_)))); }
        }
        let Content::Shape(shape) = content(ellipse(Value::Length(Length::em(4.0)))) else { panic!("shape") };
        assert_eq!(shape.origin, ShapeOrigin::Geometry);
        for generic in [
            Content::shape(ShapeKind::Ellipse, None, None, None, None),
            Content::shape_with_radius(ShapeKind::Rect, None, None, None, None, Corners::uniform(Length::ZERO)),
        ] {
            let Content::Shape(shape) = generic else { panic!("shape") };
            assert_eq!(shape.origin, ShapeOrigin::Geometry);
        }
    }

    #[test]
    fn circle_origin_terminal_maps_and_clone_retain_origin() {
        use crate::entities::elements::{Element, shape::ShapeOrigin};
        let Content::Shape(shape) = content(circle(Value::Length(Length::em(2.0)))) else { panic!("shape") };
        let copied = shape.as_ref().clone();
        assert_eq!(copied.origin, ShapeOrigin::Circle);
        let mut content_visits = 0;
        let mapped = copied.map_content(&mut |_| { content_visits += 1; Ok(None) }).unwrap();
        let mut text_visits = 0;
        let text_mapped = copied.map_text(&mut |_| { text_visits += 1; "changed".into() });
        for mapped in [mapped, text_mapped] {
            let Content::Shape(mapped) = mapped else { panic!("shape") };
            assert_eq!(mapped.origin, ShapeOrigin::Circle);
            assert_eq!(*mapped, copied);
        }
        assert_eq!((content_visits, text_visits), (0, 0));
    }

    #[test]
    fn circle_origin_structural_relation_and_hash_include_only_changed_origin() {
        use crate::entities::elements::shape::ShapeOrigin;
        use std::hash::{Hash, Hasher};
        let Content::Shape(shape) = content(circle(Value::Int(2))) else { panic!("shape") };
        let circle = shape.as_ref().clone();
        let mut geometry = circle.clone();
        geometry.origin = ShapeOrigin::Geometry;
        assert_ne!(circle, geometry);
        let digest = |shape: &ShapeElem| {
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            shape.hash(&mut hash);
            hash.finish()
        };
        assert_eq!(digest(&circle), digest(&circle.clone()));
        assert_ne!(digest(&circle), digest(&geometry), "this controlled structural pair must include origin; this is not a hash-uniqueness claim");
        let a = crate::compiler::layout::layout(&Content::Shape(Arc::new(circle)));
        let b = crate::compiler::layout::layout(&Content::Shape(Arc::new(geometry)));
        let measure = |document: &crate::entities::layout_types::PagedDocument| {
            let mut shapes = vec![];
            for page in &document.pages { emitted_shapes(&page.items, &mut shapes); }
            shapes
        };
        assert_eq!(measure(&a), measure(&b), "render geometry is selected by kind");
    }

    #[test]
    fn circle_origin_corrects_float_ellipse_alias_in_language_compositions() {
        let a = circle(Value::Length(Length::pt(2.0)));
        let other = ellipse(Value::Float(4.0));
        assert_language(&a, &other, false);
        assert_language(&Value::Array(vec![a.clone()]), &Value::Array(vec![other.clone()]), false);
        assert_language(&Value::Dict([("shape".into(), a.clone())].into_iter().collect()), &Value::Dict([("shape".into(), other.clone())].into_iter().collect()), false);
        assert_eq!(apply_binary(BinOp::In, a.clone(), Value::Array(vec![other.clone()])), Ok(Value::Bool(false)));
        assert_eq!(apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![other.clone()])), Ok(Value::Bool(true)));
        for wrap in 0..4 {
            let wrap = |body: Content| match wrap {
                0 => body.with_source_span(at(22)),
                1 => Content::strong(body),
                2 => Content::emph(body),
                _ => Content::styled(Box::new(body), Styles::from_iter([Style::Size(Length::pt(50.0))])),
            };
            assert_language(&Value::Content(wrap(content(a.clone()))), &Value::Content(wrap(content(other.clone()))), false);
        }
        assert_language(&Value::Content(content(a.clone()).with_source_span(at(22))), &Value::Content(content(a).with_source_span(at(30))), true);
    }
