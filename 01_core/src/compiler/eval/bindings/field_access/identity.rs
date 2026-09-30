//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access/identity.md
//! @prompt-hash ac7a25b8
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn native_public_name(mut func: &Func) -> Option<&str> {
    use crate::entities::func::FuncRepr;
    loop {
        match func.repr() {
            FuncRepr::Native(native) => {
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
            FuncRepr::Native(native) => {
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
    module.name() == "format"
        && ["svg", "png", "pdf", "html"].into_iter().all(|name| {
            matches!(module.scope().get(name), Some(Value::Func(func)) if func.name() == Some(name))
        })
}

pub(super) fn is_effective_std_module(module: &crate::entities::module::Module) -> bool {
    module.name() == "global"
        && matches!(module.scope().get("type"), Some(Value::Type(Type::Type)))
        && matches!(module.scope().get("format"), Some(Value::Module(format)) if is_effective_format_module(format))
        && matches!(module.scope().get("document"), Some(Value::Func(func)) if func.name() == Some("document"))
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
