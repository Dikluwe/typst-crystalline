//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/spacing.md
//! @prompt-hash 700c1bcf
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::spacing`.

use super::*;

/// Resolve `weak: bool` em named args (ou default false). Erro hard se
/// tipo não-bool.
pub(crate) fn extract_weak(args: &Args, fn_name: &str) -> SourceResult<(bool, bool)> {
    match args.named.get("weak") {
        Some(Value::Bool(b)) => Ok((*b, true)),
        Some(other) => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{}(weak:) espera bool, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Ok((false, false)),
    }
}

/// Lógica partilhada por `native_h` e `native_v`:
/// extrai `amount` (posicional obrigatório), valida não-negativo,
/// resolve `weak`. Aceita Length, Float (interpretado em pt) ou Int (idem)
/// per `extract_length`. **P842 (#38)**: aceita também `Fraction`
/// (`Spacing::Fractional`, paridade vanilla `layout/spacing.rs`) — quem
/// não suporta fração (`v()` neste passo) rejeita no caller.
pub(crate) fn build_spacing(
    args: &Args,
    fn_name: &str,
    valid_named: &[&str],
) -> SourceResult<(crate::entities::elements::h_space::Spacing, bool, bool)> {
    use crate::entities::elements::h_space::Spacing;
    // amount posicional obrigatório
    let amount = match args.items.first() {
        Some(Value::Fraction(fr)) => {
            // P842 (#38) — validação não-negativo também para frações
            // (mesmo perfil ADR-0054 graded dos comprimentos).
            if *fr < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(): amount negativo não suportado neste passo (P156D)",
                        fn_name
                    ),
                )]);
            }
            Spacing::Fractional(*fr)
        }
        Some(v) => Spacing::Absolute(extract_length(v).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}() espera amount como length, recebeu {}",
                    fn_name,
                    v.type_name()
                ),
            )]
        })?),
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}() exige amount como argumento posicional", fn_name),
            )])
        }
    };

    // Validação: amount negativo rejeitado per perfil ADR-0054 graded
    // (vanilla aceita-o; cristalino diverge intencionalmente neste passo).
    if let Spacing::Absolute(l) = amount {
        if l.abs.0 < 0.0 || l.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}(): amount negativo não suportado neste passo (P156D)",
                    fn_name
                ),
            )]);
        }
    }

    // Validação: rejeitar named args desconhecidos.
    for key in args.named.keys() {
        if !valid_named.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(): argumento nomeado inesperado '{}'", fn_name, key),
            )]);
        }
    }

    let (weak, weak_explicit) = extract_weak(args, fn_name)?;

    Ok((amount, weak, weak_explicit))
}

/// `h(amount, weak: false)` → `Content::HSpace`.
///
/// `amount` Length ou Fraction posicional obrigatório. **P842 (#38)** —
/// `Fraction` aceite (paridade vanilla medida em `temp/p842/l7_h_*.typ`:
/// `#h(1fr)` distribui o espaço restante da linha proporcionalmente);
/// antes era rejeitado ("h() espera amount como length, recebeu
/// fraction"). `weak` armazenado mas comportamento de collapse adiado
/// neste passo (perfil ADR-0054 graded).
pub fn native_h(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::elements::h_space::Spacing;
    let (amount, weak, weak_explicit) = build_spacing(args, "h", &["weak"])?;
    Ok(Value::Content(match amount {
        Spacing::Absolute(l) => {
            Content::h_space_with_weak_presence(l, weak, weak_explicit)
        }
        Spacing::Fractional(fr) => {
            Content::h_space_fraction_with_weak_presence(fr, weak, weak_explicit)
        }
    }))
}

/// `v(amount, weak: false)` → `Content::VSpace`.
///
/// Análogo a `native_h`, produz spacing primitive vertical.
/// **P842 (#38)**: `Fraction` continua rejeitado aqui (distribuição
/// vertical fracionária é outro mecanismo — scope-out registado no
/// relatório de P842); a mensagem pré-P842 preserva-se verbatim.
pub fn native_v(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::elements::h_space::Spacing;
    let (amount, weak, weak_explicit) = build_spacing(args, "v", &["weak"])?;
    match amount {
        Spacing::Absolute(l) => Ok(Value::Content(Content::v_space_with_weak_presence(
            l,
            weak,
            weak_explicit,
        ))),
        Spacing::Fractional(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "v() espera amount como length, recebeu fraction".to_string(),
        )]),
    }
}

/// `linebreak(justify: false)` — P1140.10.
///
/// Este passo materializa constructor e presença do campo. A diferença de
/// layout quando `justify` é true pertence explicitamente ao P1140.11.
pub fn native_linebreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "linebreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    for key in args.named.keys() {
        if key.as_str() != "justify" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("linebreak(): argumento nomeado inesperado '{}'", key),
            )]);
        }
    }

    let (justify, justify_explicit) = match args.named.get("justify") {
        Some(Value::Bool(value)) => (*value, true),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("linebreak(justify:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        None => (false, false),
    };

    Ok(Value::Content(Content::linebreak_with_justify_presence(
        justify,
        justify_explicit,
    )))
}
