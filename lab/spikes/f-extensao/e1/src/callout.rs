//! THIRD-PARTY user element `callout`. Lives OUTSIDE the toy core.
//!
//! It imports ONLY the public boundary surface — the `Element` trait and the
//! public value/style types. It NEVER touches core internals (the Content
//! match arms, NativeElem, the registry table). This file is the entire cost
//! of adding a new element in design E1.
//!
//! callout { body, title, tone }
//!   - body:  Content   (child)
//!   - title: String    (own prop, also queryable)
//!   - tone:  String    (own prop; resolved from instance OR #set chain)

use std::sync::Arc;

use crate::core::{Content, Element, PropMap, StyleChain, Value};

#[derive(Clone, Debug)]
pub struct Callout {
    pub title: String,
    pub body: Content,
    pub tone: String,
}

impl Callout {
    pub const KIND: &'static str = "callout";

    pub fn new(title: impl Into<String>, body: Content, tone: impl Into<String>) -> Self {
        Callout {
            title: title.into(),
            body,
            tone: tone.into(),
        }
    }

    /// Registry constructor: build from an own-prop map.
    pub fn construct(props: PropMap) -> Arc<dyn Element> {
        let title = props
            .get("title")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        let tone = props
            .get("tone")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_else(|| "note".to_string());
        let body = match props.get("body") {
            Some(Value::Content(c)) => c,
            _ => Content::text(""),
        };
        Arc::new(Callout::new(title, body, tone))
    }
}

impl Element for Callout {
    fn kind(&self) -> &'static str {
        Callout::KIND
    }

    fn plain_text(&self) -> String {
        if self.title.is_empty() {
            self.body.plain_text()
        } else {
            format!("{}: {}", self.title, self.body.plain_text())
        }
    }

    fn render(&self, chain: &StyleChain) -> String {
        // Resolve `tone`: instance value, else a `#set callout(tone: ...)` layer.
        let tone = chain
            .get(Callout::KIND, "tone")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_else(|| self.tone.clone());
        format!(
            "[{}|{}] {}",
            tone.to_uppercase(),
            self.title,
            self.body.render(chain)
        )
    }

    fn get_prop(&self, key: &str) -> Option<Value> {
        match key {
            "tone" => Some(Value::Str(self.tone.clone())),
            "title" => Some(Value::Str(self.title.clone())),
            "body" => Some(Value::Content(self.body.clone())),
            _ => None,
        }
    }

    fn with_prop(&self, key: &str, val: Value) -> Arc<dyn Element> {
        let mut c = self.clone();
        match (key, val) {
            ("tone", Value::Str(s)) => c.tone = s,
            ("title", Value::Str(s)) => c.title = s,
            ("body", Value::Content(b)) => c.body = b,
            _ => {}
        }
        Arc::new(c)
    }

    fn children(&self) -> Vec<Content> {
        vec![self.body.clone()]
    }
}
