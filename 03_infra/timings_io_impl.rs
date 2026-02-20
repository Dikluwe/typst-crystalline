//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Timings
//! Responsabilidade: Integrar com a manipulação global do `typst_timing` e acesso em disco para relatórios JSON.

use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use typst::diag::HintedStrResult;
use typst::syntax::Span;

use super::args_cli::{CliArguments, Command};

#[path = "../00_nucleo/contracts/timings_io.rs"]
pub mod timings_io;

use timings_io::ITimingExporter;
use super::timings_logic::{format_recording_path, resolve_span};

/// Utilitário acoplado com `typst_timing` e Filesystem capaz de rodar uma macro closure.
pub struct SystemTimer {
    path: Option<PathBuf>,
    index: usize,
}

impl SystemTimer {
    /// Construtor concreto que extrai os argumentos CLI.
    pub fn new(args: &CliArguments) -> Self {
        let record = match &args.command {
            Command::Compile(command) => command.args.timings.clone(),
            Command::Watch(command) => command.args.timings.clone(),
            _ => None,
        };

        if record.is_some() {
            typst_timing::enable();
        }

        let path = record.map(|path: Option<PathBuf>| path.unwrap_or_else(|| PathBuf::from("record-{n}.json")));
        Self { path, index: 0 }
    }

    /// Executa a rotina `f` delegada gravando os tempos, limpando e exportando o resultado.
    pub fn record<W: typst::World, T>(
        &mut self,
        world: &mut W,
        f: impl FnOnce(&mut W) -> T,
    ) -> HintedStrResult<T> {
        let Some(path) = &self.path else {
            return Ok(f(world));
        };

        typst_timing::clear();

        let export_path = format_recording_path(path, self.index)?;

        let output = f(world);
        self.index += 1;

        let file = File::create(&export_path).map_err(|e| format!("failed to create file: {e}"))?;
        let writer = BufWriter::with_capacity(1 << 20, file);

        typst_timing::export_json(writer, |span| {
            // Conversão de `span` bruto em File/Line Number invocando L1 pura.
            resolve_span(world, Span::from_raw(span))
                .unwrap_or_else(|| ("unknown".to_string(), 0))
        }).map_err(|e| format!("failed to export timings: {e}"))?;

        Ok(output)
    }
}
