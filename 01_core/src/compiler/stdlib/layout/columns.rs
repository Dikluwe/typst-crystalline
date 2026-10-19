//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/columns.md
//! @prompt-hash 796c92c8
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::columns`.

use super::*;

/// **P218 (DEBT-56 sub-fase b — Layout Fase 3)** —
/// `columns(count, body, gutter: ?)` → `Content::Columns`.
///
/// Forma: `#columns(2)[body]` ou `#columns(2, gutter: 1em)[body]`.
///
/// Aditivo P218 — arm Layouter é stub transparente em P217 (consumer
/// real P219 sub-fase (b) DEBT-56).
///
/// Validações:
/// - `count >= 1` rejeita `count = 0` (paridade `NonZeroUsize`
///   vanilla per ADR-0054 graded).
/// - `gutter` negativo rejeitado (paridade `Stack.spacing` P156I,
///   `Repeat.gap` P156J).
/// - Named arg desconhecido rejeitado.
/// - Body `Value::Content` ou `Value::Str` obrigatório.
pub fn native_columns(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // 1. Extract count (posicional [0] obrigatório).
    let count = extract_count(args, "columns")?;

    // 2. Extract body (posicional [1], Content ou Str).
    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "columns(body): espera Content ou Str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "columns: argumento posicional body obrigatório ausente".to_string(),
            )])
        }
    };

    // 3. Validate no extra positionals.
    if args.items.len() > 2 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "columns: aceita 2 posicionais (count, body), recebeu {}",
                args.items.len()
            ),
        )]);
    }

    // 4. Extract gutter (named opcional, Option<Length>); validar negativo.
    let mut gutter: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gutter" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "columns(gutter:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "columns(gutter:): valor negativo não suportado".to_string(),
                    )]);
                }
                gutter = Some(len);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("columns(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::columns(body, count, gutter)))
}

/// `colbreak(weak: false)` → `Content::Colbreak` — Passo 220
/// (ADR-0078 PROPOSTO sub-fase b 4/4 — fecha sub-fase b).
///
/// Forma: `#colbreak()` ou `#colbreak(weak: true)`.
///
/// Sem argumentos posicionais. `weak` armazenado mas semantic
/// de collapse adiada (paridade `Pagebreak.weak` P156E).
/// **Sem `to:`** — vanilla `ColbreakElem` não tem (paridade só
/// faz sentido em páginas).
///
/// **Semantic graded P220** — colbreak downgrade a pagebreak
/// pós-P219 (Opção B graded; sem multi-region flow real).
/// Refino multi-region salto entre colunas reais é
/// P-Layout-Fase4 candidato (não-reservado).
pub fn native_colbreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "colbreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "colbreak(weak:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "colbreak(): argumento nomeado inesperado '{}' (esperado: weak)",
                        other
                    ),
                )])
            }
        }
    }

    Ok(Value::Content(Content::colbreak_with_weak_presence(
        weak,
        args.named.contains_key("weak"),
    )))
}
