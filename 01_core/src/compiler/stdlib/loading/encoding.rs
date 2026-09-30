//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/encoding.md
//! @prompt-hash 47da4177
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn canonical_numeric_token(mut token: String) -> String {
    if let Some(exponent) = token.find("e+") {
        token.remove(exponent + 1);
    }
    token
}

/// Reescreve somente tokens numéricos; strings JSON, inclusive escapes e
/// texto semelhante a números, são copiadas sem interpretação.
pub(super) fn textual_content_fields(value: &Value) -> Result<Dict, String> {
    use crate::entities::content::Content;
    let (content, snapshot) = match value {
        Value::Content(content) => (content, None),
        Value::LocatedContent(content, _) => (content.content(), content.fields()),
        _ => return Err("expected content".into()),
    };
    let mut fields = new_dict();
    let function = match content {
        Content::Empty => "sequence",
        other => other.elem_name(),
    };
    fields.insert("func".into(), Value::Str(function.into()));
    if let Some(snapshot) = snapshot {
        fields.extend(snapshot.iter().map(|(name, value)| (name.clone(), value.clone())));
    } else {
        // Campos de payload cuja presença é estrutural. O helper legado de
        // métodos só enumera body para essas famílias e não os pode recuperar.
        match content {
            Content::Empty => {
                fields.insert("children".into(), Value::Array(vec![]));
                return Ok(fields);
            }
            Content::Sequence(children) => {
                fields.insert(
                    "children".into(),
                    Value::Array(children.iter().cloned().map(Value::Content).collect()),
                );
                return Ok(fields);
            }
            Content::Metadata(metadata) => {
                fields.insert("value".into(), metadata.value.as_ref().clone());
                return Ok(fields);
            }
            Content::Styled(styled) => {
                let child = &styled.body;
                fields.insert("child".into(), Value::Content(child.clone()));
                fields.insert("styles".into(), Value::Str("styles(..)".into()));
                return Ok(fields);
            }
            Content::Raw(raw_elem) => {
                fields.insert("text".into(), Value::Str(raw_elem.text.clone()));
                if let Some(language) = &raw_elem.lang {
                    fields.insert("lang".into(), Value::Str(language.clone()));
                }
                // O bool block legado não distingue default de presença.
                return Ok(fields);
            }
            Content::Link(link) => {
                fields.insert("dest".into(), Value::Str(link.url.clone()));
                fields.insert("body".into(), Value::Content(link.body.clone()));
                return Ok(fields);
            }
            Content::Boxed(boxed) => {
                fields.insert("body".into(), Value::Content(boxed.body.clone()));
                return Ok(fields);
            }
            Content::Block(block) => {
                fields.insert("body".into(), Value::Content(block.body.clone()));
                return Ok(fields);
            }
            Content::HtmlElem(element) => {
                fields.insert("tag".into(), Value::Str(element.tag.clone()));
                if let Some(attributes) = &element.attrs {
                    fields.insert(
                        "attrs".into(),
                        Value::Dict(
                            attributes
                                .iter()
                                .map(|(key, value)| {
                                    (key.clone(), Value::Str(value.clone()))
                                })
                                .collect(),
                        ),
                    );
                }
                match &element.body {
                    crate::entities::html::HtmlBody::Unset => {}
                    crate::entities::html::HtmlBody::None => {
                        fields.insert("body".into(), Value::None);
                    }
                    crate::entities::html::HtmlBody::Content(body) => {
                        fields
                            .insert("body".into(), Value::Content(body.as_ref().clone()));
                    }
                }
                return Ok(fields);
            }
            _ => {}
        }
        let projected = crate::compiler::eval::bindings::eval_content_method(
            content,
            "fields",
            Args::positional(vec![]),
            Span::detached(),
        )
        .map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.message)
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        let Value::Dict(projected) = projected else {
            return Err("content fields must be a dictionary".into());
        };
        fields.extend(projected);
    }
    Ok(fields)
}

/// Casts seguem a ordem dos parâmetros do contrato: value, todos os pretty,
/// e então o primeiro argumento remanescente na sequência causal.
pub(super) fn textual_encoder_args(
    args: &Args,
    toml: bool,
    allow_pretty: bool,
) -> SourceResult<(Value, Span, bool)> {
    let mut occurrences = args.occurrence_sequence();
    let Some(index) = occurrences.iter().position(|argument| argument.name.is_none())
    else {
        if let Some(argument) = occurrences
            .iter()
            .find(|argument| argument.name.as_deref() == Some("value"))
        {
            return Err(vec![SourceDiagnostic::error(
                argument.span,
                "the argument `value` is positional",
            )
            .with_hint("try removing `value:`")]);
        }
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: value")]);
    };
    let argument = occurrences.remove(index);
    if toml && !matches!(argument.value, Value::Dict(_)) {
        return Err(vec![SourceDiagnostic::error(
            argument.value_span,
            format!("expected dictionary, found {}", vanilla_type_name(&argument.value)),
        )]);
    }
    let mut pretty = true;
    if allow_pretty {
        for occurrence in &occurrences {
            if occurrence.name.as_deref() == Some("pretty") {
                match &occurrence.value {
                    Value::Bool(value) => pretty = *value,
                    value => {
                        return Err(vec![SourceDiagnostic::error(
                            occurrence.value_span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name(value)
                            ),
                        )])
                    }
                }
            }
        }
        occurrences.retain(|occurrence| occurrence.name.as_deref() != Some("pretty"));
    }
    if let Some(occurrence) = occurrences.first() {
        let message = match &occurrence.name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(occurrence.span, message)]);
    }
    Ok((argument.value, argument.value_span, pretty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_projection_has_func_and_structural_fields() {
        let fields = textual_content_fields(&Value::Content(
            crate::entities::content::Content::Empty,
        ))
        .unwrap();
        assert!(
            matches!(fields.get("func"), Some(Value::Str(name)) if name.as_str() == "sequence")
        );
        assert!(
            matches!(fields.get("children"), Some(Value::Array(children)) if children.is_empty())
        );
    }
}
