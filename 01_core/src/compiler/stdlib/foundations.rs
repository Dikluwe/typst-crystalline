//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations.md
//! @prompt-hash 418e9a4e
//! @layer L1
//! @updated 2026-06-24
//!
//! Funções nativas fundamentais (type, len, rgb, luma, range, str, int, float).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::file_id::FileId;
use ecow::EcoString;

use super::{err, expect_no_named};

use crate::compiler::eval::repr::repr_value;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `type(v)` → valor-tipo do argumento (`Value::Type`). P685: paridade vanilla —
/// `type(1) == int` funciona por comparação directa de valores de tipo.
pub fn native_type(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Type(v.type_of())),
        _ => err(format!("type() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `repr(v)` → representação textual reconhecível do valor (P421).
pub fn native_repr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(repr_value(v).into())),
        _ => err(format!("repr() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)] => Ok(Value::Int(d.len() as i64)),
        [other] => err(format!("len() não suporta {}", other.type_name())),
        _ => err(format!("len() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `rgb(r, g, b)` ou `rgb(r, g, b, a)` → Color.
///
/// **P740D** — paridade vanilla `Component` (`visualize/color.rs:2678-2692`):
/// cada componente aceita Int [0, 255] ou Ratio [0%, 100%] (mesmo padrão
/// aplicado a `linear-rgb` em P736). Ratio → u8 via `(fracção × 255).round()`
/// — medido: `rgb(50%, 0%, 0%)` → `rgb("#800000")` (127.5 → 128 = 0x80).
/// Erros verbatim do cast `Component`: Int fora de gama → "number must be
/// between 0 and 255"; Ratio fora de gama → "ratio must be between 0% and
/// 100%"; Float (e outros tipos) → "expected integer or ratio, found {type}".
pub fn native_rgb(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_u8(v: &Value) -> SourceResult<u8> {
        match v {
            Value::Int(i) if (0..=255).contains(i) => Ok(*i as u8),
            Value::Int(_) => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "number must be between 0 and 255".to_string(),
            )]),
            Value::Relative(r) if r.abs.is_zero() && (0.0..=1.0).contains(&r.rel) => {
                Ok((r.rel * 255.0).round() as u8)
            }
            Value::Relative(r) if r.abs.is_zero() => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ratio must be between 0% and 100%".to_string(),
            )]),
            // P842 (#32) — o literal percentual é agora `Value::Ratio`
            // (antes `Relative` com abs zero). Mesma validação do vanilla.
            Value::Ratio(r) if (0.0..=1.0).contains(&r.get()) => {
                Ok((r.get() * 255.0).round() as u8)
            }
            Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ratio must be between 0% and 100%".to_string(),
            )]),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer or ratio, found {}", other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [Value::Str(s)] => parse_hex_color(s.as_str()),
        [r, g, b] => Ok(Value::Color(Color::rgb(as_u8(r)?, as_u8(g)?, as_u8(b)?))),
        [r, g, b, a] => {
            Ok(Value::Color(Color::rgba(as_u8(r)?, as_u8(g)?, as_u8(b)?, as_u8(a)?)))
        }
        _ => err(format!(
            "rgb() requer 3 ou 4 componentes (Int/Ratio), recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P703** — `rgb(hex)`: cor a partir de notação hexadecimal (3/4/6/8
/// dígitos, `#` opcional, maiúsculas/minúsculas indiferentes). Paridade
/// vanilla verbatim — algoritmo e mensagens de `visualize/color.rs:2072-2107`
/// do vanilla (`impl FromStr for Rgb`).
fn parse_hex_color(s: &str) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.chars().any(|c| !c.is_ascii_hexdigit()) {
        return err("color string contains non-hexadecimal letters");
    }
    let len = hex.len();
    let long = len == 6 || len == 8;
    let short = len == 3 || len == 4;
    let has_alpha = len == 4 || len == 8;
    if !long && !short {
        return err("color string has wrong length");
    }
    let count = if has_alpha { 4 } else { 3 };
    let item_len = if long { 2 } else { 1 };
    let mut values = [255u8; 4];
    for (i, value) in values.iter_mut().enumerate().take(count) {
        let pos = i * item_len;
        let item = &hex[pos..pos + item_len];
        let mut v = u8::from_str_radix(item, 16).unwrap();
        if short {
            v += v * 16;
        }
        *value = v;
    }
    Ok(Value::Color(Color::rgba(values[0], values[1], values[2], values[3])))
}

/// `luma(l)` → Color::Luma (paridade vanilla D65Gray pós-P257).
///
/// **P257 (ADR-0083 PROPOSTO)** — refactor: constrói `Color::Luma`
/// dedicado (em vez de `Color::Srgb` cinza). Aceita Int [0, 255]
/// como paridade construtor anterior; converte para f32 [0.0, 1.0]
/// internamente. PDF output bit-equivalente via `to_srgb()` que
/// expande Luma para sRGB cinza.
///
/// **P705** — aceita também `Ratio` [0%, 100%] (paridade vanilla
/// `Component`, `visualize/color.rs:2677-2692`). Fallback silencioso
/// verbatim: qualquer valor que não caste (tipo errado, fora de gama) ou a
/// ausência do argumento devolve **branco**, não erro — replica
/// `args.expect(...).unwrap_or(Component(Ratio::one()))` do vanilla,
/// medido directamente (`luma("bad")`, `luma(300)`, `luma(150%)`,
/// `luma()` → todos `luma(100%)` no vanilla). ADR-0107: comportamento
/// observável da língua, não mecânica — paridade exige replicar, não
/// substituir por erro. 2+ argumentos continua a ser erro estrutural
/// (`alpha` não suportado — `Color::Luma` do cristalino não tem esse campo).
pub fn native_luma(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn component_to_ratio(v: &Value) -> Option<f32> {
        match v {
            Value::Int(i) if (0..=255).contains(i) => Some(*i as f32 / 255.0),
            Value::Ratio(r) if (0.0..=1.0).contains(&r.get()) => Some(r.get() as f32),
            // P705 — percentagens simples (`50%`, `v * 1%`) avaliavam para
            // Value::Relative neste cristalino (unificado com `length`);
            // desde P842 (#32) avaliam para `Value::Ratio` (braço acima) —
            // este braço fica para `Relative` construídos por outras vias.
            // Só conta como componente de cor se não tiver parte absoluta
            // (`50% + 1pt` não é um componente válido, cai no fallback).
            Value::Relative(rel)
                if rel.abs == crate::entities::layout_types::Length::ZERO
                    && (0.0..=1.0).contains(&rel.rel) =>
            {
                Some(rel.rel as f32)
            }
            _ => None,
        }
    }
    match args.items.as_slice() {
        [] => Ok(Value::Color(Color::luma(1.0))),
        [v] => Ok(Value::Color(Color::luma(component_to_ratio(v).unwrap_or(1.0)))),
        _ => err(format!(
            "luma() requer 0 ou 1 argumento, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklab(l, a, b[, alpha])` →
/// `Color::Oklab`. Componentes f32 (Float ou Int).
pub fn native_oklab(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i) => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "oklab({}): espera Float/Int, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        }
    }
    match args.items.as_slice() {
        [l, a, b] => Ok(Value::Color(Color::oklab(
            as_f32(l, "l")?,
            as_f32(a, "a")?,
            as_f32(b, "b")?,
            1.0,
        ))),
        [l, a, b, alpha] => Ok(Value::Color(Color::oklab(
            as_f32(l, "l")?,
            as_f32(a, "a")?,
            as_f32(b, "b")?,
            as_f32(alpha, "alpha")?,
        ))),
        _ => err(format!(
            "oklab() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklch(l, c, h[, alpha])` →
/// `Color::Oklch`. `h` em graus.
pub fn native_oklch(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i) => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "oklch({}): espera Float/Int, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        }
    }
    match args.items.as_slice() {
        [l, c, h] => Ok(Value::Color(Color::oklch(
            as_f32(l, "l")?,
            as_f32(c, "c")?,
            as_f32(h, "h")?,
            1.0,
        ))),
        [l, c, h, alpha] => Ok(Value::Color(Color::oklch(
            as_f32(l, "l")?,
            as_f32(c, "c")?,
            as_f32(h, "h")?,
            as_f32(alpha, "alpha")?,
        ))),
        _ => err(format!(
            "oklch() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `linear_rgb(r, g, b[, alpha])`
/// → `Color::LinearRgb`. Componentes f32 [0.0, 1.0].
pub fn native_linear_rgb(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    // **P736** — paridade vanilla medida: `linear-rgb` aceita Int [0, 255]
    // (÷255) ou Ratio (percentagem); **rejeita Float** com a mensagem
    // verbatim "expected integer or ratio, found float". Pré-P736 havia
    // dupla divergência (falso-aceite de Float, rejeição de Ratio).
    fn as_f32(v: &Value) -> SourceResult<f32> {
        match v {
            Value::Int(i) => Ok(*i as f32 / 255.0),
            Value::Relative(r) if r.abs.is_zero() => Ok(r.rel as f32),
            // P842 (#32) — o literal percentual é agora `Value::Ratio`.
            Value::Ratio(r) => Ok(r.get() as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer or ratio, found {}", other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [r, g, b] => {
            Ok(Value::Color(Color::linear_rgb(as_f32(r)?, as_f32(g)?, as_f32(b)?, 1.0)))
        }
        [r, g, b, a] => Ok(Value::Color(Color::linear_rgb(
            as_f32(r)?,
            as_f32(g)?,
            as_f32(b)?,
            as_f32(a)?,
        ))),
        _ => err(format!(
            "linear_rgb() requer 3 ou 4 argumentos (Int/Ratio), recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `cmyk(c, m, y, k)` →
/// `Color::Cmyk`. Componentes f32 [0.0, 1.0].
/// PDF native `/DeviceCMYK` scope-out P257 (converte para sRGB
/// via `Color::to_srgb()` no exporter; ADR-0083 §"Scope-out
/// PDF native CMYK").
pub fn native_cmyk(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i) => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cmyk({}): espera Float/Int, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        }
    }
    match args.items.as_slice() {
        [c, m, y, k] => Ok(Value::Color(Color::cmyk(
            as_f32(c, "c")?,
            as_f32(m, "m")?,
            as_f32(y, "y")?,
            as_f32(k, "k")?,
        ))),
        _ => err(format!("cmyk() requer 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsl(h, s, l[, alpha])` →
/// `Color::Hsl`. `h` em graus.
pub fn native_hsl(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i) => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("hsl({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [h, s, l] => Ok(Value::Color(Color::hsl(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(l, "l")?,
            1.0,
        ))),
        [h, s, l, a] => Ok(Value::Color(Color::hsl(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(l, "l")?,
            as_f32(a, "a")?,
        ))),
        _ => err(format!(
            "hsl() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsv(h, s, v[, alpha])` →
/// `Color::Hsv`. `h` em graus.
pub fn native_hsv(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i) => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("hsv({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [h, s, v] => Ok(Value::Color(Color::hsv(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(v, "v")?,
            1.0,
        ))),
        [h, s, v, a] => Ok(Value::Color(Color::hsv(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(v, "v")?,
            as_f32(a, "a")?,
        ))),
        _ => err(format!(
            "hsv() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// `range(n)` → Array de 0..n; `range(start, end)` → Array de start..end.
/// P504: `inclusive: true` inclui o `end`. P704: `step: n` (não-zero,
/// default 1, aceita negativo) — algoritmo verbatim do vanilla
/// (`foundations/array.rs:384-430`, `Array::range`).
pub fn native_range(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let inclusive = match args.named.get("inclusive") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "range() argumento 'inclusive' requer bool, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => false,
    };
    let step: i64 = match args.named.get("step") {
        Some(Value::Int(0)) => return err("number must not be zero"),
        Some(Value::Int(s)) => *s,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "range() argumento 'step' requer int, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => 1,
    };
    if let Some(bad) = args
        .named
        .keys()
        .find(|k| k.as_str() != "inclusive" && k.as_str() != "step")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("range() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }

    fn stepped_range(start: i64, end: i64, step: i64, inclusive: bool) -> Vec<Value> {
        let step_dir = 0i64.cmp(&step);
        let in_bounds = |x: i64| {
            if inclusive {
                x.cmp(&end) != step_dir.reverse()
            } else {
                x.cmp(&end) == step_dir
            }
        };
        let mut out = Vec::new();
        let mut x = start;
        while in_bounds(x) {
            out.push(Value::Int(x));
            x += step;
        }
        out
    }

    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Array(stepped_range(0, *n, step, inclusive))),
        [Value::Int(start), Value::Int(end)] => {
            Ok(Value::Array(stepped_range(*start, *end, step, inclusive)))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.items.len())),
    }
}

// ── Funções de conversão de tipo (Passo 27) ─────────────────────────────────

/// `str(v)` → representação textual do valor.
/// P491: `str(int, base: n)` converte inteiro para base 2–36.
/// P501: `str.from-unicode(codepoint)` constrói carácter a partir de um scalar Unicode.
pub fn native_str(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `base` (aplicável apenas a Int). Apenas `base` é aceite.
    let base_arg = args.named.get("base");
    if args.named.len() > 1 || (args.named.len() == 1 && base_arg.is_none()) {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("str() argumento nomeado desconhecido: '{bad}'"));
    }

    match args.items.as_slice() {
        [v] => {
            // P491: str(Int, base: n)
            if let Some(base_val) = base_arg {
                if let Value::Int(i) = v {
                    let base = match base_val {
                        Value::Int(b) => *b as u32,
                        other => {
                            return err(format!(
                                "str() argumento 'base' requer Int, recebeu {}",
                                other.type_name()
                            ))
                        }
                    };
                    if !(2..=36).contains(&base) {
                        return err(format!(
                            "str() base deve estar entre 2 e 36, recebeu {}",
                            base
                        ));
                    }
                    return Ok(Value::Str(EcoString::from(format_radix(*i, base))));
                }
                return err(format!(
                    "str() argumento 'base' só se aplica a Int, recebeu {}",
                    v.type_name()
                ));
            }

            let s: String = match v {
                Value::None => "none".into(),
                Value::Bool(b) => if *b { "true" } else { "false" }.into(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => format_float(*f),
                Value::Str(s) => return Ok(Value::Str(s.clone())),
                Value::Auto => "auto".into(),
                Value::Length(l) => format_length(l),
                Value::Ratio(r) => format!("{}%", r.to_percent()),
                Value::Angle(a) => format!("{}deg", a.to_deg()),
                Value::Bytes(b) => match std::str::from_utf8(b.as_slice()) {
                    Ok(s) => s.to_string(),
                    Err(_) => return err("bytes are not valid UTF-8"),
                },
                Value::Color(_) => return err("str() não suporta color"),
                other => return err(format!("str() não suporta {}", other.type_name())),
            };
            Ok(Value::Str(EcoString::from(s)))
        }
        _ => err(format!("str() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `str.from-unicode(codepoint)` → carácter correspondente ao scalar Unicode.
pub fn native_str_from_unicode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => match char::try_from(*i as u32) {
            Ok(c) if c != '\0' => Ok(Value::Str(EcoString::from(c.to_string()))),
            _ => err("str.from-unicode() requer um codepoint Unicode válido"),
        },
        [other] => {
            err(format!("str.from-unicode() requer Int, recebeu {}", other.type_name()))
        }
        _ => err(format!(
            "str.from-unicode() requer 1 argumento, recebeu {}",
            args.items.len()
        )),
    }
}

/// Formata f64 de forma compacta — sem trailing zeros desnecessários.
fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') {
        s
    } else {
        format!("{s}.0")
    }
}

/// Converte um inteiro para a representação textual numa base 2–36.
/// P491: paridade vanilla `str(255, base: 16) == "ff"`.
fn format_radix(mut n: i64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let negative = n < 0;
    let mut n = n.abs();
    let mut result = String::new();
    while n > 0 {
        let digit = (n % base as i64) as u32;
        let c = std::char::from_digit(digit, base).unwrap();
        result.push(c);
        n /= base as i64;
    }
    if negative {
        result.push('-');
    }
    result.chars().rev().collect()
}

/// Formata Length como string (ex: "12pt", "1.5em", "6pt + 1em").
fn format_length(l: &Length) -> String {
    let abs = l.abs.to_pt();
    let em = l.em;
    match (abs == 0.0, em == 0.0) {
        (true, true) => "0pt".into(),
        (false, true) => format!("{abs}pt"),
        (true, false) => format!("{em}em"),
        (false, false) => format!("{abs}pt + {em}em"),
    }
}

/// `int(v)` → inteiro. Aceita Int, Str (decimal), Bool.
/// Float → Err (semântica vanilla: Float não é `ToInt`).
/// P504: `int(str, base: n)` parseia string na base indicada (2–36).
pub fn native_int(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P504 — arg nomeado `base` (apenas este é aceite).
    let base = match args.named.get("base") {
        Some(Value::Int(b)) => {
            let base = *b as u32;
            if !(2..=36).contains(&base) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("int() base deve estar entre 2 e 36, recebeu {}", b),
                )]);
            }
            Some(base)
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "int() argumento 'base' requer Int, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => None,
    };
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("base"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("int() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }

    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Bool(b)] => Ok(Value::Int(if *b { 1 } else { 0 })),
        [Value::Str(s)] => {
            if let Some(base) = base {
                i64::from_str_radix(s.as_str(), base).map(Value::Int).map_err(|_| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "int() não consegue parsear {:?} na base {}",
                            s.as_str(),
                            base
                        ),
                    )]
                })
            } else {
                s.parse::<i64>().map(Value::Int).map_err(|_| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("int() não consegue parsear {:?}", s.as_str()),
                    )]
                })
            }
        }
        [Value::Float(f)] => err(format!(
            "int() não converte float {f} — usar int(calc.round(x)) ou int(calc.floor(x))"
        )),
        [other] => err(format!("int() não suporta {}", other.type_name())),
        _ => err(format!("int() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `float(v)` → float. Aceita Float, Int (coerção), Str.
pub fn native_float(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Float(f)] => Ok(Value::Float(*f)),
        [Value::Int(i)] => Ok(Value::Float(*i as f64)),
        [Value::Str(s)] => s.parse::<f64>().map(Value::Float).map_err(|_| {
            vec![SourceDiagnostic::error(
                args.span,
                format!("float() não consegue parsear {:?}", s.as_str()),
            )]
        }),
        [other] => err(format!("float() não suporta {}", other.type_name())),
        _ => err(format!("float() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `metadata(value)` — embeber valor opaco no documento para query
/// via `Introspector::query_metadata`. P169 (M9 sub-passo 1).
///
/// Vanilla: `metadata(value)` em `introspection/metadata.rs`. Cristalino
/// minimal: 1 argumento posicional; sem named args; produz
/// `Content::metadata(Box<Value>)` que é zero-size em layout.
pub fn native_metadata(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Content(crate::entities::content::Content::metadata(v.clone()))),
        _ => err(format!("metadata() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `state(key, init)` — define runtime mutable state. P171 (M9 sub-passo 3).
///
/// Vanilla: `state(key, init)` em `introspection/state.rs`. Cristalino
/// minimal: 2 argumentos posicionais (key: Str, init: Value); produz
/// `Content::State { key, init: Box<Value> }`. Invisível em layout.
pub fn native_state(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), init] => Ok(Value::Content(
            crate::entities::content::Content::state(key.to_string(), init.clone()),
        )),
        [other, _] => err(format!(
            "state() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state() requer 2 argumentos (key, init), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_update(key, value)` — actualiza runtime state. P171 (M9 sub-passo 3).
///
/// Forma funcional cristalina (vanilla expõe como `state.update(key, fn)`
/// método; cristalino não suporta methods em values em P171). `value`
/// é o novo valor (Set variant); callbacks `Func` adiados.
pub fn native_state_update(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), value] => {
            Ok(Value::Content(crate::entities::content::Content::state_update(
                key.to_string(),
                crate::entities::state_update::StateUpdate::Set(Box::new(value.clone())),
            )))
        }
        [other, _] => err(format!(
            "state_update() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_update() requer 2 argumentos (key, value), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_update_with(key, fn)` — actualiza runtime state via callback.
/// P172 (M9 sub-passo 4).
///
/// **Stub em P172**: a callback é capturada na variant
/// `StateUpdate::Func(fn)` mas **NÃO é avaliada** em from_tags
/// (eval requer `Engine + EvalContext` que não estão disponíveis após
/// eval — pipeline restructuring deferido para passo dedicado).
/// Usar este stdlib func em P172 cria uma `StateUpdate::Func` que é
/// silenciosamente ignorada pelo `StateRegistry::apply_update`. O
/// state continua a refletir apenas as `Set` updates.
///
/// Para uso real, aguardar passo M7+ ou refactor de pipeline que
/// permita threading de Engine para from_tags.
pub fn native_state_update_with(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Func(func)] => {
            Ok(Value::Content(crate::entities::content::Content::state_update(
                key.to_string(),
                crate::entities::state_update::StateUpdate::Func(func.clone()),
            )))
        }
        [_, other] => err(format!(
            "state_update_with() requer função como segundo argumento, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_update_with() requer 2 argumentos (key, fn), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_display(key, [callback])` — render-mediated state display.
/// P240 (M9d/M7+1; per ADR-0081 PROPOSTO P239 Opção γ).
///
/// **Vanilla `state.display(callback)`**: durante walk, captura
/// `(key, callback)`; pós-fixpoint, `apply_state_displays`
/// (em `from_tags.rs`) chama `apply_func(callback, [state.value_at(loc)],
/// ctx, engine)` com Engine+ctx disponíveis e armazena Content
/// resultado em `intr.state_displays[(key, loc)]`. Layout arm
/// `Content::StateDisplay` consome via `state_display_value`.
/// Layouter permanece puro (sem Engine+ctx em signature).
///
/// **Forma 1-arg (`state_display(key)`)**: callback ausente; valor é
/// renderizado directamente — `Value::Content` passa-through;
/// `Value::Str` via `Content::text`; outros tipos fallback
/// `Content::Empty`.
///
/// **Forma 2-arg (`state_display(key, callback)`)**: callback aplicada
/// ao valor; resultado convertido para Content por mesma regra.
pub fn native_state_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // 1-arg: sem callback (renderiza Value→Content directo pós-fixpoint).
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::state_display(key.to_string(), None),
        )),
        // 2-arg: com callback.
        [Value::Str(key), Value::Func(callback)] => {
            Ok(Value::Content(crate::entities::content::Content::state_display(
                key.to_string(),
                Some(callback.clone()),
            )))
        }
        // 2-arg com segundo arg não-Func.
        [Value::Str(_), other] => err(format!(
            "state_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        // Primeiro arg não-string (1 OR 2 args).
        [other, ..] => err(format!(
            "state_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        // Arity errada (0 ou 3+).
        _ => err(format!(
            "state_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_display(key, [callback])` — render-mediated counter display.
/// P241 (M9d/M7+2; per ADR-0081 IMPLEMENTADO parcial M7+2 paralelo
/// absoluto M7+1 P240).
///
/// **Vanilla `counter.display(callback)`**: durante walk, captura
/// `(key, callback)`; pós-fixpoint, `apply_counter_displays` (em
/// `from_tags.rs`) converte `intr.counters.value_at(key, loc)` para
/// `Value::Array(Vec<Value::Int>)` representando counter state
/// (paridade vanilla `CounterState = SmallVec<[u64; 3]>`) e chama
/// `apply_func(callback, [array], ctx, engine)`. Resultado Content
/// armazenado em `intr.counter_displays[(key, loc)]`. Layout arm
/// `Content::CounterDisplayCallback` consome via
/// `counter_display_value`.
///
/// **Forma 1-arg (`counter_display(key)`)**: callback ausente;
/// formato default "1.2.3" via join "." (paridade
/// `formatted_counter_at` P177). Counter inexistente: `Content::Empty`.
///
/// **Forma 2-arg (`counter_display(key, callback)`)**: callback
/// aplicada ao Value::Array; resultado convertido para Content
/// (paridade `native_state_display` P240). Counter inexistente:
/// callback recebe `Value::Array(vec![])`.
///
/// **Distinto de `Content::CounterDisplay { kind }` legacy
/// single-pass** — variant nova paralela coexiste (Decisão 1 P241
/// Opção α: variant nova vs refino legacy).
pub fn native_counter_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // 1-arg: sem callback (formato default "1.2.3" pós-fixpoint).
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::counter_display_callback(key.to_string(), None),
        )),
        // 2-arg: com callback.
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(
            crate::entities::content::Content::counter_display_callback(key.to_string(), Some(callback.clone())),
        )),
        // 2-arg com segundo arg não-Func.
        [Value::Str(_), other] => err(format!(
            "counter_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        // Primeiro arg não-string.
        [other, ..] => err(format!(
            "counter_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        // Arity errada (0 ou 3+).
        _ => err(format!(
            "counter_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_at(key_str, label_str)` — valor do counter `key` na
/// `Location` associada à `label_str`. P177 (M9 sub-passo 7).
///
/// **Forma minimal P177**: retorna `Value::Str` formatado
/// hierárquicamente (e.g. `"1.2.3"`). Reusa
/// `Introspector::query_by_label` (P165) +
/// `Introspector::formatted_counter_at` (P177).
///
/// Casos de borda (todos retornam `Value::Str("")`):
/// - Label inexistente.
/// - Counter sem update prévia à Location do label.
/// - Counter inexistente.
pub fn native_counter_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            let formatted = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.formatted_counter_at(key.as_str(), loc))
                .unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [_, other] => err(format!(
            "counter_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_final(key_str)` — consulta o valor final do counter `key`
/// no `Introspector` da iteração de fixpoint anterior. P176 (M9
/// sub-passo 6).
///
/// **Forma minimal P176** (Opção β): retorna `Value::Str` com o
/// formato hierárquico (e.g. `"1.2.3"`) reusando
/// `Introspector::formatted_counter` (P170). Sem `Value::Counter`
/// rich type — refino futuro.
///
/// Iteração 0 (sem fixpoint, ou primeira iter de
/// `introspect_to_fixpoint`): `ctx.introspector` está vazio →
/// retorna `Value::Str("")`. Iterações seguintes vêem counter
/// populado pela iter anterior.
pub fn native_counter_final(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let formatted =
                ctx.introspector.formatted_counter(key.as_str()).unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [other] => err(format!(
            "counter_final() requer string como argumento, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_final(key)` — valor final do state `key` pós-walk.
///
/// **P236 (Fase 5 Layout candidata Categoria D 1/? — refino aditivo)**.
///
/// State runtime mutable foi materializado em **P171/M9 + P172**
/// (`Content::State`, `Content::StateUpdate`, `native_state`,
/// `native_state_update`, `native_state_update_with`,
/// `StateRegistry`, `state_update::StateUpdate`); pipeline activo
/// via `Introspector::state_final_value` (P171). P236 expõe `state_final`
/// user-facing — paralelo absoluto a `counter_final` (P176) +
/// `counter_at` (P177).
///
/// Subset minimal: argumento posicional Str `key` → `Value` (init se
/// state nunca actualizado; valor da última update caso contrário).
/// Iteração 0 de fixpoint (`introspector` vazio) → `Value::None`.
///
/// **Divergência factual** `P236.div-1`: spec P236 assumia ADR-0066
/// PROPOSTO + state runtime ausente; audit C1 confirmou ADR-0066
/// SUPERSEDED-BY 0073 + state runtime já materializado P171+M9+M9c.
/// Materialização P236 limitada a refino aditivo `state_final` per
/// directiva humana pós-divergência.
///
/// **P240 (M9d/M7+1) — two-pass real confirmado**: audit P239 C3.3
/// identificou sobreposição grande bloqueador A (walk-time Func
/// dispatch) + D (state.final two-pass). Audit P240 C7 cenário α
/// confirmou empíricamente: `state_final_value` baseline retorna
/// `state.final_value(key)` que delega a `history.last()` em
/// `StateRegistry`. Após fixpoint convergência, `apply_state_funcs`
/// (P191B) avaliou cumulativamente `StateUpdate::Func` updates;
/// `history.last()` reflete o valor final two-pass real. Portanto
/// **`state_final` semantic já é two-pass real pós-P240** —
/// paridade vanilla `state.final()` sem refactor adicional.
pub fn native_state_final(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let value = ctx
                .introspector
                .state_final_value(key.as_str())
                .cloned()
                .unwrap_or(Value::None);
            Ok(value)
        }
        [other] => err(format!(
            "state_final() requer string como argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_at(key, label)` — valor do state `key` na Location associada
/// ao `label`. P237 (Fase 5 Layout candidata Categoria D 1/? refino
/// estendido).
///
/// **Paralelo absoluto a `counter_at(key, label)` P177** — pattern
/// "stdlib func runtime para label-based lookup" N=1 inaugurado P237
/// (counter_at baseline P177 não conta no N novo por ser anterior à
/// série Categoria D refino).
///
/// Reusa `Introspector::query_by_label` (P139+P140) +
/// `Introspector::state_value` (P171) — wrapper trivial paralelo
/// pattern counter_at literal.
///
/// Retorna `Value::None` se: key inexistente, label inexistente, ou
/// state nunca actualizado antes da Location resolved (paridade
/// `counter_at` que retorna `Value::Str("")` em ambos os casos —
/// state `Value::None` distinto pois state pode ter qualquer Value
/// type, paridade `state_final` P236).
pub fn native_state_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            // Paralelo absoluto counter_at P177: query_by_label →
            // Option<Location>; state_value(key, loc) → Option<&Value>;
            // chain via and_then; default Value::None.
            let value = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.state_value(key.as_str(), loc).cloned())
                .unwrap_or(Value::None);
            Ok(value)
        }
        [_, other] => err(format!(
            "state_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}

/// `query(kind_str)` — consulta o `Introspector` da iteração de fixpoint
/// anterior por elementos do kind indicado.
///
/// **P175 (M9 sub-passo 5)**: forma original retornava `Value::Int(count)`.
/// **P179 upgrade**: retorna `Value::Array(Vec<Value::Location>)` com
/// as Locations dos elementos matched, em ordem de aparecimento.
/// Cliente que precise apenas de count usa `len(query("heading"))`.
///
/// `kind_str` válidos: `"heading"`, `"figure"`, `"citation"`,
/// `"metadata"`, `"state"`, `"state_update"`, `"outline"` (P178).
///
/// Iteração 0 (sem fixpoint, ou primeira iter de
/// `introspect_to_fixpoint`): `ctx.introspector` está vazio →
/// retorna `Value::Array(vec![])`. Iterações seguintes vêem
/// introspector populado pela iter anterior.
pub fn native_query(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "query")?;
    let locations = ctx.introspector.query(&selector);
    // **P844** (achado #47 de P831) — devolve o `Content` do elemento
    // encontrado (com campos acessíveis, ex.: `.value` em metadata) —
    // paridade vanilla `query() -> array<content>`. Fallback para
    // `Value::Location` quando o introspector não tem o elemento
    // registado (introspectors sintéticos sem walk, ex.: testes que
    // populam `kind_index` directamente — contrato P179 preservado
    // nesse caso).
    let values: Vec<Value> = locations
        .into_iter()
        .map(|loc| match ctx.introspector.element_at(loc) {
            Some(c) => Value::Content(c.clone()),
            None => Value::Location(loc),
        })
        .collect();
    Ok(Value::Array(values))
}

/// **P844** (achado #48 de P831) — Mapeia uma função nativa de
/// elemento para o `ElementKind` correspondente (selector por tipo de
/// elemento, paridade vanilla `locate(heading)`). `None` para funções
/// que não são de elemento (ou de elemento fora do subset com
/// `kind_index` populado — ver `parse_selector_arg`).
fn element_kind_of_native_func(
    f: &crate::entities::func::Func,
) -> Option<crate::entities::element_kind::ElementKind> {
    use crate::compiler::stdlib::{
        native_figure, native_heading, native_metadata, native_table,
    };
    use crate::entities::element_kind::ElementKind;
    use std::ptr::fn_addr_eq;

    let addr = f.native_fn_addr()?;
    if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
        Some(ElementKind::Heading)
    } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
        Some(ElementKind::Figure)
    } else if fn_addr_eq(addr, native_table as fn(_, _, _, _) -> _) {
        Some(ElementKind::Table)
    } else if fn_addr_eq(addr, native_metadata as fn(_, _, _, _) -> _) {
        Some(ElementKind::Metadata)
    } else {
        None
    }
}

/// **P209B (M9c)** — Parse selector arg para `native_query` +
/// `native_locate`. Dispatch por `Value` variant:
///
/// - `Value::Str("<name>")` (entre `<` e `>`) → `Selector::Label(Label(name))`.
/// - `Value::Str("kind")` → `Selector::Kind(ElementKind::from_name(kind))`.
/// - `Value::Location(loc)` → `Selector::Location(loc)`.
///
/// `func_name` é usado nas mensagens de erro para diferenciar
/// `query()` vs `locate()`.
fn parse_selector_arg(
    items: &[Value],
    func_name: &str,
) -> SourceResult<crate::entities::selector::Selector> {
    use crate::entities::element_kind::ElementKind;
    use crate::entities::label::Label;
    use crate::entities::selector::Selector;
    let msg = |s: String| -> SourceResult<Selector> {
        Err(vec![SourceDiagnostic::error(Span::detached(), s)])
    };
    match items {
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
            // P209B: <name> syntax → Selector::Label.
            let name = &s[1..s.len() - 1];
            Ok(Selector::Label(Label(name.to_string())))
        }
        [Value::Str(kind_str)] => match ElementKind::from_name(kind_str.as_str()) {
            Some(kind) => Ok(Selector::Kind(kind)),
            None => msg(format!(
                "{}(): kind '{}' não reconhecido (válidos: \
                     heading, figure, citation, metadata, state, \
                     state_update, outline, bibliography, equation, \
                     counter_update, table, list, enum, par, link, raw, \
                     quote, footnote). Para label, use `<nome>` syntax.",
                func_name, kind_str
            )),
        },
        [Value::Location(loc)] => {
            // P209B: Value::Location dispatch.
            Ok(Selector::Location(*loc))
        }
        [Value::Selector(sel)] => {
            // P417: Selector como valor de primeira classe.
            Ok(sel.clone())
        }
        [Value::Label(l)] => {
            // P509: <label> como valor de primeira classe.
            Ok(Selector::Label(l.clone()))
        }
        [Value::Func(f)] => {
            // **P844** (achado #48 de P831) — função de elemento como
            // seletor por tipo (paridade vanilla: `locate(heading)`,
            // `query(figure)`). Mensagem verbatim medida no vanilla
            // 0.15.0 para funções que não são de elemento:
            // "only element functions can be used as selectors".
            // Subset: apenas kinds com `kind_index` populado no
            // introspector L1 (heading/figure/table/metadata) — demais
            // funções de elemento caem no mesmo erro (limitação
            // documentada no relatório P844).
            match element_kind_of_native_func(f) {
                Some(kind) => Ok(Selector::Kind(kind)),
                None => msg(
                    "only element functions can be used as selectors".to_string(),
                ),
            }
        }
        [other] => msg(format!(
            "{}() requer string ou location, recebeu {}. \
             Tipos suportados: \"kind\", \"<label>\", \
             Value::Location. (Regex requer P209D; And/Or \
             ainda só Rust API.)",
            func_name,
            other.type_name()
        )),
        _ => msg(format!(
            "{}() requer 1 argumento (selector), recebeu {}",
            func_name,
            items.len()
        )),
    }
}

/// **P504** — `selector(target)` — converte um kind string, label
/// string ou função nativa de elemento num `Value::Selector`.
pub fn native_selector(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::compiler::stdlib::{native_figure, native_heading};
    use crate::entities::element_kind::ElementKind;
    use crate::entities::selector::Selector;
    use std::ptr::fn_addr_eq;

    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') => {
            let name = &s[1..s.len() - 1];
            Ok(Value::Selector(Selector::Label(crate::entities::label::Label(
                name.to_string(),
            ))))
        }
        [Value::Str(kind_str)] => match ElementKind::from_name(kind_str.as_str()) {
            Some(kind) => Ok(Value::Selector(Selector::Kind(kind))),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("selector(): kind '{}' não reconhecido", kind_str),
            )]),
        },
        [Value::Func(f)] => {
            let addr = f.native_fn_addr().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa esperada".to_string(),
                )]
            })?;
            if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Heading)))
            } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                Ok(Value::Selector(Selector::Kind(ElementKind::Figure)))
            } else {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "selector(): função nativa não suportada como selector".to_string(),
                )])
            }
        }
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector(): argumento inválido ({})", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("selector() requer 1 argumento, recebeu {}", args.items.len()),
        )]),
    }
}

/// **P208B (M9c)** — `here()` — retorna a Location "actual" disponível
/// no `EvalContext`.
///
/// Paridade vanilla: `here(context: Tracked<Context>) -> HintedStrResult<Location>`.
/// Cristalino diverge per P205A.div-1: lê `ctx.current_location`
/// directamente (sem `Tracked<Context>` envolvendo, pattern P174 +
/// `native_query` P175/P179). Retorna `Value::Location(loc)` quando
/// `current_location` está populated; erro contextual coerente caso
/// contrário.
///
/// **Mecanismo de população** (P208B infra minimal): `current_location`
/// é `None` por defeito. Caller que conhece a Location actual
/// (futuro show-rule para `Content::Context` block análogo a vanilla
/// `ContextElem`; tests sintéticos) seta o field antes de invocar
/// eval. Sub-mecanismo de captura automática no eval walk é deferred
/// per P208B C1 (zero consumers production confirmado em P208A A5 +
/// P208B C1.3).
///
/// **Sem args** (vanilla recebe Tracked<Context> só; cristalino sem
/// args explícitos).
pub fn native_here(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return err(format!(
            "here() não aceita argumentos, recebeu {}",
            args.items.len()
        ));
    }
    match ctx.current_location {
        Some(loc) => Ok(Value::Location(loc)),
        None => err("here() chamado fora de contexto locatable — \
             current_location não populado (P208B: infra minimal; \
             captura automática no walk é deferred)"
            .to_string()),
    }
}

/// **P772w** — `target()` — devolve o alvo de exportação actual.
///
/// Paridade vanilla: `target(context: Tracked<Context>) -> HintedStrResult<Target>`
/// (`foundations/target_.rs`), `#[func(contextual)]`, devolve `"paged"`,
/// `"html"` ou `"bundle"` consoante o pipeline de exportação activo.
/// Cristalino só produz PDF via layout paginado — não há pipeline HTML nem
/// Bundle (fora de escopo) — por isso devolve sempre `"paged"`, sem
/// depender de `Tracked<Context>`/`StyleChain` (não há outro valor possível
/// a determinar). Achado por P772w: antes desta correcção, `target()` não
/// estava definido no scope global — `#context target()` errava "unknown
/// variable: target" em vez de devolver `"paged"`, quebrando qualquer
/// template que use o padrão documentado pelo vanilla
/// (`if target() == "html" { .. } else { .. }`) para se adaptar ao alvo de
/// exportação.
///
/// **P821** — gate de contexto (achado #8 de P810): o vanilla acede a
/// `context.styles()?` (`foundations/target.rs:135-137`), que fora de
/// `#context` falha com `can only be used when context is known` + 2 hints
/// (`foundations/context.rs:55-61`). Replicado via `ctx.in_context` (mesma
/// convenção de `counter.get`/`measure`). Args: mensagens verbatim do
/// vanilla (`unexpected argument` / `unexpected argument: {nome}`), medidas
/// em P821 — antes: `target() não aceita argumentos, recebeu N` com
/// `<detached>`.
pub fn native_target(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Args primeiro (ordem do vanilla — `#target(1)` fora de context erra
    // `unexpected argument`, não o gate de contexto; medido em P821).
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]);
    }
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "can only be used when context is known",
        )
        .with_hint("try wrapping this in a `context` expression")
        .with_hint(
            "the `context` expression should wrap everything that depends on this function",
        )]);
    }
    Ok(Value::Str("paged".into()))
}

