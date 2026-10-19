#![allow(dead_code,unused_variables)]
enum Value{None,Bool(()),Int(()),Float(()),Str(()),Array(()),Dict(()),Module(()),Datetime(()),Func(()),Content(()),LocatedContent((),()),Auto,Length(()),Relative(()),Ratio(()),Angle(()),Color(()),Stroke(()),Fraction(()),Align(()),Location(()),Gradient(()),Regex(()),Tiling(()),Bytes(()),Decimal(()),Duration(()),Version(()),Selector(()),Symbol(()),Args(()),State(()),Counter(()),Label(()),Dir(()),Path(()),Type(()),SpotColorant(())}
fn check(subject:Value,field:&str){match subject{Value::Array(arr)=>(),
Value::Dict(dict)=>(),
Value::Str(s)=>(),
Value::Bytes(bytes)=>(),
Value::Args(arguments)=>(),
Value::None
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Float(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(..)
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..)=>(),
}}
