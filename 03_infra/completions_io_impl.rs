//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Completions
//! Responsabilidade: Implementar geração de shell completions via clap_complete.

#[path = "../00_nucleo/contracts/completions_io.rs"]
pub mod completions_io;

use completions_io::ICompletionGenerator;

/// Gerador de completions via `clap_complete` para a CLI do Typst.
pub struct ClapCompletionGenerator;

impl ICompletionGenerator for ClapCompletionGenerator {
    fn generate(&self, shell: clap_complete::Shell, buf: &mut dyn std::io::Write) {
        use clap::CommandFactory;
        let mut cmd = super::args_cli::CliArguments::command();
        let bin_name = cmd.get_name().to_string();
        clap_complete::generate(shell, &mut cmd, bin_name, buf);
    }
}
