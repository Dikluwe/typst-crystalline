//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1295_watch_contract.md
//! @prompt-hash e1000023
//! @layer L3
//! @updated 2026-09-02
//!
//! Contrato externo independente para o snapshot opaco de watch do P1295.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use typst_infra::watch::{
    snapshot, wait_for_change, wait_for_change_since, WatchSnapshot,
};

static NEXT_CASE: AtomicUsize = AtomicUsize::new(0);

const PINNED_RUSTC: &str = "rustc 1.92.0 (ded5c06cf 2025-12-08)";
const PINNED_RUSTDOC: &str = "rustdoc 1.92.0 (ded5c06cf 2025-12-08)";
const PINNED_CARGO: &str = "cargo 1.92.0 (344c4567c 2025-10-21)";
const PINNED_RUSTDOC_FORMAT: u64 = 56;

struct TempCase {
    root: PathBuf,
}

impl TempCase {
    fn new(label: &str) -> Self {
        let serial = NEXT_CASE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "typst-p1295-watch-contract-{label}-{}-{serial}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("criar fixture P1295");
        Self { root }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

impl Drop for TempCase {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn assert_wait_returns(snapshot: WatchSnapshot, phase: &str) {
    let (returned_tx, returned_rx) = mpsc::channel();
    let waiter = thread::spawn(move || {
        wait_for_change_since(snapshot, Duration::from_millis(5));
        returned_tx
            .send(())
            .expect("receptor do contrato deve permanecer vivo");
    });

    returned_rx
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|error| {
            panic!("wait_for_change_since não retornou em {phase}: {error}")
        });
    waiter.join().expect("thread de wait não deve entrar em panic");
}

#[test]
fn snapshot_anterior_detecta_alteracao_de_mesmo_tamanho_sem_recaptura() {
    let fixture = TempCase::new("same-length");
    let path = fixture.path("observed.txt");
    fs::write(&path, "alpha").unwrap();

    let captured = snapshot(std::slice::from_ref(&path));
    fs::write(&path, "omega").unwrap();

    assert_wait_returns(captured, "alteração de mesmo tamanho já capturada");
}

#[test]
fn snapshot_anterior_detecta_criacao_sem_recaptura() {
    let fixture = TempCase::new("creation");
    let path = fixture.path("created-later.txt");
    assert!(!path.exists());

    let captured = snapshot(std::slice::from_ref(&path));
    fs::write(&path, "created").unwrap();

    assert_wait_returns(captured, "criação posterior ao snapshot");
}

#[test]
fn snapshot_anterior_detecta_remocao_sem_recaptura() {
    let fixture = TempCase::new("removal");
    let path = fixture.path("removed-later.txt");
    fs::write(&path, "present").unwrap();

    let captured = snapshot(std::slice::from_ref(&path));
    fs::remove_file(&path).unwrap();

    assert_wait_returns(captured, "remoção posterior ao snapshot");
}

#[test]
fn snapshot_e_api_legada_preservam_assinaturas_publicas() {
    let capture: fn(&[PathBuf]) -> WatchSnapshot = snapshot;
    let wait_since: fn(WatchSnapshot, Duration) = wait_for_change_since;
    let legacy: fn(&[PathBuf], Duration) = wait_for_change;

    let token = capture(&[]);
    let transported: WatchSnapshot = token;
    let _ = (wait_since, legacy, transported);
}

#[cfg(target_os = "linux")]
#[test]
fn wait_for_change_legado_permanece_funcional_sem_race_de_prontidao() {
    let changing = PathBuf::from("/proc/sys/kernel/random/uuid");
    assert!(changing.exists(), "pseudo-ficheiro UUID ausente");
    let (returned_tx, returned_rx) = mpsc::channel();
    let waiter = thread::spawn(move || {
        wait_for_change(&[changing], Duration::from_millis(5));
        returned_tx
            .send(())
            .expect("receptor do teste legado deve permanecer vivo");
    });

    returned_rx
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|error| panic!("wait_for_change legado não retornou: {error}"));
    waiter
        .join()
        .expect("thread de wait_for_change legado não deve entrar em panic");
}

fn command_version(program: impl AsRef<std::ffi::OsStr>) -> String {
    let output = Command::new(program)
        .arg("--version")
        .output()
        .expect("executar versão da toolchain pinada");
    assert!(
        output.status.success(),
        "consulta de versão falhou: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("versão deve ser UTF-8")
        .trim()
        .to_owned()
}

fn inferred_target_dir() -> PathBuf {
    let executable =
        std::env::current_exe().expect("resolver executável do integration test");
    executable
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("inferir CARGO_TARGET_DIR a partir de target/<profile>/deps")
        .to_path_buf()
}

fn id_key(id: &Value, context: &str) -> String {
    id.as_u64()
        .unwrap_or_else(|| panic!("ID não numérico em {context}: {id}"))
        .to_string()
}

fn signature_mentions_id(value: &Value, target: u64) -> bool {
    match value {
        Value::Array(values) => {
            values.iter().any(|value| signature_mentions_id(value, target))
        }
        Value::Object(object) => {
            let resolved_here = object
                .get("resolved_path")
                .and_then(Value::as_object)
                .and_then(|path| path.get("id"))
                .and_then(Value::as_u64)
                == Some(target);
            resolved_here
                || object.values().any(|value| signature_mentions_id(value, target))
        }
        _ => false,
    }
}

fn assert_private_struct_storage(struct_data: &Value) {
    let kind = struct_data["kind"].as_object().expect("struct.kind deve ser objeto");
    assert_eq!(kind.len(), 1, "struct.kind deve ter uma variante: {kind:?}");

    if let Some(plain) = kind.get("plain") {
        let plain = plain.as_object().expect("struct plain deve ser objeto");
        let fields = plain["fields"].as_array().expect("plain.fields deve ser array");
        assert!(fields.is_empty(), "WatchSnapshot expõe field público: {fields:?}");
        assert_eq!(
            plain["has_stripped_fields"].as_bool(),
            Some(true),
            "WatchSnapshot named deve possuir estado privado"
        );
        return;
    }

    if let Some(tuple) = kind.get("tuple") {
        let fields = tuple.as_array().expect("struct tuple deve ser array");
        assert!(!fields.is_empty(), "WatchSnapshot tuple não possui estado privado");
        assert!(
            fields.iter().all(Value::is_null),
            "WatchSnapshot tuple expõe field público: {fields:?}"
        );
        return;
    }

    panic!("WatchSnapshot deve ter estado privado named ou tuple: {kind:?}");
}

#[test]
fn rustdoc_json_pinado_prova_superficie_opaca_e_fechada() {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let rustdoc = std::env::var_os("RUSTDOC").unwrap_or_else(|| "rustdoc".into());
    assert_eq!(command_version(&cargo), PINNED_CARGO);
    assert_eq!(command_version(&rustc), PINNED_RUSTC);
    assert_eq!(command_version(&rustdoc), PINNED_RUSTDOC);

    let target_dir = inferred_target_dir();
    let json_path = target_dir.join("doc/typst_infra.json");
    let _ = fs::remove_file(&json_path);
    let output = Command::new(&cargo)
        .args([
            "rustdoc",
            "-p",
            "typst-infra",
            "--lib",
            "--offline",
            "--",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
        ])
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTC_BOOTSTRAP", "1")
        .output()
        .expect("executar rustdoc JSON pinado");
    assert!(
        output.status.success(),
        "rustdoc JSON falhou:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let document: Value = serde_json::from_slice(
        &fs::read(&json_path).expect("rustdoc deve produzir target/doc/typst_infra.json"),
    )
    .expect("rustdoc JSON deve ser válido");
    assert_eq!(
        document["format_version"].as_u64(),
        Some(PINNED_RUSTDOC_FORMAT),
        "schema rustdoc JSON divergente"
    );
    assert_eq!(document["includes_private"].as_bool(), Some(false));

    let paths = document["paths"].as_object().expect("paths deve ser objeto");
    let index = document["index"].as_object().expect("index deve ser objeto");
    let expected_type_path = ["typst_infra", "watch", "WatchSnapshot"];
    let matches = paths
        .iter()
        .filter(|(_, metadata)| {
            metadata["crate_id"].as_u64() == Some(0)
                && metadata["kind"].as_str() == Some("struct")
                && metadata["path"]
                    .as_array()
                    .map(|segments| {
                        segments.iter().map(Value::as_str).collect::<Option<Vec<_>>>()
                            == Some(expected_type_path.to_vec())
                    })
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "WatchSnapshot deve ter um único ID público");

    let (snapshot_id_key, _) = matches[0];
    let snapshot_id = snapshot_id_key
        .parse::<u64>()
        .expect("ID de WatchSnapshot deve ser numérico");
    let snapshot_item = index
        .get(snapshot_id_key)
        .expect("ID de WatchSnapshot deve existir no index");
    assert_eq!(snapshot_item["visibility"].as_str(), Some("public"));
    let struct_data = snapshot_item["inner"]["struct"]
        .as_object()
        .expect("WatchSnapshot deve ser struct")
        .clone();
    let struct_value = Value::Object(struct_data.clone());
    assert_private_struct_storage(&struct_value);

    for impl_id in struct_data["impls"].as_array().expect("impls deve ser array") {
        let impl_item = index
            .get(&id_key(impl_id, "WatchSnapshot.impls"))
            .expect("impl ID deve existir no index");
        let implementation = impl_item["inner"]["impl"]
            .as_object()
            .expect("impl ID deve referenciar impl");
        let public_items =
            implementation["items"].as_array().expect("impl.items deve ser array");
        if implementation["trait"].is_null() {
            assert!(
                public_items.is_empty(),
                "WatchSnapshot possui item inherent público: {public_items:?}"
            );
        } else {
            let automatic = implementation["is_synthetic"].as_bool() == Some(true);
            let blanket = !implementation["blanket_impl"].is_null();
            assert!(
                automatic || blanket,
                "WatchSnapshot possui trait impl direto público: {}",
                implementation["trait"]
            );
        }
    }

    let mut token_functions = Vec::new();
    for (item_id, metadata) in paths {
        if metadata["crate_id"].as_u64() != Some(0)
            || metadata["kind"].as_str() != Some("function")
        {
            continue;
        }
        let item = index.get(item_id).expect("function ID deve existir no index");
        if item["visibility"].as_str() != Some("public") {
            continue;
        }
        let Some(signature) = item["inner"]["function"]["sig"].as_object() else {
            panic!("function path sem signature: {metadata}");
        };
        if signature_mentions_id(&Value::Object(signature.clone()), snapshot_id) {
            let path = metadata["path"]
                .as_array()
                .expect("function path deve ser array")
                .iter()
                .map(|segment| segment.as_str().expect("path segment deve ser string"))
                .collect::<Vec<_>>();
            token_functions.push(path);
        }
    }
    token_functions.sort_unstable();
    assert_eq!(
        token_functions,
        vec![
            vec!["typst_infra", "watch", "snapshot"],
            vec!["typst_infra", "watch", "wait_for_change_since"],
        ],
        "superfície livre de WatchSnapshot divergiu"
    );
}
