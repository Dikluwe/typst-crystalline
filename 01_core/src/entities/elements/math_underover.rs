//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_underover.md
//! @prompt-hash ecd88f08
//! @layer L1
//! @updated 2026-10-10
//!
//! `MathUnderoverElem` — Lote 2 P317 (família math). Anotações verticais (P297).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Anotações verticais sobre/sob conteúdo matemático — variant agregado P297
/// (ADR-0054 graded). Layouter empilha verticalmente conforme presença de cada.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderoverElem {
    pub base: Content,
    pub under: Option<Content>,
    pub over: Option<Content>,
}

/// Elemento público `math.overbrace`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOverbraceElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.underbrace`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderbraceElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.underbracket`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderbracketElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.underparen`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderparenElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.undershell`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUndershellElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.overbracket`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOverbracketElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.overparen`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOverparenElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

/// Elemento público `math.overshell`: identidade e presença são preservadas;
/// a realização geométrica permanece no owner de layout under/over.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOvershellElem {
    pub body: Content,
    pub annotation: Option<Content>,
    pub annotation_explicit: bool,
}

impl Element for MathOvershellElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏠{}", annotation, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathOvershell(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathOvershell(Arc::new(self.clone()))
    }
}

impl Element for MathOverparenElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏜{}", annotation, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathOverparen(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathOverparen(Arc::new(self.clone()))
    }
}

impl Element for MathOverbracketElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⎴{}", annotation, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathOverbracket(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathOverbracket(Arc::new(self.clone()))
    }
}

impl Element for MathOverbraceElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏞{}", annotation, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathOverbrace(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathOverbrace(Arc::new(self.clone()))
    }
}

impl Element for MathUnderbraceElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏟{}", self.body.plain_text(), annotation)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUnderbrace(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUnderbrace(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "annotation" if self.annotation_explicit => {
                Some(self.annotation.clone().map(Value::Content).unwrap_or(Value::None))
            }
            _ => None,
        }
    }
}

impl Element for MathUnderbracketElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⎵{}", self.body.plain_text(), annotation)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUnderbracket(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUnderbracket(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "annotation" if self.annotation_explicit => {
                Some(self.annotation.clone().map(Value::Content).unwrap_or(Value::None))
            }
            _ => None,
        }
    }
}

impl Element for MathUnderparenElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏝{}", self.body.plain_text(), annotation)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUnderparen(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUnderparen(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "annotation" if self.annotation_explicit => {
                Some(self.annotation.clone().map(Value::Content).unwrap_or(Value::None))
            }
            _ => None,
        }
    }
}

impl Element for MathUndershellElem {
    fn plain_text(&self) -> String {
        let annotation =
            self.annotation.as_ref().map(Content::plain_text).unwrap_or_default();
        format!("{}⏡{}", self.body.plain_text(), annotation)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUndershell(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUndershell(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "annotation" if self.annotation_explicit => {
                Some(self.annotation.clone().map(Value::Content).unwrap_or(Value::None))
            }
            _ => None,
        }
    }
}

impl Element for MathUnderoverElem {
    fn plain_text(&self) -> String {
        let o = self.over.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        let b = self.base.plain_text();
        let u = self.under.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        format!("{}{}{}", o, b, u)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUnderover(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathUnderover(Arc::new(self.clone()))
    }
}

impl MathOvershellElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathOvershell(Arc::new(mapped))))
    }
}

impl MathOverparenElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathOverparen(Arc::new(mapped))))
    }
}

impl MathOverbracketElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathOverbracket(Arc::new(mapped))))
    }
}

impl MathOverbraceElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathOverbrace(Arc::new(mapped))))
    }
}

impl MathUnderbraceElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathUnderbrace(Arc::new(mapped))))
    }
}

impl MathUnderbracketElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathUnderbracket(Arc::new(mapped))))
    }
}

impl MathUnderparenElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathUnderparen(Arc::new(mapped))))
    }
}

impl MathUndershellElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        let annotation = self
            .annotation
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if body.is_none() && annotation.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        if let Some(changed) = annotation {
            mapped.annotation = Some(changed);
        }
        Ok(Some(Content::MathUndershell(Arc::new(mapped))))
    }
}

impl MathUnderoverElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let base = self.base.map_content_if_changed(transform)?;
        let under = self
            .under
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        let over = self
            .over
            .as_ref()
            .map(|child| child.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if base.is_none() && under.is_none() && over.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = base {
            mapped.base = changed;
        }
        if let Some(changed) = under {
            mapped.under = Some(changed);
        }
        if let Some(changed) = over {
            mapped.over = Some(changed);
        }
        Ok(Some(Content::MathUnderover(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathUnderoverElem {
        MathUnderoverElem {
            base: Content::text("x"),
            under: Some(Content::text("u")),
            over: Some(Content::text("o")),
        }
    }

    #[test]
    fn plain_text_ordem_visual() {
        assert_eq!(ex().plain_text(), "oxu");
        let so = MathUnderoverElem { base: Content::text("x"), under: None, over: None };
        assert_eq!(so.plain_text(), "x");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.under = None;
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_preserva_none() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let so = MathUnderoverElem { base: Content::text("x"), under: None, over: None };
        match so.map_content(&mut f).unwrap() {
            Content::MathUnderover(e) => {
                assert!(e.under.is_none() && e.over.is_none());
                assert_eq!(e.plain_text(), "Z");
            }
            _ => panic!("esperado MathUnderover"),
        }
    }
}
