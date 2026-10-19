//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/fonts.md
//! @prompt-hash 68e31f36
//! @layer L4
//! @updated 2026-09-26

use std::io::Write;
use std::process::ExitCode;
use typst_shell::cli::FontsIntent;

pub(super) fn run_fonts(intent: FontsIntent) -> ExitCode {
    let entries =
        typst_infra::fonts::inventory_fonts(&intent.font_paths, intent.include_system)
            .into_iter()
            .map(|entry| typst_shell::fonts::FontDisplayEntry {
                family: entry.family,
                path: entry.path,
                index: entry.index,
                style: entry.style,
                weight: entry.weight,
                stretch: entry.stretch,
                embedded: entry.embedded,
            })
            .collect::<Vec<_>>();
    let output = typst_shell::fonts::format_fonts(&entries, intent.variants);
    if let Err(error) = std::io::stdout().lock().write_all(output.as_bytes()) {
        eprintln!("error: failed to write font list: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}
