//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch.md
//! @prompt-hash 1ddee996
//! @layer L1
//! @updated 2026-09-01
//!
//! Dispatch de chamadas de função: avaliação de args, intercepção de method
//! calls especiais, aplicação de nativas/plugins/elementos. Extraído de
//! `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização —
//! forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::{
    extract_measure_body,
    native_bytes,
    // P737 — counter/state chamáveis via despacho de tipos.
    native_counter,
    native_datetime,
    native_decimal,
    native_duration,
    native_float,
    native_int,
    native_label,
    native_layout,
    native_measure,
    native_path,
    native_regex,
    native_selector,
    native_state,
    native_str,
    native_stroke,
    native_symbol,
    native_tiling,
    native_type,
    native_version,
    try_dispatch_collection_method,
    CollectionCallSpans,
};
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::ast::expr::{Arg, Expr, FuncCall as FuncCallNode};
use crate::entities::ast::AstNode;
use crate::entities::bytes::Bytes;
use crate::entities::engine::Engine;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::plugin_func::PluginFunc;
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::source_result::Tracepoint;
use crate::entities::span::{Span, Spanned};
use crate::entities::value::{Type, Value};

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;

use super::{bindings, closures, eval_expr, rules, EvalContext};

/// Metadados sintáticos privados e pontuais para os diagnósticos de
/// `float.is-nan`. Não atravessam `entities::Args` nem sobrevivem à chamada.
struct FloatIsNanCallSpans {
    call: Span,
    positional: Vec<Span>,
    named: IndexMap<EcoString, Span, FxBuildHasher>,
    has_spread: bool,
}

impl FloatIsNanCallSpans {
    fn capture(call: FuncCallNode<'_>) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;

        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(named_arg.name().as_str().into(), named_arg.span());
                }
                Arg::Spread(_) => has_spread = true,
            }
        }

        Self { call: call.span(), positional, named, has_spread }
    }

    fn anchor(&self, args: &Args, bound: bool) -> Span {
        if let Some((name, _)) = args.named.first() {
            return self.named.get(name.as_str()).copied().unwrap_or(args.span);
        }

        if bound {
            if args.items.is_empty() || self.has_spread {
                args.span
            } else {
                self.positional.first().copied().unwrap_or(args.span)
            }
        } else {
            match args.items.len() {
                0 => self.call,
                1 if !self.has_spread => {
                    self.positional.first().copied().unwrap_or(args.span)
                }
                2.. if !self.has_spread => {
                    self.positional.get(1).copied().unwrap_or(args.span)
                }
                _ => args.span,
            }
        }
    }
}

/// Identidades nativas P1293-B que exigem âncoras por argumento. A seleção é
/// feita sobre o `FuncRepr` já resolvido, nunca sobre texto do callee.
#[derive(Clone, Copy)]
enum P1293MathNative {
    Attach,
    Binom,
    Mono,
    Script,
}

impl P1293MathNative {
    fn from_func(func: &Func) -> Option<Self> {
        let FuncRepr::Native(native) = func.repr() else {
            return None;
        };
        match native.name {
            "attach" => Some(Self::Attach),
            "binom" => Some(Self::Binom),
            "mono" => Some(Self::Mono),
            "script" => Some(Self::Script),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct P1293NamedSpan {
    full: Span,
    value: Span,
}

/// Metadados sintáticos privados e transitórios para os quatro constructors
/// math P1293-B. Não atravessam `Args` nem sobrevivem à chamada.
struct P1293MathCallSpans {
    call: Span,
    positional: Vec<Span>,
    named: IndexMap<EcoString, P1293NamedSpan, FxBuildHasher>,
    has_spread: bool,
}

impl P1293MathCallSpans {
    fn capture(call: FuncCallNode<'_>) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;

        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(
                        named_arg.name().as_str().into(),
                        P1293NamedSpan {
                            full: named_arg.span(),
                            value: named_arg.expr().span(),
                        },
                    );
                }
                Arg::Spread(_) => has_spread = true,
            }
        }

        Self { call: call.span(), positional, named, has_spread }
    }

    fn positional(&self, index: usize, fallback: Span) -> Span {
        self.positional.get(index).copied().unwrap_or(fallback)
    }

    fn named_full(&self, name: &str, fallback: Span) -> Span {
        self.named.get(name).map(|span| span.full).unwrap_or(fallback)
    }

    fn named_value(&self, name: &str, fallback: Span) -> Span {
        self.named.get(name).map(|span| span.value).unwrap_or(fallback)
    }

    fn structural_content(value: &Value) -> bool {
        matches!(
            value,
            Value::Content(_)
                | Value::LocatedContent(_, _)
                | Value::Str(_)
                | Value::Symbol(_)
        )
    }

    fn style_content(value: &Value) -> bool {
        matches!(value, Value::Content(_) | Value::Str(_))
    }

    fn anchor(&self, identity: P1293MathNative, args: &Args) -> Span {
        if self.has_spread {
            return args.span;
        }

        match identity {
            P1293MathNative::Attach => {
                match args.items.len() {
                    0 => return self.call,
                    2.. => return self.positional(1, args.span),
                    _ => {}
                }
                if !Self::structural_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
                for (name, value) in &args.named {
                    if matches!(name.as_str(), "t" | "b" | "tl" | "bl" | "tr" | "br") {
                        if !matches!(value, Value::None)
                            && !Self::structural_content(value)
                        {
                            return self.named_value(name, args.span);
                        }
                    } else {
                        return self.named_full(name, args.span);
                    }
                }
            }
            P1293MathNative::Binom => {
                match args.items.len() {
                    0 => {
                        return args
                            .named
                            .get_key_value("upper")
                            .map(|(name, _)| self.named_full(name, args.span))
                            .unwrap_or(self.call);
                    }
                    1 => return self.call,
                    _ => {}
                }
                if let Some((name, _)) = args.named.first() {
                    return self.named_full(name, args.span);
                }
                for (index, value) in args.items.iter().enumerate() {
                    if !Self::structural_content(value) {
                        return self.positional(index, args.span);
                    }
                }
            }
            P1293MathNative::Mono => {
                if let Some((name, _)) = args.named.first() {
                    return self.named_full(name, args.span);
                }
                match args.items.len() {
                    0 => return self.call,
                    2.. => return self.positional(1, args.span),
                    _ => {}
                }
                if !Self::style_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
            }
            P1293MathNative::Script => {
                for (name, _) in &args.named {
                    if name.as_str() != "cramped" {
                        return self.named_full(name, args.span);
                    }
                }
                if let Some(value) = args.named.get("cramped") {
                    if !matches!(value, Value::Bool(_)) {
                        return self.named_value("cramped", args.span);
                    }
                }
                match args.items.len() {
                    0 => return self.call,
                    2.. => return self.positional(1, args.span),
                    _ => {}
                }
                if !Self::style_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
            }
        }

        args.span
    }
}

#[derive(Clone, Copy)]
enum P1293HtmlNative {
    Normal,
    Void,
}

impl P1293HtmlNative {
    fn from_func(func: &Func) -> Option<Self> {
        let name = match func.repr() {
            FuncRepr::Native(native) => native.name,
            _ => return None,
        };
        let kind = match name {
            "button" | "iframe" | "select" | "template" | "video" => Self::Normal,
            "col" | "wbr" => Self::Void,
            _ => return None,
        };
        let module = crate::compiler::stdlib::make_html_module();
        let Value::Func(reference) = module.scope().get(name)? else {
            return None;
        };
        let (Some(actual), Some(expected)) =
            (func.native_fn_addr(), reference.native_fn_addr())
        else {
            return None;
        };
        std::ptr::fn_addr_eq(actual, expected).then_some(kind)
    }
}

