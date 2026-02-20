//! # Script de Construção (Cargo Build Script)
//! Responsabilidade: Preparar a variável `TARGET` pro ambiente Rust e exportar Man/Autocompletions
//! Posição Arquitetural: Root do App Crate (04_wiring) - O Cargo exige isso na raiz do manifesto.

use std::env;
use std::fs::{create_dir_all, File};
use std::path::Path;

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

// O Cargo inclui este arquivo autônomo. Para acessar nossa infraestrutura CLI, 
// mapeamos diretamente o módulo já isolado em L1/L3 do wiring.
#[path = "../02_shell/args_cli.rs"]
#[allow(dead_code)]
mod args_cli;

fn main() {
    // https://stackoverflow.com/a/51311222/11494565
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    println!("cargo:rerun-if-env-changed=GEN_ARTIFACTS");

    if let Some(dir) = env::var_os("GEN_ARTIFACTS") {
        let out = &Path::new(&dir);
        create_dir_all(out).unwrap();
        
        // Usa nosso DTO limpo de comandos do Terminal
        let cmd = &mut args_cli::CliArguments::command();

        Man::new(cmd.clone())
            .render(&mut File::create(out.join("typst.1")).unwrap())
            .unwrap();

        for subcmd in cmd.get_subcommands() {
            let name = format!("typst-{}", subcmd.get_name());
            Man::new(subcmd.clone().name(&name))
                .render(&mut File::create(out.join(format!("{name}.1"))).unwrap())
                .unwrap();
        }

        for shell in Shell::value_variants() {
            generate_to(*shell, cmd, "typst", out).unwrap();
        }
    }
}
