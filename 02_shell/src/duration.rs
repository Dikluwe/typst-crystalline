//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/duration.md
//! @prompt-hash 400eb0ed
//! @layer L2
//! @updated 2026-09-13
//!
//! Formatter humano puro de durações e corpos de status, materializados nos
//! Passos 1361 e 1362.

use std::fmt::{self, Display, Formatter};
use std::time::Duration;

/// Formata uma duração para apresentação humana.
pub fn format_duration(duration: Duration) -> impl Display {
    FormattedDuration(duration)
}

/// Corpo público de status emitido por um consumidor durante compilação observada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchStatus {
    Compiling,
    Success(Duration),
    PartialSuccess(Duration),
    Error,
}

impl WatchStatus {
    /// Produz somente o corpo textual; canal, quebra de linha e relógio pertencem
    /// ao consumidor.
    pub fn message(&self) -> String {
        match self {
            Self::Compiling => "compiling ...".to_owned(),
            Self::Success(duration) => {
                format!("compiled successfully in {}", format_duration(*duration))
            }
            Self::PartialSuccess(duration) => {
                format!("compiled with warnings in {}", format_duration(*duration))
            }
            Self::Error => "compiled with errors".to_owned(),
        }
    }
}

/// Envolve um corpo de status no timestamp fornecido pelo consumidor.
pub fn format_watch_line(
    hour: u8,
    minute: u8,
    second: u8,
    status: &WatchStatus,
) -> String {
    format!("[{hour:02}:{minute:02}:{second:02}] {}", status.message())
}

struct FormattedDuration(Duration);

impl Display for FormattedDuration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let duration = self.0;
        let total_seconds = duration.as_secs();
        let days = total_seconds / (24 * 60 * 60);
        let hours = total_seconds / (60 * 60) % 24;
        let minutes = total_seconds / 60 % 60;
        let seconds = total_seconds % 60;
        let mut has_piece = false;

        if days > 0 {
            write_piece(formatter, &mut has_piece, days, "d")?;
        }
        if hours > 0 {
            write_piece(formatter, &mut has_piece, hours, "h")?;
        }
        if days > 0 || hours > 0 {
            if minutes > 0 {
                write_piece(formatter, &mut has_piece, minutes, "min")?;
            }
            return Ok(());
        }

        if minutes > 0 {
            write_piece(formatter, &mut has_piece, minutes, "min")?;
        }

        if duration >= Duration::from_secs(5 * 60) {
            return write_piece(formatter, &mut has_piece, seconds, "s");
        }

        let nanos = duration.subsec_nanos();
        if duration >= Duration::from_secs(1) {
            let value = seconds as f64 + nanos as f64 / 1_000_000_000.0;
            write_piece(formatter, &mut has_piece, round_to_hundredths(value), "s")
        } else if duration >= Duration::from_millis(1) {
            let value = nanos as f64 / 1_000_000.0;
            write_piece(formatter, &mut has_piece, round_to_hundredths(value), "ms")
        } else if duration >= Duration::from_micros(1) {
            let value = nanos as f64 / 1_000.0;
            write_piece(formatter, &mut has_piece, round_to_hundredths(value), "µs")
        } else if nanos > 0 {
            write_piece(formatter, &mut has_piece, nanos, "ns")
        } else {
            write_piece(formatter, &mut has_piece, 0, "s")
        }
    }
}

fn write_piece(
    formatter: &mut Formatter<'_>,
    has_piece: &mut bool,
    value: impl Display,
    unit: &str,
) -> fmt::Result {
    if *has_piece {
        formatter.write_str(" ")?;
    }
    *has_piece = true;
    write!(formatter, "{value} {unit}")
}

fn round_to_hundredths(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
