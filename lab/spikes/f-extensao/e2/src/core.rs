//! Toy "core" for spike E2 — type-erased à vanilla (C honesta, mínima).
//!
//! This mirrors vanilla typst's *structure* in minimal form:
//!  - A `Content` model (a couple native elements + user elements).
//!  - A type-erased style `Chain` of `enum Style { Property | Recipe }`,
//!    keyed by `(ElementId, PropId)`.
//!  - An element registry mapping `ElementId -> metadata`.
//!  - Property access via downcast on `Box<dyn Any>`.
//!  - Resolution by FALLBACK: walk innermost -> outermost, first match wins.
//!
//! The whole point is to measure the REAL cost of dyn/type-erasure (vtable +
//! downcast on every read), not to reject it by impression.

use std::any::Any;

// ---------------------------------------------------------------------------
// Ids. Convention copied from vanilla in minimal form:
//   - ElementId: a unique integer per element *kind*, allocated at registration.
//   - PropId:    a unique integer per *property slot* of an element. Convention:
//                each element numbers its own props from 0; the chain key is the
//                PAIR (ElementId, PropId), so PropId need only be unique *within*
//                an element. Third parties must follow this same convention.
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ElementId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PropId(pub u16);

/// Metadata the registry holds per element kind.
pub struct ElementMeta {
    pub id: ElementId,
    pub name: &'static str,
}

/// Registry: ElementId -> metadata. In real typst this is a global inventory;
/// here it's a plain owned table threaded through the doc explicitly (L1-pure
/// shape: no global mutable state). Ids are allocated sequentially.
#[derive(Default)]
pub struct Registry {
    metas: Vec<ElementMeta>,
}

impl Registry {
    pub fn new() -> Self {
        Self { metas: Vec::new() }
    }

    /// Allocate a fresh ElementId and store metadata. Sequential allocation —
    /// the returned id equals the index, but callers must treat it as opaque.
    pub fn register(&mut self, name: &'static str) -> ElementId {
        let id = ElementId(self.metas.len() as u32);
        self.metas.push(ElementMeta { id, name });
        id
    }

    pub fn name_of(&self, id: ElementId) -> &'static str {
        self.metas[id.0 as usize].name
    }
}

// ---------------------------------------------------------------------------
// The type-erased style chain — the heart of E2.
// ---------------------------------------------------------------------------

/// A recipe is a transform applied at show time. Type-erased as a boxed Fn.
/// Vanilla erases the element type the same way (dyn Fn over Content).
pub type RecipeFn = Box<dyn Fn(&str) -> String>;

/// One style entry. Either a fixed property value (type-erased) or a recipe.
/// This is vanilla's `Style { Property, Recipe }` split, minimized.
pub enum Style {
    /// `#set callout(tone: "warn")` => a Property override for (elem, prop).
    Property {
        elem: ElementId,
        prop: PropId,
        // INVARIANT (non-local, must be respected by every reader):
        // the dynamic type behind this box is exactly the declared Rust type of
        // (elem, prop). Readers downcast to that type. A wrong downcast yields
        // None (we never `unwrap` blindly — see `resolve_*`).
        value: Box<dyn Any>,
    },
    /// `#show callout: ...` => a recipe keyed by element kind.
    Recipe { elem: ElementId, recipe: RecipeFn },
}

/// The chain. Innermost styles are pushed LAST; resolution walks from the end
/// (innermost) toward the front (outermost) and takes the FIRST match — vanilla
/// fallback semantics.
#[derive(Default)]
pub struct Chain {
    entries: Vec<Style>,
}

impl Chain {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Push an inner scope (e.g. a `#set`/`#show`). Most-recent wins.
    pub fn push(&mut self, s: Style) {
        self.entries.push(s);
    }

