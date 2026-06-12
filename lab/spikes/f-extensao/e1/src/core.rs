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
    /// A node carrying show-rule GUARDS (toy analogue of vanilla
    /// `meta().lifecycle` bitset). Transparent for kind/render/children; it
    /// only records which recipe indices were already applied to `inner`.
    Guarded {
        guards: Vec<RecipeIndex>,
        inner: Arc<Content>,
    },
    /// A subtree wrapped with scoped `#set` layers produced by a show-SET
    /// recipe (toy analogue of vanilla `StyledElem{child, styles}`).
    Styled {
        layers: Vec<StyleLayer>,
        child: Arc<Content>,
    },
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
    pub fn styled(child: Content, layers: Vec<StyleLayer>) -> Content {
        Content::Styled { layers, child: Arc::new(child) }
    }

    /// Peel any transparent `Guarded` shell to reach the underlying element.
    /// Show-rule closures use this so they can pattern-match the real node
    /// while the guard set still rides along on the original (it is re-applied
    /// to the produced output by the realizer). Vanilla closures get this for
    /// free because the public Content API sees through the meta bitset.
    pub fn peeled(&self) -> &Content {
        match self {
            Content::Guarded { inner, .. } => inner.peeled(),
            other => other,
        }
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
            // Transparent wrappers report the kind of what they wrap, so
            // recipes still match the underlying element.
            Content::Guarded { inner, .. } => inner.kind(),
            Content::Styled { child, .. } => child.kind(),
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
            Content::Guarded { inner, .. } => inner.plain_text(),
            Content::Styled { child, .. } => child.plain_text(),
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
            Content::Guarded { inner, .. } => inner.render(chain),
            // A show-set wrapper renders its child under an AUGMENTED chain:
            // its scoped layers stack on top of the inherited ones, then leave
            // scope (they don't leak to siblings).
            Content::Styled { layers, child } => {
                let mut augmented = chain.clone();
                for l in layers {
                    augmented = augmented.set(l.target, l.props.clone());
                }
                child.render(&augmented)
            }
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
            Content::Guarded { inner, .. } => inner.children(),
            Content::Styled { child, .. } => child.children(),
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
// #show — REAL harness modelling vanilla recipe semantics (Passo 333, Parte 2)
// ---------------------------------------------------------------------------
//
// This replaces the P332 stub (`fn(&Content)->Content`, single rule, single
// pass). It mirrors the vanilla machinery measured in lab/typst-original:
//
//   * Recipes live IN the StyleChain (a `Recipe` layer), so they are SCOPED:
//     a recipe only sees the subtree it wraps (vanilla `StyledElem.styles`).
//   * Recipes are tried INNERMOST-FIRST (vanilla `Entries::next_back` walks
//     the head link in reverse, then moves outward). First matching, non-
//     guarded recipe wins → ONE show step per pass (vanilla `verdict`, the
//     `if step.is_some() { continue }` guard).
//   * Each Content node carries a GUARD set (`Vec<RecipeIndex>`), the toy
//     analogue of vanilla `meta().lifecycle` bitset. When a recipe fires, its
//     output is `.guarded(index)`, so the SAME recipe never re-fires on the
//     produced body → recursion terminates (vanilla `is_guarded` / `guarded`).
//   * A show-SET recipe (`Transformation::Style`) does NOT consume the step;
//     it pushes a `#set` layer for that node's subtree and realization
//     continues (vanilla `if let Transformation::Style(..) => map.apply(..)`).
//
// `RecipeIndex` here is the recipe's stable position in the active chain,
// computed innermost-first exactly like vanilla `RecipeIndex(depth - r)`.

/// Stable identity of a recipe inside the active chain. Two passes over the
/// same chain produce the same index for the same recipe → guards are stable.
pub type RecipeIndex = usize;

/// What a matched recipe does to a node.
pub enum Transformation {
    /// `#show k: it => <content>` — replace the node with arbitrary content.
    /// The closure gets the matched node and returns its replacement.
    Func(Box<dyn Fn(&Content) -> Content>),
    /// `#show k: set <prop>(...)` — a show-SET. Pushes these props as a scoped
    /// `#set k(..)` layer instead of replacing the node.
    Style(StyleLayer),
}

/// A single `#show kind: transform` recipe.
pub struct Recipe {
    pub kind: &'static str,
    pub transform: Transformation,
}

impl Recipe {
    pub fn func(
        kind: &'static str,
        f: impl Fn(&Content) -> Content + 'static,
    ) -> Recipe {
        Recipe { kind, transform: Transformation::Func(Box::new(f)) }
    }
    /// `#show kind: set target(props)` — a show-set recipe.
    pub fn set(
        kind: &'static str,
        target: &'static str,
        props: PropMap,
    ) -> Recipe {
        Recipe {
            kind,
            transform: Transformation::Style(StyleLayer { target, props }),
        }
    }
}

/// The realization environment: the active set-layers AND the active recipes,
/// both scoped. This is the toy StyleChain extended with recipes (vanilla keeps
/// both `Property` and `Recipe` in the same `Style` chain — we split them for
/// readability but keep the same scoping/ordering semantics).
#[derive(Default)]
pub struct ShowChain<'a> {
    /// Set-layers in effect (innermost last), for `render`.
    pub styles: StyleChain,
    /// Recipes in effect, OUTERMOST-FIRST in this Vec. We iterate it REVERSED
    /// so the effective order is innermost-first (vanilla).
    pub recipes: Vec<&'a Recipe>,
}

