#![allow(dead_code,unused_variables)]
enum Type{None,Auto,Bool,Int,Float,Str,Array,Dictionary,Module,Datetime,Function,Content,Length,Ratio,Relative,Angle,Color,Stroke,Fraction,Alignment,Location,Gradient,Regex,Tiling,Bytes,Decimal,Duration,Version,Selector,Symbol,Arguments,State,Counter,Label,Direction,Path,Type,SpotColorant,TektCatalogueFuture}
fn check(subject:Type,field:&str){match subject{Type::Direction=>(),
Type::Alignment=>(),
Type::Angle=>(),
Type::Function=>(),
Type::Duration=>(),
Type::Length=>(),
Type::Selector=>(),
Type::State=>(),
Type::Location=>(),
Type::None
        | Type::Auto
        | Type::Bool
        | Type::Int
        | Type::Float
        | Type::Str
        | Type::Array
        | Type::Dictionary
        | Type::Module
        | Type::Datetime
        | Type::Content
        | Type::Ratio
        | Type::Relative
        | Type::Color
        | Type::Stroke
        | Type::Fraction
        | Type::Gradient
        | Type::Regex
        | Type::Tiling
        | Type::Bytes
        | Type::Decimal
        | Type::Version
        | Type::Symbol
        | Type::Arguments
        | Type::Counter
        | Type::Label
        | Type::Path
        | Type::Type
        | Type::SpotColorant=>(),
}}
