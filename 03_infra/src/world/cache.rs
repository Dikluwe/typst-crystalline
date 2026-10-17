//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world/cache.md
//! @prompt-hash 9b756126
//! @layer L3
//! @updated 2026-10-08

use super::SystemWorld;
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use typst_core::entities::file_id::FileId;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{Bytes, FileError, FileResult};

/// Slot de source com carregamento lazy e thread-safe.
///
/// `OnceLock` garante que o ficheiro é lido no máximo uma vez,
/// mesmo com acessos concorrentes. TOCTOU-safe: a leitura acontece
/// dentro do `get_or_init` sem race condition.
pub(super) struct SourceSlot {
    id: FileId,
    pub(super) path: PathBuf,
    source: OnceLock<FileResult<Source>>,
    session: Option<Arc<super::SourceSession>>,
}

impl SourceSlot {
    pub(super) fn new(id: FileId, path: PathBuf) -> Self {
        Self { id, path, source: OnceLock::new(), session: None }
    }

    pub(super) fn with_session(
        id: FileId,
        path: PathBuf,
        session: Option<Arc<super::SourceSession>>,
    ) -> Self {
        let mut slot = Self::new(id, path);
        slot.session = session;
        slot
    }

    pub(super) fn synthetic(id: FileId, path: PathBuf, text: String) -> Self {
        Self::from_source(path, Source::new(id, text))
    }

    fn from_source(path: PathBuf, source: Source) -> Self {
        Self {
            id: source.id(),
            path,
            source: OnceLock::from(Ok(source)),
            session: None,
        }
    }

    /// Carrega o source do disco (apenas na primeira chamada).
    pub(super) fn get(&self) -> FileResult<Source> {
        self.source
            .get_or_init(|| {
                let text = std::fs::read_to_string(&self.path).map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        FileError::NotFound
                    } else {
                        FileError::Other(e.to_string())
                    }
                })?;
                if let Some(session) = &self.session {
                    if let Ok(path) = self.path.canonicalize() {
                        return Ok(session.get_or_parse(path, self.id, text));
                    }
                }
                Ok(Source::new(self.id, text))
            })
            .clone()
    }
}
impl SystemWorld {
    /// Regista um path e retorna o `FileId` correspondente
    /// (cria novo slot se o path ainda não estava registado).
    pub fn register_file(&self, path: PathBuf) -> FileId {
        let canon = path.canonicalize().unwrap_or(path.clone());

        let existing = self.path_to_id.lock().unwrap().get(&canon).copied();
        if let Some(id) = existing {
            return id;
        }

        let mut next = self.next_id.lock().unwrap();
        let mut identities = self.path_to_id.lock().unwrap();
        if let Some(&id) = identities.get(&canon) {
            return id;
        }
        let Some(raw) = u16::try_from(*next).ok().and_then(NonZeroU16::new) else {
            drop(identities);
            drop(next);
            panic!("FileId counter exhausted");
        };
        *next += 1;
        let id = FileId::from_raw(raw);

        identities.insert(canon.clone(), id);
        self.slots.lock().unwrap().insert(
            id,
            Arc::new(SourceSlot::with_session(id, canon, self.source_session.clone())),
        );
        id
    }

    /// Registra uma expressão code sem criar uma dependência física.
    pub fn source_for_expression(&self, text: String) -> FileResult<Source> {
        let mut next = self.next_id.lock().unwrap();
        let raw = u16::try_from(*next)
            .ok()
            .and_then(NonZeroU16::new)
            .ok_or_else(|| FileError::Other("FileId counter exhausted".into()))?;
        let id = FileId::from_raw(raw);
        let bytes = Arc::new(text.as_bytes().to_vec());
        let source =
            Source::new_with_parser(id, text, typst_core::compiler::parse::parse_code);
        // Byte reads acquire read_cache before slots. Keep the same order,
        // publishing both entries before any reader can observe the new id.
        let mut bytes_cache = self.read_cache.lock().unwrap();
        let mut slots = self.slots.lock().unwrap();
        bytes_cache.insert(id, bytes);
        slots.insert(
            id,
            Arc::new(SourceSlot::from_source(
                self.root.join("<input-expression>"),
                source.clone(),
            )),
        );
        *next += 1;
        Ok(source)
    }

    /// Returns the filesystem dependencies registered while compiling.
    ///
    /// The set contains the main source and every source/asset resolved through
    /// this world. Sorting makes the inventory deterministic for watch mode.
    pub fn dependencies(&self) -> Vec<PathBuf> {
        let mut paths =
            self.path_to_id.lock().unwrap().keys().cloned().collect::<Vec<_>>();
        paths.sort();
        paths.dedup();
        paths
    }

    /// **P876** — Lê bytes do disco e embrulha o erro no formato usado por
    /// `file()` (vanilla `FileError`).
    fn load_file_bytes(&self, path: &Path) -> FileResult<Arc<Vec<u8>>> {
        let data = std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FileError::NotFound
            } else {
                FileError::Other(e.to_string())
            }
        })?;
        Ok(Arc::new(data))
    }

    /// **P876** — Devolve os bytes de `id` a partir do cache `read_cache`,
    /// lendo do disco apenas na primeira vez.
    pub(super) fn read_bytes_cached(&self, id: FileId) -> FileResult<Arc<Vec<u8>>> {
        let mut cache = self.read_cache.lock().unwrap();
        if let Some(arc) = cache.get(&id) {
            return Ok(Arc::clone(arc));
        }
        let path = self
            .slots
            .lock()
            .unwrap()
            .get(&id)
            .map(|s| s.path.clone())
            .ok_or(FileError::NotFound)?;
        let arc = self.load_file_bytes(&path)?;
        cache.insert(id, Arc::clone(&arc));
        Ok(arc)
    }

    pub(super) fn world_source(&self, id: FileId) -> FileResult<Source> {
        let slot = self.slots.lock().unwrap().get(&id).cloned();
        slot.ok_or(FileError::NotFound)?.get()
    }

    pub(super) fn world_file(&self, id: FileId) -> FileResult<Bytes> {
        // P876 — usa o cache de bytes para evitar releituras do disco.
        let arc = self.read_bytes_cached(id)?;
        Ok(Bytes::new(arc.as_ref().clone()))
    }
}
