//! Plugin cache identity lifetime regression, outside pure L1 because its
//! eviction isolation launches a subprocess. Uses real public domain APIs.

use ecow::EcoString;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use typst_core::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use typst_core::entities::bytes::Bytes;
use typst_core::entities::plugin_func::PluginFunc;

#[derive(Clone, Copy, PartialEq, Eq)]
enum IdentityFailure {
    None,
    Call,
    Transition,
    Exports,
    EmptyExports,
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
        if self.failure == IdentityFailure::EmptyExports {
            return Ok(Vec::new());
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

#[test]
fn cached_hosts_live_until_eviction() {
    // Global eviction must not race unrelated tests of one-call memoization.
    const CHILD: &str = "TEKT_PLUGIN_IDENTITY_EVICTION_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "cached_hosts_live_until_eviction", "--test-threads=1"])
            .env(CHILD, "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    comemo::evict(0);
    for (index, failure) in [
        IdentityFailure::None,
        IdentityFailure::Call,
        IdentityFailure::Transition,
        IdentityFailure::Exports,
        IdentityFailure::EmptyExports,
    ]
    .into_iter()
    .enumerate()
    {
        let host = IdentityHost::new("retained host", failure);
        let weak = Arc::downgrade(&host);
        let name = format!("identity_retained_{index}");
        let function = identity_func(&host, &name);
        match failure {
            IdentityFailure::None => {
                function.call(identity_args()).unwrap();
            }
            IdentityFailure::Call => {
                function.call(identity_args()).unwrap_err();
            }
            IdentityFailure::EmptyExports => {
                let derived = function.transition(identity_args()).unwrap();
                assert!(derived.scope().get("identity_read").is_none());
                drop(derived);
            }
            _ => {
                function.transition(identity_args()).unwrap_err();
            }
        }
        drop(function);
        drop(host);
        assert!(
            weak.upgrade().is_some(),
            "hashed host identity must stay allocated while its cache entry survives"
        );
        comemo::evict(0);
        assert!(
            weak.upgrade().is_none(),
            "eviction must release cached ownership, including failures"
        );
    }
    // Recreate hosts with the same local identity after dropping all callers.
    for tag in ["sequential A", "sequential B", "sequential C"] {
        let host = IdentityHost::new(tag, IdentityFailure::None);
        let weak = Arc::downgrade(&host);
        let function = identity_func(&host, "identity_sequential");
        assert_eq!(
            function.call(Vec::new()).unwrap().as_slice(),
            format!("{tag}:1:").as_bytes()
        );
        drop(function);
        drop(host);
        assert!(weak.upgrade().is_some());
    }
    comemo::evict(0);
}
