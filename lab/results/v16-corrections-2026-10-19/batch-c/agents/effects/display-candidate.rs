/// Display a value at markup, lexical code-tail and math insertion boundaries.
/// Markup handles its own None/Label semantics before this shared adapter.
/// Residual repr presentation is explicit pending rich-value display completion.
pub(crate) fn value_to_display_content(value: Value) -> Option<Content> {
    match value {
        Value::Content(content) => Some(content),
        Value::LocatedContent(content, _) => Some(content.into_content()),
        Value::Str(text) => Some(Content::Text(text)),
        Value::Auto => Some(Content::raw("auto", Some("typc".into()), false)),
        Value::Bool(value) => Some(Content::raw(
            if value { "true" } else { "false" }, Some("typc".into()), false,
        )),
        Value::Symbol(symbol) => Some(Content::Text(symbol.value)),
        Value::State(state) => Some(Content::state(state.key.to_string(), state.init.as_ref().clone())),
        Value::Counter(_) | Value::None => None,
        Value::Version(version) => {
            let text = version.to_string();
            if text.is_empty() { None } else { Some(Content::Text(text.into())) }
        }
        // Numeric display differs from constructor-form repr.
        Value::Float(value) => Some(Content::Text(format!("{value}").into())),
        Value::Decimal(value) => Some(Content::Text(value.to_string().into())),
        // Preserve this bounded legacy path; it does not certify rich display parity.
        other @ (Value::Int(..)
            | Value::Array(..)
            | Value::Dict(..)
            | Value::Module(..)
            | Value::Datetime(..)
            | Value::Func(..)
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
            | Value::Bytes(..)
            | Value::Duration(..)
            | Value::Selector(..)
            | Value::Args(..)
            | Value::Label(..)
            | Value::Dir(..)
            | Value::Path(..)
            | Value::Type(..)
            | Value::SpotColorant(..)) => {
            let text = crate::compiler::eval::repr::repr_value(&other);
            if text.is_empty() { None } else { Some(Content::Text(text.into())) }
        }
    }
}
