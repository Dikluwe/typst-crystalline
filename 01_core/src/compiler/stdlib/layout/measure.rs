//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/measure.md
//! @prompt-hash cee4fea8
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::measure`.

use super::*;

/// Extrai e valida o argumento `body` de `measure(...)` — 1 posicional
/// (`Content` ou `Str`), sem argumentos nomeados (`width`/`height` override
/// scope-out, Opção β graded per ADR-0054). Partilhado (P712) entre
/// `native_measure` (fallback de invocação indirecta) e a intercepção real
/// em `eval_func_call` (`eval/closures.rs`, com acesso a `engine.styles`).
pub fn extract_measure_body(args: &Args) -> SourceResult<Content> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "measure(body): espera Content ou Str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "measure: argumento posicional body obrigatório ausente".to_string(),
            )])
        }
    };

    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("measure: aceita 1 posicional (body), recebeu {}", args.items.len()),
        )]);
    }

    if let Some(key) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("measure: named arg `{}` não suportado (paridade graded; refino futuro candidato NÃO-reservado per ADR-0054)", key),
        )]);
    }

    Ok(body)
}

/// `measure(body) -> dict(width: length, height: length)` —
/// Passo 222 (Fase 4 Layout candidata; ADR-0066 §"Plano
/// promoção" Bloco C cross-módulo primeira materialização
/// parcial).
///
/// **P712** — este `NativeFn` é apenas o fallback para invocação indirecta
/// de `measure` (ex.: passado como valor de primeira classe a outra função,
/// sem sintaxe de chamada directa). A invocação real e directa
/// (`measure(...)`/`std.measure(...)`) é interceptada em `eval_func_call`
/// (`eval/closures.rs` §P712, mesmo padrão de `.to-absolute()`/`.with()`),
/// porque precisa de `engine.styles` (tamanho medido depende do `#set
/// text(size:)` activo) e do gate `ctx.in_context` — nenhum dos dois
/// acessível pela assinatura genérica `NativeFn(ctx, args, world, file)`.
/// Sem consumidor medido para o caminho indirecto (nenhum documento de
/// teste ou `cetz` passa `measure` como valor); falha alto em vez de
/// devolver `(0, 0)` silenciosamente (ADR-0108).
/// **P792** — `layout(func)` — stub de despacho.
///
/// A lógica real vive em `eval/closures.rs::eval_func_call` (intercepção
/// pelo `native_fn_addr`, mesmo padrão de `native_measure`/P712).
/// Este stub nunca é invocado directamente — serve apenas para:
/// 1. Existir no scope global como `Value::Func("layout")`.
/// 2. Expor um endereço de função identificável para a intercepção.
///
/// Paridade vanilla: `layout/layout.rs:66` `#[func]` nativo global.
pub fn native_layout(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "layout(): invocação indirecta não suportada — usar layout(size => ...) — P792"
            .to_string(),
    )])
}

pub fn native_measure(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "measure(): invocação indirecta (fora de measure(...)/std.measure(...) \
         directo) não suportada — P712"
            .to_string(),
    )])
}