struct P1293HtmlCallSpans {
    kind: P1293HtmlNative,
    positional: Vec<Span>,
    named: IndexMap<EcoString, P1293NamedSpan, FxBuildHasher>,
    has_spread: bool,
}

impl P1293HtmlCallSpans {
    fn capture(call: FuncCallNode<'_>, kind: P1293HtmlNative) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;
        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(
                        named_arg.name().as_str().into(),
                        P1293NamedSpan {
                            full: named_arg.span(),
                            value: named_arg.expr().span(),
                        },
                    );
                }
                Arg::Spread(_) => has_spread = true,
            }
        }
        Self { kind, positional, named, has_spread }
    }

    fn anchor(&self, args: &Args, message: &str) -> Span {
        if self.has_spread {
            return args.span;
        }
        match self.kind {
            P1293HtmlNative::Void if !args.items.is_empty() => {
                return self.positional.first().copied().unwrap_or(args.span)
            }
            P1293HtmlNative::Normal if args.items.len() > 1 => {
                return self.positional.get(1).copied().unwrap_or(args.span)
            }
            P1293HtmlNative::Normal
                if args
                    .items
                    .first()
                    .is_some_and(|value| !matches!(value, Value::Content(_))) =>
            {
                return self.positional.first().copied().unwrap_or(args.span)
            }
            _ => {}
        }
        if let Some(name) = message.strip_prefix("unexpected argument: ") {
            return self.named.get(name).map(|span| span.full).unwrap_or(args.span);
        }
        self.named.first().map(|(_, span)| span.value).unwrap_or(args.span)
    }
}

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. **P718** — `..spread` mirror de `ast::Args::eval` (vanilla
/// `call.rs:367-416`): `none` ignorado; `array` vira posicionais; `dict`
/// vira nomeados; `Value::Args` (reencaminhamento de um sink `..rest` de
/// outra closure) funde posicionais e nomeados; outro tipo erra `cannot
/// spread {ty}` — **sem** o sufixo "into X" das mensagens de literal
/// (`Expr::Array`/`Expr::Dict`, `mod.rs`), mensagem distinta no vanilla.
pub(super) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args> {
    // P772s — span da lista de argumentos da chamada real no documento,
    // propagado para todas as mensagens de erro de validação de argumento
    // das funções nativas que usam `args.span` em vez de `Span::detached()`.
    let call_span = args_node.span();
    let mut occurrences = Vec::new();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(expr) => occurrences.push(ArgOccurrence {
                name: None,
                value: eval_expr(expr, scopes, ctx, engine)?,
                span: expr.span(),
                value_span: expr.span(),
            }),
            Arg::Named(name_expr) => {
                occurrences.push(ArgOccurrence {
                    name: Some(name_expr.name().as_str().into()),
                    value: eval_expr(name_expr.expr(), scopes, ctx, engine)?,
                    span: name_expr.span(),
                    value_span: name_expr.expr().span(),
                });
            }
            Arg::Spread(spread) => {
                let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                match value {
                    Value::None => {}
                    Value::Array(arr) => {
                        occurrences.extend(arr.into_iter().map(|value| ArgOccurrence {
                            name: None,
                            value,
                            span: spread.span(),
                            value_span: spread.span(),
                        }))
                    }
                    Value::Dict(dict) => {
                        occurrences.extend(dict.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }))
                    }
                    Value::Args(args) => {
                        occurrences.extend(args.occurrence_sequence());
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", vanilla_type_name(&other)),
                        )]);
                    }
                }
            }
        }
    }
    Ok(Args::from_occurrences(call_span, occurrences))
}

/// Transporta a chamada inteira para nativas cujo erro usa o agregado.
pub(super) fn transport_native_call_span(func: &Func, args: &mut Args, call_span: Span) {
    use crate::compiler::stdlib::{
        calc_abs, native_csv, native_json_encode, native_panic, native_toml_encode,
        native_yaml_encode,
    };
    match func.repr() {
        FuncRepr::With(with) => transport_native_call_span(&with.0, args, call_span),
        FuncRepr::Native(native)
            if std::ptr::fn_addr_eq(
                native.call,
                native_json_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_toml_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_yaml_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_panic as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_csv as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                calc_abs as fn(_, _, _, _) -> _,
            ) =>
        {
            args.span = call_span;
        }
        _ => {}
    }
}

/// Aplica uma função (closure, native ou native-with-engine) aos args dados.
pub fn apply_func(
    func: Func,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => {
            closures::apply_closure(closure, &func, args, ctx, engine)
        }
        // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — `#name(args)`
        // invoca o construtor do registry e devolve `Content::Dynamic`. Mesmo
        // ponto de despacho dos nativos (sem caminho paralelo). Erro do catálogo
        // existente se o ctor falhar (não panic).
        FuncRepr::Element(ef) => Ok(Value::Content((ef.ctor)(&args.items)?)),
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
        FuncRepr::Native(native) => {
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
    }
}

/// **P846 (#57)** — envolve o resultado de uma chamada com um
/// `Tracepoint::Call`, mirror de `call_func` + `Trace::trace` do vanilla
/// (`typst-eval/src/call.rs:166-180`, `typst-library/src/diag.rs:464-479`).
///
/// Regras verbatim do vanilla:
/// - Se o span da chamada não resolve para um byte range (detached ou
///   ficheiro inacessível), os erros propagam inalterados.
/// - Um erro **contido** no span da chamada (mesma fonte, range da chamada ⊇
///   range do erro) não ganha tracepoint — é o caso dos erros de validação
///   de argumentos de nativas e dos erros de `eval` (âncora = literal dentro
///   da chamada). Erros no **corpo** de closures ficam fora do span da
///   chamada → ganham um nível por chamada, innermost primeiro.
/// - `func.name()` → `while calling \`name\``; `None` (closure anónima) →
///   `while calling function` (Display espelhado no renderer L2).
pub(crate) fn p1284_type_field(t: Type, field: &str) -> Option<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};

    let constant = match (t, field) {
        (Type::Direction, "ltr") => Some(Value::Dir(Dir::LTR)),
        (Type::Direction, "rtl") => Some(Value::Dir(Dir::RTL)),
        (Type::Direction, "ttb") => Some(Value::Dir(Dir::TTB)),
        (Type::Direction, "btt") => Some(Value::Dir(Dir::BTT)),
        (Type::Alignment, "left") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Left), v: None }))
        }
        (Type::Alignment, "center") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Center), v: None }))
        }
        (Type::Alignment, "right") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Right), v: None }))
        }
        (Type::Alignment, "start") => {
            Some(Value::Align(Align2D { h: Some(HAlign::Start), v: None }))
        }
        (Type::Alignment, "end") => {
            Some(Value::Align(Align2D { h: Some(HAlign::End), v: None }))
        }
        (Type::Alignment, "top") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Top) }))
        }
        (Type::Alignment, "horizon") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }))
        }
        (Type::Alignment, "bottom") => {
            Some(Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }))
        }
        _ => None,
    };
    if constant.is_some() {
        return constant;
    }

    let function = match (t, field) {
        (Type::Direction, "axis") => {
            Func::native_with_engine("axis", direction_axis_static)
        }
        (Type::Direction, "end") => Func::native_with_engine("end", direction_end_static),
        (Type::Direction, "inv") => Func::native_with_engine("inv", direction_inv_static),
        (Type::Direction, "sign") => {
            Func::native_with_engine("sign", direction_sign_static)
        }
        (Type::Direction, "start") => {
            Func::native_with_engine("start", direction_start_static)
        }
        (Type::Direction, "from") => Func::native_with_engine("from", direction_from),
        (Type::Direction, "to") => Func::native_with_engine("to", direction_to),
        (Type::Alignment, "axis") => {
            Func::native_with_engine("axis", alignment_axis_static)
        }
        (Type::Alignment, "inv") => Func::native_with_engine("inv", alignment_inv_static),
        (Type::Duration, "days") => {
            Func::native_with_engine("days", duration_days_static)
        }
        (Type::Duration, "hours") => {
            Func::native_with_engine("hours", duration_hours_static)
        }
        (Type::Duration, "minutes") => {
            Func::native_with_engine("minutes", duration_minutes_static)
        }
        (Type::Duration, "seconds") => {
            Func::native_with_engine("seconds", duration_seconds_static)
        }
        (Type::Duration, "weeks") => {
            Func::native_with_engine("weeks", duration_weeks_static)
        }
        (Type::Length, "pt") => Func::native_with_engine("pt", length_pt_static),
        (Type::Length, "mm") => Func::native_with_engine("mm", length_mm_static),
        (Type::Length, "cm") => Func::native_with_engine("cm", length_cm_static),
        (Type::Length, "inches") => {
            Func::native_with_engine("inches", length_inches_static)
        }
        (Type::Length, "to-absolute") => {
            Func::native_with_engine("to-absolute", length_to_absolute_static)
        }
        (Type::Selector, "and") => Func::native_with_engine("and", selector_and_static),
        (Type::Selector, "or") => Func::native_with_engine("or", selector_or_static),
        (Type::Selector, "within") => {
            Func::native_with_engine("within", selector_within_static)
        }
        (Type::State, "at") => Func::native_with_engine("at", state_at_static),
        (Type::State, "final") => Func::native_with_engine("final", state_final_static),
        (Type::State, "get") => Func::native_with_engine("get", state_get_static),
        (Type::State, "update") => {
            Func::native_with_engine("update", state_update_static)
        }
        (Type::Location, "page") => {
            Func::native_with_engine("page", location_page_static)
        }
        (Type::Location, "page-numbering") => {
            Func::native_with_engine("page-numbering", location_page_numbering_static)
        }
        (Type::Location, "position") => {
            Func::native_with_engine("position", location_position_static)
        }
        _ => return None,
    };
    Some(Value::Func(function))
}

