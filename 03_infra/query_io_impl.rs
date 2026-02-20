//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Query
//! Responsabilidade: Implementar escrita em stdout para resultados de pesquisa e set_failed.

use std::io::{self, Write};

#[path = "../00_nucleo/contracts/query_io.rs"]
pub mod query_io;

use query_io::IQueryOutputWriter;

/// Escritor padrão no terminal e controle global de falhas via sys process.
pub struct StandardQueryOutputWriter;

impl IQueryOutputWriter for StandardQueryOutputWriter {
    fn write_result(&mut self, text: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", text)
    }

    fn set_failed(&self) {
        // Semelhante ao eval, simulamos o comportamento de crate::set_failed
        // encerrando o escopo estritamente com processo exit ou delegate main app error.
        eprintln!("query failed.");
        std::process::exit(1);
    }
}
