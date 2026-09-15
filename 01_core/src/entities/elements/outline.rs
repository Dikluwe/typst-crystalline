//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline.md
//! @prompt-hash 6aa40f8a
//! @layer L1
//! @updated 2026-06-11
//!
//! `OutlineElem` — Lote 8 P323 (por largura). `outline()` — índice do documento.
//! P457: expande para `{ title, depth, indent }` (campos settable do vanilla).
//! **Locatável** (queryable desde P178) — absorve `extract_payload`.

use std::hash::Hash;
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::layout_types::Length;
use crate::entities::show::ShowRule;
use crate::entities::source_result::SourceResult;
use crate::entities::style::Styles;

/// Alvo do índice: headings (TOC), figures (LoF) ou tables (LoT).
///
/// **P472** — `OutlineTarget` distingue os três tipos de lista gerados
/// por `outline()`, `lof()` e `lot()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutlineTarget {
    #[default]
    Headings,
    Figures,
    Tables,
}

/// **P502** — Valor aceite por `outline(indent:)`.
/// Vanilla 0.14.2/0.15.0: `length | function | auto`.
/// Cristalino mantém `bool` para compatibilidade reversa.
#[derive(Debug, Clone, PartialEq)]
pub enum OutlineIndent {
    Auto,
    Bool(bool),
    Length(Length),
    Function(Func),
}

impl std::hash::Hash for OutlineIndent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Function e Length não implementam Hash; usa representação Debug
        // (paridade content_hash; OutlineIndent nunca é chave de mapa).
        format!("{self:?}").hash(state);
    }
}

impl OutlineIndent {
    /// Indica se a indentação está activa no layout actual.
    /// `Length`/`Function` são aceites mas tratados como `Auto` até
    /// a renderização específica ser implementada (scope-out P502).
    pub fn is_active(&self) -> bool {
        match self {
            OutlineIndent::Auto => true,
            OutlineIndent::Bool(b) => *b,
            OutlineIndent::Length(_) | OutlineIndent::Function(_) => true,
        }
    }
}

/// Marcador de índice (table of contents / list of figures / list of tables).
#[derive(Debug, Clone)]
pub struct OutlineElem {
    pub title: Option<Content>,
    pub depth: usize,
    pub indent: OutlineIndent,
    /// **P472** — alvo do índice. Default `Headings` (TOC normal).
    pub target: OutlineTarget,
    title_set: bool,
    show_rules: Arc<[ShowRule]>,
    captured_styles: Styles,
    parent_key: i64,
    realized_body: Option<Content>,
}

impl PartialEq for OutlineElem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.depth == other.depth
            && self.indent == other.indent
            && self.target == other.target
            && self.title_set == other.title_set
            && self.captured_styles == other.captured_styles
            && self
                .show_rules
                .iter()
                .map(|rule| rule.id)
                .eq(other.show_rules.iter().map(|rule| rule.id))
            && self.realized_body == other.realized_body
    }
}

impl std::hash::Hash for OutlineElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.depth.hash(state);
        self.indent.hash(state);
        self.target.hash(state);
        self.title_set.hash(state);
        format!("{:?}", self.captured_styles).hash(state);
        for rule in self.show_rules.iter() {
            rule.id.hash(state);
        }
        self.realized_body.hash(state);
    }
}

impl OutlineElem {
    /// Defaults do vanilla: sem título customizado, profundidade 3, indentação auto.
    pub fn new(title: Option<Content>, depth: usize, indent: OutlineIndent) -> Self {
        Self {
            title,
            depth,
            indent,
            target: OutlineTarget::Headings,
            title_set: false,
            show_rules: Arc::from([]),
            captured_styles: Styles::new(),
            parent_key: 0,
            realized_body: None,
        }
    }

    /// **P472** — constrói com `target` explícito.
    pub fn with_target(
        title: Option<Content>,
        depth: usize,
        indent: OutlineIndent,
        target: OutlineTarget,
    ) -> Self {
        Self {
            title,
            depth,
            indent,
            target,
            title_set: false,
            show_rules: Arc::from([]),
            captured_styles: Styles::new(),
            parent_key: 0,
            realized_body: None,
        }
    }

    pub fn with_target_and_title_presence(
        title: Option<Content>,
        title_set: bool,
        depth: usize,
        indent: OutlineIndent,
        target: OutlineTarget,
    ) -> Self {
        let mut elem = Self::with_target(title, depth, indent, target);
        elem.title_set = title_set;
        elem
    }

