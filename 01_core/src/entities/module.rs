//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/module.md
//! @prompt-hash 99919097
//! @layer L1
//! @updated 2026-10-05
//!
//! Resultado da avaliação de um ficheiro Typst.
//!
//! Usa `Arc<ModuleInner>` para que `clone()` seja O(1) — módulos são
//! passados entre ramos de eval() e copiar um IndexMap inteiro seria O(n).
//!
//! **P429 (DEBT-63)** — adicionado `bib_styles`: tabela lateral de styles
//! CSL resolvidos em eval time, indexados pela chave determinística do
//! `BibliographyElem`. Transporte eval → pipeline → `BibStore`.

use std::collections::HashMap;
use std::sync::Arc;

use hayagriva::citationberg::IndependentStyle;

use crate::entities::content::Content;
use crate::entities::document_info::DocumentInfo;
use crate::entities::introspector::TagIntrospector;
use crate::entities::scope::Scope;

/// Resultado da avaliação de um ficheiro Typst.
///
/// Usa `Arc<ModuleInner>` para que `clone()` seja O(1) — módulos são
/// passados entre ramos de eval() e copiar um IndexMap inteiro seria O(n).
pub struct Module(Arc<ModuleInner>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleKind {
    Ordinary,
    Std,
    Format,
}

#[derive(Debug)]
struct ModuleInner {
    kind: ModuleKind,
    name: String,
    scope: Scope,
    content: Option<Content>,
    /// Semantic graph of originals and products for introspection.
    /// Permite que `query(heading)` encontre o elemento mesmo quando uma
    /// show-rule o transforma no output renderizado.
    introspection_content: Option<Content>,
    realization_introspector: Option<Arc<TagIntrospector>>,
    /// **P429** — styles CSL resolvidos em eval time. Tabela lateral
    /// indexada por `BibliographyElem::style_key()`.
    bib_styles: HashMap<u64, Arc<IndependentStyle>>,
    /// **P536** — metadados do documento definidos por `#set document(...)`.
    document_info: DocumentInfo,
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").field("name", &self.0.name).finish()
    }
}