impl<'a> ShowChain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    /// Enter a scope that adds `recipes` (and optionally set-layers). The added
    /// recipes are visible ONLY while this chain value is alive → scoping.
    pub fn scoped(&self, recipes: &'a [Recipe]) -> ShowChain<'a> {
        let mut child = ShowChain { styles: self.styles.clone(), recipes: self.recipes.clone() };
        for r in recipes {
            child.recipes.push(r);
        }
        child
    }
}

/// Per-pass realization: walk the tree, applying AT MOST ONE recipe step per
/// node per pass, bottom-up, re-realizing produced content (multi-pass) until a
/// fixpoint. Guards on nodes make it terminate.
///
/// Returns the realized tree. `max_passes` bounds runaway (it should never be
/// hit once guards work — we assert it isn't, to PROVE termination).
pub fn realize<'a>(root: &Content, chain: &ShowChain<'a>, max_passes: usize) -> (Content, usize) {
    let mut current = root.clone();
    let mut passes = 0;
    loop {
        passes += 1;
        let (next, changed) = realize_pass(&current, chain);
        current = next;
        if !changed {
            break;
        }
        assert!(passes < max_passes, "show realization did not converge — guards failed");
    }
    (current, passes)
}

/// One realization pass: recurse into children first (bottom-up), then try to
/// apply ONE recipe to THIS node. Returns (rebuilt, changed_this_pass).
///
/// Crucially: a recipe is applied to a node AT MOST ONCE per call. The
/// `Guarded` carrier is transparent — we descend into its children but the
/// guard set stays attached to the node so `apply_recipes` can see it. This is
/// what makes recursion terminate.
fn realize_pass<'a>(node: &Content, chain: &ShowChain<'a>) -> (Content, bool) {
    // A `Styled` (show-set) wrapper is realized by recursing its child under the
    // same recipe chain; the wrapper itself never matches a recipe.
    if let Content::Styled { layers, child } = node {
        let (r, ch) = realize_pass(child, chain);
        return (Content::Styled { layers: layers.clone(), child: Arc::new(r) }, ch);
    }

    // Split the node into (guards, bare). Guards travel with the node; we rebuild
    // the bare node's CHILDREN (bottom-up) WITHOUT applying a recipe to the bare
    // node itself, then run `apply_recipes` exactly once on the guarded node.
    let (guards, bare): (Vec<RecipeIndex>, &Content) = match node {
        Content::Guarded { guards, inner } => (guards.clone(), inner.as_ref()),
        other => (Vec::new(), other),
    };

    let (rebuilt_bare, child_changed) = realize_children(bare, chain);

    // Re-attach the guards before trying a recipe on this node.
    let guarded_node = if guards.is_empty() {
        rebuilt_bare
    } else {
        Content::Guarded { guards, inner: Arc::new(rebuilt_bare) }
    };

    apply_recipes(guarded_node, child_changed, chain)
}

/// Rebuild a node's children (recursively realizing them) but do NOT apply any
/// recipe to `node` itself. `node` here is always a BARE node (no outer guard).
fn realize_children<'a>(node: &Content, chain: &ShowChain<'a>) -> (Content, bool) {
    match node {
        Content::Seq(items) => {
            let mut any = false;
            let mut out = Vec::with_capacity(items.len());
            for c in items.iter() {
                let (r, ch) = realize_pass(c, chain);
                any |= ch;
                out.push(r);
            }
            (Content::seq(out), any)
        }
        Content::Native(n) => match &**n {
            NativeElem::Heading { level, body } => {
                let (b, ch) = realize_pass(body, chain);
                (Content::heading(*level, b), ch)
            }
            NativeElem::Text(_) => (node.clone(), false),
        },
        // Dynamic leaves: the spike's callout body is realized via children().
        // For the toy we treat dynamic children as already-realized (the demo
        // callouts hold plain text); descending would require a with_children
        // on the trait. Recorded as a measurement limit (see findings).
        Content::Dynamic(_) => (node.clone(), false),
        // A bare node is never Guarded/Styled (those are peeled above).
        Content::Guarded { .. } | Content::Styled { .. } => (node.clone(), false),
    }
}

