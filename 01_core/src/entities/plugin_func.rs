//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/plugin_func.md
//! @prompt-hash 7090312c
//! @layer L1
//! @updated 2026-11-05
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
/// A identidade da alocação de `host` participa de `PartialEq`/`Hash`.
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
    /// **Memoizada**: a chave inclui a identidade do host, módulo, export e
    /// argumentos. Arc clones do mesmo host partilham o cache; hosts distintos
    /// não confundem IDs locais coincidentes. A mensagem de `PluginError`
    /// propaga verbatim (observável — ADR-0107).
    pub fn call(&self, args: Vec<Bytes>) -> Result<Bytes, PluginError> {
        self.call_cached(args).1
    }

    /// O comemo guarda o hash dos inputs, mas não retém `self`. O Arc no
    /// output mantém o endereço do host válido até a eviction, mesmo em erro.
    #[comemo::memoize]
    fn call_cached(
        &self,
        args: Vec<Bytes>,
    ) -> (Arc<dyn PluginHost>, Result<Bytes, PluginError>) {
        let result = self.host.call(self.module, self.name.as_str(), &args);
        (Arc::clone(&self.host), result)
    }

    /// **P819** — transition API: executa a chamada **mutável** e devolve o
    /// `Module` derivado (mesmos exports; as chamadas sobre o derivado
    /// observam a mutação; o módulo original fica inalterado). Réplica de
    /// `PluginFunc::transition` do vanilla (`plugin.rs:226-231`).
    ///
    /// **Memoizada** como o vanilla: duas transições com a mesma chave
    /// `(host, module, name, args)` chamam `host.transition` uma vez e devolvem o
    /// mesmo derivado (o id fresco por derivação distingue "siblings" — o
    /// papel do fingerprint u128 do vanilla).
    pub fn transition(&self, args: Vec<Bytes>) -> Result<Module, PluginError> {
        self.transition_cached(args).1
    }

    /// Retém o host também quando transition/exports falham ou o módulo
    /// derivado não tem exports que o retenham por si mesmos.
    #[comemo::memoize]
    fn transition_cached(
        &self,
        args: Vec<Bytes>,
    ) -> (Arc<dyn PluginHost>, Result<Module, PluginError>) {
        let result = (|| {
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
        })();
        (Arc::clone(&self.host), result)
    }
}

/// Mesmo host, módulo e export identificam a mesma função. O host em cada
/// resultado memoizado impede reciclagem do endereço enquanto houver cache.
impl PartialEq for PluginFunc {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.host, &other.host)
            && self.module == other.module
            && self.name == other.name
    }
}

impl Eq for PluginFunc {}

