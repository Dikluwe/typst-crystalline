//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: World
//! Responsabilidade: Implementar contratos de acesso ao filesystem, stdin e configuração de runtime.

use std::io::{self, Read, Result};
use std::path::{Path, PathBuf};

#[path = "../00_nucleo/contracts/world_io.rs"]
pub mod world_io;

use world_io::{IFileSystem, IStdinReader, IRuntimeConfig};

/// Implementação real de acesso ao sistema de arquivos.
pub struct OsFileSystem;

impl IFileSystem for OsFileSystem {
    fn get_current_dir(&self) -> Result<PathBuf> {
        std::env::current_dir()
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        path.canonicalize()
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path)
    }
}

/// Implementação real de leitura do stdin.
pub struct OsStdinReader;

impl IStdinReader for OsStdinReader {
    fn read_to_end(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        match io::stdin().read_to_end(&mut buf) {
            Ok(_) => Ok(buf),
            Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(buf),
            Err(err) => Err(err),
        }
    }
}

/// Implementação real de configuração do pool de threads Rayon.
pub struct RayonRuntimeConfig;

impl IRuntimeConfig for RayonRuntimeConfig {
    fn set_num_threads(&self, num: usize) {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .use_current_thread()
            .build_global()
            .ok();
    }
}
