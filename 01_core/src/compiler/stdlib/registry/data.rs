//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/data.md
//! @prompt-hash cb95551a
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::{
    native_cbor, native_cbor_encode, native_csv, native_json, native_json_encode,
    native_plugin, native_plugin_transition, native_read, native_toml,
    native_toml_encode, native_xml, native_yaml, native_yaml_encode,
};
use crate::contracts::world::World;
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use std::sync::Arc;

pub(super) fn register(scope: &mut Scope) {
    // P387 (ADR-0111) — data import: read + 6 parsers. Decode L1 puro compõe
    // com L3 World::read_bytes. Paridade do Value de saída (ADR-0107).
    scope.define("read", Value::Func(Func::native("read", native_read)));
    scope.define("csv", Value::Func(Func::native("csv", native_csv)));
    type Codec = fn(
        &mut EvalContext,
        &crate::entities::args::Args,
        &dyn World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value>;
    let codecs: [(&'static str, Codec, Codec); 3] = [
        ("json", native_json, native_json_encode),
        ("yaml", native_yaml, native_yaml_encode),
        ("toml", native_toml, native_toml_encode),
    ];
    for (name, decode, encode) in codecs {
        let mut namespace = Scope::new();
        namespace.define("encode", Value::Func(Func::native("encode", encode)));
        scope.define(
            name,
            Value::Func(Func::native_with_namespace(name, decode, Arc::new(namespace))),
        );
    }
    // P701 — `cbor` ganha namespace com `encode` (mesmo padrão de curve/grid/table).
    {
        let mut cbor_namespace = Scope::new();
        cbor_namespace.define(
            "encode",
            Value::Func(Func::native("cbor.encode", native_cbor_encode)),
        );
        scope.define(
            "cbor",
            Value::Func(Func::native_with_namespace(
                "cbor",
                native_cbor,
                Arc::new(cbor_namespace),
            )),
        );
    }
    scope.define("xml", Value::Func(Func::native("xml", native_xml)));
    // P697 — builtin `plugin()` (nível 2 de P696: sintaxe + leitura; runtime em P698).
    // P819 — `plugin` ganha namespace com `transition` (mesmo padrão de cbor/curve/grid).
    {
        let mut plugin_namespace = Scope::new();
        plugin_namespace.define(
            "transition",
            Value::Func(Func::native("plugin.transition", native_plugin_transition)),
        );
        scope.define(
            "plugin",
            Value::Func(Func::native_with_namespace(
                "plugin",
                native_plugin,
                Arc::new(plugin_namespace),
            )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("json").unwrap() else {
            panic!("json function");
        };
        assert_eq!(func.name(), Some("json"));
    }
}
