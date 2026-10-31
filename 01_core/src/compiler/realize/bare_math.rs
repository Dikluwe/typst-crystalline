//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/bare_math.md
//! @prompt-hash 40c97433
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
            let cells = e
                .resolved
                .as_ref()
                .map_or(e.items.as_ref().unwrap_or(&e.children), |r| &r.cells);
            cells.iter().all(plain_without_bare_math)
                && e.caption.as_ref().is_none_or(plain_without_bare_math)
                && (e.resolved.is_some()
                    || e.items.is_some()
                    || (e.header.as_ref().is_none_or(plain_without_bare_math)
                        && e.footer.as_ref().is_none_or(plain_without_bare_math)))
        }
        // These carriers do not prove absence; retain full transforming traversal.
        Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
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
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => false,
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
            Content::Label(e) => {
                if mathy(&e.body) {
                    Ok(Some(Content::equation(node.clone(), block(styles))))
                } else {
                    // Ordinary labels keep their child insertion boundary.
                    Ok(None)
                }
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
            // No special root handling: let the preorder owner visit children.
            // Proof-positive and supported Mathy roots already returned above.
            Content::Source(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::MathSequence(_)
            | Content::Symbol(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
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
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => Ok(None),
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

#[cfg(test)]
mod batch_g_tests {
    use super::*;
    use crate::entities::content::LocatedContent;
    use crate::entities::file_id::FileId;
    use crate::entities::location::Location;
    use std::num::NonZeroU16;

    #[test]
    fn batch_g_absence_proof_preserves_boundaries_without_certifying_other_containers() {
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let original_only = Content::Realized(Arc::new(RealizedContent {
            original: frac.clone(),
            visual: Content::text("visible"),
        }));
        assert!(plain_without_bare_math(&original_only));
        let visual_only = Content::Realized(Arc::new(RealizedContent {
            original: Content::text("original"),
            visual: frac.clone(),
        }));
        assert!(!plain_without_bare_math(&visual_only));
        assert!(!plain_without_bare_math(&Content::title(Content::text("plain"))));
        let title =
            prepare(&Content::title(frac.clone()), &StyleChain::default_chain()).unwrap();
        assert!(
            matches!(title, Content::Title(e) if matches!(e.body, Content::Equation(_)))
        );
        let located = Content::Located(Arc::new(LocatedContent {
            location: Location::from_raw(31),
            body: frac,
        }));
        let result = prepare(&located, &StyleChain::default_chain()).unwrap();
        assert!(
            matches!((&located, &result), (Content::Located(a), Content::Located(b)) if Arc::ptr_eq(a,b))
        );
    }

    #[test]
    fn batch_g_label_guard_wraps_root_or_descends_into_non_mathy_body() {
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let direct = Content::label("direct", frac.clone());
        let wrapped = prepare(&direct, &StyleChain::default_chain()).unwrap();
        assert!(
            matches!(wrapped, Content::Equation(e) if matches!(&e.body, Content::Label(l) if l.name == "direct" && matches!(l.body, Content::MathFrac(_))))
        );
        let nested = Content::label("nested", Content::title(frac));
        let prepared = prepare(&nested, &StyleChain::default_chain()).unwrap();
        assert!(
            matches!(prepared, Content::Label(l) if matches!(&l.body, Content::Title(t) if matches!(t.body, Content::Equation(_))))
        );
    }

    #[test]
    fn batch_g_recipe_styles_and_realized_visual_preserve_causal_snapshot() {
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let recipe = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: frac.clone(),
            textual: true,
            context_id_floor: 29,
            rules: Arc::from([]),
            styles: StyleChain::default_chain()
                .push_custom("equation.block", Value::Bool(true)),
            guards: vec![17],
            file,
        }));
        let prepared = prepare(&recipe, &StyleChain::default_chain()).unwrap();
        let Content::ShowRecipe(after) = prepared else { panic!("recipe lost") };
        assert!(matches!(&after.body, Content::Equation(e) if e.block));
        assert!(after.textual);
        assert_eq!(after.context_id_floor, 29);
        assert_eq!(after.guards, vec![17]);
        assert_eq!(after.file, file);
        let Content::ShowRecipe(before) = recipe else { unreachable!() };
        assert!(Arc::ptr_eq(&before.rules, &after.rules));
        let tree = Content::Realized(Arc::new(RealizedContent {
            original: frac.clone(),
            visual: frac,
        }));
        let prepared = prepare(&tree, &StyleChain::default_chain()).unwrap();
        assert!(
            matches!(prepared, Content::Realized(e) if matches!(e.original, Content::MathFrac(_)) && matches!(e.visual, Content::Equation(_)))
        );
    }

    #[test]
    fn batch_g_table_absence_uses_resolved_cells_and_always_checks_caption() {
        use crate::entities::elements::table::TableGridResolved;
        let frac = Content::math_frac(Content::text("1"), Content::text("2"));
        let Content::Table(base) = Content::table(vec![], vec![], vec![]) else {
            unreachable!()
        };
        let mut raw = base.as_ref().clone();
        raw.header = Some(frac.clone());
        assert!(!plain_without_bare_math(&Content::Table(Arc::new(raw.clone()))));
        raw.header = None;
        raw.footer = Some(frac.clone());
        assert!(!plain_without_bare_math(&Content::Table(Arc::new(raw.clone()))));
        raw.children.push(frac.clone());
        raw.resolved = Some(Arc::new(TableGridResolved {
            header_groups: Vec::new(),
            cells: vec![Content::text("resolved")],
            num_cols: 1,
            num_rows: 1,
            header_rows: 0,
            footer_start: 1,
            complete: true,
        }));
        assert!(plain_without_bare_math(&Content::Table(Arc::new(raw.clone()))));
        raw.caption = Some(frac);
        assert!(!plain_without_bare_math(&Content::Table(Arc::new(raw))));
    }
    #[test]
    fn issue_8954_ordered_raw_items_do_not_bypass_math_certificate() {
        let Content::Table(base) = Content::table(vec![], vec![], vec![]) else {
            panic!("table")
        };
        let mut table = base.as_ref().clone();
        table.items = Some(vec![Content::table_header(
            Content::equation(Content::text("x"), false),
            true,
        )]);
        assert!(!plain_without_bare_math(&Content::Table(Arc::new(table))));
    }
}
