//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world/packages.md
//! @prompt-hash 9e6d7bff
//! @layer L3
//! @updated 2026-09-26

use super::SystemWorld;
use std::path::{Path, PathBuf};
use typst_core::contracts::package_downloader::PackageDownloader;
use typst_core::contracts::world::World;
use typst_core::entities::package_spec::PackageSpec;
use typst_core::entities::path::VirtualRoot;
use typst_core::entities::source::Source;

/// Bases de procura de pacotes, em ordem de prioridade (P678, P681): data dir
/// primeiro, depois cache dir. Lê variáveis de ambiente em L3 (I/O permitido;
/// L1 nunca lê env). Cada base é o directório-pai de `{namespace}/{name}/{version}`.
pub(super) fn package_candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    // Data dir — $XDG_DATA_HOME/typst/packages ou ~/.local/share/typst/packages.
    if let Some(d) = std::env::var_os("XDG_DATA_HOME") {
        dirs.push(PathBuf::from(d).join("typst/packages"));
    } else if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(&home).join(".local/share/typst/packages"));
    }
    // Cache dir — $XDG_CACHE_HOME/typst/packages ou ~/.cache/typst/packages.
    if let Some(c) = std::env::var_os("XDG_CACHE_HOME") {
        dirs.push(PathBuf::from(c).join("typst/packages"));
    } else if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(&home).join(".cache/typst/packages"));
    }
    dirs
}

/// Cache dir padrão para pacotes (última entrada de `package_candidate_dirs`).
///
/// Usada pelo downloader HTTP de P763a. Se não for possível determinar uma
/// cache dir, o downloader não é instalado e `resolve_package` mantém o
/// comportamento offline anterior.
pub(super) fn default_package_cache_dir() -> Option<PathBuf> {
    package_candidate_dirs().into_iter().last()
}
impl SystemWorld {
    /// Configura a CA customizada usada por todos os downloads deste world.
    /// O ficheiro permanece lazy: só é lido quando houver uma request HTTPS.
    pub fn with_custom_ca(mut self, cert_path: Option<PathBuf>) -> Self {
        self.package_downloader = default_package_cache_dir().map(|cache_dir| {
            Box::new(
                crate::package_downloader::HttpPackageDownloader::new(cache_dir)
                    .with_custom_ca(cert_path),
            ) as Box<dyn PackageDownloader>
        });
        self
    }

    /// **P686** — Se `file_path` vive dentro de um pacote
    /// (`{base}/{namespace}/{name}/{version}/...`), devolve a raiz do pacote
    /// (`{base}/{namespace}/{name}/{version}`). Caso contrário, `None`.
    /// A detecção é por prefixo de path (sem I/O), coerente com `resolve_package`.
    pub(super) fn package_root_of(&self, file_path: &Path) -> Option<PathBuf> {
        for base in package_candidate_dirs() {
            if let Ok(rest) = file_path.strip_prefix(&base) {
                let mut comps = rest.components();
                let (Some(ns), Some(name), Some(ver)) =
                    (comps.next(), comps.next(), comps.next())
                else {
                    continue;
                };
                return Some(
                    base.join(ns.as_os_str())
                        .join(name.as_os_str())
                        .join(ver.as_os_str()),
                );
            }
        }
        None
    }

    pub(super) fn package_identity_of(
        &self,
        file_path: &Path,
    ) -> Option<(PackageSpec, PathBuf)> {
        for base in package_candidate_dirs() {
            let Ok(rest) = file_path.strip_prefix(&base) else { continue };
            let mut comps = rest.components();
            let namespace = comps.next()?.as_os_str().to_str()?;
            let name = comps.next()?.as_os_str().to_str()?;
            let version = comps.next()?.as_os_str().to_str()?;
            let spec = format!("@{namespace}/{name}:{version}").parse().ok()?;
            return Some((spec, base.join(namespace).join(name).join(version)));
        }
        None
    }

    pub(super) fn physical_root(&self, root: &VirtualRoot) -> Result<PathBuf, String> {
        match root {
            VirtualRoot::Project => Ok(self.root.clone()),
            VirtualRoot::Package(spec) => package_candidate_dirs()
                .into_iter()
                .map(|base| {
                    base.join(&spec.namespace)
                        .join(&spec.name)
                        .join(spec.version.to_string())
                })
                .find(|candidate| candidate.is_dir())
                .ok_or_else(|| format!("package root not found for {spec}")),
        }
    }

    /// **P681** — Lê o manifesto `typst.toml` de `dir`, extrai
    /// `[package].entrypoint`, regista o ficheiro do entrypoint e devolve o
    /// seu `Source`. Os imports internos do pacote resolvem relativamente ao
    /// `current_file` (entrypoint), cujo `directory_of` é o directório do
    /// entrypoint dentro da cache — logo `#import "..."` internos funcionam
    /// sem tratamento especial.
    fn load_package_entrypoint(
        &self,
        dir: &Path,
        spec: &PackageSpec,
    ) -> Result<Source, String> {
        let manifest_path = dir.join("typst.toml");
        let text = std::fs::read_to_string(&manifest_path).map_err(|e| {
            format!("pacote '{}': falha a ler '{}': {}", spec, manifest_path.display(), e)
        })?;
        let manifest: toml::Value = toml::from_str(&text)
            .map_err(|e| format!("pacote '{}': typst.toml inválido: {}", spec, e))?;
        let entrypoint = manifest
            .get("package")
            .and_then(|p| p.get("entrypoint"))
            .and_then(|e| e.as_str())
            .ok_or_else(|| {
                format!("pacote '{}': typst.toml sem [package].entrypoint", spec)
            })?;
        let entry_path = dir.join(entrypoint);
        let id = self.register_file(entry_path.clone());
        self.source(id).map_err(|_| {
            format!(
                "pacote '{}': entrypoint '{}' não encontrado",
                spec,
                entry_path.display()
            )
        })
    }

    pub(super) fn world_resolve_package(
        &self,
        spec: &PackageSpec,
    ) -> Result<typst_core::entities::source::Source, String> {
        let version = spec.version.to_string();
        for base in package_candidate_dirs() {
            let cand = base
                .join(spec.namespace.as_str())
                .join(spec.name.as_str())
                .join(&version);
            if cand.is_dir() {
                return self.load_package_entrypoint(&cand, spec);
            }
        }

        // P763a — tenta descarregar pacotes `@preview` do registo oficial.
        if spec.namespace == "preview" {
            if let Some(downloader) = &self.package_downloader {
                match downloader.download(spec) {
                    Ok(dir) => return self.load_package_entrypoint(&dir, spec),
                    Err(err) => return Err(err.to_string()),
                }
            }
        }

        // P827 — mensagem em paridade com o vanilla (`PackageError::NotFound`,
        // `typst-library/src/diag.rs`): inglês, sem sufixo PT desactualizado.
        Err(format!("package not found (searched for {spec})"))
    }
}
