
    #[test]
    fn circle_origin_rejects_remaining_value_classes_at_causal_span() {
        use crate::entities::bytes::Bytes;
        use crate::entities::color::SpotColorant;
        use crate::entities::counter::{Counter, CounterKey};
        use crate::entities::decimal::Decimal;
        use crate::entities::dir::Dir;
        use crate::entities::duration::Duration;
        use crate::entities::element_kind::ElementKind;
        use crate::entities::func::Func;
        use crate::entities::gradient::{Gradient, GradientStop};
        use crate::entities::label::Label;
        use crate::entities::layout_types::{Align2D, Angle, Ratio};
        use crate::entities::locator::Locator;
        use crate::entities::module::Module;
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        use crate::entities::regex::Regex;
        use crate::entities::scope::Scope;
        use crate::entities::selector::Selector;
        use crate::entities::state::State;
        use crate::entities::symbol::Symbol;
        use crate::entities::tiling::{Tiling, TilingBody};
        use crate::entities::value::{IntrospectedContent, Type};
        use crate::entities::version::Version;
        use crate::entities::world_types::Datetime;

        let location = Locator::new().next();
        let black = Color::rgb(0, 0, 0);
        let cases = [
            (Value::Module(Module::new("fixture", Scope::new())), "module"),
            (Value::Datetime(Datetime::new_date(2026, 10, 19).unwrap()), "datetime"),
            (Value::Func(Func::native("circle", native_circle)), "function"),
            (Value::LocatedContent(IntrospectedContent::new(Content::Empty, None), location), "content"),
            (Value::Angle(Angle::deg(30.0)), "angle"),
            (Value::Color(black.clone()), "color"),
            (Value::Stroke(Arc::new(Stroke::default())), "stroke"),
            (Value::Align(Align2D::default()), "alignment"),
            (Value::Location(location), "location"),
            (Value::Gradient(Gradient::linear(vec![GradientStop::new(black.clone(), Ratio(0.0)), GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0))], Angle::deg(0.0))), "gradient"),
            (Value::Regex(Regex::new("x").unwrap()), "regex"),
            (Value::Tiling(Arc::new(Tiling::new(TilingBody::Color(black.clone())))), "tiling"),
            (Value::Bytes(Bytes::new(vec![1, 2])), "bytes"),
            (Value::Decimal(Decimal::new(2, 0)), "decimal"),
            (Value::Duration(Duration::SECOND), "duration"),
            (Value::Version(Arc::new(Version::new(0, 15, 1))), "version"),
            (Value::Selector(Selector::Kind(ElementKind::Heading)), "selector"),
            (Value::Symbol(Symbol::new("x", "x")), "symbol"),
            (Value::Args(Args::positional(vec![])), "arguments"),
            (Value::State(State { key: "fixture".into(), init: Box::new(Value::Int(0)) }), "state"),
            (Value::Counter(Counter { key: CounterKey::Str("fixture".into()) }), "counter"),
            (Value::Label(Label("fixture".into())), "label"),
            (Value::Dir(Dir::LTR), "direction"),
            (Value::Path(RootedPath::new(VirtualRoot::Project, VirtualPath::new("fixture.typ").unwrap())), "path"),
            (Value::Type(Type::Int), "type"),
            (Value::SpotColorant(SpotColorant { name: Some("fixture".into()), fallback: black }), "spot colorant"),
        ];
        assert_eq!(cases.len(), 26);
        for (value, name) in cases {
            assert_diagnostic(named(value), at(5), &format!("expected length or auto, found {name}"));
        }
    }
