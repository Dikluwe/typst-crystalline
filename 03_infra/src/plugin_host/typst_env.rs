//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host/typst_env.md
//! @prompt-hash 8ef3368f
//! @layer L3
//! @updated 2026-09-26

/// Dados por chamada, guardados no `Store` (espelha `plugin.rs:558-566`).
#[derive(Default)]
pub(super) struct CallData {
    pub(super) args: Vec<Vec<u8>>,
    pub(super) output: Vec<u8>,
    pub(super) memory_error: Option<MemoryError>,
}

/// Último erro de bounds registado pelos host funcs (`plugin.rs:570-574`).
pub(super) struct MemoryError {
    pub(super) offset: u32,
    pub(super) length: u32,
    pub(super) write: bool,
}

/// Registra exclusivamente os dois imports do protocolo typst_env.
pub(super) fn linker(engine: &wasmi::Engine) -> wasmi::Linker<CallData> {
    // Imports do host sob "typst_env" — `plugin.rs:283-297`
    let mut linker = wasmi::Linker::new(engine);
    linker
        .func_wrap(
            "typst_env",
            "wasm_minimal_protocol_send_result_to_host",
            wasm_minimal_protocol_send_result_to_host,
        )
        .expect("registo do import typst_env send_result_to_host (assinatura conhecida)");
    linker
        .func_wrap(
            "typst_env",
            "wasm_minimal_protocol_write_args_to_buffer",
            wasm_minimal_protocol_write_args_to_buffer,
        )
        .expect(
            "registo do import typst_env write_args_to_buffer (assinatura conhecida)",
        );

    linker
}

/// Escreve os args no buffer do plugin — `plugin.rs:577-595`.
fn wasm_minimal_protocol_write_args_to_buffer(
    mut caller: wasmi::Caller<'_, CallData>,
    ptr: u32,
) {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let arguments = std::mem::take(&mut caller.data_mut().args);
    let mut offset = ptr as usize;
    for arg in arguments {
        if memory.write(&mut caller, offset, arg.as_slice()).is_err() {
            caller.data_mut().memory_error = Some(MemoryError {
                offset: offset as u32,
                length: arg.len() as u32,
                write: true,
            });
            return;
        }
        offset += arg.len();
    }
}

/// Lê o resultado do plugin para o host — `plugin.rs:598-612`.
fn wasm_minimal_protocol_send_result_to_host(
    mut caller: wasmi::Caller<'_, CallData>,
    ptr: u32,
    len: u32,
) {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut buffer = std::mem::take(&mut caller.data_mut().output);
    buffer.resize(len as usize, 0);
    if memory.read(&caller, ptr as usize, &mut buffer).is_err() {
        caller.data_mut().memory_error =
            Some(MemoryError { offset: ptr, length: len, write: false });
        return;
    }
    caller.data_mut().output = buffer;
}
