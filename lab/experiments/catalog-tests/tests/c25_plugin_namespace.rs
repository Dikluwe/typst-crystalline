use ecow::EcoString;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use typst_core::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
use typst_core::entities::{bytes::Bytes, plugin_func::PluginFunc};
use typst_infra::plugin_host::WasmiPluginHost;

struct Host {
    tag: &'static str,
    fail: bool,
    calls: AtomicUsize,
    transitions: AtomicUsize,
}
impl Host {
    fn new(tag: &'static str, fail: bool) -> Arc<Self> {
        Arc::new(Self {
            tag,
            fail,
            calls: AtomicUsize::new(0),
            transitions: AtomicUsize::new(0),
        })
    }
}
impl PluginHost for Host {
    fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
        Ok(PluginModuleId(1))
    }
    fn exports(&self, _: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
        Ok(vec![self.tag.into()])
    }
    fn call(
        &self,
        module: PluginModuleId,
        _: &str,
        _: &[Bytes],
    ) -> Result<Bytes, PluginError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(PluginError::new(self.tag));
        }
        if module.0 == 0 {
            return Err(PluginError::new("plugin module not found"));
        }
        Ok(Bytes::new(self.tag.as_bytes().to_vec()))
    }
    fn transition(
        &self,
        module: PluginModuleId,
        _: &str,
        _: &[Bytes],
    ) -> Result<PluginModuleId, PluginError> {
        self.transitions.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(PluginError::new(self.tag));
        }
        if module.0 == 0 {
            return Err(PluginError::new("plugin module not found"));
        }
        Ok(PluginModuleId(module.0 + 100))
    }
}
fn function(host: Arc<dyn PluginHost>, id: u64, name: &str) -> PluginFunc {
    PluginFunc {
        host,
        module: PluginModuleId(id),
        name: name.into(),
    }
}
fn args() -> Vec<Bytes> {
    vec![Bytes::new(b"identical argument".to_vec())]
}

#[test]
fn control_same_host_call_and_transition_memoize() {
    let host = Host::new("same_host", false);
    let a = function(host.clone(), 1, "c25_control_repeat");
    let b = function(host.clone(), 1, "c25_control_repeat");
    for f in [&a, &b, &a] {
        assert_eq!(f.call(args()).unwrap().as_slice(), b"same_host");
        assert!(f.transition(args()).unwrap().scope().get("same_host").is_some());
    }
    assert_eq!(host.calls.load(Ordering::SeqCst), 1);
    assert_eq!(host.transitions.load(Ordering::SeqCst), 1);
}

#[test]
fn control_distinct_ids_and_failures_remain_distinct() {
    let a = Host::new("id_a", false);
    let b = Host::new("id_b", false);
    for (host, id, expected) in [(a, 11, b"id_a".as_slice()), (b, 12, b"id_b".as_slice())]
    {
        let f = function(host, id, "c25_control_ids");
        assert_eq!(f.call(args()).unwrap().as_slice(), expected);
        assert!(f.transition(args()).is_ok());
    }
    let f = function(Host::new("failure verbatim", true), 1, "c25_control_fail");
    assert_eq!(f.call(args()).unwrap_err().message.as_str(), "failure verbatim");
    assert_eq!(f.transition(args()).unwrap_err().message.as_str(), "failure verbatim");
    let missing = function(Host::new("unused", false), 0, "c25_control_missing");
    assert_eq!(
        missing.call(args()).unwrap_err().message.as_str(),
        "plugin module not found"
    );
    assert_eq!(
        missing.transition(args()).unwrap_err().message.as_str(),
        "plugin module not found"
    );
}

