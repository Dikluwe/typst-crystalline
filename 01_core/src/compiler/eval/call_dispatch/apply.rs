//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/apply.md
//! @prompt-hash 4fa10e90
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn p1339_native_call_span(func: &Func) -> bool {
    if let FuncRepr::With(with) = func.repr() {
        return p1339_native_call_span(&with.0);
    }
    let Some(addr) = func.native_fn_addr() else { return false };
    if std::ptr::fn_addr_eq(addr, angle_deg_static as fn(_, _, _, _) -> _)
        || std::ptr::fn_addr_eq(addr, angle_rad_static as fn(_, _, _, _) -> _)
        || std::ptr::fn_addr_eq(addr, function_with_static as fn(_, _, _, _) -> _)
        || std::ptr::fn_addr_eq(
            addr,
            bindings::native_function_where as fn(_, _, _, _) -> _,
        )
    {
        return true;
    }
    [
        crate::compiler::stdlib::float_type_field("signum"),
        crate::compiler::stdlib::float_type_field("from-bytes"),
        crate::compiler::stdlib::float_type_field("to-bytes"),
        crate::compiler::stdlib::version_type_field("at"),
    ]
    .into_iter()
    .flatten()
    .any(|value| match value {
        Value::Func(native) => native
            .native_fn_addr()
            .is_some_and(|other| std::ptr::fn_addr_eq(addr, other)),
        _ => false,
    })
}

/// Aplica uma função (closure, native ou native-with-engine) aos args dados.
pub fn apply_func(
    func: Func,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let previous_styles = ctx.replace_context_read_styles(engine.styles.clone());
    let previous_file = ctx.replace_context_read_file(Some(engine.current_file));
    let previous_rules = ctx.replace_context_read_rules(
        engine.show_rules.clone(),
        engine.active_guards.clone(),
    );
    let result = match func.repr() {
        FuncRepr::Closure(closure) => {
            closures::apply_closure(closure, &func, args, ctx, engine)
        }
        // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — `#name(args)`
        // invoca o construtor do registry e devolve `Content::Dynamic`. Mesmo
        // ponto de despacho dos nativos (sem caminho paralelo). Erro do catálogo
        // existente se o ctor falhar (não panic).
        FuncRepr::Element(ef) => (ef.ctor)(&args.items).map(Value::Content),
        // P699 — export de plugin WASM: delega ao host capturado (memoizado
        // em `PluginFunc::call`). Valida args (só bytes) e propaga erros
        // verbatim (`PluginError.message` é observável — ADR-0107).
        FuncRepr::Plugin(p) => call_plugin(p, &args),
        // P702 — aplicação parcial (`f.with(...)`): funde os args pré-ligados
        // com os da chamada final e delega recursivamente — o encadeamento
        // (`f.with(a).with(b)`) resolve-se sozinho, sem lógica extra.
        FuncRepr::With(w) => {
            let (inner, pre) = w.as_ref();
            apply_func(inner.clone(), merge_with_args(pre, args), scopes, ctx, engine)
        }
        FuncRepr::Native(native) | FuncRepr::Format(_, native) => {
            let world = engine.world;
            let current_file = engine.current_file;
            // F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9): o dispatch não lê mais
            // `custom("figure.numbering")` para alimentar a native — o padrão vive
            // só na chain e é lido pelo **consumidor** (layout/introspect). Fonte
            // única.
            (native.call)(ctx, &args, world, current_file)
        }
        // P394: native function com acesso ao scope/engine actuais (eval).
        FuncRepr::NativeWithEngine(native) => {
            let world = engine.world;
            let current_file = engine.current_file;
            (native.call)(ctx, &args, world, current_file, scopes, engine)
        }
    };
    ctx.replace_context_read_styles(previous_styles);
    ctx.replace_context_read_file(previous_file);
    ctx.replace_context_read_rules(previous_rules.0, previous_rules.1);
    result
}
