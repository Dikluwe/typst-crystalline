//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/foundations.md
//! @layer L1
//! @updated 2026-06-23
//!
//! P421 — `repr()` exaustivo para `Value`, `Content` e `Selector`.
//!
//! Implementação como free functions (ADR-0109 forma B). A representação é
//! **reconhecível**, não garante round-trip. Variants complexos usam
//! representação scope-out ("function", "module", nome do tipo).


use crate::entities::layout_types::Length;
use crate::entities::content::Content;
use crate::entities::rel::Rel;
use crate::entities::selector::Selector;
use crate::entities::value::Value;

/// Representação de um `Value`.
pub fn repr_value(v: &Value) -> String {
    match v {
        Value::None => "none".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => repr_float(*f),
        Value::Str(s) => format!("\"{}\"", s.as_str().escape_debug()),
        Value::Content(c) => repr_content(c),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(repr_value).collect();
            format!("({})", items.join(", "))
        }
        Value::Dict(dict) => {
            let items: Vec<String> = dict
                .iter()
                .map(|(k, v)| format!("{}: {}", k.as_str(), repr_value(v)))
                .collect();
            format!("({})", items.join(", "))
        }
        Value::Module(m) => format!("module({})", m.name()),
        Value::Datetime(d) => repr_datetime(d),
        Value::Func(f) => {
            if let Some(name) = f.name() {
                if name.is_empty() {
                    "#function(...)".to_string()
                } else {
                    format!("#{name}")
                }
            } else {
                "#function(...)".to_string()
            }
        }
        Value::Auto => "auto".to_string(),
        Value::Length(l) => format!("{:?}", l),
        Value::Relative(rel) => repr_relative(rel),
        Value::Ratio(r) => format!("{:?}", r),
        Value::Angle(a) => format!("{:?}", a),
        Value::Color(c) => format!("{:?}", c),
        Value::Stroke(s) => format!("{:?}", s),
        Value::Fraction(f) => repr_float(*f),
        Value::Align(a) => format!("{:?}", a),
        Value::Location(_) => "location(...)".to_string(),
        Value::Gradient(_) => "gradient(...)".to_string(),
        Value::Regex(r) => format!("regex(\"{}\")", r.pattern()),
        Value::Tiling(_) => "tiling(...)".to_string(),
        Value::Bytes(b) => format!("bytes({})", b.len()),
        Value::Decimal(d) => d.to_string(),
        Value::Duration(d) => format!("duration({})", d.to_string()),
        Value::Version(ver) => {
            format!("version({}, {}, {})", ver.major, ver.minor, ver.patch)
        }
        Value::Selector(s) => repr_selector(s),
    }
}

/// Representação de um comprimento relativo (`Rel<Length>`).
///
/// Reconhecível, mas não round-trip: usa `{:?}` para a componente absoluta.
fn repr_relative(rel: &Rel<Length>) -> String {
    let pct = rel.rel * 100.0;
    if rel.abs.is_zero() {
        format!("{}%", pct)
    } else {
        format!("{}% + {:?}", pct, rel.abs)
    }
}

