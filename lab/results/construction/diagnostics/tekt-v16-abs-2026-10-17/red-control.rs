#![allow(dead_code)]
enum Value {None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((), ()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(())}
impl Value {fn type_name(&self)->&str {"stub"}}
fn label(other:&Value)->&str{match other {
                Value::Str(_) => "string",
                Value::Bool(_) => "boolean",
                _ => other.type_name(),
            }}
