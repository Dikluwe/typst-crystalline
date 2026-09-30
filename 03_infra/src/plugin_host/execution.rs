//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host/execution.md
//! @prompt-hash 420065b4
//! @layer L3
//! @updated 2026-09-26

use super::snapshot::{restore_memory, snapshot_memory};
use super::typst_env::{CallData, MemoryError};
use super::{PluginEntry, WasmiPluginHost};
use std::sync::{atomic::Ordering, Arc};
use typst_core::contracts::plugin_host::{PluginError, PluginModuleId};
use typst_core::entities::bytes::Bytes;

pub(super) fn call(
    host: &WasmiPluginHost,
    module: PluginModuleId,
    func_name: &str,
    args: &[Bytes],
) -> Result<Bytes, PluginError> {
    let entry = host.lookup(module)?;

    // Instância fresca por chamada — `plugin.rs:433-437` (sem pool).
    let mut store = wasmi::Store::new(entry.base.linker.engine(), CallData::default());
    let instance = entry
        .base
        .linker
        .instantiate_and_start(&mut store, &entry.base.module)
        .map_err(|err| PluginError::new(format!("{err}")))?;

    // P819 — derivados de `transition` restauram o snapshot antes da
    // chamada (`plugin.rs:440-443`); módulos do `load` têm `None`.
    if let Some(snapshot) = &entry.snapshot {
        restore_memory(&instance, &mut store, snapshot);
    }

    run_call(&instance, &mut store, func_name, args)
}

/// **P819** — transition API (`plugin.rs:328-352`): executa a chamada
/// mutável sobre uma instância fresca (com o snapshot do módulo, se
/// houver), faz snapshot da memória resultante e regista um módulo
/// derivado que a restaura em cada chamada. O módulo original fica
/// inalterado. Erros da chamada: os de `run_call` (catálogo verbatim).
pub(super) fn transition(
    host: &WasmiPluginHost,
    module: PluginModuleId,
    func_name: &str,
    args: &[Bytes],
) -> Result<PluginModuleId, PluginError> {
    let entry = host.lookup(module)?;

    let mut store = wasmi::Store::new(entry.base.linker.engine(), CallData::default());
    let instance = entry
        .base
        .linker
        .instantiate_and_start(&mut store, &entry.base.module)
        .map_err(|err| PluginError::new(format!("{err}")))?;
    if let Some(snapshot) = &entry.snapshot {
        restore_memory(&instance, &mut store, snapshot);
    }

    // Chamada mutável — o output é descartado (o vanilla também o
    // descarta, `plugin.rs:337`); um erro aborta sem derivado.
    let _ = run_call(&instance, &mut store, func_name, args)?;

    // Snapshot da memória após a mutação (`plugin.rs:340,522-530`).
    let snapshot = snapshot_memory(&instance, &store);

    // Novo módulo derivado — id fresco por derivação (o fingerprint do
    // vanilla é mecânica; a memoização em L1 serve gémeos da cache).
    let id = PluginModuleId(host.next_id.fetch_add(1, Ordering::Relaxed));
    let derived = Arc::new(PluginEntry {
        base: Arc::clone(&entry.base),
        snapshot: Some(snapshot),
    });
    host.modules.lock().expect("modules lock").insert(id, derived);
    Ok(id)
}

/// Executa a chamada `func_name(args)` sobre a instância dada — corpo
/// partilhado de `call` e `transition` (P819). Réplica de
/// `PluginInstance::call` (`plugin.rs:447-520`).
fn run_call(
    instance: &wasmi::Instance,
    store: &mut wasmi::Store<CallData>,
    func_name: &str,
    args: &[Bytes],
) -> Result<Bytes, PluginError> {
    // Obter a função exportada — `plugin.rs:448-453` (vanilla faz unwrap;
    // aqui devolvemos erro em vez de panic — defesa de API directa).
    let func = instance
        .get_export(&*store, func_name)
        .and_then(|ext| ext.into_func())
        .ok_or_else(|| {
            PluginError::new(format!("plugin function `{func_name}` not found"))
        })?;
    let ty = func.ty(&*store);

    // Validação de assinatura (lazy) — `plugin.rs:459-466`
    if ty.params().iter().any(|&v| v != wasmi::ValType::I32) {
        return Err(PluginError::new(format!(
            "plugin function `{func_name}` has a parameter that is not a 32-bit integer",
        )));
    }
    if ty.results() != [wasmi::ValType::I32] {
        return Err(PluginError::new(format!(
            "plugin function `{func_name}` does not return exactly one 32-bit integer",
        )));
    }

    // Contagem de args — `plugin.rs:468-477`
    let expected = ty.params().len();
    let given = args.len();
    if expected != given {
        return Err(PluginError::new(format!(
            "plugin function takes {expected} argument{}, but {given} {} given",
            if expected == 1 { "" } else { "s" },
            if given == 1 { "was" } else { "were" },
        )));
    }

    // Lengths como i32 + guardar buffers — `plugin.rs:479-486`
    let lengths: Vec<wasmi::Val> =
        args.iter().map(|a| wasmi::Val::I32(a.len() as i32)).collect();
    store.data_mut().args = args.iter().map(|a| a.as_slice().to_vec()).collect();

    // Chamada — `plugin.rs:489-492` (trap → "plugin panicked: {err}")
    let mut code = wasmi::Val::I32(-1);
    func.call(&mut *store, &lengths, std::slice::from_mut(&mut code))
        .map_err(|err| PluginError::new(format!("plugin panicked: {err}")))?;

    // Out-of-bounds — `plugin.rs:494-502`
    if let Some(MemoryError { offset, length, write }) =
        store.data_mut().memory_error.take()
    {
        let kind = if write { "write" } else { "read" };
        return Err(PluginError::new(format!(
            "plugin tried to {kind} out of bounds: \
             pointer {offset:#x} is out of bounds for {kind} of length {length}",
        )));
    }

    // Output + código de retorno — `plugin.rs:504-519`
    let output = std::mem::take(&mut store.data_mut().output);
    match code {
        wasmi::Val::I32(0) => Ok(Bytes::new(output)),
        wasmi::Val::I32(1) => match std::str::from_utf8(&output) {
            Ok(message) => {
                Err(PluginError::new(format!("plugin errored with: {message}")))
            }
            Err(_) => Err(PluginError::new(
                "plugin errored, but did not return a valid error message",
            )),
        },
        _ => Err(PluginError::new("plugin did not respect the protocol")),
    }
}
