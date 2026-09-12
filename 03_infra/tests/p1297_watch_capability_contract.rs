//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md
//! @prompt-hash c1624a1f
//! @layer L3
//! @updated 2026-09-03
//!
//! Contrato externo independente para a capacidade armada de watch do P1297/R1.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use typst_infra::watch::{
    arm, snapshot, wait_for_change, wait_for_change_since, ArmedWatch, WatchSnapshot,
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
            "typst-p1297-r1-watch-contract-{label}-{}-{serial}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("criar fixture P1297/R1");
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

fn assert_same_io_error(actual: &io::Error, expected: &io::Error, phase: &str) {
    assert_eq!(
        actual.kind(),
        expected.kind(),
        "publish alterou ErrorKind do rename em {phase}"
    );
    assert_eq!(
        actual.raw_os_error(),
        expected.raw_os_error(),
        "publish alterou raw_os_error do rename em {phase}"
    );
}

#[test]
fn abandon_detecta_some_para_none_sem_captura_lazy_ou_recaptura() {
    let fixture = TempCase::new("abandon-transition");
    let staging = fixture.path("observed-and-staging.typ");
    fs::write(&staging, "present before arm").unwrap();

    let armed = arm(std::slice::from_ref(&staging));
    let captured = armed.abandon(&staging);

    assert!(!staging.exists(), "abandon deve remover o staging observado");
    assert_wait_returns(captured, "abandon Some -> None já concluído");
}

#[test]
fn publish_detecta_staging_some_para_none_sem_captura_lazy_ou_recaptura() {
    let fixture = TempCase::new("publish-transition");
    let staging = fixture.path("observed-staging.pdf");
    let destination = fixture.path("destination.pdf");
    fs::write(&staging, b"new artifact").unwrap();
    fs::write(&destination, b"old artifact").unwrap();

    let armed = arm(std::slice::from_ref(&staging));
    let captured = armed
        .publish(&staging, &destination)
        .expect("publish deve renomear staging válido");

    assert!(!staging.exists(), "publish deve consumir o staging por rename");
    assert_eq!(fs::read(&destination).unwrap(), b"new artifact");
    assert_wait_returns(captured, "publish staging Some -> None já concluído");
}

#[test]
fn abandon_preserva_destino_valido() {
    let fixture = TempCase::new("abandon-preserves-destination");
    let staging = fixture.path("staging.pdf");
    let destination = fixture.path("destination.pdf");
    fs::write(&staging, b"abandoned artifact").unwrap();
    fs::write(&destination, b"last valid artifact").unwrap();

    let armed = arm(std::slice::from_ref(&staging));
    let _captured = armed.abandon(&staging);

    assert!(!staging.exists(), "abandon deve limpar staging removível");
    assert_eq!(
        fs::read(&destination).unwrap(),
        b"last valid artifact",
        "abandon nunca deve tocar no destino válido"
    );
}

#[test]
fn publish_falho_limpa_staging_e_preserva_erro_original_do_rename() {
    let fixture = TempCase::new("publish-error-cleanup-success");
    let staging = fixture.path("staging.pdf");
    let destination = fixture.path("missing-parent/destination.pdf");
    fs::write(&staging, b"completed artifact").unwrap();
    assert!(!destination.parent().unwrap().exists());

    let expected = fs::rename(&staging, &destination)
        .expect_err("controle deve medir falha de rename");
    assert!(staging.exists(), "rename falho deve preservar staging no controle");

    let armed = arm(std::slice::from_ref(&staging));
    let actual = match armed.publish(&staging, &destination) {
        Ok(_) => panic!("publish deveria devolver falha do rename"),
        Err(error) => error,
    };

    assert_same_io_error(&actual, &expected, "cleanup removível");
    assert!(
        !staging.exists(),
        "publish falho deve tentar remover staging em best-effort"
    );
    assert!(!destination.exists());
}

#[test]
fn publish_falho_nao_mascara_rename_quando_cleanup_tambem_falha() {
    let fixture = TempCase::new("publish-error-cleanup-failure");
    let staging = fixture.path("staging-directory");
    let destination = fixture.path("missing-parent/destination.pdf");
    fs::create_dir(&staging).unwrap();
    assert!(!destination.parent().unwrap().exists());

    let expected = fs::rename(&staging, &destination)
        .expect_err("controle deve medir falha de rename");
    let cleanup_error = fs::remove_file(&staging)
        .expect_err("controle deve medir falha distinta de cleanup");
    assert_ne!(
        (expected.kind(), expected.raw_os_error()),
        (cleanup_error.kind(), cleanup_error.raw_os_error()),
        "fixture deve distinguir erro original e erro de cleanup"
    );

    let armed = arm(&[]);
    let actual = match armed.publish(&staging, &destination) {
        Ok(_) => panic!("publish deveria devolver falha do rename"),
        Err(error) => error,
    };

    assert_same_io_error(&actual, &expected, "cleanup também falho");
    assert!(staging.is_dir(), "cleanup best-effort falho não deve inventar sucesso");
    assert!(!destination.exists());
}

