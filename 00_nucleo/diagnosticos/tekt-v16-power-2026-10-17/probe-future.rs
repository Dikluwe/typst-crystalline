#![allow(dead_code)]
#[derive(Clone,Copy)] struct Decimal;
type SourceResult<T> = Result<T,Vec<SourceDiagnostic>>; struct SourceDiagnostic; impl SourceDiagnostic { fn error(_:Span,_:impl Into<String>) -> Self { Self } } struct Span; impl Span { fn detached() -> Self { Self } }

enum Value { None,Bool(()),Int(i64),Float(f64),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((), ()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(Decimal),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(()),TektPowerFuture }
impl Value { fn type_name(&self) -> &str { "stub" } }
#[derive(Clone, Copy)]
enum PowerBase {
    Int(i64),
    Float(f64),
    Decimal(Decimal),
}

#[derive(Clone, Copy)]
enum PowerExponent {
    Int(i64),
    Float(f64),
}

fn power_base(value: &Value, context: &str, expected: &str) -> SourceResult<PowerBase> {
    match value {
        Value::Int(i) => Ok(PowerBase::Int(*i)),
        Value::Float(f) => Ok(PowerBase::Float(*f)),
        Value::Decimal(d) => Ok(PowerBase::Decimal(*d)),
        Value::None
        | Value::Bool(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{context}: esperava {expected}, recebeu {}", value.type_name()),
        )]),
    }
}
