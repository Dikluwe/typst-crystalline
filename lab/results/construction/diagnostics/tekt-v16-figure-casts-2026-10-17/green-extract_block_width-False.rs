#![allow(dead_code,unused_variables)]
enum Value{None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((),()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(())}
fn check(val:&Value){match val{Value::Relative(value)=>(),
Value::Ratio(value)=>(),
Value::Length(value)=>(),
Value::Float(value)=>(),
Value::Int(value)=>(),
Value::None
        | Value::Bool(..)
        | Value::Str(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..)=>(),
}}
