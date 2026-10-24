//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text/style.md
//! @prompt-hash 5e546e94
//! @layer L1
//! @updated 2026-10-24

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    layout_types::{Length, Pt, TextEdge, TextStyle},
    value::Value,
};

pub(super) fn effective_style<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> TextStyle {
    from_parts(&layouter.chain, &layouter.style)
}

pub(super) fn from_parts(
    chain: &crate::entities::style_chain::StyleChain,
    style: &TextStyle,
) -> TextStyle {
    resolve_parts(chain, style, true)
}

pub(super) fn script_request<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> TextStyle {
    let mut style = resolve_parts(&layouter.chain, &layouter.style, false);
    if layouter.superscript_source_break {
        style.superscript_native = Some(false);
    }
    style
}

pub(super) fn for_text<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
    text: &str,
) -> (TextStyle, Pt) {
    let mut style = resolve_parts(&layouter.chain, &layouter.style, false);
    let mut horizontal = Pt(0.0);
    if style.superscript && !style.subscript {
        let outer = style.size;
        let (size, x, y, native) =
            layouter.metrics.text_superscript_metrics(text, outer, &style);
        style.size = if native {
            size
        } else {
            style
                .superscript_size
                .map(|l| Pt(l.resolve_pt(outer.0)))
                .unwrap_or(size)
        };
        style.baseline_offset = Length::pt(y.0);
        style.superscript_native = Some(native);
        horizontal = x;
    }
    (style, horizontal)
}

