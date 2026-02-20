//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Compile
//! Responsabilidade: Implementar contratos de integração física e chamadas externas (OS Processes, Relógio do Sistema).

use std::io::Result;
use chrono::{DateTime, Local};

#[path = "../00_nucleo/contracts/compile_io.rs"]
pub mod compile_io;

use compile_io::{IProcessOpener, IClockProvider, IDiagnosticPublisher};

pub struct SystemProcessOpener;

impl IProcessOpener for SystemProcessOpener {
    fn open_detached(&self, target: &str) -> Result<()> {
        open::that_detached(target)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
    
    fn open_with_viewer(&self, target: &str, viewer: &str) -> Result<()> {
        open::with_detached(target, viewer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}

pub struct SystemClockProvider;

impl IClockProvider for SystemClockProvider {
    fn now_local(&self) -> DateTime<Local> {
        Local::now()
    }
}

pub struct SystemDiagnosticPublisher;

impl IDiagnosticPublisher for SystemDiagnosticPublisher {
    fn print_success(&self, duration_ms: u64) {
        println!("Compiled successfully in {} ms", duration_ms);
    }

    fn print_error(&self, error_message: &str) {
        eprintln!("Error: {}", error_message);
    }
}