/// Identidade de dados do Arc, módulo e export, consistente com `PartialEq`.
impl Hash for PluginFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.host).cast::<()>().hash(state);
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
    fn identity_eq_hash_include_host_and_preserve_arc_clones() {
        use std::collections::hash_map::DefaultHasher;
        let h1 = Arc::new(CountingHost::new());
        let h2 = Arc::new(CountingHost::new());
        let a = make_func(&h1, PluginModuleId(42), "identity_eq");
        let b = make_func(&h2, PluginModuleId(42), "identity_eq");
        let same = make_func(&Arc::clone(&h1), PluginModuleId(42), "identity_eq");
        assert_ne!(a, b, "local module IDs from separate hosts are not identity");
        assert_eq!(a, same);
        let hash = |pf: &PluginFunc| {
            let mut state = DefaultHasher::new();
            pf.hash(&mut state);
            state.finish()
        };
        assert_eq!(hash(&a), hash(&same));
        // A mechanical key regression guard, separate from language results.
        assert_ne!(hash(&a), hash(&b));
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
            fn exports(
                &self,
                _module: PluginModuleId,
            ) -> Result<Vec<EcoString>, PluginError> {
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
                Err(PluginError::new(
                    "plugin panicked: wasm `unreachable` instruction executed",
                ))
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

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum IdentityFailure {
        None,
        Call,
        Transition,
        Exports,
    }

    struct IdentityHost {
        tag: &'static str,
        failure: IdentityFailure,
        calls: AtomicUsize,
        transitions: AtomicUsize,
    }

    impl IdentityHost {
        fn new(tag: &'static str, failure: IdentityFailure) -> Arc<Self> {
            Arc::new(Self {
                tag,
                failure,
                calls: AtomicUsize::new(0),
                transitions: AtomicUsize::new(0),
            })
        }
    }

    impl PluginHost for IdentityHost {
        fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(1))
        }
        fn exports(&self, _: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
            if self.failure == IdentityFailure::Exports {
                return Err(PluginError::new(self.tag));
            }
            Ok(vec!["identity_read".into()])
        }
        fn call(
            &self,
            module: PluginModuleId,
            name: &str,
            args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.failure == IdentityFailure::Call {
                return Err(PluginError::new(self.tag));
            }
            if name == "identity_missing" {
                return Err(PluginError::new(format!("{} missing export", self.tag)));
            }
            let mut out = format!("{}:{}:", self.tag, module.0).into_bytes();
            for arg in args {
                out.extend_from_slice(arg.as_slice());
                out.push(0);
            }
            Ok(Bytes::new(out))
        }
        fn transition(
            &self,
            module: PluginModuleId,
            name: &str,
            _: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            self.transitions.fetch_add(1, Ordering::SeqCst);
            if self.failure == IdentityFailure::Transition {
                return Err(PluginError::new(self.tag));
            }
            if name == "identity_missing" {
                return Err(PluginError::new(format!("{} missing export", self.tag)));
            }
            Ok(PluginModuleId(module.0 + 100))
        }
    }

    fn identity_func(host: &Arc<IdentityHost>, name: &str) -> PluginFunc {
        PluginFunc {
            host: host.clone(),
            module: PluginModuleId(1),
            name: name.into(),
        }
    }

    fn identity_args() -> Vec<Bytes> {
        vec![Bytes::new(b"same argument".to_vec())]
    }

    fn identity_derived_read(module: &Module) -> Vec<u8> {
        let Some(Value::Func(func)) = module.scope().get("identity_read") else {
            panic!("derived module must retain its export");
        };
        let crate::entities::func::FuncRepr::Plugin(plugin) = func.repr() else {
            panic!("derived export must remain a plugin function");
        };
        plugin.call(Vec::new()).unwrap().as_slice().to_vec()
    }

    #[test]
    fn identity_cross_host_calls_ab_ba_and_clone_hits() {
        for reverse in [false, true] {
            let hosts = [
                IdentityHost::new("identity A", IdentityFailure::None),
                IdentityHost::new("identity B", IdentityFailure::None),
            ];
            let name = if reverse { "identity_call_ba" } else { "identity_call_ab" };
            for index in if reverse { [1, 0, 1, 0] } else { [0, 1, 0, 1] } {
                let f = identity_func(&hosts[index], name);
                let expected = format!("{}:1:same argument\0", hosts[index].tag);
                assert_eq!(
                    f.call(identity_args()).unwrap().as_slice(),
                    expected.as_bytes()
                );
            }
            for host in &hosts {
                assert_eq!(host.calls.load(Ordering::SeqCst), 1);
            }
        }
    }

    #[test]
    fn identity_full_argument_vector_distinguishes_calls() {
        let host = IdentityHost::new("identity args", IdentityFailure::None);
        let f = identity_func(&host, "identity_args");
        let a = vec![Bytes::new(b"ab".to_vec()), Bytes::new(b"c".to_vec())];
        let b = vec![Bytes::new(b"a".to_vec()), Bytes::new(b"bc".to_vec())];
        assert_eq!(f.call(a.clone()).unwrap().as_slice(), b"identity args:1:ab\0c\0");
        assert_eq!(f.call(b.clone()).unwrap().as_slice(), b"identity args:1:a\0bc\0");
        f.call(a).unwrap();
        f.call(b).unwrap();
        assert_eq!(host.calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn identity_cross_host_transitions_ab_ba_keep_derived_host() {
        for reverse in [false, true] {
            let hosts = [
                IdentityHost::new("derived A", IdentityFailure::None),
                IdentityHost::new("derived B", IdentityFailure::None),
            ];
            let name =
                if reverse { "identity_transition_ba" } else { "identity_transition_ab" };
            for index in if reverse { [1, 0, 1, 0] } else { [0, 1, 0, 1] } {
                let f = identity_func(&hosts[index], name);
                let derived = f.transition(identity_args()).unwrap();
                assert_eq!(
                    identity_derived_read(&derived),
                    format!("{}:101:", hosts[index].tag).as_bytes()
                );
                assert_eq!(
                    f.call(Vec::new()).unwrap().as_slice(),
                    format!("{}:1:", hosts[index].tag).as_bytes()
                );
            }
            for host in &hosts {
                assert_eq!(host.transitions.load(Ordering::SeqCst), 1);
            }
        }
    }

    #[test]
    fn identity_call_errors_and_missing_exports_do_not_cross_hosts() {
        for reverse in [false, true] {
            for missing in [false, true] {
                let failure =
                    if missing { IdentityFailure::None } else { IdentityFailure::Call };
                let hosts = [
                    IdentityHost::new("error A", failure),
                    IdentityHost::new("error B", failure),
                ];
                let name = if missing {
                    "identity_missing"
                } else if reverse {
                    "identity_error_ba"
                } else {
                    "identity_error_ab"
                };
                for index in if reverse { [1, 0, 1] } else { [0, 1, 0] } {
                    let expected = if missing {
                        format!("{} missing export", hosts[index].tag)
                    } else {
                        hosts[index].tag.to_string()
                    };
                    assert_eq!(
                        identity_func(&hosts[index], name)
                            .call(identity_args())
                            .unwrap_err()
                            .message
                            .as_str(),
                        expected
                    );
                }
                for host in &hosts {
                    assert_eq!(host.calls.load(Ordering::SeqCst), 1);
                }
            }
        }
    }

    #[test]
    fn identity_transition_and_exports_errors_do_not_cross_hosts() {
        for reverse in [false, true] {
            for failure in [IdentityFailure::Transition, IdentityFailure::Exports] {
                let hosts = [
                    IdentityHost::new("transition error A", failure),
                    IdentityHost::new("transition error B", failure),
                ];
                let name = match (reverse, failure) {
                    (false, IdentityFailure::Transition) => {
                        "identity_transition_error_ab"
                    }
                    (true, IdentityFailure::Transition) => "identity_transition_error_ba",
                    (false, _) => "identity_exports_error_ab",
                    (true, _) => "identity_exports_error_ba",
                };
                for index in if reverse { [1, 0, 1] } else { [0, 1, 0] } {
                    assert_eq!(
                        identity_func(&hosts[index], name)
                            .transition(identity_args())
                            .unwrap_err()
                            .message
                            .as_str(),
                        hosts[index].tag
                    );
                }
                for host in &hosts {
                    assert_eq!(host.transitions.load(Ordering::SeqCst), 1);
                }
            }
        }
    }

    #[test]
    fn identity_missing_transition_is_scoped_to_host() {
        for reverse in [false, true] {
            let hosts = [
                IdentityHost::new("missing A", IdentityFailure::None),
                IdentityHost::new("missing B", IdentityFailure::None),
            ];
            for index in if reverse { [1, 0, 1] } else { [0, 1, 0] } {
                let expected = format!("{} missing export", hosts[index].tag);
                assert_eq!(
                    identity_func(&hosts[index], "identity_missing")
                        .transition(identity_args())
                        .unwrap_err()
                        .message
                        .as_str(),
                    expected
                );
            }
            for host in &hosts {
                assert_eq!(host.transitions.load(Ordering::SeqCst), 1);
            }
        }
    }

    #[test]
    fn identity_full_argument_vector_distinguishes_transitions() {
        let host = IdentityHost::new("transition args", IdentityFailure::None);
        let f = identity_func(&host, "identity_transition_args");
        let a = vec![Bytes::new(b"ab".to_vec()), Bytes::new(b"c".to_vec())];
        let b = vec![Bytes::new(b"a".to_vec()), Bytes::new(b"bc".to_vec())];
        for args in [a.clone(), b.clone(), a, b] {
            assert_eq!(
                identity_derived_read(&f.transition(args).unwrap()),
                b"transition args:101:"
            );
        }
        assert_eq!(host.transitions.load(Ordering::SeqCst), 2);
    }
}
