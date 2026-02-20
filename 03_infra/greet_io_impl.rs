//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Greet
//! Responsabilidade: Implementar leitura/escrita do estado de saudação e saída do processo.

use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[path = "../00_nucleo/contracts/greet_io.rs"]
pub mod greet_io;

use greet_io::{IGreetStateRepository, IGreetTerminal};

/// Repositório baseado em File System para estado de boavindas.
pub struct FsGreetStateRepository;

impl IGreetStateRepository for FsGreetStateRepository {
    fn get_data_dir(&self) -> Option<PathBuf> {
        dirs::data_dir()
    }

    fn read_version(&self, path: &Path) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn write_version(&self, path: &Path, version: &str) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, version)
    }
}

/// Manipulador de terminal e saída que "abusa" do Clap para formatar a saída
/// antes de chamar `std::process::exit(0)`.
pub struct ClapGreetTerminal;

impl IGreetTerminal for ClapGreetTerminal {
    fn print_and_exit(&self, message: &'static str) -> ! {
        // Abuse clap for line wrapping ...
        let err = clap::Command::new("typst")
            .max_term_width(80)
            .help_template("{about}")
            .about(message)
            .try_get_matches_from(["typst", "--help"])
            .unwrap_err();
        let _ = err.print();

        // Windows users might have double-clicked the .exe file and have no chance
        // to read it before the terminal closes.
        if cfg!(windows) {
            pause();
        }

        std::process::exit(err.exit_code());
    }
}

/// Waits for the user.
#[allow(clippy::unused_io_amount)]
fn pause() {
    eprintln!();
    eprintln!("Press enter to continue...");
    io::stdin().lock().read(&mut [0]).unwrap();
}
