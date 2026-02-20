use typst_library::diag::SourceResult;
use typst_library::engine::Engine;
use typst_library::foundations::{Content, StyleChain, Target};
use typst_library::introspection::Introspector;
use crate::model::DocumentInfo; // Ajustaremos depois os imports de lib_ast

/// Semente Transcrita: Representa o artefato de saída compilado (ex: PDF ou HTML).
pub trait Document {
    fn info(&self) -> &DocumentInfo;
    fn introspector(&self) -> &Introspector;
    
    // Associado da Sealed trait na implementação legado
    fn target() -> Target where Self: Sized;
    fn create(engine: &mut Engine, content: &Content, styles: StyleChain) -> SourceResult<Self> where Self: Sized;
}

/// Semente Transcrita: Coerção conveniente para trait object de Document
pub trait AsDocument {
    fn as_document(&self) -> &dyn Document;
}

impl AsDocument for &dyn Document {
    fn as_document(&self) -> &dyn Document {
        *self
    }
}

impl<D: Document> AsDocument for &D {
    fn as_document(&self) -> &dyn Document {
        *self
    }
}