    /// Resolve a property (elem, prop) to a typed value by fallback.
    /// Walks innermost -> outermost; first Property match that downcasts to T
    /// wins. Returns None if no match (caller supplies the element default).
    ///
    /// DOWNCAST COST: every read pays a vtable type-id compare here. For natives
    /// that have no override in the chain, we still walk every entry testing the
    /// (elem, prop) key — that is the "overhead even for natives" we measure.
    pub fn resolve_property<T: 'static + Clone>(
        &self,
        elem: ElementId,
        prop: PropId,
    ) -> Option<T> {
        for s in self.entries.iter().rev() {
            if let Style::Property { elem: e, prop: p, value } = s {
                if *e == elem && *p == prop {
                    // The downcast. Box<dyn Any> -> &T. None on type mismatch.
                    if let Some(v) = value.downcast_ref::<T>() {
                        return Some(v.clone());
                    }
                }
            }
        }
        None
    }

    /// Resolve the (first/innermost) recipe for an element kind, if any.
    pub fn resolve_recipe(&self, elem: ElementId) -> Option<&RecipeFn> {
        for s in self.entries.iter().rev() {
            if let Style::Recipe { elem: e, recipe } = s {
                if *e == elem {
                    return Some(recipe);
                }
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Content model. A closed enum: native elements + a generic "user" element that
// carries its ElementId and a small field bag. (Vanilla uses dyn Element; here
// the user element is represented uniformly so the core needs ZERO changes to
// host a new third-party element — see report "Atomização".)
// ---------------------------------------------------------------------------

/// A native text run (one of the 1-2 native elements).
pub struct TextElem {
    pub text: String,
}

/// A native sequence (the other native element) — just to have >1 native.
pub struct SeqElem {
    pub children: Vec<Content>,
}

/// A user-defined element instance. The core knows only: its ElementId, its
/// body Content, and a bag of type-erased fields keyed by PropId. The element's
/// own module knows how to interpret these.
pub struct UserElem {
    pub elem: ElementId,
    pub body: Box<Content>,
    /// Inline-set fields on the *instance* (distinct from styled overrides).
    /// (PropId -> type-erased value). Same downcast invariant as the chain.
    pub fields: Vec<(PropId, Box<dyn Any>)>,
}

impl UserElem {
    /// Read an instance field by PropId with downcast. None if absent/mismatch.
    pub fn field<T: 'static + Clone>(&self, prop: PropId) -> Option<T> {
        for (p, v) in &self.fields {
            if *p == prop {
                if let Some(t) = v.downcast_ref::<T>() {
                    return Some(t.clone());
                }
            }
        }
        None
    }
}

pub enum Content {
    Text(TextElem),
    Seq(SeqElem),
    User(UserElem),
}

impl Content {
    pub fn text(s: impl Into<String>) -> Content {
        Content::Text(TextElem { text: s.into() })
    }

    /// ElementId of a Content node, if it is an element with a registered id.
    /// Natives carry no registry id in this toy (they're matched structurally);
    /// users do. This is enough for `query(callout)`.
    pub fn user_id(&self) -> Option<ElementId> {
        match self {
            Content::User(u) => Some(u.elem),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering. The core renders natives directly; for a user element it delegates
// to a render hook registered by the element's module. The hook receives the
// instance, the registry, and the style chain so it can resolve props/recipe.
//
// STUB: in real typst this is the layout/eval pipeline. Here "render" = produce
// a String. Declared as a stub.
// ---------------------------------------------------------------------------

/// A render hook: how to turn a user element instance into a String, given the
/// registry, the active style chain, and the render table (so it can recurse
/// into its body, which may contain other user elements). Registered per id.
pub type RenderHook = Box<dyn Fn(&UserElem, &Registry, &Chain, &RenderTable) -> String>;

#[derive(Default)]
pub struct RenderTable {
    hooks: Vec<(ElementId, RenderHook)>,
}

impl RenderTable {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
    pub fn register(&mut self, elem: ElementId, hook: RenderHook) {
        self.hooks.push((elem, hook));
    }
    fn hook(&self, elem: ElementId) -> Option<&RenderHook> {
        self.hooks.iter().find(|(e, _)| *e == elem).map(|(_, h)| h)
    }
}

/// Render any Content to a String. Natives inline; users via their hook.
pub fn render(c: &Content, reg: &Registry, chain: &Chain, rt: &RenderTable) -> String {
    match c {
        Content::Text(t) => t.text.clone(),
        Content::Seq(s) => {
            let mut out = String::new();
            for ch in &s.children {
                out.push_str(&render(ch, reg, chain, rt));
            }
            out
        }
        Content::User(u) => match rt.hook(u.elem) {
            Some(h) => h(u, reg, chain, rt),
            // STUB: unknown user element with no hook -> placeholder.
            None => format!("<unrendered:{}>", reg.name_of(u.elem)),
        },
    }
}

/// query(elem): collect references to all instances of an ElementId in a tree.
pub fn query<'a>(root: &'a Content, target: ElementId, out: &mut Vec<&'a UserElem>) {
    match root {
        Content::User(u) => {
            if u.elem == target {
                out.push(u);
            }
            query(&u.body, target, out);
        }
        Content::Seq(s) => {
            for ch in &s.children {
                query(ch, target, out);
            }
        }
        Content::Text(_) => {}
    }
}
