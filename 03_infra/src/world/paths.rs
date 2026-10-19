//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world/paths.md
//! @prompt-hash f909ab38
//! @layer L3
//! @updated 2026-09-26

use super::SystemWorld;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use typst_core::contracts::world::World;
use typst_core::entities::bib_entry::BibEntry;
use typst_core::entities::file_id::FileId;
use typst_core::entities::path::{PathError, RootedPath, VirtualPath, VirtualRoot};
use typst_core::entities::source::Source;
use typst_core::entities::world_types::FileError;

impl SystemWorld {
    /// Directório raiz do projecto.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Directório do ficheiro identificado por `id`.
    /// Retorna a raiz se o FileId não estiver registado.
    pub(super) fn directory_of(&self, id: FileId) -> PathBuf {
        self.slots
            .lock()
            .unwrap()
            .get(&id)
            .and_then(|s| s.path.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| self.root.clone())
    }

    /// **P686/P772b** — Path registado para `id`, se existir.
    /// Público para permitir ao formatter de diagnósticos (L4) mostrar o
    /// path correcto em spans cross-file.
    pub fn path_of(&self, id: FileId) -> Option<PathBuf> {
        self.slots.lock().unwrap().get(&id).map(|s| s.path.clone())
    }

    fn realize_rooted(&self, path: &RootedPath) -> Result<PathBuf, String> {
        let root = self.physical_root(path.root())?;
        let relative = path.vpath().get_with_slash().trim_start_matches('/');
        let physical = root.join(relative);
        if !physical.starts_with(&root) {
            return Err("path would escape its sandbox root".into());
        }
        Ok(physical)
    }

    /// **P686** — Resolve `path` de um `#import`/`#include` contra `current_file`.
    ///
    /// - Absoluto (`/...`): base = raiz do pacote de `current_file` se existir,
    ///   senão `self.root` (raiz do projecto); junta `path` sem a barra inicial.
    /// - Relativo: `directory_of(current_file).join(path)` (sem regressão).
    fn resolve_physical_path(&self, current_file: FileId, path: &str) -> PathBuf {
        if let Some(rest) = path.strip_prefix('/') {
            let base = self
                .path_of(current_file)
                .and_then(|p| self.package_root_of(&p))
                .unwrap_or_else(|| self.root.clone());
            base.join(rest)
        } else {
            self.directory_of(current_file).join(path)
        }
    }

    /// **P450** — Carrega um ficheiro `.bib` do disco e parseia-o com o
    /// parser BibTeX minimal do núcleo.
    pub fn load_bibliography(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<Vec<BibEntry>, String> {
        let bytes = self.read_bytes(current_file, path)?;
        let text = std::str::from_utf8(&bytes).map_err(|e| {
            format!("bibliography file is not valid UTF-8 '{}': {}", path, e)
        })?;
        typst_core::compiler::eval::bibtex::parse_bibtex(text)
            .map_err(|e| format!("failed to parse BibTeX '{}': {}", path, e))
    }

    pub(super) fn world_resolve_path(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<RootedPath, String> {
        let current = self
            .path_of(current_file)
            .ok_or_else(|| "cannot access file system from here".to_string())?;
        let (root, physical_root) = match self.package_identity_of(&current) {
            Some((spec, root)) => (VirtualRoot::Package(spec), root),
            None => (VirtualRoot::Project, self.root.clone()),
        };
        let relative = current
            .strip_prefix(&physical_root)
            .map_err(|_| "current file is outside its sandbox root".to_string())?;
        let current_virtual =
            VirtualPath::new(&format!("/{}", relative.to_string_lossy()))
                .map_err(|_| "current file has an invalid virtual path".to_string())?;
        let base = current_virtual
            .parent()
            .unwrap_or_else(|| VirtualPath::new("/").unwrap());
        let vpath = base.join(path).map_err(|error| match error {
            PathError::Escapes => format!(
                "path `\"{path}\"` would escape the {} root",
                if matches!(root, VirtualRoot::Package(_)) {
                    "package"
                } else {
                    "project"
                }
            ),
            PathError::Backslash => "path must not contain a backslash".to_string(),
        })?;
        Ok(RootedPath::new(root, vpath))
    }

    pub(super) fn world_read_path(
        &self,
        path: &RootedPath,
    ) -> Result<Arc<Vec<u8>>, String> {
        let full_path = self.realize_rooted(path)?;
        let id = self.register_file(full_path.clone());
        self.read_bytes_cached(id).map_err(|e| match e {
            FileError::NotFound => {
                format!("file not found (searched at {})", full_path.display())
            }
            FileError::Other(msg) if msg.contains("access denied") => {
                "failed to load file (access denied)".to_string()
            }
            FileError::Other(e) => format!("failed to load file ({e})"),
            _ => format!("failed to load file ({e})"),
        })
    }

    pub(super) fn world_include_path(&self, path: &RootedPath) -> Result<Source, String> {
        let full_path = self.realize_rooted(path)?;
        let id = self.register_file(full_path.clone());
        self.source(id).map_err(|_| {
            format!("include: ficheiro não encontrado: {}", full_path.display())
        })
    }

    pub(super) fn world_read_bytes(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let full_path = self.resolve_physical_path(current_file, path);
        // P876 — regista o ficheiro para obter um FileId canónico e partilhar
        // o mesmo `Arc<Vec<u8>>` entre chamadas repetidas ao mesmo ficheiro.
        // Isso permite que a deduplicação por `Arc::as_ptr` no exportador PDF
        // funcione para múltiplos `#image("...")` do mesmo ficheiro.
        let id = self.register_file(full_path.clone());
        self.read_bytes_cached(id).map_err(|e| {
            // P819 — formato do `Display` de `FileError` do vanilla
            // (`typst-library/src/diag.rs:647-660`): a mensagem é o observável
            // (ADR-0107) e `plugin()`/`read()` propagam-na verbatim.
            match e {
                FileError::NotFound => {
                    format!("file not found (searched at {})", full_path.display())
                }
                FileError::Other(msg) if msg.contains("access denied") => {
                    "failed to load file (access denied)".to_string()
                }
                FileError::Other(e) => format!("failed to load file ({e})"),
                _ => format!("failed to load file ({e})"),
            }
        })
    }

    pub(super) fn world_include_source(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<typst_core::entities::source::Source, String> {
        let abs_path = self.resolve_physical_path(current_file, path);
        let id = self.register_file(abs_path.clone());
        self.source(id).map_err(|_| {
            format!("include: ficheiro não encontrado: {}", abs_path.display())
        })
    }
}
