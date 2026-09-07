//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/repr.md
//! @prompt-hash 14f917a5
//! @layer L1
//! @updated 2026-09-01
//!
//! P421 — `repr()` exaustivo para `Value`, `Content` e `Selector`.
//!
//! Implementação como free functions (ADR-0109 forma B). A representação é
//! **reconhecível**, não garante round-trip. Variants complexos usam
//! representação scope-out ("function", "module", nome do tipo).

use crate::entities::content::Content;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::geometry::Stroke;
use crate::entities::gradient::Gradient;
use crate::entities::layout_types::{
    Align2D, Angle, Color, HAlign, Length, Ratio, VAlign,
};
use crate::entities::paint::Paint;
use crate::entities::rel::Rel;
use crate::entities::selector::Selector;
use crate::entities::tiling::Tiling;
use crate::entities::value::{Type, Value};

/// Representação de um `Value`.
pub fn repr_value(v: &Value) -> String {
    match v {
        Value::None => "none".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => repr_float(*f),
        Value::Str(s) => format!("\"{}\"", s.as_str().escape_debug()),
        Value::Content(c) => repr_content(c),
        Value::LocatedContent(c, _) => match c.fields() {
            Some(fields) => {
                let pieces: Vec<String> = fields
                    .iter()
                    .filter(|(name, _)| name.as_str() != "label")
                    .map(|(name, value)| format!("{name}: {}", repr_value(value)))
                    .collect();
                format!(
                    "{}{}",
                    c.content().elem_name(),
                    pretty_array_like(&pieces, false)
                )
            }
            None => repr_content(c.content()),
        },
        Value::Array(arr) => {
            let mut items: Vec<String> = arr.iter().take(40).map(repr_value).collect();
            if arr.len() > 40 {
                items.push(format!(".. ({} items omitted)", arr.len() - 40));
            }
            // P801 — array de exactamente 1 elemento leva vírgula final
            // (paridade vanilla `pretty_array_like(_, len == 1)`),
            // distinguindo-o de parênteses de agrupamento: `(5,)` ≠ `(5)`.
            pretty_array_like(&items, items.len() == 1)
        }
        Value::Dict(dict) => {
            // P695 — dict vazio é `(:)`, distinto de array vazio `()` (paridade
            // vanilla). Sem isto, `repr(()) == repr((:)) == "()"` — dois tipos
            // diferentes com texto de depuração idêntico.
            if dict.is_empty() {
                return "(:)".to_string();
            }
            let items: Vec<String> = dict
                .iter()
                .map(|(k, v)| format!("{}: {}", k.as_str(), repr_value(v)))
                .collect();
            format!("({})", items.join(", "))
        }
        Value::Module(m) => format!("<module {}>", m.name()),
        Value::Datetime(d) => repr_datetime(d),
        Value::Func(f) => {
            // **P744** — paridade vanilla medida: closures (anónimas ou
            // nomeadas) repr como `"(..) => .."`; nativas usam o nome
            // (`repr(rgb)` → `"rgb"`, P742); função sem nome mantém
            // `"#function(...)"`.
            if is_closure(&f) {
                "(..) => ..".to_string()
            } else if let Some(name) = f.name() {
                if name.is_empty() {
                    "#function(...)".to_string()
                } else {
                    name.rsplit('.').next().unwrap_or(name).to_string()
                }
            } else {
                "#function(...)".to_string()
            }
        }
        Value::Auto => "auto".to_string(),
        Value::Length(l) => repr_length(l),
        Value::Relative(rel) => repr_relative(rel),
        Value::Ratio(r) => repr_ratio(*r),
        Value::Angle(a) => repr_angle(*a),
        Value::Color(c) => repr_color(c),
        Value::Stroke(s) => repr_stroke(s),
        Value::Fraction(f) => format_float_with_unit(*f, "fr"),
        Value::Align(a) => repr_align(a),
        Value::Location(_) => "location(..)".to_string(),
        Value::Gradient(gradient) => repr_gradient(gradient),
        Value::Regex(r) => format!("regex(\"{}\")", r.pattern()),
        Value::Tiling(tiling) => repr_tiling(tiling),
        Value::Bytes(b) => format!("bytes({})", b.len()),
        // P817-D — paridade vanilla: repr de decimal é `decimal("...")`.
        Value::Decimal(d) => format!("decimal(\"{}\")", d.to_string()),
        Value::Duration(d) => repr_duration(d),
        Value::Version(ver) => {
            // P684 — componentes arbitrários; só os três primeiros têm nome, mas
            // todos são impressos (`version(1, 2, 3, 4, 5)`, `version()` se vazio).
            let comps = ver
                .components
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("version({})", comps)
        }
        Value::Selector(s) => repr_selector(s),
        Value::Symbol(s) => format!("symbol({})", s.repr_variants()),
        Value::Args(a) => {
            if let Some(occurrences) = &a.occurrences {
                let pieces: Vec<String> = occurrences
                    .iter()
                    .map(|occurrence| match &occurrence.name {
                        Some(name) => {
                            format!("{name}: {}", repr_value(&occurrence.value))
                        }
                        None => repr_value(&occurrence.value),
                    })
                    .collect();
                return format!("arguments{}", pretty_array_like(&pieces, false));
            }
            // **P740B** — paridade vanilla `Args::repr`
            // (foundations/args.rs:457-461): lista os NOMEADOS primeiro
            // (ordem de inserção), depois os posicionais. Medido:
            // `f(1, 2, z: 3)` no sink → "arguments(z: 3, 1, 2)";
            // vazio → "arguments()". Antes: "arguments(...)" (lossy).
            let mut pieces: Vec<String> = a
                .named
                .iter()
                .map(|(k, v)| format!("{k}: {}", repr_value(v)))
                .collect();
            pieces.extend(a.items.iter().map(repr_value));
            format!("arguments({})", pieces.join(", "))
        }
        Value::State(state) => format!(
            "state({}, {})",
            repr_value(&Value::Str(state.key.clone())),
            repr_value(&state.init),
        ),
        Value::Counter(counter) => {
            use crate::entities::counter::CounterKey;
            let key = match &counter.key {
                CounterKey::Page => "page".to_string(),
                CounterKey::Str(key) => repr_value(&Value::Str(key.clone())),
                CounterKey::Selector(selector) => repr_selector(selector),
            };
            format!("counter({key})")
        }
        Value::Label(l) => {
            if crate::compiler::lexer::is_valid_label_literal_id(&l.0) {
                format!("<{}>", l.0)
            } else {
                format!("label({})", repr_value(&Value::Str(l.0.as_str().into())))
            }
        }
        Value::Dir(d) => format!("{:?}", d).to_lowercase(),
        Value::Path(path) => {
            format!("path(\"{}\")", path.vpath().get_with_slash().escape_debug())
        }
        // P685 — nome de tipo como valor: repr(int) == "int", repr(type) == "type".
        // **P843 (F3)** — exceções medidas no vanilla (`ty.rs:159-163`,
        // fixture `temp/p843/f3_type_none_auto.typ`): repr(type(none)) →
        // "type(none)"; repr(type(auto)) → "type(auto)".
        Value::Type(t) => match t {
            Type::None => "type(none)".to_string(),
            Type::Auto => "type(auto)".to_string(),
            other => other.name().to_string(),
        },
    }
}

/// **P843/P850 (F1)** — repr de `Duration` no formato nomeado do vanilla
/// (`Duration::repr`, foundations/duration.rs:137-160): só os componentes
/// não-zero, de `weeks` a `seconds`; segundos truncados (sub-segundo não
/// aparece — medido: `repr(duration(seconds: 3) / 2)` →
/// `duration(seconds: 1)`); zero → `duration()`. Durações negativas
/// representam-se com componentes negativos (P850).
fn repr_duration(d: &crate::entities::duration::Duration) -> String {
    const MINUTE: i128 = 60;
    const HOUR: i128 = 60 * MINUTE;
    const DAY: i128 = 24 * HOUR;
    const WEEK: i128 = 7 * DAY;

    if d.nanos == 0 {
        return "duration()".to_string();
    }

    let negative = d.nanos < 0;
    let mut rem = (d.nanos / 1_000_000_000).abs();
    let weeks = rem / WEEK;
    rem %= WEEK;
    let days = rem / DAY;
    rem %= DAY;
    let hours = rem / HOUR;
    rem %= HOUR;
    let minutes = rem / MINUTE;
    let seconds = rem % MINUTE;

    let mut parts: Vec<String> = Vec::with_capacity(5);
    let push = |parts: &mut Vec<String>, name: &str, value: i128| {
        if value != 0 {
            parts.push(format!("{name}: {value}"));
        }
    };
    let sign = if negative { -1 } else { 1 };
    push(&mut parts, "weeks", sign * weeks as i128);
    push(&mut parts, "days", sign * days as i128);
    push(&mut parts, "hours", sign * hours as i128);
    push(&mut parts, "minutes", sign * minutes as i128);
    push(&mut parts, "seconds", sign * seconds as i128);
    format!("duration{}", pretty_array_like(&parts, false))
}

/// Representação de um comprimento relativo (`Rel<Length>`).
///
/// Percentagem pura (abs zero) imprime só a percentagem — paridade
/// observável com o vanilla, onde `50%` é `Ratio`, não `Rel`.
/// Com offset absoluto: `"{pct} + {length}"` (paridade vanilla `Rel::repr`,
/// layout/rel.rs:149 — incluindo `50% + -3pt` para offset negativo).
fn repr_relative(rel: &Rel<Length>) -> String {
    let pct = format_float_with_unit(rel.rel * 100.0, "%");
    if rel.abs.is_zero() {
        pct
    } else {
        format!("{} + {}", pct, repr_length(&rel.abs))
    }
}

