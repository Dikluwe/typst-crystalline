//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host.md
//! @prompt-hash f6e1963c
//! @layer L3
//! @updated 2026-09-26
//! Host WASM facade; runtime responsibilities live in private L3 descendants.

use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use ecow::EcoString;

use typst_core::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use typst_core::entities::bytes::Bytes;

/// Host concreto de plugins WASM (L3). `&self` no trait obriga mutabilidade
/// interior aqui (`Mutex`); instância fresca por `call` (sem pool — ver L0).
pub struct WasmiPluginHost {
    engine: wasmi::Engine,
    next_id: AtomicU64,
    modules: Mutex<HashMap<PluginModuleId, Arc<PluginEntry>>>,
}

use loading::PluginBase;
use snapshot::Snapshot;

mod execution;
mod loading;
mod snapshot;
#[cfg(test)]
mod tests;
mod typst_env;

/// **P819** — entrada do mapa de módulos: o `base` partilhado (módulo
/// compilado + linker) mais o snapshot de memória a restaurar em cada
/// instância fresca (`None` para módulos do `load`; `Some` para derivados
/// de `transition`). Espelha `Plugin { base, snapshot }` do vanilla
/// (`plugin.rs:242-257`) sem pool nem fingerprint (mecânica — ver L0).
struct PluginEntry {
    base: Arc<PluginBase>,
    snapshot: Option<Snapshot>,
}

impl WasmiPluginHost {
    /// Novo host com `wasm_relaxed_simd(false)` para determinismo
    /// (`plugin.rs:269-274`).
    pub fn new() -> Self {
        let mut config = wasmi::Config::default();
        config.wasm_relaxed_simd(false);
        let engine = wasmi::Engine::new(&config);
        Self {
            engine,
            next_id: AtomicU64::new(1),
            modules: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for WasmiPluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost for WasmiPluginHost {
    fn load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
        loading::load(self, bytes)
    }

    fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
        loading::exports(self, module)
    }

    fn call(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<Bytes, PluginError> {
        execution::call(self, module, func_name, args)
    }

    fn transition(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<PluginModuleId, PluginError> {
        execution::transition(self, module, func_name, args)
    }
}

impl WasmiPluginHost {
    /// Lookup da entry por id (ramo defensivo — não tem equivalente no
    /// vanilla; ver L0 "inferência marcada").
    fn lookup(&self, module: PluginModuleId) -> Result<Arc<PluginEntry>, PluginError> {
        let map = self.modules.lock().expect("modules lock");
        map.get(&module)
            .cloned()
            .ok_or_else(|| PluginError::new("plugin module not found"))
    }
}
