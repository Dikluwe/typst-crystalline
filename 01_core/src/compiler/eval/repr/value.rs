//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/repr/value.md
//! @prompt-hash 716cc8bc
//! @layer L1
//! @updated 2026-09-30

use super::*;

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
            pretty_array_like(&items, false)
        }
        Value::Module(m) => format!("<module {}>", m.name()),
        Value::Datetime(d) => repr_datetime(d),
        Value::Func(f) => {
            // **P744** — paridade vanilla medida: closures (anónimas ou
            // nomeadas) repr como `"(..) => .."`; nativas usam o nome
            // (`repr(rgb)` → `"rgb"`, P742); função sem nome mantém
            // `"#function(...)"`.
            if is_closure(f) {
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
        // **P1473** — `color.spot(name, fallback)`: `name.repr()` segue a
        // convenção já usada acima para `Value::Str` (`none` quando ausente,
        // string entre aspas quando presente).
        Value::SpotColorant(sc) => {
            let name = match &sc.name {
                Some(n) => format!("\"{}\"", n.as_str().escape_debug()),
                None => "none".to_string(),
            };
            format!("color.spot({name}, {})", repr_color(&sc.fallback))
        }
        Value::Fraction(f) => format_float_with_unit(*f, "fr"),
        Value::Align(a) => repr_align(a),
        Value::Location(_) => "location(..)".to_string(),
        Value::Gradient(gradient) => repr_gradient(gradient),
        Value::Regex(r) => format!("regex(\"{}\")", r.pattern()),
        Value::Tiling(tiling) => repr_tiling(tiling),
        Value::Bytes(b) => format!("bytes({})", b.len()),
        // P817-D — paridade vanilla: repr de decimal é `decimal("...")`.
        Value::Decimal(d) => format!("decimal(\"{}\")", d),
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
/// Closures e qualquer aplicação parcial são anônimas na representação pública.
pub(super) fn is_closure(f: &Func) -> bool {
    matches!(f.repr(), FuncRepr::Closure(_) | FuncRepr::With(_))
}

/// Float com decimal sempre visível (evita "1" para 1.0).
///
/// **P740E** — valores não-finitos seguem a forma literal do vanilla:
/// `float.nan` / `float.inf` / `-float.inf` (medido: `repr(calc.inf -
/// calc.inf)` → "float.nan", `repr(calc.inf)` → "float.inf"). Antes
/// produzia "NaN.0" / "inf.0".
pub(super) fn repr_float(f: f64) -> String {
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
pub(super) fn repr_datetime(d: &crate::entities::world_types::Datetime) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_array_and_empty_dictionary_have_distinct_morphology() {
        assert_eq!(repr_value(&Value::Array(vec![Value::Int(5)])), "(5,)");
        assert_eq!(repr_value(&Value::Dict(Default::default())), "(:)");
    }
    #[test]
    fn dictionary_repr_uses_the_shared_multiline_layout() {
        let values = [
            ("angle", Value::Angle(Angle::deg(23.0))),
            (
                "origin",
                Value::Align(crate::entities::layout_types::Align2D::from_string(
                    "top-left",
                )),
            ),
            ("reflow", Value::Bool(false)),
            ("body", Value::Content(Content::text("X"))),
        ];
        let dict = values.into_iter().map(|(k, v)| (k.into(), v)).collect();
        assert_eq!(repr_value(&Value::Dict(dict)), "(\n  angle: 23deg,\n  origin: left + top,\n  reflow: false,\n  body: [X],\n)");
    }
}