// ── P721 — repr Typst (paridade vanilla), não Debug do Rust ────────────────

/// Arredondamento "half away from zero" a `precision` casas decimais —
/// paridade vanilla `round_with_precision` (typst-utils/src/round.rs:26).
/// Valores não-finitos passam inalterados.
fn round_with_precision(value: f64, precision: i32) -> f64 {
    if !value.is_finite() {
        return value;
    }
    let offset = 10f64.powi(precision);
    (value * offset).round() / offset
}

/// Float formatado com unidade, arredondado a 2 casas decimais — paridade
/// vanilla `repr::format_float_with_unit` (foundations/repr.rs:128).
/// NaN/Inf seguem a forma `float.nan * 1{unit}` / `float.inf * 1{unit}`.
fn format_float_with_unit(value: f64, unit: &str) -> String {
    format_float_rounded(value, 2, unit)
}

/// Componente float de cor (a/b de Oklab, chroma de Oklch), arredondado a
/// 3 casas decimais — paridade vanilla `repr::format_float_component`
/// (foundations/repr.rs:122).
fn format_float_component(value: f64) -> String {
    format_float_rounded(value, 3, "")
}

fn format_float_rounded(value: f64, precision: i32, unit: &str) -> String {
    let value = round_with_precision(value, precision);
    let multiplication = if unit.is_empty() { "" } else { " * 1" };
    if value.is_nan() {
        format!("float.nan{multiplication}{unit}")
    } else if value.is_infinite() {
        let sign = if value < 0.0 { "-" } else { "" };
        format!("{sign}float.inf{multiplication}{unit}")
    } else {
        format!("{value}{unit}")
    }
}

/// Representação de um `Length` — paridade vanilla `Length::repr`
/// (layout/length.rs:173): `"{abs} + {em}"` quando ambos não-zero;
/// só `em` quando abs é zero; só `abs` nos restantes casos.
fn repr_length(l: &Length) -> String {
    let abs = format_float_with_unit(l.abs.to_pt(), "pt");
    let em = format_float_with_unit(l.em, "em");
    match (l.abs.is_zero(), l.em == 0.0) {
        (false, false) => format!("{abs} + {em}"),
        (true, false) => em,
        (_, true) => abs,
    }
}

/// Representação de um `Ratio` — paridade vanilla `Ratio::repr`
/// (layout/ratio.rs:135).
fn repr_ratio(r: Ratio) -> String {
    format_float_with_unit(r.get() * 100.0, "%")
}

/// Representação de um `Angle` em graus — paridade vanilla `Angle::repr`
/// (layout/angle.rs:171).
fn repr_angle(a: Angle) -> String {
    format_float_with_unit(a.to_deg(), "deg")
}

/// Representação de uma `Color` — paridade vanilla `ProcessColor::repr`
/// (visualize/color.rs:1918). sRGB imprime-se em hex (`rgb("#rrggbb")`,
/// com sufixo alpha quando != 255); os restantes espaços imprimem-se como
/// chamada de construtor. Alpha diferente de 1.0 aparece como argumento
/// extra (excepto CMYK, que não tem alpha).
fn repr_color(c: &Color) -> String {
    let pct = |v: f32| format_float_with_unit(v as f64 * 100.0, "%");
    let comp = |v: f32| format_float_component(v as f64);
    let hue = |h: f32| format_float_with_unit((h as f64).rem_euclid(360.0), "deg");
    match *c {
        Color::Srgb { r, g, b, a } => {
            let to_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            let (r, g, b, a) = (to_u8(r), to_u8(g), to_u8(b), to_u8(a));
            if a == 255 {
                format!("rgb(\"#{r:02x}{g:02x}{b:02x}\")")
            } else {
                format!("rgb(\"#{r:02x}{g:02x}{b:02x}{a:02x}\")")
            }
        }
        Color::Luma { l, a } => {
            if a == 1.0 {
                format!("luma({})", pct(l))
            } else {
                format!("luma({}, {})", pct(l), pct(a))
            }
        }
        Color::LinearRgb { r, g, b, a } => {
            if a == 1.0 {
                format!("color.linear-rgb({}, {}, {})", pct(r), pct(g), pct(b))
            } else {
                format!(
                    "color.linear-rgb({}, {}, {}, {})",
                    pct(r),
                    pct(g),
                    pct(b),
                    pct(a)
                )
            }
        }
        Color::Oklab { l, a, b, alpha } => {
            if alpha == 1.0 {
                format!("oklab({}, {}, {})", pct(l), comp(a), comp(b))
            } else {
                format!("oklab({}, {}, {}, {})", pct(l), comp(a), comp(b), pct(alpha))
            }
        }
        Color::Oklch { l, c: chroma, h, alpha } => {
            if alpha == 1.0 {
                format!("oklch({}, {}, {})", pct(l), comp(chroma), hue(h))
            } else {
                format!("oklch({}, {}, {}, {})", pct(l), comp(chroma), hue(h), pct(alpha))
            }
        }
        Color::Hsl { h, s, l, a } => {
            if a == 1.0 {
                format!("color.hsl({}, {}, {})", hue(h), pct(s), pct(l))
            } else {
                format!("color.hsl({}, {}, {}, {})", hue(h), pct(s), pct(l), pct(a))
            }
        }
        Color::Hsv { h, s, v, a } => {
            if a == 1.0 {
                format!("color.hsv({}, {}, {})", hue(h), pct(s), pct(v))
            } else {
                format!("color.hsv({}, {}, {}, {})", hue(h), pct(s), pct(v), pct(a))
            }
        }
        Color::Cmyk { c, m, y, k } => {
            format!("cmyk({}, {}, {}, {})", pct(c), pct(m), pct(y), pct(k))
        }
    }
}

