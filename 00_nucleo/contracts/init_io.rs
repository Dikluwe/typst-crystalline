//! Contratos de Interação Externa detectados no `init.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Init (Scaffolding)

use std::path::{Path, PathBuf};
use typst::diag::StrResult;
use typst::syntax::package::{PackageManifest, PackageSpec, TemplateInfo};

/// Interface para interações com sistema de arquivos específicas de inicialização.
pub trait IInitFileSystem {
    /// Lê e faz o parse do arquivo de manifesto de um pacote (`typst.toml`).
    fn parse_manifest(&self, package_path: &Path) -> StrResult<PackageManifest>;

    /// Copia o conteúdo do diretório de template do pacote para o diretório do projeto.
    fn scaffold_project(
        &self,
        project_dir: &Path,
        package_path: &Path,
        template: &TemplateInfo,
    ) -> StrResult<()>;
}

/// Interface para outputs de CLI específicos do Init.
pub trait IInitOutputPrinter {
    /// Imprime o sumário contendo os comandos para os próximos passos da inicialização.
    fn print_summary(
        &mut self,
        spec: &PackageSpec,
        project_dir: &Path,
        template: &TemplateInfo,
    ) -> std::io::Result<()>;
}
