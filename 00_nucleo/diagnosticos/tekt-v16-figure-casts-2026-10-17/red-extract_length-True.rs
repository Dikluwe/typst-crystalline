#![allow(dead_code,unused_variables)]
enum Value{None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((),()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(()),TektMeasureFuture}
fn check(val:&Value){match val{Value::Length(l)=>(),
Value::Float(f)=>(),
Value::Int(i)=>(),
Value::Relative(r)=>(),
Value::Ratio(_)=>(),
_=>(),
}}