/// Representação de um `Content`.
pub fn repr_content(c: &Content) -> String {
    match c {
        Content::Empty => "empty".to_string(),
        Content::Text(t) => format!("\"{}\"", t.as_str().escape_debug()),
        Content::Space => "space".to_string(),
        Content::Sequence(seq) => {
            let inner: String = seq.iter().map(repr_content).collect();
            format!("[{}]", inner)
        }
        Content::Heading(h) => {
            format!("heading(level: {})[{}]", h.level, repr_content(&h.body))
        }
        Content::Strong(s) => format!("*{}*", repr_content(&s.body)),
        Content::Emph(e) => format!("_{}_", repr_content(&e.body)),
        Content::Raw(r) => format!("`{}`", r.text),
        Content::ListItem(li) => format!("- {}", repr_content(&li.body)),
        Content::EnumItem(ei) => format!("+ {}", repr_content(&ei.body)),
        Content::Link(l) => format!("link(\"{}\")", l.url),
        Content::Equation(e) => {
            let delim = if e.block { "$$" } else { "$" };
            format!("{}{}{}", delim, repr_content(&e.body), delim)
        }
        Content::MathSequence(seq) => seq.iter().map(repr_content).collect(),
        Content::MathIdent(s) => s.to_string(),
        Content::MathText(s) => format!("\"{}\"", s.as_str()),
        Content::MathFrac(f) => {
            format!("({})/({})", repr_content(&f.num), repr_content(&f.den))
        }
        Content::MathAttach(a) => {
            let mut out = repr_content(&a.base);
            if let Some(tl) = &a.tl {
                out.push_str(&format!("^({})", repr_content(tl)));
            }
            if let Some(bl) = &a.bl {
                out.push_str(&format!("_({})", repr_content(bl)));
            }
            if let Some(sub) = &a.sub {
                out.push_str(&format!("_({})", repr_content(sub)));
            }
            if let Some(sup) = &a.sup {
                out.push_str(&format!("^({})", repr_content(sup)));
            }
            out
        }
        Content::MathRoot(r) => {
            if let Some(idx) = &r.index {
                format!("root({}, {})", repr_content(idx), repr_content(&r.radicand))
            } else {
                format!("sqrt({})", repr_content(&r.radicand))
            }
        }
        Content::MathDelimited(d) => {
            format!("{}{}{}", d.open, repr_content(&d.body), d.close)
        }
        Content::MathAlignPoint(_) => "&".to_string(),
        Content::Linebreak(_) => "linebreak".to_string(),
        Content::MathMatrix(_) => "matrix".to_string(),
        Content::MathCases(_) => "cases".to_string(),
        Content::MathAccent(a) => format!(
            "accent({}, {})",
            repr_content(&a.base),
            repr_content(&a.accent)
        ),
        Content::MathCancel(c) => format!("cancel({})", repr_content(&c.body)),
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
        Content::MathOp(o) => format!("op({})", repr_content(&o.text)),
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
        Content::Transform(t) => format!("transform[{}]", repr_content(&t.body)),
        Content::Grid(_) => "grid".to_string(),
        Content::GridHeader(_) => "grid.header".to_string(),
        Content::GridFooter(_) => "grid.footer".to_string(),
        Content::GridCell(_) => "grid.cell".to_string(),
        Content::Align(a) => format!("align({:?})[...]", a.alignment),
        Content::Place(p) => format!("place({:?})[...]", p.scope),
        Content::Styled(child, _styles) => repr_content(child),
        Content::Divider(_) => "divider".to_string(),
        Content::Terms(_) => "terms".to_string(),
        Content::TermItem(ti) => {
            format!(
                "/ {}: {}",
                repr_content(&ti.term),
                repr_content(&ti.description)
            )
        }
        Content::Quote(q) => format!("quote[{}]", repr_content(&q.body)),
        Content::SmartQuote(_) => "\"".to_string(),
        Content::Underline(u) => format!("underline[{}]", repr_content(&u.body)),
        Content::Strike(s) => format!("strike[{}]", repr_content(&s.body)),
        Content::Overline(o) => format!("overline[{}]", repr_content(&o.body)),
        Content::SmallCaps { body } => format!("smallcaps[{}]", repr_content(body)),
        Content::Pad(p) => format!("pad[{}]", repr_content(&p.body)),
        Content::Hide(h) => format!("hide[{}]", repr_content(&h.body)),
        Content::HSpace(_) => "hspace".to_string(),
        Content::VSpace(_) => "vspace".to_string(),
        Content::Pagebreak(_) => "pagebreak".to_string(),
        Content::Colbreak(_) => "colbreak".to_string(),
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
        Content::Table(_) => "table".to_string(),
        Content::Repeat(_) => "repeat".to_string(),
        Content::Columns(_) => "columns".to_string(),
        Content::Metadata(_) => "metadata".to_string(),
        Content::State(_) => "state".to_string(),
        Content::StateUpdate(_) => "state.update".to_string(),
        Content::StateDisplay(_) => "state.display".to_string(),
        Content::CounterDisplayCallback(_) => "counter.display.callback".to_string(),
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
            format!("{}.where({}: {})", repr_selector(base), field.as_str(), repr_value(value))
        }
    }
}

