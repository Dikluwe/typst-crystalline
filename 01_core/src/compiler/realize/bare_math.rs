//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/bare_math.md
//! @prompt-hash 9962c788
//! @layer L1
//! @updated 2026-10-09

use crate::entities::content::{Content, RealizedContent, ShowRecipeContent};
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use std::sync::Arc;

/// Static counterpart of the implemented upstream Mathy elements.
fn mathy(content: &Content) -> bool {
    matches!(
        content,
        Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathOp(_)
            | Content::MathAlignPoint(_)
    )
}

fn block(styles: &StyleChain) -> bool {
    matches!(styles.custom("equation.block"), Some(Value::Bool(true)))
}

/// A closed, read-only certificate for subtrees where `prepare` cannot insert
/// an equation. Unknown carriers always use the transforming traversal.
fn plain_without_bare_math(content: &Content) -> bool {
    match content {
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline => true,
        Content::Equation(_) | Content::Located(_) => true,
        Content::Source(e) => plain_without_bare_math(&e.body),
        Content::Styled(e) => plain_without_bare_math(&e.body),
        Content::Sequence(parts) => parts.iter().all(plain_without_bare_math),
        Content::ShowRecipe(e) => plain_without_bare_math(&e.body),
        Content::Realized(e) => plain_without_bare_math(&e.visual),
        Content::TableCell(e) => plain_without_bare_math(e.visual()),
        Content::Table(e) => {
            let cells = e.resolved.as_ref().map_or(&e.children, |r| &r.cells);
            cells.iter().all(plain_without_bare_math)
                && e.caption.as_ref().is_none_or(plain_without_bare_math)
                && (e.resolved.is_some()
                    || (e.header.as_ref().is_none_or(plain_without_bare_math)
                        && e.footer.as_ref().is_none_or(plain_without_bare_math)))
        }
        _ => false,
    }
}

pub(in crate::compiler) fn prepare(
    content: &Content,
    styles: &StyleChain,
) -> SourceResult<Content> {
    if plain_without_bare_math(content) {
        return Ok(content.clone());
    }
    content.map_content_pre(&mut |node| {
        if plain_without_bare_math(node) {
            return Ok(Some(node.clone()));
        }
        if let Some(equation) = prepare_root(node, styles) {
            return Ok(Some(equation));
        }
        match node {
            Content::Equation(_) | Content::Located(_) => Ok(Some(node.clone())),
            Content::Styled(e) => Ok(Some(Content::styled(
                prepare(&e.body, &styles.push_styles(&e.styles))?,
                e.styles.clone(),
            ))),
            Content::ShowRecipe(e) => {
                Ok(Some(Content::ShowRecipe(Arc::new(ShowRecipeContent {
                    body: prepare(&e.body, &e.styles)?,
                    ..e.as_ref().clone()
                }))))
            }
            Content::Realized(e) => {
                Ok(Some(Content::Realized(Arc::new(RealizedContent {
                    original: e.original.clone(),
                    visual: prepare(&e.visual, styles)?,
                }))))
            }
            Content::Label(e) if mathy(&e.body) => {
                Ok(Some(Content::equation(node.clone(), block(styles))))
            }
            Content::MathStyled(e) => {
                let body = prepare(&e.body, styles)?;
                let mut found = false;
                let lifted = body.map_content_pre(&mut |child| {
                    if let Content::Equation(eq) = child {
                        found = true;
                        let mut styled = e.as_ref().clone();
                        styled.body = eq.body.clone();
                        Ok(Some(Content::equation(
                            Content::MathStyled(Arc::new(styled)),
                            eq.block,
                        )))
                    } else {
                        Ok(None)
                    }
                })?;
                Ok(Some(if found { lifted } else { node.clone() }))
            }
            _ => Ok(None),
        }
    })
}

pub(in crate::compiler) fn prepare_root(
    content: &Content,
    styles: &StyleChain,
) -> Option<Content> {
    mathy(content).then(|| Content::equation(content.clone(), block(styles)))
}

pub(in crate::compiler) fn needs_preparation(content: &Content) -> bool {
    let mut found = false;
    let _ = content.map_content_pre(&mut |node| {
        if mathy(node) {
            found = true;
        }
        Ok(if mathy(node) || matches!(node, Content::Equation(_) | Content::Located(_)) {
            Some(node.clone())
        } else {
            None
        })
    });
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(content: &Content) -> usize {
        let mut count = 0;
        content
            .map_content(&mut |node| {
                if matches!(node, Content::Equation(_)) {
                    count += 1;
                }
                Ok(None)
            })
            .unwrap();
        count
    }

    #[test]
    fn math_bare_insertion_boundaries_and_idempotence() {
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let content = Content::sequence(vec![
            frac.clone(),
            Content::equation(frac.clone(), false),
            frac,
        ]);
        let prepared = prepare(&content, &StyleChain::default_chain()).unwrap();
        assert_eq!(count(&prepared), 3);
        assert_eq!(count(&prepare(&prepared, &StyleChain::default_chain()).unwrap()), 3);
        assert!(!needs_preparation(&prepared));
        assert_eq!(
            count(&prepare(&Content::text("x"), &StyleChain::default_chain()).unwrap()),
            0
        );
    }

    #[test]
    fn math_bare_insertion_local_block_and_math_style() {
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let outer =
            StyleChain::default_chain().push_custom("equation.block", Value::Bool(true));
        let local = Content::styled(
            frac.clone(),
            crate::entities::style::Styles::new()
                .push_custom("equation.block", Value::Bool(false)),
        );
        let prepared = prepare(&local, &outer).unwrap();
        let Content::Styled(styled) = prepared else { panic!("local style lost") };
        let Content::Equation(eq) = &styled.body else { panic!("missing equation") };
        assert!(!eq.block);
        let bold = Content::math_styled(None, Some(true), None, frac, None);
        let prepared = prepare(&bold, &outer).unwrap();
        let Content::Equation(eq) = prepared else {
            panic!("math style swallowed equation")
        };
        assert!(eq.block);
        assert!(matches!(&eq.body, Content::MathStyled(e) if e.bold == Some(true)));
    }

    #[test]
    fn plain_table_subtree_keeps_identity_but_mathy_cell_and_caption_transform() {
        let plain = Content::table_with_caption(
            vec![],
            vec![],
            vec![Content::styled(
                Content::table_cell(Content::text("Local"), None, None, None, None),
                crate::entities::style::Styles::new().push_custom(
                    "cell.fill",
                    Value::Color(crate::entities::layout_types::Color::rgb(255, 0, 0)),
                ),
            )],
            Some(Content::text("caption")),
        );
        let prepared = prepare(&plain, &StyleChain::default_chain()).unwrap();
        let (Content::Table(before), Content::Table(after)) = (&plain, &prepared) else {
            panic!("table carrier lost")
        };
        assert!(Arc::ptr_eq(before, after), "plain subtree should not be rebuilt");

        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let math_cell = Content::table_cell(frac.clone(), None, None, None, None);
        let math_table =
            Content::table_with_caption(vec![], vec![], vec![math_cell], Some(frac));
        let prepared = prepare(&math_table, &StyleChain::default_chain()).unwrap();
        let Content::Table(table) = prepared else { panic!("table carrier lost") };
        assert!(
            matches!(&table.children[0], Content::TableCell(cell) if matches!(cell.body, Content::Equation(_)))
        );
        assert!(matches!(table.caption, Some(Content::Equation(_))));
    }
}
