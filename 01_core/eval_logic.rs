//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Eval
//! Responsabilidade: Avaliação de expressões no contexto Typst e serialização dos resultados.

use comemo::Track;
use ecow::{EcoString, eco_format};
use serde::Serialize;
use typst::diag::SourceResult;
use typst::engine::Sink;
use typst::foundations::{Context, Scope, StyleChain, Value};
use typst::introspection::Introspector;
use typst::syntax::{Span, SyntaxMode};
use typst::World;
use typst_eval::eval_string;

use super::args_cli::SerializationFormat;

/// Avalia uma expressão no modo `SyntaxMode::Code` sem escopo inicial.
pub fn evaluate_expression(
    expression: String,
    sink: &mut Sink,
    world: &dyn World,
    introspector: &Introspector,
) -> SourceResult<Value> {
    eval_string(
        &typst::ROUTINES,
        world.track(),
        sink.track_mut(),
        introspector.track(),
        Context::new(None, Some(StyleChain::new(&world.library().styles))).track(),
        &expression,
        Span::detached(),
        SyntaxMode::Code,
        Scope::default(),
    )
}

/// Serializa dados genéricos (como `Value`) em JSON ou YAML.
pub fn serialize_value(
    data: &impl Serialize,
    format: SerializationFormat,
    pretty: bool,
) -> Result<String, EcoString> {
    match format {
        SerializationFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).map_err(|e| eco_format!("{e}"))
            } else {
                serde_json::to_string(data).map_err(|e| eco_format!("{e}"))
            }
        }
        SerializationFormat::Yaml => {
            serde_yaml::to_string(data).map_err(|e| eco_format!("{e}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialize_json_raw() {
        let data = json!({"key": "value", "num": 42});
        let result = serialize_value(&data, SerializationFormat::Json, false).unwrap();
        assert_eq!(result, r#"{"key":"value","num":42}"#);
    }

    #[test]
    fn test_serialize_json_pretty() {
        let data = json!({"key": "value"});
        let result = serialize_value(&data, SerializationFormat::Json, true).unwrap();
        assert!(result.contains("{\n"));
        assert!(result.contains("  \"key\": \"value\""));
    }

    #[test]
    fn test_serialize_yaml() {
        let data = json!({"key": "value"});
        let result = serialize_value(&data, SerializationFormat::Yaml, false).unwrap();
        assert!(result.contains("key: value"));
    }
}
