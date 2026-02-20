//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Terminal
//! Responsabilidade: Implementar o handle concreto de terminal (write colorido + clear).

use std::io::{self, IsTerminal, Write};

use codespan_reporting::term::termcolor::{self, ColorChoice, ColorSpec, WriteColor};

#[path = "../00_nucleo/contracts/terminal_io.rs"]
pub mod terminal_io;

use terminal_io::ITerminalOutput;

use super::terminal_logic::{ANSI_CLEAR_SCREEN, ANSI_CLEAR_LAST_LINE, resolve_color_choice};

/// Handle concreto de escrita colorida no terminal usando `termcolor::StandardStream`.
pub struct AnsiTerminalOutput {
    stream: termcolor::StandardStream,
}

impl AnsiTerminalOutput {
    /// Cria um novo terminal output para `stderr` com a cor determinada pela CLI + detecção de TTY.
    pub fn new(clap_color: clap::ColorChoice) -> Self {
        let is_tty = io::stderr().is_terminal();
        let color_choice = resolve_color_choice(clap_color, is_tty);
        let stream = termcolor::StandardStream::stderr(color_choice);
        Self { stream }
    }
}

impl Write for AnsiTerminalOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.lock().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.lock().flush()
    }
}

impl WriteColor for AnsiTerminalOutput {
    fn supports_color(&self) -> bool {
        self.stream.supports_color()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.stream.lock().set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.stream.lock().reset()
    }
}

impl ITerminalOutput for AnsiTerminalOutput {
    fn clear_screen(&mut self) -> io::Result<()> {
        if self.stream.supports_color() {
            let mut lock = self.stream.lock();
            write!(lock, "{}", ANSI_CLEAR_SCREEN)?;
            lock.flush()?;
        }
        Ok(())
    }

    fn clear_last_line(&mut self) -> io::Result<()> {
        if self.stream.supports_color() {
            let mut lock = self.stream.lock();
            write!(lock, "{}", ANSI_CLEAR_LAST_LINE)?;
            lock.flush()?;
        }
        Ok(())
    }
}