/// Float com decimal sempre visível (evita "1" para 1.0).
fn repr_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// Data/hora no formato reconhecível pelo Typst.
fn repr_datetime(d: &crate::entities::world_types::Datetime) -> String {
    let date = format!("{:04}-{:02}-{:02}", d.year(), d.month(), d.day());
    match (d.hour(), d.minute(), d.second()) {
        (Some(h), Some(m), Some(s)) => {
            format!("datetime({}T{:02}:{:02}:{:02})", date, h, m, s)
        }
        _ => format!("datetime({})", date),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::heading::HeadingElem;
    use crate::entities::label::Label;
    use crate::entities::selector::Selector;
    use crate::entities::value::Value;
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
    fn repr_value_selector() {
        let sel = Value::Selector(Selector::Kind(crate::entities::element_kind::ElementKind::Heading));
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
        assert!(r.contains("12") || r.contains("Length"), "Length repr: {r}");
        let r = repr_value(&Value::Ratio(Ratio::from_percent(50.0)));
        assert!(r.contains("0.5") || r.contains("Ratio"), "Ratio repr: {r}");
        let r = repr_value(&Value::Angle(Angle::deg(90.0)));
        assert!(r.contains("90") || r.contains("Angle"), "Angle repr: {r}");
        let r = repr_value(&Value::Color(Color::rgb(255, 0, 0)));
        assert!(r.contains("Srgb") || r.contains("rgb"), "Color repr: {r}");
        assert_eq!(repr_value(&Value::Fraction(0.5)), "0.5");
        assert_eq!(
            repr_value(&Value::Location(Location::from_raw(42))),
            "location(...)"
        );
        assert_eq!(repr_value(&Value::Gradient(Gradient::Linear(std::sync::Arc::new(
            crate::entities::gradient::Linear {
                angle: Angle::deg(0.0),
                stops: std::sync::Arc::from([]),
                space: crate::entities::layout_types::ColorSpace::Oklab,
                relative: None,
            }
        )))), "gradient(...)");
        assert_eq!(
            repr_value(&Value::Regex(Regex::new("\\d+").unwrap())),
            "regex(\"\\d+\")"
        );
        assert_eq!(repr_value(&Value::Tiling(std::sync::Arc::new(
            crate::entities::tiling::Tiling::new(
                crate::entities::tiling::TilingBody::Color(Color::rgb(0, 0, 0))
            )
        ))), "tiling(...)");
        assert_eq!(
            repr_value(&Value::Bytes(Bytes::from(vec![0u8, 1, 2]))),
            "bytes(3)"
        );
        assert_eq!(repr_value(&Value::Decimal(Decimal::from_i64(123))), "123");
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(1_000_000_000))),
            "duration(1s)"
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
        assert_eq!(repr_value(&Value::Module(m)), "module(mylib)");
    }

    #[test]
    fn repr_value_func_named() {
        let f = crate::entities::func::Func::native("repr", |_ctx, _args, _world, _cf| {
            Ok(Value::None)
        });
        assert_eq!(repr_value(&Value::Func(f)), "#repr");
    }

    #[test]
    fn repr_value_func_unnamed() {
        let f = crate::entities::func::Func::native("", |_ctx, _args, _world, _cf| {
            Ok(Value::None)
        });
        assert_eq!(repr_value(&Value::Func(f)), "#function(...)");
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
        assert_eq!(repr_value(&dt), "datetime(2026-06-25)");
    }

    #[test]
    fn repr_value_datetime_with_time() {
        use crate::entities::world_types::Datetime;
        let dt =
            Value::Datetime(Datetime::new_datetime(2026, 6, 25, 14, 30, 0).unwrap());
        assert_eq!(repr_value(&dt), "datetime(2026-06-25T14:30:00)");
    }

    #[test]
    fn repr_value_duration() {
        use crate::entities::duration::Duration;
        let d = Value::Duration(Duration::from_seconds(3661));
        assert_eq!(repr_value(&d), "duration(1h1m1s)");
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
        let s = repr_value(&r);
        assert!(s.starts_with("50% + "), "repr relative with offset: {s}");
    }

    #[test]
    fn repr_value_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem::new(1, Content::text("Title"))));
        assert_eq!(
            repr_value(&Value::Content(h)),
            "heading(level: 1)[\"Title\"]"
        );
    }

    #[test]
    fn repr_value_content_label() {
        use crate::entities::elements::label::LabelElem;
        let l = Content::Label(Arc::new(LabelElem {
            name: "sec1".into(),
            body: Content::text("Section"),
            auto: false,
        }));
        assert_eq!(
            repr_content(&l),
            "label(\"sec1\", \"Section\")"
        );
    }

    #[test]
    fn repr_value_mixed_array() {
        let arr = Value::Array(vec![
            Value::Int(1),
            Value::Str("a".into()),
            Value::None,
        ]);
        assert_eq!(repr_value(&arr), "(1, \"a\", none)");
    }

    #[test]
    fn repr_content_text_and_sequence() {
        let seq = Content::Sequence(Arc::from(vec![Content::text("hello"), Content::Space]));
        assert_eq!(repr_content(&seq), "[\"hello\"space]");
    }

    #[test]
    fn repr_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem {
            level: 2,
            body: Content::text("Title"),
        }));
        assert_eq!(repr_content(&h), "heading(level: 2)[\"Title\"]");
    }

    #[test]
    fn repr_content_cite_and_bibliography() {
        let c = Content::Cite(Arc::new(crate::entities::elements::cite::CiteElem {
            key: "key".into(),
            form: None,
            supplement: None,
        }));
        assert_eq!(repr_content(&c), "cite(<key>)");
    }

    #[test]
    fn repr_selector_variants() {
        assert_eq!(
            repr_selector(&Selector::Kind(crate::entities::element_kind::ElementKind::Heading)),
            "heading"
        );
        assert_eq!(repr_selector(&Selector::Label(Label("l".to_string()))), "<l>");
        assert_eq!(
            repr_selector(&Selector::Where {
                base: Box::new(Selector::Kind(crate::entities::element_kind::ElementKind::Heading)),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            }),
            "heading.where(level: 1)"
        );
    }
}
