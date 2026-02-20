//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L4 (Wiring / Composição Base)
//! Módulo: Global
//! Responsabilidade: Fazer a composição e amarrar interfaces puras da L0, com frameworks L2, validadores de L1, e executáveis de E/S de L3.

#[path = "../00_nucleo/contracts/args_io.rs"]
pub mod args_io;

#[path = "../01_core/args_logic.rs"]
pub mod args_logic;

#[path = "../01_core/compile_logic.rs"]
pub mod compile_logic;

#[path = "../01_core/info_logic.rs"]
pub mod info_logic;

#[path = "../03_infra/args_io_impl.rs"]
pub mod args_io_impl;

#[path = "../03_infra/compile_io_impl.rs"]
pub mod compile_io_impl;

#[path = "../03_infra/info_io_impl.rs"]
pub mod info_io_impl;

#[path = "../02_shell/args_cli.rs"]
pub mod args_cli;

// (Não estamos adicionando um 02_shell para compile/info porque esses módulos ATUAIS funcionam como controllers legados no projeto inteiro)

pub use args_io_impl::{StandardOutputWriter, SystemEnvProvider};
pub use compile_io_impl::{SystemProcessOpener, SystemClockProvider, SystemDiagnosticPublisher};
pub use info_io_impl::{SystemEnvReader, TermColorWriter, SystemPathResolver};
pub use args_cli::CliArguments;

pub struct TypstApp {
    pub io: Box<dyn args_io::IOutputWriter>,
    pub env: Box<dyn args_io::IEnvProvider>,
    pub process: Box<dyn compile_io_impl::compile_io::IProcessOpener>,
    pub clock: Box<dyn compile_io_impl::compile_io::IClockProvider>,
    pub diagnostics: Box<dyn compile_io_impl::compile_io::IDiagnosticPublisher>,
    pub env_reader: Box<dyn info_io_impl::info_io::IEnvReader>,
    pub path_resolver: Box<dyn info_io_impl::info_io::ISystemPathResolver>,
}

impl Default for TypstApp {
    fn default() -> Self {
        Self {
            io: Box::new(StandardOutputWriter),
            env: Box::new(SystemEnvProvider),
            process: Box::new(SystemProcessOpener),
            clock: Box::new(SystemClockProvider),
            diagnostics: Box::new(SystemDiagnosticPublisher),
            env_reader: Box::new(SystemEnvReader),
            path_resolver: Box::new(SystemPathResolver),
        }
    }
}

impl TypstApp {
    pub fn run(&self) {
        // Inicializa CLI (L2) em posse dos Drivers (L3).
    }
}
