//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L3 (Infra / Repositórios e Serviços de IO)
//! Módulo: Init
//! Responsabilidade: Implementar infraestrutura de scaffolding usando `fs_extra` e impressão no terminal.

use std::io::Write;
use std::path::Path;

use codespan_reporting::term::termcolor::{Color, ColorSpec, WriteColor};
use ecow::eco_format;
use fs_extra::dir::CopyOptions;
use typst::diag::{FileError, StrResult, bail};
use typst::syntax::package::{PackageManifest, PackageSpec, TemplateInfo};

#[path = "../00_nucleo/contracts/init_io.rs"]
pub mod init_io;

use init_io::{IInitFileSystem, IInitOutputPrinter};

/// Implementação do Filesystem baseada no OS para inicialização de templates.
pub struct OsInitFileSystem;

impl IInitFileSystem for OsInitFileSystem {
    fn parse_manifest(&self, package_path: &Path) -> StrResult<PackageManifest> {
        let toml_path = package_path.join("typst.toml");
        let string = std::fs::read_to_string(&toml_path).map_err(|err| {
            eco_format!(
                "failed to read package manifest ({})",
                FileError::from_io(err, &toml_path)
            )
        })?;

        toml::from_str(&string)
            .map_err(|err| eco_format!("package manifest is malformed ({})", err.message()))
    }

    fn scaffold_project(
        &self,
        project_dir: &Path,
        package_path: &Path,
        template: &TemplateInfo,
    ) -> StrResult<()> {
        if project_dir.exists() {
            bail!("project directory already exists (at {})", project_dir.display());
        }

        let template_dir = package_path.join(template.path.as_str());
        if !template_dir.exists() {
            bail!("template directory does not exist (at {})", template_dir.display());
        }

        fs_extra::dir::copy(
            &template_dir,
            project_dir,
            &CopyOptions::new().content_only(true),
        )
        .map_err(|err| eco_format!("failed to create project directory ({err})"))?;

        Ok(())
    }
}

/// Impressor do terminal padrão.
pub struct StandardInitPrinter;

impl IInitOutputPrinter for StandardInitPrinter {
    fn print_summary(
        &mut self,
        spec: &PackageSpec,
        project_dir: &Path,
        template: &TemplateInfo,
    ) -> std::io::Result<()> {
        let mut gray = ColorSpec::new();
        gray.set_fg(Some(Color::White));
        gray.set_dimmed(true);

        // Instancia um StandardStream puro ao invés de depender do `crate::terminal` legado.
        let mut out = codespan_reporting::term::termcolor::StandardStream::stdout(
            codespan_reporting::term::termcolor::ColorChoice::Auto
        );
        writeln!(out, "Successfully created new project from {spec} 🎉")?;
        writeln!(out, "To start writing, run:")?;
        out.set_color(&gray)?;
        write!(out, "> ")?;
        out.reset()?;
        writeln!(
            out,
            "cd {}",
            shell_escape::escape(project_dir.display().to_string().into()),
        )?;
        out.set_color(&gray)?;
        write!(out, "> ")?;
        out.reset()?;
        writeln!(
            out,
            "typst watch {}",
            shell_escape::escape(template.entrypoint.to_string().into()),
        )?;
        writeln!(out)?;
        Ok(())
    }
}