fn resolve_parts(
    chain: &crate::entities::style_chain::StyleChain,
    style: &TextStyle,
    legacy_super: bool,
) -> TextStyle {
    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text`/
    // `#set par` não vem mais assado no node — vive na chain pelo canal
    // `custom` (`"text.<campo>"` / `"par.leading"`), levado até aqui pelo
    // transporte aninhado. Decodifica-o num node-render `ns` (Value→tipo)
    // e aplica a MESMA regra de merge top-wins de antes: a chain tipada
    // (`style` — heading, e Bold/Italic de `Content::Styled`) vence;
    // o `ns` cobre o resto.
    let cs = |k: &str| chain.custom(k);
    let ns_bold = matches!(cs("text.bold"), Some(Value::Bool(true)));
    let ns_italic = matches!(cs("text.italic"), Some(Value::Bool(true)));
    let ns_fill = match cs("text.fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        _ => None,
    };
    let ns_weight = match cs("text.weight") {
        Some(Value::Int(n)) => u16::try_from(*n).ok(),
        _ => None,
    };
    let ns_style_italic = matches!(cs("text.style"), Some(Value::Str(s)) if s.as_str() == "italic" || s.as_str() == "oblique");
    let ns_tracking = match cs("text.tracking") {
        Some(Value::Length(l)) => Some(*l),
        _ => None,
    };
    let ns_leading = match cs("par.leading") {
        Some(Value::Length(l)) => Some(*l),
        _ => None,
    };
    // P836 — `variations` não usa `custom()` (top-wins de 1 nível): o campo
    // é `#[fold]` no vanilla, logo resolve com fold por tag sobre TODOS os
    // níveis da chain (`StyleChain::variations()`).
    let ns_variations = chain.variations();
    // P837 — `top-edge`/`bottom-edge` aceitam métrica nomeada (`Value::Str`)
    // ou comprimento explícito (`Value::Length`), paridade dos casts
    // `TopEdge`/`BottomEdge` do vanilla (`text/mod.rs:1169-1225`).
    let ns_top_edge = match cs("text.top-edge") {
        Some(Value::Str(s)) => Some(TextEdge::Metric(s.clone())),
        Some(Value::Length(l)) => Some(TextEdge::Length(*l)),
        _ => None,
    };
    let ns_bottom_edge = match cs("text.bottom-edge") {
        Some(Value::Str(s)) => Some(TextEdge::Metric(s.clone())),
        Some(Value::Length(l)) => Some(TextEdge::Length(*l)),
        _ => None,
    };
    let ns_lang = match cs("text.lang") {
        Some(Value::Str(s)) => {
            use std::str::FromStr;
            crate::entities::lang::Lang::from_str(s).ok()
        }
        _ => None,
    };
    let ns_dir = match cs("text.dir") {
        Some(Value::Dir(dir)) => Some(*dir),
        _ => None,
    };
    let ns_font = match cs("text.font") {
        Some(Value::Array(arr)) => {
            use crate::entities::font_list::{FontFamily, FontNamePattern};
            use ecow::EcoString;

            let fams: Vec<_> = arr
                .iter()
                .filter_map(|v| {
                    match v {
                        // Forma P292/P373: string literal, variants vazio.
                        Value::Str(s) => Some(FontFamily::new(s.clone())),
                        // Forma P407/P414: dict com name (Str|Regex) + variants
                        // (Array[Str]) e campos named optionais variant/weight/style.
                        Value::Dict(dict) => {
                            let name = dict.get("name")?;
                            let pattern = match name {
                                Value::Str(s) => FontNamePattern::Literal(s.clone()),
                                Value::Regex(re) => FontNamePattern::Regex(re.clone()),
                                Value::None
                                | Value::Bool(..)
                                | Value::Int(..)
                                | Value::Float(..)
                                | Value::Array(..)
                                | Value::Dict(..)
                                | Value::Module(..)
                                | Value::Datetime(..)
                                | Value::Func(..)
                                | Value::Content(..)
                                | Value::LocatedContent(..)
                                | Value::Auto
                                | Value::Length(..)
                                | Value::Relative(..)
                                | Value::Ratio(..)
                                | Value::Angle(..)
                                | Value::Color(..)
                                | Value::Stroke(..)
                                | Value::Fraction(..)
                                | Value::Align(..)
                                | Value::Location(..)
                                | Value::Gradient(..)
                                | Value::Tiling(..)
                                | Value::Bytes(..)
                                | Value::Decimal(..)
                                | Value::Duration(..)
                                | Value::Version(..)
                                | Value::Selector(..)
                                | Value::Symbol(..)
                                | Value::Args(..)
                                | Value::State(..)
                                | Value::Counter(..)
                                | Value::Label(..)
                                | Value::Dir(..)
                                | Value::Path(..)
                                | Value::Type(..)
                                | Value::SpotColorant(..) => return None,
                            };
                            let variants: Vec<EcoString> = match dict.get("variants") {
                                Some(Value::Array(arr)) => arr
                                    .iter()
                                    .filter_map(|v| match v {
                                        Value::Str(s) => Some(s.clone()),
                                        _ => None,
                                    })
                                    .collect(),
                                _ => return None,
                            };
                            let variant = dict.get("variant").and_then(|v| {
                                if let Value::Str(s) = v {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            });
                            let weight = dict.get("weight").and_then(|v| {
                                if let Value::Str(s) = v {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            });
                            let style = dict.get("style").and_then(|v| {
                                if let Value::Str(s) = v {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            });
                            Some(FontFamily {
                                name: pattern,
                                variants,
                                variant,
                                weight,
                                style,
                                covers: None,
                            })
                        }
                        Value::None
                        | Value::Bool(..)
                        | Value::Int(..)
                        | Value::Float(..)
                        | Value::Array(..)
                        | Value::Module(..)
                        | Value::Datetime(..)
                        | Value::Func(..)
                        | Value::Content(..)
                        | Value::LocatedContent(..)
                        | Value::Auto
                        | Value::Length(..)
                        | Value::Relative(..)
                        | Value::Ratio(..)
                        | Value::Angle(..)
                        | Value::Color(..)
                        | Value::Stroke(..)
                        | Value::Fraction(..)
                        | Value::Align(..)
                        | Value::Location(..)
                        | Value::Gradient(..)
                        | Value::Regex(..)
                        | Value::Tiling(..)
                        | Value::Bytes(..)
                        | Value::Decimal(..)
                        | Value::Duration(..)
                        | Value::Version(..)
                        | Value::Selector(..)
                        | Value::Symbol(..)
                        | Value::Args(..)
                        | Value::State(..)
                        | Value::Counter(..)
                        | Value::Label(..)
                        | Value::Dir(..)
                        | Value::Path(..)
                        | Value::Type(..)
                        | Value::SpotColorant(..) => None,
                    }
                })
                .collect();
            crate::entities::font_list::FontList::new(fams)
        }
        _ => None,
    };
    let typed_font_is_nearest = chain
        .raw_deltas()
        .find_map(|delta| {
            if delta.font.is_some() {
                Some(true)
            } else if delta.custom.iter().any(|(key, _)| key.as_str() == "text.font") {
                Some(false)
            } else {
                None
            }
        })
        .unwrap_or(false);
    // **P978** — `size`: a regra documentada do merge ("a chain tipada
    // vence") estava invertida — `ns_size.unwrap_or(...)` deixava o
    // `#set text(size:)` sobrepor-se a tamanhos deliberados do layout
    // (escala de heading, super/subscrito). `style.size` já
    // inclui o `#set` via sync da chain (verificado por instrumentação),
    // logo vence sempre — `heading.md` §P978.
    let mut effective = TextStyle {
        bold: ns_bold || style.bold,
        italic: ns_italic || ns_style_italic || style.italic,
        semantic_strong: style.semantic_strong,
        semantic_emph: style.semantic_emph,
        size: style.size,
        fill: style.fill.clone().or(ns_fill),
        heading_level: style.heading_level,
        // Top-wins: chain tipada (heading) vence; senão o ns (#set).
        weight: style.weight.or(ns_weight),
        tracking: style.tracking.or(ns_tracking),
        leading: style.leading.or(ns_leading),
        top_edge: style.top_edge.clone().or(ns_top_edge),
        bottom_edge: style.bottom_edge.clone().or(ns_bottom_edge),
        lang: style.lang.or(ns_lang),
        // Respect the nearest declaration across typed and custom channels.
        font: if typed_font_is_nearest {
            style.font.clone()
        } else {
            ns_font.or(style.font.clone())
        },
        dir: style.dir.or(ns_dir),
        subscript: style.subscript,
        superscript: style.superscript,
        superscript_native: style.superscript_native,
        highlight: style.highlight.clone(),
        highlight_radius: style.highlight_radius,
        highlight_extent: style.highlight_extent,
        subscript_size: style.subscript_size,
        superscript_size: style.superscript_size,
        baseline_offset: style.baseline_offset,
        // P784 — herda do style corrente (regular ou math); este merge não
        // é math-específico, só reflecte o valor já activo no layouter.
        math: style.math,
        // P1293 — preserva a proveniência já escolhida pelo layouter math;
        // este merge textual não sintetiza nem apaga `TextItem`.
        math_text_item: style.math_text_item,
        // P891 — mesmo motivo: herda o valor já activo, este merge não é
        // script-específico.
        math_script: style.math_script,
        // P915 — mesmo motivo de math_script: herda o valor já activo,
        // este merge não é cramped-específico.
        cramped: style.cramped,
        // P945 — mesmo motivo de math_script/cramped: herda o nível já
        // activo, este merge não é math-size-específico.
        math_size: style.math_size,
        // P836 — eixos explícitos: o herdado (`style`, ex. vindo de
        // `Content::Styled` do constructor) vence; senão o fold da chain.
        variations: style.variations.clone().or(ns_variations),
    };

    // **P448/P471**: subscrito/sobrescrito reduzem o corpo e deslocam a baseline.
    // P471: `size` explícito substitui a escala padrão de 65%.
    const SCRIPT_SCALE: f64 = 0.65;
    if effective.subscript {
        effective.size = effective
            .subscript_size
            .map(|l| Pt(l.resolve_pt(effective.size.val())))
            .unwrap_or_else(|| Pt(effective.size.0 * SCRIPT_SCALE));
        effective.baseline_offset = Length::em(-0.2);
    } else if effective.superscript && legacy_super {
        effective.size = effective
            .superscript_size
            .map(|l| Pt(l.resolve_pt(effective.size.val())))
            .unwrap_or_else(|| Pt(effective.size.0 * SCRIPT_SCALE));
        effective.baseline_offset = Length::em(0.3);
    }

    effective
}

#[cfg(test)]
mod v16_d_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        font_list::FontList,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        regex::Regex,
    };
    use comemo::Track;

    fn family(name: Value) -> Value {
        Value::Dict(
            [
                ("name".into(), name),
                (
                    "variants".into(),
                    Value::Array(vec![
                        Value::Str("Regular".into()),
                        Value::Str("Bold".into()),
                    ]),
                ),
                ("variant".into(), Value::Str("Book".into())),
                ("weight".into(), Value::Str("450".into())),
                ("style".into(), Value::Str("oblique".into())),
            ]
            .into_iter()
            .collect(),
        )
    }

    #[test]
    fn remaining_raw_typed_font_scope_precedes_outer_custom_font() {
        use crate::entities::style_chain::{StyleChain, StyleDelta};
        let outer = StyleChain::default_chain().push_custom(
            "text.font",
            Value::Array(vec![Value::Str("DejaVu Sans".into())]),
        );
        let mono = FontList::single("DejaVu Sans Mono".into());
        let mut delta = StyleDelta::empty();
        delta.font = Some(mono.clone());
        let chain = outer.push(delta);
        let active = TextStyle::from(&chain);
        assert_eq!(
            active.font,
            Some(mono.clone()),
            "actual typed chain chooses Raw scope"
        );
        let got = from_parts(&chain, &active);
        assert_eq!(
            got.font,
            Some(mono),
            "effective generic Text must retain nearest Raw font"
        );
        assert_eq!(got.size, active.size);
        assert_eq!(got.fill, active.fill);
        assert_eq!(got.bold, active.bold);
        assert_eq!(got.italic, active.italic);
    }

    #[test]
    fn remaining_raw_inner_custom_font_keeps_rich_schema_and_invalid_fallback() {
        use crate::entities::style_chain::{StyleChain, StyleDelta};
        let mono = FontList::single("DejaVu Sans Mono".into());
        let mut delta = StyleDelta::empty();
        delta.font = Some(mono.clone());
        let outer = StyleChain::default_chain().push(delta);
        let chain = outer.push_custom(
            "text.font",
            Value::Array(vec![family(Value::Str("inner".into()))]),
        );
        let got = from_parts(&chain, &TextStyle::from(&chain));
        let list = got.font.unwrap();
        let chosen = &list.as_slice()[0];
        assert_eq!(chosen.name.as_str(), Some("inner"));
        assert_eq!(chosen.variants, vec!["Regular", "Bold"]);
        assert_eq!(chosen.variant.as_deref(), Some("Book"));
        assert_eq!(chosen.weight.as_deref(), Some("450"));
        assert_eq!(chosen.style.as_deref(), Some("oblique"));
        let invalid = outer.push_custom("text.font", Value::Array(vec![Value::Int(17)]));
        let active = TextStyle::from(&invalid);
        let fallback = active.font.clone();
        assert_eq!(from_parts(&invalid, &active).font, fallback);
    }

    #[test]
    fn v16_d_font_schema_preserves_names_order_fields_and_typed_style() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.style.bold = true;
        l.style.math = true;
        l.style.weight = Some(700);
        l.chain = l.chain.push_custom(
            "text.font",
            Value::Array(vec![
                Value::Int(17),
                Value::Str("LITERAL".into()),
                family(Value::Bool(false)),
                family(Value::Str("second".into())),
                family(Value::Regex(Regex::new("third.*").unwrap())),
            ]),
        );
        let got = effective_style(&l);
        let fams = got.font.as_ref().unwrap().as_slice();
        assert_eq!(fams.len(), 3);
        assert_eq!(fams[0].name.as_str(), Some("literal"));
        assert_eq!(fams[1].name.as_str(), Some("second"));
        assert!(fams[2].name.is_match("third family"));
        for f in &fams[1..] {
            assert_eq!(f.variants, vec!["Regular", "Bold"]);
            assert_eq!(f.variant.as_deref(), Some("Book"));
            assert_eq!(f.weight.as_deref(), Some("450"));
            assert_eq!(f.style.as_deref(), Some("oblique"));
        }
        assert!(got.bold && got.math);
        assert_eq!(got.weight, Some(700));
        assert_eq!(got.size, l.style.size);
    }

    #[test]
    fn v16_d_invalid_raw_font_entries_preserve_ambient_fallback() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.style.font = Some(FontList::single("ambient".into()));
        for entry in [
            Value::None,
            Value::Content(crate::entities::content::Content::text("not a family")),
            Value::Regex(Regex::new(".*").unwrap()),
            family(Value::Int(1)),
            Value::Dict(
                [("name".into(), Value::Str("missing variants".into()))]
                    .into_iter()
                    .collect(),
            ),
        ] {
            l.chain = l.chain.push_custom("text.font", Value::Array(vec![entry]));
            assert_eq!(effective_style(&l).font, l.style.font);
        }
    }
}