/// **P210B (M9c)** — `counter_step(key)` — emite
/// `Content::CounterUpdate { key, action: Step }` que aplica
/// em layout time.
///
/// Paridade vanilla: `counter.step()` → emite `CounterUpdateElem`.
/// Cristalino devolve `Value::Content(Content::CounterUpdate {...})`
/// que, quando inserido no documento, faz o Layouter aplicar
/// `CounterAction::Step` ao counter `key`.
///
/// **Não depende de `current_location`** (per P210A A3) — emite
/// Content estático; layout-time semantics. Distinto de
/// `counter.display`/`state.get` (deferred per P210A C3).
///
/// Q1=β subset minimal — apenas `counter.step()` materializado
/// nesta passada; display/get aguardam walk advance
/// implementação.
pub fn native_counter_step(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    use crate::entities::counter_update::CounterUpdate as CounterAction;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let content = Content::counter_update(key.to_string(), CounterAction::Step);
            Ok(Value::Content(content))
        }
        [other] => err(format!(
            "counter_step() requer string como argumento (key), \
             recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_step() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P208C (M9c)** — `locate(kind)` — retorna a **primeira** Location
/// de um elemento do `kind` indicado.
///
/// Paridade vanilla: `locate(selector) -> Location` retorna primeira
/// match. Cristalino P208C aceita **apenas `kind-as-string`** (paridade
/// com `native_query` P175 minimal). `locate(<label>)` requer
/// `Selector::Label` que será materializado em P209 (per
/// `P207A.div-1` Q-decisões).
///
/// Reusa pattern literal de `native_query`:
/// - 1 arg `Value::Str(kind)`.
/// - `ElementKind::from_name(kind)` → `Selector::Kind`.
/// - `ctx.introspector.query(&selector).first().copied()`.
///
/// Retorno:
/// - `Value::Location(loc)` se kind tem ≥1 match.
/// - `Value::None` se kind válido mas sem matches (`Vec::first` →
///   `None`).
/// - `SourceResult::Err` se kind inválido ou arg não-string.
pub fn native_locate(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "locate")?;
    let first = ctx.introspector.query(&selector).first().copied();
    Ok(match first {
        Some(loc) => Value::Location(loc),
        None => Value::None,
    })
}

/// `symbol(...)` — constrói um símbolo Unicode nomeado com variantes.
///
/// **P765a**: paridade com vanilla CLI 0.15.0. Cada argumento posicional é
/// uma variante:
/// - string de um só grapheme → variante base;
/// - array `(modifiers, char)` → variante nomeada.
///
/// Ex.: `symbol("🖂", ("stamped", "🖃"))`.
pub fn native_symbol(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::symbol::{Symbol, SymbolVariant};
    use ecow::EcoString;
    use unicode_segmentation::UnicodeSegmentation;

    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("expected at least one variant");
    }

    let mut variants: Vec<SymbolVariant> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Str(s) => {
                let s = s.as_str();
                if s.graphemes(true).count() != 1 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "invalid variant value: {}",
                            s.escape_debug().collect::<String>()
                        ),
                    )]);
                }
                variants.push((EcoString::default(), s.chars().next().unwrap()));
            }
            Value::Array(arr) if arr.len() == 2 => {
                let mods = match &arr[0] {
                    Value::Str(s) => s.clone(),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "symbol modifier must be string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                let ch = match &arr[1] {
                    Value::Str(s) => {
                        let s = s.as_str();
                        if s.graphemes(true).count() != 1 {
                            return Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "invalid variant value: {}",
                                    s.escape_debug().collect::<String>()
                                ),
                            )]);
                        }
                        s.chars().next().unwrap()
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "symbol variant value must be string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                variants.push((mods, ch));
            }
            Value::Array(arr) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "symbol variant array must have length 2, found {}",
                        arr.len()
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "symbol variant must be string or array, found {}",
                        other.type_name()
                    ),
                )]);
            }
        }
    }

    Ok(Value::Symbol(Symbol::runtime(variants)))
}

