//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math.md
//! @prompt-hash 8d134ea5
//! @layer L1
//! @updated 2026-08-31
//!
//! Nativas de matemática: `accent`, `cancel`, `class`, `underover`, `op`,
//! e a montagem do módulo `math`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::compiler::eval::EvalContext;
#[cfg(test)]
use crate::entities::args::ArgOccurrence;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_accent::MathAccentExplicit;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::elements::math_cancel::{MathCancelAngle, MathCancelExplicit};
use crate::entities::elements::math_cases::MathCasesExplicit;
use crate::entities::elements::math_matrix::{MathMatrixAugment, MathMatrixExplicit};
use crate::entities::elements::math_vec::MathVecExplicit;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Align2D, HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

#[cfg(test)]
mod p1425_tests;

#[cfg(test)]
mod p1426_tests;

#[cfg(test)]
mod p1427_tests;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// Construtor puro compartilhado pela função pública e pelo modo math.
pub(crate) fn sqrt_content(radicand: Content) -> Content {
    Content::math_root(None, radicand, false)
}

/// Construtor puro compartilhado pela função pública e pela sintaxe `$...$`.
pub(crate) fn equation_content(body: Content, block: bool) -> Content {
    Content::equation(body, block)
}

/// Cast público de `math.equation.number-align`.
pub(crate) fn equation_number_align(value: &Value, span: Span) -> SourceResult<Align2D> {
    let Value::Align(align) = value else {
        let found = match value.type_name() {
            "int" => "integer",
            "str" => "string",
            "bool" => "boolean",
            other => other,
        };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected alignment, found {found}"),
        )]);
    };
    if matches!(align.h, Some(HAlign::Center)) {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected `start`, `left`, `right`, or `end`, found center".to_string(),
        )]);
    }
    Ok(*align)
}

pub(crate) fn equation_supplement(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(text) => Ok(Value::Content(Content::text(text.as_str()))),
        Value::Content(_) | Value::Func(_) | Value::None | Value::Auto => {
            Ok(value.clone())
        }
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected content, function, none, or auto, found {found}"),
            )])
        }
    }
}

pub(crate) fn equation_alt(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(_) | Value::None => Ok(value.clone()),
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected string or none, found {found}"),
            )])
        }
    }
}

/// `math.equation(body, block: false)` — primeira fase do elemento público.
pub fn native_math_equation(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(body)] => body.clone(),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    let mut block = false;
    let mut numbering: Option<Value> = None;
    let mut number_align: Option<Value> = None;
    let mut supplement: Option<Value> = None;
    let mut alt: Option<Value> = None;
    for (name, value) in &args.named {
        match name.as_str() {
            "block" => match value {
                Value::Bool(value) => block = *value,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected bool, found {}", other.type_name()),
                    )])
                }
            },
            "numbering" => match value {
                Value::Str(_) | Value::Func(_) | Value::None => {
                    numbering = Some(value.clone());
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "expected string, function, or none, found {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "number-align" => {
                equation_number_align(value, args.span)?;
                number_align = Some(value.clone());
            }
            "supplement" => {
                supplement = Some(equation_supplement(value, args.span)?);
            }
            "alt" => {
                alt = Some(equation_alt(value, args.span)?);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    let equation = equation_content(body, block);
    let mut styles = Styles::new();
    if let Some(value) = numbering {
        styles = styles.push_custom("equation.numbering", value);
    }
    if let Some(value) = number_align {
        styles = styles.push_custom("equation.number-align", value);
    }
    if let Some(value) = supplement {
        styles = styles.push_custom("equation.supplement", value);
    }
    if let Some(value) = alt {
        styles = styles.push_custom("equation.alt", value);
    }
    Ok(Value::Content(if styles.delta().custom.is_empty() {
        equation
    } else {
        Content::Styled(Box::new(equation), styles)
    }))
}

/// `math.sqrt(radicand)` — raiz quadrada estrutural.
pub fn native_math_sqrt(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    crate::compiler::stdlib::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Content(radicand)] => Ok(Value::Content(sqrt_content(radicand.clone()))),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("expected content, found {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]),
    }
}

/// `math.root(index, radicand)` — raiz estrutural com índice causalmente presente.
pub fn native_math_root(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("radicand") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `radicand` is positional".to_string(),
        )]);
    }
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let [index, radicand] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.len() < 2 {
                "missing argument: radicand".to_string()
            } else {
                "unexpected argument".to_string()
            },
        )]);
    };

    let index = match index {
        Value::Content(index) => Some(index.clone()),
        Value::None => None,
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
    };
    let radicand = match radicand {
        Value::Content(radicand) => radicand.clone(),
        Value::None => Content::Empty,
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
    };

    Ok(Value::Content(Content::math_root(index, radicand, true)))
}

fn math_frac_style(
    value: &Value,
    span: Span,
) -> SourceResult<crate::entities::elements::math_frac::MathFracStyle> {
    use crate::entities::elements::math_frac::MathFracStyle;
    match value {
        Value::Str(value) if value.as_str() == "vertical" => Ok(MathFracStyle::Vertical),
        Value::Str(value) if value.as_str() == "skewed" => Ok(MathFracStyle::Skewed),
        Value::Str(value) if value.as_str() == "horizontal" => {
            Ok(MathFracStyle::Horizontal)
        }
        Value::Str(_) => Err(vec![SourceDiagnostic::error(
            span,
            "expected \"vertical\", \"skewed\", or \"horizontal\"".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected \"vertical\", \"skewed\", or \"horizontal\", found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

fn math_frac_operand(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// Parser/constructor compartilhado por `math.frac` e pelo bare `frac`.
pub(crate) fn math_frac_content(args: &Args) -> SourceResult<Content> {
    for positional in ["num", "denom"] {
        if args.named.contains_key(positional) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("the argument `{positional}` is positional"),
            )]);
        }
    }
    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "style")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let style_count = args
        .occurrence_sequence()
        .iter()
        .filter(|occurrence| occurrence.name.as_deref() == Some("style"))
        .count();
    if style_count > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "duplicate argument: style".to_string(),
        )]);
    }
    let [num, den] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.len() {
                0 => "missing argument: num",
                1 => "missing argument: denom",
                _ => "unexpected argument",
            }
            .to_string(),
        )]);
    };
    let style = args
        .named
        .get("style")
        .map(|value| math_frac_style(value, args.span))
        .transpose()?;
    Ok(Content::math_frac_full(
        math_frac_operand(num, args.span)?,
        math_frac_operand(den, args.span)?,
        style,
        false,
        false,
        true,
    ))
}

/// `math.frac(num, denom, style: ...)`.
pub fn native_math_frac(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_frac_content(args).map(Value::Content)
}

const ACCENT_ALIASES: &[(char, &[&str])] = &[
    ('\u{0300}', &["`"]),
    ('\u{0301}', &["´"]),
    ('\u{0302}', &["^", "ˆ"]),
    ('\u{0303}', &["~", "∼", "˜"]),
    ('\u{0304}', &["¯"]),
    ('\u{0305}', &["-", "–", "‾", "−"]),
    ('\u{0306}', &["˘"]),
    ('\u{0307}', &[".", "˙", "⋅"]),
    ('\u{0308}', &["¨"]),
    ('\u{20db}', &[]),
    ('\u{20dc}', &[]),
    ('\u{030a}', &["∘", "○"]),
    ('\u{030b}', &["˝"]),
    ('\u{030c}', &["ˇ"]),
    ('\u{20d6}', &["←"]),
    ('\u{20d7}', &["→", "⟶"]),
    ('\u{20e1}', &["↔", "↔\u{fe0e}", "⟷"]),
    ('\u{20d0}', &["↼"]),
    ('\u{20d1}', &["⇀"]),
];

pub(crate) fn normalize_math_accent(value: &str) -> Option<char> {
    let direct = value.parse::<char>().ok();
    ACCENT_ALIASES
        .iter()
        .find(|(accent, aliases)| Some(*accent) == direct || aliases.contains(&value))
        .map(|(accent, _)| *accent)
        .or(direct)
}

fn math_accent_base(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

fn math_accent_char(value: &Value, span: Span) -> SourceResult<char> {
    match value {
        Value::Str(text) => normalize_math_accent(text.as_str()).ok_or_else(|| {
            vec![SourceDiagnostic::error(span, "expected exactly one character")]
        }),
        Value::Symbol(symbol) => {
            normalize_math_accent(symbol.value.as_str()).ok_or_else(|| {
                vec![SourceDiagnostic::error(span, "expected exactly one character")]
            })
        }
        Value::Content(Content::MathText(text)) => normalize_math_accent(text.as_str())
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(span, "expected a single-codepoint symbol")]
            }),
        _ => {
            Err(vec![SourceDiagnostic::error(span, "expected a single-codepoint symbol")])
        }
    }
}

/// Parser/constructor compartilhado pelo binding qualificado e pelas rotas math.
pub(crate) fn math_accent_content(args: &Args) -> SourceResult<Content> {
    for positional in ["base", "accent"] {
        if args.named.contains_key(positional) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("the argument `{positional}` is positional"),
            )]);
        }
    }
    if let Some((name, _)) = args
        .named
        .iter()
        .find(|(name, _)| !matches!(name.as_str(), "size" | "dotless"))
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    for name in ["size", "dotless"] {
        let count = args
            .occurrence_sequence()
            .iter()
            .filter(|occurrence| occurrence.name.as_deref() == Some(name))
            .count();
        if count > 1 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("duplicate argument: {name}"),
            )]);
        }
    }
    let [base, accent] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.len() {
                0 => "missing argument: base",
                1 => "missing argument: accent",
                _ => "unexpected argument",
            },
        )]);
    };

    let mut size = Rel::from_percent(100.0);
    let mut dotless = true;
    let mut explicit = MathAccentExplicit::default();
    if let Some(value) = args.named.get("size") {
        explicit.size = true;
        size = match value {
            Value::Relative(value) => *value,
            Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
            Value::Length(value) => Rel { rel: 0.0, abs: *value },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "expected relative length, found {}",
                        vanilla_type_name_class(other)
                    ),
                )])
            }
        };
    }
    if let Some(value) = args.named.get("dotless") {
        explicit.dotless = true;
        dotless = match value {
            Value::Bool(value) => *value,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected boolean, found {}", vanilla_type_name_class(other)),
                )])
            }
        };
    }
    Ok(Content::math_accent_full(
        math_accent_base(base, args.span)?,
        math_accent_char(accent, args.span)?,
        size,
        dotless,
        explicit,
    ))
}

