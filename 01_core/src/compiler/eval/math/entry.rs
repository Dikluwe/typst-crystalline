//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/entry.md
//! @prompt-hash d4f38e7f
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Avalia o corpo de uma equação matemática — produz `Content` a partir de `Math<'_>`.
///
/// Stub intencional (Passo 34): produz a estrutura de nós correcta sem motor de
/// renderização. O motor real (Passo 36+) substitui esta função com layout tipográfico.
pub(in crate::compiler::eval) fn eval_math_content(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    math: Math<'_>,
) -> SourceResult<Content> {
    let mut nodes: Vec<Content> = Vec::new();
    for expr in math.exprs() {
        let node = eval_math_expr(scopes, ctx, engine, expr)?;
        // **P997** — uma sub-`MathSequence` com `Linebreak` ao seu nível do
        // topo é especialada na sequência pai (os itens entram em linha, a
        // quebra sobe para o nível do run): no vanilla a quebra divide a
        // run INTEIRA (`expand_multiline_fence` devolve os
        // `RawMathItem::Linebreak` no nível do run,
        // `ir/multiline.rs:56-107`), não fica presa dentro do grupo — sem
        // isto, o conteúdo a seguir a `(n \ k)` ancorava na linha de cima.
        // Sem `Linebreak`: aninhamento preservado (inalterado).
        match node {
            Content::MathSequence(items)
                if items.iter().any(|c| matches!(c, Content::Linebreak(_))) =>
            {
                nodes.extend(items.iter().cloned());
            }
            Content::Empty => {}
            other => nodes.push(other),
        }
    }
    match nodes.len() {
        0 => Ok(Content::Empty),
        1 => Ok(nodes.remove(0)),
        _ => Ok(Content::MathSequence(nodes.into())),
    }
}
