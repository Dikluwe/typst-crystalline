//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world.md
//! @prompt-hash 662ca2dc
//! @layer L3
//! @updated 2026-03-26

use std::collections::HashMap;
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use typst_core::contracts::world::World;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

use crate::fonts::FontSlot;

/// Slot de source com carregamento lazy e thread-safe.
///
/// `OnceLock` garante que o ficheiro é lido no máximo uma vez,
/// mesmo com acessos concorrentes. TOCTOU-safe: a leitura acontece
/// dentro do `get_or_init` sem race condition.
struct SourceSlot {
    id:     FileId,
    path:   PathBuf,
    source: OnceLock<FileResult<Source>>,
}

impl SourceSlot {
    fn new(id: FileId, path: PathBuf) -> Self {
        Self { id, path, source: OnceLock::new() }
    }

    /// Carrega o source do disco (apenas na primeira chamada).
    fn get(&self) -> FileResult<Source> {
        self.source.get_or_init(|| {
            let text = std::fs::read_to_string(&self.path)
                .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                    FileError::NotFound
                } else {
                    FileError::Other(e.to_string())
                })?;
            Ok(Source::new(self.id, text))
        }).clone()
    }
}

/// Erro de criação do `SystemWorld`.
#[derive(Debug)]
pub enum SystemWorldError {
    /// O ficheiro principal não existe.
    MainNotFound(PathBuf),
    /// Erro de I/O ao processar o ficheiro principal.
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
/// Usa `SourceSlot`+`OnceLock` para carregamento lazy e thread-safe
/// das sources. `FontSlot`+`OnceLock` para as fontes (ADR-0019).
/// `Library` e `FontBook` são stubs opacos até ao Passo 5.
pub struct SystemWorld {
    /// Directório raiz do projecto.
    root:       PathBuf,
    /// `FileId` do ficheiro principal.
    main:       FileId,
    /// Slots de source por `FileId` (interior mutável via Mutex).
    slots:      Mutex<HashMap<FileId, Arc<SourceSlot>>>,
    /// Mapa de path canónico → `FileId`.
    path_to_id: Mutex<HashMap<PathBuf, FileId>>,
    /// Contador de IDs (não-global — sem V13).
    next_id:    Mutex<u16>,
    /// Slots de fontes (índice = parâmetro de `font()`).
    font_slots: Vec<FontSlot>,
    /// Stub do catálogo de fontes.
    font_book:  FontBook,
    /// Stub de biblioteca padrão.
    library:    Library,
}

impl SystemWorld {
    /// Cria um `SystemWorld` com `root` como directório base e
    /// `main` como ficheiro principal (relativo a `root` ou absoluto).
    /// Fonte slots inicializados com `Vec::new()` — usar `with_fonts`
    /// para adicionar fontes.
    pub fn new(root: impl Into<PathBuf>, main: impl AsRef<Path>) -> Result<Self, SystemWorldError> {
        let root = root.into();
        let main_path = if main.as_ref().is_absolute() {
            main.as_ref().to_path_buf()
        } else {
            root.join(main.as_ref())
        };

        let main_path_canon = main_path.canonicalize()
            .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                SystemWorldError::MainNotFound(main_path.clone())
            } else {
                SystemWorldError::Io(e)
            })?;

        let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let main_slot = Arc::new(SourceSlot::new(main_id, main_path_canon.clone()));

        // Carrega eagerly para falhar rápido se o ficheiro não existir.
        main_slot.get()
            .map_err(|_| SystemWorldError::MainNotFound(main_path_canon.clone()))?;

        let mut slots = HashMap::new();
        slots.insert(main_id, main_slot);

        let mut path_to_id = HashMap::new();
        path_to_id.insert(main_path_canon, main_id);

