//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/counter/display.md
//! @prompt-hash 573a1357
//! @layer L1
//! @updated 2026-10-05

use super::filtered_reads::filtered_history;
use super::*;

/// Resolve `.display([pattern|callback])` dentro ou fora de context.
pub(super) fn render(
    counter: &Counter,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    ctx.mark_filtered_counter_read(&counter.key);
    let requested = args.named.get("at").unwrap_or(&Value::Auto);
    if matches!(requested, Value::Auto) {
        if !ctx.in_context {
            return Err(vec![SourceDiagnostic::error(
                span,
                "counter.display() can only be used inside context",
            )]);
        }
        if ctx.current_location.is_none() {
            return Err(vec![SourceDiagnostic::error(
                span,
                "counter.display() requer uma localização de contexto",
            )]);
        }
    }
    let location = match resolve_counter_location(requested, ctx, span)? {
        Value::Location(location) => location,
        _ => crate::entities::location::Location::from_raw(0),
    };

    if let Some((name, _)) = args
        .named
        .iter()
        .find(|(name, _)| name.as_str() != "at" && name.as_str() != "both")
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let both = match args.named.get("both") {
        None => false,
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };

    let history = if is_filtered_key(&counter.key) {
        Some(filtered_history(counter, ctx, scopes, engine, span)?)
    } else {
        None
    };
    let page_values = page_counter_value(counter, location, ctx);
    let mut values = page_values
        .as_deref()
        .unwrap_or_else(|| match &history {
            Some(history) => history
                .iter()
                .rev()
                .find(|(at, _)| {
                    ctx.introspector.counters.order_key(*at)
                        <= ctx.introspector.counters.order_key(location)
                })
                .map(|(_, state)| state.as_slice())
                .unwrap_or(&[0]),
            None => ctx
                .introspector
                .counter_values_at(&counter.key, location)
                .unwrap_or(&[0]),
        })
        .to_vec();
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterAt {
            key: counter.key.clone(),
            location,
        },
        span,
        Ok(state_value(&values)),
    )?;
    if both {
        let total = match &history {
            Some(history) => history
                .last()
                .and_then(|(_, state)| state.first())
                .copied()
                .unwrap_or(0),
            None => page_counter_final_value(counter, ctx)
                .as_deref()
                .or_else(|| ctx.introspector.counter_final_values(&counter.key))
                .and_then(|state| state.first())
                .copied()
                .unwrap_or(0),
        };
        ctx.observe_context_read(
            crate::compiler::eval::ContextReadRequest::CounterTotal {
                key: counter.key.clone(),
            },
            span,
            Ok(Value::Int(total as i64)),
        )?;
        values.push(total);
    }

    match args.items.as_slice() {
        [] | [Value::Auto] => {
            // **P844** (achado #52 de P831) — sem padrão explícito, usa
            // o numbering activo do contexto para o counter (medido no
            // vanilla 0.15.0: `#set heading(numbering: "1.")` →
            // `counter(heading).display()` renderiza "1."). O pattern
            // viaja na chain como custom `"{key}.numbering.pattern"`
            // (canal `rules.rs`, mesmo mecanismo do próprio heading).
            // Sem pattern na chain, mantém o join pré-P844.
            let pattern_key = match &counter.key {
                CounterKey::Str(s) => format!("{}.numbering.pattern", s),
                CounterKey::Selector(Selector::Kind(k)) => {
                    format!("{:?}.numbering.pattern", k).to_lowercase()
                }
                CounterKey::Page => "page.numbering.pattern".to_string(),
                _ => "counter.numbering.pattern".to_string(),
            };
            let pattern = engine.styles.custom(&pattern_key).and_then(|v| match v {
                Value::Str(s) => Some(s.to_string()),
                _ => None,
            });
            match pattern {
                Some(p) => {
                    let numbers: Vec<u32> = values.iter().map(|&n| n as u32).collect();
                    let text = super::super::numbering::format_pattern(
                        engine, span, &p, &numbers,
                    )?;
                    Ok(Value::Content(Content::text(text)))
                }
                None => {
                    let text = values
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(".");
                    Ok(Value::Content(Content::text(text)))
                }
            }
        }
        [Value::Str(pattern)] => {
            // **P844** (achado #53 de P831) — partilha o algoritmo real
            // de numbering (`format_pattern`, P793 em
            // `stdlib/numbering.rs`; extraído de `structural.rs` no P847):
            // estilos romano/alfabético/
            // circled, descarte de tokens extra e repetição do último
            // token — paridade vanilla medida (`II B ii ② 2` para
            // counter=2). Substitui o stub "Pattern minimal".
            let numbers: Vec<u32> = values.iter().map(|&n| n as u32).collect();
            let text = super::super::numbering::format_pattern(
                engine,
                span,
                pattern.as_str(),
                &numbers,
            )?;
            Ok(Value::Content(Content::text(text)))
        }
        [Value::Func(callback)] => {
            let result = apply_func(
                callback.clone(),
                Args::positional(values.iter().map(|n| Value::Int(*n as i64)).collect()),
                scopes,
                ctx,
                engine,
            )?;
            Ok(Value::Content(super::super::state::value_to_content(&result)))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "counter.display() requer string ou função como argumento, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "counter.display() requer 0 ou 1 argumentos".to_string(),
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_entrypoint_has_static_signature() {
        let _: fn(
            &Counter,
            &Args,
            &mut Scopes<'_>,
            &mut EvalContext,
            &mut Engine<'_>,
            Span,
        ) -> SourceResult<Value> = render;
    }
}
