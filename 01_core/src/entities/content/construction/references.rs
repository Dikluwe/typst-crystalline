//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/construction/references.md
//! @prompt-hash ceec4574
//! @layer L1
//! @updated 2026-09-30

use super::*;

impl Content {
    /// **P462** — `Content::Ref` (referência cruzada `@label` / `ref("label")`).
    /// Nome `reference` (não `r#ref`: evita raw identifier nos call-sites).
    pub fn reference(name: impl Into<EcoString>) -> Self {
        Self::reference_with_supplement(name, None)
    }

    /// **P462** — `Content::Ref` com supplement explícito (ex: `ref("fig1", supplement: "Fig. ")`).
    pub fn reference_with_supplement(
        name: impl Into<EcoString>,
        supplement: Option<Content>,
    ) -> Self {
        Self::reference_with_form(
            name,
            supplement,
            crate::entities::elements::r#ref::RefForm::Normal,
        )
    }

    pub fn reference_with_form(
        name: impl Into<EcoString>,
        supplement: Option<Content>,
        form: crate::entities::elements::r#ref::RefForm,
    ) -> Self {
        Self::Ref(Arc::new(RefElem { name: name.into(), supplement, form }))
    }

    /// **Lote 8 P323** — `Content::Outline` (índice). Unit struct.
    /// P457: adiciona parâmetros `title`, `depth`, `indent` com defaults vanilla.
    /// P502: `indent` é `OutlineIndent` (auto/bool/length/function).
    pub fn outline() -> Self {
        use crate::entities::elements::outline::OutlineIndent;
        Self::outline_with(None, 3, OutlineIndent::Auto)
    }

    pub fn outline_entry(level: usize, element: Content, fill: Option<Content>) -> Self {
        Self::outline_entry_with_fill_state(level, element, fill, true)
    }

    pub fn outline_entry_with_fill_state(
        level: usize,
        element: Content,
        fill: Option<Content>,
        fill_set: bool,
    ) -> Self {
        Self::OutlineEntry(Arc::new(OutlineEntryElem::new(
            level, element, fill, fill_set,
        )))
    }

    pub fn realized_outline_entry(
        level: usize,
        element: Content,
        fill: Option<Content>,
        prefix: Option<Content>,
        element_location: crate::entities::location::Location,
    ) -> Self {
        Self::OutlineEntry(Arc::new(OutlineEntryElem::realized(
            level,
            element,
            fill,
            true,
            prefix,
            element_location,
        )))
    }

    pub fn outline_with(
        title: Option<Content>,
        depth: usize,
        indent: crate::entities::elements::outline::OutlineIndent,
    ) -> Self {
        Self::Outline(Arc::new(OutlineElem::new(title, depth, indent)))
    }

    /// **P472** — List of Figures (`lof()`).
    pub fn lof(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Figures,
        )))
    }

    /// **P472** — List of Tables (`lot()`).
    pub fn lot(title: Option<Content>) -> Self {
        use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
        Self::Outline(Arc::new(OutlineElem::with_target(
            title,
            1,
            OutlineIndent::Bool(false),
            OutlineTarget::Tables,
        )))
    }

    /// `bibliography(entries, title)` — Passo 159A (par acoplado
    /// com `cite`). Subset minimal per ADR-0054 graded; input
    /// cristalino literal `Vec<BibEntry>`; sem hayagriva.
    pub fn bibliography(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style: None,
            locale: None,
        }))
    }

    /// **P418** — `bibliography(entries, title, style, locale)` com CSL.
    pub fn bibliography_with_style(
        entries: Vec<crate::entities::bib_entry::BibEntry>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries,
            path: None,
            title,
            style,
            locale,
        }))
    }

    /// **P419** — `bibliography(path, title, style, locale)` carregado de disco.
    pub fn bibliography_from_path(
        path: impl Into<EcoString>,
        title: Option<Content>,
        style: Option<EcoString>,
        locale: Option<EcoString>,
    ) -> Self {
        Self::Bibliography(Arc::new(BibliographyElem {
            entries: Vec::new(),
            path: Some(path.into()),
            title,
            style,
            locale,
        }))
    }

    /// `cite(key, supplement, form, style)` — Passo 159A (par acoplado com
    /// `bibliography`) + Passo 159C (form variants) + P468 (style).
    /// Sem validação cross-reference (ADR-0017 Introspection runtime adiada).
    pub fn cite(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
    ) -> Self {
        Self::cite_with_style(key, supplement, form, None)
    }

    /// **P468** — constructor de `Content::Cite` com estilo explícito.
    pub fn cite_with_style(
        key: impl Into<String>,
        supplement: Option<Content>,
        form: Option<crate::entities::citation_form::CitationForm>,
        style: Option<crate::entities::citation_style::CitationStyle>,
    ) -> Self {
        Self::Cite(Arc::new(CiteElem { key: key.into(), supplement, form, style }))
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