// ── P843 (F4/F5) — constructors `bytes(...)` e `datetime(...)` ─────────────
//
// Sondas e mensagens medidas no vanilla (`temp/p843/f4_*.typ`, `f5_*.typ`,
// binário release de `lab/typst-original`, 2026-07-22). As mensagens de
// erro são o observável (ADR-0107) — verbatim do vanilla.

/// Nome longo do tipo (paridade vanilla `Type::long_name`, usado nas
/// mensagens de erro de cast): difere do nome curto só em int/str/bool.
fn long_type_name(v: &Value) -> &'static str {
    match v.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    }
}

/// `bytes(value)` → Bytes. Aceita Str (bytes UTF-8), Array de ints 0–255
/// e Bytes (passthrough) — paridade vanilla `ToBytes`
/// (foundations/bytes.rs:425-441). Int não é aceite nesta versão do
/// vanilla (medido: `bytes(3)` → erro).
pub fn native_bytes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [] => Err(vec![SourceDiagnostic::error(args.span, "missing argument: value")]),
        [Value::Str(s)] => Ok(Value::Bytes(crate::entities::bytes::Bytes::from(
            s.as_str().as_bytes().to_vec(),
        ))),
        [Value::Array(arr)] => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    Value::Int(i) if (0..=255).contains(i) => out.push(*i as u8),
                    Value::Int(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "number must be between 0 and 255",
                        )])
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!("expected integer, found {}", long_type_name(other)),
                        )])
                    }
                }
            }
            Ok(Value::Bytes(crate::entities::bytes::Bytes::from(out)))
        }
        [Value::Bytes(b)] => Ok(Value::Bytes(b.clone())),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "expected string, array, or bytes, found {}",
                long_type_name(other)
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    }
}

