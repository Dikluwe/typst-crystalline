//! Vanilla CLI smoke sentinel — P206B.
//!
//! Sentinel de runtime: usa o build vanilla local em
//! `lab/typst-original/target/release/typst` e confere a versão
//! do pin ratificado `upstream/main 9f2b6e87`.
//!
//! **Skip graceful**: se o build local não puder ser iniciado
//! (por exemplo, CI sem build do lab), o teste avisa via
//! `eprintln!` e completa com sucesso. Quando presente, versão
//! incompatível ou falha de `query --help` reprova o teste.
//!
//! Não usa `PATH` nem `/usr/local/bin/typst`: esses binários
//! não substituem o build local do snapshot pinado.

use std::path::PathBuf;
use std::process::Command;

/// Identidade pública esperada do binário construído com o pin explícito.
/// A prova completa continua a exigir fonte, árvore, receita e SHA-256.
const VANILLA_EXPECTED_VERSION: &str = "typst 0.15.1 (9f2b6e87)";

fn vanilla_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../typst-original/target/release/typst")
}

#[test]
fn p206b_vanilla_cli_disponivel_e_versao_compativel() {
    let output = match Command::new(vanilla_binary()).arg("--version").output() {
        Ok(o) => o,
        Err(e) => {
            // Skip graceful: build vanilla local ausente ou indisponível.
            eprintln!(
                "[p206b smoke] build vanilla local indisponível em \
                 lab/typst-original/target/release/typst ({}); skip graceful. \
                 Para ativar a comparação, compilar typst-cli do lab no pin 9f2b6e87.",
                e
            );
            return;
        }
    };

    assert!(
        output.status.success(),
        "build vanilla local presente mas `typst --version` falhou (exit {:?})",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_trim = stdout.trim();

    assert!(
        stdout_trim == VANILLA_EXPECTED_VERSION,
        "vanilla CLI incompatível: esperado exatamente \
         `{}`, output `{}`. Pinning per vanilla ratificado \
         `upstream/main 9f2b6e87`.",
        VANILLA_EXPECTED_VERSION,
        stdout_trim
    );

    eprintln!("[p206b smoke] ✓ build vanilla local detectado: {}", stdout_trim);
}

#[test]
fn p206b_vanilla_cli_query_subcomando_existe() {
    // Confirma que subcomando `query` está disponível —
    // pré-requisito para P206C comparação estrutural.
    let output = match Command::new(vanilla_binary()).arg("query").arg("--help").output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!(
                "[p206b smoke] build vanilla local indisponível em \
                 lab/typst-original/target/release/typst ({}); skip graceful.",
                e
            );
            return;
        }
    };

    assert!(
        output.status.success(),
        "vanilla `typst query --help` falhou (exit {:?})",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("metadata") || stdout.contains("SELECTOR"),
        "vanilla `typst query --help` output não contém keywords \
         esperadas (metadata/SELECTOR); output: `{}`",
        stdout
    );
}