pub fn native_accent(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_accent_content(args).map(Value::Content)
}

/// `cancel(body)` — emite `Content::MathCancel { body }`.
/// Body posicional obrigatório (content ou string).
pub fn native_cancel(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    };

    let mut length = Rel::from_percent(100.0) + Length::em(0.3);
    let mut inverted = false;
    let mut cross = false;
    let mut angle = MathCancelAngle::Auto;
    let mut stroke = None;
    let mut background = false;
    let mut explicit = MathCancelExplicit::default();
    for (name, value) in &args.named {
        match name.as_str() {
            "length" => {
                explicit.length = true;
                match value {
                    Value::Relative(value) => length = *value,
                    Value::Length(value) => length = Rel { rel: 0.0, abs: *value },
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected relative length, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "inverted" => {
                explicit.inverted = true;
                match value {
                    Value::Bool(value) => inverted = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "cross" => {
                explicit.cross = true;
                match value {
                    Value::Bool(value) => cross = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "angle" => {
                explicit.angle = true;
                match value {
                    Value::Auto => angle = MathCancelAngle::Auto,
                    Value::Angle(value) => angle = MathCancelAngle::Angle(*value),
                    Value::Func(value) => angle = MathCancelAngle::Func(value.clone()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected angle, function, or auto, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "stroke" => {
                explicit.stroke = true;
                match value {
                    Value::None => stroke = None,
                    other => {
                        stroke = Some(crate::compiler::stdlib::layout::extract_stroke(
                            other, "cancel", "stroke",
                        )?);
                    }
                }
            }
            "background" => {
                explicit.background = true;
                match value {
                    Value::Bool(value) => background = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    Ok(Value::Content(Content::math_cancel_full(
        body, length, inverted, cross, angle, stroke, background, args.span, explicit,
    )))
}

/// `math.underline(body)` — construtor body-only do elemento matemático.
pub fn native_math_underline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(body)] => body.clone(),
        [Value::Str(text)] => Content::text(text.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    };

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(Value::Content(Content::math_underline(body)))
}

/// Valores cascatiáveis do construtor canónico de `math.vec`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct MathVecOptions {
    pub delim: (char, char),
    pub align: HAlign,
    pub gap: Rel<Length>,
    pub explicit: MathVecExplicit,
}

impl Default for MathVecOptions {
    fn default() -> Self {
        Self {
            delim: ('(', ')'),
            align: HAlign::Center,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathVecExplicit::default(),
        }
    }
}

fn vec_delim_char(value: &Value) -> Option<char> {
    match value {
        Value::None => Some('\0'),
        Value::Str(text) => text.chars().next(),
        Value::Symbol(symbol) => symbol.value.chars().next(),
        _ => None,
    }
}

fn vec_delim_string(text: &str) -> (char, char) {
    match text {
        "(" | ")" => ('(', ')'),
        "[" | "]" => ('[', ']'),
        "{" | "}" => ('{', '}'),
        "|" => ('|', '|'),
        "||" | "|||" => ('‖', '‖'),
        "⌊" | "floor" => ('⌊', '⌋'),
        "⌈" | "ceil" => ('⌈', '⌉'),
        "<" | ">" | "chevron" => ('⟨', '⟩'),
        "" | "none" => ('\0', '\0'),
        other => {
            let mut chars = other.chars();
            let left = chars.next().unwrap_or('\0');
            (left, chars.next().unwrap_or(left))
        }
    }
}

pub(crate) fn math_vec_delim(value: &Value, span: Span) -> SourceResult<(char, char)> {
    match value {
        Value::None => Ok(('\0', '\0')),
        Value::Str(text) => Ok(vec_delim_string(text.as_str())),
        Value::Symbol(symbol) => Ok(vec_delim_string(symbol.value.as_str())),
        Value::Array(values) => {
            let left = values.first().and_then(vec_delim_char).unwrap_or('\0');
            let right = values.get(1).and_then(vec_delim_char).unwrap_or(left);
            Ok((left, right))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected array, none, symbol, or string, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

/// Aplica um named de `math.vec`; set rules usam `explicit = false`.
pub(crate) fn apply_math_vec_option(
    options: &mut MathVecOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_vec_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "align" => {
            let Value::Align(align) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected alignment, found {}",
                        vanilla_type_name_class(value)
                    ),
                )]);
            };
            let Some(horizontal) = align.h.filter(|_| align.v.is_none()) else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected horizontal alignment".to_string(),
                )]);
            };
            options.align = horizontal;
            options.explicit.align |= explicit;
        }
        "gap" => {
            options.gap = match value {
                Value::Relative(gap) => *gap,
                Value::Ratio(gap) => Rel { rel: gap.get(), abs: Length::ZERO },
                Value::Length(gap) => Rel { rel: 0.0, abs: *gap },
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "expected relative length, found {}",
                            vanilla_type_name_class(other)
                        ),
                    )])
                }
            };
            options.explicit.gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
    }
    Ok(())
}

pub(crate) fn math_vec_content(
    children: Vec<Content>,
    options: MathVecOptions,
) -> Content {
    Content::math_vec_full(
        children,
        options.delim,
        options.align,
        options.gap,
        options.explicit,
    )
}

/// `math.vec(..children, delim:, align:, gap:)`.
pub fn native_math_vec(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut children = Vec::with_capacity(args.items.len());
    for value in &args.items {
        let child = match value {
            Value::Content(content) => content.clone(),
            Value::LocatedContent(content, _) => content.content().clone(),
            Value::Str(text) => Content::text(text.as_str()),
            Value::Symbol(symbol) => Content::MathText(symbol.value.clone()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected content, found {}", vanilla_type_name_class(other)),
                )])
            }
        };
        children.push(child);
    }

    let mut options = MathVecOptions::default();
    for (name, value) in &args.named {
        apply_math_vec_option(&mut options, name, value, args.span, true)?;
    }
    Ok(Value::Content(math_vec_content(children, options)))
}

#[derive(Debug, Clone)]
pub(crate) struct MathMatrixOptions {
    pub delim: (Option<char>, Option<char>),
    pub align: HAlign,
    pub augment: Option<MathMatrixAugment>,
    pub row_gap: Rel<Length>,
    pub column_gap: Rel<Length>,
    pub explicit: MathMatrixExplicit,
}

impl Default for MathMatrixOptions {
    fn default() -> Self {
        Self {
            delim: (Some('('), Some(')')),
            align: HAlign::Center,
            augment: None,
            row_gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            column_gap: Rel { rel: 0.0, abs: Length::em(0.5) },
            explicit: MathMatrixExplicit::default(),
        }
    }
}

fn math_matrix_rel(value: &Value, span: Span) -> SourceResult<Rel<Length>> {
    match value {
        Value::Relative(value) => Ok(*value),
        Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
        Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected relative length, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

fn math_matrix_offsets(value: &Value, span: Span) -> SourceResult<Vec<i64>> {
    match value {
        Value::Int(value) => Ok(vec![*value]),
        Value::Array(values) => values
            .iter()
            .map(|value| match value {
                Value::Int(value) => Ok(*value),
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected integer, found {}", vanilla_type_name_class(other)),
                )]),
            })
            .collect(),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer or array of integers, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

pub(crate) fn math_matrix_augment(
    value: &Value,
    span: Span,
) -> SourceResult<Option<MathMatrixAugment>> {
    match value {
        Value::None => Ok(None),
        Value::Int(offset) => Ok(Some(MathMatrixAugment {
            hline: vec![],
            vline: vec![*offset],
            stroke: None,
        })),
        Value::Dict(dict) => {
            let hline = dict
                .get("hline")
                .map(|value| math_matrix_offsets(value, span))
                .transpose()?
                .unwrap_or_default();
            let vline = dict
                .get("vline")
                .map(|value| math_matrix_offsets(value, span))
                .transpose()?
                .unwrap_or_default();
            let stroke = match dict.get("stroke") {
                None | Some(Value::Auto) => None,
                Some(value) => Some(crate::compiler::stdlib::layout::extract_stroke(
                    value,
                    "math.mat",
                    "augment.stroke",
                )?),
            };
            Ok(Some(MathMatrixAugment { hline, vline, stroke }))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer, dictionary, or none, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

pub(crate) fn apply_math_matrix_option(
    options: &mut MathMatrixOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_cases_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "align" => {
            let Value::Align(align) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected alignment, found {}",
                        vanilla_type_name_class(value)
                    ),
                )]);
            };
            let Some(horizontal) = align.h.filter(|_| align.v.is_none()) else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected start, left, center, right, or end".to_string(),
                )]);
            };
            options.align = horizontal;
            options.explicit.align |= explicit;
        }
        "augment" => {
            options.augment = math_matrix_augment(value, span)?;
            options.explicit.augment |= explicit;
        }
        "gap" => {
            let gap = math_matrix_rel(value, span)?;
            options.row_gap = gap;
            options.column_gap = gap;
            options.explicit.row_gap |= explicit;
            options.explicit.column_gap |= explicit;
        }
        "row-gap" => {
            options.row_gap = math_matrix_rel(value, span)?;
            options.explicit.row_gap |= explicit;
        }
        "column-gap" => {
            options.column_gap = math_matrix_rel(value, span)?;
            options.explicit.column_gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
    }
    Ok(())
}

