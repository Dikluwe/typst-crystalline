//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/plugin_host.md
//! @prompt-hash 11768b62
//! @layer L3
//! @updated 2026-07-10
//!
//! `WasmiPluginHost` — implementação L3 de `PluginHost` (trait L1) com o
//! runtime `wasmi`. Réplica do protocolo `typst_env` confirmado em P696 contra
//! `lab/typst-original/.../foundations/plugin.rs` (citado `file:line` no L0).
//! Âmbito P698: só o host, testado directamente em Rust (sem sintaxe Typst —
//! isso é P699). Sem `Module`/`PluginFunc`/`transition`/pool multi-thread.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use ecow::EcoString;

use typst_core::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use typst_core::entities::bytes::Bytes;

/// Host concreto de plugins WASM (L3). `&self` no trait obriga mutabilidade
/// interior aqui (`Mutex`); instância fresca por `call` (sem pool — ver L0).
pub struct WasmiPluginHost {
    engine: wasmi::Engine,
    next_id: AtomicU64,
    modules: Mutex<HashMap<PluginModuleId, Arc<PluginBase>>>,
}

struct PluginBase {
    module: wasmi::Module,
    linker: wasmi::Linker<CallData>,
}

/// Dados por chamada, guardados no `Store` (espelha `plugin.rs:558-566`).
#[derive(Default)]
struct CallData {
    args: Vec<Vec<u8>>,
    output: Vec<u8>,
    memory_error: Option<MemoryError>,
}

/// Último erro de bounds registado pelos host funcs (`plugin.rs:570-574`).
struct MemoryError {
    offset: u32,
    length: u32,
    write: bool,
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
        // `plugin.rs:275-276`
        let module = wasmi::Module::new(&self.engine, bytes).map_err(|err| {
            PluginError::new(format!("failed to load WebAssembly module ({err})"))
        })?;

        // Memória obrigatória — `plugin.rs:279-281`
        if !matches!(
            module.get_export("memory"),
            Some(wasmi::ExternType::Memory(_))
        ) {
            return Err(PluginError::new("plugin does not export its memory"));
        }

        // Imports do host sob "typst_env" — `plugin.rs:283-297`
        let mut linker = wasmi::Linker::new(&self.engine);
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
            .expect("registo do import typst_env write_args_to_buffer (assinatura conhecida)");

