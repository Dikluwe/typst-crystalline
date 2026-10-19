//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/trace_plugin.md
//! @prompt-hash cccd14bc
//! @layer L1
//! @updated 2026-10-07

use super::*;
use crate::entities::content::Content;

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
pub(super) fn trace_call(
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

/// Table realization keeps the constructor's causal call even after eval returns.
pub(crate) fn trace_table_realization(
    result: SourceResult<Content>,
    arguments_span: Span,
    engine: &Engine<'_>,
) -> SourceResult<Content> {
    let call_span = arguments_span
        .id()
        .and_then(|id| engine.world.source(id).ok())
        .and_then(|source| {
            let root = crate::entities::syntax_node::LinkedNode::new(source.root());
            let args = root.find(arguments_span)?;
            if args.kind() == crate::entities::syntax_kind::SyntaxKind::FuncCall {
                return Some(args.span());
            }
            let parent = args.parent()?;
            (parent.kind() == crate::entities::syntax_kind::SyntaxKind::FuncCall)
                .then(|| parent.span())
        });
    let Some(span) = call_span else {
        return result;
    };
    trace_call(result.map(Value::Content), Some("table"), span, engine).map(|value| {
        match value {
            Value::Content(c) => c,
            _ => unreachable!(),
        }
    })
}

/// **P702** — funde os `Args` pré-ligados por `.with(...)` com os da chamada
/// final. Posicionais: pré-ligados primeiro (paridade vanilla,
/// `foundations/func.rs:360` do vanilla: `pre.items.chain(new.items)`).
/// Todas as ocorrências nomeadas são preservadas; a view projeta a última.
pub(super) fn merge_with_args(pre: &Args, new: Args) -> Args {
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
pub(super) fn call_plugin(p: &PluginFunc, args: &Args) -> SourceResult<Value> {
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