        Ok(Self {
            root,
            main: main_id,
            slots:      Mutex::new(slots),
            path_to_id: Mutex::new(path_to_id),
            next_id:    Mutex::new(2),
            font_slots: Vec::new(),
            font_book:  FontBook::new(),
            library:    Library::new(),
        })
    }

    /// Builder: associa slots de fontes ao world e popula o `FontBook`.
    pub fn with_fonts(mut self, font_slots: Vec<FontSlot>) -> Self {
        self.font_book  = crate::fonts::build_font_book(&font_slots);
        self.font_slots = font_slots;
        self
    }

    /// Regista um path e retorna o `FileId` correspondente
    /// (cria novo slot se o path ainda não estava registado).
    pub fn register_file(&self, path: PathBuf) -> FileId {
        let canon = path.canonicalize().unwrap_or(path.clone());

        let existing = self.path_to_id.lock().unwrap().get(&canon).copied();
        if let Some(id) = existing {
            return id;
        }

        let mut next = self.next_id.lock().unwrap();
        let raw = *next;
        *next = next.wrapping_add(1);
        if *next == 0 { *next = 1; }
        let id = FileId::from_raw(NonZeroU16::new(raw).expect("FileId counter exhausted"));

        self.path_to_id.lock().unwrap().insert(canon.clone(), id);
        self.slots.lock().unwrap().insert(id, Arc::new(SourceSlot::new(id, canon)));
        id
    }

    /// Directório raiz do projecto.
    pub fn root(&self) -> &Path {
        &self.root
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
        let slot = self.slots.lock().unwrap().get(&id).cloned();
        slot.ok_or(FileError::NotFound)?.get()
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = self.slots.lock().unwrap()
            .get(&id)
            .map(|s| s.path.clone());
        let path = path.ok_or(FileError::NotFound)?;
        std::fs::read(&path)
            .map(Bytes::new)
            .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
                FileError::NotFound
            } else {
                FileError::Other(e.to_string())
            })
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.font_slots.get(index)?.get()
    }

    fn read_bytes(&self, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let full_path = self.root.join(path);
        std::fs::read(&full_path)
            .map(|bytes| std::sync::Arc::new(bytes))
            .map_err(|e| format!("erro ao ler '{}': {}", path, e))
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        use time::OffsetDateTime;
        let now = OffsetDateTime::now_utc();
        let now = match offset {
            Some(h) => now + time::Duration::hours(h),
            None    => now,
        };
        Datetime::new_date(now.year(), now.month() as u8, now.day())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use typst_core::contracts::world::World;
    use typst_core::entities::font_book::FontBook;
    use typst_core::entities::world_types::{Bytes, Datetime, FileError, Font, Library};

    // ── MockWorld para testes sem filesystem ──────────────────────────────

    struct MockWorld {
        main_id: FileId,
        source:  Source,
        library: Library,
        book:    FontBook,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(42).unwrap());
            Self {
                main_id: id,
                source:  Source::new(id, text.to_string()),
                library: Library::new(),
                book:    FontBook::new(),
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
        fn file(&self, _: FileId)        -> FileResult<Bytes>   { Err(FileError::NotFound) }
        fn font(&self, _: usize)         -> Option<Font>        { None }
        fn today(&self, _: Option<i64>)  -> Option<Datetime>    { None }
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
    fn system_world_today_returns_some() {
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let dt = world.today(None);
        assert!(dt.is_some());
        // Ano razoável (> 2020) para verificar que é data real
        assert!(dt.unwrap().year() > 2020);
    }

    #[test]
    fn system_world_source_lazy_via_onclock() {
        // OnceLock: segunda chamada retorna o mesmo resultado
        let dir = tempfile_write("main.typ", "lazy content");
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let src1 = world.source(world.main()).unwrap();
        let src2 = world.source(world.main()).unwrap();
        assert!(src1 == src2);
    }

    #[test]
    fn system_world_register_file_returns_same_id() {
        let dir = tempfile_write("a.typ", "content a");
        let world = SystemWorld::new(dir.path(), "a.typ").unwrap();

        let extra = dir.path().join("extra.typ");
        std::fs::write(&extra, "extra").unwrap();

        let id1 = world.register_file(extra.clone());
        let id2 = world.register_file(extra);
        assert_eq!(id1, id2);
    }

    #[test]
    fn system_world_font_invalid_slot_returns_none() {
        let dir = tempfile_write("main.typ", "text");
        let font_dir = tempdir();
        std::fs::write(font_dir.path().join("fake.ttf"), b"not a font").unwrap();

        let slots = crate::fonts::discover_fonts(&[font_dir.path().to_path_buf()]);
        assert_eq!(slots.len(), 1);

        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_fonts(slots);

        // Slot existe mas bytes inválidos → font() retorna None
        assert!(world.font(0).is_none());
        // Índice fora dos limites → None
        assert!(world.font(1).is_none());
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
