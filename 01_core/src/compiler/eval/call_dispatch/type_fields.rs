//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/type_fields.md
//! @prompt-hash e7619e4a
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn p1284_type_field(t: Type, field: &str) -> Option<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};

    match t {
        Type::Direction => match field {
            "ltr" => Some(Value::Dir(Dir::LTR)),
            "rtl" => Some(Value::Dir(Dir::RTL)),
            "ttb" => Some(Value::Dir(Dir::TTB)),
            "btt" => Some(Value::Dir(Dir::BTT)),
            "axis" => {
                Some(Value::Func(Func::native_with_engine("axis", direction_axis_static)))
            }
            "end" => {
                Some(Value::Func(Func::native_with_engine("end", direction_end_static)))
            }
            "inv" => {
                Some(Value::Func(Func::native_with_engine("inv", direction_inv_static)))
            }
            "sign" => {
                Some(Value::Func(Func::native_with_engine("sign", direction_sign_static)))
            }
            "start" => Some(Value::Func(Func::native_with_engine(
                "start",
                direction_start_static,
            ))),
            "from" => Some(Value::Func(Func::native_with_engine("from", direction_from))),
            "to" => Some(Value::Func(Func::native_with_engine("to", direction_to))),
            _ => None,
        },
        Type::Alignment => match field {
            "left" => Some(Value::Align(Align2D { h: Some(HAlign::Left), v: None })),
            "center" => Some(Value::Align(Align2D { h: Some(HAlign::Center), v: None })),
            "right" => Some(Value::Align(Align2D { h: Some(HAlign::Right), v: None })),
            "start" => Some(Value::Align(Align2D { h: Some(HAlign::Start), v: None })),
            "end" => Some(Value::Align(Align2D { h: Some(HAlign::End), v: None })),
            "top" => Some(Value::Align(Align2D { h: None, v: Some(VAlign::Top) })),
            "horizon" => {
                Some(Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }))
            }
            "bottom" => Some(Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) })),
            "axis" => {
                Some(Value::Func(Func::native_with_engine("axis", alignment_axis_static)))
            }
            "inv" => {
                Some(Value::Func(Func::native_with_engine("inv", alignment_inv_static)))
            }
            _ => None,
        },
        Type::Angle => match field {
            "deg" => Some(Value::Func(Func::native("deg", angle_deg_static))),
            "rad" => Some(Value::Func(Func::native("rad", angle_rad_static))),
            _ => None,
        },
        Type::Function => match field {
            "with" => Some(Value::Func(Func::native("with", function_with_static))),
            "where" => {
                Some(Value::Func(Func::native("where", bindings::native_function_where)))
            }
            _ => None,
        },
        Type::Duration => match field {
            "days" => {
                Some(Value::Func(Func::native_with_engine("days", duration_days_static)))
            }
            "hours" => Some(Value::Func(Func::native_with_engine(
                "hours",
                duration_hours_static,
            ))),
            "minutes" => Some(Value::Func(Func::native_with_engine(
                "minutes",
                duration_minutes_static,
            ))),
            "seconds" => Some(Value::Func(Func::native_with_engine(
                "seconds",
                duration_seconds_static,
            ))),
            "weeks" => Some(Value::Func(Func::native_with_engine(
                "weeks",
                duration_weeks_static,
            ))),
            _ => None,
        },
        Type::Length => match field {
            "pt" => Some(Value::Func(Func::native_with_engine("pt", length_pt_static))),
            "mm" => Some(Value::Func(Func::native_with_engine("mm", length_mm_static))),
            "cm" => Some(Value::Func(Func::native_with_engine("cm", length_cm_static))),
            "inches" => Some(Value::Func(Func::native_with_engine(
                "inches",
                length_inches_static,
            ))),
            "to-absolute" => Some(Value::Func(Func::native_with_engine(
                "to-absolute",
                length_to_absolute_static,
            ))),
            _ => None,
        },
        Type::Selector => match field {
            "and" => {
                Some(Value::Func(Func::native_with_engine("and", selector_and_static)))
            }
            "or" => Some(Value::Func(Func::native_with_engine("or", selector_or_static))),
            "within" => Some(Value::Func(Func::native_with_engine(
                "within",
                selector_within_static,
            ))),
            "before" => Some(Value::Func(Func::native_with_engine(
                "before",
                selector_before_static,
            ))),
            "after" => Some(Value::Func(Func::native_with_engine(
                "after",
                selector_after_static,
            ))),
            _ => None,
        },
        Type::State => match field {
            "at" => Some(Value::Func(Func::native_with_engine("at", state_at_static))),
            "final" => {
                Some(Value::Func(Func::native_with_engine("final", state_final_static)))
            }
            "get" => Some(Value::Func(Func::native_with_engine("get", state_get_static))),
            "update" => {
                Some(Value::Func(Func::native_with_engine("update", state_update_static)))
            }
            _ => None,
        },
        Type::Location => match field {
            "page" => {
                Some(Value::Func(Func::native_with_engine("page", location_page_static)))
            }
            "page-numbering" => Some(Value::Func(Func::native_with_engine(
                "page-numbering",
                location_page_numbering_static,
            ))),
            "position" => Some(Value::Func(Func::native_with_engine(
                "position",
                location_position_static,
            ))),
            _ => None,
        },
        // These types have no members owned by this catalogue.
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
        | Type::SpotColorant => None,
    }
}

