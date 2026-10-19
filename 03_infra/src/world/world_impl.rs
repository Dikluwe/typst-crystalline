//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/world/world_impl.md
//! @prompt-hash d93092a6
//! @layer L3
//! @updated 2026-09-26

use super::SystemWorld;
use std::sync::Arc;
use typst_core::contracts::plugin_host::PluginHost;
use typst_core::contracts::world::{SysInputs, World};
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::FontBook;
use typst_core::entities::package_spec::PackageSpec;
use typst_core::entities::path::RootedPath;
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{Bytes, Datetime, FileResult, Font, Library};

impl World for SystemWorld {
    fn library(&self) -> &Library {
        &self.library
    }

    fn book(&self) -> &FontBook {
        &self.font_catalog().1
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.world_source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.world_file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.world_font(index)
    }

    /// **P938/P1410** — cobertura Unicode exacta híbrida por slot.
    ///
    /// Fontes do sistema chegam com coverage materializada pelo `fontdb`;
    /// somente slots ainda vazios abrem seus bytes e usam o cache lazy. Isso
    /// impede que uma consulta mapeie todos os ficheiros de fontes instaladas.
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        self.world_candidates_for_char(c)
    }

    fn resolve_path(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<RootedPath, String> {
        self.world_resolve_path(current_file, path)
    }

    fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        self.world_read_path(path)
    }

    fn include_path(&self, path: &RootedPath) -> Result<Source, String> {
        self.world_include_path(path)
    }

    fn read_bytes(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<std::sync::Arc<Vec<u8>>, String> {
        self.world_read_bytes(current_file, path)
    }

    fn include_source(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<typst_core::entities::source::Source, String> {
        self.world_include_source(current_file, path)
    }

    fn resolve_package(
        &self,
        spec: &PackageSpec,
    ) -> Result<typst_core::entities::source::Source, String> {
        self.world_resolve_package(spec)
    }

    fn today(
        &self,
        offset: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        use time::OffsetDateTime;
        let now = match offset {
            Some(offset) => {
                let seconds = i64::try_from(offset.nanos / 1_000_000_000).ok()?;
                let nanos = (offset.nanos % 1_000_000_000) as i32;
                OffsetDateTime::now_utc()
                    .checked_add(time::Duration::new(seconds, nanos))?
            }
            None => OffsetDateTime::now_local().ok()?,
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
