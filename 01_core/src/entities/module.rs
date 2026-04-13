//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/module.md
//! @prompt-hash 913115d9
//! @layer L1
//! @updated 2026-03-28

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::scope::Scope;

/// Resultado da avaliação de um ficheiro Typst.
///
/// Usa `Arc<ModuleInner>` para que `clone()` seja O(1) — módulos são
/// passados entre ramos de eval() e copiar um IndexMap inteiro seria O(n).
pub struct Module(Arc<ModuleInner>);

#[derive(Debug)]
struct ModuleInner {
    name:    String,
    scope:   Scope,
    content: Option<Content>,
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
}
