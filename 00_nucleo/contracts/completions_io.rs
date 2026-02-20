//! Contratos de Interação Externa detectados no `completions.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Completions

/// Interface para geração de shell completions.
/// Abstrai a escrita do script de completions para um destino.
pub trait ICompletionGenerator {
    /// Gera o script de completions para o shell especificado e escreve no destino.
    fn generate(&self, shell: clap_complete::Shell, buf: &mut dyn std::io::Write);
}
