//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/plugin.md
//! @prompt-hash 2f7422ed
//! @layer L1
//! @updated 2026-07-10
//!
//! Builtin `plugin()` (P697/P699) — lê o payload WASM (caminho via
//! `World::read_bytes`, ou `bytes` directos), carrega-o no `PluginHost`
//! injectado via `World::plugin_host()` e devolve um `Module` real com uma
//! função (`Func::plugin(PluginFunc{...})`) por export do módulo (P699).

use std::sync::Arc;

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::plugin_func::PluginFunc;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn err(msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), msg.into())]
}

fn reject_named(args: &Args) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(format!("argumento nomeado inesperado em plugin(): '{k}'")));
    }
    Ok(())
}

/// `plugin(source)` — carrega um plugin WebAssembly e devolve o seu `Module`.
///
/// **P697** resolve `source` para bytes (`str` via `World::read_bytes`, com a
/// mesma resolução relativa/`/...` de `read()`/`#import`; ou `bytes` directos).
/// **P699** carrega-os no `PluginHost` (`World::plugin_host()`), enumera os
/// exports (funções) e devolve `Value::Module(Module::new("plugin", scope))`
/// com uma função por export — chamável como `p.funcao(bytes)` e importável
/// via `#import plugin("f.wasm"): funcao`.
pub fn native_plugin(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args)?;

    let bytes: Vec<u8> = match args.items.as_slice() {
        [Value::Str(s)] => {
            let data = world.read_bytes(current_file, s.as_str()).map_err(|msg| {
                err(format!("plugin(): não foi possível ler '{}': {msg}", s))
            })?;
            data.to_vec()
        }
        [Value::Bytes(b)] => b.as_slice().to_vec(),
        [other] => {
            return Err(err(format!(
                "plugin() requer caminho (str) ou bytes, recebeu {}",
                other.type_name()
            )));
        }
        _ => {
            return Err(err(format!(
                "plugin() requer 1 argumento, recebeu {}",
                args.items.len()
            )));
        }
    };

    // P699 — resolve o host injectado, carrega o módulo, enumera os exports
    // (funções) e constrói um `Module` com uma `Func::plugin(PluginFunc{...})`
    // por export. `PluginError.message` propaga verbatim (observável — ADR-0107).
    let host = world
        .plugin_host()
        .ok_or_else(|| err("plugins não suportados neste World"))?;

    let id = host.load(&bytes).map_err(|e| err(e.message.to_string()))?;
    let names = host.exports(id).map_err(|e| err(e.message.to_string()))?;

    let mut scope = Scope::new();
    for name in names {
        let pf = PluginFunc {
            host: Arc::clone(&host),
            module: id,
            name: name.clone(),
        };
        scope.define(name.as_str(), Value::Func(Func::plugin(pf)));
    }

    Ok(Value::Module(Module::new("plugin", scope)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;
    use ecow::EcoString;
    use std::collections::HashMap;
    use std::num::NonZeroU16;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};

    struct MockWorld {
        files: HashMap<String, Arc<Vec<u8>>>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl Default for MockWorld {
        fn default() -> Self {
            Self {
                files: HashMap::new(),
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    /// Host de teste (sem WASM): exports configuráveis por módulo e `load`
    /// que pode falhar com mensagem fixa.
    struct StubHost {
        exports_map: Mutex<HashMap<u64, Vec<EcoString>>>,
        fail_load: Mutex<Option<String>>,
        next: AtomicU64,
    }
    impl StubHost {
        fn new() -> Self {
            Self {
                exports_map: Mutex::new(HashMap::new()),
                fail_load: Mutex::new(None),
                next: AtomicU64::new(1),
            }
        }
        fn set_exports(&self, id: PluginModuleId, names: Vec<EcoString>) {
            self.exports_map.lock().unwrap().insert(id.0, names);
        }
        fn set_fail_load(&self, msg: &str) {
            *self.fail_load.lock().unwrap() = Some(msg.to_string());
        }
    }
    impl PluginHost for StubHost {
        fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
            if let Some(msg) = self.fail_load.lock().unwrap().clone() {
                return Err(PluginError::new(msg));
            }
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
        fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
            Ok(self
                .exports_map
                .lock()
                .unwrap()
                .get(&module.0)
                .cloned()
                .unwrap_or_default())
        }
        fn call(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            Ok(Bytes::new(Vec::new()))
        }
    }

    struct WorldWithHost {
        host: Arc<StubHost>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl WorldWithHost {
        fn new(host: Arc<StubHost>) -> Self {
            Self {
                host,
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for WorldWithHost {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn plugin_host(&self) -> Option<Arc<dyn PluginHost>> {
            Some(Arc::clone(&self.host) as Arc<dyn PluginHost>)
        }
    }

    fn fid() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn plugin_sem_host_devolve_erro_suporte() {
        let world = MockWorld::default();
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![0, 1, 2, 3]))]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(
            e[0].message.contains("plugins não suportados neste World"),
            "msg: {}",
            e[0].message,
        );
    }

    #[test]
    fn plugin_devolve_module_com_funcao_por_export() {
        let host = Arc::new(StubHost::new());
        let world = WorldWithHost::new(Arc::clone(&host));
        // Primeiro carregamos para descobrir o id atribuído ao módulo.
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![
            0x00, 0x61, 0x73, 0x6d,
        ]))]);
        // O StubHost atribui id=1 no primeiro load; registamos os exports
        // antes de chamar, usando o mesmo id (next começa em 1).
        host.set_exports(
            PluginModuleId(1),
            vec![EcoString::from("hello"), EcoString::from("add")],
        );

        let v = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap();
        let m = match v {
            Value::Module(m) => m,
            other => panic!("esperava Module, recebeu {}", other.type_name()),
        };
        assert_eq!(m.name(), "plugin");
        assert!(matches!(m.scope().get("hello"), Some(Value::Func(_))), "hello ausente");
        assert!(matches!(m.scope().get("add"), Some(Value::Func(_))), "add ausente");
    }

    #[test]
    fn plugin_host_load_falha_propaga_verbatim() {
        let host = Arc::new(StubHost::new());
        host.set_fail_load("plugin does not export its memory");
        let world = WorldWithHost::new(Arc::clone(&host));
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![0, 1, 2]))]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].message.as_str(), "plugin does not export its memory");
    }

    #[test]
    fn plugin_caminho_inexistente_erro_de_leitura() {
        let world = MockWorld::default();
        let args = Args::positional(vec![Value::Str("nao-existe.wasm".into())]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(
            e[0].message
                .contains("plugin(): não foi possível ler 'nao-existe.wasm'"),
            "msg: {}",
            e[0].message
        );
        assert!(!e[0].message.contains("unknown variable"), "msg: {}", e[0].message);
    }

    #[test]
    fn plugin_tipo_errado_e_aridade_dao_erro_de_argumento() {
        let world = MockWorld::default();
        // tipo errado
        let args = Args::positional(vec![Value::Int(42)]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(
            e[0].message.contains("requer caminho (str) ou bytes"),
            "msg: {}",
            e[0].message
        );
        // aridade errada
        let args = Args::positional(vec![]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("requer 1 argumento"), "msg: {}", e[0].message);
    }
}