fn p1284_no_args(args: &Args) -> SourceResult<()> {
    if !args.items.is_empty() || !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    Ok(())
}

fn p1284_static_call(
    method: &str,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let Some(target) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")]);
    };
    let mut rest = args.clone();
    rest.remove_positional(0);
    dispatch_p1284_value_method(target.clone(), method, rest, scopes, ctx, engine)
}

macro_rules! p1284_static {
    ($rust:ident, $method:literal) => {
        fn $rust(
            ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _file: crate::entities::file_id::FileId,
            scopes: &mut Scopes<'_>,
            engine: &mut Engine<'_>,
        ) -> SourceResult<Value> {
            p1284_static_call($method, args, scopes, ctx, engine)
        }
    };
}

p1284_static!(direction_axis_static, "axis");
p1284_static!(direction_end_static, "end");
p1284_static!(direction_inv_static, "inv");
p1284_static!(direction_sign_static, "sign");
p1284_static!(direction_start_static, "start");
p1284_static!(alignment_axis_static, "axis");
p1284_static!(alignment_inv_static, "inv");
p1284_static!(duration_days_static, "days");
p1284_static!(duration_hours_static, "hours");
p1284_static!(duration_minutes_static, "minutes");
p1284_static!(duration_seconds_static, "seconds");
p1284_static!(duration_weeks_static, "weeks");
p1284_static!(length_pt_static, "pt");
p1284_static!(length_mm_static, "mm");
p1284_static!(length_cm_static, "cm");
p1284_static!(length_inches_static, "inches");
p1284_static!(length_to_absolute_static, "to-absolute");
p1284_static!(selector_and_static, "and");
p1284_static!(selector_or_static, "or");
p1284_static!(selector_within_static, "within");
p1284_static!(state_at_static, "at");
p1284_static!(state_final_static, "final");
p1284_static!(state_get_static, "get");
p1284_static!(state_update_static, "update");
p1284_static!(location_page_static, "page");
p1284_static!(location_page_numbering_static, "page-numbering");
p1284_static!(location_position_static, "position");

fn direction_from(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
    _scopes: &mut Scopes<'_>,
    _engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    direction_from_to(args, false)
}

fn direction_to(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
    _scopes: &mut Scopes<'_>,
    _engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    direction_from_to(args, true)
}

fn direction_from_to(args: &Args, to: bool) -> SourceResult<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{HAlign, VAlign};
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "expected one side")]);
    }
    let direction = match &args.items[0] {
        Value::Align(align) => match (align.h, align.v, to) {
            (Some(HAlign::Left), None, false) => Dir::LTR,
            (Some(HAlign::Right), None, false) => Dir::RTL,
            (None, Some(VAlign::Top), false) => Dir::TTB,
            (None, Some(VAlign::Bottom), false) => Dir::BTT,
            (Some(HAlign::Right), None, true) => Dir::LTR,
            (Some(HAlign::Left), None, true) => Dir::RTL,
            (None, Some(VAlign::Bottom), true) => Dir::TTB,
            (None, Some(VAlign::Top), true) => Dir::BTT,
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "expected side alignment",
                )])
            }
        },
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected alignment, found {}", vanilla_type_name(other)),
            )])
        }
    };
    Ok(Value::Dir(direction))
}