/// `datetime(year:, month:, day:, hour:, minute:, second:)` → Datetime.
///
/// Paridade vanilla `Datetime::construct` (foundations/datetime.rs:265-352):
/// os 6 argumentos são nomeados e opcionais; data completa (year+month+day),
/// hora completa (hour+minute+second) ou ambas. Mensagens e hints medidos
/// no vanilla (`temp/p843/f5_*.typ`).
pub fn native_datetime(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::world_types::Datetime;

    const FIELDS: [&str; 6] = ["year", "month", "day", "hour", "minute", "second"];

    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for key in args.named.keys() {
        if !FIELDS.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {}", key.as_str()),
            )]);
        }
    }

    let get_int = |name: &str| -> SourceResult<Option<i64>> {
        match args.named.get(name) {
            None => Ok(None),
            Some(Value::Int(i)) => Ok(Some(*i)),
            Some(other) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", long_type_name(other)),
            )]),
        }
    };

    let year = get_int("year")?;
    let month = get_int("month")?;
    let day = get_int("day")?;
    let hour = get_int("hour")?;
    let minute = get_int("minute")?;
    let second = get_int("second")?;

    /// Hints do vanilla: "the `hour` and `minute` arguments" etc.
    fn format_missing_args(missing: &[&str]) -> String {
        match missing {
            [arg] => format!("the `{arg}` argument"),
            [a, b] => format!("the `{a}` and `{b}` arguments"),
            [rest @ .., tail] => format!(
                "the {}, and `{tail}` arguments",
                rest.iter().map(|a| format!("`{a}`")).collect::<Vec<_>>().join(", ")
            ),
            [] => unreachable!(),
        }
    }

    let time = match (hour, minute, second) {
        (Some(h), Some(m), Some(s)) => {
            let (Ok(h), Ok(m), Ok(s)) =
                (u8::try_from(h), u8::try_from(m), u8::try_from(s))
            else {
                return Err(vec![SourceDiagnostic::error(args.span, "time is invalid")]);
            };
            match time::Time::from_hms(h, m, s) {
                Ok(t) => Some(t),
                Err(_) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "time is invalid",
                    )])
                }
            }
        }
        (None, None, None) => None,
        (h, m, s) => {
            let mut missing = Vec::new();
            if h.is_none() {
                missing.push("hour");
            }
            if m.is_none() {
                missing.push("minute");
            }
            if s.is_none() {
                missing.push("second");
            }
            return Err(vec![SourceDiagnostic::error(args.span, "time is incomplete")
                .with_hint(format!(
                    "add {} to get a valid time",
                    format_missing_args(&missing)
                ))]);
        }
    };

    let date = match (year, month, day) {
        (Some(y), Some(mo), Some(d)) => {
            let month = match u8::try_from(mo).ok().and_then(|m| time::Month::try_from(m).ok())
            {
                Some(m) => m,
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "month is invalid",
                    )])
                }
            };
            let (Ok(y), Ok(d)) = (i32::try_from(y), u8::try_from(d)) else {
                return Err(vec![SourceDiagnostic::error(args.span, "date is invalid")]);
            };
            match time::Date::from_calendar_date(y, month, d) {
                Ok(date) => Some(date),
                Err(_) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "date is invalid",
                    )])
                }
            }
        }
        (None, None, None) => None,
        (y, mo, d) => {
            let mut missing = Vec::new();
            if y.is_none() {
                missing.push("year");
            }
            if mo.is_none() {
                missing.push("month");
            }
            if d.is_none() {
                missing.push("day");
            }
            return Err(vec![SourceDiagnostic::error(args.span, "date is incomplete")
                .with_hint(format!(
                    "add {} to get a valid date",
                    format_missing_args(&missing)
                ))]);
        }
    };

    match Datetime::from_parts(date, time) {
        Some(dt) => Ok(Value::Datetime(dt)),
        None => Err(vec![
            SourceDiagnostic::error(
                args.span,
                "at least one of date or time must be fully specified",
            )
            .with_hint("add the `hour`, `minute`, and `second` arguments to get a valid time")
            .with_hint("add the `year`, `month`, and `day` arguments to get a valid date"),
        ]),
    }
}

