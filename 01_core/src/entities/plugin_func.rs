//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/plugin_func.md
//! @prompt-hash a8626473
//! @layer L1
//! @updated 2026-07-11
//!
//! `PluginFunc` — export de plugin WASM chamável como função da linguagem
//! (P699). Vive em `FuncRepr::Plugin`; a chamada é delegada ao `PluginHost`
//! capturado e memoizada via `#[comemo::memoize]`.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use ecow::EcoString;

use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use crate::entities::bytes::Bytes;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::value::Value;

/// Export de plugin WASM chamável como função da linguagem.
///
/// `host` é partilhado por todos os exports do mesmo módulo (clone O(1));
/// `(module, name)` identifica o export de forma única **dentro do host**.
/// O `host` é excluído de `PartialEq`/`Hash` (não é `Hash`) — ver notas.
pub struct PluginFunc {
    /// Host que executa a chamada WASM (L3 em produção; mock em testes).
    pub host: Arc<dyn PluginHost>,
    /// Handle opaco do módulo (emitido por `PluginHost::load`).
    pub module: PluginModuleId,
    /// Nome do export (função) dentro do módulo.
    pub name: EcoString,
}

impl PluginFunc {
    /// Chama o export com os buffers `args` (cada `Bytes` é um argumento).
    ///
    /// **Memoizada** (`#[comemo::memoize]`): a chave de cache é
    /// `(module, name, args)` — `host` é excluído via `Hash` manual. Duas
    /// chamadas com os mesmos `(module, name, args)` só invocam o host uma
    /// vez; a segunda é servida do cache global do comemo.
    ///
    /// `Result<Bytes, PluginError>` é `Clone + Send + Sync` (requisito do
    /// output memoizado) e a mensagem de `PluginError` propaga verbatim
    /// (observável — ADR-0107).
    #[comemo::memoize]
    pub fn call(&self, args: Vec<Bytes>) -> Result<Bytes, PluginError> {
        self.host.call(self.module, self.name.as_str(), &args)
    }

    /// **P819** — transition API: executa a chamada **mutável** e devolve o
    /// `Module` derivado (mesmos exports; as chamadas sobre o derivado
    /// observam a mutação; o módulo original fica inalterado). Réplica de
    /// `PluginFunc::transition` do vanilla (`plugin.rs:226-231`).
    ///
    /// **Memoizada** como o vanilla: duas transições com a mesma chave
    /// `(module, name, args)` chamam `host.transition` uma vez e devolvem o
    /// mesmo derivado (o id fresco por derivação distingue "siblings" — o
    /// papel do fingerprint u128 do vanilla).
    #[comemo::memoize]
    pub fn transition(&self, args: Vec<Bytes>) -> Result<Module, PluginError> {
        let derived = self.host.transition(self.module, self.name.as_str(), &args)?;
        let names = self.host.exports(derived)?;
        let mut scope = Scope::new();
        for name in names {
            let pf = PluginFunc {
                host: Arc::clone(&self.host),
                module: derived,
                name: name.clone(),
            };
            scope.define(name.as_str(), Value::Func(Func::plugin(pf)));
        }
        Ok(Module::new("plugin", scope))
    }
}

/// Igualdade por `(module, name)` — `host` excluído. Duas `PluginFunc` do
/// mesmo export são a mesma função para efeitos de cache e identidade,
/// independentemente do `Arc` de host que as carrega.
impl PartialEq for PluginFunc {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.name == other.name
    }
}

impl Eq for PluginFunc {}

/// Hash por `(module, name)` — `host` excluído (não é `Hash`). Mantém a
/// chave de cache estável e independente do handle partilhado.
impl Hash for PluginFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.module.hash(state);
        self.name.hash(state);
    }
}