/// Step 2+3 of a pass: try ONE recipe on `rebuilt`, innermost-first.
fn apply_recipes<'a>(
    rebuilt: Content,
    child_changed: bool,
    chain: &ShowChain<'a>,
) -> (Content, bool) {

    // 2) Try recipes on THIS node, innermost-first; first matching, non-guarded,
    //    non-show-set recipe fires (one step). Show-set recipes accumulate a
    //    set-layer for re-rendering but do NOT count as the step.
    let kind = rebuilt.kind();
    let depth = chain.recipes.len();
    let mut set_layers: Vec<StyleLayer> = Vec::new();
    // innermost-first = reversed Vec (Vec holds outermost-first)
    for (i, recipe) in chain.recipes.iter().enumerate().rev() {
        if recipe.kind != kind {
            continue;
        }
        // Stable recipe index, innermost-first: vanilla RecipeIndex(depth - r).
        let index: RecipeIndex = depth - i;
        match &recipe.transform {
            Transformation::Style(layer) => {
                // show-set: record the layer (applies to this node's render).
                set_layers.push(layer.clone());
            }
            Transformation::Func(f) => {
                if guards_of(&rebuilt).contains(&index) {
                    continue; // already applied to this node → skip (termination)
                }
                // Fire: guard the INPUT so the recipe can't re-fire on it
                // (vanilla `output.into_owned().guarded(guard)`), then transform.
                let guarded_input = push_guard(&rebuilt, index);
                let produced = f(&guarded_input);
                // Carry the INPUT's full guard set forward onto every same-kind
                // node of the produced body, PLUS this recipe's index. Vanilla
                // gets accumulation for free (the guarded input is nested inside
                // the output); the spike re-applies it structurally so that
                // already-applied recipes stay applied → multi-rule converges
                // and self-producing recipes terminate.
                let mut carry = guards_of(&rebuilt).to_vec();
                if !carry.contains(&index) {
                    carry.push(index);
                }
                let mut produced = produced;
                for g in carry {
                    produced = guard_kind(&produced, recipe.kind, g);
                }
                return (produced, true);
            }
        }
    }

    // 3) No func step. If we collected show-set layers, attach them so that this
    //    node renders under them. Toy: stash them as guards-free wrapper via a
    //    Seq carrying a marker is overkill; instead we fold them into the node's
    //    own props by wrapping in a Styled marker. For the spike we render with
    //    an augmented chain (handled by `render_with_sets`), so we attach them
    //    onto a lightweight wrapper.
    if !set_layers.is_empty() {
        return (Content::styled(rebuilt, set_layers), child_changed);
    }

    (rebuilt, child_changed)
}

// --- guard plumbing (toy analogue of meta().lifecycle bitset) ---
//
// We can't store guards inside Arc<NativeElem>/Arc<dyn Element> without growing
// the trait, so the spike keeps a SIDE map keyed by a per-node tag. To stay
// simple and self-contained, we model guards as a wrapper variant. We thread
// them via `Content::Guarded`. (In product L1 this would be the meta bitset on
// the packed element, NOT a wrapper — see findings S2.)

fn guards_of(_c: &Content) -> &[RecipeIndex] {
    // guards live on Content::Guarded; for non-guarded nodes, empty.
    match _c {
        Content::Guarded { guards, .. } => guards,
        _ => &[],
    }
}

fn push_guard(c: &Content, index: RecipeIndex) -> Content {
    match c {
        Content::Guarded { guards, inner } => {
            let mut g = guards.clone();
            if !g.contains(&index) {
                g.push(index);
            }
            Content::Guarded { guards: g, inner: inner.clone() }
        }
        other => Content::Guarded {
            guards: vec![index],
            inner: Arc::new(other.clone()),
        },
    }
}

/// Add `index` to the guard set of EVERY node whose kind == `kind` in `c`
/// (recursively). Terminates recursion for self-producing recipes.
fn guard_kind(c: &Content, kind: &str, index: RecipeIndex) -> Content {
    // First recurse into children, rebuilding the tree.
    let rebuilt = match c {
        Content::Seq(items) => {
            Content::seq(items.iter().map(|x| guard_kind(x, kind, index)).collect())
        }
        Content::Native(n) => match &**n {
            NativeElem::Heading { level, body } => {
                Content::heading(*level, guard_kind(body, kind, index))
            }
            NativeElem::Text(_) => c.clone(),
        },
        Content::Dynamic(_) => c.clone(),
        Content::Guarded { guards, inner } => Content::Guarded {
            guards: guards.clone(),
            inner: Arc::new(guard_kind(inner, kind, index)),
        },
        Content::Styled { layers, child } => Content::Styled {
            layers: layers.clone(),
            child: Arc::new(guard_kind(child, kind, index)),
        },
    };
    // Then, if THIS node matches, ensure the guard is present.
    if rebuilt.kind() == kind {
        push_guard(&rebuilt, index)
    } else {
        rebuilt
    }
}