#[cfg(test)]
mod tests_p699b_str_bytes {
    use super::*;
    use crate::entities::bytes::Bytes;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

    fn ctx() -> EvalContext {
        EvalContext::new()
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }

    impl crate::contracts::world::World for NullWorld {
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
        fn file(&self, _: FileId) -> FileResult<crate::entities::world_types::Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn null_world() -> NullWorld {
        NullWorld::default()
    }

    /// P699b — `str()` sobre `Value::Bytes` válido UTF-8 decodifica (paridade
    /// vanilla `foundations/str.rs:871`). Reproduz o gap encontrado ao
    /// compilar `str(plugin("hello.wasm").hello())` via documento real.
    #[test]
    fn str_de_bytes_valido_utf8_decodifica() {
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"hello".to_vec()))]);
        let v = native_str(&mut ctx(), &args, &null_world(), test_file_id()).unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn str_de_bytes_invalido_utf8_erro_verbatim() {
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![0xFF, 0xFE]))]);
        let e = native_str(&mut ctx(), &args, &null_world(), test_file_id()).unwrap_err();
        assert!(
            e[0].message.contains("bytes are not valid UTF-8"),
            "msg: {}",
            e[0].message,
        );
    }
}

#[cfg(test)]
mod tests_p703_rgb_hex {
    use super::*;
    use crate::entities::layout_types::Color;

