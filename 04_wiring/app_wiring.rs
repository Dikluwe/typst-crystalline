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

#[path = "../01_core/world_logic.rs"]
pub mod world_logic;

#[path = "../03_infra/world_io_impl.rs"]
pub mod world_io_impl;

#[path = "../01_core/update_logic.rs"]
pub mod update_logic;

#[path = "../03_infra/update_io_impl.rs"]
pub mod update_io_impl;

#[path = "../01_core/watch_logic.rs"]
pub mod watch_logic;

#[path = "../03_infra/watch_io_impl.rs"]
pub mod watch_io_impl;

#[path = "../01_core/deps_logic.rs"]
pub mod deps_logic;

#[path = "../03_infra/deps_io_impl.rs"]
pub mod deps_io_impl;

#[path = "../01_core/completions_logic.rs"]
pub mod completions_logic;

#[path = "../02_shell/args_cli.rs"]
pub mod args_cli;

#[path = "../03_infra/completions_io_impl.rs"]
pub mod completions_io_impl;

#[path = "../01_core/download_logic.rs"]
pub mod download_logic;

#[path = "../03_infra/download_io_impl.rs"]
pub mod download_io_impl;

#[path = "../01_core/eval_logic.rs"]
pub mod eval_logic;

#[path = "../03_infra/eval_io_impl.rs"]
pub mod eval_io_impl;

#[path = "../01_core/fonts_logic.rs"]
pub mod fonts_logic;

#[path = "../03_infra/fonts_io_impl.rs"]
pub mod fonts_io_impl;

#[path = "../01_core/greet_logic.rs"]
pub mod greet_logic;

#[path = "../03_infra/greet_io_impl.rs"]
pub mod greet_io_impl;

#[path = "../01_core/packages_logic.rs"]
pub mod packages_logic;

#[path = "../03_infra/packages_io_impl.rs"]
pub mod packages_io_impl;

#[path = "../01_core/init_logic.rs"]
pub mod init_logic;

#[path = "../03_infra/init_io_impl.rs"]
pub mod init_io_impl;

#[path = "../01_core/query_logic.rs"]
pub mod query_logic;

#[path = "../03_infra/query_io_impl.rs"]
pub mod query_io_impl;

#[path = "../01_core/terminal_logic.rs"]
pub mod terminal_logic;

#[path = "../03_infra/terminal_io_impl.rs"]
pub mod terminal_io_impl;

// (Não estamos adicionando um 02_shell para compile/info porque esses módulos ATUAIS funcionam como controllers legados no projeto inteiro)

pub use args_io_impl::{StandardOutputWriter, SystemEnvProvider};
pub use compile_io_impl::{SystemProcessOpener, SystemClockProvider, SystemDiagnosticPublisher};
pub use info_io_impl::{SystemEnvReader, TermColorWriter, SystemPathResolver};
pub use world_io_impl::{OsFileSystem, OsStdinReader, RayonRuntimeConfig};
pub use update_io_impl::{OsBackupPathResolver};
#[cfg(feature = "self-update")]
pub use update_io_impl::{OsSelfReplacer, SystemArchiveExtractor};
pub use watch_io_impl::ChronoTimestampProvider;
pub use deps_io_impl::OsWorkingDirectory;
pub use completions_io_impl::ClapCompletionGenerator;
pub use eval_io_impl::StandardEvalOutputWriter;
pub use fonts_io_impl::{SystemFontDiscoverer, StandardFontPrinter};
pub use greet_io_impl::{FsGreetStateRepository, ClapGreetTerminal};
pub use packages_io_impl::OsPackageRegistryFactory;
pub use init_io_impl::{OsInitFileSystem, StandardInitPrinter};
pub use query_io_impl::StandardQueryOutputWriter;
pub use terminal_io_impl::AnsiTerminalOutput;
pub use args_cli::CliArguments;

pub struct TypstApp {
    pub io: Box<dyn args_io::IOutputWriter>,
    pub env: Box<dyn args_io::IEnvProvider>,
    pub process: Box<dyn compile_io_impl::compile_io::IProcessOpener>,
    pub clock: Box<dyn compile_io_impl::compile_io::IClockProvider>,
    pub diagnostics: Box<dyn compile_io_impl::compile_io::IDiagnosticPublisher>,
    pub env_reader: Box<dyn info_io_impl::info_io::IEnvReader>,
    pub path_resolver: Box<dyn info_io_impl::info_io::ISystemPathResolver>,
    pub filesystem: Box<dyn world_io_impl::world_io::IFileSystem>,
    pub stdin_reader: Box<dyn world_io_impl::world_io::IStdinReader>,
    pub runtime: Box<dyn world_io_impl::world_io::IRuntimeConfig>,
    pub backup_resolver: Box<dyn update_io_impl::update_io::IBackupPathResolver>,
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
            filesystem: Box::new(OsFileSystem),
            stdin_reader: Box::new(OsStdinReader),
            runtime: Box::new(RayonRuntimeConfig),
            backup_resolver: Box::new(OsBackupPathResolver),
        }
    }
}

impl TypstApp {
    pub fn run(&self) {
        // Inicializa CLI (L2) em posse dos Drivers (L3).
    }
}