fn dispatch_p1284_value_method(
    target: Value,
    method: &str,
    args: Args,
    _scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{Abs, Align2D, HAlign, Length, VAlign};

    let inverse_h = |h| match h {
        HAlign::Left => HAlign::Right,
        HAlign::Right => HAlign::Left,
        HAlign::Start => HAlign::End,
        HAlign::End => HAlign::Start,
        HAlign::Center => HAlign::Center,
    };
    let inverse_v = |v| match v {
        VAlign::Top => VAlign::Bottom,
        VAlign::Bottom => VAlign::Top,
        VAlign::Horizon => VAlign::Horizon,
    };

    match target {
        Value::Dir(direction) => {
            p1284_no_args(&args)?;
            Ok(match method {
                "axis" => Value::Str(
                    if direction.is_horizontal() { "horizontal" } else { "vertical" }
                        .into(),
                ),
                "sign" => Value::Int(if direction.is_reverse() { -1 } else { 1 }),
                "inv" => Value::Dir(match direction {
                    Dir::LTR => Dir::RTL,
                    Dir::RTL => Dir::LTR,
                    Dir::TTB => Dir::BTT,
                    Dir::BTT => Dir::TTB,
                }),
                "start" | "end" => {
                    let start = method == "start";
                    let alignment = match (direction, start) {
                        (Dir::LTR, true) | (Dir::RTL, false) => {
                            Align2D { h: Some(HAlign::Left), v: None }
                        }
                        (Dir::LTR, false) | (Dir::RTL, true) => {
                            Align2D { h: Some(HAlign::Right), v: None }
                        }
                        (Dir::TTB, true) | (Dir::BTT, false) => {
                            Align2D { h: None, v: Some(VAlign::Top) }
                        }
                        (Dir::TTB, false) | (Dir::BTT, true) => {
                            Align2D { h: None, v: Some(VAlign::Bottom) }
                        }
                    };
                    Value::Align(alignment)
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )])
                }
            })
        }
        Value::Align(alignment) => {
            p1284_no_args(&args)?;
            Ok(match method {
                "axis" => match (alignment.h, alignment.v) {
                    (Some(_), None) => Value::Str("horizontal".into()),
                    (None, Some(_)) => Value::Str("vertical".into()),
                    _ => Value::None,
                },
                "inv" => Value::Align(Align2D {
                    h: alignment.h.map(inverse_h),
                    v: alignment.v.map(inverse_v),
                }),
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )])
                }
            })
        }
        Value::Duration(duration) => {
            p1284_no_args(&args)?;
            let seconds = duration.nanos as f64 / 1_000_000_000.0;
            Ok(Value::Float(match method {
                "seconds" => seconds,
                "minutes" => seconds / 60.0,
                "hours" => seconds / 3_600.0,
                "days" => seconds / 86_400.0,
                "weeks" => seconds / 604_800.0,
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )])
                }
            }))
        }
        Value::Length(length) => {
            p1284_no_args(&args)?;
            if method == "to-absolute" {
                if !ctx.in_context {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "can only be used when context is known",
                    )]);
                }
                let points = length.abs.to_pt() + length.em * engine.styles.size();
                return Ok(Value::Length(Length { abs: Abs(points), em: 0.0 }));
            }
            if length.em != 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "relative length cannot be converted without context",
                )]);
            }
            let points = length.abs.to_pt();
            Ok(Value::Float(match method {
                "pt" => points,
                "mm" => points * 25.4 / 72.0,
                "cm" => points * 2.54 / 72.0,
                "inches" => points / 72.0,
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "unknown method",
                    )])
                }
            }))
        }
        Value::Selector(base) => {
            if !args.named.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "unexpected argument",
                )]);
            }
            match method {
                "and" | "or" => {
                    let mut selectors = vec![base];
                    for value in &args.items {
                        let selector = bindings::value_to_query_selector(value)
                            .ok_or_else(|| {
                                vec![SourceDiagnostic::error(
                                    args.span,
                                    format!(
                                        "expected selector, found {}",
                                        value.type_name()
                                    ),
                                )]
                            })?;
                        selectors.push(selector);
                    }
                    Ok(Value::Selector(if method == "and" {
                        crate::entities::selector::Selector::And(ecow::EcoVec::from(
                            selectors,
                        ))
                    } else {
                        crate::entities::selector::Selector::Or(ecow::EcoVec::from(
                            selectors,
                        ))
                    }))
                }
                "within" => {
                    let [ancestor] = args.items.as_slice() else {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "within requires one ancestor",
                        )]);
                    };
                    let ancestor = bindings::value_to_query_selector(ancestor)
                        .ok_or_else(|| {
                            vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "expected selector, found {}",
                                    ancestor.type_name()
                                ),
                            )]
                        })?;
                    Ok(Value::Selector(crate::entities::selector::Selector::Within {
                        base: Box::new(base),
                        ancestor: Box::new(ancestor),
                    }))
                }
                _ => Err(vec![SourceDiagnostic::error(args.span, "unknown method")]),
            }
        }
        Value::State(state) => {
            use crate::compiler::stdlib::state::{
                state_at_location, state_final, state_get, state_update,
            };
            match method {
                "update" => match args.items.as_slice() {
                    [value] if args.named.is_empty() => {
                        Ok(state_update(state.key.clone(), value.clone()))
                    }
                    _ => Err(vec![SourceDiagnostic::error(
                        args.span,
                        "state.update() requires one argument",
                    )]),
                },
                "get" => {
                    p1284_no_args(&args)?;
                    state_get(&state, ctx, args.span)
                }
                "final" => {
                    p1284_no_args(&args)?;
                    state_final(&state, ctx, args.span)
                }
                "at" => match args.items.as_slice() {
                    [Value::Location(location)] if args.named.is_empty() => {
                        state_at_location(&state, *location, ctx, args.span)
                    }
                    [other] => Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected location, found {}", other.type_name()),
                    )]),
                    _ => Err(vec![SourceDiagnostic::error(
                        args.span,
                        "state.at() requires one selector",
                    )]),
                },
                _ => Err(vec![SourceDiagnostic::error(args.span, "unknown method")]),
            }
        }
        Value::Location(location) => {
            p1284_no_args(&args)?;
            eval_location_method(location, method, ctx)
        }
        other => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("type {} has no method `{method}`", other.type_name()),
        )]),
    }
}

fn trace_call(
    result: SourceResult<Value>,
    name: Option<&str>,
    call_span: Span,
    engine: &Engine<'_>,
) -> SourceResult<Value> {
    let mut errors = match result {
        Ok(value) => return Ok(value),
        Err(errors) => errors,
    };
    // vanilla: `let Some(trace_range) = world.range(span) else { return errors }`.
    let trace_range = call_span
        .id()
        .and_then(|id| engine.world.source(id).ok())
        .and_then(|src| src.span_byte_range(call_span));
    let Some(trace_range) = trace_range else {
        return Err(errors);
    };
    for error in &mut errors {
        // "Skip traces that surround the error" (vanilla `diag.rs:471-478`):
        // mesmo ficheiro e chamada contém o erro → não acrescenta nível.
        if error.span.id() == call_span.id() {
            let contained = call_span
                .id()
                .and_then(|id| engine.world.source(id).ok())
                .and_then(|src| src.span_byte_range(error.span))
                .is_some_and(|error_range| {
                    trace_range.start <= error_range.start
                        && trace_range.end >= error_range.end
                });
            if contained {
                continue;
            }
        }
        error
            .trace
            .push(Spanned::new(Tracepoint::Call(name.map(String::from)), call_span));
    }
    Err(errors)
}

/// **P702** — funde os `Args` pré-ligados por `.with(...)` com os da chamada
/// final. Posicionais: pré-ligados primeiro (paridade vanilla,
/// `foundations/func.rs:360` do vanilla: `pre.items.chain(new.items)`).
/// Todas as ocorrências nomeadas são preservadas; a view projeta a última.
fn merge_with_args(pre: &Args, new: Args) -> Args {
    // P772s — span da chamada final (mais próxima do erro visto pelo
    // utilizador do que o span da chamada de `.with(...)` original).
    let span = new.span;
    let mut occurrences = pre.occurrence_sequence();
    occurrences.extend(new.occurrence_sequence());
    Args::from_occurrences(span, occurrences)
}

/// **P699** — Aplica um export de plugin WASM (`FuncRepr::Plugin`).
///
/// - Rejeita named args (a ABI WASM é posicional).
/// - Exige que todos os args posicionais sejam `Value::Bytes`; reúne-os num
///   `Vec<Bytes>` (clone de handle, não de conteúdo).
/// - Delega a `PluginFunc::call` (memoizada): `Ok(bytes) ⇒ Value::Bytes`,
///   `Err(e) ⇒ SourceDiagnostic` com `e.message` verbatim (observável).
/// - **P819** — mensagem verbatim do vanilla (`expected bytes, found <tipo>`,
///   nome longo) e span do callsite (`args.span`, P772s) em vez de detached.
fn call_plugin(p: &PluginFunc, args: &Args) -> SourceResult<Value> {
    if let Some(k) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("argumento nomeado inesperado em {}(): '{k}'", p.name),
        )]);
    }

    let mut bufs: Vec<Bytes> = Vec::with_capacity(args.items.len());
    for v in &args.items {
        match v {
            Value::Bytes(b) => bufs.push(b.clone()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected bytes, found {}", vanilla_type_name(other),),
                )]);
            }
        }
    }

    match p.call(bufs) {
        Ok(bytes) => Ok(Value::Bytes(bytes)),
        Err(e) => Err(vec![SourceDiagnostic::error(args.span, e.message.to_string())]),
    }
}