    fn hex(s: &str) -> Value {
        parse_hex_color(s).unwrap()
    }

    #[test]
    fn hex_6_digitos_com_hash() {
        assert_eq!(hex("#FF0000"), Value::Color(Color::rgba(255, 0, 0, 255)));
    }

    #[test]
    fn hex_6_digitos_sem_hash() {
        assert_eq!(hex("FF0000"), Value::Color(Color::rgba(255, 0, 0, 255)));
    }

    #[test]
    fn hex_8_digitos_com_alpha() {
        assert_eq!(hex("#FF0000FF"), Value::Color(Color::rgba(255, 0, 0, 255)));
        assert_eq!(hex("FF000080"), Value::Color(Color::rgba(255, 0, 0, 0x80)));
    }

    #[test]
    fn hex_3_digitos_curto_duplica() {
        assert_eq!(hex("F00"), Value::Color(Color::rgba(255, 0, 0, 255)));
        assert_eq!(hex("abc"), Value::Color(Color::rgba(0xaa, 0xbb, 0xcc, 255)));
    }

    #[test]
    fn hex_4_digitos_curto_com_alpha() {
        // P703 — 4º dígito curto é alpha, não mais um componente de cor.
        // Medido contra o vanilla: rgb("FF00") -> rgb("#ffff0000").
        assert_eq!(hex("FF00"), Value::Color(Color::rgba(255, 255, 0, 0)));
    }

