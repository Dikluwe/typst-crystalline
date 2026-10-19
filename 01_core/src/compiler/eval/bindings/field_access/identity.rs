//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/field_access/identity.md
//! @prompt-hash 363e7519
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn native_public_name(mut func: &Func) -> Option<&str> {
    use crate::entities::func::FuncRepr;
    loop {
        match func.repr() {
            FuncRepr::Native(native) | FuncRepr::Format(_, native) => {
                return native.name.rsplit('.').next();
            }
            FuncRepr::NativeWithEngine(native) => {
                return native.name.rsplit('.').next();
            }
            FuncRepr::With(with) => func = &with.0,
            _ => return None,
        }
    }
}

/// Identify closures without inferring their category from a public name.
pub(super) fn is_user_defined_closure(mut func: &Func) -> bool {
    use crate::entities::func::FuncRepr;
    loop {
        match func.repr() {
            FuncRepr::Closure(_) => return true,
            FuncRepr::With(with) => func = &with.0,
            FuncRepr::Native(_)
            | FuncRepr::Format(_, _)
            | FuncRepr::NativeWithEngine(_)
            | FuncRepr::Element(_)
            | FuncRepr::Plugin(_) => return false,
        }
    }
}

/// **P1471** — reconhece tanto o namespace da raiz global `pdf` (só
/// `attach`/`artifact`[/trio a11y]) quanto a projeção `format.pdf` (que
/// acrescenta `pages`/`standard`/`tagged`/`pretty` por cima dos mesmos dois
/// bindings clonados). Antes de P1471, só `format.pdf` chegava aqui — a
/// raiz `pdf` era reconhecida pelo ramo `Value::Module` agora removido
/// (ver `field_access.md` P1471). Os quatro defaults extras são exclusivos
/// da projeção e não distinguem re-identificação válida; `attach`/`artifact`
/// como `Value::Func` já são suficientemente específicos combinados com o
/// nome nativo `"pdf"` verificado pelo chamador.
pub(super) fn is_format_pdf_namespace(namespace: &crate::entities::scope::Scope) -> bool {
    matches!(namespace.get("attach"), Some(Value::Func(_)))
        && matches!(namespace.get("artifact"), Some(Value::Func(_)))
}

pub(super) fn is_format_pdf_function(mut func: &Func) -> bool {
    use crate::entities::func::FuncRepr;

    loop {
        match func.repr() {
            FuncRepr::With(with) => func = &with.0,
            FuncRepr::Native(native) | FuncRepr::Format(_, native) => {
                return native.name == "pdf"
                    && native.namespace.as_deref().is_some_and(is_format_pdf_namespace);
            }
            FuncRepr::Closure(_)
            | FuncRepr::NativeWithEngine(_)
            | FuncRepr::Element(_)
            | FuncRepr::Plugin(_) => return false,
        }
    }
}

pub(super) fn is_a11y_pdf_target(target: &Value) -> bool {
    match target {
        Value::Func(func) => is_format_pdf_function(func),
        _ => false,
    }
}

pub(super) fn is_effective_format_module(
    module: &crate::entities::module::Module,
) -> bool {
    module.is_format()
}

pub(super) fn is_effective_std_module(module: &crate::entities::module::Module) -> bool {
    module.is_std()
}
/// Compara por identidade Arc com o namespace da stdlib, não por nome: um
/// `sym.typ` ou `math.typ` importado pode ter o mesmo `Module::name()`.
pub(super) fn is_effective_symbol_module(
    scopes: &Scopes<'_>,
    module: &crate::entities::module::Module,
    name: &str,
) -> bool {
    if module.name() != name {
        return false;
    }
    let std = scopes
        .base
        .and_then(|base| base.global.get("std"))
        .or_else(|| scopes.captured.as_ref().and_then(|captured| captured.get("std")));
    matches!(std, Some(Value::Module(std)) if
        matches!(std.scope().get(name), Some(Value::Module(canonical)) if canonical == module))
}

/// Emite a depreciação de um binding de símbolo já resolvido. O caller
/// preserva o Module para a checagem por identidade depois do lookup.
pub(in crate::compiler::eval) fn warn_deprecated_symbol_binding(
    scopes: &Scopes<'_>,
    module: &crate::entities::module::Module,
    field: &str,
    value: &Value,
    span: Span,
    engine: &mut Engine<'_>,
) {
    if !matches!(value, Value::Symbol(_)) {
        return;
    }
    let in_sym = is_effective_symbol_module(scopes, module, "sym");
    let in_math = is_effective_symbol_module(scopes, module, "math");
    if in_sym || (in_math && field != "join") {
        if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(field) {
            engine.sink.warn_note(span, msg, "");
        }
    }
}

