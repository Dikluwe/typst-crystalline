//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/type_fields.md
//! @prompt-hash a6fef8af
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn p1284_type_field(t: Type, field: &str) -> Option<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};

    let constant = match (t, field) {
        (Type::Direction, "ltr") => Some(Value::Dir(Dir::LTR)),
        (Type::Direction, "rtl") => Some(Value::Dir(Dir::RTL)),
        (Type::Direction, "ttb") => Some(Value::Dir(Dir::TTB)),
        (Type::Direction, "btt") => Some(Value::Dir(Dir::BTT)),
        (Type::Alignment, "left") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Left), v: None }))
        }
        (Type::Alignment, "center") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Center), v: None }))
        }
        (Type::Alignment, "right") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Right), v: None }))
        }
        (Type::Alignment, "start") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Start), v: None }))
        }
        (Type::Alignment, "end") => {
            Some(Value::Align(Align2D { h: Some(HAlign::End), v: None }))
        }
        (Type::Alignment, "top") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Top) }))
        }
        (Type::Alignment, "horizon") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }))
        }
        (Type::Alignment, "bottom") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }))
        }
        _ => None,
    };
    if constant.is_some() {
        return constant;
    }

    let function = match (t, field) {
        (Type::Angle, "deg") => Func::native("deg", angle_deg_static),
        (Type::Angle, "rad") => Func::native("rad", angle_rad_static),
        (Type::Function, "with") => Func::native("with", function_with_static),
        (Type::Function, "where") => {
            Func::native("where", bindings::native_function_where)
        }
        (Type::Direction, "axis") => {
            Func::native_with_engine("axis", direction_axis_static)
        }
        (Type::Direction, "end") => Func::native_with_engine("end", direction_end_static),
        (Type::Direction, "inv") => Func::native_with_engine("inv", direction_inv_static),
        (Type::Direction, "sign") => {
            Func::native_with_engine("sign", direction_sign_static)
        }
        (Type::Direction, "start") => {
            Func::native_with_engine("start", direction_start_static)
        }
        (Type::Direction, "from") => Func::native_with_engine("from", direction_from),
        (Type::Direction, "to") => Func::native_with_engine("to", direction_to),
        (Type::Alignment, "axis") => {
            Func::native_with_engine("axis", alignment_axis_static)
        }
        (Type::Alignment, "inv") => Func::native_with_engine("inv", alignment_inv_static),
        (Type::Duration, "days") => {
            Func::native_with_engine("days", duration_days_static)
        }
        (Type::Duration, "hours") => {
            Func::native_with_engine("hours", duration_hours_static)
        }
        (Type::Duration, "minutes") => {
            Func::native_with_engine("minutes", duration_minutes_static)
        }
        (Type::Duration, "seconds") => {
            Func::native_with_engine("seconds", duration_seconds_static)
        }
        (Type::Duration, "weeks") => {
            Func::native_with_engine("weeks", duration_weeks_static)
        }
        (Type::Length, "pt") => Func::native_with_engine("pt", length_pt_static),
        (Type::Length, "mm") => Func::native_with_engine("mm", length_mm_static),
        (Type::Length, "cm") => Func::native_with_engine("cm", length_cm_static),
        (Type::Length, "inches") => {
            Func::native_with_engine("inches", length_inches_static)
        }
        (Type::Length, "to-absolute") => {
            Func::native_with_engine("to-absolute", length_to_absolute_static)
        }
        (Type::Selector, "and") => Func::native_with_engine("and", selector_and_static),
        (Type::Selector, "or") => Func::native_with_engine("or", selector_or_static),
        (Type::Selector, "within") => {
            Func::native_with_engine("within", selector_within_static)
        }
        (Type::Selector, "before") => {
            Func::native_with_engine("before", selector_before_static)
        }
        (Type::Selector, "after") => {
            Func::native_with_engine("after", selector_after_static)
        }
        (Type::State, "at") => Func::native_with_engine("at", state_at_static),
        (Type::State, "final") => Func::native_with_engine("final", state_final_static),
        (Type::State, "get") => Func::native_with_engine("get", state_get_static),
        (Type::State, "update") => {
            Func::native_with_engine("update", state_update_static)
        }
        (Type::Location, "page") => {
            Func::native_with_engine("page", location_page_static)
        }
        (Type::Location, "page-numbering") => {
            Func::native_with_engine("page-numbering", location_page_numbering_static)
        }
        (Type::Location, "position") => {
            Func::native_with_engine("position", location_position_static)
        }
        _ => return None,
    };
    Some(Value::Func(function))
}
