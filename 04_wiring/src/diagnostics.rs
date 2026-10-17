//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/diagnostics.md
//! @prompt-hash b45b6b83
//! @layer L4
//! @updated 2026-10-06

use std::path::Path;
use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_infra::world::SystemWorld;
use typst_shell::diagnostic::{format_diagnostic, DiagnosticSource};

/// Drena diagnósticos depois de materializar todas as fontes referidas.
pub(super) fn drain_to_stderr(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
) {
    drain_to_stderr_with_primary(world, diagnostics, input, colored, None);
}

pub(super) fn drain_to_stderr_with_primary(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
    primary: Option<&Source>,
) {
    let main_id = world.main();
    let cwd = std::env::current_dir().ok();
    for diag in diagnostics {
        let mut ids = Vec::new();
        if let Some(id) = diag.span.id() {
            ids.push(id);
        }
        for point in &diag.trace {
            if let Some(id) = point.span.id() {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        if ids.is_empty() {
            ids.push(main_id);
        }

        let sources = ids
            .into_iter()
            .filter_map(|id| {
                let is_primary = primary.is_some_and(|source| source.id() == id);
                let source = primary
                    .filter(|source| source.id() == id)
                    .cloned()
                    .or_else(|| world.source(id).ok())?;
                let path = if is_primary {
                    input.to_path_buf()
                } else {
                    world.path_of(id).unwrap_or_else(|| input.to_path_buf())
                };
                let display = cwd
                    .as_deref()
                    .and_then(|base| pathdiff::diff_paths(&path, base))
                    .unwrap_or(path)
                    .display()
                    .to_string();
                Some(DiagnosticSource::new(source, display))
            })
            .collect::<Vec<_>>();
        eprint!("{}", format_diagnostic(diag, &sources, colored));
    }
}
