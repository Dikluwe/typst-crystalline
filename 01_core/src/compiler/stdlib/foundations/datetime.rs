//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/datetime.md
//! @prompt-hash 1521e4a6
//! @layer L1
//! @updated 2026-08-24

use ecow::EcoString;
use time::error::{Format, InvalidFormatDescription};

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::duration::Duration;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::entities::world_types::Datetime;

type NativeResult = SourceResult<Value>;

pub fn datetime_type_field(field: &str) -> Option<Value> {
    Some(Value::Func(match field {
        "today" => Func::native("today", native_datetime_today),
        "display" => Func::native("display", native_datetime_display),
        "year" => Func::native("year", native_datetime_year),
        "month" => Func::native("month", native_datetime_month),
        "weekday" => Func::native("weekday", native_datetime_weekday),
        "day" => Func::native("day", native_datetime_day),
        "hour" => Func::native("hour", native_datetime_hour),
        "minute" => Func::native("minute", native_datetime_minute),
        "second" => Func::native("second", native_datetime_second),
        "ordinal" => Func::native("ordinal", native_datetime_ordinal),
        _ => return None,
    }))
}

fn error(args: &Args, message: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(args.span, message.into())]
}

fn self_only(args: &Args) -> Result<Datetime, Vec<SourceDiagnostic>> {
    if !args.named.is_empty() {
        return Err(error(args, "the argument `self` is positional"));
    }
    match args.items.as_slice() {
        [Value::Datetime(value)] => Ok(*value),
        [] => Err(error(args, "missing argument: self")),
        [other] => {
            Err(error(args, format!("expected datetime, found {}", other.type_name())))
        }
        _ => Err(error(args, "unexpected argument")),
    }
}

fn optional_int(value: Option<impl Into<i64>>) -> Value {
    value.map(|v| Value::Int(v.into())).unwrap_or(Value::None)
}

macro_rules! accessor {
    ($name:ident, $method:ident) => {
        fn $name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> NativeResult {
            Ok(optional_int(self_only(args)?.$method()))
        }
    };
}

accessor!(native_datetime_year, year);
accessor!(native_datetime_month, month);
accessor!(native_datetime_weekday, weekday);
accessor!(native_datetime_day, day);
accessor!(native_datetime_hour, hour);
accessor!(native_datetime_minute, minute);
accessor!(native_datetime_second, second);
accessor!(native_datetime_ordinal, ordinal);

fn native_datetime_today(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> NativeResult {
    if !args.items.is_empty() {
        return Err(error(args, "the argument `offset` is named"));
    }
    if args.named.keys().any(|name| name.as_str() != "offset") {
        return Err(error(args, "unexpected argument"));
    }
    let offset = match args.named.get("offset") {
        None | Some(Value::Auto) => None,
        Some(Value::Int(hours)) => Some(Duration::from_hours(*hours)),
        Some(Value::Duration(duration)) => Some(*duration),
        Some(other) => {
            return Err(error(
                args,
                format!(
                    "expected auto, integer, or duration, found {}",
                    other.type_name()
                ),
            ))
        }
    };
    world
        .today(offset)
        .map(Value::Datetime)
        .ok_or_else(|| error(args, "unable to get the current date"))
}

fn native_datetime_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> NativeResult {
    if !args.named.is_empty() {
        return Err(error(args, "the arguments `self` and `pattern` are positional"));
    }
    let (value, pattern) = match args.items.as_slice() {
        [Value::Datetime(value)] => (*value, None),
        [Value::Datetime(value), Value::Auto] => (*value, None),
        [Value::Datetime(value), Value::Str(pattern)] => (*value, Some(pattern.as_str())),
        [] => return Err(error(args, "missing argument: self")),
        [other, ..] if !matches!(other, Value::Datetime(_)) => {
            return Err(error(
                args,
                format!("expected datetime, found {}", other.type_name()),
            ))
        }
        [Value::Datetime(_), other] => {
            return Err(error(
                args,
                format!("expected auto or string, found {}", other.type_name()),
            ))
        }
        _ => return Err(error(args, "unexpected argument")),
    };

    let pattern = pattern.unwrap_or_else(|| match (value.year(), value.hour()) {
        (Some(_), Some(_)) => "[year]-[month]-[day] [hour]:[minute]:[second]",
        (Some(_), None) => "[year]-[month]-[day]",
        (None, Some(_)) => "[hour]:[minute]:[second]",
        (None, None) => unreachable!("Datetime invariant"),
    });
    let format = time::format_description::parse_owned::<2>(pattern)
        .map_err(|e| error(args, format_invalid_description(e)))?;
    let rendered = match (
        value.year(),
        value.month(),
        value.day(),
        value.hour(),
        value.minute(),
        value.second(),
    ) {
        (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) => {
            let date =
                time::Date::from_calendar_date(y, time::Month::try_from(m).unwrap(), d)
                    .unwrap();
            let time = time::Time::from_hms(h, min, s).unwrap();
            time::PrimitiveDateTime::new(date, time).format(&format)
        }
        (Some(y), Some(m), Some(d), None, None, None) => {
            time::Date::from_calendar_date(y, time::Month::try_from(m).unwrap(), d)
                .unwrap()
                .format(&format)
        }
        (None, None, None, Some(h), Some(min), Some(s)) => {
            time::Time::from_hms(h, min, s).unwrap().format(&format)
        }
        _ => unreachable!("Datetime invariant"),
    }
    .map_err(|e| error(args, format_error(e)))?;
    Ok(Value::Str(EcoString::from(rendered)))
}

fn format_error(error: Format) -> String {
    match error {
        Format::InvalidComponent(name) => format!("invalid component '{name}'"),
        Format::InsufficientTypeInformation { .. } => {
            "failed to format datetime (insufficient information)".into()
        }
        err => format!("failed to format datetime in the requested format ({err})"),
    }
}

fn format_invalid_description(error: InvalidFormatDescription) -> String {
    match error {
        InvalidFormatDescription::UnclosedOpeningBracket { index, .. } => {
            format!("missing closing bracket for bracket at index {index}")
        }
        InvalidFormatDescription::InvalidComponentName { name, index, .. } => {
            format!("invalid component name '{name}' at index {index}")
        }
        InvalidFormatDescription::InvalidModifier { value, index, .. } => {
            format!("invalid modifier '{value}' at index {index}")
        }
        InvalidFormatDescription::Expected { what, index, .. } => {
            format!("expected {what} at index {index}")
        }
        InvalidFormatDescription::MissingComponentName { index, .. } => {
            format!("expected component name at index {index}")
        }
        InvalidFormatDescription::MissingRequiredModifier { name, index, .. } => {
            format!("missing required modifier {name} for component at index {index}")
        }
        InvalidFormatDescription::NotSupported { context, what, index, .. } => {
            format!("{what} is not supported in {context} at index {index}")
        }
        err => format!("failed to parse datetime format ({err})"),
    }
}
