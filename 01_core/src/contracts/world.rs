//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/contracts/world.md
//! @prompt-hash a6301e13
//! @layer L1
//! @updated 2026-07-10

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::contracts::plugin_host::PluginHost;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::package_spec::PackageSpec;
use crate::entities::source::Source;
use crate::entities::world_types::{Bytes, Datetime, FileResult, Font, Library};

/// Pares `chave → valor` passados à CLI via `--input chave=valor` e expostos à
/// linguagem em `sys.inputs` (P694). `IndexMap` preserva a ordem de inserção
/// (repr determinista); `FxBuildHasher` é o hasher de `Value::Dict`; `EcoString`
/// dá clone O(1). Os três crates estão em `[l1_allowed_external]`, logo o alias
/// na assinatura do trait não dispara V14. Os valores são sempre strings
/// (paridade vanilla: `--input n=42` → `sys.inputs.n == "42"`).
pub type SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>;

/// Contrato entre o compilador Typst e o ambiente de execução.
///
/// Sem `comemo` — testável com qualquer implementação simples.
/// A separação `World` / `TrackedWorld` segue o padrão B3 (ADR-0005):
/// o contrato puro declara *o quê*; `TrackedWorld` adiciona *como*
/// o pipeline incremental rastreia as dependências.
pub trait World: Send + Sync {
    /// A biblioteca de funções e valores padrão do Typst.
    fn library(&self) -> &Library;

    /// O catálogo de fontes disponíveis.
    fn book(&self) -> &FontBook;

    /// O ficheiro principal a compilar.
    fn main(&self) -> FileId;

    /// Obter o source de um ficheiro pelo seu id.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Obter o conteúdo binário de um ficheiro pelo seu id.
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Ler ficheiro binário por caminho relativo ao ficheiro actual (Passo 75, DEBT-25).
    /// `current_file`: FileId do ficheiro em avaliação — base para resolução relativa.
    /// Retorna `Arc<Vec<u8>>` partilhado — clones do AST não copiam os bytes.
    /// Implementação por omissão: retorna Err — MockWorlds que não precisam de
    /// I/O não necessitam de implementar este método.
    fn read_bytes(
        &self,
        current_file: FileId,
        path: &str,
    ) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let _ = current_file;
        Err(format!("leitura de ficheiro por caminho não suportada: {}", path))
    }

    /// Carregar um ficheiro Typst incluído via `#include` (Passo 75, DEBT-25).
    /// Resolve `path` relativamente ao directório do ficheiro `current_file`.
    /// Implementação por omissão: retorna Err — MockWorlds sem filesystem.
    fn include_source(&self, current_file: FileId, path: &str) -> Result<Source, String> {
        let _ = current_file;
        Err(format!("include não suportado nesta implementação de World: {}", path))
    }

    /// Resolver um `PackageSpec` (`@preview/nome:versao`) para o `Source` do
    /// entrypoint do pacote, procurando na cache local (P681, P-β de P678).
    /// A resolução real (data dir → cache dir, manifesto `typst.toml`,
    /// `entrypoint`) é I/O e vive em L3 (`SystemWorld`); L1 só declara o
    /// contrato. Implementação por omissão: retorna Err — MockWorlds e worlds
    /// sem filesystem não resolvem pacotes.
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        let _ = spec;
        Err("resolução de pacotes não suportada nesta implementação de World".into())
    }

    /// Pares `--input chave=valor` da CLI para expor em `sys.inputs` (P694).
    /// Implementação por omissão retorna vazio — MockWorlds e worlds sem CLI
    /// não precisam de implementar este método. Devolve owned (clone barato —
    /// poucos pares); L1 lê-o em `eval_with_full_error` para construir `sys`.
    fn inputs(&self) -> SysInputs {
        SysInputs::default()
    }

    /// **P699** — Host de plugins WASM, se este `World` suportar plugins.
    /// `native_plugin` chama-o para obter o `Arc<dyn PluginHost>`; `None`
    /// ⇒ erro "plugins não suportados neste World". Implementação por
    /// omissão retorna `None` — MockWorlds e worlds sem runtime WASM não
    /// precisam de implementar. `SystemWorld` (L3) sobrescreve e devolve
    /// `Some(...)` quando tem um host instalado via `with_plugin_host`.
    fn plugin_host(&self) -> Option<std::sync::Arc<dyn PluginHost>> {
        None
    }

    /// Obter uma fonte pelo índice no `FontBook`.
    fn font(&self, index: usize) -> Option<Font>;

    /// A data actual com offset em horas UTC (None se indisponível).
    ///
    /// Usa `i64` em vez de `Duration` — o tipo `Duration` do Typst
    /// não existe em L1 neste passo.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        main_id: FileId,
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main_id
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
    }

    fn mock() -> MockWorld {
        MockWorld {
            library: Library::new(),
            book: FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }

    #[test]
    fn world_main_returns_correct_id() {
        let w = mock();
        let expected = FileId::from_raw(NonZeroU16::new(1).unwrap());
        assert_eq!(World::main(&w), expected);
    }

    #[test]
    fn world_source_not_found() {
        let w = mock();
        let id = FileId::from_raw(NonZeroU16::new(2).unwrap());
        assert!(matches!(World::source(&w, id), Err(FileError::NotFound)));
    }

    #[test]
    fn world_file_not_found() {
        let w = mock();
        let id = FileId::from_raw(NonZeroU16::new(2).unwrap());
        assert!(matches!(World::file(&w, id), Err(FileError::NotFound)));
    }

    #[test]
    fn world_font_none() {
        assert!(World::font(&mock(), 0).is_none());
    }

    #[test]
    fn world_today_none() {
        let w = mock();
        assert!(World::today(&w, None).is_none());
        assert!(World::today(&w, Some(2)).is_none());
    }

    #[test]
    fn world_pure_no_comemo_import_needed() {
        // World pura compila sem importar comemo — testabilidade confirmada.
        // Chamadas explícitas via World trait para evitar ambiguidade com TrackedWorld.
        // Contrato correcto — teste adicionado para prevenir regressão.
        let w = mock();
        let _ = World::library(&w);
        let _ = World::book(&w);
    }
}
