//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/native_decorations.md
//! @prompt-hash 041b2900
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Despacha ornamentos over/under e o construtor de vetor matemático.
pub(super) fn dispatch_math_decoration(
    name: &str,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    match name {
        // **§P906** — `underbrace`/`overbrace`/`underbracket`/`overbracket`:
        // conveniência de `Content::math_underover` (P297) para o caso
        // comum. Chars confirmados no vanilla real
        // (`ir/resolve.rs::resolve_underbrace/overbrace/underbracket/
        // overbracket`): underbrace=`⏟` U+23DF, overbrace=`⏞` U+23DE,
        // underbracket=`⎵` U+23B5, overbracket=`⎴` U+23B4. Assinatura:
        // `nome(body, annotation?)` — `body` posicional obrigatório,
        // `annotation` posicional opcional (paridade vanilla:
        // `UnderbraceElem { body: required, annotation: positional
        // Option }`). Sem anotação: `MathUnderover` de 1 nível
        // (`under`/`over` = chave). Com anotação: aninhamento de 2
        // `MathUnderover` — interno = `(body, chave)`, externo =
        // `(interno, anotação)`; o esticamento em `layout_underover`
        // usa `base_box.width` do seu PRÓPRIO `base`, logo no nível
        // interno a chave estica só sobre o body, e no nível externo a
        // anotação (texto normal) fica centrada sob/sobre o conjunto
        // já composto — ver `compiler/layout.md` §P906.
        "overbrace" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overbrace_from_args(&args)
        }

        "underbrace" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underbrace_from_args(&args)
        }

        "overbracket" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overbracket_from_args(&args)
        }

        "overline" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overline_from_args(&args)
        }

        "overparen" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overparen_from_args(&args)
        }

        "overshell" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overshell_from_args(&args)
        }

        "underbracket" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underbracket_from_args(&args)
        }

        "underparen" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underparen_from_args(&args)
        }

        "undershell" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_undershell_from_args(&args)
        }

        // `vec` converge com `math.vec`: identidade própria, children
        // variádicos e os mesmos casts/named do construtor público.
        "vec" => {
            let mut delim = ('(', ')');
            let mut align = crate::entities::layout_types::HAlign::Center;
            let mut gap = crate::entities::rel::Rel {
                rel: 0.0,
                abs: crate::entities::layout_types::Length::em(0.2),
            };
            let mut explicit =
                crate::entities::elements::math_vec::MathVecExplicit::default();
            for name in ["delim", "align", "gap"] {
                let key = format!("math.vec.{name}");
                if let Some(value) = engine.styles.custom(&key) {
                    apply_syntax_vec_option(
                        &mut delim,
                        &mut align,
                        &mut gap,
                        &mut explicit,
                        name,
                        value,
                        call.span(),
                        false,
                    )?;
                }
            }

            let mut children = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(expr) => {
                        children.push(eval_math_expr(scopes, ctx, engine, expr)?);
                    }
                    Arg::Named(named) => {
                        let value =
                            eval_math_arg_value(scopes, ctx, engine, named.expr())?;
                        apply_syntax_vec_option(
                            &mut delim,
                            &mut align,
                            &mut gap,
                            &mut explicit,
                            named.name().as_str(),
                            &value,
                            call.span(),
                            true,
                        )?;
                    }
                    Arg::Spread(_) => {}
                }
            }
            Ok(Content::math_vec_full(children, delim, align, gap, explicit))
        }

        _ => unreachable!("math decoration already classified"),
    }
}
