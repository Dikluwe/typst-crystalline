//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash d7df8f5e
//! @layer L4
//! @updated 2026-06-24
//!
//! Integration tests para whitelist type-level do `crystalline-lint` (P440).
//!
//! Verifica que tipos não autorizados de crates autorizadas disparam V14,
//! enquanto tipos autorizados continuam a passar.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Path do binário `crystalline-lint` no PATH do sistema.
const LINT_BIN: &str = "crystalline-lint";

/// Cria um projecto temporário vazio para o teste de lint.
fn temp_project(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("typst-passo-440-{}-{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("criar dir projecto");
    path
}

/// Escreve `content` no ficheiro `rel` dentro de `dir`.
fn write_file(dir: &PathBuf, rel: &str, content: &str) {
    let path = dir.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("criar parent");
    }
    fs::write(&path, content).expect("escrever ficheiro");
}

/// Cria a estrutura mínima de directórios que o walker de prompts espera.
fn setup_project_dirs(dir: &PathBuf) {
    fs::create_dir_all(dir.join("00_nucleo/prompts")).expect("criar prompts dir");
}

/// Configuração mínima de `crystalline.toml` para os testes de lint.
const MINIMAL_CONFIG: &str = r#"
[project]
root = "."

[languages]
rust = { grammar = "tree-sitter-rust", enabled = true }

[layers]
L0 = "00_nucleo"
L1 = "01_core"

[module_layers]
entities = "L1"

[l1_ports]
entities = "entities"

[rules]
V0  = { level = "fatal" }
V1  = { level = "warning" }
V2  = { level = "warning" }
V3  = { level = "warning" }
V4  = { level = "warning" }
V5  = { level = "warning" }
V6  = { level = "warning" }
V7  = { level = "warning" }
V8  = { level = "fatal" }
V9  = { level = "warning" }
V10 = { level = "fatal" }
V11 = { level = "warning" }
V12 = { level = "warning" }
V13 = { level = "warning" }
V14 = { level = "error" }

[l1_allowed_external.ecow]
types = ["EcoString", "EcoVec"]
"#;

/// Header mínimo para satisfazer V1 sem depender de prompts existentes.
const MINIMAL_HEADER: &str = "//! Crystalline Lineage\n";

#[test]
fn type_level_violation_ecow_ecomap() {
    let root = temp_project("ecomap");
    setup_project_dirs(&root);
    write_file(&root, "crystalline.toml", MINIMAL_CONFIG);
    write_file(
        &root,
        "01_core/foo.rs",
        &format!("{}use ecow::EcoMap;\n", MINIMAL_HEADER),
    );

    let result = Command::new(LINT_BIN)
        .current_dir(&root)
        .arg(".")
        .output()
        .expect("executar crystalline-lint");

    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    let output = format!("{}{}", stdout, stderr);

    assert!(output.contains("V14"), "esperava violação V14; output:\n{}", output);
    assert!(
        output.contains("ecow::EcoMap"),
        "esperava ecow::EcoMap na mensagem; output:\n{}",
        output
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn type_level_allowed_ecow_ecostring() {
    let root = temp_project("ecostring");
    setup_project_dirs(&root);
    write_file(&root, "crystalline.toml", MINIMAL_CONFIG);
    write_file(
        &root,
        "01_core/foo.rs",
        &format!("{}use ecow::EcoString;\n", MINIMAL_HEADER),
    );

    let result = Command::new(LINT_BIN)
        .current_dir(&root)
        .arg(".")
        .output()
        .expect("executar crystalline-lint");

    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    let output = format!("{}{}", stdout, stderr);

    assert!(!output.contains("V14"), "não esperava violação V14; output:\n{}", output);

    let _ = fs::remove_dir_all(&root);
}
