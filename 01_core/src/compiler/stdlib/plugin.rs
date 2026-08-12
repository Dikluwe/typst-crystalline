//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/plugin.md
//! @prompt-hash 17428035
//! @layer L1
//! @updated 2026-07-22
//!
//! Builtin `plugin()` (P697/P699) — lê o payload WASM (caminho via
//! `World::read_bytes`, ou `bytes` directos), carrega-o no `PluginHost`
//! injectado via `World::plugin_host()` e devolve um `Module` real com uma
//! função (`Func::plugin(PluginFunc{...})`) por export do módulo (P699).
//! **P819** — `plugin.transition` (transition API), mensagens verbatim do
//! vanilla e spans do callsite (`Args::span`, P772s) em vez de `<detached>`.

use std::sync::Arc;

use crate::compiler::eval::long_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::module::Module;
use crate::entities::plugin_func::PluginFunc;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn err(span: Span, msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(span, msg.into())]
}

fn reject_named(fname: &str, args: &Args) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(
            args.span,
            format!("argumento nomeado inesperado em {fname}(): '{k}'"),
        ));
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
/// **P819** — mensagens verbatim do vanilla (ficheiro inexistente propagado
/// de L3 sem prefixo; tipo do argumento) e span do callsite (`args.span`).
pub fn native_plugin(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named("plugin", args)?;
    let span = args.span;

    let bytes: Vec<u8> = match args.items.as_slice() {
        [Value::Str(s)] => {
            // P819 — o erro de leitura propaga verbatim de L3 (formato do
            // `FileError` do vanilla: "file not found (searched at …)").
            let data = world
                .read_bytes(current_file, s.as_str())
                .map_err(|msg| err(span, msg))?;
            data.to_vec()
        }
        [Value::Bytes(b)] => b.as_slice().to_vec(),
        [other] => {
            return Err(err(
                span,
                format!(
                    "expected path, string, or bytes, found {}",
                    long_type_name(other)
                ),
            ));
        }
        _ => {
            return Err(err(
                span,
                format!(
                    "plugin() requer 1 argumento, recebeu {}",
                    args.items.len()
                ),
            ));
        }
    };

    // P699 — resolve o host injectado, carrega o módulo, enumera os exports
    // (funções) e constrói um `Module` com uma `Func::plugin(PluginFunc{...})`
    // por export. `PluginError.message` propaga verbatim (observável — ADR-0107).
    let host = world
        .plugin_host()
        .ok_or_else(|| err(span, "plugins não suportados neste World"))?;

    let id = host.load(&bytes).map_err(|e| err(span, e.message.to_string()))?;
    let names = host.exports(id).map_err(|e| err(span, e.message.to_string()))?;

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

/// **P819** — `plugin.transition(func, ..args)` — transition API do vanilla
/// (`plugin.rs:158-202`): executa a chamada mutável `func(args)` e devolve um
/// `Module` derivado cujas funções observam a mutação (o módulo original
/// fica inalterado). Mensagens verbatim medidas em P819 (tm2–tm6):
/// `missing argument: func`, `expected function, found <tipo>`,
/// `expected plugin function`, `expected bytes, found <tipo>`.
pub fn native_plugin_transition(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    reject_named("plugin.transition", args)?;
    let span = args.span;

    let mut items = args.items.iter();
    let func_value = items
        .next()
        .ok_or_else(|| err(span, "missing argument: func"))?;
    let func = match func_value {
        Value::Func(f) => f,
        other => {
            return Err(err(
                span,
                format!("expected function, found {}", long_type_name(other)),
            ));
        }
    };
    let pf = match func.repr() {
        FuncRepr::Plugin(p) => p,
        _ => return Err(err(span, "expected plugin function")),
    };

    let mut bufs: Vec<crate::entities::bytes::Bytes> = Vec::with_capacity(items.len());
    for v in items {
        match v {
            Value::Bytes(b) => bufs.push(b.clone()),
            other => {
                return Err(err(
                    span,
                    format!("expected bytes, found {}", long_type_name(other)),
                ));
            }
        }
    }

    // `PluginFunc::transition` é memoizada (P819) — o host é chamado uma vez
    // por chave `(module, name, args)`; a mensagem propaga verbatim.
    match pf.transition(bufs) {
        Ok(module) => Ok(Value::Module(module)),
        Err(e) => Err(err(span, e.message.to_string())),
    }
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
            self.files.get(path).cloned().ok_or_else(|| {
                // P819 — o contrato de `read_bytes` mudou: a mensagem é o
                // `Display` do `FileError` do vanilla (propagada verbatim).
                format!("file not found (searched at {})", path)
            })
        }
    }

    /// Host de teste (sem WASM): exports configuráveis por módulo e `load`
    /// que pode falhar com mensagem fixa.
    struct StubHost {
        exports_map: Mutex<HashMap<u64, Vec<EcoString>>>,
        fail_load: Mutex<Option<String>>,
        next: AtomicU64,
        /// P819 — contagem de `transition` (prova de memoização em L1).
        transitions: AtomicU64,
    }
    impl StubHost {
        fn new() -> Self {
            Self {
                exports_map: Mutex::new(HashMap::new()),
                fail_load: Mutex::new(None),
                next: AtomicU64::new(1),
                transitions: AtomicU64::new(0),
            }
        }
        fn set_exports(&self, id: PluginModuleId, names: Vec<EcoString>) {
            self.exports_map.lock().unwrap().insert(id.0, names);
        }
        fn set_fail_load(&self, msg: &str) {
            *self.fail_load.lock().unwrap() = Some(msg.to_string());
        }
        fn transitions(&self) -> u64 {
            self.transitions.load(Ordering::Relaxed)
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
        fn transition(
            &self,
            module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            self.transitions.fetch_add(1, Ordering::Relaxed);
            let derived = PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed));
            // O derivado herda os exports do módulo de origem (como o host real).
            let names = self
                .exports_map
                .lock()
                .unwrap()
                .get(&module.0)
                .cloned()
                .unwrap_or_default();
            self.exports_map.lock().unwrap().insert(derived.0, names);
            Ok(derived)
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
        // P819 — o erro de leitura propaga verbatim de L3 (sem prefixo PT).
        assert_eq!(
            e[0].message.as_str(),
            "file not found (searched at nao-existe.wasm)",
            "msg: {}",
            e[0].message,
        );
        assert!(!e[0].message.contains("unknown variable"), "msg: {}", e[0].message);
    }

    #[test]
    fn plugin_tipo_errado_e_aridade_dao_erro_de_argumento() {
        let world = MockWorld::default();
        // tipo errado — P819: verbatim do vanilla (cast de DataSource).
        let args = Args::positional(vec![Value::Int(42)]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(
            e[0].message.as_str(),
            "expected path, string, or bytes, found integer",
            "msg: {}",
            e[0].message
        );
        // aridade errada (texto PT de P697 — routing de aridade fora de P819)
        let args = Args::positional(vec![]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("requer 1 argumento"), "msg: {}", e[0].message);
    }

    // ----- P819 — plugin.transition / spans --------------------------------

    /// Constrói um módulo de plugin via `native_plugin` com um StubHost e
    /// devolve (host, world, módulo). Os exports ficam registados para o id
    /// atribuído no primeiro `load` (1).
    fn module_com_exports(names: &[&str]) -> (Arc<StubHost>, WorldWithHost, Module) {
        let host = Arc::new(StubHost::new());
        host.set_exports(
            PluginModuleId(1),
            names.iter().map(|n| EcoString::from(*n)).collect(),
        );
        let world = WorldWithHost::new(Arc::clone(&host));
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![
            0x00, 0x61, 0x73, 0x6d,
        ]))]);
        let v = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap();
        match v {
            Value::Module(m) => (host, world, m),
            other => panic!("esperava Module, recebeu {}", other.type_name()),
        }
    }

    #[test]
    fn plugin_transition_caminho_feliz_devolve_module_derivado() {
        let (host, world, m) = module_com_exports(&["p819_std_add", "p819_std_get"]);
        let func = m.scope().get("p819_std_add").cloned().unwrap();
        let args = Args::positional(vec![func, Value::Bytes(Bytes::new(b"x".to_vec()))]);
        let v =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap();
        let derived = match v {
            Value::Module(m) => m,
            other => panic!("esperava Module, recebeu {}", other.type_name()),
        };
        assert_eq!(derived.name(), "plugin");
        assert!(
            matches!(derived.scope().get("p819_std_get"), Some(Value::Func(_))),
            "derivado sem p819_std_get",
        );
        assert_eq!(host.transitions(), 1);
    }

    #[test]
    fn plugin_transition_cacheia_segunda_transicao_identica() {
        let (host, world, m) = module_com_exports(&["p819_std_cache"]);
        let func = m.scope().get("p819_std_cache").cloned().unwrap();
        let mk_args = || {
            Args::positional(vec![func.clone(), Value::Bytes(Bytes::new(b"c".to_vec()))])
        };
        let _ = native_plugin_transition(&mut EvalContext::new(), &mk_args(), &world, fid())
            .unwrap();
        let _ = native_plugin_transition(&mut EvalContext::new(), &mk_args(), &world, fid())
            .unwrap();
        assert_eq!(
            host.transitions(),
            1,
            "segunda transição idêntica deve bater no cache (host chamado 1 vez)",
        );
    }

    #[test]
    fn plugin_transition_erros_verbatim() {
        let world = MockWorld::default();
        // missing argument: func
        let args = Args::positional(vec![]);
        let e =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].message.as_str(), "missing argument: func");
        // expected function, found integer
        let args = Args::positional(vec![Value::Int(42)]);
        let e =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].message.as_str(), "expected function, found integer");
        // expected plugin function (função nativa)
        let args = Args::positional(vec![Value::Func(Func::native("plugin", native_plugin))]);
        let e =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].message.as_str(), "expected plugin function");
        // expected bytes, found integer
        let (_host, world, m) = module_com_exports(&["p819_std_args"]);
        let func = m.scope().get("p819_std_args").cloned().unwrap();
        let args = Args::positional(vec![func, Value::Int(1)]);
        let e =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].message.as_str(), "expected bytes, found integer");
    }

    #[test]
    fn plugin_erros_usam_span_dos_args_nao_detached() {
        let world = MockWorld::default();
        let span = Span::from_range(fid(), 3..10);
        let args = Args {
            items: vec![Value::Int(42)],
            named: Default::default(),
            span,
        };
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].span, span, "span do erro de plugin() deve ser args.span");
        assert!(!e[0].span.is_detached());

        // O mesmo para `plugin.transition()`.
        let args = Args { items: vec![], named: Default::default(), span };
        let e =
            native_plugin_transition(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert_eq!(e[0].span, span, "span do erro de plugin.transition deve ser args.span");
    }
}