fn cross_host_call(reverse: bool, key: &str) {
    let hosts = [Host::new("host_a", false), Host::new("host_b", false)];
    for i in if reverse { [1, 0] } else { [0, 1] } {
        let f = function(hosts[i].clone(), 1, key);
        assert_eq!(
            f.call(args()).unwrap().as_slice(),
            hosts[i].tag.as_bytes(),
            "call result belongs to its captured host"
        );
    }
    assert_eq!(hosts[0].calls.load(Ordering::SeqCst), 1);
    assert_eq!(hosts[1].calls.load(Ordering::SeqCst), 1);
}
fn cross_host_transition(reverse: bool, key: &str) {
    let hosts = [Host::new("derived_a", false), Host::new("derived_b", false)];
    for i in if reverse { [1, 0] } else { [0, 1] } {
        let module = function(hosts[i].clone(), 1, key).transition(args()).unwrap();
        assert!(
            module.scope().get(hosts[i].tag).is_some(),
            "derived module belongs to its captured host"
        );
        assert!(module.scope().get(hosts[1 - i].tag).is_none());
    }
    assert_eq!(hosts[0].transitions.load(Ordering::SeqCst), 1);
    assert_eq!(hosts[1].transitions.load(Ordering::SeqCst), 1);
}
#[test]
fn red_call_ab() {
    cross_host_call(false, "c25_red_call_ab");
}
#[test]
fn red_call_ba() {
    cross_host_call(true, "c25_red_call_ba");
}
#[test]
fn red_transition_ab() {
    cross_host_transition(false, "c25_red_transition_ab");
}
#[test]
fn red_transition_ba() {
    cross_host_transition(true, "c25_red_transition_ba");
}

fn failure_namespace(name: &str, missing: bool, error_first: bool, transition: bool) {
    // One host issues zero legitimately; zero is missing in the other host.
    let good = if missing {
        function(Arc::new(ZeroHost), 0, name)
    } else {
        function(Host::new("ok", false), 1, name)
    };
    let bad =
        function(Host::new("host failure", !missing), if missing { 0 } else { 1 }, name);
    let expected = if missing { "plugin module not found" } else { "host failure" };
    for error in if error_first { [true, false] } else { [false, true] } {
        let f = if error { &bad } else { &good };
        if transition {
            let result = f.transition(args());
            if error {
                assert_eq!(result.unwrap_err().message.as_str(), expected);
            } else {
                assert!(
                    result.is_ok(),
                    "another host's cached error leaked into transition"
                );
            }
        } else {
            let result = f.call(args());
            if error {
                assert_eq!(result.unwrap_err().message.as_str(), expected);
            } else {
                assert_eq!(result.unwrap().as_slice(), b"ok");
            }
        }
    }
}
macro_rules! failure_case {
    ($name:ident, $missing:expr, $reverse:expr, $transition:expr) => {
        #[test]
        fn $name() {
            failure_namespace(stringify!($name), $missing, $reverse, $transition);
        }
    };
}
failure_case!(red_call_failure_ab, false, false, false);
failure_case!(red_call_failure_ba, false, true, false);
failure_case!(red_call_missing_ab, true, false, false);
failure_case!(red_call_missing_ba, true, true, false);
failure_case!(red_transition_failure_ab, false, false, true);
failure_case!(red_transition_failure_ba, false, true, true);
failure_case!(red_transition_missing_ab, true, false, true);
failure_case!(red_transition_missing_ba, true, true, true);
struct ZeroHost;
impl PluginHost for ZeroHost {
    fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
        Ok(PluginModuleId(0))
    }
    fn exports(&self, _: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
        Ok(vec!["zero".into()])
    }
    fn call(
        &self,
        _: PluginModuleId,
        _: &str,
        _: &[Bytes],
    ) -> Result<Bytes, PluginError> {
        Ok(Bytes::new(b"ok".to_vec()))
    }
    fn transition(
        &self,
        _: PluginModuleId,
        _: &str,
        _: &[Bytes],
    ) -> Result<PluginModuleId, PluginError> {
        Ok(PluginModuleId(10))
    }
}