    #[test]
    fn hex_invalido_erro_verbatim() {
        let e = parse_hex_color("nothex").unwrap_err();
        assert!(
            e[0].message.contains("color string contains non-hexadecimal letters"),
            "msg: {}",
            e[0].message,
        );
    }

    #[test]
    fn hex_nome_de_cor_erro_como_no_vanilla() {
        // "red" não é hex válido — o vanilla também erra aqui, não é feature.
        let e = parse_hex_color("red").unwrap_err();
        assert!(
            e[0].message.contains("color string contains non-hexadecimal letters"),
            "msg: {}",
            e[0].message,
        );
    }

    #[test]
    fn hex_comprimento_errado_erro_verbatim() {
        let e = parse_hex_color("FFFFF").unwrap_err(); // 5 dígitos
        assert!(
            e[0].message.contains("color string has wrong length"),
            "msg: {}",
            e[0].message,
        );
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    #[test]
    fn native_rgb_forma_numerica_sem_regressao() {
        let args =
            Args::positional(vec![Value::Int(255), Value::Int(0), Value::Int(128)]);
        let v = native_rgb(
            &mut crate::compiler::eval::EvalContext::new(),
            &args,
            &NullWorld::default(),
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Color(Color::rgb(255, 0, 128)));
    }
}

#[cfg(test)]
mod tests_p704_range_step {
    use super::*;

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn r(items: Vec<Value>, named: &[(&str, Value)]) -> Value {
        let mut args = Args::positional(items);
        for (k, v) in named {
            args.named.insert((*k).into(), v.clone());
        }
        native_range(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap()
    }

    fn arr(items: &[i64]) -> Value {
        Value::Array(items.iter().map(|i| Value::Int(*i)).collect())
    }

    #[test]
    fn sem_step_sem_regressao() {
        assert_eq!(r(vec![Value::Int(3)], &[]), arr(&[0, 1, 2]));
        assert_eq!(r(vec![Value::Int(2), Value::Int(5)], &[]), arr(&[2, 3, 4]));
        assert_eq!(r(vec![Value::Int(5), Value::Int(2)], &[]), arr(&[]));
    }

    #[test]
    fn negativo_devolve_vazio_nao_erro() {
        // P704 — corrige divergência: o vanilla devolve `()`, não Err.
        assert_eq!(r(vec![Value::Int(-5)], &[]), arr(&[]));
    }

    #[test]
    fn step_positivo_e_negativo() {
        assert_eq!(
            r(vec![Value::Int(90), Value::Int(40)], &[("step", Value::Int(-12))]),
            arr(&[90, 78, 66, 54, 42])
        );
        assert_eq!(
            r(vec![Value::Int(0), Value::Int(10)], &[("step", Value::Int(2))]),
            arr(&[0, 2, 4, 6, 8])
        );
        assert_eq!(
            r(vec![Value::Int(10), Value::Int(0)], &[("step", Value::Int(-1))]),
            arr(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn step_com_1_argumento() {
        assert_eq!(
            r(vec![Value::Int(20)], &[("step", Value::Int(4))]),
            arr(&[0, 4, 8, 12, 16])
        );
        assert_eq!(
            r(vec![Value::Int(21)], &[("step", Value::Int(4))]),
            arr(&[0, 4, 8, 12, 16, 20])
        );
    }

    #[test]
    fn direcao_incompativel_devolve_vazio() {
        assert_eq!(
            r(vec![Value::Int(0), Value::Int(10)], &[("step", Value::Int(-1))]),
            arr(&[])
        );
    }

    #[test]
    fn step_com_inclusive() {
        assert_eq!(
            r(
                vec![Value::Int(-6)],
                &[("step", Value::Int(-2)), ("inclusive", Value::Bool(true))]
            ),
            arr(&[0, -2, -4, -6])
        );
        assert_eq!(
            r(vec![Value::Int(0)], &[("inclusive", Value::Bool(true))]),
            arr(&[0])
        );
        assert_eq!(
            r(vec![Value::Int(7), Value::Int(10)], &[("inclusive", Value::Bool(true))]),
            arr(&[7, 8, 9, 10])
        );
    }

    #[test]
    fn step_zero_erro_verbatim() {
        let mut args = Args::positional(vec![Value::Int(0), Value::Int(10)]);
        args.named.insert("step".into(), Value::Int(0));
        let e =
            native_range(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap_err();
        assert!(
            e[0].message.contains("number must not be zero"),
            "msg: {}",
            e[0].message
        );
    }
}

#[cfg(test)]
mod tests_p705_luma_ratio {
    use super::*;
    use crate::entities::layout_types::{Color, Ratio};

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn luma(items: Vec<Value>) -> Value {
        native_luma(&mut ctx(), &Args::positional(items), &NullWorld::default(), tfid())
            .unwrap()
    }

    #[test]
    fn ratio_valido_mapeia_diretamente() {
        assert_eq!(luma(vec![Value::Ratio(Ratio(0.5))]), Value::Color(Color::luma(0.5)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(0.0))]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(1.0))]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn percentagem_literal_via_relative_mapeia_diretamente() {
        // P705 — causa raiz real: `50%`/`v * 1%` avaliam para Value::Relative
        // neste cristalino (unificado com `length`), não Value::Ratio.
        // Reproduz exactamente o caminho de cetz: `range(...).map(v => luma(v * 1%))`.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel = |pct: f64| Value::Relative(Rel { rel: pct, abs: Length::ZERO });
        assert_eq!(luma(vec![rel(0.9)]), Value::Color(Color::luma(0.9)));
        assert_eq!(luma(vec![rel(0.5)]), Value::Color(Color::luma(0.5)));
        assert_eq!(luma(vec![rel(0.0)]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![rel(1.0)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn percentagem_com_parte_absoluta_cai_no_fallback() {
        // `50% + 1pt` não é um componente de cor válido (nem no vanilla) —
        // deve cair no fallback branco, não ser tratado como 50%.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        assert_eq!(luma(vec![rel]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn int_valido_sem_regressao() {
        assert_eq!(luma(vec![Value::Int(0)]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![Value::Int(255)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn int_fora_de_gama_devolve_branco_nao_erro() {
        // P705 — corrigido: paridade vanilla medida (luma(300) -> branco).
        assert_eq!(luma(vec![Value::Int(300)]), Value::Color(Color::luma(1.0)));
        assert_eq!(luma(vec![Value::Int(256)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn ratio_fora_de_gama_devolve_branco_nao_erro() {
        assert_eq!(luma(vec![Value::Ratio(Ratio(1.5))]), Value::Color(Color::luma(1.0)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(-0.2))]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn tipo_errado_devolve_branco_nao_erro() {
        assert_eq!(luma(vec![Value::Str("bad".into())]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn sem_argumentos_devolve_branco() {
        assert_eq!(luma(vec![]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn dois_ou_mais_argumentos_erro_estrutural() {
        let e = native_luma(
            &mut ctx(),
            &Args::positional(vec![Value::Int(0), Value::Int(1), Value::Int(2)]),
            &NullWorld::default(),
            tfid(),
        )
        .unwrap_err();
        assert!(
            e[0].message.contains("requer 0 ou 1 argumento"),
            "msg: {}",
            e[0].message
        );
    }
}
