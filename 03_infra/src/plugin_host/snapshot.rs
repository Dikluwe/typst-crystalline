//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host/snapshot.md
//! @prompt-hash 92a424ea
//! @layer L3
//! @updated 2026-09-26

use super::typst_env::CallData;

/// **P819** — snapshot da memória linear (`plugin.rs:420-426`): páginas +
/// dados. Globals WASM não são capturados (limitação do vanilla, paridade).
pub(super) struct Snapshot {
    mem_pages: u64,
    mem_data: Vec<u8>,
}

/// Handle da memória exportada (`plugin.rs:548-554`) — `load` garante que
/// existe, daí o `expect`.
fn instance_memory(
    instance: &wasmi::Instance,
    store: &wasmi::Store<CallData>,
) -> wasmi::Memory {
    instance
        .get_export(store, "memory")
        .and_then(|ext| ext.into_memory())
        .expect("memory export validada no load")
}

/// **P819** — snapshot da memória linear (`plugin.rs:522-530`).
pub(super) fn snapshot_memory(
    instance: &wasmi::Instance,
    store: &wasmi::Store<CallData>,
) -> Snapshot {
    let memory = instance_memory(instance, store);
    Snapshot {
        mem_pages: memory.size(store),
        mem_data: memory.data(store).to_vec(),
    }
}

/// **P819** — restaura a memória de uma instância fresca para o snapshot
/// (`plugin.rs:532-545`): cresce se necessário e copia os dados.
pub(super) fn restore_memory(
    instance: &wasmi::Instance,
    store: &mut wasmi::Store<CallData>,
    snapshot: &Snapshot,
) {
    let memory = instance_memory(instance, store);
    let current = memory.size(&*store);
    if current < snapshot.mem_pages {
        memory
            .grow(&mut *store, snapshot.mem_pages - current)
            .expect("grow de memória ao restaurar snapshot (páginas conhecidas)");
    }
    memory.data_mut(store)[..snapshot.mem_data.len()].copy_from_slice(&snapshot.mem_data);
}