        let id = PluginModuleId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let base = Arc::new(PluginBase { module, linker });
        self.modules.lock().expect("modules lock").insert(id, base);
        Ok(id)
    }

    fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
        // Lookup do módulo (mesma defesa de API directa de `call`).
        let base = {
            let map = self.modules.lock().expect("modules lock");
            map.get(&module)
                .cloned()
                .ok_or_else(|| PluginError::new("plugin module not found"))?
        };

        // `into_module` (`plugin.rs:366-380`), filtrando só `ExternType::Func`.
        let mut names = Vec::new();
        for export in base.module.exports() {
            if let wasmi::ExternType::Func(_) = export.ty() {
                names.push(EcoString::from(export.name()));
            }
        }
        Ok(names)
    }

    fn call(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<Bytes, PluginError> {
        // Lookup do módulo (ramo defensivo — não tem equivalente no vanilla;
        // ver L0 "inferência marcada").
        let base = {
            let map = self.modules.lock().expect("modules lock");
            map.get(&module)
                .cloned()
                .ok_or_else(|| PluginError::new("plugin module not found"))?
        };

        // Instância fresca por chamada — `plugin.rs:433-437` (sem pool).
        let mut store = wasmi::Store::new(base.linker.engine(), CallData::default());
        let instance = base
            .linker
            .instantiate_and_start(&mut store, &base.module)
            .map_err(|err| PluginError::new(format!("{err}")))?;

        // Obter a função exportada — `plugin.rs:448-453` (vanilla faz unwrap;
        // aqui devolvemos erro em vez de panic — defesa de API directa).
        let func = instance
            .get_export(&store, func_name)
            .and_then(|ext| ext.into_func())
            .ok_or_else(|| {
                PluginError::new(format!("plugin function `{func_name}` not found"))
            })?;
        let ty = func.ty(&store);

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
        func.call(&mut store, &lengths, std::slice::from_mut(&mut code))
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

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------
    // Encoder WASM mínimo (port de `gen_wasm.py` de P696 §A) — mantém os
    // testes auto-contidos e deterministas (sem toolchain WASM / Python).
    // ---------------------------------------------------------------------
    const I32: u8 = 0x7f;
    const I64: u8 = 0x7e;
    const F64: u8 = 0x7c;

    fn uleb(mut n: u32) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let b = (n & 0x7f) as u8;
            n >>= 7;
            if n == 0 {
                out.push(b);
                break;
            } else {
                out.push(b | 0x80);
            }
        }
        out
    }

    fn sleb(mut n: i32) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let b = (n & 0x7f) as u8;
            n >>= 7; // arithmetic shift (i32)
            let done =
                (n == 0 && (b & 0x40) == 0) || (n == -1 && (b & 0x40) != 0);
            out.push(if done { b } else { b | 0x80 });
            if done {
                break;
            }
        }
        out
    }

    fn vbytes(b: &[u8]) -> Vec<u8> {
        let mut v = uleb(b.len() as u32);
        v.extend_from_slice(b);
        v
    }

    fn name(s: &str) -> Vec<u8> {
        vbytes(s.as_bytes())
    }

    fn section(id: u8, payload: &[u8]) -> Vec<u8> {
        let mut s = vec![id];
        s.extend(uleb(payload.len() as u32));
        s.extend_from_slice(payload);
        s
    }

    fn i32const(n: i32) -> Vec<u8> {
        let mut v = vec![0x41];
        v.extend(sleb(n));
        v
    }
    fn call(f: u32) -> Vec<u8> {
        let mut v = vec![0x10];
        v.extend(uleb(f));
        v
    }
    fn unreachable() -> Vec<u8> {
        vec![0x00]
    }
    fn cat(parts: &[Vec<u8>]) -> Vec<u8> {
        parts.iter().flat_map(|p| p.iter().copied()).collect()
    }

    #[derive(Default)]
    struct Wasm {
        types: Vec<(Vec<u8>, Vec<u8>)>,
        imports: Vec<(String, String, u32)>,
        funcs: Vec<u32>,
        mem_min: Option<u32>,
        exports: Vec<(String, u8, u32)>,
        bodies: Vec<Vec<u8>>,
        data: Vec<(i32, Vec<u8>)>,
    }

    impl Wasm {
        fn new() -> Self {
            Self::default()
        }
        fn typ(&mut self, p: &[u8], r: &[u8]) -> u32 {
            let i = self.types.len() as u32;
            self.types.push((p.to_vec(), r.to_vec()));
            i
        }
        fn import_func(&mut self, m: &str, n: &str, ty: u32) {
            self.imports.push((m.into(), n.into(), ty));
        }
        fn func(&mut self, ty: u32, body: Vec<u8>) -> u32 {
            let idx = (self.imports.len() as u32) + (self.funcs.len() as u32);
            self.funcs.push(ty);
            self.bodies.push(body);
            idx
        }
        fn memory(&mut self, min: u32) {
            self.mem_min = Some(min);
        }
        fn export(&mut self, nm: &str, kind: u8, idx: u32) {
            self.exports.push((nm.into(), kind, idx));
        }
        fn data_seg(&mut self, off: i32, bytes: &[u8]) {
            self.data.push((off, bytes.to_vec()));
        }
        fn build(self) -> Vec<u8> {
            let mut secs: Vec<Vec<u8>> = Vec::new();
            if !self.types.is_empty() {
                let mut p = uleb(self.types.len() as u32);
                for (params, results) in &self.types {
                    p.push(0x60);
                    p.extend(vbytes(params));
                    p.extend(vbytes(results));
                }
                secs.push(section(1, &p));
            }
            if !self.imports.is_empty() {
                let mut p = uleb(self.imports.len() as u32);
                for (m, n, ty) in &self.imports {
                    p.extend(name(m));
                    p.extend(name(n));
                    p.push(0x00);
                    p.extend(uleb(*ty));
                }
                secs.push(section(2, &p));
            }
            if !self.funcs.is_empty() {
                let mut p = uleb(self.funcs.len() as u32);
                for ty in &self.funcs {
                    p.extend(uleb(*ty));
                }
                secs.push(section(3, &p));
            }
            if let Some(min) = self.mem_min {
                let mut p = uleb(1);
                p.push(0x00); // flags: sem max
                p.extend(uleb(min));
                secs.push(section(5, &p));
            }
            if !self.exports.is_empty() {
                let mut p = uleb(self.exports.len() as u32);
                for (nm, kind, idx) in &self.exports {
                    p.extend(name(nm));
                    p.push(*kind);
                    p.extend(uleb(*idx));
                }
                secs.push(section(7, &p));
            }
            if !self.bodies.is_empty() {
                let mut p = uleb(self.bodies.len() as u32);
                for body in &self.bodies {
                    let mut inner = uleb(0); // 0 declarações de locais
                    inner.extend(body);
                    inner.push(0x0b); // end
                    p.extend(uleb(inner.len() as u32));
                    p.extend(inner);
                }
                secs.push(section(10, &p));
            }
            if !self.data.is_empty() {
                let mut p = uleb(self.data.len() as u32);
                for (off, bytes) in &self.data {
                    p.extend(uleb(0)); // memidx 0 (activo)
                    p.push(0x41);
                    p.extend(sleb(*off));
                    p.push(0x0b); // expr de offset
                    p.extend(vbytes(bytes));
                }
                secs.push(section(11, &p));
            }
            let mut w = b"\x00asm\x01\x00\x00\x00".to_vec();
            for s in secs {
                w.extend(s);
            }
            w
        }
    }

    /// Módulo base com os 2 imports `typst_env`, `memory` (1 página) e export
    /// de `memory`. Devolve (Wasm, t_wab, t_srh) para o cenário completar.
    fn base() -> (Wasm, u32, u32) {
        let mut w = Wasm::new();
        let t_wab = w.typ(&[I32], &[]);
        let t_srh = w.typ(&[I32, I32], &[]);
        w.import_func(
            "typst_env",
            "wasm_minimal_protocol_write_args_to_buffer",
            t_wab,
        );
        w.import_func(
            "typst_env",
            "wasm_minimal_protocol_send_result_to_host",
            t_srh,
        );
        w.memory(1);
        w.export("memory", 0x02, 0);
        (w, t_wab, t_srh)
    }

    fn host() -> WasmiPluginHost {
        WasmiPluginHost::new()
    }

    // ----- sucesso -------------------------------------------------------

    #[test]
    fn hello_devolve_bytes_hello() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        // send_result_to_host(0, 5)  [func idx 1]; return 0
        let body = cat(&[i32const(0), i32const(5), call(1), i32const(0)]);
        let hello = w.func(t, body);
        w.export("hello", 0x00, hello);
        w.data_seg(0, b"hello");
        let bytes = w.build();

        let h = host();
        let id = h.load(&bytes).expect("load hello.wasm");
        let out = h.call(id, "hello", &[]).expect("call hello");
        assert_eq!(out.as_slice(), b"hello");
    }

    // ----- catálogo de mensagens (L0) ------------------------------------

    #[test]
    fn msg_01_modulo_sem_memory() {
        let mut w = Wasm::new();
        let t_wab = w.typ(&[I32], &[]);
        let t_srh = w.typ(&[I32, I32], &[]);
        w.import_func("typst_env", "wasm_minimal_protocol_write_args_to_buffer", t_wab);
        w.import_func("typst_env", "wasm_minimal_protocol_send_result_to_host", t_srh);
        // SEM memory / SEM export de memory
        let t_f = w.typ(&[I32], &[I32]);
        let f = w.func(t_f, i32const(0));
        w.export("f", 0x00, f);
        let e = host().load(&w.build()).unwrap_err();
        assert_eq!(e.message.as_str(), "plugin does not export its memory");
    }

    #[test]
    fn msg_02_wasm_invalido() {
        let garbage = b"\x00asm\x01\x00\x00\x00\xff\xff";
        let e = host().load(garbage).unwrap_err();
        assert!(
            e.message.starts_with("failed to load WebAssembly module ("),
            "msg: {e}",
        );
    }

    #[test]
    fn msg_03_param_nao_i32() {
        let (mut w, _, _) = base();
        let t_bad = w.typ(&[F64], &[I32]); // param f64
        let f = w.func(t_bad, i32const(0));
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin function `f` has a parameter that is not a 32-bit integer"
        );
    }

    #[test]
    fn msg_04_resultado_nao_um_i32() {
        let (mut w, _, _) = base();
        let t_bad = w.typ(&[], &[]); // sem resultado
        let f = w.func(t_bad, Vec::new());
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin function `f` does not return exactly one 32-bit integer"
        );
    }

    #[test]
    fn msg_05_arg_count_singular() {
        let (mut w, _, _) = base();
        let t = w.typ(&[I32], &[I32]); // 1 param
        let f = w.func(t, i32const(0));
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err(); // 0 given
        assert_eq!(
            e.message.as_str(),
            "plugin function takes 1 argument, but 0 were given"
        );
    }

    #[test]
    fn msg_05_arg_count_plural() {
        let (mut w, _, _) = base();
        let t = w.typ(&[I32, I32], &[I32]); // 2 params
        let f = w.func(t, i32const(0));
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h
            .call(id, "f", &[Bytes::new(vec![1])])
            .unwrap_err(); // 1 given
        assert_eq!(
            e.message.as_str(),
            "plugin function takes 2 arguments, but 1 was given"
        );
    }

    #[test]
    fn msg_06_trap() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        let body = cat(&[unreachable(), i32const(0)]);
        let f = w.func(t, body);
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert!(e.message.starts_with("plugin panicked:"), "msg: {e}");
    }

    #[test]
    fn msg_07_out_of_bounds_write() {
        let (mut w, _, _) = base();
        let t = w.typ(&[I32], &[I32]); // 1 param (length do arg)
        // write_args_to_buffer(0x100000) [func idx 0]; return 0
        let body = cat(&[i32const(0x100000), call(0), i32const(0)]);
        let f = w.func(t, body);
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h
            .call(id, "f", &[Bytes::new(b"abcde".to_vec())])
            .unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin tried to write out of bounds: pointer 0x100000 is out of bounds for write of length 5"
        );
    }

    #[test]
    fn msg_07_out_of_bounds_read() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        // send_result_to_host(0x100000, 5) [func idx 1]; return 0
        let body = cat(&[i32const(0x100000), i32const(5), call(1), i32const(0)]);
        let f = w.func(t, body);
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin tried to read out of bounds: pointer 0x100000 is out of bounds for read of length 5"
        );
    }

    #[test]
    fn msg_08_erro_utf8() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        // send_result_to_host(0, 4); return 1
        let body = cat(&[i32const(0), i32const(4), call(1), i32const(1)]);
        let f = w.func(t, body);
        w.export("f", 0x00, f);
        w.data_seg(0, b"boom");
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(e.message.as_str(), "plugin errored with: boom");
    }

    #[test]
    fn msg_09_erro_nao_utf8() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        // send_result_to_host(0, 2); return 1  (0xff 0xfe não é UTF-8)
        let body = cat(&[i32const(0), i32const(2), call(1), i32const(1)]);
        let f = w.func(t, body);
        w.export("f", 0x00, f);
        w.data_seg(0, &[0xff, 0xfe]);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin errored, but did not return a valid error message"
        );
    }

    #[test]
    fn msg_10_codigo_retorno_invalido() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        let f = w.func(t, i32const(2)); // return 2
        w.export("f", 0x00, f);
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let e = h.call(id, "f", &[]).unwrap_err();
        assert_eq!(e.message.as_str(), "plugin did not respect the protocol");
    }

    // ----- P699 — exports / Send+Sync ------------------------------------

    #[test]
    fn exports_devolve_so_funcoes_exportadas() {
        let (mut w, _, _) = base();
        let t = w.typ(&[], &[I32]);
        let hello = w.func(t, i32const(0));
        let add = w.func(t, i32const(0));
        w.export("hello", 0x00, hello);
        w.export("add", 0x00, add);
        // `base()` já exporta "memory" (kind 0x02) — não é função, filtrada.
        let h = host();
        let id = h.load(&w.build()).unwrap();
        let mut names = h.exports(id).unwrap();
        names.sort();
        assert_eq!(names, vec![EcoString::from("add"), EcoString::from("hello")]);
    }

    #[test]
    fn exports_modulo_inexistente_erro_defensivo() {
        let h = host();
        let e = h.exports(PluginModuleId(9999)).unwrap_err();
        assert_eq!(e.message.as_str(), "plugin module not found");
    }

    #[test]
    fn host_eh_send_e_sync() {
        // Obrigatório: `World: Send + Sync` e o host viaja em `SystemWorld` e
        // dentro de `Value::Func(FuncRepr::Plugin(...))`.
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<WasmiPluginHost>();
    }
}
