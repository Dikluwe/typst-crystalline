//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Info
//! Responsabilidade: Implementar contratos de leitura de env vars, escrita colorida e resolução de paths do sistema.

use std::io::Result;
use std::path::PathBuf;

#[path = "../00_nucleo/contracts/info_io.rs"]
pub mod info_io;

use info_io::{IEnvReader, IColorWriter, ISystemPathResolver};

/// Leitor de variáveis de ambiente via `std::env::var()`.
pub struct SystemEnvReader;

impl IEnvReader for SystemEnvReader {
    fn read_var(&self, key: &str) -> Result<Option<String>> {
        match std::env::var(key) {
            Ok(val) => Ok(Some(val)),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(std::env::VarError::NotUnicode(_)) => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("a variável de ambiente `{key}` não é UTF-8 válido"),
                ))
            }
        }
    }
}

/// Escritor colorido via `codespan_reporting::term::termcolor`.
pub struct TermColorWriter;

impl IColorWriter for TermColorWriter {
    fn write_key(&mut self, key: &str, pad: Option<usize>) -> Result<()> {
        if let Some(pad) = pad {
            print!("\x1b[36m{key: <pad$}\x1b[0m");
        } else {
            print!("\x1b[36m{key}\x1b[0m");
        }
        Ok(())
    }

    fn write_value(&mut self, val: &str, pad: Option<usize>) -> Result<()> {
        if let Some(pad) = pad {
            print!("\x1b[32m{val: <pad$}\x1b[0m");
        } else {
            print!("\x1b[32m{val}\x1b[0m");
        }
        Ok(())
    }

    fn write_special(&mut self, val: &str, pad: Option<usize>) -> Result<()> {
        if let Some(pad) = pad {
            print!("\x1b[34m{val: <pad$}\x1b[0m");
        } else {
            print!("\x1b[34m{val}\x1b[0m");
        }
        Ok(())
    }

    fn write_plain(&mut self, text: &str) -> Result<()> {
        print!("{text}");
        Ok(())
    }

    fn write_newline(&mut self) -> Result<()> {
        println!();
        Ok(())
    }
}

/// Resolvedor de paths do sistema via `typst_kit::packages::FsPackages`.
pub struct SystemPathResolver;

impl ISystemPathResolver for SystemPathResolver {
    fn system_data_path(&self) -> Option<PathBuf> {
        dirs::data_dir().map(|d| d.join("typst").join("packages"))
    }

    fn system_cache_path(&self) -> Option<PathBuf> {
        dirs::cache_dir().map(|d| d.join("typst").join("packages"))
    }
}