/// `Debug` manual — `Arc<dyn PluginHost>` não implementa `Debug`.
impl fmt::Debug for PluginFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PluginFunc")
            .field("module", &self.module)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Host de teste que conta chamadas a `call` e devolve bytes fixos por
    /// `(module, name)`. Não usa WASM — prova a memoização do `PluginFunc`.
    struct CountingHost {
        calls: AtomicUsize,
        /// Resposta por `(module_id, name)` → bytes.
        table: std::sync::Mutex<HashMap<(u64, String), Vec<u8>>>,
        next: AtomicUsize,
        /// P819 — contagem de `transition` (prova da memoização) e nomes de
        /// exports configuráveis (o derivado herda os mesmos nomes).
        transitions: AtomicUsize,
        export_names: std::sync::Mutex<Vec<EcoString>>,
    }

    impl CountingHost {
        fn new() -> Self {
            Self {
                calls: AtomicUsize::new(0),
                table: std::sync::Mutex::new(HashMap::new()),
                next: AtomicUsize::new(1),
                transitions: AtomicUsize::new(0),
                export_names: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn set(&self, module: PluginModuleId, name: &str, bytes: Vec<u8>) {
            self.table.lock().unwrap().insert((module.0, name.to_string()), bytes);
        }

        fn set_export_names(&self, names: &[&str]) {
            *self.export_names.lock().unwrap() =
                names.iter().map(|n| EcoString::from(*n)).collect();
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }

        fn transitions(&self) -> usize {
            self.transitions.load(Ordering::SeqCst)
        }
    }

    impl PluginHost for CountingHost {
        fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
            let id = self.next.fetch_add(1, Ordering::SeqCst) as u64;
            Ok(PluginModuleId(id))
        }

        fn exports(
            &self,
            _module: PluginModuleId,
        ) -> Result<Vec<EcoString>, PluginError> {
            Ok(self.export_names.lock().unwrap().clone())
        }

        fn call(
            &self,
            module: PluginModuleId,
            func_name: &str,
            _args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let bytes = self
                .table
                .lock()
                .unwrap()
                .get(&(module.0, func_name.to_string()))
                .cloned()
                .unwrap_or_default();
            Ok(Bytes::new(bytes))
        }

        fn transition(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            self.transitions.fetch_add(1, Ordering::SeqCst);
            let id = self.next.fetch_add(1, Ordering::SeqCst) as u64;
            Ok(PluginModuleId(id))
        }
    }

    fn make_func(
        host: &Arc<CountingHost>,
        module: PluginModuleId,
        name: &str,
    ) -> PluginFunc {
        PluginFunc {
            host: Arc::clone(host) as Arc<dyn PluginHost>,
            module,
            name: EcoString::from(name),
        }
    }

    #[test]
    fn call_devolve_bytes_do_host() {
        let host: Arc<CountingHost> = Arc::new(CountingHost::new());
        let id = host.load(&[]).unwrap();
        host.set(id, "p699_devolve_a", b"Ola".to_vec());
        let pf = make_func(&host, id, "p699_devolve_a");
        let out = pf.call(vec![Bytes::new(b"x".to_vec())]).unwrap();
        assert_eq!(out.as_slice(), b"Ola");
        assert_eq!(host.calls(), 1);
    }

    #[test]
    fn call_cacheia_segunda_chamada_identica() {
        let host: Arc<CountingHost> = Arc::new(CountingHost::new());
        let id = host.load(&[]).unwrap();
        // Nome único a este teste (evita colisão no cache global do comemo
        // com outros testes que partilhem `(module, name, args)`).
        host.set(id, "p699_cache_probe_unico", b"v".to_vec());
        let pf = make_func(&host, id, "p699_cache_probe_unico");
        let args = vec![Bytes::new(b"arg-unico".to_vec())];
        let o1 = pf.call(args.clone()).unwrap();
        let o2 = pf.call(args).unwrap();
        assert_eq!(o1.as_slice(), b"v");
        assert_eq!(o2.as_slice(), b"v");
        assert_eq!(
            host.calls(),
            1,
            "segunda chamada idêntica deve bater no cache (host chamado 1 vez)",
        );
    }

    #[test]
    fn eq_e_hash_excluem_host() {
        use std::collections::hash_map::DefaultHasher;
        let h1: Arc<CountingHost> = Arc::new(CountingHost::new());
        let h2: Arc<CountingHost> = Arc::new(CountingHost::new());
        let id = PluginModuleId(42);
        let a = make_func(&h1, id, "p699_eq_nome");
        let b = make_func(&h2, id, "p699_eq_nome");
        // Hosts distintos, mesmo (module, name) ⇒ iguais e mesmo hash.
        assert_eq!(a, b);
        let hash = |pf: &PluginFunc| {
            let mut s = DefaultHasher::new();
            pf.hash(&mut s);
            s.finish()
        };
        assert_eq!(hash(&a), hash(&b));
    }

    // ----- P819 — transition ----------------------------------------------

    #[test]
    fn transition_devolve_module_derivado_com_os_mesmos_exports() {
        let host: Arc<CountingHost> = Arc::new(CountingHost::new());
        host.set_export_names(&["p819_get", "p819_add"]);
        let id = host.load(&[]).unwrap();
        let pf = make_func(&host, id, "p819_add");
        let m = pf.transition(vec![Bytes::new(b"x".to_vec())]).unwrap();
        assert_eq!(m.name(), "plugin");
        assert!(
            matches!(m.scope().get("p819_get"), Some(Value::Func(_))),
            "p819_get ausente no derivado",
        );
        assert!(
            matches!(m.scope().get("p819_add"), Some(Value::Func(_))),
            "p819_add ausente no derivado",
        );
        assert_eq!(host.transitions(), 1);
    }

    #[test]
    fn transition_cacheia_segunda_transicao_identica() {
        let host: Arc<CountingHost> = Arc::new(CountingHost::new());
        host.set_export_names(&["p819_transition_cache_unico"]);
        let id = host.load(&[]).unwrap();
        // Nome único a este teste (cache global do comemo — ver nota em
        // `call_cacheia_segunda_chamada_identica`).
        let pf = make_func(&host, id, "p819_transition_cache_unico");
        let args = vec![Bytes::new(b"arg-transition-unico".to_vec())];
        let m1 = pf.transition(args.clone()).unwrap();
        let m2 = pf.transition(args).unwrap();
        assert_eq!(
            host.transitions(),
            1,
            "segunda transição idêntica deve bater no cache (host chamado 1 vez)",
        );
        // O derivado cacheado é o mesmo módulo (mesmo conteúdo).
        assert!(m1.scope().get("p819_transition_cache_unico").is_some());
        assert!(m2.scope().get("p819_transition_cache_unico").is_some());
    }

    #[test]
    fn transition_erro_do_host_propaga_verbatim() {
        /// Host cuja `transition` falha com mensagem fixa.
        struct FailHost;
        impl PluginHost for FailHost {
            fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
                Ok(PluginModuleId(1))
            }
            fn exports(&self, _module: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
                Ok(Vec::new())
            }
            fn call(
                &self,
                _module: PluginModuleId,
                _func_name: &str,
                _args: &[Bytes],
            ) -> Result<Bytes, PluginError> {
                unreachable!()
            }
            fn transition(
                &self,
                _module: PluginModuleId,
                _func_name: &str,
                _args: &[Bytes],
            ) -> Result<PluginModuleId, PluginError> {
                Err(PluginError::new("plugin panicked: wasm `unreachable` instruction executed"))
            }
        }
        let pf = PluginFunc {
            host: Arc::new(FailHost),
            module: PluginModuleId(1),
            name: EcoString::from("p819_fail_transition"),
        };
        let e = pf.transition(vec![Bytes::new(b"y".to_vec())]).unwrap_err();
        assert_eq!(
            e.message.as_str(),
            "plugin panicked: wasm `unreachable` instruction executed",
        );
    }
}
