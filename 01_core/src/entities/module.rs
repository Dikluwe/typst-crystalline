//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/module.md
//! @prompt-hash 0abfc6bf
//! @layer L1
//! @updated 2026-06-23
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
use crate::entities::scope::Scope;

/// Resultado da avaliação de um ficheiro Typst.
///
/// Usa `Arc<ModuleInner>` para que `clone()` seja O(1) — módulos são
/// passados entre ramos de eval() e copiar um IndexMap inteiro seria O(n).
pub struct Module(Arc<ModuleInner>);

#[derive(Debug)]
struct ModuleInner {
    name: String,
    scope: Scope,
    content: Option<Content>,
    /// **P498** — conteúdo original (pré-show-rules) para introspecção.
    /// Permite que `query(heading)` encontre o elemento mesmo quando uma
    /// show-rule o transforma no output renderizado.
    introspection_content: Option<Content>,
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
        Self(Arc::new(ModuleInner {
            name: name.into(),
            scope,
            content: None,
            introspection_content: None,
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
            inner.introspection_content = content;
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
}
