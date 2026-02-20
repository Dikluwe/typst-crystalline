//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Args

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::env;

use super::args_io::{IEnvProvider, IOutputWriter};

pub struct StandardOutputWriter;

impl IOutputWriter for StandardOutputWriter {
    fn write_to_file(&self, path: &Path, content: &[u8]) -> io::Result<()> {
        fs::write(path, content)
    }

    fn write_to_stdout(&self, content: &[u8]) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        stdout.write_all(content)?;
        stdout.flush()
    }
}

pub struct SystemEnvProvider;

impl IEnvProvider for SystemEnvProvider {
    fn get_var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
}