/// Representação de um `Stroke` — paridade vanilla `Stroke::repr`
/// (visualize/stroke.rs:308) adaptada: o cristalino não modela
/// `Smart::Auto` (thickness colapsa para 1.0pt na construção, P227), logo
/// imprime sempre `"{thickness} + {paint}"`.
pub(super) fn repr_stroke(s: &Stroke) -> String {
    use crate::entities::geometry::{DashLength, LineCap, LineJoin};
    let complex = s.specified.cap
        || s.specified.join
        || s.specified.dash
        || s.specified.miter_limit;
    if !complex {
        if !s.specified.paint
            && !s.specified.thickness
            && s.paint == Stroke::default().paint
            && s.thickness == 1.0
        {
            return "1pt + black".to_string();
        }
        return match (s.specified.paint, s.specified.thickness) {
            (true, false) => repr_paint(&s.paint),
            (false, true) => format_float_with_unit(s.thickness, "pt"),
            _ => format!(
                "{} + {}",
                format_float_with_unit(s.thickness, "pt"),
                repr_paint(&s.paint)
            ),
        };
    }
    let mut fields = Vec::new();
    if s.specified.paint {
        fields.push(format!("paint: {}", repr_paint(&s.paint)));
    }
    if s.specified.thickness {
        fields.push(format!("thickness: {}", format_float_with_unit(s.thickness, "pt")));
    }
    if s.specified.cap {
        fields.push(format!(
            "cap: {:?}",
            match s.cap {
                LineCap::Butt => "butt",
                LineCap::Round => "round",
                LineCap::Square => "square",
            }
        ));
    }
    if s.specified.join {
        fields.push(format!(
            "join: {:?}",
            match s.join {
                LineJoin::Miter => "miter",
                LineJoin::Round => "round",
                LineJoin::Bevel => "bevel",
            }
        ));
    }
    if s.specified.dash {
        if let Some(dash) = &s.dash {
            let array = dash
                .array
                .iter()
                .map(|value| match value {
                    DashLength::Length(value) => format_float_with_unit(*value, "pt"),
                    DashLength::LineWidth => "\"dot\"".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(format!(
                "dash: (array: ({array}), phase: {})",
                format_float_with_unit(dash.phase, "pt")
            ));
        } else {
            fields.push("dash: none".to_string());
        }
    }
    if s.specified.miter_limit {
        let value = if s.miter_limit.fract() == 0.0 {
            format!("{:.1}", s.miter_limit)
        } else {
            format_float_component(s.miter_limit)
        };
        fields.push(format!("miter-limit: {value}"));
    }
    format!("({})", fields.join(", "))
}

/// Representação de um gradient linear com stops já convertidas para o espaço
/// de interpolação e offsets automáticos resolvidos.
fn repr_gradient(gradient: &Gradient) -> String {
    match gradient {
        Gradient::Linear(linear) => {
            let offsets = linear.effective_offsets();
            let stops = linear
                .stops
                .iter()
                .zip(offsets)
                .map(|(stop, offset)| {
                    format!(
                        "({}, {})",
                        repr_color(&stop.color.to_space(linear.space)),
                        repr_ratio(Ratio(offset as f64))
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("gradient.linear({stops})")
        }
        Gradient::Radial(_) | Gradient::Conic(_) => "gradient(...)".to_string(),
    }
}

/// Representação pública mínima de tiling: a dimensão explícita é observável;
/// o corpo declarativo permanece representado pelo spread morfológico.
fn repr_tiling(tiling: &Tiling) -> String {
    match tiling.size {
        Some(size) => format!(
            "tiling(({}, {}), ..)",
            format_float_with_unit(size.width.val(), "pt"),
            format_float_with_unit(size.height.val(), "pt")
        ),
        None => "tiling(...)".to_string(),
    }
}

/// Representação de um `Paint` delegada às mesmas formas públicas de `Value`.
fn repr_paint(p: &Paint) -> String {
    match p {
        Paint::Solid(c) => repr_color(c),
        Paint::Gradient(gradient) => repr_gradient(gradient),
        Paint::Tiling(tiling) => repr_tiling(tiling),
    }
}

/// Representação de um `Align2D` — paridade vanilla `Alignment::repr`
/// (layout/align.rs:250): componente horizontal primeiro (`"{h} + {v}"`).
fn repr_align(a: &Align2D) -> String {
    let h = a.h.map(|h| match h {
        HAlign::Start => "start",
        HAlign::Left => "left",
        HAlign::Center => "center",
        HAlign::Right => "right",
        HAlign::End => "end",
    });
    let v = a.v.map(|v| match v {
        VAlign::Top => "top",
        VAlign::Horizon => "horizon",
        VAlign::Bottom => "bottom",
    });
    match (h, v) {
        (Some(h), Some(v)) => format!("{h} + {v}"),
        (Some(h), None) => h.to_string(),
        (None, Some(v)) => v.to_string(),
        // `Align2D { None, None }` comporta-se como `left + top`
        // (entities/layout_types.rs:359-361).
        (None, None) => "left + top".to_string(),
    }
}

fn repr_math_attach(
    attach: &crate::entities::elements::math_attach::MathAttachElem,
) -> String {
    let mut fields = vec![format!("base: {}", repr_p1293_math_field(&attach.base))];
    for (name, slot) in [
        ("t", &attach.t),
        ("b", &attach.b),
        ("tl", &attach.tl),
        ("bl", &attach.bl),
        ("tr", &attach.tr),
        ("br", &attach.br),
    ] {
        match slot {
            MathAttachSlot::Omitted => {}
            MathAttachSlot::ExplicitNone => fields.push(format!("{name}: none")),
            MathAttachSlot::Present(content) => {
                fields.push(format!("{name}: {}", repr_p1293_math_field(content)))
            }
        }
    }
    format!("attach{}", pretty_array_like(&fields, false))
}

/// Projeção textual canônica de uma folha matemática direta nos campos
/// morfológicos fechados por P1292/P1293. O payload permanece matemático;
/// apenas a representação pública usa a forma de conteúdo `[texto]`.
fn repr_p1293_math_field(content: &Content) -> String {
    match content {
        Content::MathIdent(text) | Content::MathText(text) => {
            repr_content(&Content::Text(text.clone()))
        }
        other => repr_content(other),
    }
}

fn repr_canonical_binom(
    delimited: &crate::entities::elements::math_delimited::MathDelimitedElem,
) -> Option<String> {
    if (delimited.open, delimited.close) != ('(', ')') {
        return None;
    }
    let Content::MathFrac(frac) = &delimited.body else {
        return None;
    };
    if frac.line {
        return None;
    }
    let Content::MathSequence(denominator) = &frac.den else {
        return None;
    };
    if denominator.is_empty() || denominator.len() % 2 == 0 {
        return None;
    }

    let mut lower = Vec::with_capacity(denominator.len().div_ceil(2));
    for (index, content) in denominator.iter().enumerate() {
        if index % 2 == 0 {
            lower.push(repr_p1293_math_field(content));
        } else if !matches!(content, Content::MathText(text) if text.as_str() == ", ") {
            return None;
        }
    }
    let fields = [
        format!("upper: {}", repr_p1293_math_field(&frac.num)),
        format!("lower: {}", pretty_array_like(&lower, lower.len() == 1)),
    ];
    Some(format!("binom{}", pretty_array_like(&fields, false)))
}

/// Representação de um `Content`.
///
/// **P843 (F2)** — formato estrutural do vanilla para os variantes
/// centrais (medido em `temp/p843/f2_*.typ`): Text → `[texto]` (cru, sem
/// aspas — `TextElem::repr`), Space → `[ ]`, vazio/sequência vazia → `[]`,
/// sequência → `sequence(...)` com [`pretty_array_like`], Strong/Emph →
/// `strong(body: ...)` / `emph(body: ...)`. Os restantes variantes mantêm a
/// forma cristalina prévia (scope-out documentado no relatório do passo).
pub fn repr_content(c: &Content) -> String {
    match c {
        Content::Empty => "[]".to_string(),
        Content::Text(t) => format!("[{}]", t.as_str()),
        Content::Space => "[ ]".to_string(),
        Content::Parbreak => "parbreak()".to_string(),
        Content::Par { body } => format!("par(body: {})", repr_content(body)),
        Content::HtmlElem(elem) => {
            let mut fields = vec![format!("tag: {:?}", elem.tag.as_str())];
            if let Some(attrs) = &elem.attrs {
                let attrs = attrs
                    .iter()
                    .map(|(key, value)| format!("{}: {:?}", key, value.as_str()))
                    .collect::<Vec<_>>();
                fields.push(format!("attrs: {}", pretty_array_like(&attrs, false)));
            }
            match &elem.body {
                crate::entities::html::HtmlBody::Unset => {}
                crate::entities::html::HtmlBody::None => fields.push("body: none".into()),
                crate::entities::html::HtmlBody::Content(body) => {
                    fields.push(format!("body: {}", repr_content(body)));
                }
            }
            format!("elem{}", pretty_array_like(&fields, false))
        }
        Content::PageRun(e) => {
            let sequence = format!(
                "sequence(\n  pagebreak(weak: true),\n  flush(),\n  {},\n  pagebreak(weak: true),\n)",
                repr_content(&e.body)
            );
            let styled = e.paper.is_some()
                || e.flipped.is_some()
                || e.binding.is_some()
                || e.width.is_some()
                || e.height.is_some()
                || e.margin.is_some()
                || e.numbering.is_some()
                || e.number_align.is_some()
                || e.header.is_some()
                || e.header_ascent.is_some()
                || e.footer.is_some()
                || e.footer_descent.is_some()
                || e.supplement.is_some()
                || e.columns.is_some()
                || e.bleed.is_some()
                || e.fill.is_some()
                || e.background.is_some()
                || e.foreground.is_some();
            if styled {
                format!("styled(child: {sequence}, ..)")
            } else {
                sequence
            }
        }
        Content::Sequence(seq) => {
            if seq.is_empty() {
                return "[]".to_string();
            }
            let parts: Vec<String> = seq.iter().map(repr_content).collect();
            format!("sequence{}", pretty_array_like(&parts, false))
        }
        Content::Heading(h) => {
            format!("heading(level: {})[{}]", h.level, repr_content(&h.body))
        }
        Content::Title(t) => format!("title[{}]", repr_content(&t.body)),
        Content::Strong(s) => format!("strong(body: {})", repr_content(&s.body)),
        Content::Emph(e) => format!("emph(body: {})", repr_content(&e.body)),
        Content::Raw(r) => format!("`{}`", r.text),
        Content::ListItem(li) => format!("- {}", repr_content(&li.body)),
        Content::EnumItem(ei) => format!("+ {}", repr_content(&ei.body)),
        Content::Link(l) => format!("link(\"{}\")", l.url),
        Content::Equation(e) => {
            if e.block {
                format!("equation(block: true, body: {})", repr_content(&e.body))
            } else {
                format!("equation(body: {})", repr_content(&e.body))
            }
        }
        Content::MathSequence(seq) => seq.iter().map(repr_content).collect(),
        Content::MathIdent(s) => s.to_string(),
        Content::MathText(s) => format!("\"{}\"", s.as_str()),
        Content::MathFrac(f) => {
            format!("({})/({})", repr_content(&f.num), repr_content(&f.den))
        }
        Content::MathAttach(a) => repr_math_attach(a),
        Content::MathRoot(r) => {
            if let Some(idx) = &r.index {
                format!("root({}, {})", repr_content(idx), repr_content(&r.radicand))
            } else {
                format!("root(radicand: {})", repr_content(&r.radicand))
            }
        }
        Content::MathDelimited(d) => repr_canonical_binom(d)
            .unwrap_or_else(|| format!("{}{}{}", d.open, repr_content(&d.body), d.close)),
        Content::MathAlignPoint(_) => "&".to_string(),
        Content::Linebreak(e) => {
            if e.justify_explicit {
                format!("linebreak(justify: {})", e.justify)
            } else {
                "linebreak()".to_string()
            }
        }
        Content::MathMatrix(_) => "matrix".to_string(),
        Content::MathCases(_) => "cases".to_string(),
        Content::MathAccent(a) => {
            format!("accent({}, {})", repr_content(&a.base), repr_content(&a.accent))
        }
        Content::MathCancel(c) => {
            let mut fields = vec![format!("body: {}", repr_content(&c.body))];
            if c.explicit.length {
                fields.push(format!("length: {}", repr_relative(&c.length)));
            }
            if c.explicit.inverted {
                fields.push(format!("inverted: {}", c.inverted));
            }
            if c.explicit.cross {
                fields.push(format!("cross: {}", c.cross));
            }
            if c.explicit.angle {
                let angle = match &c.angle {
                    crate::entities::elements::math_cancel::MathCancelAngle::Auto => {
                        "auto".to_string()
                    }
                    crate::entities::elements::math_cancel::MathCancelAngle::Angle(
                        value,
                    ) => repr_angle(*value),
                    crate::entities::elements::math_cancel::MathCancelAngle::Func(
                        value,
                    ) => repr_value(&Value::Func(value.clone())),
                };
                fields.push(format!("angle: {angle}"));
            }
            if c.explicit.stroke {
                let stroke =
                    c.stroke.as_ref().map_or_else(|| "none".to_string(), repr_stroke);
                fields.push(format!("stroke: {stroke}"));
            }
            if c.explicit.background {
                fields.push(format!("background: {}", c.background));
            }
            format!("cancel{}", pretty_array_like(&fields, false))
        }
        Content::MathUnderline(u) => {
            format!("underline(body: {})", repr_content(&u.body))
        }
        Content::MathVec(v) => {
            let mut fields = Vec::new();
            if v.explicit.delim {
                let repr_delim = |value: char| {
                    if value == '\0' {
                        "none".to_string()
                    } else {
                        format!("\"{}\"", value.escape_default())
                    }
                };
                fields.push(format!(
                    "delim: ({}, {})",
                    repr_delim(v.delim.0),
                    repr_delim(v.delim.1)
                ));
            }
            if v.explicit.align {
                let align = match v.align {
                    HAlign::Left => "left",
                    HAlign::Center => "center",
                    HAlign::Right => "right",
                    HAlign::Start => "start",
                    HAlign::End => "end",
                };
                fields.push(format!("align: {align}"));
            }
            if v.explicit.gap {
                fields.push(format!(
                    "gap: {} + {}",
                    format_float_with_unit(v.gap.rel * 100.0, "%"),
                    repr_length(&v.gap.abs)
                ));
            }
            let children = v
                .children
                .iter()
                .map(|child| match child {
                    // P1292-C v7: uma folha math direta continua armazenada
                    // como math (logo layout/igualdade não mudam), mas sua
                    // projeção pública é a de Content textual `[x]`.
                    Content::MathIdent(_) | Content::MathText(_) => {
                        repr_p1293_math_field(child)
                    }
                    // Estruturas e markup conservam o formatter próprio; a
                    // projeção estreita não recursa nos seus descendentes.
                    other => repr_content(other),
                })
                .collect::<Vec<_>>();
            fields.push(format!(
                "children: {}",
                pretty_array_like(&children, children.len() == 1)
            ));
            format!("vec{}", pretty_array_like(&fields, false))
        }
        Content::MathClassOverride(c) => format!(
            "class(\"{}\", {})",
            crate::entities::math_class::math_class_name(c.class),
            repr_content(&c.body)
        ),
        Content::MathLimitsOverride(o) => {
            if o.limits {
                format!("limits({}, inline: {})", repr_content(&o.body), o.inline)
            } else {
                format!("scripts({})", repr_content(&o.body))
            }
        }
        Content::MathUnderover(u) => {
            let mut out = repr_content(&u.base);
            if let Some(under) = &u.under {
                out.push_str(&format!("under({})", repr_content(under)));
            }
            if let Some(over) = &u.over {
                out.push_str(&format!("over({})", repr_content(over)));
            }
            out
        }
        Content::MathOp(o) => {
            let fields = [
                format!("text: {}", repr_content(&o.text)),
                format!("limits: {}", o.limits),
            ];
            format!("op{}", pretty_array_like(&fields, false))
        }
        Content::MathStyled(s)
            if s.bold.is_none()
                && s.italic.is_none()
                && ((s.kind
                    == Some(crate::entities::math_style::MathStyleKind::Monospace)
                    && s.cramped.is_none())
                    || (s.kind
                        == Some(crate::entities::math_style::MathStyleKind::Script)
                        && s.cramped.is_some())) =>
        {
            format!("styled(child: {}, ..)", repr_p1293_math_field(&s.body))
        }
        Content::MathStyled(s) => repr_content(&s.body),
        Content::Label(l) if l.auto => format!("label(\"{}\")", l.name),
        Content::Label(l) => format!("label(\"{}\", {})", l.name, repr_content(&l.body)),
        Content::Ref(r) => format!("ref(<{}>)", r.name),
        Content::CounterDisplay(_) => "counter.display".to_string(),
        Content::CounterUpdate(_) => "counter.update".to_string(),
        Content::Outline(_) => "outline".to_string(),
        Content::Figure(f) => format!("figure[{}]", repr_content(&f.body)),
        Content::Image(i) => format!("image(\"{}\")", i.path),
        Content::Shape(_) => "shape".to_string(),
        Content::Curve(_) => "curve".to_string(),
        Content::Transform(t) => format!("transform[{}]", repr_content(&t.body)),
        Content::Grid(_) => "grid".to_string(),
        Content::GridHeader(_) => "grid.header".to_string(),
        Content::GridFooter(_) => "grid.footer".to_string(),
        Content::GridCell(_) => "grid.cell".to_string(),
        Content::GridHLine(_) => "grid.hline".to_string(),
        Content::GridVLine(_) => "grid.vline".to_string(),
        Content::Align(a) => format!("align({:?})[...]", a.alignment),
        Content::Place(p) => format!("place({:?})[...]", p.scope),
        Content::Flush(_) => "flush()".to_string(),
        Content::Styled(child, styles) => {
            if let Content::Equation(e) = child.as_ref() {
                let numbering = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.numbering")
                    .map(|(_, value)| value);
                let number_align = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.number-align")
                    .map(|(_, value)| value);
                let supplement = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.supplement")
                    .map(|(_, value)| value);
                let alt = styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(key, _)| key.as_str() == "equation.alt")
                    .map(|(_, value)| value);
                if numbering.is_some()
                    || number_align.is_some()
                    || supplement.is_some()
                    || alt.is_some()
                {
                    let mut fields = Vec::new();
                    if e.block {
                        fields.push("block: true".to_string());
                    }
                    if let Some(numbering) = numbering {
                        fields.push(format!("numbering: {}", repr_value(numbering)));
                    }
                    if let Some(number_align) = number_align {
                        fields
                            .push(format!("number-align: {}", repr_value(number_align)));
                    }
                    if let Some(supplement) = supplement {
                        fields.push(format!("supplement: {}", repr_value(supplement)));
                    }
                    if let Some(alt) = alt {
                        fields.push(format!("alt: {}", repr_value(alt)));
                    }
                    fields.push(format!("body: {}", repr_content(&e.body)));
                    format!("equation({})", fields.join(", "))
                } else {
                    repr_content(child)
                }
            } else {
                repr_content(child)
            }
        }
        Content::Divider(_) => "divider".to_string(),
        Content::Terms(_) => "terms".to_string(),
        Content::TermItem(ti) => {
            format!("/ {}: {}", repr_content(&ti.term), repr_content(&ti.description))
        }
        Content::Quote(q) => format!("quote[{}]", repr_content(&q.body)),
        Content::SmartQuote(_) => "\"".to_string(),
        Content::Underline(u) => format!("underline[{}]", repr_content(&u.body)),
        Content::Strike(s) => format!("strike[{}]", repr_content(&s.body)),
        Content::Overline(o) => format!("overline[{}]", repr_content(&o.body)),
        Content::SmallCaps { body } => format!("smallcaps[{}]", repr_content(body)),
        Content::Pad(p) => format!("pad[{}]", repr_content(&p.body)),
        Content::Hide(h) => format!("hide[{}]", repr_content(&h.body)),
        Content::HSpace(h) => {
            use crate::entities::elements::h_space::Spacing;
            let amount = match h.amount {
                Spacing::Absolute(length) => repr_length(&length),
                Spacing::Fractional(fr) => format_float_with_unit(fr, "fr"),
            };
            if h.weak_explicit {
                format!("h(amount: {amount}, weak: {})", h.weak)
            } else {
                format!("h(amount: {amount})")
            }
        }
        Content::VSpace(v) => {
            let amount = repr_length(&v.amount);
            if v.weak_explicit {
                format!("v(amount: {amount}, weak: {})", v.weak)
            } else {
                format!("v(amount: {amount})")
            }
        }
        Content::Pagebreak(p) => {
            let mut fields = Vec::new();
            if p.weak_explicit {
                fields.push(format!("weak: {}", p.weak));
            }
            if let Some(parity) = p.to {
                let parity = match parity {
                    crate::entities::parity::Parity::Even => "even",
                    crate::entities::parity::Parity::Odd => "odd",
                };
                fields.push(format!("to: \"{parity}\""));
            }
            format!("pagebreak({})", fields.join(", "))
        }
        Content::Colbreak(c) => {
            if c.weak_explicit {
                format!("colbreak(weak: {})", c.weak)
            } else {
                "colbreak()".to_string()
            }
        }
        Content::Stack(_) => "stack".to_string(),
        Content::Boxed(b) => format!("box[{}]", repr_content(&b.body)),
        Content::Block(b) => format!("block[{}]", repr_content(&b.body)),
        Content::TableCell(_) => "table.cell".to_string(),
        Content::Bibliography(b) => {
            let path = b.path.as_ref().map(|p| p.as_str()).unwrap_or("");
            format!("bibliography(\"{}\")", path)
        }
        Content::Cite(c) => format!("cite(<{}>)", c.key.as_str()),
        Content::Footnote(f) => format!("footnote[{}]", repr_content(&f.body)),
        Content::TableHeader(_) => "table.header".to_string(),
        Content::TableFooter(_) => "table.footer".to_string(),
        Content::TableHLine(_) => "table.hline".to_string(),
        Content::TableVLine(_) => "table.vline".to_string(),
        Content::Table(_) => "table".to_string(),
        Content::Repeat(_) => "repeat".to_string(),
        Content::Columns(_) => "columns".to_string(),
        Content::Metadata(_) => "metadata".to_string(),
        Content::State(_) => "state".to_string(),
        Content::StateUpdate(_) => "state.update".to_string(),
        Content::StateDisplay(_) => "state.display".to_string(),
        Content::CounterDisplayCallback(_) => "counter.display.callback".to_string(),
        Content::ContextBlock(_) => "context(...)".to_string(),
        Content::Dynamic(_) => "dynamic".to_string(),
        Content::SetPage { .. } => "setpage".to_string(),
        Content::Document { title, .. } => {
            if let Some(t) = title {
                format!("document[{}]", repr_content(t))
            } else {
                "document".to_string()
            }
        }
        Content::Asset { path, .. } => format!("asset(\"{}\")", path.as_str()),
        Content::PdfAttach(e) => format!("pdf.attach(\"{}\")", e.path),
        Content::PdfArtifact(e) => {
            if e.kind == crate::entities::elements::pdf_artifact::ArtifactKind::Other {
                format!("pdf.artifact[{}]", repr_content(&e.body))
            } else {
                format!(
                    "pdf.artifact(kind: \"{}\")[{}]",
                    e.kind.as_str(),
                    repr_content(&e.body)
                )
            }
        }
    }
}

/// Representação de um `Selector`.
pub fn repr_selector(sel: &Selector) -> String {
    match sel {
        Selector::Kind(kind) => kind.as_str().to_string(),
        Selector::Label(l) => format!("<{0}>", l.0),
        Selector::Location(_) => "location".to_string(),
        Selector::And(list) => {
            let parts: Vec<String> = list.iter().map(repr_selector).collect();
            format!("({})", parts.join(" & "))
        }
        Selector::Or(list) => {
            let parts: Vec<String> = list.iter().map(repr_selector).collect();
            format!("({})", parts.join(" | "))
        }
        Selector::Regex(r) => format!("regex(\"{}\")", r.pattern()),
        Selector::Where { base, field, value } => {
            format!(
                "{}.where({}: {})",
                repr_selector(base),
                field.as_str(),
                repr_value(value)
            )
        }
        Selector::Within { base, ancestor } => {
            format!("{}.within({})", repr_selector(base), repr_selector(ancestor))
        }
    }
}

/// Closures e qualquer aplicação parcial são anônimas na representação pública.
fn is_closure(f: &Func) -> bool {
    match f.repr() {
        FuncRepr::Closure(_) | FuncRepr::With(_) => true,
        _ => false,
    }
}

/// Float com decimal sempre visível (evita "1" para 1.0).
///
/// **P740E** — valores não-finitos seguem a forma literal do vanilla:
/// `float.nan` / `float.inf` / `-float.inf` (medido: `repr(calc.inf -
/// calc.inf)` → "float.nan", `repr(calc.inf)` → "float.inf"). Antes
/// produzia "NaN.0" / "inf.0".
fn repr_float(f: f64) -> String {
    if f.is_nan() {
        return "float.nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-float.inf" } else { "float.inf" }.to_string();
    }
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// Data/hora no formato nomeado do vanilla (`Datetime::repr`,
/// foundations/datetime.rs:501-515) — **P843 (F5)**. Só os componentes
/// presentes: data (`year`, `month`, `day`), hora (`hour`, `minute`,
/// `second`) ou ambos. Medido em `temp/p843/f5_datetime.typ` e
/// `f5_time_only.typ`. Antes: formato ISO (`datetime(2026-06-25)`), sem
/// paridade com o vanilla.
fn repr_datetime(d: &crate::entities::world_types::Datetime) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(6);
    if let Some(y) = d.year() {
        parts.push(format!("year: {y}"));
    }
    if let Some(m) = d.month() {
        parts.push(format!("month: {m}"));
    }
    if let Some(day) = d.day() {
        parts.push(format!("day: {day}"));
    }
    if let Some(h) = d.hour() {
        parts.push(format!("hour: {h}"));
    }
    if let Some(m) = d.minute() {
        parts.push(format!("minute: {m}"));
    }
    if let Some(s) = d.second() {
        parts.push(format!("second: {s}"));
    }
    format!("datetime{}", pretty_array_like(&parts, false))
}

// ── P843 — `pretty_comma_list` / `pretty_array_like` do vanilla ────────────
//
// Porte literal de `foundations/repr.rs:170-223`: horizontal se a soma dos
// comprimentos das peças + separadores couber em 50 colunas; caso contrário
// vertical (uma peça por linha, vírgula final sempre). `pretty_array_like`
// envolve em parênteses, indentando cada linha com 2 espaços no modo
// vertical.

/// Lista separada por vírgulas — paridade vanilla `pretty_comma_list`.
fn pretty_comma_list(pieces: &[String], trailing_comma: bool) -> String {
    const MAX_WIDTH: usize = 50;
    let mut buf = String::new();
    let len = pieces.iter().map(|s| s.len()).sum::<usize>()
        + 2 * pieces.len().saturating_sub(1);

    if !pieces.iter().any(|piece| piece.contains('\n')) && len <= MAX_WIDTH {
        for (i, piece) in pieces.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            buf.push_str(piece);
        }
        if trailing_comma {
            buf.push(',');
        }
    } else {
        for piece in pieces {
            buf.push_str(piece.trim());
            buf.push_str(",\n");
        }
    }
    buf
}

/// Construto tipo-array `(...)` — paridade vanilla `pretty_array_like`.
fn pretty_array_like(parts: &[String], trailing_comma: bool) -> String {
    let list = pretty_comma_list(parts, trailing_comma);
    let mut buf = String::new();
    buf.push('(');
    if list.contains('\n') {
        buf.push('\n');
        for (i, line) in list.lines().enumerate() {
            if i > 0 {
                buf.push('\n');
            }
            buf.push_str("  ");
            buf.push_str(line);
        }
        buf.push('\n');
    } else {
        buf.push_str(&list);
    }
    buf.push(')');
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::heading::HeadingElem;
    use crate::entities::label::Label;
    use crate::entities::selector::Selector;
    use crate::entities::value::Value;

    #[test]
    fn p1293_c_html_elem_usa_formatter_canonico_curto_e_multiline() {
        use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem};
        use std::sync::Arc;

        let mut short_attrs = HtmlAttrs::default();
        short_attrs.insert("span".into(), "2".into());
        short_attrs.insert("id".into(), "c".into());
        let short = Content::HtmlElem(Arc::new(HtmlElem::new(
            "col".into(),
            Some(short_attrs),
            HtmlBody::Unset,
        )));
        assert_eq!(
            repr_content(&short),
            "elem(tag: \"col\", attrs: (span: \"2\", id: \"c\"))"
        );

        let mut long_attrs = HtmlAttrs::default();
        long_attrs.insert("value".into(), "a rather long attribute value".into());
        long_attrs.insert("aria-label".into(), "another long value".into());
        let long = Content::HtmlElem(Arc::new(HtmlElem::new(
            "button".into(),
            Some(long_attrs),
            HtmlBody::Content(Box::new(Content::text("Body"))),
        )));
        assert_eq!(
            repr_content(&long),
            "elem(\n  tag: \"button\",\n  attrs: (\n    value: \"a rather long attribute value\",\n    aria-label: \"another long value\",\n  ),\n  body: [Body],\n)",
        );
    }
    use std::sync::Arc;

    #[test]
    fn repr_value_primitives() {
        assert_eq!(repr_value(&Value::Int(1)), "1");
        assert_eq!(repr_value(&Value::Float(1.0)), "1.0");
        assert_eq!(repr_value(&Value::Bool(true)), "true");
        assert_eq!(repr_value(&Value::None), "none");
        assert_eq!(repr_value(&Value::Auto), "auto");
        assert_eq!(repr_value(&Value::Str("hello".into())), "\"hello\"");
    }

    #[test]
    fn repr_value_array_and_dict() {
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(repr_value(&arr), "(1, 2)");

        let mut dict = indexmap::IndexMap::default();
        dict.insert("a".into(), Value::Int(1));
        assert_eq!(repr_value(&Value::Dict(dict)), "(a: 1)");
    }

    #[test]
    fn repr_value_empty_array_e_dict_distinguem() {
        // P695 — `()` (array vazio) e `(:)` (dict vazio) não podem colidir.
        let empty_arr = Value::Array(vec![]);
        let empty_dict = Value::Dict(indexmap::IndexMap::default());
        assert_eq!(repr_value(&empty_arr), "()");
        assert_eq!(repr_value(&empty_dict), "(:)");
        assert_ne!(repr_value(&empty_arr), repr_value(&empty_dict));
    }

    #[test]
    fn repr_value_array_um_elemento_virgula_final() {
        // P801 — array de exactamente 1 elemento leva vírgula final
        // (paridade vanilla `pretty_array_like(_, len == 1)`), distinguindo-o
        // de parênteses de agrupamento. Controlos: 0 e 2+ elementos sem
        // vírgula final.
        let single = Value::Array(vec![Value::Int(5)]);
        assert_eq!(repr_value(&single), "(5,)");
        let nested = Value::Array(vec![Value::Array(vec![Value::Int(5)])]);
        assert_eq!(repr_value(&nested), "((5,),)");
        let empty = Value::Array(vec![]);
        assert_eq!(repr_value(&empty), "()");
        let two = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(repr_value(&two), "(1, 2)");
    }

    #[test]
    fn repr_value_selector() {
        let sel = Value::Selector(Selector::Kind(
            crate::entities::element_kind::ElementKind::Heading,
        ));
        assert_eq!(repr_value(&sel), "heading");
    }

    #[test]
    fn repr_value_complex_types() {
        use crate::entities::{
            bytes::Bytes,
            decimal::Decimal,
            duration::Duration,
            gradient::Gradient,
            layout_types::{Angle, Color, Length, Ratio},
            location::Location,
            regex::Regex,
            version::Version,
        };

        let r = repr_value(&Value::Length(Length::pt(12.0)));
        assert_eq!(r, "12pt", "Length repr: {r}");
        let r = repr_value(&Value::Ratio(Ratio::from_percent(50.0)));
        assert_eq!(r, "50%", "Ratio repr: {r}");
        let r = repr_value(&Value::Angle(Angle::deg(90.0)));
        assert_eq!(r, "90deg", "Angle repr: {r}");
        let r = repr_value(&Value::Color(Color::rgb(255, 0, 0)));
        assert_eq!(r, "rgb(\"#ff0000\")", "Color repr: {r}");
        assert_eq!(repr_value(&Value::Fraction(0.5)), "0.5fr");
        assert_eq!(repr_value(&Value::Location(Location::from_raw(42))), "location(..)");
        assert_eq!(
            repr_value(&Value::Gradient(Gradient::Linear(std::sync::Arc::new(
                crate::entities::gradient::Linear {
                    angle: Angle::deg(0.0),
                    stops: std::sync::Arc::from([]),
                    space: crate::entities::layout_types::ColorSpace::Oklab,
                    relative: None,
                    anti_alias: true,
                }
            )))),
            "gradient.linear()"
        );
        assert_eq!(
            repr_value(&Value::Regex(Regex::new("\\d+").unwrap())),
            "regex(\"\\d+\")"
        );
        assert_eq!(
            repr_value(&Value::Tiling(std::sync::Arc::new(
                crate::entities::tiling::Tiling::new(
                    crate::entities::tiling::TilingBody::Color(Color::rgb(0, 0, 0))
                )
            ))),
            "tiling(...)"
        );
        assert_eq!(repr_value(&Value::Bytes(Bytes::from(vec![0u8, 1, 2]))), "bytes(3)");
        assert_eq!(
            repr_value(&Value::Decimal(Decimal::from_i64(123))),
            "decimal(\"123\")"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(1_000_000_000))),
            "duration(seconds: 1)"
        );
        assert_eq!(
            repr_value(&Value::Version(std::sync::Arc::new(Version::new(1, 2, 3)))),
            "version(1, 2, 3)"
        );
    }

    #[test]
    fn repr_value_float_with_decimal() {
        assert_eq!(repr_value(&Value::Float(1.5)), "1.5");
    }

    #[test]
    fn repr_value_module() {
        use crate::entities::{module::Module, scope::Scope};
        let m = Module::new("mylib", Scope::new());
        assert_eq!(repr_value(&Value::Module(m)), "<module mylib>");
    }

    #[test]
    fn repr_value_func_named() {
        // **P742** — paridade vanilla medida: `repr(rgb)` → `rgb` (sem `#`).
        let f =
            crate::entities::func::Func::native("repr", |_ctx, _args, _world, _cf| {
                Ok(Value::None)
            });
        assert_eq!(repr_value(&Value::Func(f)), "repr");
    }

    #[test]
    fn repr_value_func_unnamed() {
        let f = crate::entities::func::Func::native("", |_ctx, _args, _world, _cf| {
            Ok(Value::None)
        });
        assert_eq!(repr_value(&Value::Func(f)), "#function(...)");
    }

    #[test]
    fn p744_repr_closure() {
        use crate::entities::func::{ClosureParam, ClosureRepr};
        use crate::entities::scope::{Capturer, Scope};
        use crate::entities::source::Source;
        let source = Source::detached("x + 1");
        let body = source.root().clone();
        let f = crate::entities::func::Func::closure(ClosureRepr {
            name: Some("f".into()),
            params: vec![ClosureParam { name: "x".into(), default: None, pattern: None }],
            sink_name: None,
            body,
            captured: std::sync::Arc::new(Scope::new()),
            capturer: Capturer::Function,
        });
        assert_eq!(repr_value(&Value::Func(f)), "(..) => ..");
    }

    #[test]
    fn repr_value_version() {
        use crate::entities::version::Version;
        let v = Value::Version(std::sync::Arc::new(Version::new(0, 11, 0)));
        assert_eq!(repr_value(&v), "version(0, 11, 0)");
    }

    #[test]
    fn repr_value_bytes() {
        use crate::entities::bytes::Bytes;
        let b = Value::Bytes(Bytes::from(vec![0u8; 10]));
        assert_eq!(repr_value(&b), "bytes(10)");
    }

    #[test]
    fn repr_value_datetime_date_only() {
        use crate::entities::world_types::Datetime;
        let dt = Value::Datetime(Datetime::new_date(2026, 6, 25).unwrap());
        assert_eq!(repr_value(&dt), "datetime(year: 2026, month: 6, day: 25)");
    }

    #[test]
    fn repr_value_datetime_with_time() {
        use crate::entities::world_types::Datetime;
        let dt = Value::Datetime(Datetime::new_datetime(2026, 6, 25, 14, 30, 0).unwrap());
        // 62 colunas > 50 → modo vertical (paridade vanilla
        // `pretty_comma_list`), medido em `temp/p843/f5_datetime.typ`.
        assert_eq!(
            repr_value(&dt),
            "datetime(\n  year: 2026,\n  month: 6,\n  day: 25,\n  hour: 14,\n  minute: 30,\n  second: 0,\n)"
        );
    }

    #[test]
    fn repr_value_duration() {
        use crate::entities::duration::Duration;
        let d = Value::Duration(Duration::from_seconds(3661));
        assert_eq!(repr_value(&d), "duration(hours: 1, minutes: 1, seconds: 1)");
    }

    #[test]
    fn repr_value_regex() {
        use crate::entities::regex::Regex;
        let r = Value::Regex(Regex::new("a+").unwrap());
        assert_eq!(repr_value(&r), "regex(\"a+\")");
    }

    #[test]
    fn repr_value_relative_pure_percent() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let r = Value::Relative(Rel::<Length>::from_percent(50.0));
        assert_eq!(repr_value(&r), "50%");
    }

    #[test]
    fn repr_value_relative_with_abs_offset() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let r = Value::Relative(Rel::<Length>::from_percent(50.0) + Length::cm(2.0));
        // 2cm = 56.692pt → arredondado a 2 decimais (paridade vanilla).
        assert_eq!(repr_value(&r), "50% + 56.69pt");
    }

    // ── P721 — repr Typst (não Debug do Rust) ──────────────────────────────

    #[test]
    fn p721_repr_length_pt_em_e_combinado() {
        use crate::entities::layout_types::Length;
        assert_eq!(repr_value(&Value::Length(Length::pt(6.0))), "6pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(6.5))), "6.5pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(-3.0))), "-3pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(0.1))), "0.1pt");
        assert_eq!(repr_value(&Value::Length(Length::em(2.0))), "2em");
        assert_eq!(
            repr_value(&Value::Length(Length::pt(2.0) + Length::em(1.0))),
            "2pt + 1em"
        );
        assert_eq!(repr_value(&Value::Length(Length::ZERO)), "0pt");
    }

    #[test]
    fn p721_repr_ratio_arredonda_2_decimais() {
        use crate::entities::layout_types::Ratio;
        assert_eq!(repr_value(&Value::Ratio(Ratio::from_percent(33.333))), "33.33%");
        assert_eq!(repr_value(&Value::Ratio(Ratio::from_percent(0.5))), "0.5%");
    }

    #[test]
    fn p721_repr_angle_sempre_graus() {
        use crate::entities::layout_types::Angle;
        assert_eq!(repr_value(&Value::Angle(Angle::deg(45.0))), "45deg");
        assert_eq!(repr_value(&Value::Angle(Angle::deg(45.5))), "45.5deg");
        assert_eq!(repr_value(&Value::Angle(Angle::rad(1.0))), "57.3deg");
    }

    #[test]
    fn p721_repr_fraction_com_unidade_fr() {
        assert_eq!(repr_value(&Value::Fraction(1.0)), "1fr");
        assert_eq!(repr_value(&Value::Fraction(2.5)), "2.5fr");
    }

    #[test]
    fn p721_repr_color_srgb_hex_com_e_sem_alpha() {
        use crate::entities::layout_types::Color;
        assert_eq!(repr_value(&Value::Color(Color::rgb(255, 0, 0))), "rgb(\"#ff0000\")");
        assert_eq!(
            repr_value(&Value::Color(Color::rgba(255, 0, 0, 128))),
            "rgb(\"#ff000080\")"
        );
    }

    #[test]
    fn p721_repr_color_espacos_nao_srgb() {
        use crate::entities::layout_types::Color;
        assert_eq!(repr_value(&Value::Color(Color::luma(0.5))), "luma(50%)");
        assert_eq!(
            repr_value(&Value::Color(Color::linear_rgb(0.5, 0.5, 0.5, 1.0))),
            "color.linear-rgb(50%, 50%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::cmyk(0.0, 0.5, 1.0, 0.0))),
            "cmyk(0%, 50%, 100%, 0%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklab(0.5, 0.1, 0.1, 1.0))),
            "oklab(50%, 0.1, 0.1)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklch(0.5, 0.1, 30.0, 1.0))),
            "oklch(50%, 0.1, 30deg)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::hsl(0.0, 1.0, 0.5, 1.0))),
            "color.hsl(0deg, 100%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::hsv(120.0, 0.5, 0.8, 1.0))),
            "color.hsv(120deg, 50%, 80%)"
        );
    }

    #[test]
    fn p721_repr_color_alpha_omitido_quando_1() {
        use crate::entities::layout_types::Color;
        // Alpha != 1.0 aparece como argumento extra (paridade vanilla).
        assert_eq!(
            repr_value(&Value::Color(Color::Luma { l: 0.5, a: 0.5 })),
            "luma(50%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklab(0.5, 0.1, 0.1, 0.5))),
            "oklab(50%, 0.1, 0.1, 50%)"
        );
    }

    #[test]
    fn p721_repr_stroke_thickness_mais_paint() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        // `stroke(paint: red, thickness: 2pt)` — red Typst = #ff4136.
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(255, 65, 54)),
            thickness: 2.0,
            overhang: true,
            ..Stroke::default()
        };
        assert_eq!(repr_value(&Value::Stroke(s)), "2pt + rgb(\"#ff4136\")");
    }

    #[test]
    fn p1225_repr_stroke_preserva_default_nominal() {
        assert_eq!(repr_value(&Value::Stroke(Stroke::default())), "1pt + black");
    }

    #[test]
    fn p721_repr_align_horizontal_primeiro() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign};
        let left = Align2D { h: Some(HAlign::Left), v: None };
        assert_eq!(repr_value(&Value::Align(left)), "left");
        let ch = Align2D { h: Some(HAlign::Center), v: Some(VAlign::Horizon) };
        assert_eq!(repr_value(&Value::Align(ch)), "center + horizon");
        // `top + right` → "right + top" (horizontal primeiro, paridade vanilla).
        let tr = Align2D { h: Some(HAlign::Right), v: Some(VAlign::Top) };
        assert_eq!(repr_value(&Value::Align(tr)), "right + top");
        let start = Align2D { h: Some(HAlign::Start), v: None };
        assert_eq!(repr_value(&Value::Align(start)), "start");
    }

    #[test]
    fn repr_value_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem::new(1, Content::text("Title"))));
        assert_eq!(repr_value(&Value::Content(h)), "heading(level: 1)[[Title]]");
    }

    #[test]
    fn repr_value_content_label() {
        use crate::entities::elements::label::LabelElem;
        let l = Content::Label(Arc::new(LabelElem {
            name: "sec1".into(),
            body: Content::text("Section"),
            auto: false,
        }));
        assert_eq!(repr_content(&l), "label(\"sec1\", [Section])");
    }

    #[test]
    fn p11402_repr_value_label_literal_ou_constructor() {
        use crate::entities::label::Label;

        assert_eq!(repr_value(&Value::Label(Label("sec1".into()))), "<sec1>");
        assert_eq!(repr_value(&Value::Label(Label("a b".into()))), "label(\"a b\")");
    }

    #[test]
    fn repr_value_mixed_array() {
        let arr = Value::Array(vec![Value::Int(1), Value::Str("a".into()), Value::None]);
        assert_eq!(repr_value(&arr), "(1, \"a\", none)");
    }

    #[test]
    fn repr_content_text_and_sequence() {
        let seq =
            Content::Sequence(Arc::from(vec![Content::text("hello"), Content::Space]));
        assert_eq!(repr_content(&seq), "sequence([hello], [ ])");
    }

    #[test]
    fn p1140_9_repr_content_spacing_e_breaks_canonicos() {
        use crate::entities::layout_types::Abs;
        use crate::entities::parity::Parity;

        let combined = Length { abs: Abs::pt(2.0), em: 1.5 };
        assert_eq!(
            repr_content(&Content::h_space_with_weak_presence(combined, false, false,)),
            "h(amount: 2pt + 1.5em)"
        );
        assert_eq!(
            repr_content(&Content::h_space_fraction_with_weak_presence(
                1.25, false, true,
            )),
            "h(amount: 1.25fr, weak: false)"
        );
        assert_eq!(
            repr_content(&Content::v_space_with_weak_presence(
                Length::em(2.0),
                true,
                true,
            )),
            "v(amount: 2em, weak: true)"
        );
        assert_eq!(
            repr_content(&Content::pagebreak_with_weak_presence(
                false,
                false,
                Some(Parity::Odd),
            )),
            "pagebreak(to: \"odd\")"
        );
        assert_eq!(
            repr_content(&Content::pagebreak_with_weak_presence(
                false,
                true,
                Some(Parity::Even),
            )),
            "pagebreak(weak: false, to: \"even\")"
        );
        assert_eq!(
            repr_content(&Content::colbreak_with_weak_presence(false, false)),
            "colbreak()"
        );
    }

    #[test]
    fn repr_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem {
            level: 2,
            body: Content::text("Title"),
            outlined: true,
            bookmarked: None,
            set_fields: 0,
        }));
        assert_eq!(repr_content(&h), "heading(level: 2)[[Title]]");
    }

    #[test]
    fn repr_content_cite_and_bibliography() {
        let c = Content::Cite(Arc::new(crate::entities::elements::cite::CiteElem {
            key: "key".into(),
            form: None,
            supplement: None,
            style: None,
        }));
        assert_eq!(repr_content(&c), "cite(<key>)");
    }

    #[test]
    fn repr_selector_variants() {
        assert_eq!(
            repr_selector(&Selector::Kind(
                crate::entities::element_kind::ElementKind::Heading
            )),
            "heading"
        );
        assert_eq!(repr_selector(&Selector::Label(Label("l".to_string()))), "<l>");
        assert_eq!(
            repr_selector(&Selector::Where {
                base: Box::new(Selector::Kind(
                    crate::entities::element_kind::ElementKind::Heading
                )),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            }),
            "heading.where(level: 1)"
        );
    }

    // ── P843 (F1) — repr(duration): formato nomeado do vanilla ─────────────
    //
    // Medido no vanilla (`temp/p843/f1_duration.typ`,
    // `lab/typst-original` release, 2026-07-22): só os componentes não-zero,
    // por ordem weeks→seconds; sub-segundo truncado; zero → `duration()`.

    #[test]
    fn p843_f1_repr_duration_segundos_simples() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_seconds(3))),
            "duration(seconds: 3)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_composta_horas_minutos_segundos() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_seconds(3661))),
            "duration(hours: 1, minutes: 1, seconds: 1)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_dias_horas_minutos_segundos() {
        use crate::entities::duration::Duration;
        let d = Duration::from_nanos(
            Duration::from_days(1).nanos
                + Duration::from_hours(2).nanos
                + Duration::from_minutes(3).nanos
                + Duration::from_seconds(4).nanos,
        );
        assert_eq!(
            repr_value(&Value::Duration(d)),
            "duration(days: 1, hours: 2, minutes: 3, seconds: 4)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_minutos_normalizam_para_horas() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_minutes(90))),
            "duration(hours: 1, minutes: 30)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_semanas_e_dias() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_days(8))),
            "duration(weeks: 1, days: 1)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_zero_e_subsegundo() {
        use crate::entities::duration::Duration;
        assert_eq!(repr_value(&Value::Duration(Duration::ZERO)), "duration()");
        // Medido vanilla: `repr(duration(seconds: 1) / 3)` → "duration()"
        // (componentes inteiros; sub-segundo não aparece).
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(500_000_000))),
            "duration()"
        );
    }

    #[test]
    fn p850_repr_duration_negative() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(-Duration::from_seconds(3))),
            "duration(seconds: -3)"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(-2_000_000_000))),
            "duration(seconds: -2)"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(
                -86_400_000_000_000 - 3_600_000_000_000
            ))),
            "duration(days: -1, hours: -1)"
        );
    }

    // ── P843 (F2) — repr de content: formato estrutural do vanilla ─────────
    //
    // Medido no vanilla (`temp/p843/f2_content.typ`, `f2_nest.typ`,
    // `f2_misc.typ`): Text → `[texto]` (cru), Space → `[ ]`, vazio → `[]`,
    // sequência → `sequence(...)` com `pretty_array_like` (horizontal até 50
    // colunas, vertical com vírgula final acima disso), Strong/Emph →
    // `strong(body: ...)` / `emph(body: ...)`.

    #[test]
    fn p843_f2_repr_content_texto_cru_sem_aspas() {
        assert_eq!(repr_content(&Content::text("hi")), "[hi]");
    }

    #[test]
    fn p843_f2_repr_content_vazio_e_sequencia_vazia() {
        assert_eq!(repr_content(&Content::Empty), "[]");
        let seq = Content::Sequence(Arc::from(Vec::new()));
        assert_eq!(repr_content(&seq), "[]");
    }

    #[test]
    fn p843_f2_repr_content_sequencia_com_strong() {
        use crate::entities::elements::strong::StrongElem;
        let seq = Content::Sequence(Arc::from(vec![
            Content::text("hi"),
            Content::Space,
            Content::Strong(Arc::new(StrongElem::new(Content::text("bold")))),
        ]));
        assert_eq!(repr_content(&seq), "sequence([hi], [ ], strong(body: [bold]))");
    }

    #[test]
    fn p843_f2_repr_content_aninhamento_dois_niveis() {
        use crate::entities::elements::emph::EmphElem;
        use crate::entities::elements::strong::StrongElem;
        // strong(body: emph(body: [it])) — recursão no campo body.
        let c = Content::Strong(Arc::new(StrongElem::new(Content::Emph(Arc::new(
            EmphElem::new(Content::text("it")),
        )))));
        assert_eq!(repr_content(&c), "strong(body: emph(body: [it]))");
    }

    #[test]
    fn p843_f2_repr_content_sequencia_longa_quebra_vertical() {
        use crate::entities::elements::emph::EmphElem;
        use crate::entities::elements::strong::StrongElem;
        // 59 colunas > 50 → modo vertical (paridade vanilla
        // `pretty_comma_list`, MAX_WIDTH = 50): uma peça por linha,
        // indentação de 2 espaços, vírgula final.
        let seq = Content::Sequence(Arc::from(vec![
            Content::text("a"),
            Content::Space,
            Content::Strong(Arc::new(StrongElem::new(Content::Sequence(Arc::from(
                vec![
                    Content::text("b"),
                    Content::Space,
                    Content::Emph(Arc::new(EmphElem::new(Content::text("c")))),
                ],
            ))))),
        ]));
        let esperado = "sequence(\n  [a],\n  [ ],\n  strong(body: sequence([b], [ ], emph(body: [c]))),\n)";
        assert_eq!(repr_content(&seq), esperado);
    }

    // ── P843 (F3) — repr de type(none)/type(auto) ──────────────────────────
    //
    // Medido no vanilla (`temp/p843/f3_type_none_auto.typ`):
    // `repr(type(none))` → "type(none)", `repr(type(auto))` → "type(auto)";
    // os restantes tipos mantêm o nome curto (`repr(type(1))` → "int").

    #[test]
    fn p843_f3_repr_type_none_e_auto() {
        use crate::entities::value::Type;
        assert_eq!(repr_value(&Value::Type(Type::None)), "type(none)");
        assert_eq!(repr_value(&Value::Type(Type::Auto)), "type(auto)");
    }

    #[test]
    fn p843_f3_repr_type_restantes_nome_curto() {
        use crate::entities::value::Type;
        assert_eq!(repr_value(&Value::Type(Type::Int)), "int");
        assert_eq!(repr_value(&Value::Type(Type::Str)), "str");
        assert_eq!(repr_value(&Value::Type(Type::Bytes)), "bytes");
    }

    #[test]
    fn p1290_array_vazio_singleton_e_curto() {
        assert_eq!(repr_value(&Value::Array(vec![])), "()");
        assert_eq!(repr_value(&Value::Array(vec![Value::Int(0)])), "(0,)");
        assert_eq!(
            repr_value(&Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])),
            "(0, 1, 2)"
        );
    }

    #[test]
    fn p1290_array_fronteira_ascii_49_50_51() {
        let array_with_text = |count: usize| {
            repr_value(&Value::Array(vec![Value::Str("x".repeat(count).into())]))
        };

        assert_eq!(
            array_with_text(47),
            "(\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",)"
        );
        assert_eq!(
            array_with_text(48),
            "(\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",)"
        );
        assert_eq!(
            array_with_text(49),
            "(\n  \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",\n)"
        );
    }

    #[test]
    fn p1290_array_item_multiline_reindenta_recursivamente() {
        let inner = Value::Array(vec![Value::Str("x".repeat(49).into())]);
        let outer = Value::Array(vec![inner]);

        assert_eq!(
            repr_value(&outer),
            "(\n  (\n    \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",\n  ),\n)"
        );
    }

    #[test]
    fn p1290_math_op_estrutura_geral_preserva_limits_false_e_true() {
        let false_op = Content::math_op(Content::text("tensor-op"), false);
        let true_op = Content::math_op(Content::text("custom-limit"), true);

        assert_eq!(repr_content(&false_op), "op(text: [tensor-op], limits: false)");
        assert_eq!(repr_content(&true_op), "op(text: [custom-limit], limits: true)");
    }

    #[test]
    fn p1290_math_op_campos_em_ordem_e_conteudo_markup_multiline() {
        use crate::entities::elements::strong::StrongElem;

        let markup = Content::Sequence(Arc::from(vec![
            Content::Strong(Arc::new(StrongElem::new(Content::text("f")))),
            Content::Space,
            Content::text("oo"),
        ]));
        let op = Content::math_op(markup, false);

        assert_eq!(
            repr_content(&op),
            "op(\n  text: sequence(strong(body: [f]), [ ], [oo]),\n  limits: false,\n)"
        );
    }

    #[test]
    fn p1290_math_op_conteudo_preserva_aspas_e_barra_invertida() {
        let op = Content::math_op(Content::text(r#"a "quote" \ slash"#), false);

        assert_eq!(repr_content(&op), r#"op(text: [a "quote" \ slash], limits: false)"#);
    }

    #[test]
    fn p1290_math_op_conteudo_escapado_estrutural() {
        let escaped = Content::Sequence(Arc::from(vec![
            Content::text("a"),
            Content::Space,
            Content::text("#"),
            Content::Space,
            Content::text("*"),
        ]));
        let op = Content::math_op(escaped, false);

        assert_eq!(
            repr_content(&op),
            "op(\n  text: sequence([a], [ ], [#], [ ], [*]),\n  limits: false,\n)"
        );
    }

    #[test]
    fn p1292_c_v7_math_vec_projeta_folhas_math_diretas_como_content() {
        let vec = Content::math_vec(vec![
            Content::MathIdent("a".into()),
            Content::MathText("23".into()),
            Content::MathText("α".into()),
        ]);

        assert_eq!(repr_content(&vec), "vec(children: ([a], [23], [α]))");
    }

    #[test]
    fn p1292_c_v7_math_vec_nao_recursa_nem_duplica_markup() {
        let structured = Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathText("1".into()),
        );
        let vec = Content::math_vec(vec![
            structured,
            Content::strong(Content::text("b")),
            Content::text("quoted"),
        ]);

        assert_eq!(
            repr_content(&vec),
            "vec(children: ((a)/(\"1\"), strong(body: [b]), [quoted]))"
        );
    }

    #[test]
    fn p1292_d_flush_repr_is_the_zero_field_constructor() {
        assert_eq!(repr_content(&Content::flush()), "flush()");
    }

    #[test]
    fn p1293_b_attach_repr_preserva_slots_ordem_e_none_explicito() {
        let attach = Content::math_attach(
            Content::text("x"),
            MathAttachSlot::ExplicitNone,
            MathAttachSlot::Present(Content::text("b")),
            MathAttachSlot::Present(Content::text("tl")),
            MathAttachSlot::Present(Content::text("bl")),
            MathAttachSlot::Present(Content::text("tr")),
            MathAttachSlot::Present(Content::text("br")),
        );
        assert_eq!(
            repr_content(&attach),
            "attach(\n  base: [x],\n  t: none,\n  b: [b],\n  tl: [tl],\n  bl: [bl],\n  tr: [tr],\n  br: [br],\n)"
        );

        let omitted = Content::math_attach(
            Content::text("x"),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        assert_eq!(repr_content(&omitted), "attach(base: [x])");
    }

    #[test]
    fn p1293_b_binom_repr_preserva_singleton_e_lower_variadico() {
        let singleton = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![Content::text("k")])),
            ),
            ')',
        );
        assert_eq!(repr_content(&singleton), "binom(upper: [n], lower: ([k],))");

        let multiple = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![
                    Content::text("k"),
                    Content::MathText(", ".into()),
                    Content::text("j"),
                    Content::MathText(", ".into()),
                    Content::text("m"),
                ])),
            ),
            ')',
        );
        assert_eq!(repr_content(&multiple), "binom(upper: [n], lower: ([k], [j], [m]))");
    }

    #[test]
    fn p1293_b_independent_red_projeta_folhas_e_preserva_styled() {
        let attach = Content::math_attach(
            Content::MathIdent("x".into()),
            MathAttachSlot::Present(Content::MathIdent("T".into())),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        assert_eq!(repr_content(&attach), "attach(base: [x], t: [T])");

        let binom = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::MathIdent("n".into()),
                Content::MathSequence(Arc::from(vec![
                    Content::MathIdent("k".into()),
                    Content::MathText(", ".into()),
                    Content::MathIdent("j".into()),
                ])),
            ),
            ')',
        );
        assert_eq!(repr_content(&binom), "binom(upper: [n], lower: ([k], [j]))");

        let mono = Content::math_styled(
            Some(crate::entities::math_style::MathStyleKind::Monospace),
            None,
            None,
            Content::MathIdent("x".into()),
            None,
        );
        let script = Content::math_styled(
            Some(crate::entities::math_style::MathStyleKind::Script),
            None,
            None,
            Content::MathIdent("x".into()),
            Some(true),
        );
        assert_eq!(repr_content(&mono), "styled(child: [x], ..)");
        assert_eq!(repr_content(&script), "styled(child: [x], ..)");
    }

    #[test]
    fn p1293_b_binom_repr_rejeita_envelopes_nao_canonicos() {
        let lined = Content::math_delimited(
            '(',
            Content::math_frac(Content::text("n"), Content::text("k")),
            ')',
        );
        let wrong_delimiter = Content::math_delimited(
            '[',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![Content::text("k")])),
            ),
            ']',
        );
        let wrong_separator = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![
                    Content::text("k"),
                    Content::MathText(",".into()),
                    Content::text("j"),
                ])),
            ),
            ')',
        );

        for noncanonical in [lined, wrong_delimiter, wrong_separator] {
            assert!(
                !repr_content(&noncanonical).starts_with("binom("),
                "noncanonical envelope must keep the generic representation"
            );
        }
    }
}