pub(super) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // **P417 (M)** — Intercepção de `heading.where(field: value)` (method call
    // syntax) antes de avaliar o callee genérico. O target deve avaliar para
    // uma `Value::Func` nativa de elemento (heading, figure, strong, emph, raw).
    // O resultado é `Value::Selector(Selector::Where { base: Kind(...), ... })`.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "where" {
            if let Some(selector) = bindings::eval_element_where(
                access.target(),
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P423 (S-M)** — Intercepção de `selector.or(other)` e
    // `selector.and(other)` antes de avaliar o callee genérico. O target e o
    // argumento devem avaliar para `Value::Selector`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if method == "or" || method == "and" {
            if let Some(selector) = bindings::eval_selector_or_and(
                access.target(),
                method,
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P504** — Intercepção de `selector.within(ancestor)`.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "within" {
            if let Some(selector) = bindings::eval_selector_within(
                access.target(),
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Value::Selector(selector));
            }
        }
    }

    // **P717** — Métodos mutantes (`push`/`pop`/`insert`/`remove`): mirror
    // de `maybe_resolve_mutating` (vanilla `call.rs:189-212`), sobre o
    // `access()` de P716. Tem de correr **antes** do bloco P466, que avalia
    // o target como valor (clone) — mutação exige o local. `Ok(None)` =
    // fall-through para a cadeia normal (módulos/funcs com campos com estes
    // nomes continuam a resolver abaixo).
    if let Expr::FieldAccess(access) = call.callee() {
        if bindings::is_mutating_method(access.field().as_str()) {
            if let Some(result) = bindings::try_eval_mutating_method(
                access,
                call.args(),
                call.span(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(result);
            }
        }
    }

    // **P506** — Métodos de instância para `state` e `counter`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        let method = access.field().as_str();
        match target {
            Value::State(ref state) => {
                return super::bindings::eval_state_method(
                    state,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                )
            }
            Value::Counter(ref counter) => {
                return super::bindings::eval_counter_method_value(
                    counter,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                )
            }
            Value::Type(crate::entities::value::Type::Counter)
                if matches!(method, "at" | "display") =>
            {
                return super::bindings::eval_counter_static_method_value(
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                )
            }
            // **P742** — Métodos de instância de `Value::Color` (9, padrão
            // P506). Só intercepta os métodos conhecidos; os restantes caem
            // no caminho genérico (erro de field access pré-P742).
            Value::Color(ref color) => {
                if crate::compiler::stdlib::color::is_color_instance_method(method) {
                    return super::bindings::eval_color_method(
                        color,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
            }
            Value::Gradient(ref gradient) => {
                if crate::compiler::stdlib::gradients::is_gradient_instance_method(method)
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return crate::compiler::stdlib::gradients::dispatch_gradient_method(
                        gradient,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    );
                }
            }
            Value::Int(value) => {
                if crate::compiler::stdlib::is_int_instance_method(method) {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return crate::compiler::stdlib::dispatch_int_method(
                        value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    );
                }
            }
            Value::Float(value) => {
                if crate::compiler::stdlib::is_float_instance_method(method) {
                    let spans =
                        (method == "is-nan").then(|| FloatIsNanCallSpans::capture(call));
                    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                    if let Some(spans) = spans {
                        args.span = spans.anchor(&args, true);
                    }
                    return crate::compiler::stdlib::dispatch_float_method(
                        value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    );
                }
            }
            value @ Value::Dir(_)
                if matches!(method, "axis" | "end" | "inv" | "sign" | "start") =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return dispatch_p1284_value_method(
                    value, method, args, scopes, ctx, engine,
                );
            }
            value @ Value::Align(_) if matches!(method, "axis" | "inv") => {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return dispatch_p1284_value_method(
                    value, method, args, scopes, ctx, engine,
                );
            }
            value @ Value::Duration(_)
                if matches!(
                    method,
                    "days" | "hours" | "minutes" | "seconds" | "weeks"
                ) =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return dispatch_p1284_value_method(
                    value, method, args, scopes, ctx, engine,
                );
            }
            value @ Value::Length(_)
                if matches!(method, "cm" | "inches" | "mm" | "pt" | "to-absolute") =>
            {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return dispatch_p1284_value_method(
                    value, method, args, scopes, ctx, engine,
                );
            }
            // **P796** — `.at(index)` em `Value::Version` (único método de
            // instância; ver `entities/version.md` §8a). Só intercepta
            // "at"; outros métodos caem no caminho genérico existente.
            Value::Version(ref v) => {
                if method == "at" {
                    return super::bindings::eval_version_method_value(
                        v,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    );
                }
            }
            _ => {}
        }
    }

    // **P466** — Métodos de instância para `array`, `dict` e `str`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        let method = access.field().as_str();
        let trace_arguments_callback =
            matches!(&target, Value::Args(_)) && matches!(method, "filter" | "map");
        let mut positional = Vec::new();
        let mut named: IndexMap<EcoString, Span, FxBuildHasher> = IndexMap::default();
        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(named_arg.name().as_str().into(), named_arg.span());
                }
                Arg::Spread(_) => {}
            }
        }
        let call_spans = CollectionCallSpans { call: call.span(), positional, named };
        let args = eval_args(call.args(), scopes, ctx, engine)?;
        if let Some(result) = try_dispatch_collection_method(
            target,
            method,
            args,
            Some(&call_spans),
            scopes,
            ctx,
            engine,
        ) {
            return if trace_arguments_callback {
                trace_call(result, Some(method), call.span(), engine)
            } else {
                result
            };
        }
    }

    // **P702** — `f.with(...)`: aplicação parcial de argumentos, disponível
    // em qualquer `Value::Func` (nativa com/sem namespace, closure, elemento,
    // plugin, ou já parcialmente aplicada). Mesmo padrão de intercepção das
    // secções acima; se o alvo não for `Value::Func`, não intercepta — cai
    // no field access genérico, sem mudança de comportamento para
    // não-funções (ver `entities/func.md` §"Variante `With`").
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "with" {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Func(f) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return Ok(Value::Func(f.with(args)));
            }
        }
    }

    // **P712** — `measure(body)` / `std.measure(body)`: precisa de
    // `engine.styles` (o tamanho medido depende do `#set text(size:)`
    // activo, paridade com o vanilla `context.styles()`) e do gate
    // `ctx.in_context` (mesma convenção já usada por `counter.get()`/
    // `state.get()`, `stdlib/counter.rs:144`/`stdlib/state.rs:63`) —
    // nenhum dos dois acessível pela assinatura genérica
    // `NativeFn(ctx, args, world, file)`. Verifica primeiro a forma
    // sintáctica do callee (barato, sem side-effects) para não avaliar
    // nada quando não é sequer chamado "measure"; só depois avalia e
    // compara identidade de fn-ptr (`native_fn_addr`, não o nome — mesmo
    // padrão de `bindings::eval_element_where`, `bindings.rs:354`) para
    // não capturar um `measure` sombreado pelo utilizador.
    let measure_name_matches = match call.callee() {
        Expr::Ident(ident) => ident.as_str() == "measure",
        Expr::FieldAccess(access) => access.field().as_str() == "measure",
        _ => false,
    };
    if measure_name_matches {
        let target = eval_expr(call.callee(), scopes, ctx, engine)?;
        if let Value::Func(ref f) = target {
            if f.native_fn_addr().is_some_and(|addr| {
                std::ptr::fn_addr_eq(addr, native_measure as fn(_, _, _, _) -> _)
            }) {
                if !ctx.in_context {
                    return Err(vec![SourceDiagnostic::error(
                        call.callee().span(),
                        "measure() can only be used inside context".to_string(),
                    )]);
                }
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                let body = extract_measure_body(&args)?;
                let (width_pt, height_pt) = crate::compiler::layout::measure_content_real(
                    &body,
                    engine.styles,
                    engine.font_metrics,
                );
                let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                dict.insert(
                    "width".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(width_pt)),
                );
                dict.insert(
                    "height".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(height_pt)),
                );
                return Ok(Value::Dict(dict));
            }
        }
    }

    // **P792** — `layout(func)`: paridade vanilla `layout/layout.rs:66`.
    // Chama a callback com as dimensões disponíveis (single-pass graded:
    // `available_width`/`available_height` calculadas a partir de
    // `engine.styles`). Mesmo padrão de intercepção que `measure`/P712:
    // verifica nome sintáctico → avalia o callee → compara fn-ptr
    // `native_layout` para não capturar um `layout` sombreado.
    let layout_name_matches = match call.callee() {
        Expr::Ident(ident) => ident.as_str() == "layout",
        Expr::FieldAccess(access) => access.field().as_str() == "layout",
        _ => false,
    };
    if layout_name_matches {
        let target = eval_expr(call.callee(), scopes, ctx, engine)?;
        if let Value::Func(ref f) = target {
            if f.native_fn_addr().is_some_and(|addr| {
                std::ptr::fn_addr_eq(addr, native_layout as fn(_, _, _, _) -> _)
            }) {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                // Extrair a callback — único argumento posicional obrigatório.
                let func = match args.items.as_slice() {
                    [Value::Func(f)] => f.clone(),
                    [other] => {
                        return Err(vec![SourceDiagnostic::error(
                            call.callee().span(),
                            format!(
                                "layout() requer uma função, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            call.callee().span(),
                            "layout() requer exatamente 1 argumento".to_string(),
                        )])
                    }
                };
                // Dimensões do container (single-pass graded — P792 scope-out two-pass).
                // Lidas dinamicamente da StyleChain caso configuradas por um `#set page`
                // anterior no escopo; caso contrário usa defaults da página A4.
                let page_width_pt = match engine.styles.custom("page.width") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => crate::entities::page_geometry::Paper::A4.width_pt(),
                };
                let page_height_pt = match engine.styles.custom("page.height") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => crate::entities::page_geometry::Paper::A4.height_pt(),
                };
                let margin_left = match engine.styles.custom("page.margin-left") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_right = match engine.styles.custom("page.margin-right") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_top = match engine.styles.custom("page.margin-top") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let margin_bottom = match engine.styles.custom("page.margin-bottom") {
                    Some(Value::Float(f)) => *f,
                    Some(Value::Int(i)) => *i as f64,
                    _ => 56.69f64,
                };
                let avail_w = f64::max(0.0, page_width_pt - margin_left - margin_right);
                let avail_h = f64::max(0.0, page_height_pt - margin_top - margin_bottom);
                let mut size_dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                size_dict.insert(
                    "width".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(avail_w)),
                );
                size_dict.insert(
                    "height".into(),
                    Value::Length(crate::entities::layout_types::Length::pt(avail_h)),
                );
                let size_arg = Args::from_parts(
                    vec![Value::Dict(size_dict)],
                    indexmap::IndexMap::default(),
                    call.span(),
                );
                let result = apply_func(func, size_arg, scopes, ctx, engine)?;
                return if let Value::Content(c) = result {
                    Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
                } else {
                    Ok(result)
                };
            }
        }
    }

    // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
    // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "page" | "position" | "page-numbering") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Location(loc) = target {
                let _args = eval_args(call.args(), scopes, ctx, engine)?;
                return eval_location_method(loc, method, ctx);
            }
        }
    }

    // **P829-B** — Métodos de instância de `Value::Content`:
    // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
    // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
    // todos os despachos legítimos acima (nenhum intercepta estes nomes para
    // Content) e antes do fallback P815, que de outra forma reportaria
    // `element strong has no method `func`` (medido m14/b1).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "func" | "has" | "at" | "fields" | "location") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Content(c) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_content_method(&c, method, args, call.span());
            }
            if let Value::LocatedContent(c, loc) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_introspected_content_method_at(
                    &c,
                    loc,
                    method,
                    args,
                    call.span(),
                );
            }
        }
    }

    // **P815** — `eval_field_callee` do vanilla (`call.rs:239-345`): depois
    // de todos os despachos de método legítimos acima, um callee
    // `target.field` chamado como função cujo alvo não é
    // Symbol/Func/Type/Module produz os erros verbatim do vanilla —
    // método inexistente (`type integer has no method `foo``), dict-key-call
    // com hints, "not a valid method". Sem isto, o caminho genérico avaliava
    // o field access e errava com mensagens divergentes (ou, pior, chamava
    // funções guardadas em dict keys — medido em P815).
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        if let Some(err) = bindings::field_callee_error(&target, access) {
            return Err(err);
        }
    }

    let callee = eval_expr(call.callee(), scopes, ctx, engine)?;

    let float_is_nan_spans = matches!(
        &callee,
        Value::Func(f)
            if matches!(f.repr(), FuncRepr::Native(n) if n.name == "is-nan")
    )
    .then(|| FloatIsNanCallSpans::capture(call));

    let p1293_math_spans = match &callee {
        Value::Func(func) => P1293MathNative::from_func(func)
            .map(|identity| (identity, P1293MathCallSpans::capture(call))),
        _ => None,
    };
    let p1293_html_spans = match &callee {
        Value::Func(func) => P1293HtmlNative::from_func(func)
            .map(|kind| P1293HtmlCallSpans::capture(call, kind)),
        _ => None,
    };

    // **P846 (#56)** — `eval`: ancorar os erros da re-avaliação ao span do
    // **literal string** (primeiro argumento posicional), paridade com o
    // `SpanMode::Uniform` do vanilla (`foundations/mod.rs:267,318` — medido:
    // `span7.typ` van `4:2` vs cris `3:5`). Solução pontual só para `eval`
    // (override de `args.span` no call site), sem o débito estrutural de
    // span-por-argumento em `Args` (P772s). O cast error (`#eval(5)` →
    // `1:6`) fica igualmente corrigido — o vanilla ancora erros de validação
    // de argumento no span do argumento.
    let eval_anchor = match &callee {
        Value::Func(f) if matches!(f.repr(), FuncRepr::NativeWithEngine(n) if n.name == "eval") => {
            call.args().items().find_map(|arg| match arg {
                Arg::Pos(expr) => Some(expr.span()),
                _ => None,
            })
        }
        _ => None,
    };

    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
    if let Some(spans) = float_is_nan_spans {
        args.span = spans.anchor(&args, false);
    } else if let Some((identity, spans)) = p1293_math_spans {
        args.span = spans.anchor(identity, &args);
    } else if let Some(anchor) = eval_anchor {
        args.span = anchor;
    }

    match callee {
        Value::Func(func) => {
            transport_native_call_span(&func, &mut args, call.span());
            let html_args = p1293_html_spans.as_ref().map(|_| args.clone());
            let mut result = apply_func(func.clone(), args, scopes, ctx, engine);
            if let (Some(spans), Some(args), Err(errors)) =
                (p1293_html_spans.as_ref(), html_args.as_ref(), &mut result)
            {
                if let Some(error) = errors.first_mut() {
                    error.span = spans.anchor(args, &error.message);
                }
            }
            // **P846 (#57)** — call trace: um `Tracepoint::Call` por chamada
            // cujo span não contém o erro (mirror de `call_func` +
            // `Trace::trace` do vanilla — `typst-eval/src/call.rs:166-180`,
            // `typst-library/src/diag.rs:464-479`).
            let result = trace_call(result, func.name(), call.span(), engine)?;
            // Intercepção eager — show rules aplicadas após apply_func (Passo 68).
            if let Value::Content(c) = result {
                Ok(Value::Content(rules::intercept_content(c, ctx, engine)?))
            } else {
                Ok(result)
            }
        }
        // P685 — nomes de tipo chamáveis: `int("5")`, `str(5)`, `float("3.5")`,
        // `type(1)`. Despacha para o construtor nativo; tipos não chamáveis
        // (`bool`, `length`, `array`, …) → erro "type X does not have a
        // constructor" (paridade vanilla).
        Value::Type(t) => {
            let world = engine.world;
            let current_file = engine.current_file;
            match t {
                Type::Int => native_int(ctx, &args, world, current_file),
                Type::Float => native_float(ctx, &args, world, current_file),
                Type::Str => native_str(ctx, &args, world, current_file),
                Type::Type => native_type(ctx, &args, world, current_file),
                Type::Path => native_path(ctx, &args, world, current_file),
                // P737 — `counter`/`state` são tipos chamáveis (paridade
                // vanilla — medido: type(counter)/type(state) → type;
                // counter("x")/state("y", 0) criam instâncias).
                Type::Counter => native_counter(ctx, &args, world, current_file),
                Type::State => native_state(ctx, &args, world, current_file),
                // P765a — `symbol(...)` constructor.
                Type::Symbol => native_symbol(ctx, &args, world, current_file),
                // P843 (F4/F5) — constructors `bytes(...)` e `datetime(...)`.
                Type::Bytes => native_bytes(ctx, &args, world, current_file),
                // P1284 — `arguments(...)` preserva simultaneamente a ordem
                // dos posicionais e dos named na representação já existente.
                Type::Arguments => Ok(Value::Args(args)),
                Type::Datetime => native_datetime(ctx, &args, world, current_file),
                // P1140.1-B — tipos públicos chamáveis, delegados aos
                // mesmos construtores atomizados da fase estrutural.
                Type::Decimal => native_decimal(ctx, &args, world, current_file),
                Type::Duration => native_duration(ctx, &args, world, current_file),
                Type::Regex => native_regex(ctx, &args, world, current_file),
                Type::Selector => native_selector(ctx, &args, world, current_file),
                Type::Stroke => native_stroke(ctx, &args, world, current_file),
                Type::Tiling => native_tiling(ctx, &args, world, current_file),
                Type::Version => native_version(ctx, &args, world, current_file),
                // P1140.2 — `label` é tipo chamável e produz Value::Label.
                Type::Label => native_label(ctx, &args, world, current_file),
                other => Err(vec![SourceDiagnostic::error(
                    call.callee().span(),
                    format!("type {} does not have a constructor", other.name()),
                )]),
            }
        }
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}

