#![allow(dead_code,unused_variables)]
enum Value{None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((),()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(())}
fn check(val:&Value){match val{Value::Relative(value)=>(),
Value::Ratio(value)=>(),
Value::Length(value)=>(),
Value::Float(value)=>(),
Value::Int(value)=>(),
_=>(),
}}
