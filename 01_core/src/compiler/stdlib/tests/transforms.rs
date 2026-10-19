//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/transforms.md
//! @prompt-hash badc7a29
//! @layer L1
//! @updated 2026-09-26

use super::*;
use crate::entities::layout_types::{Angle, Length, Ratio};

#[test]
fn typed_transform_identity_does_not_collapse_at_zero() {
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("x"))]);
    let rotate = native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    let scale = native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_ne!(rotate, scale);
}

#[test]
fn typed_transform_explicit_false_is_not_omitted() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    let implicit = native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    args.named.insert("reflow".into(), Value::Bool(false));
    let explicit = native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_ne!(implicit, explicit);
}

#[test]
fn typed_transform_move_preserves_relative_and_em() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    let dx =
        Value::Relative(crate::entities::rel::Rel { rel: 0.5, abs: Length::em(1.0) });
    args.named.insert("dx".into(), dx.clone());
    let Value::Content(c) =
        native_move(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    assert_eq!(c.get_field("dx"), Some(dx));
    assert_eq!(c.get_field("dy"), None);
    assert_eq!(c.get_field("body"), Some(Value::Content(Content::text("x"))));
}

#[test]
fn typed_transform_scale_named_x_preserves_omitted_y() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("x".into(), Value::Ratio(Ratio(1.5)));
    let Value::Content(c) =
        native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    assert_eq!(c.get_field("x"), Some(Value::Ratio(Ratio(1.5))));
    assert_eq!(c.get_field("y"), None);
}

#[test]
fn typed_transform_rejects_silently_ignored_arguments() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("angle".into(), Value::Angle(Angle::deg(23.0)));
    assert!(native_rotate(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    args.named.clear();
    args.items.insert(0, Value::Int(2));
    assert!(native_scale(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn typed_transform_string_body_and_factor_after_it() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("x".into()), Value::Angle(Angle::deg(23.0))]);
    let Value::Content(c) =
        native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    assert_eq!(c.plain_text(), "x");
    assert_eq!(c.get_field("angle"), Some(Value::Angle(Angle::deg(23.0))));
}

#[test]
fn typed_transform_skew_preserves_axes_and_large_angles() {
    null_ctx!(ctx);
    for (ax, ay) in [(30.0, 0.0), (0.0, 30.0), (15.0, 45.0), (90.0, 100.0)] {
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Angle(Angle::deg(ax)));
        args.named.insert("ay".into(), Value::Angle(Angle::deg(ay)));
        let Value::Content(c) =
            native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
        else {
            panic!("content")
        };
        assert_eq!(c.get_field("ax"), Some(Value::Angle(Angle::deg(ax))));
        assert_eq!(c.get_field("ay"), Some(Value::Angle(Angle::deg(ay))));
    }
}

#[test]
fn typed_transform_factor_sets_axes_before_named_override() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Ratio(Ratio(1.5)), Value::Content(Content::text("x"))]);
    args.named.insert("x".into(), Value::Ratio(Ratio(0.5)));
    let Value::Content(c) =
        native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    assert_eq!(c.get_field("x"), Some(Value::Ratio(Ratio(0.5))));
    assert_eq!(c.get_field("y"), Some(Value::Ratio(Ratio(1.5))));
}

#[test]
fn typed_transform_two_auto_remain_inspectable_before_layout() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::None]);
    args.named.insert("x".into(), Value::Auto);
    args.named.insert("y".into(), Value::Auto);
    let Value::Content(c) =
        native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    assert_eq!(c.get_field("x"), Some(Value::Auto));
    assert_eq!(c.get_field("y"), Some(Value::Auto));
    assert_eq!(c.get_field("body"), Some(Value::Content(Content::Empty)));
}

#[test]
fn typed_transform_invalid_types_and_surplus_are_errors() {
    null_ctx!(ctx);
    for value in [Value::Int(1), Value::Float(1.0), Value::Str("bad".into())] {
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("dx".into(), value.clone());
        assert!(native_move(&mut ctx, &args, &null_world(), test_file_id()).is_err());
        args.named.clear();
        args.named.insert("ax".into(), value);
        assert!(native_skew(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }
    let args = p(vec![
        Value::Ratio(Ratio(2.0)),
        Value::Ratio(Ratio(0.5)),
        Value::Content(Content::text("x")),
    ]);
    assert!(native_scale(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    assert!(native_skew(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
}

#[test]
fn typed_transform_type_errors_keep_the_value_span() {
    null_ctx!(ctx);
    use crate::entities::args::ArgOccurrence;
    use crate::entities::span::Span;
    let file = test_file_id();
    let span = Span::from_range(file, 2..20);
    let value_span = Span::from_range(file, 12..13);
    let args = Args::from_occurrences(
        span,
        vec![
            ArgOccurrence {
                name: Some("dx".into()),
                value: Value::Int(2),
                span: Span::from_range(file, 8..13),
                value_span,
            },
            ArgOccurrence {
                name: None,
                value: Value::Content(Content::text("x")),
                span,
                value_span: span,
            },
        ],
    );
    let errors = native_move(&mut ctx, &args, &null_world(), file).unwrap_err();
    assert_eq!(errors[0].message, "expected relative length, found integer");
    assert_eq!(errors[0].span, value_span);
}

#[test]
fn typed_transform_maps_preserve_operation_and_presence() {
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Content(Content::text("x")), Value::Angle(Angle::deg(0.0))]);
    args.named.insert("reflow".into(), Value::Bool(false));
    let Value::Content(c) =
        native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("content")
    };
    let mapped = c.map_text(&mut |_| "y".into());
    assert_eq!(mapped.elem_name(), "rotate");
    assert_eq!(mapped.get_field("angle"), Some(Value::Angle(Angle::deg(0.0))));
    assert_eq!(mapped.get_field("reflow"), Some(Value::Bool(false)));
    assert_eq!(mapped.get_field("origin"), None);
    assert_eq!(mapped.plain_text(), "y");
}
