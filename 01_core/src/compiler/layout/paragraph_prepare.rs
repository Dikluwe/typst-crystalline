//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_prepare.md
//! @prompt-hash 1fae5196
//! @layer L1
//! @updated 2026-10-08

use super::{text_measure, FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    content::Content,
    dir::Dir,
    layout_types::{TextSeparatorOrigin, TextStyle},
    style::{Style, Styles},
    style_chain::StyleDelta,
    value::Value,
};

#[derive(Clone, Debug)]
pub(super) struct Fragment {
    pub text: String,
    pub replacement: Option<String>,
    pub style: TextStyle,
    pub width: f64,
    pub hyphenate: bool,
    pub overhang: bool,
}
#[derive(Clone, Debug)]
pub(super) struct Gap {
    pub style: TextStyle,
    pub width: f64,
    pub origin: TextSeparatorOrigin,
}
#[derive(Clone, Debug)]
pub(super) struct Word {
    pub fragments: Vec<Fragment>,
    pub before: Vec<Gap>,
    pub width: f64,
    pub hyphenation: Option<super::paragraph_hyphen::Hyphenation>,
}
#[derive(Default, Debug)]
pub(super) struct Prepared {
    pub words: Vec<Word>,
    pub trailing: Vec<Gap>,
    pub right_overhang: f64,
}

impl Prepared {
    /// Intervalo entre unidades, distinto do tracking interno medido por FontMetrics.
    pub(super) fn tracking(&self) -> f64 {
        let Some(style) = self
            .words
            .first()
            .and_then(|word| word.fragments.first())
            .map(|f| &f.style)
        else {
            return 0.0;
        };
        style
            .tracking
            .map(|value| value.resolve_pt(style.size.0))
            .unwrap_or(0.0)
    }
}

fn styles_supported(styles: &Styles) -> bool {
    let d = styles.delta();
    let mut permitted = StyleDelta::empty();
    permitted.bold = d.bold;
    permitted.bold_from_strong = d.bold_from_strong;
    permitted.italic = d.italic;
    permitted.italic_from_emph = d.italic_from_emph;
    permitted.size = d.size;
    permitted.fill = d.fill.clone();
    permitted.weight = d.weight;
    permitted.lang = d.lang;
    permitted.font = d.font.clone();
    for (key, value) in &d.custom {
        if !matches!(
            key.as_str(),
            "text.size"
                | "text.fill"
                | "text.bold"
                | "text.italic"
                | "text.weight"
                | "text.style"
                | "text.font"
                | "text.lang"
                | "text.hyphenate"
                | "text.overhang"
                | "eval.flow.scope"
        ) && !crate::compiler::text_costs::is_channel(key.as_str())
        {
            return false;
        }
        permitted.custom.push((key.clone(), value.clone()));
    }
    *d == permitted
}

fn hyphenate<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    match l.chain.custom("text.hyphenate") {
        Some(Value::Bool(b)) => *b,
        _ => matches!(l.chain.custom("par.justify"), Some(Value::Bool(true))),
    }
}
fn overhang<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    !matches!(l.chain.custom("text.overhang"), Some(Value::Bool(false)))
}
fn style_supported<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    style: &TextStyle,
) -> bool {
    (style.dir != Some(Dir::RTL) || (l.bidi_cursor.is_none() && style.tracking.is_none()))
        && !style.math
        && !l.smallcaps
}

fn gap<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    p: &mut Prepared,
    origin: TextSeparatorOrigin,
) -> Option<()> {
    if p.words.is_empty() && origin != TextSeparatorOrigin::Literal {
        return Some(());
    }
    let style = super::text::effective_style(l);
    if !style_supported(l, &style) {
        return None;
    }
    let width = text_measure::space(&l.metrics, &style).0;
    if !width.is_finite() || width < 0.0 {
        return None;
    }
    p.trailing.push(Gap { style, width, origin });
    Some(())
}

fn text<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    p: &mut Prepared,
    text: &str,
) -> Option<()> {
    // This is a certified Latin space-break subset, not an approximation of UAX14.
    if !text.chars().all(|c| {
        c == ' '
            || ((c as u32) <= 0x024f
                && (c.is_alphanumeric() || matches!(c, ',' | '.' | '!' | '?' | '\'')))
    }) {
        return None;
    }
    let style = super::text::effective_style(l);
    if !style_supported(l, &style) {
        return None;
    }
    let mut parts = text.split(' ').peekable();
    while let Some(part) = parts.next() {
        if !part.is_empty() {
            if p.words.is_empty() || !p.trailing.is_empty() {
                p.words.push(Word {
                    fragments: Vec::new(),
                    before: std::mem::take(&mut p.trailing),
                    width: 0.0,
                    hyphenation: None,
                });
            }
            let word = p.words.last_mut()?;
            if let Some(last) = word.fragments.last_mut().filter(|f| {
                f.replacement.is_none()
                    && f.style == style
                    && f.hyphenate == hyphenate(l)
                    && f.overhang == overhang(l)
            }) {
                last.text.push_str(part);
            } else {
                word.fragments.push(Fragment {
                    replacement: None,
                    text: part.into(),
                    style: style.clone(),
                    width: 0.0,
                    hyphenate: hyphenate(l),
                    overhang: overhang(l),
                });
            }
        }
        if parts.peek().is_some() {
            gap(l, p, TextSeparatorOrigin::Literal)?;
        }
    }
    Some(())
}

