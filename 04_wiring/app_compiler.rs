// -----------------------------------------------------------------------------
// Tipologia: Component Architecture Wiring (L4)
// Módulo: Compiler
// Descrição: Amarração de classes no nível superior da aplicação.
// Preenche e constrói o Controller com os Adaptadores Concretos (Inject) e
// injeta as dependências engolfadas na Quarentena (L20) para Strangler Fig.
// -----------------------------------------------------------------------------

use crate::infra::compiler_io_impl::CompileEnvAdapter;
use crate::shell::compiler_controller::CompilerOrchestrator;
use crate::shell::compiler_proxy::CompileProxy;

pub fn initialize_compiler() -> CompilerOrchestrator<CompileEnvAdapter> {
    let env_adapter = CompileEnvAdapter::new();
    let compiler = CompilerOrchestrator::new(env_adapter);
    compiler
}

/// Cria a casca suja temporal.
pub fn initialize_legacy_proxy<'a>(
    env: &'a CompileEnvAdapter, 
    world: &'a dyn typst::World
) -> CompileProxy<'a, CompileEnvAdapter> {
    CompileProxy::new(env, world)
}