/// **P792** — Despacho de métodos de instância de `Value::Location`.
///
/// Intercepção em `eval_func_call` para `loc.page()`, `loc.position()`,
/// `loc.page-numbering()`. Paridade vanilla `location.rs #[scope]`.
///
/// Acede ao `ctx.introspector` (TagIntrospector implementa `Introspector::position_of`).
/// Se o introspector não tiver a posição da localização (pre-layout ou localização
/// inexistente), usa defaults seguros (page=1, x=0pt, y=0pt).
fn eval_location_method(
    loc: crate::entities::location::Location,
    method: &str,
    ctx: &mut EvalContext,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::layout_types::Length;

    match method {
        "page" => {
            // Número da página (1-based) via introspector; default 1 se não disponível.
            let page_num = ctx
                .introspector
                .position_of(loc)
                .map(|p| p.page.get() as i64)
                .unwrap_or(1);
            Ok(Value::Int(page_num))
        }
        "position" => {
            // Dict {page: int, x: length, y: length} em pontos.
            let pos = ctx.introspector.position_of(loc);
            let page_num = pos.as_ref().map(|p| p.page.get() as i64).unwrap_or(1);
            let x_pt = pos.as_ref().map(|p| p.point.x.0).unwrap_or(0.0);
            let y_pt = pos.as_ref().map(|p| p.point.y.0).unwrap_or(0.0);

            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            dict.insert("page".into(), Value::Int(page_num));
            dict.insert("x".into(), Value::Length(Length::pt(x_pt)));
            dict.insert("y".into(), Value::Length(Length::pt(y_pt)));
            Ok(Value::Dict(dict))
        }
        "page-numbering" => Ok(match ctx.introspector.page_numbering(loc) {
            Some(crate::entities::numbering::Numbering::Pattern(pattern)) => {
                Value::Str(pattern.clone())
            }
            Some(crate::entities::numbering::Numbering::Func(func)) => {
                Value::Func(func.clone())
            }
            None => Value::None,
        }),
        _ => unreachable!("eval_location_method chamado com método inesperado: {method}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;
    use crate::entities::file_id::FileId;

    fn p1293_span(start: usize) -> Span {
        use std::num::NonZeroU16;
        Span::from_range(FileId::from_raw(NonZeroU16::new(9).unwrap()), start..start + 1)
    }

    #[test]
    fn p1321_csv_transport_uses_identity_and_preserves_occurrences() {
        use crate::entities::args::ArgOccurrence;
        let native = Func::native("alias-not-csv", crate::compiler::stdlib::native_csv);
        let preargs = Args::from_occurrences(
            p1293_span(70),
            vec![ArgOccurrence {
                name: None,
                value: Value::Int(7),
                span: p1293_span(71),
                value_span: p1293_span(72),
            }],
        );
        let with = native.clone().with(preargs).with(Args::positional(vec![]));
        for func in [native, with] {
            for populated in [false, true] {
                let occurrences = if populated {
                    vec![ArgOccurrence {
                        name: Some("delimiter".into()),
                        value: Value::Int(3),
                        span: p1293_span(20),
                        value_span: p1293_span(21),
                    }]
                } else {
                    vec![]
                };
                let mut args = Args::from_occurrences(p1293_span(10), occurrences);
                transport_native_call_span(&func, &mut args, p1293_span(1));
                assert_eq!(args.span, p1293_span(1));
                assert!(args.items.is_empty());
                assert_eq!(args.named.len(), usize::from(populated));
                let after = args.occurrences.as_ref().unwrap();
                assert_eq!(after.len(), usize::from(populated));
                if populated {
                    assert_eq!(after[0].name.as_deref(), Some("delimiter"));
                    assert!(matches!(after[0].value, Value::Int(3)));
                    assert_eq!(after[0].span, p1293_span(20));
                    assert_eq!(after[0].value_span, p1293_span(21));
                }
            }
            if let FuncRepr::With(outer) = func.repr() {
                let FuncRepr::With(inner) = outer.0.repr() else { panic!() };
                assert_eq!(inner.1.span, p1293_span(70));
                let occurrence = &inner.1.occurrences.as_ref().unwrap()[0];
                assert!(matches!(occurrence.value, Value::Int(7)));
                assert_eq!(occurrence.span, p1293_span(71));
                assert_eq!(occurrence.value_span, p1293_span(72));
            }
        }
    }

    #[test]
    fn p1321_csv_transport_preserves_other_identities() {
        use crate::compiler::stdlib::{
            native_json, native_json_encode, native_panic, native_read,
            native_toml_encode, native_yaml_encode,
        };
        let fake = Func::native("csv", |_ctx, _args, _world, _file| Ok(Value::None));
        for func in [
            fake.clone(),
            fake.with(Args::positional(vec![])),
            Func::native("csv", native_read),
            Func::native("csv", native_json),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(10));
            assert!(args.occurrences.is_none());
        }
        for func in [
            Func::native("alias", native_json_encode),
            Func::native("alias", native_toml_encode),
            Func::native("alias", native_yaml_encode),
            Func::native("alias", native_panic),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(1));
            assert!(args.occurrences.is_none());
        }
    }

    #[test]
    fn p1293_c_identidades_html_e_ancoras_sao_estritas() {
        let module = crate::compiler::stdlib::make_html_module();
        for (name, expected_void) in [
            ("button", false),
            ("col", true),
            ("iframe", false),
            ("select", false),
            ("template", false),
            ("video", false),
            ("wbr", true),
        ] {
            let Value::Func(func) = module.scope().get(name).unwrap() else { panic!() };
            assert!(
                matches!(
                    P1293HtmlNative::from_func(func),
                    Some(P1293HtmlNative::Void) if expected_void
                ) || matches!(
                    P1293HtmlNative::from_func(func),
                    Some(P1293HtmlNative::Normal) if !expected_void
                )
            );
        }
        let fake = Func::native("video", |_ctx, _args, _world, _file| Ok(Value::None));
        assert!(P1293HtmlNative::from_func(&fake).is_none());

        let mut named = IndexMap::default();
        named.insert(
            EcoString::from("width"),
            P1293NamedSpan { full: p1293_span(20), value: p1293_span(21) },
        );
        let spans = P1293HtmlCallSpans {
            kind: P1293HtmlNative::Normal,
            positional: vec![],
            named,
            has_spread: false,
        };
        let mut args = Args::positional(vec![]);
        args.named.insert("width".into(), Value::Int(-1));
        assert_eq!(spans.anchor(&args, "number must be at least zero"), p1293_span(21));
        assert_eq!(spans.anchor(&args, "unexpected argument: width"), p1293_span(20));
    }

    /// Host de teste (sem WASM) para o braço `call_plugin`. Conta chamadas e
    /// devolve `b"OK"`. Nomes de export únicos por teste evitam colisão no
    /// cache global do comemo (`PluginFunc::call` é memoizado).
    struct StubHost {
        next: AtomicU64,
        calls: AtomicUsize,
    }
    impl StubHost {
        fn new() -> Self {
            Self {
                next: AtomicU64::new(1),
                calls: AtomicUsize::new(0),
            }
        }
    }
    impl PluginHost for StubHost {
        fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
        fn exports(
            &self,
            _module: PluginModuleId,
        ) -> Result<Vec<EcoString>, PluginError> {
            Ok(Vec::new())
        }
        fn call(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Bytes::new(b"OK".to_vec()))
        }
        fn transition(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
    }

    fn make_pf(name: &str) -> PluginFunc {
        let host: Arc<StubHost> = Arc::new(StubHost::new());
        let id = host.load(&[]).unwrap();
        PluginFunc {
            host: host as Arc<dyn PluginHost>,
            module: id,
            name: EcoString::from(name),
        }
    }

    #[test]
    fn call_plugin_devolve_bytes() {
        let pf = make_pf("cp_devolve");
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"x".to_vec()))]);
        let v = call_plugin(&pf, &args).unwrap();
        assert!(matches!(v, Value::Bytes(b) if b.as_slice() == b"OK"));
    }

    #[test]
    fn call_plugin_arg_nao_bytes_erro() {
        let pf = make_pf("cp_argbad");
        let args = Args::positional(vec![Value::Int(1)]);
        let e = call_plugin(&pf, &args).unwrap_err();
        // P819 — verbatim do vanilla (medido t6: `expected bytes, found integer`).
        assert_eq!(
            e[0].message.as_str(),
            "expected bytes, found integer",
            "msg: {}",
            e[0].message
        );
    }

    #[test]
    fn call_plugin_named_arg_erro() {
        let pf = make_pf("cp_named");
        let mut named = IndexMap::with_hasher(FxBuildHasher);
        named.insert(EcoString::from("x"), Value::None);
        let args = Args::from_parts(
            vec![Value::Bytes(Bytes::default())],
            named,
            Span::detached(),
        );
        let e = call_plugin(&pf, &args).unwrap_err();
        assert!(
            e[0].message.contains("argumento nomeado inesperado"),
            "msg: {}",
            e[0].message,
        );
    }
}