fn math_matrix_cell(value: &Value) -> Content {
    match value {
        Value::Content(content) => content.clone(),
        Value::LocatedContent(content, _) => content.content().clone(),
        Value::None => Content::Empty,
        Value::Symbol(symbol) => Content::MathText(symbol.value.clone()),
        other => crate::compiler::stdlib::state::value_to_content(other),
    }
}

fn math_matrix_delimited_row(value: &Value) -> Option<Vec<Content>> {
    let Value::Content(Content::MathDelimited(delimited)) = value else {
        return None;
    };
    if delimited.open != '(' || delimited.close != ')' {
        return None;
    }
    let nodes: Vec<Content> = match &delimited.body {
        Content::MathSequence(nodes) => nodes.iter().cloned().collect(),
        Content::Empty => Vec::new(),
        other => vec![other.clone()],
    };
    let mut cells = Vec::new();
    let mut current = Vec::new();
    for node in nodes {
        let comma = matches!(
            &node,
            Content::MathIdent(text) | Content::MathText(text) if text.as_str() == ","
        );
        if comma {
            cells.push(math_matrix_row_cell(std::mem::take(&mut current)));
        } else {
            current.push(node);
        }
    }
    cells.push(math_matrix_row_cell(current));
    Some(cells)
}

fn math_matrix_row_cell(mut nodes: Vec<Content>) -> Content {
    match nodes.len() {
        0 => Content::Empty,
        1 => nodes.pop().unwrap(),
        _ => Content::MathSequence(nodes.into()),
    }
}

pub(crate) fn math_matrix_content(args: &Args) -> SourceResult<Content> {
    const NAMES: &[&str] = &["delim", "align", "augment", "gap", "row-gap", "column-gap"];
    for name in NAMES {
        let count = args
            .occurrence_sequence()
            .iter()
            .filter(|occurrence| occurrence.name.as_deref() == Some(*name))
            .count();
        if count > 1 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("duplicate argument: {name}"),
            )]);
        }
    }
    for name in args.named.keys() {
        if !NAMES.contains(&name.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }

    let mut options = MathMatrixOptions::default();
    if let Some(value) = args.named.get("gap") {
        apply_math_matrix_option(&mut options, "gap", value, args.span, true)?;
    }
    for name in ["delim", "align", "augment", "row-gap", "column-gap"] {
        if let Some(value) = args.named.get(name) {
            apply_math_matrix_option(&mut options, name, value, args.span, true)?;
        }
    }

    let array_mode = args.items.iter().any(|value| matches!(value, Value::Array(_)));
    let delimited_mode = args
        .items
        .iter()
        .any(|value| math_matrix_delimited_row(value).is_some());
    let mut rows = if array_mode || delimited_mode {
        args.items
            .iter()
            .map(|value| match value {
                Value::Array(values) if array_mode && !delimited_mode => {
                    Ok(values.iter().map(math_matrix_cell).collect::<Vec<_>>())
                }
                _ if delimited_mode && !array_mode => math_matrix_delimited_row(value)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected array, found {}",
                                vanilla_type_name_class(value)
                            ),
                        )]
                    }),
                _ => Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected array, found {}", vanilla_type_name_class(value)),
                )]),
            })
            .collect::<SourceResult<Vec<_>>>()?
    } else {
        vec![args.items.iter().map(math_matrix_cell).collect()]
    };
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    for row in &mut rows {
        row.resize(columns, Content::Empty);
    }

    let validate = |offsets: &[i64], dimension: usize, kind: &str| -> SourceResult<()> {
        if let Some(offset) = offsets
            .iter()
            .find(|offset| offset.unsigned_abs() as usize > dimension)
        {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{kind} offset {offset} exceeds dimension {dimension}"),
            )]);
        }
        Ok(())
    };
    if (array_mode || delimited_mode)
        && !args.span.is_detached()
        && !rows.is_empty()
        && columns > 0
    {
        if let Some(augment) = &options.augment {
            validate(&augment.hline, rows.len(), "horizontal line")?;
            validate(&augment.vline, columns, "vertical line")?;
        }
    }

    Ok(Content::math_matrix_complete(
        rows,
        options.delim,
        options.align,
        options.augment,
        options.row_gap,
        options.column_gap,
        options.explicit,
    ))
}

pub fn native_math_mat(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_matrix_content(args).map(Value::Content)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MathCasesOptions {
    pub delim: (Option<char>, Option<char>),
    pub reverse: bool,
    pub gap: Rel<Length>,
    pub explicit: MathCasesExplicit,
}

impl Default for MathCasesOptions {
    fn default() -> Self {
        Self {
            delim: (Some('{'), Some('}')),
            reverse: false,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathCasesExplicit::default(),
        }
    }
}

fn math_cases_delim_char(value: &Value, span: Span) -> SourceResult<Option<char>> {
    let text = match value {
        Value::None => return Ok(None),
        Value::Str(text) => text.as_str(),
        Value::Symbol(symbol) => symbol.value.as_str(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected none, symbol, or string, found {}",
                    vanilla_type_name_class(other)
                ),
            )])
        }
    };
    let mut chars = text.chars();
    let Some(character) = chars.next() else {
        return Err(vec![SourceDiagnostic::error(span, "expected a single delimiter")]);
    };
    if chars.next().is_some() {
        return Err(vec![SourceDiagnostic::error(span, "expected a single delimiter")]);
    }
    if !matches!(
        crate::entities::math_class::default_math_class(character),
        Some(
            crate::entities::math_class::MathClass::Opening
                | crate::entities::math_class::MathClass::Closing
                | crate::entities::math_class::MathClass::Fence
        )
    ) {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("invalid delimiter: \"{character}\""),
        )]);
    }
    Ok(Some(character))
}

fn math_cases_matching_delim(character: char) -> char {
    match character {
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '⌊' => '⌋',
        '⌋' => '⌊',
        '⌈' => '⌉',
        '⌉' => '⌈',
        '⟨' | '<' => '⟩',
        '⟩' | '>' => '⟨',
        other => other,
    }
}

pub(crate) fn math_cases_delim(
    value: &Value,
    span: Span,
) -> SourceResult<(Option<char>, Option<char>)> {
    match value {
        Value::None => Ok((None, None)),
        Value::Str(_) | Value::Symbol(_) => {
            let single = math_cases_delim_char(value, span)?;
            Ok(single.map_or((None, None), |character| {
                let matching = math_cases_matching_delim(character);
                match crate::entities::math_class::default_math_class(character) {
                    Some(crate::entities::math_class::MathClass::Closing) => {
                        (Some(matching), Some(character))
                    }
                    _ => (Some(character), Some(matching)),
                }
            }))
        }
        Value::Array(values) => {
            let [left, right] = values.as_slice() else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected exactly two delimiters".to_string(),
                )]);
            };
            Ok((math_cases_delim_char(left, span)?, math_cases_delim_char(right, span)?))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected array, none, symbol, or string, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

pub(crate) fn apply_math_cases_option(
    options: &mut MathCasesOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_cases_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "reverse" => {
            let Value::Bool(reverse) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected boolean, found {}", vanilla_type_name_class(value)),
                )]);
            };
            options.reverse = *reverse;
            options.explicit.reverse |= explicit;
        }
        "gap" => {
            options.gap = match value {
                Value::Relative(gap) => *gap,
                Value::Ratio(gap) => Rel { rel: gap.get(), abs: Length::ZERO },
                Value::Length(gap) => Rel { rel: 0.0, abs: *gap },
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "expected relative length, found {}",
                            vanilla_type_name_class(other)
                        ),
                    )])
                }
            };
            options.explicit.gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
    }
    Ok(())
}

fn math_cases_child(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(Content::MathIdent(text)) => Ok(Content::MathText(text.clone())),
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => match content.content() {
            Content::MathIdent(text) => Ok(Content::MathText(text.clone())),
            content => Ok(content.clone()),
        },
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

pub(crate) fn math_cases_content(args: &Args) -> SourceResult<Content> {
    for name in ["delim", "reverse", "gap"] {
        let count = args
            .occurrence_sequence()
            .iter()
            .filter(|occurrence| occurrence.name.as_deref() == Some(name))
            .count();
        if count > 1 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("duplicate argument: {name}"),
            )]);
        }
    }
    let mut options = MathCasesOptions::default();
    for (name, value) in &args.named {
        apply_math_cases_option(&mut options, name, value, args.span, true)?;
    }
    let children = args
        .items
        .iter()
        .map(|value| math_cases_child(value, args.span))
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(Content::math_cases_full(
        children,
        options.delim,
        options.reverse,
        options.gap,
        options.explicit,
    ))
}

pub fn native_math_cases(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_cases_content(args).map(Value::Content)
}

fn math_lr_content_value(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// Validação e construção compartilhada de `math.lr` e `lr(...)` bare.
fn math_lr_size_from_args(args: &Args) -> SourceResult<(Rel<Length>, bool)> {
    let size_occurrences = args
        .occurrence_sequence()
        .iter()
        .filter(|occurrence| occurrence.name.as_deref() == Some("size"))
        .count();
    if size_occurrences > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "duplicate argument: size".to_string(),
        )]);
    }
    for name in args.named.keys() {
        if name != "size" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let (size, size_explicit) = match args.named.get("size") {
        None => (Rel::from_percent(100.0), false),
        Some(Value::Relative(size)) => (*size, true),
        Some(Value::Ratio(size)) => (Rel { rel: size.get(), abs: Length::ZERO }, true),
        Some(Value::Length(size)) => (Rel { rel: 0.0, abs: *size }, true),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "expected relative length, found {}",
                    vanilla_type_name_class(other)
                ),
            )])
        }
    };
    Ok((size, size_explicit))
}

