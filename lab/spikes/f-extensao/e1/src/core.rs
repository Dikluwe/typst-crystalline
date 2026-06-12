//! Toy "core" for design E1 — the analogue of 01_core's Content/Element hub.
//!
//! Boundary shape (E1): a hybrid static/dynamic content tree.
//!   - native elements stay STATIC, behind `Content::Native(Arc<NativeElem>)`;
//!   - third-party elements cross the boundary as `Content::Dynamic(Arc<dyn Element>)`.
//! The hub's matches dispatch through the `Element` trait for the dynamic arm,
//! while native arms are plain enum matches (monomorphic, no vtable).
//!
//! A USER who writes a new element only needs: the `Element` trait, `Value`,
//! `PropMap`, `StyleChain`, and the registry. They never touch this match.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Values & properties
// ---------------------------------------------------------------------------

/// A property value. Toy: just the few shapes the demo needs.
#[derive(Clone, Debug)]
pub enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
    Content(Content),
}

// Manual PartialEq: compare scalars; Content values are never equal (toy —
// the demo only ever compares Bool/Str, e.g. `== Value::Bool(true)`).
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// Property bag for ONE element instance / ONE style layer.
///
/// E1 property model: a few CLOSED native style fields (here: 3) for the props
/// the hub itself understands and wants cheap typed access to, PLUS an OPEN
/// map for everything else (native overflow + all user element props).
///
/// At 3 native props this is fine. At ~273 (see report) the closed fields would
/// either stay a fixed handful of hot ones and let the long tail live in `open`,
/// or the whole thing collapses to just the open map. Marginal cost of a new
/// prop in the OPEN map is O(1) and touches ZERO core files.
#[derive(Clone, Debug, Default)]
pub struct PropMap {
    // ---- 3 closed NATIVE style props (typed, hot path) ----
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub color: Option<String>,
    // ---- open map: user props + native long-tail ----
    pub open: HashMap<String, Value>,
}