/// Igualdade por identidade de ponteiro Arc — dois Modules são iguais
/// se e só se partilham o mesmo ModuleInner (mesma avaliação).
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Module {
    pub fn new(name: impl Into<String>, scope: Scope) -> Self {
        Self::with_kind(name.into(), scope, ModuleKind::Ordinary)
    }

    pub(crate) fn stdlib(scope: Scope) -> Self {
        Self::with_kind("global".into(), scope, ModuleKind::Std)
    }

    pub(crate) fn format(scope: Scope) -> Self {
        Self::with_kind("format".into(), scope, ModuleKind::Format)
    }

    pub(crate) fn is_std(&self) -> bool {
        self.0.kind == ModuleKind::Std
    }

    pub(crate) fn is_format(&self) -> bool {
        self.0.kind == ModuleKind::Format
    }

    fn with_kind(name: String, scope: Scope, kind: ModuleKind) -> Self {
        Self(Arc::new(ModuleInner {
            kind,
            name,
            scope,
            content: None,
            introspection_content: None,
            realization_introspector: None,
            bib_styles: HashMap::new(),
            document_info: DocumentInfo::empty(),
        }))
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn scope(&self) -> &Scope {
        &self.0.scope
    }

    /// Conteúdo produzido por `eval()`.
    pub fn content(&self) -> Option<&Content> {
        self.0.content.as_ref()
    }

    /// Define o conteúdo — chamado em `eval()` após avaliar o markup.
    ///
    /// Requer que o Arc tenha exactamente uma referência (imediatamente
    /// após `Module::new`). Se já foi clonado, não muta.
    pub fn set_content(&mut self, content: Option<Content>) {
        if let Some(inner) = Arc::get_mut(&mut self.0) {
            inner.realization_introspector = None;
            inner.content = content;
        }
    }

    /// **P498** — conteúdo original (pré-show-rules) para introspecção.
    pub fn introspection_content(&self) -> Option<&Content> {
        self.0.introspection_content.as_ref()
    }

    /// **P498** — define o conteúdo original para introspecção.
    pub fn set_introspection_content(&mut self, content: Option<Content>) {
        if let Some(inner) = Arc::get_mut(&mut self.0) {
            inner.realization_introspector = None;
            inner.introspection_content = content;
        }
    }

    /// Semantic index produced by this module's realization, before layout.
    pub fn realization_introspector(&self) -> Option<&TagIntrospector> {
        self.0.realization_introspector.as_deref()
    }

    pub(crate) fn set_realization_introspector(&mut self, introspector: TagIntrospector) {
        if let Some(inner) = Arc::get_mut(&mut self.0) {
            inner.realization_introspector = Some(Arc::new(introspector));
        }
    }

    /// **P429** — styles CSL resolvidos em eval time.
    pub fn bibliography_styles(&self) -> &HashMap<u64, Arc<IndependentStyle>> {
        &self.0.bib_styles
    }

    /// **P429** — define os styles resolvidos. Chamado em `eval()` antes de
    /// devolver o módulo. Requer Arc com referência única.
    pub fn set_bibliography_styles(
        &mut self,
        styles: HashMap<u64, Arc<IndependentStyle>>,
    ) {
        if let Some(inner) = Arc::get_mut(&mut self.0) {
            inner.bib_styles = styles;
        }
    }

    /// **P536** — metadados do documento definidos por `#set document(...)`.
    pub fn document_info(&self) -> &DocumentInfo {
        &self.0.document_info
    }

    /// **P536** — define os metadados do documento. Chamado em `eval()` antes
    /// de devolver o módulo. Requer Arc com referência única.
    pub fn set_document_info(&mut self, info: DocumentInfo) {
        if let Some(inner) = Arc::get_mut(&mut self.0) {
            inner.document_info = info;
        }
    }
}

/// Clone é O(1) — incrementa contagem de Arc.
/// Necessário porque módulos são passados entre ramos de eval().
impl Clone for Module {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{scope::Scope, value::Value};

    #[test]
    fn nome_e_scope() {
        let mut scope = Scope::new();
        scope.define("x", Value::None);
        let m = Module::new("my-file", scope);
        assert_eq!(m.name(), "my-file");
        assert!(m.scope().get("x").is_some());
    }

    #[test]
    fn clone_consistente() {
        let scope = Scope::new();
        let m1 = Module::new("test", scope);
        let m2 = m1.clone();
        assert_eq!(m1.name(), m2.name());
        // m1 e m2 partilham o mesmo ModuleInner via Arc
    }

    #[test]
    fn scope_vazio_valido() {
        let m = Module::new("empty", Scope::new());
        assert!(m.scope().is_empty());
    }

    #[test]
    fn bibliography_styles_default_vazio() {
        let m = Module::new("empty", Scope::new());
        assert!(m.bibliography_styles().is_empty());
    }

    #[test]
    fn realization_index_is_invalidated_by_content_carriers() {
        let mut module = Module::new("test", Scope::new());
        module.set_realization_introspector(TagIntrospector::empty());
        assert!(module.realization_introspector().is_some());
        module.set_content(Some(Content::text("changed")));
        assert!(module.realization_introspector().is_none());
        module.set_realization_introspector(TagIntrospector::empty());
        module.set_introspection_content(Some(Content::text("semantic change")));
        assert!(module.realization_introspector().is_none());
    }

    #[test]
    fn realization_index_clone_keeps_snapshot_isolated() {
        let content = Content::heading(1, Content::text("Original"));
        let index = crate::compiler::introspect::introspect_with_introspector(&content);
        let mut module = Module::new("test", Scope::new());
        module.set_content(Some(content));
        module.set_realization_introspector(index);
        let clone = module.clone();
        let mut attempt = clone.realization_introspector().unwrap().clone();
        attempt.elements.clear();
        assert_eq!(module.realization_introspector().unwrap().elements.len(), 1);
        assert_eq!(clone.realization_introspector().unwrap().elements.len(), 1);
    }
}

#[cfg(test)]
mod builtin_module_identity_layout_tests {
    use super::*;

    #[test]
    fn builtin_module_identity_public_control_and_footprint() {
        let module = Module::new("global", Scope::new());
        let clone = module.clone();
        assert_eq!(module, clone);
        assert_ne!(module, Module::new("global", Scope::new()));
        assert_eq!(format!("{module:?}"), "Module { name: \"global\" }");
        assert!(module.content().is_none());
        assert!(module.introspection_content().is_none());
        assert!(module.realization_introspector().is_none());
        assert!(module.document_info().is_empty());
        assert!(module.bibliography_styles().is_empty());
        println!(
            "MODULE_LAYOUT {} {}",
            std::mem::size_of::<Module>(),
            std::mem::size_of::<ModuleInner>()
        );
    }
}
