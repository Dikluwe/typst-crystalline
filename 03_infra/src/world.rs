//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world.md
//! @prompt-hash 00aa19fa
//! @layer L3
//! @updated 2026-03-26

use std::collections::HashMap;
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU16, Ordering};

use typst_core::contracts::world::World;
use typst_core::entities::file_id::FileId;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, FontBook, Library,
};

/// Contador atómico para geração de `FileId` únicos nesta sessão.
static NEXT_ID: AtomicU16 = AtomicU16::new(1);

fn next_file_id() -> FileId {
    let raw = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    FileId::from_raw(NonZeroU16::new(raw).unwrap_or(NonZeroU16::new(1).unwrap()))
}

/// Erro de criação do `SystemWorld`.
#[derive(Debug)]
pub enum SystemWorldError {
    /// O ficheiro principal não existe.
    MainNotFound(PathBuf),
    /// Erro de I/O ao ler o ficheiro principal.
    Io(std::io::Error),
}

impl std::fmt::Display for SystemWorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainNotFound(p) => write!(f, "main file not found: {}", p.display()),
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for SystemWorldError {}

/// Implementação concreta de `World` para o filesystem real.
///
/// Vive em L3 porque faz I/O de disco. Usa stubs para `Library`,
/// `FontBook`, `Font` e `Datetime` até esses tipos serem migrados.
pub struct SystemWorld {
    /// Directório raiz do projecto.
    root: PathBuf,
    /// `FileId` do ficheiro principal.
    main: FileId,
    /// Cache de `Source` por `FileId`.
    sources: HashMap<FileId, Source>,
    /// Mapa de path canónico → `FileId`.
    paths: HashMap<PathBuf, FileId>,
    /// Stub de biblioteca padrão.
    library: Library,
    /// Stub do catálogo de fontes.
    font_book: FontBook,
}

impl SystemWorld {
    /// Cria um `SystemWorld` com `root` como directório base e
    /// `main` como ficheiro principal (relativo a `root` ou absoluto).
    pub fn new(root: impl Into<PathBuf>, main: impl AsRef<Path>) -> Result<Self, SystemWorldError> {
        let root = root.into();
        let main_path = if main.as_ref().is_absolute() {
            main.as_ref().to_path_buf()
        } else {
            root.join(main.as_ref())
        };

        let text = std::fs::read_to_string(&main_path)
            .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                SystemWorldError::MainNotFound(main_path.clone())
            } else {
                SystemWorldError::Io(e)
            })?;

        let main_id = next_file_id();
        let source = Source::new(main_id, text);

        let mut sources = HashMap::new();
        let mut paths = HashMap::new();
        sources.insert(main_id, source);
        paths.insert(main_path.canonicalize().unwrap_or(main_path), main_id);

        Ok(Self {
            root,
            main: main_id,
            sources,
            paths,
            library: Library::new(),
            font_book: FontBook::new(),
        })
    }

    /// Resolve um `FileId` para o path no filesystem.
    fn path_for_id(&self, id: FileId) -> Option<PathBuf> {
        self.paths.iter()
            .find(|(_, &fid)| fid == id)
            .map(|(p, _)| p.clone())
    }

    /// Carrega um `Source` do disco e cacheia-o.
    fn load_source(&mut self, id: FileId) -> FileResult<Source> {
        let path = self.path_for_id(id).ok_or(FileError::NotFound)?;
        let text = std::fs::read_to_string(&path)
            .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                FileError::NotFound
            } else {
                FileError::Other(e.to_string())
            })?;
        let src = Source::new(id, text);
        self.sources.insert(id, src.clone());
        Ok(src)
    }
}

impl World for SystemWorld {
    fn library(&self) -> &Library {
        &self.library
    }

