//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de composição visual (figure, image).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

/// `figure(body, caption: content)` → `Content::Figure`.
///
/// Migrada do interceptador em `eval.rs` para `stdlib.rs` — o avaliador deixa
/// de conhecer o nome "figure" (DEBT-16 encerrado).
///
/// - `body`: argumento posicional obrigatório.
/// - `caption:`: argumento nomeado opcional; `none` → sem legenda.
pub fn native_figure(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Argumento posicional: body (obrigatório)
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(_)                 => Content::Empty,
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "figure() requer um argumento posicional (body)".to_string(),
        )]),
    };

    // Argumento nomeado: caption (opcional)
    // Value::None → ausência de legenda (comportamento intencional).
    let caption = args.named.get("caption").and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.type_name()))),
    });

    // Argumento nomeado: kind (Passo 75, DEBT-15).
    let kind = args.named.get("kind")
        .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
        .unwrap_or_else(|| "image".to_string());

    // Numeração capturada do contexto (Passo 75, DEBT-14).
    // Reflecte o estado activo de `#set figure(numbering: ...)` no momento da chamada.
    let numbering = ctx.figure_numbering.clone();

    Ok(Value::Content(Content::Figure {
        body: Box::new(body),
        caption,
        kind,
        numbering,
    }))
}

// ── `image()` — carregamento de imagens do disco (Passo 71, DEBT-24) ─────────

/// `image(path, width?, height?)` → `Content::Image`.
///
/// Lê os bytes do ficheiro através de `ctx.world.read_bytes(path)`.
/// `width` e `height` são preservados no AST para o Passo 72 (dimensões reais).
/// O layouter usa placeholder 100×100 pt neste passo (DEBT-24b).
pub fn native_image(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "width" e "height" são aceites.
    for key in args.named.keys() {
        if key.as_str() != "width" && key.as_str() != "height" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em image(): '{}'", key),
            )]);
        }
    }

    let path = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image() requer string com o caminho, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "image() requer 1 argumento posicional (caminho do ficheiro)".to_string(),
        )]),
    };

    let data = match ctx.world.read_bytes(ctx.current_file, &path) {
        Ok(arc) => arc,
        Err(msg) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image(): não foi possível ler '{}': {}", path, msg),
        )]),
    };

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);

    Ok(Value::Content(Content::Image { path, data: PtrEqArc(data), width, height }))
}
