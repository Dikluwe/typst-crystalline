//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/system-world.md
//! @prompt-hash 8396b5e9
//! @layer L3
//! @updated 2026-06-30
//!
//! **P515** — Adicionado `SystemWorld::with_system_fonts` e
//! `with_fonts_and_system` para descoberta automática de fontes do sistema
//! via `fontdb` (ativação da ADR-0020).

use std::collections::HashMap;
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use typst_core::contracts::package_downloader::PackageDownloader;
use typst_core::contracts::plugin_host::PluginHost;
use typst_core::contracts::world::{SysInputs, World};
use typst_core::entities::bib_entry::BibEntry;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::package_spec::PackageSpec;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

use crate::fonts::FontSlot;
use crate::image_sizer::apply_exif_rotation;

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

/// Bases de procura de pacotes, em ordem de prioridade (P678, P681): data dir
/// primeiro, depois cache dir. Lê variáveis de ambiente em L3 (I/O permitido;
/// L1 nunca lê env). Cada base é o directório-pai de `{namespace}/{name}/{version}`.
fn package_candidate_dirs() -> Vec<PathBuf> {
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
fn default_package_cache_dir() -> Option<PathBuf> {
    package_candidate_dirs().into_iter().last()
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
    /// **P694** — pares `--input chave=valor` para `sys.inputs`. Vazio por
    /// omissão; populado via `with_inputs`.
    inputs:     SysInputs,
    /// **P699** — host de plugins WASM (L3). `None` por omissão; instalado via
    /// `with_plugin_host`. Lido por `World::plugin_host()` em `native_plugin`.
    plugin_host: Option<Arc<dyn PluginHost>>,
    /// **P763a** — downloader de pacotes `@preview`. `None` se não houver
    /// cache dir configurável; usado por `resolve_package` quando o pacote
    /// não está presente localmente.
    package_downloader: Option<Box<dyn PackageDownloader>>,
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

        let package_downloader = default_package_cache_dir()
            .map(|cache_dir| Box::new(crate::package_downloader::HttpPackageDownloader::new(cache_dir))
                as Box<dyn PackageDownloader>);

        Ok(Self {
            root,
            main: main_id,
            slots:      Mutex::new(slots),
            path_to_id: Mutex::new(path_to_id),
            next_id:    Mutex::new(2),
            font_slots: Vec::new(),
            font_book:  FontBook::new(),
            library:    Library::new(),
            inputs:     SysInputs::default(),
            plugin_host: None,
            package_downloader,
        })
    }

    /// **P694** — Builder: associa os pares `--input` (raw strings), convertendo
    /// para `SysInputs` (`EcoString`). A conversão `String → EcoString` fica em
    /// L3, mantendo L2 livre de `ecow`/`indexmap`.
    pub fn with_inputs(mut self, inputs: Vec<(String, String)>) -> Self {
        let mut map = SysInputs::default();
        for (k, v) in inputs {
            map.insert(k.into(), v.into());
        }
        self.inputs = map;
        self
    }

    /// **P699** — Builder: instala o host de plugins WASM (L3). Consumido por
    /// `World::plugin_host()` em `native_plugin`. Encadeado em
    /// `04_wiring/src/main.rs` com `Arc::new(WasmiPluginHost::new())`.
    pub fn with_plugin_host(mut self, host: Arc<dyn PluginHost>) -> Self {
        self.plugin_host = Some(host);
        self
    }

    /// Builder: associa slots de fontes ao world e popula o `FontBook`.
    pub fn with_fonts(mut self, font_slots: Vec<FontSlot>) -> Self {
        self.font_book  = crate::fonts::build_font_book(&font_slots);
        self.font_slots = font_slots;
        self
    }

    /// Builder: descobre e associa as fontes instaladas no sistema.
    ///
    /// Usa `fontdb` para carregar as fontes do sistema operativo.
    /// Comportamento defensivo: se nenhuma fonte for encontrada, o
    /// `FontBook` fica vazio mas o `SystemWorld` continua funcional.
    pub fn with_system_fonts(mut self) -> Self {
        let (slots, book) = crate::fontdb::load_system_fonts();
        self.font_slots = slots;
        self.font_book  = book;
        self
    }

    /// Builder: carrega as fontes embutidas do vanilla (P753/P754).
    ///
    /// Usa `typst-assets` para obter os bytes em tempo de compilação.
    /// Isto garante que `Libertinus Serif` e as fontes de math/code do
    /// vanilla estejam sempre disponíveis.
    pub fn with_embedded_fonts(mut self) -> Self {
        let sets = crate::embedded_fonts::load_embedded_fonts();
        let mut slots = sets.text_slots;
        slots.extend(sets.math_code_slots);
        let mut book = sets.text_book;
        for info in sets.math_code_book.infos() {
            book.push(info.clone());
        }
        self.font_slots = slots;
        self.font_book  = book;
        self
    }

    /// Builder: combina fontes embutidas, do sistema e de projecto.
    ///
    /// A ordem final é: texto embutido → sistema → math/code embutido →
    /// projecto. Isto garante que fontes especializadas do sistema (ex.:
    /// Noto Sans Devanagari) sejam preferidas no fallback de texto normal
    /// sobre fontes math/code embutidas que podem ter cobertura parcial
    /// ou glifos incorrectos para scripts não latinos (P754).
    pub fn with_fonts_and_system(mut self, font_paths: &[PathBuf]) -> Self {
        let sets = crate::embedded_fonts::load_embedded_fonts();
        let mut slots = sets.text_slots;
        let mut book = sets.text_book;

        let (system_slots, system_book) = crate::fontdb::load_system_fonts();
        slots.extend(system_slots);
        for info in system_book.infos() {
            book.push(info.clone());
        }

        slots.extend(sets.math_code_slots);
        for info in sets.math_code_book.infos() {
            book.push(info.clone());
        }

        let project_slots = crate::fonts::discover_fonts(font_paths);
        let project_book = crate::fonts::build_font_book(&project_slots);
        slots.extend(project_slots);
        for info in project_book.infos() {
            book.push(info.clone());
        }

        self.font_slots = slots;
        self.font_book  = book;
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

    /// Directório do ficheiro identificado por `id`.
    /// Retorna a raiz se o FileId não estiver registado.
    fn directory_of(&self, id: FileId) -> PathBuf {
        self.slots.lock().unwrap()
            .get(&id)
            .and_then(|s| s.path.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| self.root.clone())
    }

    /// **P686** — Path registado para `id`, se existir.
    fn path_of(&self, id: FileId) -> Option<PathBuf> {
        self.slots.lock().unwrap().get(&id).map(|s| s.path.clone())
    }

    /// **P686** — Se `file_path` vive dentro de um pacote
    /// (`{base}/{namespace}/{name}/{version}/...`), devolve a raiz do pacote
    /// (`{base}/{namespace}/{name}/{version}`). Caso contrário, `None`.
    /// A detecção é por prefixo de path (sem I/O), coerente com `resolve_package`.
    fn package_root_of(&self, file_path: &Path) -> Option<PathBuf> {
        for base in package_candidate_dirs() {
            if let Ok(rest) = file_path.strip_prefix(&base) {
                let mut comps = rest.components();
                let (Some(ns), Some(name), Some(ver)) =
                    (comps.next(), comps.next(), comps.next())
                else {
                    continue;
                };
                return Some(base.join(ns.as_os_str()).join(name.as_os_str()).join(ver.as_os_str()));
            }
        }
        None
    }

    /// **P686** — Resolve `path` de um `#import`/`#include` contra `current_file`.
    ///
    /// - Absoluto (`/...`): base = raiz do pacote de `current_file` se existir,
    ///   senão `self.root` (raiz do projecto); junta `path` sem a barra inicial.
    /// - Relativo: `directory_of(current_file).join(path)` (sem regressão).
    fn resolve_path(&self, current_file: FileId, path: &str) -> PathBuf {
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
    pub fn load_bibliography(&self, current_file: FileId, path: &str) -> Result<Vec<BibEntry>, String> {
        let bytes = self.read_bytes(current_file, path)?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|e| format!("bibliography file is not valid UTF-8 '{}': {}", path, e))?;
        typst_core::rules::eval::bibtex::parse_bibtex(text)
            .map_err(|e| format!("failed to parse BibTeX '{}': {}", path, e))
    }

    /// **P681** — Lê o manifesto `typst.toml` de `dir`, extrai
    /// `[package].entrypoint`, regista o ficheiro do entrypoint e devolve o
    /// seu `Source`. Os imports internos do pacote resolvem relativamente ao
    /// `current_file` (entrypoint), cujo `directory_of` é o directório do
    /// entrypoint dentro da cache — logo `#import "..."` internos funcionam
    /// sem tratamento especial.
    fn load_package_entrypoint(&self, dir: &Path, spec: &PackageSpec) -> Result<Source, String> {
        let manifest_path = dir.join("typst.toml");
        let text = std::fs::read_to_string(&manifest_path).map_err(|e| {
            format!("pacote '{}': falha a ler '{}': {}", spec, manifest_path.display(), e)
        })?;
        let manifest: toml::Value = toml::from_str(&text).map_err(|e| {
            format!("pacote '{}': typst.toml inválido: {}", spec, e)
        })?;
        let entrypoint = manifest
            .get("package")
            .and_then(|p| p.get("entrypoint"))
            .and_then(|e| e.as_str())
            .ok_or_else(|| format!("pacote '{}': typst.toml sem [package].entrypoint", spec))?;
        let entry_path = dir.join(entrypoint);
        let id = self.register_file(entry_path.clone());
        self.source(id).map_err(|_| {
            format!("pacote '{}': entrypoint '{}' não encontrado", spec, entry_path.display())
        })
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

    fn read_bytes(&self, current_file: FileId, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let full_path = self.resolve_path(current_file, path);
        let data = std::fs::read(&full_path)
            .map_err(|e| format!("erro ao ler '{}': {}", path, e))?;
        // P774 — aplica rotação EXIF aos pixels de forma transparente para L1.
        let data = apply_exif_rotation(&data).unwrap_or(data);
        Ok(std::sync::Arc::new(data))
    }

    fn include_source(&self, current_file: FileId, path: &str) -> Result<typst_core::entities::source::Source, String> {
        let abs_path = self.resolve_path(current_file, path);
        let id = self.register_file(abs_path.clone());
        self.source(id).map_err(|_| format!("include: ficheiro não encontrado: {}", abs_path.display()))
    }

    fn resolve_package(&self, spec: &PackageSpec) -> Result<typst_core::entities::source::Source, String> {
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

        Err(format!(
            "pacote '{}' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)",
            spec
        ))
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

    /// **P694** — Devolve os pares `--input` (clone barato; poucos pares).
    fn inputs(&self) -> SysInputs {
        self.inputs.clone()
    }

    /// **P699** — Devolve o host de plugins WASM, se instalado via
    /// `with_plugin_host`. `None` ⇒ `native_plugin` erra "plugins não
    /// suportados neste World".
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>> {
        self.plugin_host.as_ref().map(Arc::clone)
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
    fn system_world_with_system_fonts_nao_panic() {
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_system_fonts();
        // Invariante: book.len() <= slots.len()
        assert!(world.book().len() <= world.font_slots.len());
    }

    #[test]
    fn system_world_with_fonts_and_system_inclui_projecto() {
        let dir = tempfile_write("main.typ", "text");
        let font_dir = tempdir();
        std::fs::write(font_dir.path().join("fake.ttf"), b"not a font").unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_fonts_and_system(&[font_dir.path().to_path_buf()]);

        // Sistema pode ou não ter fontes; projecto adiciona 1 slot.
        assert!(world.font_slots.len() >= 1);
    }

    #[test]
    fn system_world_with_fonts_and_system_preserva_ordem() {
        let dir = tempfile_write("main.typ", "text");
        let font_dir = tempdir();
        std::fs::write(font_dir.path().join("fake.ttf"), b"not a font").unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_fonts_and_system(&[font_dir.path().to_path_buf()]);

        // O último slot deve ser o do projecto.
        let last = world.font_slots.last().unwrap();
        assert_eq!(last.path.file_name().unwrap(), "fake.ttf");
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

    // ── Passo 450 — carregamento de .bib do disco ─────────────────────────

    #[test]
    fn system_world_load_bibliography_carrega_bib() {
        let dir = tempfile_write(
            "main.typ",
            "#bibliography(\"refs.bib\")",
        );
        std::fs::write(
            dir.path().join("refs.bib"),
            br#"@article{smith2024, author = {Smith, John}, title = {On Crystal Math}, year = {2024}, journal = {Journal of Examples}}"#,
        ).unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let entries = world.load_bibliography(world.main(), "refs.bib").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "smith2024");
        assert_eq!(entries[0].author, "Smith, John");
        assert_eq!(entries[0].title, "On Crystal Math");
        assert_eq!(entries[0].year, 2024);
    }

    #[test]
    fn system_world_load_bibliography_path_relativo_ao_source() {
        let dir = tempdir();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("main.typ"), "#bibliography(\"sub/refs.bib\")").unwrap();
        std::fs::write(
            dir.path().join("sub").join("refs.bib"),
            br#"@book{doe2023, author = {Doe, Jane}, title = {A Book}, year = {2023}}"#,
        ).unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let entries = world.load_bibliography(world.main(), "sub/refs.bib").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "doe2023");
    }

    // ── Passo 686 — caminhos absolutos `/...` ─────────────────────────────

    #[test]
    fn system_world_include_source_absoluto_no_projecto() {
        let dir = tempdir();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("main.typ"), "#include \"/src/util.typ\"").unwrap();
        std::fs::write(dir.path().join("src").join("util.typ"), "de util").unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let src = world.include_source(world.main(), "/src/util.typ").unwrap();
        assert_eq!(src.text(), "de util");
    }

    #[test]
    fn system_world_read_bytes_absoluto_no_projecto() {
        let dir = tempdir();
        std::fs::create_dir_all(dir.path().join("assets")).unwrap();
        std::fs::write(dir.path().join("main.typ"), "main").unwrap();
        std::fs::write(dir.path().join("assets").join("data.bin"), b"\x01\x02\x03").unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let bytes = world.read_bytes(world.main(), "/assets/data.bin").unwrap();
        assert_eq!(bytes.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn system_world_include_source_absoluto_em_pacote() {
        let dir = tempdir();
        // Pacote fake em {dir}/typst/packages/preview/foo/0.1.0
        let pkg = dir.path().join("typst/packages/preview/foo/0.1.0");
        std::fs::create_dir_all(pkg.join("src/sub")).unwrap();
        std::fs::write(
            pkg.join("typst.toml"),
            "[package]\nname = \"foo\"\nversion = \"0.1.0\"\nentrypoint = \"src/lib.typ\"\n",
        )
        .unwrap();
        std::fs::write(pkg.join("src/lib.typ"), "library").unwrap();
        std::fs::write(pkg.join("src/sub/x.typ"), "#import \"/src/lib.typ\"").unwrap();
        std::fs::write(dir.path().join("main.typ"), "main").unwrap();

        // package_candidate_dirs() lê XDG_DATA_HOME; apontamos para o temp dir
        // para que o pacote fake seja reconhecido. Restauramos no fim.
        let prev = std::env::var_os("XDG_DATA_HOME");
        std::env::set_var("XDG_DATA_HOME", dir.path());

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let x_id = world.register_file(pkg.join("src/sub/x.typ"));
        let src = world.include_source(x_id, "/src/lib.typ");

        if let Some(v) = prev {
            std::env::set_var("XDG_DATA_HOME", v);
        } else {
            std::env::remove_var("XDG_DATA_HOME");
        }

        // Resolve à raiz do pacote (0.1.0/src/lib.typ), não a src/sub.
        assert_eq!(src.unwrap().text(), "library");
    }

    #[test]
    fn system_world_include_source_relativo_sem_regressao() {
        let dir = tempdir();
        std::fs::create_dir_all(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("main.typ"), "main").unwrap();
        std::fs::write(dir.path().join("sub").join("x.typ"), "relativo").unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let src = world.include_source(world.main(), "sub/x.typ").unwrap();
        assert_eq!(src.text(), "relativo");
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
