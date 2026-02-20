use typst_library::diag::{Warned, SourceResult};
use crate::contracts::document_traits::{Document, AsDocument};
use crate::contracts::compiler_io::ICompilerEnv;

/// Proxy (L2) simulando a dependência do mundo externo.
pub struct CompileProxy<'a, E: ICompilerEnv> {
    env: &'a E,
    world: &'a dyn typst::World // Ponte de Strangler pro velho Mundo tipado
}

impl<'a, E: ICompilerEnv> CompileProxy<'a, E> {
    pub fn new(env: &'a E, world: &'a dyn typst::World) -> Self {
        Self { env, world }
    }

    /// Roda o compile real repassando para o Legacy L20 Typst
    pub fn compile<D: Document>(&self) -> Warned<SourceResult<D>> 
    where D: typst::Document 
    {
       // O mundo real (L20) tem seu compile que chama o engine massivo.
       // Para fins de Wiring e não quebrar o compilador Tekt, delegamos
       // à versão original. "A Quarentena Intocada".
       typst::compile(self.world)
    }
}
