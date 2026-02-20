//! Contratos de Interação Externa detectados no `query.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Query

use std::io::Result;

/// Interface para gerenciar a saída do processo de query e indicação de erros críticos.
pub trait IQueryOutputWriter {
    /// Escreve o resultado da consulta no terminal ou destino final.
    fn write_result(&mut self, text: &str) -> Result<()>;

    /// Sinaliza globalmente que a consulta falhou.
    fn set_failed(&self);
}
