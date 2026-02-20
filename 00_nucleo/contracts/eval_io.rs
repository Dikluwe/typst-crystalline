//! Contratos de Interação Externa detectados no `eval.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Eval

use std::io::Result;

/// Interface para escrita de resultados da avaliação e controle de falha global.
pub trait IEvalOutputWriter {
    /// Escreve o texto formatado no destino final (ex: stdout).
    fn write_result(&mut self, text: &str) -> Result<()>;

    /// Sinaliza que o processo falhou (exibe código de erro na saída do programa).
    fn set_failed(&self);
}