    pub fn with_realization_context(
        &self,
        show_rules: Arc<[ShowRule]>,
        captured_styles: Styles,
    ) -> Self {
        use std::hash::{Hash, Hasher};

        let mut elem = self.clone();
        elem.show_rules = show_rules;
        elem.captured_styles = captured_styles;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        elem.title.hash(&mut hasher);
        elem.depth.hash(&mut hasher);
        elem.indent.hash(&mut hasher);
        elem.target.hash(&mut hasher);
        elem.title_set.hash(&mut hasher);
        format!("{:?}", elem.captured_styles).hash(&mut hasher);
        for rule in elem.show_rules.iter() {
            rule.id.hash(&mut hasher);
        }
        elem.parent_key = hasher.finish() as i64;
        elem
    }

    pub fn title_was_set(&self) -> bool {
        self.title_set
    }

    pub fn show_rules(&self) -> &[ShowRule] {
        &self.show_rules
    }

    pub fn captured_styles(&self) -> &Styles {
        &self.captured_styles
    }

    pub fn parent_key(&self) -> i64 {
        self.parent_key
    }

    pub fn with_realized_body(&self, body: Content) -> Self {
        let mut elem = self.clone();
        elem.realized_body = Some(body);
        elem
    }

    pub fn realized_body(&self) -> Option<&Content> {
        self.realized_body.as_ref()
    }
}

impl Element for OutlineElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Outline(Arc::new(OutlineElem {
            title: self.title.as_ref().map(|c| c.map_content(transform)).transpose()?,
            depth: self.depth,
            indent: self.indent.clone(),
            target: self.target,
            title_set: self.title_set,
            show_rules: self.show_rules.clone(),
            captured_styles: self.captured_styles.clone(),
            parent_key: self.parent_key,
            realized_body: self
                .realized_body
                .as_ref()
                .map(|body| body.map_content(transform))
                .transpose()?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Outline(Arc::new(OutlineElem {
            title: self.title.as_ref().map(|c| c.map_text(transform)),
            depth: self.depth,
            indent: self.indent.clone(),
            target: self.target,
            title_set: self.title_set,
            show_rules: self.show_rules.clone(),
            captured_styles: self.captured_styles.clone(),
            parent_key: self.parent_key,
            realized_body: self
                .realized_body
                .as_ref()
                .map(|body| body.map_text(transform)),
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Outline)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Outline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(OutlineElem::new(None, 3, OutlineIndent::Auto).plain_text(), "");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!OutlineElem::new(None, 3, OutlineIndent::Auto).is_empty());
    }

    #[test]
    fn igualdade_singleton() {
        assert_eq!(
            OutlineElem::new(Some(Content::text("A")), 2, OutlineIndent::Bool(true)),
            OutlineElem::new(Some(Content::text("A")), 2, OutlineIndent::Bool(true))
        );
        assert_ne!(
            OutlineElem::new(None, 3, OutlineIndent::Auto),
            OutlineElem::new(None, 2, OutlineIndent::Auto)
        );
    }

    #[test]
    fn map_content_terminal_preserva_campos() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let elem =
            OutlineElem::new(Some(Content::text("T")), 3, OutlineIndent::Bool(true));
        match elem.map_content(&mut f).unwrap() {
            Content::Outline(e) => {
                assert_eq!(
                    e.title.as_ref().map(|c| c.plain_text()),
                    Some("T".to_string())
                );
                assert_eq!(e.depth, 3);
                assert_eq!(e.indent, OutlineIndent::Bool(true));
            }
            _ => panic!("esperado Outline"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(
            OutlineElem::new(None, 3, OutlineIndent::Auto).element_kind(),
            Some(ElementKind::Outline)
        );
        assert_eq!(
            OutlineElem::new(None, 3, OutlineIndent::Auto).to_payload(),
            Some(ElementPayload::Outline)
        );
    }

    #[test]
    fn p1406_preserva_presenca_explicita_de_title_none() {
        let omitted = OutlineElem::new(None, 3, OutlineIndent::Auto);
        let explicit = OutlineElem::with_target_and_title_presence(
            None,
            true,
            3,
            OutlineIndent::Auto,
            OutlineTarget::Headings,
        );
        assert!(!omitted.title_was_set());
        assert!(explicit.title_was_set());
        assert_ne!(omitted, explicit);
    }
}
