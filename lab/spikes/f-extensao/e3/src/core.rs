//! Toy "core" for design E3 — Registro aberto por kind (B destravada, SEM dyn).
//!
//! The data model (`Content`) is a CLOSED enum that carries NO trait objects.
//! Third-party elements enter exclusively through the `Dynamic { kind_id, props }`
//! variant: pure DATA. Behaviour (props schema, layout fn, default show) lives in
//! an `ElementDescriptor` kept OUTSIDE the Content, in a `Registry` table keyed by
//! `kind_id`. Dispatch is an explicit table lookup — an injected vtable-by-data —
//! never a `dyn` method call on the Content itself.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Value — CLOSED enum. This is THE central closure of E3: a user CANNOT add a
// new Value *variant* (e.g. a Color, a Length) without editing this file.
// ---------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub enum Value {
    None,
    Str(String),
    Int(i64),
    Content(Content),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_content(&self) -> Option<&Content> {
        match self {
            Value::Content(c) => Some(c),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// PropKey — OPEN keyed identifier. A user mints new keys freely (no core edit).
// Backed by a static &str; equality/hash are by interned string. This is the
// "open" half of the open-keyed-typed PropMap: keys open, Values closed.
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PropKey(pub &'static str);

// ---------------------------------------------------------------------------
// PropMap — open map from PropKey to (closed) Value, with a fallback chain.
// `chain` points at scoped defaults (e.g. from `#set`). Resolution is local
// entry first, then walk the chain.
// ---------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
pub struct PropMap {
    map: HashMap<PropKey, Value>,
}

impl PropMap {
    pub fn new() -> Self {
        PropMap { map: HashMap::new() }
    }
    pub fn set(&mut self, key: PropKey, val: Value) {
        self.map.insert(key, val);
    }
    pub fn get(&self, key: PropKey) -> Option<&Value> {
        self.map.get(&key)
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Fallback resolution: this map first, then the scoped-default chain.
    pub fn resolve<'a>(&'a self, key: PropKey, chain: &'a Chain) -> Option<&'a Value> {
        if let Some(v) = self.map.get(&key) {
            return Some(v);
        }
        chain.lookup(key)
    }
}

// ---------------------------------------------------------------------------
// Chain — scoped defaults pushed by `#set`. Keyed by (kind_id, PropKey).
// A linked stack so `#set` can be scoped/popped. Lookup walks outward.
// ---------------------------------------------------------------------------
pub struct Chain {
    // (kind_id, key) -> value ; innermost scope wins via last-write in a single map
    // for the toy. (A real impl would be a cons-list of scopes; here one flat map
    // is enough to demonstrate fallback.)
    scoped: HashMap<(KindId, PropKey), Value>,
    // The kind currently being resolved, set transiently during resolve.
    current_kind: KindId,
}

impl Chain {
    pub fn new() -> Self {
        Chain { scoped: HashMap::new(), current_kind: KindId(usize::MAX) }
    }
    /// `#set callout(tone: "warn")` writes a scoped default here.
    pub fn set_default(&mut self, kind: KindId, key: PropKey, val: Value) {
        self.scoped.insert((kind, key), val);
    }
    fn lookup(&self, key: PropKey) -> Option<&Value> {
        self.scoped.get(&(self.current_kind, key))
    }
    fn enter(&mut self, kind: KindId) {
        self.current_kind = kind;
    }
}

// ---------------------------------------------------------------------------
// KindId — dynamic registered id for an element kind. Native kinds get fixed
// ids; third-party kinds get ids handed out by the registry at registration.
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KindId(pub usize);

// ---------------------------------------------------------------------------
// Content — CLOSED enum, NO dyn. Native elements are explicit variants; every
// third-party element is `Dynamic` carrying only (kind_id, PropMap) DATA.
// ---------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub enum Content {
    Text(String),
    Sequence(Vec<Content>),
    /// The single extension point. DATA ONLY — no trait object.
    Dynamic { kind: KindId, props: PropMap },
}

impl Content {
    pub fn kind(&self) -> KindId {
        match self {
            Content::Text(_) => NATIVE_TEXT,
            Content::Sequence(_) => NATIVE_SEQ,
            Content::Dynamic { kind, .. } => *kind,
        }
    }
}

// Fixed ids for native kinds.
pub const NATIVE_TEXT: KindId = KindId(0);
pub const NATIVE_SEQ: KindId = KindId(1);
pub const FIRST_DYNAMIC_ID: usize = 2;

// ---------------------------------------------------------------------------
// ElementDescriptor — the vtable-by-data. Behaviour for a kind, kept OUTSIDE
// the Content. `layout_fn` and `show_default` are plain fn pointers (closed
// over no state) — explicit dispatch, no dyn on the model.
// ---------------------------------------------------------------------------
pub struct ElementDescriptor {
    pub name: &'static str,
    /// Declared prop schema (key + default). Used by `#set`/resolution docs.
    pub props: Vec<(PropKey, Value)>,
    /// Renders a Dynamic Content of this kind to a string, given the registry
    /// (for nested content) and the chain (for fallback defaults).
    pub layout_fn: fn(&PropMap, &Registry, &Chain) -> String,
    /// Default show rule (pre-render transform). Returns possibly-rewritten
    /// props; here it's identity unless overridden.
    pub show_default: fn(&PropMap) -> PropMap,
}

// ---------------------------------------------------------------------------
// Registry — kind_id -> ElementDescriptor. Injected, lives outside Content.
// Hands out fresh KindIds for third-party registrations.
// ---------------------------------------------------------------------------
pub struct Registry {
    descriptors: HashMap<KindId, ElementDescriptor>,
    next_id: usize,
    /// Optional `#show` overrides: kind -> replacement layout fn.
    show_overrides: HashMap<KindId, fn(&PropMap, &Registry, &Chain) -> String>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            descriptors: HashMap::new(),
            next_id: FIRST_DYNAMIC_ID,
            show_overrides: HashMap::new(),
        }
    }

    /// Register a third-party element; returns its freshly minted KindId.
    pub fn register(&mut self, desc: ElementDescriptor) -> KindId {
        let id = KindId(self.next_id);
        self.next_id += 1;
        self.descriptors.insert(id, desc);
        id
    }

    pub fn descriptor(&self, kind: KindId) -> Option<&ElementDescriptor> {
        self.descriptors.get(&kind)
    }

    /// `#show callout: ...` — override the layout fn for a kind.
    pub fn set_show_override(
        &mut self,
        kind: KindId,
        f: fn(&PropMap, &Registry, &Chain) -> String,
    ) {
        self.show_overrides.insert(kind, f);
    }

    /// Render any Content. Explicit table dispatch for Dynamic — NO dyn.
    pub fn render(&self, c: &Content, chain: &mut Chain) -> String {
        match c {
            Content::Text(s) => s.clone(),
            Content::Sequence(items) => {
                let mut out = String::new();
                for it in items {
                    out.push_str(&self.render(it, chain));
                }
                out
            }
            Content::Dynamic { kind, props } => {
                // Set the scope for fallback resolution.
                chain.enter(*kind);
                // `#show` override wins over the descriptor's layout_fn.
                if let Some(f) = self.show_overrides.get(kind) {
                    return f(props, self, chain);
                }
                match self.descriptors.get(kind) {
                    Some(desc) => {
                        let shown = (desc.show_default)(props);
                        (desc.layout_fn)(&shown, self, chain)
                    }
                    // Unknown kind: graceful stub (see STUB note in main.rs).
                    None => format!("<?unknown kind {}>", kind.0),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// query — collect all Content nodes of a given kind_id (like `query(callout)`).
// ---------------------------------------------------------------------------
pub fn query<'a>(root: &'a Content, kind: KindId, out: &mut Vec<&'a Content>) {
    match root {
        Content::Sequence(items) => {
            for it in items {
                query(it, kind, out);
            }
        }
        other => {
            if other.kind() == kind {
                out.push(other);
            }
        }
    }
}