/// Validação e construção compartilhada de `math.lr` e `lr(...)` bare.
pub(crate) fn math_lr_from_args(args: &Args) -> SourceResult<Content> {
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let mut body =
        Vec::with_capacity(args.items.len().saturating_mul(2).saturating_sub(1));
    for (index, value) in args.items.iter().enumerate() {
        if index > 0 {
            body.push(Content::text(","));
        }
        body.push(math_lr_content_value(value, args.span)?);
    }
    let body = match body.len() {
        0 => Content::Empty,
        1 => body.pop().unwrap(),
        _ => Content::MathSequence(body.into()),
    };
    Ok(Content::math_lr(body, size, size_explicit))
}

pub fn native_math_lr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_lr_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.mid` e `mid(...)` bare.
pub(crate) fn math_mid_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_mid(body))
}

pub fn native_math_mid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_mid_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overbrace` e da forma bare.
pub(crate) fn math_overbrace_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overbrace(body, annotation, annotation_explicit))
}

pub fn native_math_overbrace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overbrace_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underbrace` e da forma bare.
pub(crate) fn math_underbrace_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underbrace(body, annotation, annotation_explicit))
}

pub fn native_math_underbrace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underbrace_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underbracket` e da forma bare.
pub(crate) fn math_underbracket_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underbracket(body, annotation, annotation_explicit))
}

pub fn native_math_underbracket(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underbracket_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.underparen` e da forma bare.
pub(crate) fn math_underparen_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_underparen(body, annotation, annotation_explicit))
}

pub fn native_math_underparen(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_underparen_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.undershell` e da forma bare.
pub(crate) fn math_undershell_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_undershell(body, annotation, annotation_explicit))
}

pub fn native_math_undershell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_undershell_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overbracket` e da forma bare.
pub(crate) fn math_overbracket_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overbracket(body, annotation, annotation_explicit))
}

pub fn native_math_overbracket(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overbracket_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overparen` e da forma bare.
pub(crate) fn math_overparen_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overparen(body, annotation, annotation_explicit))
}

pub fn native_math_overparen(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overparen_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.overshell` e da forma bare.
pub(crate) fn math_overshell_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (body, annotation, annotation_explicit) = match args.items.as_slice() {
        [body] => (math_lr_content_value(body, args.span)?, None, false),
        [body, Value::None] => (math_lr_content_value(body, args.span)?, None, true),
        [body, annotation @ (Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Str(_)
        | Value::Symbol(_))] => (
            math_lr_content_value(body, args.span)?,
            Some(math_lr_content_value(annotation, args.span)?),
            true,
        ),
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _] | [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overshell(body, annotation, annotation_explicit))
}

pub fn native_math_overshell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overshell_from_args(args).map(Value::Content)
}

/// `math.primes(count)` — grupo semântico de apóstrofos matemáticos.
pub fn native_math_primes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("count") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `count` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let count = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: count".to_string(),
            )])
        }
        [Value::Int(count)] if *count < 0 => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "number must be at least zero".to_string(),
            )])
        }
        [Value::Int(count)] => usize::try_from(*count).map_err(|_| {
            vec![SourceDiagnostic::error(args.span, "number is too large".to_string())]
        })?,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", vanilla_type_name_class(other)),
            )])
        }
        [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Value::Content(Content::math_primes(count)))
}

/// Validação e construção compartilhada de `math.overline` e da forma bare.
pub(crate) fn math_overline_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let body = match args.items.as_slice() {
        [body] => math_lr_content_value(body, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        [_, _, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_overline(body))
}

pub fn native_math_overline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_overline_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.norm` e `norm(...)` bare.
pub(crate) fn math_norm_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_lr(
        Content::MathSequence(
            vec![Content::MathText("‖".into()), body, Content::MathText("‖".into())]
                .into(),
        ),
        size,
        size_explicit,
    ))
}

pub fn native_math_norm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_norm_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.round` e `round(...)` bare.
pub(crate) fn math_round_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_lr(
        Content::MathSequence(
            vec![Content::MathText("⌊".into()), body, Content::MathText("⌉".into())]
                .into(),
        ),
        size,
        size_explicit,
    ))
}

pub fn native_math_round(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_round_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.abs` e `abs(...)` bare.
pub(crate) fn math_abs_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )
        .with_hint("try removing `body:`")]);
    }
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_lr(
        Content::MathSequence(
            vec![Content::MathText("|".into()), body, Content::MathText("|".into())]
                .into(),
        ),
        size,
        size_explicit,
    ))
}

pub fn native_math_abs(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_abs_from_args(args).map(Value::Content)
}

/// Validação e construção compartilhada de `math.stretch` e da forma bare.
pub(crate) fn math_stretch_from_args(args: &Args) -> SourceResult<Content> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )
        .with_hint("try removing `body:`")]);
    }
    let (size, size_explicit) = math_lr_size_from_args(args)?;
    let body = match args.items.as_slice() {
        [value] => math_lr_content_value(value, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };
    Ok(Content::math_stretch(body, size, size_explicit))
}

pub fn native_math_stretch(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_stretch_from_args(args).map(Value::Content)
}

fn p1293_math_content(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(content) => Ok(content.clone()),
        Value::LocatedContent(content, _) => Ok(content.content().clone()),
        Value::Str(text) => Ok(Content::MathText(text.clone())),
        Value::Symbol(symbol) => Ok(Content::MathText(symbol.value.clone())),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

fn p1293_math_attach_slot(value: &Value, span: Span) -> SourceResult<MathAttachSlot> {
    match value {
        Value::None => Ok(MathAttachSlot::ExplicitNone),
        Value::Content(content) => Ok(MathAttachSlot::Present(content.clone())),
        Value::LocatedContent(content, _) => {
            Ok(MathAttachSlot::Present(content.content().clone()))
        }
        Value::Str(text) => Ok(MathAttachSlot::Present(Content::MathText(text.clone()))),
        Value::Symbol(symbol) => {
            Ok(MathAttachSlot::Present(Content::MathText(symbol.value.clone())))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content or none, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// Construtor puro canônico de `attach`, compartilhado por sintaxe e função.
pub(crate) fn math_attach_content(
    base: Content,
    t: MathAttachSlot,
    b: MathAttachSlot,
    tl: MathAttachSlot,
    bl: MathAttachSlot,
    tr: MathAttachSlot,
    br: MathAttachSlot,
) -> Content {
    Content::math_attach(base, t, b, tl, bl, tr, br)
}

/// Validação fechada e construção única de `attach` para os dois caminhos.
pub(crate) fn math_attach_from_args(args: &Args) -> SourceResult<Content> {
    let base = match args.items.as_slice() {
        [base] => p1293_math_content(base, args.span)?,
        [] if args.named.contains_key("base") => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `base` is positional".to_string(),
            )
            .with_hint("try removing `base:`")])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: base".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    let mut t = MathAttachSlot::Omitted;
    let mut b = MathAttachSlot::Omitted;
    let mut tl = MathAttachSlot::Omitted;
    let mut bl = MathAttachSlot::Omitted;
    let mut tr = MathAttachSlot::Omitted;
    let mut br = MathAttachSlot::Omitted;
    for (name, value) in &args.named {
        match name.as_str() {
            "t" => t = p1293_math_attach_slot(value, args.span)?,
            "b" => b = p1293_math_attach_slot(value, args.span)?,
            "tl" => tl = p1293_math_attach_slot(value, args.span)?,
            "bl" => bl = p1293_math_attach_slot(value, args.span)?,
            "tr" => tr = p1293_math_attach_slot(value, args.span)?,
            "br" => br = p1293_math_attach_slot(value, args.span)?,
            "base" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "the argument `base` is positional".to_string(),
                )
                .with_hint("try removing `base:`")])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    Ok(math_attach_content(base, t, b, tl, bl, tr, br))
}

/// `math.attach(base, t:, b:, tl:, bl:, tr:, br:)`.
pub fn native_math_attach(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_attach_from_args(args).map(Value::Content)
}

/// Construtor puro canônico da fração sem barra de `binom`.
pub(crate) fn math_binom_content(upper: Content, lower: Vec<Content>) -> Content {
    debug_assert!(!lower.is_empty());
    let mut denominator =
        Vec::with_capacity(lower.len().saturating_mul(2).saturating_sub(1));
    for (index, item) in lower.into_iter().enumerate() {
        if index > 0 {
            denominator.push(Content::MathText(", ".into()));
        }
        denominator.push(item);
    }
    Content::math_delimited(
        '(',
        Content::math_frac_unlined(
            upper,
            Content::MathSequence(std::sync::Arc::from(denominator)),
        ),
        ')',
    )
}

/// Validação fechada e construção única de `binom` para os dois caminhos.
pub(crate) fn math_binom_from_args(args: &Args) -> SourceResult<Content> {
    let Some((upper, lower_values)) = args.items.split_first() else {
        if args.named.contains_key("upper") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `upper` is positional".to_string(),
            )
            .with_hint("try removing `upper:`")]);
        }
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: upper".to_string(),
        )]);
    };
    if lower_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: lower".to_string(),
        )]);
    }
    for name in args.named.keys() {
        if name.as_str() == "upper" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `upper` is positional".to_string(),
            )
            .with_hint("try removing `upper:`")]);
        }
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let upper = p1293_math_content(upper, args.span)?;
    let lower = lower_values
        .iter()
        .map(|value| p1293_math_content(value, args.span))
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(math_binom_content(upper, lower))
}

/// `math.binom(upper, ..lower)` com pelo menos um lower.
pub fn native_math_binom(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_binom_from_args(args).map(Value::Content)
}

#[cfg(test)]
mod p1292_c_vec_tests {
    use super::*;
    use crate::entities::layout_types::Ratio;

    #[test]
    fn pure_ratio_gap_converte_zero_e_dez_porcento() {
        for (percent, expected) in [(0.0, 0.0), (10.0, 0.1)] {
            let mut options = MathVecOptions::default();
            apply_math_vec_option(
                &mut options,
                "gap",
                &Value::Ratio(Ratio::from_percent(percent)),
                Span::detached(),
                true,
            )
            .expect("ratio puro deve converter para relative length");
            assert_eq!(options.gap.rel, expected);
            assert!(options.gap.abs.is_zero());
            assert!(options.explicit.gap);
        }
    }
}