#[cfg(test)]
mod catalogue_tests {
    use super::*;

    #[test]
    fn catalogue_constants_and_functions_with_same_field_stay_type_specific() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::{Align2D, HAlign};
        assert_eq!(p1284_type_field(Type::Direction, "ltr"), Some(Value::Dir(Dir::LTR)));
        assert_eq!(
            p1284_type_field(Type::Alignment, "end"),
            Some(Value::Align(Align2D { h: Some(HAlign::End), v: None }))
        );
        assert!(matches!(p1284_type_field(Type::Direction, "end"), Some(Value::Func(_))));
        assert!(p1284_type_field(Type::Direction, "deg").is_none());
        assert!(
            p1284_type_field(Type::Str, "len").is_none(),
            "collection catalogue has its own owner"
        );
    }

    #[test]
    fn catalogue_native_identity_and_location_discovery_are_preserved() {
        for (kind, field, expected) in [
            (Type::Angle, "rad", Func::native("rad", angle_rad_static)),
            (
                Type::Function,
                "where",
                Func::native("where", bindings::native_function_where),
            ),
            (
                Type::Duration,
                "days",
                Func::native_with_engine("days", duration_days_static),
            ),
            (
                Type::Location,
                "page",
                Func::native_with_engine("page", location_page_static),
            ),
            (
                Type::Location,
                "page-numbering",
                Func::native_with_engine(
                    "page-numbering",
                    location_page_numbering_static,
                ),
            ),
            (
                Type::Location,
                "position",
                Func::native_with_engine("position", location_position_static),
            ),
        ] {
            let Some(Value::Func(found)) = p1284_type_field(kind, field) else {
                panic!("missing {}.{field}", kind.name())
            };
            assert_eq!(found.name(), Some(field));
            use crate::entities::func::FuncRepr;
            match (found.repr(), expected.repr()) {
                (FuncRepr::Native(a), FuncRepr::Native(b)) => {
                    assert!(std::ptr::fn_addr_eq(a.call, b.call))
                }
                (FuncRepr::NativeWithEngine(a), FuncRepr::NativeWithEngine(b)) => {
                    assert!(std::ptr::fn_addr_eq(a.call, b.call))
                }
                _ => panic!("native ABI changed"),
            }
            assert!(p1284_type_field(kind, "nonexistent").is_none());
        }
    }
}