fn styled<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    p: &mut Prepared,
    body: &Content,
    styles: &Styles,
) -> Option<()> {
    // A preparação de tracking certifica somente o escopo textual uniforme.
    // Fronteiras Styled têm identidade lexical própria mesmo se o estilo
    // resolvido coincidir; o percurso incremental conserva essa semântica.
    if !styles_supported(styles) || super::text::effective_style(l).tracking.is_some() {
        return None;
    }
    let chain = l.chain.clone();
    let style = l.style.clone();
    l.chain = l.chain.push_styles(styles);
    l.style = TextStyle::from(&l.chain);
    let result = collect(l, p, body);
    l.chain = chain;
    l.style = style;
    result
}
fn collect<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    p: &mut Prepared,
    body: &Content,
) -> Option<()> {
    match body.source_body() {
        Content::Empty => Some(()),
        Content::Text(t) => text(l, p, t),
        Content::Space => gap(l, p, TextSeparatorOrigin::Markup),
        Content::SpaceFromNewline => gap(l, p, TextSeparatorOrigin::Newline),
        Content::Sequence(parts) => {
            for part in parts.iter() {
                collect(l, p, part)?;
            }
            Some(())
        }
        Content::Styled(s) => styled(l, p, &s.body, &s.styles),
        Content::ContextBlock(_) if !l.capture_html_context_markers => Some(()),
        Content::Strong(s) => {
            styled(l, p, &s.body, &Styles::from_iter([Style::strong()]))
        }
        Content::Emph(s) => styled(l, p, &s.body, &Styles::from_iter([Style::emph()])),
        _ => None,
    }
}
fn finish<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    mut p: Prepared,
) -> Option<Prepared> {
    if p.words.is_empty() {
        return None;
    }
    for word in &mut p.words {
        for f in &mut word.fragments {
            f.width = text_measure::word(&l.metrics, &f.text, &f.style).0;
            if !f.width.is_finite() || f.width < 0.0 {
                return None;
            }
            word.width += f.width;
        }
        if !word.width.is_finite() {
            return None;
        }
        word.hyphenation = super::paragraph_hyphen::prepare(&l.metrics, word)?;
    }
    if let Some(style) =
        p.words.first().and_then(|w| w.fragments.first()).map(|f| &f.style)
    {
        if style.tracking.is_some() {
            let tracking = p.tracking();
            if !tracking.is_finite()
                || p.words.iter().flat_map(|w| &w.fragments).any(|f| f.style != *style)
                || p.words
                    .iter()
                    .flat_map(|w| &w.before)
                    .chain(&p.trailing)
                    .any(|g| g.style != *style)
                || (tracking != 0.0 && p.words.iter().any(|w| w.hyphenation.is_some()))
            {
                return None;
            }
            for (index, word) in p.words.iter().enumerate() {
                let count = word.before.len();
                let base = word.before.iter().map(|gap| gap.width).sum::<f64>();
                let intervals = if index == 0 {
                    count
                } else if count > 0 {
                    count + 1
                } else {
                    0
                };
                if !base.is_finite() || base + tracking * (intervals as f64) < 0.0 {
                    return None;
                }
            }
            let trailing = p.trailing.iter().map(|gap| gap.width).sum::<f64>()
                + tracking * p.trailing.len() as f64;
            if !trailing.is_finite() || trailing < 0.0 {
                return None;
            }
        }
    }
    if !p
        .words
        .iter()
        .map(|w| w.width + w.before.iter().map(|g| g.width).sum::<f64>())
        .sum::<f64>()
        .is_finite()
    {
        return None;
    }
    Some(p)
}
pub(super) fn prepare<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    body: &Content,
) -> Option<Prepared> {
    let mut p = Prepared::default();
    collect(l, &mut p, body)?;
    finish(l, p)
}
pub(super) fn prepare_parts<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) -> Option<Prepared> {
    let mut p = Prepared::default();
    for part in parts {
        collect(l, &mut p, part)?;
    }
    finish(l, p)
}

