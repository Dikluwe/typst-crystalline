//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/text_costs.md
//! @prompt-hash 8e78f368
//! @layer L1
//! @updated 2026-09-28

use crate::entities::{
    source_result::{SourceDiagnostic, SourceResult},
    span::Span,
    style_chain::StyleChain,
    value::Value,
};

// Pinned inline/linebreak.rs:29-30. Configuration scales these algorithm defaults.
const DEFAULT_HYPHEN_COST: f64 = 135.0;
const DEFAULT_RUNT_COST: f64 = 100.0;
const FIELDS: [(&str, &str); 4] = [
    ("hyphenation", "text.costs.hyphenation"),
    ("runt", "text.costs.runt"),
    ("widow", "text.costs.widow"),
    ("orphan", "text.costs.orphan"),
];
#[derive(Clone, Copy, Debug)]
pub(super) struct LineCosts {
    pub(super) hyphenation: f64,
    pub(super) runt: f64,
}
impl Default for LineCosts {
    fn default() -> Self {
        Self {
            hyphenation: DEFAULT_HYPHEN_COST,
            runt: DEFAULT_RUNT_COST,
        }
    }
}
fn cast_error(expected: &str, value: &Value, span: Span) -> SourceDiagnostic {
    let kind = match value.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    SourceDiagnostic::error(span, format!("expected {expected}, found {kind}"))
}
fn list(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [one, two] => format!("{one} and {two}"),
        many => {
            format!("{}, and {}", many[..many.len() - 1].join(", "), many.last().unwrap())
        }
    }
}
pub(super) fn parse(
    value: &Value,
    span: Span,
) -> SourceResult<Vec<(&'static str, Value)>> {
    let Value::Dict(dict) = value else {
        return Err(vec![cast_error("dictionary", value, span)]);
    };
    let mut entries = Vec::new();
    // Source cast validates recognized fields in this order, before unknown keys.
    for (name, channel) in FIELDS {
        if let Some(value) = dict.get(name) {
            if !matches!(value, Value::Ratio(_)) {
                return Err(vec![cast_error("ratio", value, span)]);
            }
            entries.push((channel, value.clone()));
        }
    }
    let unknown: Vec<String> = dict
        .keys()
        .filter(|key| !FIELDS.iter().any(|(name, _)| key.as_str() == *name))
        .map(|key| format!("\"{}\"", key.as_str().escape_debug()))
        .collect();
    if !unknown.is_empty() {
        let expected: Vec<String> =
            FIELDS.iter().map(|(name, _)| format!("\"{name}\"")).collect();
        let noun = if unknown.len() == 1 { "key" } else { "keys" };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "unexpected {noun} {}, valid keys are {}",
                list(&unknown),
                list(&expected)
            ),
        )]);
    }
    Ok(entries)
}
pub(super) fn is_channel(key: &str) -> bool {
    FIELDS.iter().any(|(_, channel)| key == *channel)
}
pub(super) fn prevention(chain: &StyleChain) -> (bool, bool) {
    let enabled = |key| match chain.custom(key) {
        Some(Value::Ratio(r)) => r.get() > 0.0,
        _ => true,
    };
    (enabled(FIELDS[2].1), enabled(FIELDS[3].1))
}
pub(super) fn resolve(chain: &StyleChain) -> LineCosts {
    let multiplier = |key| match chain.custom(key) {
        Some(Value::Ratio(r)) => r.get(),
        _ => 1.0,
    };
    LineCosts {
        hyphenation: DEFAULT_HYPHEN_COST * multiplier(FIELDS[0].1),
        runt: DEFAULT_RUNT_COST * multiplier(FIELDS[1].1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Ratio;
    fn dict(fields: &[(&str, Value)]) -> Value {
        Value::Dict(fields.iter().map(|(k, v)| ((*k).into(), v.clone())).collect())
    }
    fn apply(chain: &StyleChain, fields: &[(&str, Value)]) -> StyleChain {
        let mut chain = chain.clone();
        for (key, value) in parse(&dict(fields), Span::detached()).unwrap() {
            chain = chain.push_custom(key, value);
        }
        chain
    }
    #[test]
    fn custom_costs_fold_preserves_unspecified_fields_and_restores_outer() {
        let outer = apply(
            &StyleChain::empty(),
            &[
                ("hyphenation", Value::Ratio(Ratio(10.0))),
                ("runt", Value::Ratio(Ratio(5.0))),
            ],
        );
        let inner = apply(&outer, &[("runt", Value::Ratio(Ratio(0.0)))]);
        assert_eq!(resolve(&inner).hyphenation, 1350.0);
        assert_eq!(resolve(&inner).runt, 0.0);
        assert_eq!(resolve(&outer).runt, 500.0);
        let empty = apply(&inner, &[]);
        assert_eq!(resolve(&empty).hyphenation, 1350.0);
        assert_eq!(resolve(&empty).runt, 0.0);
    }
    #[test]
    fn custom_costs_validation_prioritizes_fields_and_preserves_negative_ratios() {
        let v = dict(&[
            ("runt", Value::Int(1)),
            ("hyphenation", Value::Bool(false)),
            ("other", Value::Int(0)),
        ]);
        assert_eq!(
            parse(&v, Span::detached()).unwrap_err()[0].message,
            "expected ratio, found boolean"
        );
        let c =
            apply(&StyleChain::empty(), &[("hyphenation", Value::Ratio(Ratio(-1.0)))]);
        assert_eq!(resolve(&c).hyphenation, -135.0);
        assert_eq!(resolve(&c).runt, 100.0);
    }
    #[test]
    fn custom_costs_unknown_keys_keep_insertion_order_and_complete_list() {
        let v = dict(&[("a", Value::Ratio(Ratio(1.0))), ("b", Value::Ratio(Ratio(1.0)))]);
        assert_eq!(parse(&v,Span::detached()).unwrap_err()[0].message,
            "unexpected keys \"a\" and \"b\", valid keys are \"hyphenation\", \"runt\", \"widow\", and \"orphan\"");
    }
}
