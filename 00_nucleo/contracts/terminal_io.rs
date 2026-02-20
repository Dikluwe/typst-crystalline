//! Contratos de Interação Externa detectados no `terminal.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Terminal

use std::io::{Result, Write};
use codespan_reporting::term::termcolor::{ColorSpec, WriteColor};

/// Interface para um handle de escrita colorida no terminal.
/// Combina `Write` + `WriteColor` + operações de limpeza de tela.
pub trait ITerminalOutput: Write + WriteColor {
    /// Limpa a tela inteira e retorna o cursor ao topo-esquerdo.
    fn clear_screen(&mut self) -> Result<()>;

    /// Limpa a última linha escrita no terminal.
    fn clear_last_line(&mut self) -> Result<()>;
}

/// Interface para obtenção de um handle de terminal.
pub trait ITerminalProvider {
    /// Retorna um handle de escrita colorida (potencialmente singleton).
    fn get_output(&self) -> Box<dyn ITerminalOutput>;
}
