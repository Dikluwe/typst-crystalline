//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_session.md
//! @prompt-hash 58bc255a
//! @layer L1
//! @updated 2026-11-12
use super::{
    paragraph_indent::Shape,
    paragraph_prepare::Prepared,
    paragraph_select::{Line, Strategy},
    FontMetrics, ImageSizer, Layouter,
};
use crate::{
    compiler::text_costs::LineCosts,
    entities::{
        content::Content,
        layout_types::{TextEdge, TextStyle},
        world_types::Font,
    },
};
use std::sync::{Arc, Mutex};

const MAX_ENTRIES: usize = 512;
const MAX_PAYLOAD: usize = 8 * 1024 * 1024;
const MAX_INPUT: usize = 16 * 1024;
const MAX_KEY: usize = 64 * 1024;
const MAX_ATOMS: usize = 2048;
const MAX_FONT: usize = 16 * 1024 * 1024;
const MAX_FONT_OWNERS: usize = 4;

/// Owned identity for a certified immutable horizontal-metric policy.
#[derive(Clone)]
pub struct MetricIdentity(Identity);
#[derive(Clone)]
enum Identity {
    Generation(Arc<()>),
    Font(&'static str, Font),
}
impl Default for MetricIdentity {
    fn default() -> Self {
        Self(Identity::Generation(Arc::new(())))
    }
}
impl MetricIdentity {
    /// A fresh generation; clones retain the same measurement guarantee.
    pub fn new() -> Self {
        Self::default()
    }
    /// Certify a named deterministic provider using exactly this immutable font owner.
    pub fn for_font(provider: &'static str, font: Font) -> Self {
        Self(Identity::Font(provider, font))
    }
    pub(super) fn same(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Identity::Generation(a), Identity::Generation(b)) => Arc::ptr_eq(a, b),
            (Identity::Font(a, x), Identity::Font(b, y)) => {
                a == b && font_owner(x) == font_owner(y)
            }
            (Identity::Generation(_), Identity::Font(..))
            | (Identity::Font(..), Identity::Generation(_)) => false,
        }
    }
    pub(super) fn font(&self) -> Option<((u8, usize), usize)> {
        match &self.0 {
            Identity::Font(_, font) => Some((font_owner(font), font.as_slice().len())),
            Identity::Generation(_) => None,
        }
    }
}
fn font_owner(font: &Font) -> (u8, usize) {
    match font {
        Font::Vec(owner) => (0, Arc::as_ptr(owner) as usize),
        Font::Mmap(owner) => (1, Arc::as_ptr(owner) as usize),
    }
}