#[cfg(test)]
mod native_format_catalogue_projection_tests {
    use super::*;
    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::args::Args;
    use crate::entities::func::{ClosureRepr, Func};
    use crate::entities::plugin_func::PluginFunc;
    struct Host;
    impl PluginHost for Host {
        fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
            panic!("metadata called host")
        }
        fn exports(
            &self,
            _: PluginModuleId,
        ) -> Result<Vec<ecow::EcoString>, PluginError> {
            panic!("metadata called host")
        }
        fn call(
            &self,
            _: PluginModuleId,
            _: &str,
            _: &[crate::entities::bytes::Bytes],
        ) -> Result<crate::entities::bytes::Bytes, PluginError> {
            panic!("metadata called host")
        }
        fn transition(
            &self,
            _: PluginModuleId,
            _: &str,
            _: &[crate::entities::bytes::Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            panic!("metadata called host")
        }
    }
    #[test]
    fn native_format_catalogue_projection_groups_cover_all_representations() {
        let scope = crate::compiler::stdlib::make_stdlib_with_features(
            &crate::contracts::world::SysInputs::default(),
            crate::entities::compiler_features::Features::empty(),
        );
        let Value::Module(module) = scope.get("format").unwrap() else {
            panic!("format")
        };
        let Value::Func(svg) = module.scope().get("svg").unwrap() else { panic!("svg") };
        let Value::Func(pdf) = module.scope().get("pdf").unwrap() else { panic!("pdf") };
        let native = Func::native("ns.sample", crate::compiler::stdlib::native_assert);
        let source = crate::entities::source::Source::detached("x");
        let closure = Func::closure(ClosureRepr {
            name: None,
            params: vec![],
            sink_name: None,
            body: source.root().clone(),
            captured: std::sync::Arc::new(crate::entities::scope::Scope::new()),
            capturer: crate::entities::scope::Capturer::Function,
        });
        let functions = [
            native.clone(),
            svg.clone(),
            Func::native_with_engine("ns.sample", crate::compiler::stdlib::native_op),
            closure.clone(),
            Func::element(
                "sample",
                std::sync::Arc::new(|_| panic!("metadata called ctor")),
            ),
            Func::plugin(PluginFunc {
                host: std::sync::Arc::new(Host),
                module: PluginModuleId(0),
                name: "sample".into(),
            }),
            native.with(Args::positional(vec![])),
        ];
        let names = [
            Some("sample"),
            Some("svg"),
            Some("sample"),
            None,
            None,
            None,
            Some("sample"),
        ];
        for (i, function) in functions.iter().enumerate() {
            assert_eq!(native_public_name(function), names[i]);
            assert_eq!(is_user_defined_closure(function), i == 3);
            assert!(!is_format_pdf_function(function));
        }
        assert!(is_user_defined_closure(&closure.with(Args::positional(vec![]))));
        assert!(!is_user_defined_closure(&svg.clone().with(Args::positional(vec![]))));
        let unmarked = Func::native_with_namespace(
            "pdf",
            pdf.native_fn_addr().unwrap(),
            std::sync::Arc::new(pdf.namespace().unwrap().clone()),
        );
        for function in
            [pdf.clone(), unmarked, pdf.clone().with(Args::positional(vec![]))]
        {
            assert!(is_format_pdf_function(&function));
        }
    }
}

#[cfg(test)]
mod builtin_module_identity_tests {
    use super::*;
    use crate::contracts::world::SysInputs;
    use crate::entities::compiler_features::Features;
    use crate::entities::module::Module;

    #[test]
    fn builtin_module_identity_copied_native_bindings_do_not_authenticate_user_module() {
        let scope = crate::compiler::stdlib::make_stdlib_with_features(
            &SysInputs::default(),
            Features::empty(),
        );
        let Value::Module(format) = scope.get("format").unwrap() else {
            panic!("real format module");
        };
        assert!(is_effective_format_module(format));
        let fake_format = Module::new("format", format.scope().clone());
        assert!(!is_effective_format_module(&fake_format));
        let fake_std = Module::new("global", scope);
        assert!(!is_effective_std_module(&fake_std));
        assert!(!is_effective_std_module(&fake_std.clone()));
        assert!(!is_effective_format_module(&fake_format.clone()));
    }
}
