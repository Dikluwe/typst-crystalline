#![allow(dead_code,unused_variables)]
enum Type{None,Auto,Bool,Int,Float,Str,Array,Dictionary,Module,Datetime,Function,Content,Length,Ratio,Relative,Angle,Color,Stroke,Fraction,Alignment,Location,Gradient,Regex,Tiling,Bytes,Decimal,Duration,Version,Selector,Symbol,Arguments,State,Counter,Label,Direction,Path,Type,SpotColorant,TektCatalogueFuture}
fn check(subject:Type,field:&str){match (subject, field){(Type::Direction, "ltr")=>(),
(Type::Direction, "rtl")=>(),
(Type::Direction, "ttb")=>(),
(Type::Direction, "btt")=>(),
(Type::Alignment, "left")=>(),
(Type::Alignment, "center")=>(),
(Type::Alignment, "right")=>(),
(Type::Alignment, "start")=>(),
(Type::Alignment, "end")=>(),
(Type::Alignment, "top")=>(),
(Type::Alignment, "horizon")=>(),
(Type::Alignment, "bottom")=>(),
_=>(),
}}