#[test]
fn capacidade_consumidora_e_apis_p1295_preservam_assinaturas_publicas() {
    let arm_fn: fn(&[PathBuf]) -> ArmedWatch = arm;
    let publish_fn: fn(ArmedWatch, &Path, &Path) -> io::Result<WatchSnapshot> =
        ArmedWatch::publish;
    let abandon_fn: fn(ArmedWatch, &Path) -> WatchSnapshot = ArmedWatch::abandon;
    let capture_fn: fn(&[PathBuf]) -> WatchSnapshot = snapshot;
    let wait_since_fn: fn(WatchSnapshot, Duration) = wait_for_change_since;
    let legacy_fn: fn(&[PathBuf], Duration) = wait_for_change;

    let armed: ArmedWatch = arm_fn(&[]);
    let captured: WatchSnapshot = abandon_fn(armed, Path::new(""));
    let _ = (publish_fn, capture_fn, wait_since_fn, legacy_fn, captured);
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

fn assert_private_struct_storage(struct_data: &Value, type_name: &str) {
    let kind = struct_data["kind"]
        .as_object()
        .unwrap_or_else(|| panic!("{type_name}.kind deve ser objeto"));
    assert_eq!(kind.len(), 1, "{type_name}.kind deve ter uma variante: {kind:?}");

    if let Some(plain) = kind.get("plain") {
        let plain = plain
            .as_object()
            .unwrap_or_else(|| panic!("{type_name} plain deve ser objeto"));
        let fields = plain["fields"]
            .as_array()
            .unwrap_or_else(|| panic!("{type_name} plain.fields deve ser array"));
        assert!(fields.is_empty(), "{type_name} expõe field público: {fields:?}");
        assert_eq!(
            plain["has_stripped_fields"].as_bool(),
            Some(true),
            "{type_name} named deve possuir estado privado"
        );
        return;
    }

    if let Some(tuple) = kind.get("tuple") {
        let fields = tuple
            .as_array()
            .unwrap_or_else(|| panic!("{type_name} tuple deve ser array"));
        assert!(!fields.is_empty(), "{type_name} tuple não possui estado privado");
        assert!(
            fields.iter().all(Value::is_null),
            "{type_name} tuple expõe field público: {fields:?}"
        );
        return;
    }

    panic!("{type_name} deve ter estado privado named ou tuple: {kind:?}");
}

#[test]
fn rustdoc_json_pinado_fecha_superficie_publica_da_capacidade() {
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
    let expected_type_path = ["typst_infra", "watch", "ArmedWatch"];
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
    assert_eq!(matches.len(), 1, "ArmedWatch deve ter um único ID público");

    let (armed_id_key, _) = matches[0];
    let armed_id = armed_id_key
        .parse::<u64>()
        .expect("ID de ArmedWatch deve ser numérico");
    let armed_item = index
        .get(armed_id_key)
        .expect("ID de ArmedWatch deve existir no index");
    assert_eq!(armed_item["visibility"].as_str(), Some("public"));
    let struct_data = armed_item["inner"]["struct"]
        .as_object()
        .expect("ArmedWatch deve ser struct")
        .clone();
    assert_private_struct_storage(&Value::Object(struct_data.clone()), "ArmedWatch");

    let mut public_inherent_methods = Vec::new();
    for impl_id in struct_data["impls"].as_array().expect("impls deve ser array") {
        let impl_item = index
            .get(&id_key(impl_id, "ArmedWatch.impls"))
            .expect("impl ID deve existir no index");
        let implementation = impl_item["inner"]["impl"]
            .as_object()
            .expect("impl ID deve referenciar impl");
        let items =
            implementation["items"].as_array().expect("impl.items deve ser array");
        if implementation["trait"].is_null() {
            for item_id in items {
                let item = index
                    .get(&id_key(item_id, "ArmedWatch inherent item"))
                    .expect("inherent item deve existir no index");
                if item["visibility"].as_str() == Some("public") {
                    public_inherent_methods.push(
                        item["name"]
                            .as_str()
                            .expect("inherent item público deve ter nome")
                            .to_owned(),
                    );
                }
            }
        }
    }
    public_inherent_methods.sort_unstable();
    assert_eq!(
        public_inherent_methods,
        vec!["abandon", "publish"],
        "superfície inherent pública de ArmedWatch divergiu"
    );

    let mut armed_functions = Vec::new();
    let mut public_watch_functions = Vec::new();
    for (item_id, metadata) in paths {
        if metadata["crate_id"].as_u64() != Some(0)
            || metadata["kind"].as_str() != Some("function")
        {
            continue;
        }
        let path = metadata["path"]
            .as_array()
            .expect("function path deve ser array")
            .iter()
            .map(|segment| segment.as_str().expect("path segment deve ser string"))
            .collect::<Vec<_>>();
        if path.len() != 3 || path[..2] != ["typst_infra", "watch"] {
            continue;
        }
        let item = index.get(item_id).expect("function ID deve existir no index");
        if item["visibility"].as_str() != Some("public") {
            continue;
        }
        public_watch_functions.push(path[2].to_owned());
        let signature = item["inner"]["function"]["sig"]
            .as_object()
            .unwrap_or_else(|| panic!("function path sem signature: {metadata}"));
        if signature_mentions_id(&Value::Object(signature.clone()), armed_id) {
            armed_functions.push(path);
        }
    }
    armed_functions.sort_unstable();
    assert_eq!(
        armed_functions,
        vec![vec!["typst_infra", "watch", "arm"]],
        "superfície livre de ArmedWatch divergiu"
    );
    assert!(
        !public_watch_functions.iter().any(|name| name == "commit_output"),
        "commit_output cru não pode permanecer público para L4"
    );
    assert!(
        !public_watch_functions.iter().any(|name| name == "discard_output"),
        "discard_output cru não pode permanecer público para L4"
    );
}
