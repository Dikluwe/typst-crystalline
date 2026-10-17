#![allow(dead_code,unused_variables)]
enum Type{None,Auto,Bool,Int,Float,Str,Array,Dictionary,Module,Datetime,Function,Content,Length,Ratio,Relative,Angle,Color,Stroke,Fraction,Alignment,Location,Gradient,Regex,Tiling,Bytes,Decimal,Duration,Version,Selector,Symbol,Arguments,State,Counter,Label,Direction,Path,Type,SpotColorant}
fn check(subject:Type,field:&str){match subject{Type::Array=>(),
Type::Dictionary=>(),
Type::Str=>(),
Type::Bytes=>(),
Type::Arguments=>(),
Type::None
        | Type::Auto
        | Type::Bool
        | Type::Int
        | Type::Float
        | Type::Module
        | Type::Datetime
        | Type::Function
        | Type::Content
        | Type::Length
        | Type::Ratio
        | Type::Relative
        | Type::Angle
        | Type::Color
        | Type::Stroke
        | Type::Fraction
        | Type::Alignment
        | Type::Location
        | Type::Gradient
        | Type::Regex
        | Type::Tiling
        | Type::Decimal
        | Type::Duration
        | Type::Version
        | Type::Selector
        | Type::Symbol
        | Type::State
        | Type::Counter
        | Type::Label
        | Type::Direction
        | Type::Path
        | Type::Type
        | Type::SpotColorant=>(),
}}