#[cfg(test)]
mod p1334_dispatch_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;
    use crate::entities::file_id::FileId;

    fn p1293_span(start: usize) -> Span {
        use std::num::NonZeroU16;
        Span::from_range(FileId::from_raw(NonZeroU16::new(9).unwrap()), start..start + 1)
    }

    #[test]
    fn p1334_abs_transport_uses_identity_and_preserves_occurrences() {
        use crate::entities::args::ArgOccurrence;
        let Value::Module(module) = crate::compiler::stdlib::make_calc_module() else {
            panic!()
        };
        let Value::Func(native) = module.scope().get("abs").unwrap().clone() else {
            panic!()
        };
        let preargs = Args::from_occurrences(
            p1293_span(70),
            vec![ArgOccurrence {
                name: None,
                value: Value::Int(7),
                span: p1293_span(71),
                value_span: p1293_span(72),
            }],
        );
        let with = native.clone().with(preargs).with(Args::positional(vec![]));
        for func in [native, with] {
            for populated in [false, true] {
                let occurrences = if populated {
                    vec![ArgOccurrence {
                        name: Some("delimiter".into()),
                        value: Value::Int(3),
                        span: p1293_span(20),
                        value_span: p1293_span(21),
                    }]
                } else {
                    vec![]
                };
                let mut args = Args::from_occurrences(p1293_span(10), occurrences);
                transport_native_call_span(&func, &mut args, p1293_span(1));
                assert_eq!(args.span, p1293_span(1));
                assert!(args.items.is_empty());
                assert_eq!(args.named.len(), usize::from(populated));
                let after = args.occurrences.as_ref().unwrap();
                assert_eq!(after.len(), usize::from(populated));
                if populated {
                    assert_eq!(after[0].name.as_deref(), Some("delimiter"));
                    assert!(matches!(after[0].value, Value::Int(3)));
                    assert_eq!(after[0].span, p1293_span(20));
                    assert_eq!(after[0].value_span, p1293_span(21));
                }
            }
            if let FuncRepr::With(outer) = func.repr() {
                let FuncRepr::With(inner) = outer.0.repr() else { panic!() };
                assert_eq!(inner.1.span, p1293_span(70));
                let occurrence = &inner.1.occurrences.as_ref().unwrap()[0];
                assert!(matches!(occurrence.value, Value::Int(7)));
                assert_eq!(occurrence.span, p1293_span(71));
                assert_eq!(occurrence.value_span, p1293_span(72));
            }
        }
    }

    #[test]
    fn p1334_abs_transport_preserves_other_identities() {
        use crate::compiler::stdlib::{
            native_csv, native_json, native_json_encode, native_panic, native_read,
            native_toml_encode, native_yaml_encode,
        };
        let fake = Func::native("abs", |_ctx, _args, _world, _file| Ok(Value::None));
        for func in [
            fake.clone(),
            fake.with(Args::positional(vec![])),
            Func::native("abs", native_read),
            Func::native("abs", native_json),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(10));
            assert!(args.occurrences.is_none());
        }
        for func in [
            Func::native("alias", native_csv),
            Func::native("alias", native_json_encode),
            Func::native("alias", native_toml_encode),
            Func::native("alias", native_yaml_encode),
            Func::native("alias", native_panic),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(1));
            assert!(args.occurrences.is_none());
        }
    }
}
