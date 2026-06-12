//! THIRD-PARTY user element `callout { body, title, tone }`.
//!
//! Defined OUTSIDE the toy "core". It imports ONLY public core types — no
//! access to core internals. This is the whole "extension" story: can a third
//! party add an element + its props + set/show/query + render, touching the
//! core ZERO times?
//!
//! Props of callout, by the (ElementId, PropId) convention. Each element numbers
//! its own props from 0. These constants ARE the contract a `#set`/`#show`
//! author must know.

use crate::core::{
    Chain, Content, ElementId, PropId, Registry, RenderTable, Style, UserElem,
};

/// callout's own props. PropId is unique *within* callout.
pub const PROP_TITLE: PropId = PropId(0);
pub const PROP_TONE: PropId = PropId(1); // tone: String, callout's own prop.

/// Defaults the element ships with (used when the chain resolves nothing).
const DEFAULT_TONE: &str = "note";
const DEFAULT_TITLE: &str = "";

/// Registers the callout element: allocates its ElementId in the registry and
/// installs its render hook. Returns the ElementId so the doc can build/query.
pub fn register(reg: &mut Registry, rt: &mut RenderTable) -> ElementId {
    let id = reg.register("callout");

    // The render hook. It resolves `tone`/`title` by FALLBACK through the chain
    // (innermost #set wins), falling back to the instance field, then defaults.
    // Then it applies any #show recipe. This is the end-to-end resolution.
    rt.register(
        id,
        Box::new(move |u: &UserElem, reg: &Registry, chain: &Chain, rt: &RenderTable| {
            // tone: chain #set override -> instance field -> default.
            let tone: String = chain
                .resolve_property::<String>(id, PROP_TONE)
                .or_else(|| u.field::<String>(PROP_TONE))
                .unwrap_or_else(|| DEFAULT_TONE.to_string());

            let title: String = chain
                .resolve_property::<String>(id, PROP_TITLE)
                .or_else(|| u.field::<String>(PROP_TITLE))
                .unwrap_or_else(|| DEFAULT_TITLE.to_string());

            // body is rendered via the core (STUB: real layout omitted).
            let body = crate::core::render(&u.body, reg, chain, rt);

            let base = if title.is_empty() {
                format!("[{}] {}", tone, body)
            } else {
                format!("[{}: {}] {}", tone, title, body)
            };

            // Apply a #show recipe if present (transforms the rendered string).
            match chain.resolve_recipe(id) {
                Some(recipe) => recipe(&base),
                None => base,
            }
        }),
    );

    id
}

/// Constructor a third party offers its users: `callout(body, title, tone)`.
/// Inline fields are stored type-erased as Box<dyn Any> under callout's PropIds.
pub fn callout(
    id: ElementId,
    body: Content,
    title: Option<&str>,
    tone: Option<&str>,
) -> Content {
    let mut fields: Vec<(PropId, Box<dyn std::any::Any>)> = Vec::new();
    if let Some(t) = title {
        fields.push((PROP_TITLE, Box::new(t.to_string())));
    }
    if let Some(t) = tone {
        fields.push((PROP_TONE, Box::new(t.to_string())));
    }
    Content::User(UserElem {
        elem: id,
        body: Box::new(body),
        fields,
    })
}

/// `#set callout(tone: ...)` => push a type-erased Property onto the chain.
pub fn set_tone(id: ElementId, tone: &str) -> Style {
    Style::Property {
        elem: id,
        prop: PROP_TONE,
        value: Box::new(tone.to_string()),
    }
}

/// `#show callout: ...` (minimal) => a Recipe transforming the rendered body.
pub fn show_uppercase(id: ElementId) -> Style {
    Style::Recipe {
        elem: id,
        recipe: Box::new(|s: &str| s.to_uppercase()),
    }
}
