//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/place.md
//! @prompt-hash 6529b9d5
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::place`.

use super::*;

/// `place(alignment, dx?, dy?, scope?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
/// `scope` (Passo 84.6, encerra DEBT-37): `"column"` (default — ancora à
/// célula activa de Grid, ou à página fora de Grid) ou `"parent"` (ancora
/// sempre à página). Aceita string ou omissão.
pub fn native_place(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        // P223 — accept "float" e "clearance" novos named args.
        if !["dx", "dy", "scope", "float", "clearance"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-dimensional retorna 0.0 na extracção de pontos
        }
    }

    // Preserve an omitted vertical axis: it follows flow, unlike explicit Top.
    let alignment = extract_alignment(
        args,
        Align2D {
            h: Some(crate::entities::layout_types::HAlign::Left),
            v: None,
        },
    );

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let scope = match args.named.get("scope") {
        Some(Value::Str(s)) => match s.as_str() {
            "column" => crate::entities::layout_types::PlaceScope::Column,
            "parent" => crate::entities::layout_types::PlaceScope::Parent,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                    "place(): scope deve ser \"column\" ou \"parent\", recebeu \"{}\"",
                    other
                ),
                )])
            }
        },
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(): scope deve ser string, recebeu {}", other.type_name()),
            )])
        }
        None => crate::entities::layout_types::PlaceScope::default(),
    };

    // P223 — extract `float` (default false). Semantic real adiada per
    // ADR-0054 graded; precedente N=4 cumulativo weak/breakable/float.
    let float = match args.named.get("float") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(float): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P1292-D v10 — o default permanece relativo até ao layout, onde é
    // resolvido contra o estilo efectivo da ocorrência. Valores explícitos,
    // inclusive zero, continuam soberanos.
    let clearance = match args.named.get("clearance") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "place(clearance): espera length, recebeu {}",
                        val.type_name()
                    ),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "place(clearance): valor negativo não suportado".to_string(),
                )]);
            }
            Some(len)
        }
        None => Some(Length::em(1.5)),
    };

    // P223 — DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α):
    // vanilla `place` com `scope: "parent"` exige `float: true`.
    if matches!(scope, crate::entities::layout_types::PlaceScope::Parent) && !float {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "place: scope \"parent\" requer float: true (paridade vanilla; sem float, scope \"parent\" não tem semantic flutuante; fecha divergência DEBT-37 documentada)".to_string(),
        )]);
    }

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "place() exige um bloco de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::place(alignment, dx, dy, scope, float, clearance, body)))
}

/// `place.flush()` → sentinela que realiza os floats já pendentes.
pub fn native_flush(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    validate_flush_args(args)?;
    Ok(Value::Content(Content::flush()))
}

pub(crate) fn validate_flush_args(args: &Args) -> SourceResult<()> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(())
}