// ── P772y — `math.class(class, body)` ────────────────────────────────────
//
// Força a `MathClass` de `body`, override do valor inferido automaticamente
// por `default_math_class`/`spacing::node_math_class`. Afecta apenas o
// espaçamento automático (`rules/math/layout/spacing.rs`); `body` é
// layoutado normalmente (`MathLayouter::layout_node`). Vanilla `ClassElem`
// (`math/mod.rs`).

/// **P825** — nome do tipo no formato longo do vanilla para mensagens de
/// cast (`found integer` etc.): `str`→`string`, `int`→`integer`,
/// `bool`→`boolean`; os restantes coincidem com `type_name()` (mesma
/// convenção de `loading.rs:561`/`pdf.rs:88`).
fn vanilla_type_name_class(v: &Value) -> &'static str {
    match v {
        Value::Str(_) => "string",
        Value::Int(_) => "integer",
        Value::Bool(_) => "boolean",
        other => other.type_name(),
    }
}

/// `class(class, body)` — emite `Content::MathClassOverride { class, body }`.
/// `class` é obrigatório e uma das **10 strings do cast vanilla**
/// (P825 — `foundations/cast.rs:502-520`; as outras 5 variantes do enum —
/// `alphabetic`, `diacritic`, `glyph-part`, `space`, `special` — são
/// internas e rejeitadas no cast); `body` é obrigatório (content ou
/// string).
pub fn native_math_class(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // **P825** — domínio do cast do vanilla + mensagem verbatim.
    const CAST_DOMAIN: [&str; 10] = [
        "normal",
        "punctuation",
        "opening",
        "closing",
        "fence",
        "large",
        "relation",
        "unary",
        "binary",
        "vary",
    ];
    const CAST_MSG: &str = "expected \"normal\", \"punctuation\", \"opening\", \
         \"closing\", \"fence\", \"large\", \"relation\", \"unary\", \"binary\", \
         or \"vary\"";

    let class_name = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}, found {}", CAST_MSG, vanilla_type_name_class(other)),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige o nome da classe como 1.º argumento posicional"
                    .to_string(),
            )])
        }
    };
    if !CAST_DOMAIN.contains(&class_name.as_str()) {
        return Err(vec![SourceDiagnostic::error(Span::detached(), CAST_MSG)]);
    }
    // Domínio verificado acima — `parse_math_class` reconhece necessariamente.
    let class = crate::entities::math_class::parse_math_class(class_name.as_str())
        .expect("domínio do cast vanilla é subset dos 15 nomes reconhecidos");

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        // P772y — `math.class("relation", sym.suit.heart)`: símbolo
        // Unicode como body (paridade com a conversão de markup, P471).
        Some(Value::Symbol(s)) => Content::Text(s.value.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "class() body espera content, string ou symbol, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige body como 2.º argumento posicional".to_string(),
            )])
        }
    };

    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("class(): argumento nomeado '{}' não suportado", k),
        )]);
    }

    Ok(Value::Content(Content::math_class_override(class, body)))
}

// ── Passo 297 — `underover()` math (P296.1) ──────────────────────────────
//
// **HV'.a (A.0.0 N=5)**: vanilla typst NÃO tem `UnderoverElem`
// unificado — fragmenta em 12 elementos (UnderlineElem/OverlineElem/
// UnderbraceElem/OverbraceElem/etc.). Cristalino agrega num único
// variant `MathUnderover { base, under?, over? }` per ADR-0054
// graded.
//
// **A.2 → (b) Option `Box<Content>` estrutural**: primeira
// qualificação genuína "variant rico" N=5 desde P287 refutação.
// Promoção ADR meta adiada per P273.17 §0.

/// `underover(base, under: ?, over: ?)` — emite
/// `Content::MathUnderover`. Base posicional; under/over named
/// opcionais.
pub fn native_underover(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let base = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "underover() base espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "underover() exige base como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "under"/"over" permitidos.
    for k in args.named.keys() {
        if !["under", "over"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("underover(): argumento nomeado inesperado '{}' (válidos: under, over)", k),
            )]);
        }
    }

    let under = args.named.get("under").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });
    let over = args.named.get("over").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    Ok(Value::Content(Content::math_underover(base, under, over)))
}

// ── Passo 298 — `op()` math (P296.2 fecho cluster math 4/4) ───────────────
//
// **HV'' adaptado (A.0.0 N=6 magnitude alta)**: cristalino tinha
// heurística limits-style hardcoded em `attach.rs` via
// `symbols::is_limit_function`/`is_large_operator`. P298 estende
// para suportar `MathOp { limits: true }` user-customizable.
//
// **Cross-variant interaction inaugural**: `MathOp.limits` afecta
// layout de `MathAttach` (modificação `is_limits` em `attach.rs`).
// Heurística pré-P298 preservada — fallback `MathIdent("lim")`
// continua a funcionar.

/// `op(text, limits: false)` — emite `Content::MathOp { text, limits }`.
/// Text posicional obrigatório; `limits` named opcional (default `false`).
///
/// **P1030** — passou a `native_with_engine` para poder ler
/// `custom("math.op.limits")` da chain quando o argumento explícito está
/// ausente (`#set math.op(limits: true)`, `eval.md` §P1030). É o **consumidor**
/// a ler a chain — a alternativa (o dispatcher alimentar a native) está
/// rejeitada desde P365, que a removeu em nome de fonte única.
pub fn native_op(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    let text = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "op() text espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "op() exige text como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "limits" permitido.
    for k in args.named.keys() {
        if k.as_str() != "limits" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(): argumento nomeado inesperado '{}' (válido: limits)", k),
            )]);
        }
    }

    let limits = match args.named.get("limits") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(limits:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        // **P1030** — sem argumento explícito, a chain decide; sem chain, o
        // default `false`. Precedência arg > chain > default.
        None => match engine.styles.custom("math.op.limits") {
            Some(Value::Bool(b)) => *b,
            _ => false,
        },
    };

    Ok(Value::Content(Content::math_op(text, limits)))
}

// ── Passo 299 — `math` module: operadores pré-definidos (P298.X) ───────────
//
// **A.0.0 N=7 magnitude baixa-modesta**: cristalino tem `calc`
// module precedente claro (P283) via `make_calc_module()` paralelo
// `make_math_module()`. **41 operadores vanilla** registados como
// `Value::Content(Content::MathOp { ... })` — 1ª aplicação prática
// de MathOp pós-materialização P298.
//
// Lista canónica vanilla `lab/.../math/op.rs:62-105`:
// - 29 scripts-style.
// - 12 limits-style.
//
// Acesso user-facing: `math.sin x`, `math.lim_(x→0) f` (namespaced).
// Heurística pré-P299 preservada — `MathIdent("lim")` literal
// continua a funcionar via fallback `is_limit_function`.

fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::math_op(Content::text(text), limits))
}

/// `scripts(body)` — força attachments laterais reutilizando o constructor
/// canónico de `MathLimitsOverride`.
fn native_math_scripts(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    match args.items.as_slice() {
        [Value::Content(body)] => Ok(Value::Content(Content::math_limits_override(
            body.clone(),
            false,
            true,
            false,
        ))),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("expected content, found {}", vanilla_type_name_class(other)),
        )]),
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: body".to_string(),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]),
    }
}

/// `math.limits(body, inline: true)` — constructor qualificado convergente
/// com a sintaxe bare, inclusive na presença causal do named `inline`.
fn native_math_limits(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional".to_string(),
        )]);
    }

    let inline_explicit = args.named.contains_key("inline");
    let inline = match args.named.get("inline") {
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", vanilla_type_name_class(other)),
            )])
        }
        None => true,
    };

    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "inline")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let body = match args.items.as_slice() {
        [Value::Content(body)] => body.clone(),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    Ok(Value::Content(Content::math_limits_override(body, true, inline, inline_explicit)))
}

