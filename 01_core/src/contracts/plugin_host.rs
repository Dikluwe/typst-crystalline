//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/contracts/plugin_host.md
//! @prompt-hash 53b005a7
//! @layer L1
//! @updated 2026-07-10
//!
//! Fronteira do runtime WASM de plugins (P698, nível 3 de P696). L1 vê só o
//! contrato (`PluginHost`) e tipos de fronteira (`PluginModuleId`,
//! `PluginError`); o runtime `wasmi` vive em L3 e **nunca** atravessa aqui
//! (sem `use wasmi`, sem I/O — V3/V4/V14 limpos).

use std::fmt;

use ecow::EcoString;

use crate::entities::bytes::Bytes;

/// Handle opaco para um módulo WASM já carregado por um [`PluginHost`].
/// `Copy` para passar por valor em [`PluginHost::call`]; o conteúdo do módulo
/// fica em L3 — L1 só carrega o id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PluginModuleId(pub u64);

/// Erro do host de plugins. A `message` é **contrato** (não detalhe de L3):
/// é o observável da linguagem (ADR-0107) e tem de coincidir com o vanilla
/// verbatim (catálogo em `00_nucleo/prompts/infra/plugin_host.md`). L3
/// preenche, L1/L2 propagam sem reescrever.
///
/// Só `EcoString` (ADR-0024, em `[l1_allowed_external]`) — nenhum tipo externo
/// de runtime atravessa a fronteira (V14).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError {
    pub message: EcoString,
}

impl PluginError {
    /// Cria um erro com a mensagem exacta (verbatim) a propagar.
    pub fn new(msg: impl Into<EcoString>) -> Self {
        Self { message: msg.into() }
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl std::error::Error for PluginError {}

/// Host de plugins WebAssembly — fronteira entre o núcleo (L1) e o runtime
/// (L3, `WasmiPluginHost`). Injectado como `Arc<dyn PluginHost>` no eval via
/// `World::plugin_host()` (P699); em P698 é testado directamente em Rust (sem
/// sintaxe Typst).
///
/// `Send + Sync` é obrigatório: `World: Send + Sync` e o host viaja em
/// `SystemWorld` e dentro de `Value::Func(FuncRepr::Plugin(...))` (precedente:
/// `ElementCtor`).
pub trait PluginHost: Send + Sync {
    /// Compila e valida os bytes WASM, devolvendo um handle. Falha (mensagem
    /// exacta do vanilla) se o módulo é inválido ou não exporta `memory`.
    fn load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError>;

    /// **P699** — Enumera os nomes exportados pelo módulo `module` que são
    /// funções (`ExternType::Func`), na ordem estável do host. Usado por
    /// `native_plugin` para construir o `Module` com uma função por export.
    /// Réplica de `into_module` (`plugin.rs:366-380`), filtrando só funções.
    fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError>;

    /// Chama a função exportada `func_name` do módulo `module` com os buffers
    /// `args` (cada [`Bytes`] é um argumento; as lengths passam ao WASM como
    /// `i32`, o conteúdo via `typst_env`). Devolve os bytes de saída ou um
    /// [`PluginError`] com a mensagem exacta do vanilla.
    fn call(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<Bytes, PluginError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_error_preserva_mensagem_verbatim() {
        let e = PluginError::new("plugin does not export its memory");
        assert_eq!(e.message.as_str(), "plugin does not export its memory");
        assert_eq!(format!("{e}"), "plugin does not export its memory");
    }

    #[test]
    fn plugin_module_id_eh_copy_e_hashable() {
        let a = PluginModuleId(7);
        let b = a; // Copy
        assert_eq!(a, b);
        let mut set = std::collections::HashSet::new();
        set.insert(a);
        assert!(set.contains(&PluginModuleId(7)));
    }

    #[test]
    fn trait_eh_object_safe() {
        // Garante que `Arc<dyn PluginHost>` (injectão em P699) é possível.
        fn _accepts(_h: &dyn PluginHost) {}
    }
}
