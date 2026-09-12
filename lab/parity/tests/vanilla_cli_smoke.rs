//! Vanilla CLI smoke sentinel — P206B.
//!
//! Sentinel de runtime: confirma que o binário vanilla
//! `typst` está acessível via PATH e reporta versão
//! idêntico ao vanilla ratificado `upstream/main 586e1bd43`.
//!
//! **Skip graceful**: se vanilla CLI ausente (CI sem
//! install step ou ambiente local sem typst), o test
//! emite warning via `eprintln!` e completa com sucesso.
//! Não falha a suite — paridade é medição, não verificação
//! (consistente com `layout_parity.rs::corpus_completo_p3`).
//!
//! Per ADR-0075 PROPOSTO §"Mecanismo": vanilla CLI é
//! dependência ambiental externa (não compilada na
//! quarentena, não workspace member). Smoke confirma
//! que pre-built binário é detectável.

use std::process::Command;

/// Identidade pública esperada do binário construído com o pin explícito.
/// A prova completa continua a exigir fonte, árvore, receita e SHA-256.
const VANILLA_EXPECTED_VERSION: &str = "typst 0.15.1 (586e1bd4)";

#[test]
fn p206b_vanilla_cli_disponivel_e_versao_compativel() {
    let output = match Command::new("typst").arg("--version").output() {
        Ok(o) => o,
        Err(e) => {
            // Skip graceful: vanilla CLI ausente em PATH.
            // Per ADR-0075 §Plano de validação cond 6:
            // "abort gracefully se ausente". P206C/D dependem
            // mas são opt-in.
            eprintln!(
                "[p206b smoke] vanilla `typst` ausente em PATH ({}); skip graceful. \
                 Para activar comparação vanilla, instalar via cargo install --git \
                 https://github.com/typst/typst typst-cli ou package manager.",
                e
            );
            return;
        }
    };

    assert!(
        output.status.success(),
        "vanilla CLI presente mas `typst --version` falhou (exit {:?})",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_trim = stdout.trim();

    assert!(
        stdout_trim == VANILLA_EXPECTED_VERSION,
        "vanilla CLI incompatível: esperado exatamente \
         `{}`, output `{}`. Pinning per vanilla ratificado \
         `upstream/main 586e1bd43`.",
        VANILLA_EXPECTED_VERSION,
        stdout_trim
    );

    eprintln!("[p206b smoke] ✓ vanilla CLI detectada: {}", stdout_trim);
}

#[test]
fn p206b_vanilla_cli_query_subcomando_existe() {
    // Confirma que subcomando `query` está disponível —
    // pré-requisito para P206C comparação estrutural.
    let output = match Command::new("typst").arg("query").arg("--help").output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!(
                "[p206b smoke] vanilla `typst` ausente em PATH ({}); skip.",
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