/// Constrói o módulo `math` como `Value::Module` com 41 operadores
/// vanilla pré-definidos (paralelo `make_calc_module()` P283).
pub fn make_math_module() -> Value {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    // Scripts-style operators (29) — limits: false.
    for name in [
        "arccos", "arcsin", "arctan", "arg", "cos", "cosh", "cot", "coth", "csc", "csch",
        "ctg", "deg", "dim", "exp", "hom", "id", "im", "ker", "lg", "ln", "log", "mod",
        "sec", "sech", "sin", "sinc", "sinh", "tan", "tanh", "tg", "tr",
    ] {
        dict.insert(name.into(), op_value(name, false));
    }

    // Limits-style operators (12) — limits: true. Multi-word usam
    // o espaço fino U+2009 do literal vanila (e.g. `liminf` → "lim inf").
    for (name, text) in [
        ("det", "det"),
        ("gcd", "gcd"),
        ("lcm", "lcm"),
        ("inf", "inf"),
        ("lim", "lim"),
        ("liminf", "lim\u{2009}inf"),
        ("limsup", "lim\u{2009}sup"),
        ("max", "max"),
        ("min", "min"),
        ("Pr", "Pr"),
        ("sup", "sup"),
    ] {
        dict.insert(name.into(), op_value(text, true));
    }

    // P1140.3-B — primeira fase pública do elemento (body + block).
    dict.insert(
        "equation".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.equation",
            native_math_equation,
        )),
    );

    // P1140.3-A — função pública antes da cópia sym→math; a cópia nunca
    // sobrescreve bindings já definidos.
    dict.insert(
        "sqrt".into(),
        Value::Func(crate::entities::func::Func::native("math.sqrt", native_math_sqrt)),
    );
    dict.insert(
        "root".into(),
        Value::Func(crate::entities::func::Func::native("root", native_math_root)),
    );
    dict.insert(
        "frac".into(),
        Value::Func(crate::entities::func::Func::native("math.frac", native_math_frac)),
    );
    dict.insert(
        "accent".into(),
        Value::Func(crate::entities::func::Func::native("accent", native_accent)),
    );

    // P1292-A — a função canónica existente também é membro de `math`.
    // O binding precede o espelho sym→math, que nunca o sobrescreve.
    dict.insert(
        "cancel".into(),
        Value::Func(crate::entities::func::Func::native("math.cancel", native_cancel)),
    );

    dict.insert(
        "underline".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.underline",
            native_math_underline,
        )),
    );

    dict.insert(
        "vec".into(),
        Value::Func(crate::entities::func::Func::native("math.vec", native_math_vec)),
    );
    dict.insert(
        "mat".into(),
        Value::Func(crate::entities::func::Func::native("mat", native_math_mat)),
    );
    dict.insert(
        "cases".into(),
        Value::Func(crate::entities::func::Func::native("cases", native_math_cases)),
    );
    dict.insert(
        "lr".into(),
        Value::Func(crate::entities::func::Func::native("lr", native_math_lr)),
    );
    dict.insert(
        "mid".into(),
        Value::Func(crate::entities::func::Func::native("mid", native_math_mid)),
    );
    dict.insert(
        "norm".into(),
        Value::Func(crate::entities::func::Func::native("norm", native_math_norm)),
    );
    dict.insert(
        "round".into(),
        Value::Func(crate::entities::func::Func::native("round", native_math_round)),
    );
    dict.insert(
        "abs".into(),
        Value::Func(crate::entities::func::Func::native("abs", native_math_abs)),
    );
    dict.insert(
        "stretch".into(),
        Value::Func(crate::entities::func::Func::native("stretch", native_math_stretch)),
    );
    dict.insert(
        "overbrace".into(),
        Value::Func(crate::entities::func::Func::native(
            "overbrace",
            native_math_overbrace,
        )),
    );
    dict.insert(
        "underbrace".into(),
        Value::Func(crate::entities::func::Func::native(
            "underbrace",
            native_math_underbrace,
        )),
    );
    dict.insert(
        "underbracket".into(),
        Value::Func(crate::entities::func::Func::native(
            "underbracket",
            native_math_underbracket,
        )),
    );
    dict.insert(
        "underparen".into(),
        Value::Func(crate::entities::func::Func::native(
            "underparen",
            native_math_underparen,
        )),
    );
    dict.insert(
        "undershell".into(),
        Value::Func(crate::entities::func::Func::native(
            "undershell",
            native_math_undershell,
        )),
    );
    dict.insert(
        "overbracket".into(),
        Value::Func(crate::entities::func::Func::native(
            "overbracket",
            native_math_overbracket,
        )),
    );
    dict.insert(
        "overline".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.overline",
            native_math_overline,
        )),
    );
    dict.insert(
        "overparen".into(),
        Value::Func(crate::entities::func::Func::native(
            "overparen",
            native_math_overparen,
        )),
    );
    dict.insert(
        "overshell".into(),
        Value::Func(crate::entities::func::Func::native(
            "overshell",
            native_math_overshell,
        )),
    );
    dict.insert(
        "primes".into(),
        Value::Func(crate::entities::func::Func::native("primes", native_math_primes)),
    );

    // P1293-B — quatro membros próprios antes do espelho sym→math.
    dict.insert(
        "attach".into(),
        Value::Func(crate::entities::func::Func::native("attach", native_math_attach)),
    );
    dict.insert(
        "binom".into(),
        Value::Func(crate::entities::func::Func::native("binom", native_math_binom)),
    );
    for (name, native) in [
        ("mono", crate::compiler::stdlib::math_style::native_mono as _),
        ("script", crate::compiler::stdlib::math_style::native_script as _),
    ] {
        dict.insert(
            name.into(),
            Value::Func(crate::entities::func::Func::native(name, native)),
        );
    }

    // P795 — dif e Dif operadores em modo math (expostos no modulo math)
    // **P962** — com wrapper `upright` (`MathStyled { italic: Some(false) }`,
    // o mesmo que `upright(d)` produz): sem ele, `apply_math_default`
    // italicava o "d" para 𝑑 (U+1D451) — o vanilla define
    // `dif = HElem(THIN, weak) + ClassElem(Unary, upright(SymbolElem('d')))`
    // (`math/op.rs:52-56`) e o "d" do diferencial é reto. Ver
    // `stdlib/structural.md` §P962 (o espaço fino fraco + classe Unary do
    // vanilla ficam registados como scope-out nessa secção).
    let differential = |letter: &str| {
        Content::sequence(vec![
            Content::h_space(crate::entities::layout_types::Length::em(1.0 / 6.0), true),
            Content::math_class_override(
                crate::entities::math_class::MathClass::Unary,
                Content::math_styled(
                    None,
                    None,
                    Some(false),
                    Content::MathText(letter.into()),
                    None,
                ),
            ),
        ])
    };
    dict.insert("dif".into(), Value::Content(differential("d")));
    dict.insert("Dif".into(), Value::Content(differential("D")));

    // **P772y** — `math.class(class, body)`: override manual de `MathClass`
    // para efeitos de espaçamento automático. Vive no scope do módulo
    // `math` (não no scope global, ao contrário de `cancel`/`accent`).
    dict.insert(
        "class".into(),
        Value::Func(crate::entities::func::Func::native("class", native_math_class)),
    );

    // **P895** — `math.op(...)`: mesma função nativa `native_op` já registada
    // no scope global (`eval/mod.rs::scope.define("op", ...)`) também
    // acessível via `math.op(...)`, paridade vanilla (namespace `math`
    // reexpõe as funções de operador — achado do catálogo de terceiros em
    // `typst-passo-895-relatorio.md`, Parte B).
    dict.insert(
        "op".into(),
        Value::Func(crate::entities::func::Func::native_with_engine("op", native_op)),
    );

    // P1291 — bindings próprios medidos do namespace math. Os quatro
    // styles reutilizam exactamente as nativas donas; `scripts` é somente
    // o adapter ABI privado para o constructor estrutural existente.
    for (name, native) in [
        ("bb", crate::compiler::stdlib::math_style::native_bb as _),
        ("frak", crate::compiler::stdlib::math_style::native_frak as _),
        ("inline", crate::compiler::stdlib::math_style::native_inline as _),
        ("serif", crate::compiler::stdlib::math_style::native_serif as _),
    ] {
        dict.insert(
            name.into(),
            Value::Func(crate::entities::func::Func::native(name, native)),
        );
    }
    dict.insert(
        "scripts".into(),
        Value::Func(crate::entities::func::Func::native("scripts", native_math_scripts)),
    );
    dict.insert(
        "limits".into(),
        Value::Func(crate::entities::func::Func::native("limits", native_math_limits)),
    );

    // **P895** — espaçamentos nomeados de modo math, nunca registados
    // (paridade vanilla `math/mod.rs:36-40,98-102`: `THIN`/`MEDIUM`/`THICK`
    // = mesmas fracções de em já usadas em `spacing.rs` para o espaçamento
    // automático por `MathClass`; `QUAD`/`WIDE` são 1em/2em). `lookup_math_op`
    // (`eval/math.rs`) resolve identificadores bare em modo math via
    // `Value::Content` no scope do módulo `math` — o mesmo mecanismo que já
    // resolve `dif`/`Dif` acima serve estes cinco sem mudança de código.
    for (name, em) in [
        ("thin", 1.0 / 6.0),
        ("med", 2.0 / 9.0),
        ("thick", 5.0 / 18.0),
        ("quad", 1.0),
        ("wide", 2.0),
    ] {
        dict.insert(
            name.into(),
            Value::Content(Content::h_space(
                crate::entities::layout_types::Length::em(em),
                false,
            )),
        );
    }

    // **P731** — `Value::Module` (paridade vanilla — medido: `type(math)` →
    // `module`), não `Value::Dict`. `eval/math.rs::lookup_math_op` lê o
    // scope do módulo.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    // Vanilla Typst expõe todos os símbolos do módulo `sym` também no módulo `math`.
    if let Value::Module(sym_m) = crate::compiler::stdlib::sym::build_sym_module() {
        for (k, binding) in sym_m.scope().iter() {
            if scope.get(k).is_none() {
                scope.define(k, binding.value().clone());
            }
        }
    }
    Value::Module(crate::entities::module::Module::new("math", scope))
}

