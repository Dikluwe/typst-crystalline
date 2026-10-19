//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world.md
//! @prompt-hash 2fd4f699
//! @layer L3
//! @updated 2026-10-08
//!
//! Fachada e composição: estado e construtores de SystemWorld.
//! Cache, paths, pacotes, fontes e adapter World vivem nos descendentes.

use crate::fonts::FontSlot;
use std::collections::HashMap;
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use typst_core::contracts::package_downloader::PackageDownloader;
use typst_core::contracts::plugin_host::PluginHost;
use typst_core::contracts::world::SysInputs;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::{Coverage, FontBook};
use typst_core::entities::world_types::Library;

mod cache;
mod font_session;
mod source_session;
pub use source_session::SourceSession;
mod fonts;
pub use font_session::FontCatalogSession;
mod packages;
mod paths;
#[cfg(test)]
mod tests;
mod world_impl;

use cache::SourceSlot;
use packages::default_package_cache_dir;

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
    root: PathBuf,
    source_session: Option<Arc<SourceSession>>,
    /// `FileId` do ficheiro principal.
    main: FileId,
    /// Slots de source por `FileId` (interior mutável via Mutex).
    slots: Mutex<HashMap<FileId, Arc<SourceSlot>>>,
    /// Mapa de path canónico → `FileId`.
    path_to_id: Mutex<HashMap<PathBuf, FileId>>,
    /// Contador de IDs (não-global — sem V13).
    next_id: Mutex<u32>,
    /// Paired font slots/book, initialized once on the first font demand.
    font_catalog: OnceLock<Arc<(Vec<FontSlot>, FontBook)>>,
    /// Private acquisition policy; no font discovery occurs while configuring it.
    font_loader: Box<dyn Fn() -> Arc<(Vec<FontSlot>, FontBook)> + Send + Sync>,
    /// Stub de biblioteca padrão.
    library: Library,
    /// **P694** — pares `--input chave=valor` para `sys.inputs`. Vazio por
    /// omissão; populado via `with_inputs`.
    inputs: SysInputs,
    /// **P699** — host de plugins WASM (L3). `None` por omissão; instalado via
    /// `with_plugin_host`. Lido por `World::plugin_host()` em `native_plugin`.
    plugin_host: Option<Arc<dyn PluginHost>>,
    /// **P763a** — downloader de pacotes `@preview`. `None` se não houver
    /// cache dir configurável; usado por `resolve_package` quando o pacote
    /// não está presente localmente.
    package_downloader: Option<Box<dyn PackageDownloader>>,
    /// **P876** — cache de bytes brutos lidos via `read_bytes`/`file`,
    /// indexado por `FileId` canónico. Evita releituras e garante que
    /// chamadas repetidas ao mesmo ficheiro partilham o mesmo `Arc<Vec<u8>>`.
    read_cache: Mutex<HashMap<FileId, Arc<Vec<u8>>>>,
    /// **P938** — cache lazy de cobertura Unicode exacta por índice de slot.
    /// Evita iterar a tabela `cmap` de todas as fontes no startup; só
    /// computa quando o slot é primeiro consultado por `candidates_for_char`.
    coverage_cache: Mutex<HashMap<usize, Coverage>>,
}

impl SystemWorld {
    /// Cria um world para avaliação de expressão sem exigir main físico.
    pub fn for_eval(root: impl Into<PathBuf>) -> Self {
        Self::with_transient_main(root.into(), "<input-expression>", String::new())
    }

    /// Cria um documento stdin a partir do texto já adquirido pelo caller.
    pub fn for_stdin(root: impl Into<PathBuf>, text: String) -> Self {
        Self::with_transient_main(root.into(), "<stdin>", text)
    }

    fn with_transient_main(root: PathBuf, name: &str, text: String) -> Self {
        let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let bytes = Arc::new(text.as_bytes().to_vec());
        let main_slot = Arc::new(SourceSlot::synthetic(main_id, root.join(name), text));
        let mut slots = HashMap::new();
        slots.insert(main_id, main_slot);
        let mut read_cache = HashMap::new();
        read_cache.insert(main_id, bytes);
        let package_downloader = default_package_cache_dir().map(|cache_dir| {
            Box::new(crate::package_downloader::HttpPackageDownloader::new(cache_dir))
                as Box<dyn PackageDownloader>
        });
        Self {
            root,
            source_session: None,
            main: main_id,
            slots: Mutex::new(slots),
            path_to_id: Mutex::new(HashMap::new()),
            next_id: Mutex::new(2),
            font_catalog: OnceLock::from(Arc::new((Vec::new(), FontBook::new()))),
            font_loader: Box::new(|| Arc::new((Vec::new(), FontBook::new()))),
            library: Library::new(),
            inputs: SysInputs::default(),
            plugin_host: None,
            package_downloader,
            read_cache: Mutex::new(read_cache),
            coverage_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Cria um `SystemWorld` com `root` como directório base e
    /// `main` como ficheiro principal (relativo a `root` ou absoluto).
    /// Fonte slots inicializados com `Vec::new()` — usar `with_fonts`
    /// para adicionar fontes.
    pub fn new(
        root: impl Into<PathBuf>,
        main: impl AsRef<Path>,
    ) -> Result<Self, SystemWorldError> {
        Self::new_configured(root.into(), main.as_ref(), None)
    }

    /// Build a fresh World with validated syntax retention from this watch session.
    pub fn new_with_source_session(
        root: impl Into<PathBuf>,
        main_path: impl AsRef<Path>,
        session: Arc<SourceSession>,
    ) -> Result<Self, SystemWorldError> {
        Self::new_configured(root.into(), main_path.as_ref(), Some(session))
    }

    fn new_configured(
        root: PathBuf,
        main: &Path,
        source_session: Option<Arc<SourceSession>>,
    ) -> Result<Self, SystemWorldError> {
        let main_path =
            if main.is_absolute() { main.to_path_buf() } else { root.join(main) };

        let main_path_canon = main_path.canonicalize().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                SystemWorldError::MainNotFound(main_path.clone())
            } else {
                SystemWorldError::Io(e)
            }
        })?;

        let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let main_slot = Arc::new(SourceSlot::with_session(
            main_id,
            main_path_canon.clone(),
            source_session.clone(),
        ));

        // Carrega eagerly para falhar rápido se o ficheiro não existir.
        main_slot
            .get()
            .map_err(|_| SystemWorldError::MainNotFound(main_path_canon.clone()))?;

        let mut slots = HashMap::new();
        slots.insert(main_id, main_slot);

        let mut path_to_id = HashMap::new();
        path_to_id.insert(main_path_canon, main_id);

        let package_downloader = default_package_cache_dir().map(|cache_dir| {
            Box::new(crate::package_downloader::HttpPackageDownloader::new(cache_dir))
                as Box<dyn PackageDownloader>
        });

        Ok(Self {
            root,
            source_session,
            main: main_id,
            slots: Mutex::new(slots),
            path_to_id: Mutex::new(path_to_id),
            next_id: Mutex::new(2),
            font_catalog: OnceLock::from(Arc::new((Vec::new(), FontBook::new()))),
            font_loader: Box::new(|| Arc::new((Vec::new(), FontBook::new()))),
            library: Library::new(),
            inputs: SysInputs::default(),
            plugin_host: None,
            package_downloader,
            read_cache: Mutex::new(HashMap::new()),
            coverage_cache: Mutex::new(HashMap::new()),
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
}
