#![allow(dead_code)]
enum Value {None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((), ()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(()),TektAbsFuture}
impl Value {fn type_name(&self)->&str {"stub"}}
fn label(other:&Value)->&str{match other {
                Value::Str(_) => "string",
                Value::Bool(_) => "boolean",
                Value::None
                | Value::Int(_)
                | Value::Float(_)
                | Value::Array(_)
                | Value::Dict(_)
                | Value::Module(_)
                | Value::Datetime(_)
                | Value::Func(_)
                | Value::Content(_)
                | Value::LocatedContent(_, _)
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
                | Value::Decimal(_)
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
                | Value::SpotColorant(_) => other.type_name(),
            }}