impl PropMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&mut self, key: &str, val: Value) {
        match (key, &val) {
            ("bold", Value::Bool(b)) => self.bold = Some(*b),
            ("italic", Value::Bool(b)) => self.italic = Some(*b),
            ("color", Value::Str(s)) => self.color = Some(s.clone()),
            _ => {
                self.open.insert(key.to_string(), val);
            }
        }
    }
    pub fn get(&self, key: &str) -> Option<Value> {
        match key {
            "bold" => self.bold.map(Value::Bool),
            "italic" => self.italic.map(Value::Bool),
            "color" => self.color.clone().map(Value::Str),
            _ => self.open.get(key).cloned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Style chain
// ---------------------------------------------------------------------------

/// One `#set <kind>(...)` layer: props that apply to elements of `target` kind.
#[derive(Clone, Debug)]
pub struct StyleLayer {
    pub target: &'static str,
    pub props: PropMap,
}

/// The StyleChain: a linked list of set-layers, innermost first.
/// Toy version owns its layers in a Vec (real one is a cons-list of borrows).
#[derive(Clone, Debug, Default)]
pub struct StyleChain {
    layers: Vec<StyleLayer>,
}

impl StyleChain {
    pub fn new() -> Self {
        Self::default()
    }
    /// `#set kind(props)` — push a scoped layer. Returns a child chain (cheap clone in toy).
    pub fn set(&self, target: &'static str, props: PropMap) -> StyleChain {
        let mut c = self.clone();
        c.layers.push(StyleLayer { target, props });
        c
    }
    /// Resolve property `key` for an element of `kind`, innermost layer wins.
    pub fn get(&self, kind: &str, key: &str) -> Option<Value> {
        for layer in self.layers.iter().rev() {
            if layer.target == kind {
                if let Some(v) = layer.props.get(key) {
                    return Some(v);
                }
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// The Element trait — the BOUNDARY contract (this is all a user reads)
// ---------------------------------------------------------------------------

/// The extension contract. A third-party element implements this; the hub
/// dispatches the `Dynamic` arm through it. No core code changes per element.
pub trait Element: Debug + Send + Sync {
    /// Stable identity / registered name. Used by query() and #set/#show.
    fn kind(&self) -> &'static str;

    /// Flatten to plain text (for query/accessibility/search).
    fn plain_text(&self) -> String;

    /// Render to a string under the resolved style chain.
    fn render(&self, chain: &StyleChain) -> String;

    /// Read an own property (for query / show-rules). Own props live in the
    /// element's instance map; `None` means "not set on the instance".
    fn get_prop(&self, key: &str) -> Option<Value>;

    /// Produce a copy with one own property overridden (for #show transforms).
    fn with_prop(&self, key: &str, val: Value) -> Arc<dyn Element>;

    /// Children, so query() can recurse into user containers.
    fn children(&self) -> Vec<Content> {
        Vec::new()
    }
}

// ---------------------------------------------------------------------------
// Native elements (STATIC side of the hybrid)
// ---------------------------------------------------------------------------

/// Native leaf + container, kept as a closed enum behind Arc.
/// These never go through the vtable; the hub matches them directly.
#[derive(Clone, Debug)]
pub enum NativeElem {
    Text(String),
    Heading { level: u8, body: Box<Content> },
}

// ---------------------------------------------------------------------------
// Content — the hub enum
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Content {
    /// Static native elements (no dynamic dispatch).
    Native(Arc<NativeElem>),
    /// Extension boundary: any third-party element.
    Dynamic(Arc<dyn Element>),
    /// A flat sequence (toy Sequence).
    Seq(Arc<Vec<Content>>),
}

impl Content {
    pub fn text(s: impl Into<String>) -> Content {
        Content::Native(Arc::new(NativeElem::Text(s.into())))
    }
    pub fn heading(level: u8, body: Content) -> Content {
        Content::Native(Arc::new(NativeElem::Heading {
            level,
            body: Box::new(body),
        }))
    }
    pub fn seq(items: Vec<Content>) -> Content {
        Content::Seq(Arc::new(items))
    }
    pub fn dynamic(e: Arc<dyn Element>) -> Content {
        Content::Dynamic(e)
    }

    /// Hub kind dispatch — native arms static, dynamic arm via trait.
    pub fn kind(&self) -> &'static str {
        match self {
            Content::Native(n) => match &**n {
                NativeElem::Text(_) => "text",
                NativeElem::Heading { .. } => "heading",
            },
            Content::Dynamic(e) => e.kind(),
            Content::Seq(_) => "sequence",
        }
    }

    /// Hub plain_text — note the SAME match shape.
    pub fn plain_text(&self) -> String {
        match self {
            Content::Native(n) => match &**n {
                NativeElem::Text(s) => s.clone(),
                NativeElem::Heading { body, .. } => body.plain_text(),
            },
            Content::Dynamic(e) => e.plain_text(),
            Content::Seq(items) => items.iter().map(|c| c.plain_text()).collect::<String>(),
        }
    }

    /// Hub render — resolves native style props from the chain for natives,
    /// delegates to the trait for dynamics. This is the only place the
    /// static/dynamic split shows up in the hot path.
    pub fn render(&self, chain: &StyleChain) -> String {
        match self {
            Content::Native(n) => match &**n {
                NativeElem::Text(s) => {
                    // apply native style props (bold/color) resolved from chain
                    let mut out = s.clone();
                    if chain.get("text", "bold") == Some(Value::Bool(true)) {
                        out = format!("*{out}*");
                    }
                    if let Some(Value::Str(c)) = chain.get("text", "color") {
                        out = format!("<{c}>{out}</{c}>");
                    }
                    out
                }
                NativeElem::Heading { level, body } => {
                    format!("{} {}", "#".repeat(*level as usize), body.render(chain))
                }
            },
            Content::Dynamic(e) => e.render(chain),
            Content::Seq(items) => items.iter().map(|c| c.render(chain)).collect::<String>(),
        }
    }

    /// Recurse to children (native + dynamic) for query.
    pub fn children(&self) -> Vec<Content> {
        match self {
            Content::Native(n) => match &**n {
                NativeElem::Text(_) => Vec::new(),
                NativeElem::Heading { body, .. } => vec![(**body).clone()],
            },
            Content::Dynamic(e) => e.children(),
            Content::Seq(items) => items.as_ref().clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Registry — register element constructors by name (#set / parser target)
// ---------------------------------------------------------------------------

/// A constructor: takes an own-prop map, yields a boxed element.
pub type Ctor = fn(PropMap) -> Arc<dyn Element>;

/// Toy registry. Real core would key this off a global/interned table;
/// here it's an explicit value passed around to keep L1 purity honest
/// (no static mut).
#[derive(Default)]
pub struct Registry {
    ctors: HashMap<&'static str, Ctor>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, kind: &'static str, ctor: Ctor) {
        self.ctors.insert(kind, ctor);
    }
    pub fn construct(&self, kind: &str, props: PropMap) -> Option<Arc<dyn Element>> {
        self.ctors.get(kind).map(|c| c(props))
    }
}

// ---------------------------------------------------------------------------
// query() — collect all elements of a given kind in the tree
// ---------------------------------------------------------------------------

/// Walk the tree, return every node whose kind() == `kind`.
pub fn query(root: &Content, kind: &str) -> Vec<Content> {
    let mut out = Vec::new();
    fn walk(c: &Content, kind: &str, out: &mut Vec<Content>) {
        if c.kind() == kind {
            out.push(c.clone());
        }
        for child in c.children() {
            walk(&child, kind, out);
        }
    }
    walk(root, kind, &mut out);
    out
}

// ---------------------------------------------------------------------------
// show transform — minimal #show: a fn Content -> Content applied to matches
// ---------------------------------------------------------------------------

/// A show rule: kind to match + a transform on matching nodes.
pub struct ShowRule {
    pub kind: &'static str,
    pub transform: fn(&Content) -> Content,
}

/// Apply a show rule bottom-up over the tree (toy: one rule, one pass).
pub fn apply_show(root: &Content, rule: &ShowRule) -> Content {
    // recurse into children first
    let rebuilt = match root {
        Content::Seq(items) => Content::seq(
            items.iter().map(|c| apply_show(c, rule)).collect(),
        ),
        Content::Native(n) => match &**n {
            NativeElem::Heading { level, body } => {
                Content::heading(*level, apply_show(body, rule))
            }
            _ => root.clone(),
        },
        Content::Dynamic(_) => root.clone(),
    };
    if rebuilt.kind() == rule.kind {
        (rule.transform)(&rebuilt)
    } else {
        rebuilt
    }
}
