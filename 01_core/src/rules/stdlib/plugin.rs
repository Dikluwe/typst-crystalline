//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/plugin.md
//! @prompt-hash 83ffb8a9
//! @layer L1
//! @updated 2026-07-10
//!
//! Builtin `plugin()` (P697) — nível 2 do mapa de P696: reconhece `plugin(...)`
//! e **lê os bytes** do `source` (caminho via `World::read_bytes`, ou `bytes`
//! directos). O runtime WASM (nível 3) fica para P698 — por isso, após ler os
//! bytes com sucesso, devolve um **erro provisório claro** (não um `Module`),
//! provando que a sintaxe e a leitura funcionam.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

fn err(msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), msg.into())]
}

fn reject_named(args: &Args) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(format!("argumento nomeado inesperado em plugin(): '{k}'")));
    }
    Ok(())
}

/// `plugin(source)` — carrega um plugin WebAssembly.
///
/// **P697 (nível 2):** resolve `source` para bytes e devolve erro provisório
/// (o runtime WASM chega em P698). `source` é `str` (caminho, lido via
/// `World::read_bytes`, com a mesma resolução relativa/`/...` de `read()` e
/// `#import`) ou `bytes` (conteúdo directo).
pub fn native_plugin(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args)?;

    let bytes: Vec<u8> = match args.items.as_slice() {
        [Value::Str(s)] => {
            let data = world
                .read_bytes(current_file, s.as_str())
                .map_err(|msg| err(format!("plugin(): não foi possível ler '{}': {msg}", s)))?;
            data.to_vec()
        }
        [Value::Bytes(b)] => b.as_slice().to_vec(),
        [other] => {
            return Err(err(format!(
                "plugin() requer caminho (str) ou bytes, recebeu {}",
                other.type_name()
            )));
        }
        _ => {
            return Err(err(format!(
                "plugin() requer 1 argumento, recebeu {}",
                args.items.len()
            )));
        }
    };

    // P697 — runtime WASM ainda não implementado (P698). A leitura/resolução
    // funcionou: chegámos aqui com `bytes.len()` bytes válidos.
    Err(err(format!(
        "plugin: runtime WASM ainda não implementado (P697); {} bytes lidos com sucesso",
        bytes.len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::bytes::Bytes;
    use std::collections::HashMap;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorld {
        files: HashMap<String, Arc<Vec<u8>>>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl Default for MockWorld {
        fn default() -> Self {
            Self {
                files: HashMap::new(),
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library { &self.library }
        fn book(&self) -> &crate::entities::font_book::FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> crate::entities::world_types::FileResult<crate::entities::source::Source> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(&self, _: FileId) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> { None }
        fn today(&self, _: Option<i64>) -> Option<crate::entities::world_types::Datetime> { None }
        fn read_bytes(&self, _current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
            self.files.get(path).cloned().ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    fn fid() -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }

    #[test]
    fn plugin_bytes_directo_devolve_erro_provisorio_com_contagem() {
        let world = MockWorld::default();
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![0, 1, 2, 3]))]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("runtime WASM ainda não implementado"), "msg: {}", e[0].message);
        assert!(e[0].message.contains("4 bytes lidos com sucesso"), "msg: {}", e[0].message);
    }

    #[test]
    fn plugin_caminho_existente_le_e_devolve_erro_provisorio() {
        let mut world = MockWorld::default();
        world.files.insert("hello.wasm".into(), Arc::new(vec![0x00, 0x61, 0x73, 0x6d])); // \0asm
        let args = Args::positional(vec![Value::Str("hello.wasm".into())]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("runtime WASM ainda não implementado"), "msg: {}", e[0].message);
        assert!(e[0].message.contains("4 bytes lidos com sucesso"), "msg: {}", e[0].message);
    }

    #[test]
    fn plugin_caminho_inexistente_erro_de_leitura() {
        let world = MockWorld::default();
        let args = Args::positional(vec![Value::Str("nao-existe.wasm".into())]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("plugin(): não foi possível ler 'nao-existe.wasm'"), "msg: {}", e[0].message);
        assert!(!e[0].message.contains("unknown variable"), "msg: {}", e[0].message);
    }

    #[test]
    fn plugin_tipo_errado_e_aridade_dao_erro_de_argumento() {
        let world = MockWorld::default();
        // tipo errado
        let args = Args::positional(vec![Value::Int(42)]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("requer caminho (str) ou bytes"), "msg: {}", e[0].message);
        // aridade errada
        let args = Args::positional(vec![]);
        let e = native_plugin(&mut EvalContext::new(), &args, &world, fid()).unwrap_err();
        assert!(e[0].message.contains("requer 1 argumento"), "msg: {}", e[0].message);
    }
}