/// Caller-owned bounded retention of immutable paragraph products.
#[derive(Clone, Default)]
pub struct ParagraphSession {
    pub(super) paint: Arc<super::paragraph_paint::PaintSession>,
    memo: Arc<Mutex<Memo>>,
}
impl ParagraphSession {
    pub fn new() -> Self {
        Self::default()
    }
    /// Cumulative hits, admitted misses, retained entries.
    pub fn stats(&self) -> (usize, usize, usize) {
        let m = self.memo.lock().unwrap();
        (m.hits, m.misses, m.entries.len())
    }
}
#[derive(Default)]
struct Memo {
    entries: Vec<Entry>,
    charged: usize,
    hits: usize,
    misses: usize,
}
struct Entry {
    key: Key,
    product: Arc<Product>,
    charge: usize,
}
#[derive(PartialEq, Eq)]
enum Atom {
    Text(String),
    Space,
    Newline,
}
struct Key {
    atoms: Vec<Atom>,
    style: TextStyle,
    bits: Vec<u64>,
    identity: MetricIdentity,
    strategy: Strategy,
    numeric: [u64; 5],
    flags: [bool; 3],
    charge: usize,
}
impl Key {
    fn same(&self, other: &Self) -> bool {
        self.identity.same(&other.identity)
            && self.strategy == other.strategy
            && self.numeric == other.numeric
            && self.flags == other.flags
            && self.atoms == other.atoms
            && self.bits == other.bits
            && self.style == other.style
    }
}
pub(super) struct Product {
    pub(super) paint_id: Arc<()>,
    pub(super) paintable: bool,
    pub(super) preserves_atoms: bool,
    pub(super) prepared: Prepared,
    pub(super) lines: Vec<Line>,
    pub(super) hyphenated: Option<Vec<Prepared>>,
}
pub(super) struct Request {
    session: ParagraphSession,
    key: Key,
}
impl Request {
    pub(super) fn lookup(&self) -> Option<Arc<Product>> {
        let mut m = self.session.memo.lock().unwrap();
        if let Some(index) = m.entries.iter().position(|e| e.key.same(&self.key)) {
            let entry = m.entries.remove(index);
            let product = entry.product.clone();
            m.entries.push(entry);
            m.hits += 1;
            Some(product)
        } else {
            m.misses += 1;
            None
        }
    }
    pub(super) fn store(self, product: Arc<Product>) {
        let Some(charge) =
            product_charge(&product).and_then(|n| n.checked_add(self.key.charge))
        else {
            return;
        };
        if charge > MAX_PAYLOAD {
            return;
        }
        let mut m = self.session.memo.lock().unwrap();
        loop {
            let mut owners = Vec::new();
            for key in m.entries.iter().map(|e| &e.key).chain(std::iter::once(&self.key))
            {
                if let Some((owner, _)) = key.identity.font() {
                    if !owners.contains(&owner) {
                        owners.push(owner)
                    }
                }
            }
            if m.entries.len() < MAX_ENTRIES
                && m.charged.checked_add(charge).is_some_and(|n| n <= MAX_PAYLOAD)
                && owners.len() <= MAX_FONT_OWNERS
            {
                break;
            }
            if m.entries.is_empty() {
                return;
            }
            m.charged -= m.entries.remove(0).charge;
        }
        m.charged += charge;
        m.entries.push(Entry { key: self.key, product, charge });
    }
}
fn collect(
    body: &Content,
    atoms: &mut Vec<Atom>,
    text: &mut String,
    charge: &mut usize,
) -> Option<()> {
    match body {
        Content::Empty => {}
        Content::Source(source) => collect(&source.body, atoms, text, charge)?,
        Content::Sequence(parts) => {
            for part in parts.iter() {
                collect(part, atoms, text, charge)?
            }
        }
        Content::Text(value) => {
            if text.len().checked_add(value.len())? > MAX_INPUT
                || atoms.len() >= MAX_ATOMS
            {
                return None;
            }
            text.push_str(value);
            *charge = charge.checked_add(value.len())?;
            atoms.push(Atom::Text(value.to_string()));
        }
        Content::Space | Content::SpaceFromNewline => {
            if text.len() >= MAX_INPUT || atoms.len() >= MAX_ATOMS {
                return None;
            }
            text.push(' ');
            atoms.push(if matches!(body, Content::Space) {
                Atom::Space
            } else {
                Atom::Newline
            });
        }
        Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Symbol(..)
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Styled(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => return None,
    }
    Some(())
}
pub(super) fn preserves_atom_boundaries(parts: &[Content]) -> bool {
    let mut atoms = Vec::new();
    let mut text = String::new();
    let mut charge = 0;
    for part in parts {
        if collect(part, &mut atoms, &mut text, &mut charge).is_none() {
            return false;
        }
    }
    let mut adjacent = false;
    for atom in atoms {
        match atom {
            Atom::Text(value) => {
                if value.is_empty() {
                    continue;
                }
                if adjacent && !value.starts_with(' ') {
                    return false;
                }
                adjacent = !value.ends_with(' ');
            }
            Atom::Space | Atom::Newline => adjacent = false,
        }
    }
    true
}
fn color_payload(
    color: &crate::entities::color::Color,
    bits: &mut Vec<u64>,
) -> Option<usize> {
    let mut color = color;
    let mut size = 0usize;
    for _ in 0..32 {
        if let crate::entities::color::Color::Spot(spot) = color {
            bits.push(spot.tint.0.to_bits());
            size =
                size.checked_add(spot.colorant.name.as_ref().map_or(0, |n| n.len()))?;
            color = &spot.colorant.fallback;
        } else {
            return Some(size);
        }
    }
    None
}
pub(super) fn style_payload(style: &TextStyle, bits: &mut Vec<u64>) -> Option<usize> {
    use crate::entities::font_list::FontNamePattern;
    let mut size = std::mem::size_of::<TextStyle>();
    bits.push(style.size.0.to_bits());
    for length in [
        Some(style.baseline_offset),
        style.tracking,
        style.leading,
        style.highlight_radius,
        style.highlight_extent,
        style.subscript_size,
        style.superscript_size,
    ]
    .into_iter()
    .flatten()
    {
        bits.extend([length.abs.to_pt().to_bits(), length.em.to_bits()]);
    }
    for edge in [style.top_edge.as_ref(), style.bottom_edge.as_ref()]
        .into_iter()
        .flatten()
    {
        match edge {
            TextEdge::Length(l) => bits.extend([l.abs.to_pt().to_bits(), l.em.to_bits()]),
            TextEdge::Metric(name) => size = size.checked_add(name.len())?,
        }
    }
    for color in style.fill.iter().chain(style.highlight.iter()) {
        size = size.checked_add(color_payload(color, bits)?)?;
    }
    if let Some(fonts) = &style.font {
        for family in fonts.as_slice() {
            let name = match &family.name {
                FontNamePattern::Literal(n) => n.as_str(),
                FontNamePattern::Regex(r) => r.pattern(),
            };
            size = size.checked_add(name.len())?;
            for value in family
                .variants
                .iter()
                .chain(family.variant.iter())
                .chain(family.weight.iter())
                .chain(family.style.iter())
            {
                size = size.checked_add(value.len())?;
            }
        }
    }
    if let Some(vars) = &style.variations {
        size = size.checked_add(vars.0.len().checked_mul(8)?)?;
        bits.extend(vars.0.iter().map(|(_, v)| v.to_bits() as u64));
    }
    Some(size)
}
fn prepared_charge(p: &Prepared) -> Option<usize> {
    let mut size = std::mem::size_of::<Prepared>();
    let mut bits = Vec::new();
    for word in &p.words {
        size = size.checked_add(std::mem::size_of_val(word))?;
        for fragment in &word.fragments {
            size = size
                .checked_add(std::mem::size_of_val(fragment))?
                .checked_add(fragment.text.len())?
                .checked_add(fragment.replacement.as_ref().map_or(0, |s| s.len()))?
                .checked_add(style_payload(&fragment.style, &mut bits)?)?;
            bits.clear();
        }
        for gap in &word.before {
            size = size
                .checked_add(std::mem::size_of_val(gap))?
                .checked_add(style_payload(&gap.style, &mut bits)?)?;
            bits.clear();
        }
        // Include every retained hyphenation vector in the logical budget.
        if let Some(hyphenation) = &word.hyphenation {
            size = size.checked_add(hyphenation.retained_payload()?)?;
        }
    }
    for gap in &p.trailing {
        size = size
            .checked_add(std::mem::size_of_val(gap))?
            .checked_add(style_payload(&gap.style, &mut bits)?)?;
        bits.clear();
    }
    Some(size)
}
fn product_charge(p: &Product) -> Option<usize> {
    let mut size = std::mem::size_of::<Product>()
        .checked_add(prepared_charge(&p.prepared)?)?
        .checked_add(p.lines.len().checked_mul(std::mem::size_of::<Line>())?)?;
    if let Some(lines) = &p.hyphenated {
        for line in lines {
            size = size.checked_add(prepared_charge(line)?)?;
        }
    }
    Some(size)
}
pub(super) fn request<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
    options: (Strategy, Shape, f64, bool, LineCosts),
) -> Option<Request> {
    let session = l.metrics.paragraph_session()?;
    if l.smallcaps || l.bidi_cursor.is_some() {
        return None;
    }
    let (strategy, shape, font, justify, costs) = options;
    let mut atoms = Vec::new();
    let mut text = String::new();
    let mut charge = std::mem::size_of::<Key>();
    for part in parts {
        collect(part, &mut atoms, &mut text, &mut charge)?;
    }
    if atoms.is_empty() {
        return None;
    }
    let style = super::text::effective_style(l);
    let mut bits = Vec::new();
    charge = charge
        .checked_add(atoms.len().checked_mul(std::mem::size_of::<Atom>())?)?
        .checked_add(style_payload(&style, &mut bits)?)?
        .checked_add(bits.len().checked_mul(8)?)?;
    if charge > MAX_KEY {
        return None;
    }
    text.push_str(" -");
    let identity = l.metrics.paragraph_identity(&text, &style)?;
    if identity.font().is_some_and(|(_, bytes)| bytes > MAX_FONT) {
        return None;
    }
    let hyphenate = match l.chain.custom("text.hyphenate") {
        Some(crate::entities::value::Value::Bool(v)) => *v,
        _ => justify,
    };
    let overhang = !matches!(
        l.chain.custom("text.overhang"),
        Some(crate::entities::value::Value::Bool(false))
    );
    Some(Request {
        session,
        key: Key {
            atoms,
            style,
            bits,
            identity,
            strategy,
            numeric: [
                shape.width(true).to_bits(),
                shape.width(false).to_bits(),
                font.to_bits(),
                costs.hyphenation.to_bits(),
                costs.runt.to_bits(),
            ],
            flags: [justify, hyphenate, overhang],
            charge,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins, PagedDocument, Pt},
    };
    use comemo::Track;
    use std::sync::atomic::{AtomicUsize, Ordering};
    #[derive(Clone)]
    struct Provider {
        session: ParagraphSession,
        identity: MetricIdentity,
        calls: Arc<AtomicUsize>,
        widths: Arc<AtomicUsize>,
        scale: f64,
        proof: bool,
    }
    impl FontMetrics for Provider {
        fn paragraph_session(&self) -> Option<ParagraphSession> {
            Some(self.session.clone())
        }
        fn paragraph_identity(&self, _: &str, _: &TextStyle) -> Option<MetricIdentity> {
            self.proof.then(|| self.identity.clone())
        }
        fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            self.widths.fetch_add(1, Ordering::Relaxed);
            super::super::text_measure::default_text_width(self, text, size, style)
        }
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            self.calls.fetch_add(1, Ordering::Relaxed);
            FixedMetrics.advance(text, size, style) * self.scale
        }
        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.cap_height(size, style)
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(size, style)
        }
    }
    fn provider() -> Provider {
        Provider {
            session: ParagraphSession::new(),
            identity: MetricIdentity::new(),
            calls: Arc::new(AtomicUsize::new(0)),
            widths: Arc::new(AtomicUsize::new(0)),
            scale: 1.0,
            proof: true,
        }
    }
    fn layout(p: &Provider, text: &str, width: f64, size: f64) -> PagedDocument {
        configured(p, text, width, size, |_| {})
    }
    fn words(doc: &PagedDocument) -> Vec<(usize, String, u64, u64)> {
        doc.pages
            .iter()
            .enumerate()
            .flat_map(|(page, frame)| {
                frame.items.iter().filter_map(move |item| {
                    if let FrameItem::Text { pos, text, .. } = item {
                        Some((
                            page,
                            text.to_string(),
                            pos.x.0.to_bits(),
                            pos.y.0.to_bits(),
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn equivalent(a: &PagedDocument, b: &PagedDocument) {
        assert_eq!(a.pages.len(), b.pages.len());
        assert_eq!(a.plain_text(), b.plain_text());
        assert_eq!(words(a), words(b));
    }
    fn configured(
        p: &Provider,
        text: &str,
        width: f64,
        size: f64,
        configure: impl FnOnce(&mut Layouter<Provider, NullImageSizer>),
    ) -> PagedDocument {
        configured_route(p, text, width, size, false, configure)
    }
    fn configured_route(
        p: &Provider,
        text: &str,
        width: f64,
        size: f64,
        direct: bool,
        configure: impl FnOnce(&mut Layouter<Provider, NullImageSizer>),
    ) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            p.clone(),
            NullImageSizer,
            size,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push(crate::entities::style_chain::StyleDelta {
            size: Some(crate::entities::layout_types::Length::pt(size)),
            ..crate::entities::style_chain::StyleDelta::empty()
        });
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = width;
        l.page_config.height = 120.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = width;
        l.regions.current.height = 120.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        configure(&mut l);
        let accepted = if direct {
            super::super::paragraph_plan::try_layout(&mut l, &Content::text(text))
        } else {
            super::super::paragraph_plan::try_parts(&mut l, &[Content::text(text)])
        };
        assert!(accepted);
        l.finish()
    }
    #[test]
    fn paragraph_product_reuses_actual_measurement_across_layouters() {
        let p = provider();
        let first = layout(&p, "alpha beta gamma delta", 250.0, 11.0);
        let before = p.widths.load(Ordering::Relaxed);
        let total_before = p.calls.load(Ordering::Relaxed);
        let next = layout(&p, "alpha beta gamma delta", 250.0, 11.0);
        equivalent(&first, &next);
        assert!(
            p.widths.load(Ordering::Relaxed) - before < before,
            "paragraph preparation was measured again"
        );
        assert!(p.session.stats().0 > 0);
        assert!(p.calls.load(Ordering::Relaxed) - total_before < total_before);
    }
    #[test]
    fn paragraph_product_invalidates_text_style_width_and_provider() {
        let mut p = provider();
        let _ = layout(&p, "alpha beta gamma delta", 250.0, 11.0);
        for (text, width, size) in [
            ("changed beta gamma delta", 250.0, 11.0),
            ("alpha beta gamma delta", 180.0, 11.0),
            ("alpha beta gamma delta", 250.0, 18.0),
        ] {
            let hits = p.session.stats().0;
            let warm = layout(&p, text, width, size);
            let mut cold = p.clone();
            cold.session = ParagraphSession::new();
            let cold = layout(&cold, text, width, size);
            equivalent(&warm, &cold);
            assert_eq!(p.session.stats().0, hits);
        }
        p.identity = MetricIdentity::new();
        p.scale = 1.8;
        let hits = p.session.stats().0;
        let warm = layout(&p, "alpha beta gamma delta", 250.0, 11.0);
        let mut cold = p.clone();
        cold.session = ParagraphSession::new();
        equivalent(&warm, &layout(&cold, "alpha beta gamma delta", 250.0, 11.0));
        assert_eq!(p.session.stats().0, hits);
    }
    #[test]
    fn paragraph_product_missing_proof_and_independent_sessions_remain_cold() {
        let mut p = provider();
        p.proof = false;
        let _ = layout(&p, "alpha beta", 250.0, 11.0);
        let _ = layout(&p, "alpha beta", 250.0, 11.0);
        assert_eq!(p.session.stats(), (0, 0, 0));
        assert_eq!(provider().session.stats(), (0, 0, 0));
    }
    #[test]
    fn paragraph_product_policy_and_current_page_position() {
        use crate::entities::value::Value;
        let p = provider();
        let text = "internationalization representation extraordinary specification";
        let cold = Provider { session: ParagraphSession::new(), ..p.clone() };
        let _ = layout(&p, text, 180.0, 11.0);
        for (key, value) in [
            ("par.linebreaks", Value::Str("optimized".into())),
            ("par.justify", Value::Bool(true)),
            ("text.hyphenate", Value::Bool(true)),
            ("text.overhang", Value::Bool(false)),
            (
                "text.costs.hyphenation",
                Value::Ratio(crate::entities::layout_types::Ratio(0.0)),
            ),
            ("text.costs.runt", Value::Ratio(crate::entities::layout_types::Ratio(0.0))),
        ] {
            let hits = p.session.stats().0;
            let configure = |l: &mut Layouter<Provider, NullImageSizer>| {
                l.chain = l.chain.push_custom(key, value.clone());
            };
            let warm = configured(&p, text, 180.0, 11.0, configure);
            let cold_before = p.widths.load(Ordering::Relaxed);
            let fresh = configured(&cold, text, 180.0, 11.0, configure);
            let cold_demand = p.widths.load(Ordering::Relaxed) - cold_before;
            equivalent(&warm, &fresh);
            assert_eq!(p.session.stats().0, hits, "changed policy must miss: {key}");
            let before = p.widths.load(Ordering::Relaxed);
            equivalent(&warm, &configured(&p, text, 180.0, 11.0, configure));
            assert!(
                p.widths.load(Ordering::Relaxed) - before < cold_demand,
                "policy product should avoid preparation: {key}"
            );
        }
        let shift = |l: &mut Layouter<Provider, NullImageSizer>| {
            l.regions.current.cursor_y = Pt(75.0);
        };
        let hits = p.session.stats().0;
        equivalent(
            &configured(&p, text, 180.0, 11.0, shift),
            &configured(&cold, text, 180.0, 11.0, shift),
        );
        assert!(
            p.session.stats().0 > hits,
            "fresh placement should reuse horizontal product"
        );
    }
    #[test]
    fn paragraph_product_entries_are_bounded_and_evicted_text_is_remeasured() {
        let p = provider();
        for i in 0..600 {
            let _ = layout(&p, &format!("Paragraph {i} words"), 250.0, 11.0);
        }
        assert!(p.session.stats().2 <= 512);
        let hits = p.session.stats().0;
        let before = p.calls.load(Ordering::Relaxed);
        let actual = layout(&p, "Paragraph 0 words", 250.0, 11.0);
        assert_eq!(p.session.stats().0, hits);
        assert!(p.calls.load(Ordering::Relaxed) > before);
        equivalent(
            &actual,
            &layout(
                &Provider { session: ParagraphSession::new(), ..p.clone() },
                "Paragraph 0 words",
                250.0,
                11.0,
            ),
        );
    }
    #[test]
    fn paragraph_product_oversized_input_stays_cold() {
        let p = provider();
        let text = "a".repeat(17 * 1024);
        let first = layout(&p, &text, 250.0, 11.0);
        let next = layout(&p, &text, 250.0, 11.0);
        equivalent(&first, &next);
        assert_eq!(p.session.stats(), (0, 0, 0));
    }
    #[test]
    fn paragraph_product_retains_at_most_four_actual_font_owners() {
        let mut p = provider();
        let fonts: Vec<_> = (0..5).map(|_| Font::from_data(vec![1])).collect();
        for font in &fonts {
            p.identity = MetricIdentity::for_font("fixed-test", font.clone());
            let _ = layout(&p, "alpha beta", 250.0, 11.0);
        }
        assert!(p.session.stats().2 <= 4);
        let hits = p.session.stats().0;
        p.identity = MetricIdentity::for_font("fixed-test", fonts[0].clone());
        let actual = layout(&p, "alpha beta", 250.0, 11.0);
        assert_eq!(p.session.stats().0, hits, "old actual owner must have been evicted");
        equivalent(&actual, &layout(&provider(), "alpha beta", 250.0, 11.0));
    }
    #[test]
    fn paragraph_direct_product_reuses_measurements_and_keeps_current_geometry() {
        let mut p = provider();
        let text = "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda";
        let run = |p: &Provider, w, s| configured_route(p, text, w, s, true, |_| {});
        let cold = run(&p, 130.0, 11.0);
        let before = p.widths.load(Ordering::Relaxed);
        let warm = run(&p, 130.0, 11.0);
        equivalent(&cold, &warm);
        assert!(
            p.widths.load(Ordering::Relaxed) - before < before,
            "direct body preparation was measured again"
        );
        assert!(p.session.stats().0 > 0);
        for (width, size) in [(110.0, 11.0), (130.0, 16.0)] {
            let misses = p.session.stats().1;
            let actual = run(&p, width, size);
            let mut fresh = p.clone();
            fresh.session = ParagraphSession::new();
            equivalent(&actual, &run(&fresh, width, size));
            assert!(p.session.stats().1 > misses);
        }
        p.identity = MetricIdentity::new();
        p.scale = 1.5;
        let misses = p.session.stats().1;
        let actual = run(&p, 130.0, 11.0);
        let mut fresh = p.clone();
        fresh.session = ParagraphSession::new();
        equivalent(&actual, &run(&fresh, 130.0, 11.0));
        assert!(p.session.stats().1 > misses);
        let move_origin = |l: &mut Layouter<Provider, NullImageSizer>| {
            l.regions.current.cursor_y = Pt(55.0);
        };
        let moved = configured_route(&p, text, 130.0, 11.0, true, move_origin);
        fresh.session = ParagraphSession::new();
        equivalent(
            &moved,
            &configured_route(&fresh, text, 130.0, 11.0, true, move_origin),
        );
        let hanging = |l: &mut Layouter<Provider, NullImageSizer>| {
            l.chain = l.chain.push_custom(
                "par.hanging-indent",
                crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(12.0),
                ),
            );
        };
        let misses = p.session.stats().1;
        let actual = configured_route(&p, text, 130.0, 11.0, true, hanging);
        fresh.session = ParagraphSession::new();
        equivalent(&actual, &configured_route(&fresh, text, 130.0, 11.0, true, hanging));
        assert!(p.session.stats().1 > misses);
    }

    #[test]
    fn paragraph_direct_refusal_preserves_indent_and_following_layout() {
        let p = provider();
        let text = "alpha beta gamma delta epsilon zeta";
        let ordinary = configured_route(&p, text, 130.0, 11.0, true, |_| {});
        let after_refusal = configured_route(&p, text, 130.0, 11.0, true, |l| {
            let before = format!("{:?}", l.paragraph_indent);
            assert!(!super::super::paragraph_plan::try_layout(l, &Content::Empty));
            assert_eq!(format!("{:?}", l.paragraph_indent), before);
        });
        equivalent(&ordinary, &after_refusal);
    }
}
