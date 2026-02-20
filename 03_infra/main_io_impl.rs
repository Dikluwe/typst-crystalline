//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Main
//! Responsabilidade: Implementar impressão formatada de erros/hints com cores e controle de exit code.

use std::cell::Cell;
use std::io::{self, Write};
use std::process::ExitCode;

use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

#[path = "../00_nucleo/contracts/main_io.rs"]
pub mod main_io;

use main_io::{IAppErrorPrinter, IExitCodeController};

thread_local! {
    /// Exit code do CLI gerenciado por thread_local.
    static EXIT: Cell<ExitCode> = const { Cell::new(ExitCode::SUCCESS) };
}

/// Controle de exit code baseado em thread_local.
pub struct ThreadLocalExitController;

impl IExitCodeController for ThreadLocalExitController {
    fn set_failed(&self) {
        EXIT.with(|cell| cell.set(ExitCode::FAILURE));
    }

    fn get_exit_code(&self) -> ExitCode {
        EXIT.with(|cell| cell.get())
    }
}

/// Impressor de erros e hints usando `termcolor::StandardStream`.
pub struct ColoredErrorPrinter {
    stream: StandardStream,
}

impl ColoredErrorPrinter {
    pub fn new() -> Self {
        Self {
            stream: StandardStream::stderr(ColorChoice::Auto),
        }
    }
}

impl IAppErrorPrinter for ColoredErrorPrinter {
    fn print_error(&mut self, msg: &str) -> io::Result<()> {
        let styles = term::Styles::default();
        self.stream.set_color(&styles.header_error)?;
        write!(self.stream, "error")?;
        self.stream.reset()?;
        writeln!(self.stream, ": {msg}")
    }

    fn print_hint(&mut self, msg: &str) -> io::Result<()> {
        let styles = term::Styles::default();
        self.stream.set_color(&styles.header_help)?;
        write!(self.stream, "hint")?;
        self.stream.reset()?;
        writeln!(self.stream, ": {msg}")
    }
}
