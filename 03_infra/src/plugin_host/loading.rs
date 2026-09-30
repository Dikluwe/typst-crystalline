//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host/loading.md
//! @prompt-hash ba71df03
//! @layer L3
//! @updated 2026-09-26

use super::typst_env::CallData;
use super::{PluginEntry, WasmiPluginHost};
use ecow::EcoString;
use std::sync::{atomic::Ordering, Arc};
use typst_core::contracts::plugin_host::{PluginError, PluginModuleId};

pub(super) struct PluginBase {
    pub(super) module: wasmi::Module,
    pub(super) linker: wasmi::Linker<CallData>,
}

pub(super) fn load(
    host: &WasmiPluginHost,
    bytes: &[u8],
) -> Result<PluginModuleId, PluginError> {
    // `plugin.rs:275-276`
    let module = wasmi::Module::new(&host.engine, bytes).map_err(|err| {
        PluginError::new(format!("failed to load WebAssembly module ({err})"))
    })?;

    // Memória obrigatória — `plugin.rs:279-281`
    if !matches!(module.get_export("memory"), Some(wasmi::ExternType::Memory(_))) {
        return Err(PluginError::new("plugin does not export its memory"));
    }

    let linker = super::typst_env::linker(&host.engine);

    let id = PluginModuleId(host.next_id.fetch_add(1, Ordering::Relaxed));
    let entry = Arc::new(PluginEntry {
        base: Arc::new(PluginBase { module, linker }),
        snapshot: None,
    });
    host.modules.lock().expect("modules lock").insert(id, entry);
    Ok(id)
}

pub(super) fn exports(
    host: &WasmiPluginHost,
    module: PluginModuleId,
) -> Result<Vec<EcoString>, PluginError> {
    // Lookup do módulo (mesma defesa de API directa de `call`).
    let entry = host.lookup(module)?;

    // `into_module` (`plugin.rs:366-380`), filtrando só `ExternType::Func`.
    let mut names = Vec::new();
    for export in entry.base.module.exports() {
        if let wasmi::ExternType::Func(_) = export.ty() {
            names.push(EcoString::from(export.name()));
        }
    }
    Ok(names)
}
