//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/repr/selector.md
//! @prompt-hash bb5ddc24
//! @layer L1
//! @updated 2026-10-06

use super::*;

/// Representação de um `Selector`.
pub fn repr_selector(sel: &Selector) -> String {
    match sel {
        Selector::Kind(kind) => kind.as_str().to_string(),
        Selector::NativeElement(function) => {
            crate::compiler::eval::bindings::native_element_fields(function)
                .map(|(name, _, _)| name.to_string())
                .unwrap_or_else(|| "selector(..)".to_string())
        }
        Selector::Element { function, fields } => {
            let group = if fields.is_empty() {
                "(:)".to_string()
            } else {
                let parts: Vec<String> = fields
                    .iter()
                    .map(|(name, value)| format!("{}: {}", name, repr_value(value)))
                    .collect();
                pretty_array_like(&parts, false)
            };
            format!("{}.where{}", repr_value(&Value::Func(function.clone())), group)
        }
        Selector::Label(l) => format!("<{0}>", l.0),
        Selector::Location(_) => "location".to_string(),
        Selector::And(list) => {
            let parts: Vec<String> = list.iter().map(repr_selector).collect();
            format!("({})", parts.join(" & "))
        }
        Selector::Or(list) => {
            let parts: Vec<String> = list.iter().map(repr_selector).collect();
            format!("({})", parts.join(" | "))
        }
        Selector::Regex(r) => format!("regex(\"{}\")", r.pattern()),
        Selector::Where { base, field, value } => {
            format!(
                "{}.where({}: {})",
                repr_selector(base),
                field.as_str(),
                repr_value(value)
            )
        }
        Selector::Within { base, ancestor } => {
            format!("{}.within({})", repr_selector(base), repr_selector(ancestor))
        }
        Selector::Before { base, end, inclusive } => {
            let suffix = if *inclusive { "" } else { ", inclusive: false" };
            format!("{}.before({}{suffix})", repr_selector(base), repr_selector(end))
        }
        Selector::After { base, start, inclusive } => {
            let suffix = if *inclusive { "" } else { ", inclusive: false" };
            format!("{}.after({}{suffix})", repr_selector(base), repr_selector(start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_selector_uses_language_name() {
        assert_eq!(
            repr_selector(&Selector::Kind(
                crate::entities::element_kind::ElementKind::Heading
            )),
            "heading"
        );
    }
}