#[cfg(test)]
mod p1291_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::math_style::MathStyleKind;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    type NativeFn =
        fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }

    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            test_file_id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    fn math_func(name: &str) -> Func {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        match module.scope().get(name) {
            Some(Value::Func(func)) => func.clone(),
            Some(other) => panic!("math.{name} must be callable, got {other:?}"),
            None => panic!("missing callable math.{name}"),
        }
    }

    fn call_math(name: &str, args: Args) -> SourceResult<Value> {
        let call = math_func(name)
            .native_fn_addr()
            .unwrap_or_else(|| panic!("math.{name} must be a native callable"));
        call(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    fn body() -> Value {
        Value::Content(Content::MathIdent("x".into()))
    }

    fn diagnostic_message(name: &str, args: Args) -> String {
        let diagnostics = call_math(name, args).unwrap_err();
        assert_eq!(diagnostics.len(), 1, "math.{name} must emit one primary diagnostic");
        diagnostics.into_iter().next().unwrap().message
    }

    #[test]
    fn p1291_namespace_expoe_cinco_funcoes_com_nome_curto() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["bb", "frak", "inline", "scripts", "serif"] {
            let Some(Value::Func(func)) = module.scope().get(name) else {
                panic!("math.{name} must be callable")
            };
            assert_eq!(func.name(), Some(name), "math.{name} public name");
        }
    }

    #[test]
    fn p1291_styles_reutilizam_as_quatro_nativas_donas() {
        let expected: [(&str, NativeFn); 4] = [
            ("bb", crate::compiler::stdlib::math_style::native_bb),
            ("frak", crate::compiler::stdlib::math_style::native_frak),
            ("inline", crate::compiler::stdlib::math_style::native_inline),
            ("serif", crate::compiler::stdlib::math_style::native_serif),
        ];
        for (name, expected) in expected {
            let actual = math_func(name).native_fn_addr().unwrap();
            assert!(
                std::ptr::fn_addr_eq(actual, expected),
                "math.{name} points to the wrong global function"
            );
        }
    }

    #[test]
    fn p1291_styles_produzem_morfologia_e_defaults_medidos() {
        for (name, kind, cramped) in [
            ("bb", MathStyleKind::DoubleStruck, None),
            ("frak", MathStyleKind::Fraktur, None),
            ("inline", MathStyleKind::Inline, Some(false)),
            ("serif", MathStyleKind::Plain, None),
        ] {
            let value = call_math(name, Args::positional(vec![body()])).unwrap();
            let Value::Content(Content::MathStyled(elem)) = value else {
                panic!("math.{name} must produce MathStyled")
            };
            assert_eq!(elem.kind, Some(kind), "math.{name} style kind");
            assert_eq!(elem.cramped, cramped, "math.{name} cramped default");
            assert!(matches!(&elem.body, Content::MathIdent(x) if x == "x"));
        }

        let mut explicit = Args::positional(vec![body()]);
        explicit.named.insert("cramped".into(), Value::Bool(true));
        let Value::Content(Content::MathStyled(elem)) =
            call_math("inline", explicit).unwrap()
        else {
            panic!("math.inline(cramped: true) must produce MathStyled")
        };
        assert_eq!(elem.cramped, Some(true));
    }

    #[test]
    fn p1291_styles_rejeitam_aridade_tipo_e_named_invalidos() {
        for name in ["bb", "frak", "inline", "serif"] {
            assert!(call_math(name, Args::positional(vec![])).is_err());
            assert!(call_math(name, Args::positional(vec![Value::Int(1)])).is_err());
            assert!(call_math(name, Args::positional(vec![body(), body()])).is_err());
            let mut unknown = Args::positional(vec![body()]);
            unknown.named.insert("unknown".into(), Value::Bool(true));
            assert!(call_math(name, unknown).is_err());
        }
        let mut wrong_cramped = Args::positional(vec![body()]);
        wrong_cramped.named.insert("cramped".into(), Value::Int(1));
        assert!(call_math("inline", wrong_cramped).is_err());
    }

    #[test]
    fn p1291_scripts_constroi_override_e_valida_argumentos() {
        let value = call_math("scripts", Args::positional(vec![body()])).unwrap();
        let Value::Content(Content::MathLimitsOverride(elem)) = value else {
            panic!("math.scripts must produce MathLimitsOverride")
        };
        assert!(matches!(&elem.body, Content::MathIdent(x) if x == "x"));
        assert!(!elem.limits, "scripts must force lateral attachments");
        assert!(elem.inline, "canonical ignored inline transport must be true");

        assert!(call_math("scripts", Args::positional(vec![])).is_err());
        assert!(call_math("scripts", Args::positional(vec![Value::Int(1)])).is_err());
        assert!(call_math("scripts", Args::positional(vec![body(), body()])).is_err());
        let mut unknown = Args::positional(vec![body()]);
        unknown.named.insert("foo".into(), Value::Int(1));
        let err = call_math("scripts", unknown).unwrap_err();
        assert!(format!("{err:?}").contains("unexpected argument: foo"));
    }

    #[test]
    fn p1291_red_errors_excesso_e_unexpected_argument_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                (name, diagnostic_message(name, Args::positional(vec![body(), body()])))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "unexpected argument".to_string()),
                ("frak", "unexpected argument".to_string()),
                ("inline", "unexpected argument".to_string()),
                ("scripts", "unexpected argument".to_string()),
                ("serif", "unexpected argument".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_named_nope_e_unexpected_argument_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                let mut args = Args::positional(vec![body()]);
                args.named.insert("nope".into(), Value::Bool(true));
                (name, diagnostic_message(name, args))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "unexpected argument: nope".to_string()),
                ("frak", "unexpected argument: nope".to_string()),
                ("inline", "unexpected argument: nope".to_string()),
                ("scripts", "unexpected argument: nope".to_string()),
                ("serif", "unexpected argument: nope".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_body_integer_e_expected_content_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                (name, diagnostic_message(name, Args::positional(vec![Value::Int(1)])))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "expected content, found integer".to_string()),
                ("frak", "expected content, found integer".to_string()),
                ("inline", "expected content, found integer".to_string()),
                ("scripts", "expected content, found integer".to_string()),
                ("serif", "expected content, found integer".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_inline_cramped_integer_e_boolean_verbatim() {
        let mut args = Args::positional(vec![body()]);
        args.named.insert("cramped".into(), Value::Int(1));
        assert_eq!(diagnostic_message("inline", args), "expected boolean, found integer");
    }

    #[test]
    fn p1291_red_errors_styles_continuam_a_aceitar_string() {
        for name in ["bb", "frak", "inline", "serif"] {
            let value = call_math(name, Args::positional(vec![Value::Str("x".into())]))
                .unwrap_or_else(|diagnostics| {
                    panic!("math.{name} rejected string: {diagnostics:?}")
                });
            let Value::Content(Content::MathStyled(elem)) = value else {
                panic!("math.{name} string must produce MathStyled")
            };
            assert!(
                matches!(&elem.body, Content::MathText(text) if text == "x"),
                "math.{name} string must be converted to MathText"
            );
        }
    }

    #[test]
    fn p1291_scope_fechado_nao_copia_extras_globais() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["lorem", "read", "eval", "panic", "assert"] {
            assert!(
                module.scope().get(name).is_none(),
                "global extra math.{name} leaked"
            );
        }
    }

    #[test]
    fn p1293_b_namespace_expoe_quatro_funcoes_com_nomes_curtos() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["attach", "binom", "mono", "script"] {
            let Some(Value::Func(func)) = module.scope().get(name) else {
                panic!("missing callable math.{name}")
            };
            assert_eq!(func.name(), Some(name), "math.{name} public name");
        }
    }

    #[test]
    fn p1293_b_attach_preserva_sete_slots_e_none_explicito() {
        let values = ["x", "t", "b", "tl", "bl", "tr", "br"];
        let mut args =
            Args::positional(vec![Value::Content(Content::MathIdent(values[0].into()))]);
        for (name, text) in [
            ("t", values[1]),
            ("b", values[2]),
            ("tl", values[3]),
            ("bl", values[4]),
            ("tr", values[5]),
            ("br", values[6]),
        ] {
            args.named
                .insert(name.into(), Value::Content(Content::MathIdent(text.into())));
        }
        let Value::Content(Content::MathAttach(elem)) =
            call_math("attach", args).unwrap()
        else {
            panic!("math.attach must produce MathAttach")
        };
        assert!(matches!(&elem.base, Content::MathIdent(x) if x == "x"));
        for (actual, expected) in [
            (&elem.t, "t"),
            (&elem.b, "b"),
            (&elem.tl, "tl"),
            (&elem.bl, "bl"),
            (&elem.tr, "tr"),
            (&elem.br, "br"),
        ] {
            assert!(
                matches!(actual, MathAttachSlot::Present(Content::MathIdent(x)) if x == expected)
            );
        }

        let mut explicit_none = Args::positional(vec![body()]);
        explicit_none.named.insert("t".into(), Value::None);
        let Value::Content(Content::MathAttach(elem)) =
            call_math("attach", explicit_none).unwrap()
        else {
            panic!("math.attach(t: none) must produce MathAttach")
        };
        assert!(matches!(elem.t, MathAttachSlot::ExplicitNone));
        assert!(matches!(elem.b, MathAttachSlot::Omitted));
    }

    #[test]
    fn p1293_b_attach_none_explicito_tem_layout_equivalente_ao_omitido() {
        use crate::compiler::layout::FixedMetrics;
        use crate::compiler::math::layout::MathLayouter;
        use crate::entities::layout_types::{Pt, TextStyle};

        let style = TextStyle::regular(Pt(12.0));
        let layouter = MathLayouter::new(&FixedMetrics, true, &style);
        let omitted = math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        let explicit_none = math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::ExplicitNone,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );

        let (omitted_items, omitted_extent) =
            layouter.layout_equation_measured(&omitted, &style);
        let (explicit_items, explicit_extent) =
            layouter.layout_equation_measured(&explicit_none, &style);
        assert_eq!(explicit_extent, omitted_extent);
        assert_eq!(explicit_items.len(), omitted_items.len());
    }

    #[test]
    fn p1293_b_binom_preserva_lower_variadico_e_sem_barra() {
        let args = Args::positional(vec![
            Value::Content(Content::MathIdent("n".into())),
            Value::Content(Content::MathIdent("k".into())),
            Value::Content(Content::MathIdent("j".into())),
            Value::Content(Content::MathIdent("m".into())),
        ]);
        let Value::Content(Content::MathDelimited(delimited)) =
            call_math("binom", args).unwrap()
        else {
            panic!("math.binom must produce stretched parentheses")
        };
        assert_eq!((delimited.open, delimited.close), ('(', ')'));
        let Content::MathFrac(frac) = &delimited.body else {
            panic!("math.binom must use the canonical fraction payload")
        };
        assert!(!frac.line, "binom must not draw a fraction bar");
        assert!(matches!(&frac.num, Content::MathIdent(x) if x == "n"));
        let Content::MathSequence(lower) = &frac.den else {
            panic!("variadic lower must preserve comma morphology")
        };
        assert_eq!(lower.len(), 5);
        assert!(matches!(&lower[0], Content::MathIdent(x) if x == "k"));
        assert!(matches!(&lower[1], Content::MathText(x) if x == ", "));
        assert!(matches!(&lower[2], Content::MathIdent(x) if x == "j"));
        assert!(matches!(&lower[3], Content::MathText(x) if x == ", "));
        assert!(matches!(&lower[4], Content::MathIdent(x) if x == "m"));
    }

    #[test]
    fn p1293_b_mono_script_reusam_nativas_e_defaults() {
        let expected: [(&str, NativeFn); 2] = [
            ("mono", crate::compiler::stdlib::math_style::native_mono),
            ("script", crate::compiler::stdlib::math_style::native_script),
        ];
        for (name, expected) in expected {
            let actual = math_func(name).native_fn_addr().unwrap();
            assert!(
                std::ptr::fn_addr_eq(actual, expected),
                "math.{name} must reuse its global native"
            );
        }

        let Value::Content(Content::MathStyled(mono)) =
            call_math("mono", Args::positional(vec![body()])).unwrap()
        else {
            panic!("math.mono must produce MathStyled")
        };
        assert_eq!(mono.kind, Some(MathStyleKind::Monospace));

        for (explicit, expected) in
            [(None, true), (Some(true), true), (Some(false), false)]
        {
            let mut args = Args::positional(vec![body()]);
            if let Some(value) = explicit {
                args.named.insert("cramped".into(), Value::Bool(value));
            }
            let Value::Content(Content::MathStyled(script)) =
                call_math("script", args).unwrap()
            else {
                panic!("math.script must produce MathStyled")
            };
            assert_eq!(script.kind, Some(MathStyleKind::Script));
            assert_eq!(script.cramped, Some(expected));
        }
    }

    #[test]
    fn p1293_b_oito_vetores_alcancam_o_layout_vigente() {
        use crate::compiler::layout::FixedMetrics;
        use crate::compiler::math::layout::MathLayouter;
        use crate::entities::layout_types::{MathSize, Pt, TextStyle};

        let inline_style = TextStyle::regular(Pt(12.0));
        let display_style = TextStyle {
            math_size: MathSize::Display,
            ..inline_style.clone()
        };
        let display = MathLayouter::new(&FixedMetrics, true, &display_style);
        let inline = MathLayouter::new(&FixedMetrics, false, &inline_style);
        let attach = math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::Present(Content::MathIdent("t".into())),
            MathAttachSlot::Present(Content::MathIdent("b".into())),
            MathAttachSlot::Present(Content::MathIdent("l".into())),
            MathAttachSlot::Present(Content::MathIdent("l".into())),
            MathAttachSlot::Present(Content::MathIdent("r".into())),
            MathAttachSlot::Present(Content::MathIdent("r".into())),
        );
        let binom = math_binom_content(
            Content::MathIdent("n".into()),
            vec![
                Content::MathIdent("k".into()),
                Content::MathIdent("j".into()),
                Content::MathIdent("m".into()),
            ],
        );
        let mono = Content::math_styled(
            Some(MathStyleKind::Monospace),
            None,
            None,
            Content::MathIdent("mono".into()),
            None,
        );
        let mono_in_script = Content::math_styled(
            Some(MathStyleKind::Script),
            None,
            None,
            mono.clone(),
            Some(true),
        );
        let scripted_body = Content::math_attach_scripts(
            Content::MathIdent("x".into()),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Present(Content::MathText("1".into())),
        );
        let scripted = |cramped| {
            Content::math_styled(
                Some(MathStyleKind::Script),
                None,
                None,
                scripted_body.clone(),
                Some(cramped),
            )
        };
        let dimensions = |layouter: &MathLayouter<'_, FixedMetrics>,
                          content: &Content,
                          style: &TextStyle| {
            let (_, extent) = layouter.layout_equation_measured(content, style);
            (extent.width, extent.ascent + extent.descent)
        };

        let vectors = [
            dimensions(&display, &attach, &display_style),
            dimensions(&inline, &attach, &inline_style),
            dimensions(&display, &binom, &display_style),
            dimensions(&inline, &binom, &inline_style),
            dimensions(&display, &mono, &display_style),
            dimensions(&display, &mono_in_script, &display_style),
            dimensions(&display, &scripted(true), &display_style),
            dimensions(&display, &scripted(false), &display_style),
        ];
        assert!(vectors.iter().all(|(width, height)| {
            width.is_finite() && *width > 0.0 && height.is_finite() && *height > 0.0
        }));
        assert!(vectors[2].1 > vectors[3].1, "binom display versus inline: {vectors:?}");
        assert!(vectors[5].0 < vectors[4].0, "mono dentro de script deve reduzir");
        assert_ne!(vectors[6].1, vectors[7].1, "cramped deve permanecer observável");
    }

    #[test]
    fn p1293_b_assinaturas_sao_fechadas() {
        let span = Span::from_range(test_file_id(), 17..18);
        let assert_diag = |name: &str, mut args: Args, expected: &str| {
            args.span = span;
            let diagnostics = call_math(name, args).unwrap_err();
            assert_eq!(diagnostics.len(), 1, "math.{name}");
            assert_eq!(diagnostics[0].message, expected, "math.{name}");
            assert_eq!(diagnostics[0].span, span, "math.{name} selected span");
        };

        assert_diag("attach", Args::positional(vec![]), "missing argument: base");
        assert_diag(
            "attach",
            Args::positional(vec![Value::Int(1)]),
            "expected content, found integer",
        );
        assert_diag(
            "attach",
            Args::positional(vec![body(), body()]),
            "unexpected argument",
        );
        let mut attach_unknown = Args::positional(vec![body()]);
        attach_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("attach", attach_unknown, "unexpected argument: unknown");
        let mut attach_bad_slot = Args::positional(vec![body()]);
        attach_bad_slot.named.insert("t".into(), Value::Int(1));
        assert_diag("attach", attach_bad_slot, "expected content or none, found integer");

        assert_diag("binom", Args::positional(vec![]), "missing argument: upper");
        assert_diag("binom", Args::positional(vec![body()]), "missing argument: lower");
        assert_diag(
            "binom",
            Args::positional(vec![Value::Int(1), body()]),
            "expected content, found integer",
        );
        let mut binom_unknown = Args::positional(vec![body(), body()]);
        binom_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("binom", binom_unknown, "unexpected argument: unknown");
        let mut upper_named = Args::positional(vec![]);
        upper_named.named.insert("upper".into(), body());
        assert_diag("binom", upper_named, "the argument `upper` is positional");

        for name in ["mono", "script"] {
            assert_diag(name, Args::positional(vec![]), "missing argument: body");
            assert_diag(
                name,
                Args::positional(vec![Value::Int(1)]),
                "expected content, found integer",
            );
            assert_diag(
                name,
                Args::positional(vec![body(), body()]),
                "unexpected argument",
            );
            let mut body_named = Args::positional(vec![]);
            body_named.named.insert("body".into(), body());
            assert_diag(name, body_named, "the argument `body` is positional");
        }

        let mut wrong_cramped = Args::positional(vec![body()]);
        wrong_cramped.named.insert("cramped".into(), Value::Int(1));
        assert_diag("script", wrong_cramped, "expected boolean, found integer");
        let mut script_unknown = Args::positional(vec![body()]);
        script_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("script", script_unknown, "unexpected argument: unknown");
    }

    #[test]
    fn p1293_b_independent_red_named_positional_preserva_hints() {
        let span = Span::from_range(test_file_id(), 17..18);
        let assert_named_positional = |name: &str, mut args: Args, parameter: &str| {
            args.span = span;
            let diagnostics = call_math(name, args).unwrap_err();
            assert_eq!(diagnostics.len(), 1, "math.{name}");
            assert_eq!(
                diagnostics[0].message,
                format!("the argument `{parameter}` is positional"),
                "math.{name}"
            );
            assert_eq!(
                diagnostics[0].hints,
                vec![format!("try removing `{parameter}:`")],
                "math.{name}"
            );
            assert_eq!(diagnostics[0].span, span, "math.{name} selected span");
        };

        let mut attach = Args::positional(vec![]);
        attach.named.insert("base".into(), body());
        assert_named_positional("attach", attach, "base");

        let mut binom = Args::positional(vec![]);
        binom.named.insert("upper".into(), body());
        assert_named_positional("binom", binom, "upper");

        for name in ["mono", "script"] {
            let mut args = Args::positional(vec![]);
            args.named.insert("body".into(), body());
            assert_named_positional(name, args, "body");
        }
    }
}

