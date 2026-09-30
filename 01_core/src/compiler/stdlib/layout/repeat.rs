//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/repeat.md
//! @prompt-hash 19c3ad85
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::repeat`.

use super::*;

/// `repeat(body, gap: ?, justify: true)` → `Content::Repeat`.
///
/// **Primeira aplicação Fase 3** declarada em ADR-0061. Caso de uso
/// primário: TOC dot leaders `#box(width: 1fr, repeat[.])`.
///
/// **Atributos** (paridade vanilla `RepeatElem`):
/// - `body` posicional obrigatório (Content ou Str).
/// - `gap: Length`/`Float`/`Int` (em pt). Ausente == zero (padrão
///   Smart→Option N=6 da série).
/// - `justify: bool`. Default **`true`** (paridade vanilla;
///   distribuição de espaço residual diferida per ADR-0054).
///
/// **Limitação aceite (perfil ADR-0054 graded)**: o algoritmo de
/// runtime que calcula `floor(available / (body_width + gap))`
/// está diferido — Layouter executa single-render do body
/// (suficiente para paridade estrutural, exhaustive pattern-match
/// e walk de counters/labels dentro do body).
pub fn native_repeat(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                "repeat() espera content ou string como primeiro argumento, recebeu {}",
                other.type_name()
            ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "repeat() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut gap: Option<Length> = None;
    let mut justify: bool = true; // default vanilla

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gap" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "repeat(gap:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "repeat(gap:): valor negativo não suportado neste passo (P156J)"
                            .to_string(),
                    )]);
                }
                gap = Some(len);
            }
            "justify" => match value {
                Value::Bool(b) => justify = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "repeat(justify:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("repeat(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::repeat(body, gap, justify)))
}

/// **P218 helper** — extrai `count: usize` posicional obrigatório
/// para `native_columns`. Rejeita `count = 0` (paridade `NonZeroUsize`
/// vanilla per ADR-0054 graded).
///
/// Distinto de `extract_length` (Length) e `extract_usize_or_none_min`
/// (P157B; named opcional). N=1 pós-P218; promoção a helper público
/// diferida a N=2-3 reuso.
pub(crate) fn extract_count(args: &Args, fn_name: &str) -> SourceResult<usize> {
    match args.items.first() {
        Some(Value::Int(n)) => {
            if *n < 1 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{}(count): count deve ser >= 1, recebeu {}", fn_name, n),
                )]);
            }
            Ok(*n as usize)
        }
        Some(other) => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{}(count): espera Int, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{}: argumento posicional count obrigatório ausente", fn_name),
        )]),
    }
}
