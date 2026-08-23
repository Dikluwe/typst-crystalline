//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli.md
//! @prompt-hash 221bc7f3
//! @layer L2
//! @updated 2026-07-21
//!
//! Build script — captura o commit HEAD do git em tempo de build para
//! `--version` do CLI (P796). Mecânica copiada directamente do vanilla
//! (`lab/typst-original/crates/typst-utils/build.rs`) por decisão explícita
//! do dono do projecto em P796: "copiar direto como Vanilla" por agora —
//! identidade de versão própria do cristalino fica para decisão futura
//! (ver `shell/cli.md` §"Decisão — número de versão do CLI").
//!
//! Zero I/O em runtime: isto corre só em tempo de compilação; o binário
//! final lê o valor via `option_env!("TYPST_COMMIT_SHA")`, uma constante
//! embutida no executável.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=TYPST_COMMIT_SHA");
    // P1033 — reexecutar build.rs quando o HEAD git muda, para que
    // `--version` reflita o commit real sem exigir `cargo clean`.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/refs");

    if option_env!("TYPST_COMMIT_SHA").is_none() {
        if let Some(sha) = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
        {
            println!("cargo:rustc-env=TYPST_COMMIT_SHA={}", sha.trim());
        }
    }
}