#[cfg(test)]
mod p1283_tests {
    use super::*;

    fn assert_mirror(
        sym: &crate::entities::module::Module,
        math: &crate::entities::module::Module,
    ) {
        for (name, binding) in sym.scope().iter() {
            let mirrored = math
                .scope()
                .get(name)
                .unwrap_or_else(|| panic!("missing math.{name}"));
            match (binding.value(), mirrored) {
                (Value::Symbol(left), Value::Symbol(right)) => {
                    assert_eq!(left.value, right.value, "wrong math mirror base: {name}");
                    assert_eq!(
                        left.variants, right.variants,
                        "wrong math mirror variants: {name}"
                    );
                }
                (Value::Module(left), Value::Module(right)) => assert_mirror(left, right),
                // Bindings próprios de math ganham de símbolos homónimos.
                (_, Value::Func(_))
                    if matches!(name, "sqrt" | "root" | "class" | "equation" | "op") => {}
                (left, right) => {
                    panic!("wrong math mirror kind for {name}: {left:?} vs {right:?}")
                }
            }
        }
    }

    #[test]
    fn p1283_math_espelha_sym_sem_sobrescrever_funcoes() {
        let Value::Module(sym) = crate::compiler::stdlib::sym::build_sym_module() else {
            panic!("sym must be module")
        };
        let Value::Module(math) = make_math_module() else {
            panic!("math must be module")
        };
        assert_mirror(&sym, &math);
        for name in ["sqrt", "root", "class", "equation", "op"] {
            assert!(
                matches!(math.scope().get(name), Some(Value::Func(_))),
                "math.{name} overwritten"
            );
        }
        assert!(matches!(math.scope().get("gender"), Some(Value::Module(_))));
        assert!(matches!(math.scope().get("control"), Some(Value::Module(_))));
    }
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

// ── Passo 397: `document(...)` e `asset(...)` ────────────────────────────────
