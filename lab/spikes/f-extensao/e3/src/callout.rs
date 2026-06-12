//! Third-party user element `callout { body, title, tone }`.
//!
//! Defined OUTSIDE the toy core. It implements NO trait on the data — it just
//! builds an `ElementDescriptor` (props schema + render fn + default show) and
//! registers it. Dispatch back into `callout` happens by the core's table
//! lookup. This module imports only PUBLIC core types.

use crate::core::{Chain, Content, ElementDescriptor, KindId, PropKey, PropMap, Registry, Value};

// The user mints PropKeys freely — NO core edit required to add these.
pub const BODY: PropKey = PropKey("body");
pub const TITLE: PropKey = PropKey("title");
pub const TONE: PropKey = PropKey("tone");

/// The layout fn for callout — explicit dispatch target. Shows the tone.
fn render_callout(props: &PropMap, reg: &Registry, chain: &Chain) -> String {
    // `tone` resolves through fallback: local prop, else scoped `#set` default.
    let tone = props
        .resolve(TONE, chain)
        .and_then(Value::as_str)
        .unwrap_or("info");
    let title = props
        .resolve(TITLE, chain)
        .and_then(Value::as_str)
        .unwrap_or("");
    let body = match props.resolve(BODY, chain).and_then(Value::as_content) {
        // Nested Content rendered via the SAME registry (table dispatch).
        Some(c) => reg_render(reg, c),
        None => String::new(),
    };
    format!("[callout tone={tone} title={title}]{body}[/callout]")
}

// Render nested content. The registry's `render` needs a &mut Chain; for nested
// callout bodies we use a throwaway chain (the body's own scope). In the toy,
// bodies are plain Text so no fallback is needed.
fn reg_render(reg: &Registry, c: &Content) -> String {
    let mut throwaway = Chain::new();
    reg.render(c, &mut throwaway)
}

/// Default show rule: identity (no rewrite). Real impl could inject an icon.
fn show_default(props: &PropMap) -> PropMap {
    props.clone()
}

/// Register the callout descriptor; returns its KindId for the rest of the app.
pub fn register(reg: &mut Registry) -> KindId {
    reg.register(ElementDescriptor {
        name: "callout",
        props: vec![
            (TONE, Value::Str("info".into())), // schema default
            (TITLE, Value::Str("".into())),
        ],
        layout_fn: render_callout,
        show_default,
    })
}

/// Constructor helper — builds a callout Content (pure DATA).
pub fn make(kind: KindId, title: &str, tone: &str, body: Content) -> Content {
    let mut props = PropMap::new();
    props.set(TITLE, Value::Str(title.into()));
    props.set(TONE, Value::Str(tone.into()));
    props.set(BODY, Value::Content(body));
    Content::Dynamic { kind, props }
}
