//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_root_evaluation_tests.md
//! @prompt-hash 00d2cb18
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes da sintaxe bare e ObservationEq P1426.

use super::*;
use crate::entities::font_book::FontBook;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use std::num::NonZeroU16;

struct TestWorld {
    library: Library,
    book: FontBook,
    source: Source,
}

impl TestWorld {
    fn new(text: &str) -> Self {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        Self {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id, text.to_string()),
        }
    }
}

impl World for TestWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Ok(self.source.clone())
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        None
    }
}

fn evaluate(text: &str) -> SourceResult<Module> {
    let world = TestWorld::new(text);
    crate::compiler::eval::tests::eval_for_test(&world, &world.source)
}

fn find_root(
    content: &Content,
) -> Option<&crate::entities::elements::math_root::MathRootElem> {
    match content {
        Content::MathRoot(elem) => Some(elem),
        Content::Equation(elem) => find_root(&elem.body),
        Content::MathSequence(items) | Content::Sequence(items) => {
            items.iter().find_map(find_root)
        }
        // These variants are outside this fixture observer's allowed paths.
        // Payloads may contain targets; the observer deliberately does not descend.
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
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
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
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
        | Content::Styled(_)
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
        | Content::Dynamic(_) => None,
    }
}

fn evaluated_root(
    text: &str,
) -> std::sync::Arc<crate::entities::elements::math_root::MathRootElem> {
    let module =
        evaluate(text).unwrap_or_else(|error| panic!("fixture failed: {error:?}"));
    find_root(module.content().expect("module content"))
        .unwrap_or_else(|| panic!("missing MathRoot in {text:?}"))
        .to_owned()
        .into()
}

#[test]
fn p1426_sintaxe_bare_preserva_vazio_none_e_conteudo() {
    let square = evaluated_root("$sqrt(x)$");
    assert!(square.index.is_none());
    assert!(!square.index_explicit);

    let empty = evaluated_root("$root(, x)$");
    assert!(matches!(empty.index, Some(Content::Empty)));
    assert!(empty.index_explicit);

    let evaluated_none = evaluated_root("$root(#none, x)$");
    assert!(evaluated_none.index.is_none());
    assert!(evaluated_none.index_explicit);

    let indexed = evaluated_root("$root(3, x)$");
    assert!(indexed.index.is_some());
    assert!(indexed.index_explicit);
}

#[test]
fn p1426_root_bare_exige_slots_e_preserva_presenca() {
    let error = evaluate("$root(x)$").unwrap_err();
    assert_eq!(error.len(), 1);
    assert!(error[0].message.contains("missing argument: radicand"));

    let empty = evaluated_root("$root(, x)$");
    let none = evaluated_root("$root(#none, x)$");
    assert!(matches!(empty.index, Some(Content::Empty)));
    assert!(none.index.is_none());
    assert!(empty.index_explicit && none.index_explicit);
}

#[test]
fn p1426_observation_relation_observa_presenca_do_indice() {
    let Content::MathRoot(absent) = Content::math_root(None, Content::text("x"), false)
    else {
        unreachable!()
    };
    let Content::MathRoot(absent_twin) =
        Content::math_root(None, Content::text("x"), false)
    else {
        unreachable!()
    };
    let Content::MathRoot(explicit_none) =
        Content::math_root(None, Content::text("x"), true)
    else {
        unreachable!()
    };

    assert_eq!(absent.observation_relation(&absent_twin), ObservationRelation::Same);
    assert_eq!(
        absent.observation_relation(&explicit_none),
        ObservationRelation::Different
    );
    assert_eq!(
        explicit_none.observation_relation(&absent),
        ObservationRelation::Different
    );
}

#[test]
fn v16_b_root_observer_returns_outer_target_without_descending() {
    let inner = Content::math_root(None, Content::text("first"), false);
    let outer = Content::math_root(None, inner, false);
    let Content::MathRoot(expected) = &outer else { panic!("target fixture") };
    let actual = find_root(&outer).expect("direct target");
    assert!(std::ptr::eq(actual, expected.as_ref()));
}

#[test]
fn v16_b_root_observer_follows_only_allowed_wrapper_paths() {
    let target = Content::math_root(None, Content::text("first"), false);
    for wrapped in [
        Content::equation(target.clone(), false),
        Content::Sequence(vec![target.clone()].into()),
        Content::MathSequence(vec![target.clone()].into()),
        Content::equation(
            Content::Sequence(
                vec![Content::MathSequence(vec![target.clone()].into())].into(),
            ),
            false,
        ),
    ] {
        assert_eq!(find_root(&wrapped), find_root(&target), "{wrapped:?}");
    }
}

#[test]
fn v16_b_root_observer_preserves_first_match_and_sequence_order() {
    let target = Content::math_root(None, Content::text("first"), false);
    let later = Content::math_root(None, Content::text("later"), false);
    assert_ne!(find_root(&target), find_root(&later));
    for sequence in [
        Content::Sequence(
            vec![
                Content::strong(later.clone()),
                Content::text("miss"),
                Content::equation(target.clone(), false),
                later.clone(),
            ]
            .into(),
        ),
        Content::MathSequence(
            vec![
                Content::math_styled(None, None, None, later.clone(), None),
                Content::Empty,
                Content::Sequence(vec![target.clone()].into()),
                later.clone(),
            ]
            .into(),
        ),
    ] {
        assert_eq!(find_root(&sequence), find_root(&target), "{sequence:?}");
    }
    for reversed in [
        Content::Sequence(vec![later.clone(), target.clone()].into()),
        Content::MathSequence(vec![later.clone(), target].into()),
    ] {
        assert_eq!(find_root(&reversed), find_root(&later), "{reversed:?}");
    }
}

#[test]
fn v16_b_root_observer_stops_at_blocked_wrappers_with_target_decoys() {
    let target = Content::math_root(None, Content::text("first"), false);
    for blocked in [
        Content::strong(target.clone()),
        Content::math_styled(None, None, None, target.clone(), None),
        Content::styled(target.clone(), Default::default()),
        Content::math_delimited('(', target.clone(), ')'),
    ] {
        assert!(find_root(&blocked).is_none(), "unexpected descent: {blocked:?}");
        let sequence = Content::Sequence(vec![blocked, target.clone()].into());
        assert_eq!(find_root(&sequence), find_root(&target));
    }
    for miss in [
        Content::Empty,
        Content::text("not a target"),
        Content::Sequence(vec![].into()),
        Content::MathSequence(vec![].into()),
        Content::equation(Content::text("no target inside"), false),
    ] {
        assert!(find_root(&miss).is_none(), "{miss:?}");
    }
}