    fn book(&self) -> &FontBook {
        &self.font_book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if let Some(src) = self.sources.get(&id) {
            return Ok(src.clone());
        }
        // Ficheiro não está em cache — tentar carregar do disco.
        // Nota: sem mutabilidade interior aqui; ficheiros não-cached
        // após construção retornam NotFound nesta implementação mínima.
        // O Passo 8 introduzirá Mutex/RefCell para lazy-loading.
        Err(FileError::NotFound)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = self.path_for_id(id).ok_or(FileError::NotFound)?;
        let bytes = std::fs::read(&path)
            .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                FileError::NotFound
            } else {
                FileError::Other(e.to_string())
            })?;
        Ok(Bytes::new(bytes))
    }

    fn font(&self, _index: usize) -> Option<Font> {
        None // stub — fontes reais no Passo 8
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None // stub — Datetime real após ADR-0017
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use typst_core::contracts::world::World;
    use typst_core::entities::world_types::{Bytes, Datetime, FileError, Font, FontBook, Library};

    // ── MockWorld para testes sem filesystem ──────────────────────────────

    struct MockWorld {
        main_id: FileId,
        source: Source,
        library: Library,
        book: FontBook,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(42).unwrap());
            Self {
                main_id: id,
                source: Source::new(id, text.to_string()),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.main_id }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main_id { Ok(self.source.clone()) }
            else { Err(FileError::NotFound) }
        }
        fn file(&self, _: FileId)   -> FileResult<Bytes>  { Err(FileError::NotFound) }
        fn font(&self, _: usize)    -> Option<Font>       { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    // ── Testes de MockWorld ───────────────────────────────────────────────

    #[test]
    fn mock_world_source_roundtrip() {
        let world = MockWorld::new("Hello *world*");
        let src = world.source(world.main()).unwrap();
        assert_eq!(src.text(), "Hello *world*");
    }

    #[test]
    fn mock_world_source_not_found_other_id() {
        let world = MockWorld::new("text");
        let other = FileId::from_raw(NonZeroU16::new(99).unwrap());
        assert!(matches!(world.source(other), Err(FileError::NotFound)));
    }

    #[test]
    fn mock_world_font_none() {
        let world = MockWorld::new("text");
        assert!(world.font(0).is_none());
    }

    #[test]
    fn mock_world_today_none() {
        let world = MockWorld::new("text");
        assert!(world.today(None).is_none());
        assert!(world.today(Some(2)).is_none());
    }

    // ── Teste de integração parse→world ──────────────────────────────────

    #[test]
    fn parse_via_mock_world() {
        use typst_core::entities::syntax_kind::SyntaxKind;

        let world = MockWorld::new("= Heading\n\nParagraph.");
        let src = world.source(world.main()).unwrap();

        assert_eq!(src.root().kind(), SyntaxKind::Markup);
        assert!(!src.root().erroneous());

        let has_heading = src.root()
            .children()
            .any(|n| n.kind() == SyntaxKind::Heading);
        assert!(has_heading);
    }

    // ── Testes de SystemWorld com ficheiro temporário ─────────────────────

    #[test]
    fn system_world_main_id_valid() {
        let dir = tempfile_write("hello.typ", "Hello world");
        let world = SystemWorld::new(dir.path(), "hello.typ").unwrap();
        // main() deve retornar um FileId válido (não zero)
        let _ = world.main();
    }

    #[test]
    fn system_world_source_reads_file() {
        let dir = tempfile_write("main.typ", "= Title");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let src = world.source(world.main()).unwrap();
        assert_eq!(src.text(), "= Title");
    }

    #[test]
    fn system_world_source_not_found_unknown_id() {
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let unknown = FileId::from_raw(NonZeroU16::new(200).unwrap());
        assert!(matches!(world.source(unknown), Err(FileError::NotFound)));
    }

    #[test]
    fn system_world_new_missing_file_errors() {
        let dir = tempdir();
        let result = SystemWorld::new(dir.path(), "nonexistent.typ");
        assert!(result.is_err());
    }

    #[test]
    fn system_world_font_stub_returns_none() {
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        assert!(world.font(0).is_none());
    }

    #[test]
    fn system_world_today_stub_returns_none() {
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        assert!(world.today(None).is_none());
    }

    // ── Utilitários de teste ──────────────────────────────────────────────

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path { &self.0 }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        let path = std::env::temp_dir().join(format!(
            "typst-crystalline-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    fn tempfile_write(name: &str, content: &str) -> TempDir {
        let dir = tempdir();
        std::fs::write(dir.path().join(name), content).unwrap();
        dir
    }
}
