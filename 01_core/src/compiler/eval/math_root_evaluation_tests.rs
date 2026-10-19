//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_root_evaluation_tests.md
//! @prompt-hash 1ce763f6
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
        _ => None,
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