// Minimal binary WASM encoder: real runtime and real typst_env ABI, no wat
// dependency. Each host's module sends its own byte from linear memory.
fn leb(mut n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    loop {
        let b = (n & 127) as u8;
        n >>= 7;
        v.push(b | if n == 0 { 0 } else { 128 });
        if n == 0 {
            return v;
        }
    }
}
fn string(s: &str) -> Vec<u8> {
    [leb(s.len()), s.as_bytes().to_vec()].concat()
}
fn section(id: u8, bytes: Vec<u8>, wasm: &mut Vec<u8>) {
    wasm.push(id);
    wasm.extend(leb(bytes.len()));
    wasm.extend(bytes);
}
fn wasm(tag: u8, name: &str, marker: &str) -> Vec<u8> {
    let mut w = b"\0asm\x01\0\0\0".to_vec();
    // type0 (i32,i32)->(), type1 ()->i32
    section(1, vec![2, 0x60, 2, 0x7f, 0x7f, 0, 0x60, 0, 1, 0x7f], &mut w);
    let mut imports = vec![1];
    imports.extend(string("typst_env"));
    imports.extend(string("wasm_minimal_protocol_send_result_to_host"));
    imports.extend([0, 0]);
    section(2, imports, &mut w);
    section(3, vec![1, 1], &mut w);
    section(5, vec![1, 0, 1], &mut w);
    let mut exports = vec![3];
    for (n, kind, id) in [("memory", 2, 0), (name, 0, 1), (marker, 0, 1)] {
        exports.extend(string(n));
        exports.extend([kind, id]);
    }
    section(7, exports, &mut w);
    // locals=0; i32.const 0; i32.const 1; call send; i32.const 0; end
    section(10, vec![1, 10, 0, 0x41, 0, 0x41, 1, 0x10, 0, 0x41, 0, 0x0b], &mut w);
    section(11, vec![1, 0, 0x41, 0, 0x0b, 1, tag], &mut w);
    w
}

#[test]
fn control_real_wasm_direct_hosts_return_own_payload_and_derivations() {
    for (tag, marker) in [(b'A', "runtime_a"), (b'B', "runtime_b")] {
        let h = WasmiPluginHost::new();
        let id = h.load(&wasm(tag, "c25_wasm_direct", marker)).unwrap();
        assert_eq!(h.call(id, "c25_wasm_direct", &[]).unwrap().as_slice(), &[tag]);
        let derived = h.transition(id, "c25_wasm_direct", &[]).unwrap();
        assert_ne!(id, derived);
        assert_eq!(h.call(derived, "c25_wasm_direct", &[]).unwrap().as_slice(), &[tag]);
        assert!(h.call(PluginModuleId(u64::MAX), "c25_wasm_direct", &[]).is_err());
        assert!(h.call(id, "absent", &[]).is_err());
    }
}

fn real_wasm_namespace(reverse: bool, transition: bool, key: &str) {
    let hosts = [Arc::new(WasmiPluginHost::new()), Arc::new(WasmiPluginHost::new())];
    let ids = [
        hosts[0].load(&wasm(b'A', key, "runtime_a")).unwrap(),
        hosts[1].load(&wasm(b'B', key, "runtime_b")).unwrap(),
    ];
    assert_eq!(ids[0], ids[1], "fixture requires host-local IDs to collide");
    for i in if reverse { [1, 0] } else { [0, 1] } {
        let f = function(hosts[i].clone(), ids[i].0, key);
        if transition {
            let m = f.transition(vec![]).unwrap();
            assert!(m
                .scope()
                .get(if i == 0 { "runtime_a" } else { "runtime_b" })
                .is_some());
            assert!(m
                .scope()
                .get(if i == 0 { "runtime_b" } else { "runtime_a" })
                .is_none());
        } else {
            assert_eq!(
                f.call(vec![]).unwrap().as_slice(),
                &[if i == 0 { b'A' } else { b'B' }]
            );
        }
    }
}
#[test]
fn red_real_wasm_call_ab() {
    real_wasm_namespace(false, false, "c25_real_call_ab");
}
#[test]
fn red_real_wasm_call_ba() {
    real_wasm_namespace(true, false, "c25_real_call_ba");
}
#[test]
fn red_real_wasm_transition_ab() {
    real_wasm_namespace(false, true, "c25_real_transition_ab");
}
#[test]
fn red_real_wasm_transition_ba() {
    real_wasm_namespace(true, true, "c25_real_transition_ba");
}
