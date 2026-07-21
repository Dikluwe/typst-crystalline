//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/math_style.md
//! @prompt-hash 773df339
//! @layer L1
//! @updated 2026-05-20
//!
//! 12 funções math style — `bb`/`bold`/`cal`/`frak`/`italic`/`mono`/
//! `sans`/`scr`/`script`/`serif`/`sscript`/`upright` (P311b.3 per
//! diagnóstico P311a). Cada função wrap o body em
//! `Content::MathStyled` com `kind`/`bold`/`italic`/`cramped`
//! específicos. Composição (outer-wins variant, ortogonal bold)
//! resolvida em `MathLayouter` (P311b.4).

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::math_style::MathStyleKind;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Helper único — extrai o body do primeiro arg e wraps em
/// `Content::MathStyled` com os fields fornecidos.
///
/// Elimina duplicação nas funções nativas math style.
///
/// `default_cramped`: valor usado quando o named arg `cramped` não é
/// fornecido. `None` significa que `cramped` fica `None` e a função rejeita
/// qualquer named arg.
fn wrap_math_style(
    args: &Args,
    name: &str,
    kind: Option<MathStyleKind>,
    bold: Option<bool>,
    italic: Option<bool>,
    default_cramped: Option<bool>,
) -> SourceResult<Value> {
    let cramped = if let Some(default) = default_cramped {
        match args.named.get("cramped") {
            Some(Value::Bool(b)) => Some(*b),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("cramped deve ser bool, recebeu {}", other.type_name()),
                )]);
            }
            None => Some(default),
        }
    } else {
        for key in args.named.keys() {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}() não aceita o argumento nomeado '{}'", name, key),
            )]);
        }
        None
    };

    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}() espera 1 argumento, recebeu {}", name, args.items.len()),
        )]);
    }
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}() espera content ou string, recebeu {}",
                    name,
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty,
    };
    Ok(Value::Content(Content::math_styled(kind, bold, italic, body, cramped)))
}

/// `bb(body)` — wrap em variant DoubleStruck (blackboard bold).
pub fn native_bb(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "bb", Some(MathStyleKind::DoubleStruck), None, None, None)
}

/// `bold(body)` — flag bold ortogonal (preserva variant inner).
pub fn native_bold(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "bold", None, Some(true), None, None)
}

/// `cal(body)` — wrap em variant Chancery (script).
pub fn native_cal(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "cal", Some(MathStyleKind::Chancery), None, None, None)
}

/// `display(body, cramped: false)` — forced display style in math.
pub fn native_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(
        args,
        "display",
        Some(MathStyleKind::Display),
        None,
        None,
        Some(false),
    )
}

/// `frak(body)` — wrap em variant Fraktur.
pub fn native_frak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "frak", Some(MathStyleKind::Fraktur), None, None, None)
}

/// `inline(body, cramped: false)` — forced inline (text) style in math.
pub fn native_inline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "inline", Some(MathStyleKind::Inline), None, None, Some(false))
}

/// `italic(body)` — flag italic ortogonal.
pub fn native_math_italic(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "italic", None, None, Some(true), None)
}

/// `mono(body)` — wrap em variant Monospace.
pub fn native_mono(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "mono", Some(MathStyleKind::Monospace), None, None, None)
}

/// `sans(body)` — wrap em variant Sans-Serif.
pub fn native_sans(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "sans", Some(MathStyleKind::SansSerif), None, None, None)
}

/// `scr(body)` — wrap em variant Roundhand (Bold Script).
pub fn native_scr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "scr", Some(MathStyleKind::Roundhand), None, None, None)
}

/// `script(body, cramped: true)` — wrap em variant Script (size factor 0.7) + cramped.
pub fn native_script(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "script", Some(MathStyleKind::Script), None, None, Some(true))
}

/// `serif(body)` — force variant Plain (override outer variant).
pub fn native_serif(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "serif", Some(MathStyleKind::Plain), None, None, None)
}

/// `sscript(body, cramped: true)` — wrap em variant SScript (size factor 0.5) + cramped.
pub fn native_sscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "sscript", Some(MathStyleKind::SScript), None, None, Some(true))
}

/// `upright(body)` — flag italic=Some(false) (suprime itálico).
pub fn native_upright(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    wrap_math_style(args, "upright", None, None, Some(false), None)
}

// Tests para as funções math style ficam em `stdlib::mod.rs` per
// pattern P308 — todos os tests de submódulos stdlib partilham
// `NullWorld`/`null_ctx!`/`test_file_id` definidos no `mod` parent.
