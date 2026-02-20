//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Deps
//! Responsabilidade: Implementar contratos de escrita de deps e obtenção de working directory.

use std::io::{self};
use std::path::PathBuf;

#[path = "../00_nucleo/contracts/deps_io.rs"]
pub mod deps_io;

use deps_io::{IDepsWriter, IWorkingDirectory};

/// Escritor de deps para um `Box<dyn Write>`.
pub struct FileDepsWriter {
    writer: Box<dyn std::io::Write>,
}

impl FileDepsWriter {
    pub fn new(writer: Box<dyn std::io::Write>) -> Self {
        Self { writer }
    }
}

impl IDepsWriter for FileDepsWriter {
    fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        self.writer.write_all(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Resolvedor de diretório de trabalho via `std::env`.
pub struct OsWorkingDirectory;

impl IWorkingDirectory for OsWorkingDirectory {
    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }
}