#[cfg(test)]
mod tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;
    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());
        l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(false));
        l
    }
    #[test]
    fn preparation_preserves_adjacency_and_separator_origins() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        let body = Content::sequence(vec![
            Content::text("aa"),
            Content::text("bb "),
            Content::text("cc"),
            Content::SpaceFromNewline,
            Content::text("dd"),
            Content::Space,
            Content::text("ee"),
        ]);
        let p = prepare(&mut l, &body).unwrap();
        assert_eq!(p.words.len(), 4);
        assert_eq!(p.words[0].fragments[0].text, "aabb");
        assert_eq!(p.words[1].before[0].origin, TextSeparatorOrigin::Literal);
        assert_eq!(p.words[2].before[0].origin, TextSeparatorOrigin::Newline);
        assert_eq!(p.words[3].before[0].origin, TextSeparatorOrigin::Markup);
        assert!(l.regions.current.current_line.is_empty());
    }
    #[test]
    fn preparation_rejected_nested_scope_restores_state_and_emits_nothing() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        let before = format!("{:?}", l.style);
        let x = l.regions.current.cursor_x;
        let body = Content::styled(
            Content::sequence(vec![
                Content::text("aa"),
                Content::par(Content::text("bb")),
            ]),
            Styles::from_iter([Style::Size(crate::entities::layout_types::Length::pt(
                15.0,
            ))]),
        );
        assert!(prepare(&mut l, &body).is_none());
        assert_eq!(format!("{:?}", l.style), before);
        assert_eq!(l.regions.current.cursor_x, x);
        assert!(l.regions.current.current_line.is_empty());
        assert_eq!(l.chain.custom("text.hyphenate"), Some(&Value::Bool(false)));
    }
    #[test]
    fn uniform_tracking_is_certified_but_nested_scope_remains_incremental() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        l.chain = l.chain.push_custom(
            "text.tracking",
            Value::Length(crate::entities::layout_types::Length::pt(1.0)),
        );
        l.style = TextStyle::from(&l.chain);
        let content = Content::text("alpha beta gamma");
        let p = prepare(&mut l, &content).expect("uniform tracking is measurable");
        assert_eq!(p.words.len(), 3);
        assert_eq!(p.words[1].before.len(), 1);
        assert!(l.regions.current.current_line.is_empty());
        let nested = Content::styled(content, Styles::from_iter([Style::strong()]));
        assert!(prepare(&mut l, &nested).is_none());
    }
    #[test]
    fn preparation_declines_non_latin_without_redefining_it() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        assert!(prepare(&mut l, &Content::text("漢字")).is_none());
    }
}

#[cfg(test)]
mod global_hyphenation_red {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;
    #[test]
    fn hyphenation_admits_explicit_zero_tracking_but_declines_nonzero() {
        use crate::entities::layout_types::Length;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l
            .chain
            .push_custom("text.hyphenate", Value::Bool(true))
            .push_custom("text.tracking", Value::Length(Length::pt(0.0)));
        l.style = TextStyle::from(&l.chain);
        let body = Content::text("representation");
        assert!(prepare(&mut l, &body).is_some());
        l.chain = l.chain.push_custom("text.tracking", Value::Length(Length::pt(0.2)));
        l.style = TextStyle::from(&l.chain);
        assert!(prepare(&mut l, &body).is_none());
    }
    #[test]
    fn global_hyphenation_accepts_uniform_true_policy_before_selection() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(true));
        assert!(prepare(&mut l, &Content::text("representation")).is_some());
    }
}

#[cfg(test)]
mod optical_policy_regression {
    use super::super::{paragraph_hyphen, paragraph_select::Strategy, FixedMetrics};
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;
    #[test]
    fn hardcode_overhang_false_removes_optical_margin_without_changing_breaks() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(true));
        let active =
            prepare(&mut l, &Content::text("representation extraordinary")).unwrap();
        let lines = paragraph_hyphen::select(
            &active,
            60.0,
            12.0,
            true,
            Strategy::Simple,
            crate::compiler::text_costs::LineCosts::default(),
        );
        assert!(lines.iter().any(|p| p.right_overhang > 0.0));
        l.chain = l.chain.push_custom("text.overhang", Value::Bool(false));
        let inactive =
            prepare(&mut l, &Content::text("representation extraordinary")).unwrap();
        let off = paragraph_hyphen::select(
            &inactive,
            60.0,
            12.0,
            true,
            Strategy::Simple,
            crate::compiler::text_costs::LineCosts::default(),
        );
        assert!(off.iter().all(|p| p.right_overhang == 0.0));
        let texts = |ps: &[Prepared]| {
            ps.iter()
                .map(|p| {
                    p.words
                        .iter()
                        .flat_map(|w| &w.fragments)
                        .map(|f| f.text.as_str())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(texts(&lines), texts(&off));
    }
}

#[cfg(test)]
mod shared_cost_preparation {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::Ratio,
    };
    use comemo::Track;
    #[test]
    fn custom_costs_inline_preserves_shared_scope_and_word_adjacency() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l
            .chain
            .push_custom("text.hyphenate", Value::Bool(true))
            .push_custom("text.costs.hyphenation", Value::Ratio(Ratio(10.0)));
        let body = Content::sequence(vec![
            Content::text("rep"),
            Content::styled(
                Content::text("resentation"),
                Styles::new()
                    .push_custom("text.costs.hyphenation", Value::Ratio(Ratio(0.0))),
            ),
        ]);
        let p = prepare(&mut l, &body).expect("costs are not a glyph-style barrier");
        assert_eq!(p.words.len(), 1);
        assert_eq!(p.words[0].fragments.len(), 1);
        assert_eq!(p.words[0].fragments[0].text, "representation");
        assert_eq!(crate::compiler::text_costs::resolve(&l.chain).hyphenation, 1350.0);
        assert!(l.regions.current.current_line.is_empty());
    }
}
