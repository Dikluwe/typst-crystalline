//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_frac_evaluation_tests.md
//! @prompt-hash 855bcd2f
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes de avaliação e observação `math.frac` P1427.

use super::*;
use crate::entities::elements::math_frac::{MathFracElem, MathFracStyle};
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

fn find_frac(content: &Content) -> Option<&MathFracElem> {
    match content {
        Content::MathFrac(elem) => Some(elem),
        Content::Equation(elem) => find_frac(&elem.body),
        Content::MathDelimited(elem) => find_frac(&elem.body),
        Content::Styled(styled) => find_frac(&styled.body),
        Content::MathSequence(items) | Content::Sequence(items) => {
            items.iter().find_map(find_frac)
        }
        _ => None,
    }
}

fn evaluated_frac(source: &str) -> std::sync::Arc<MathFracElem> {
    let module =
        evaluate(source).unwrap_or_else(|error| panic!("fixture {source:?}: {error:?}"));
    find_frac(module.content().expect("module content"))
        .unwrap_or_else(|| panic!("missing MathFrac in {source:?}"))
        .clone()
        .into()
}

fn frac(style: Option<MathFracStyle>, nb: bool, db: bool, line: bool) -> MathFracElem {
    MathFracElem {
        num: Content::text("a"),
        den: Content::text("b"),
        style,
        num_deparenthesized: nb,
        denom_deparenthesized: db,
        line,
    }
}

#[test]
fn p1427_set_style_valida_e_respeita_precedencia() {
    let cascaded = evaluate("#set math.frac(style: \"horizontal\"); $a/b$").unwrap();
    let stored = find_frac(cascaded.content().unwrap()).unwrap();
    assert_eq!(stored.style, None, "cascade must not rewrite explicitness");

    let explicit = evaluated_frac(
        "#set math.frac(style: \"horizontal\"); #math.frac([a], [b], style: \"vertical\")",
    );
    assert_eq!(explicit.style, Some(MathFracStyle::Vertical));

    for source in
        ["#set math.frac(style: \"diagonal\"); $a/b$", "#set math.frac(style: 1); $a/b$"]
    {
        assert!(evaluate(source).is_err(), "invalid set survived: {source}");
    }
}

#[test]
fn p1427_frac_bare_e_slash_convergem_no_carrier() {
    let bare = evaluated_frac("$frac(a,b)$");
    let slash = evaluated_frac("$a/b$");
    let qualified = evaluated_frac("#math.frac([a], [b])");
    for elem in [&bare, &slash, &qualified] {
        assert_eq!(elem.style, None);
        assert!(!elem.num_deparenthesized && !elem.denom_deparenthesized);
        assert!(elem.line);
    }
    assert_eq!(bare, slash);
    assert_eq!(slash.num.plain_text(), qualified.num.plain_text());
    assert_eq!(slash.den.plain_text(), qualified.den.plain_text());

    for (source, fragment) in [
        ("$frac(a)$", "missing argument: denom"),
        ("$frac(a,b,c)$", "unexpected argument"),
        ("$frac(a,b,foo: 1)$", "unexpected argument"),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}

#[test]
fn p1427_slash_associa_esquerda_e_preserva_parenteses_causais() {
    let chained = evaluated_frac("$a/b/c$");
    assert!(matches!(chained.num, Content::MathFrac(_)));
    assert!(!matches!(chained.den, Content::MathFrac(_)));

    let numerator = evaluated_frac("$(a)/b$");
    assert_eq!(
        (numerator.num_deparenthesized, numerator.denom_deparenthesized),
        (true, false)
    );
    let denominator = evaluated_frac("$a/(b)$");
    assert_eq!(
        (denominator.num_deparenthesized, denominator.denom_deparenthesized),
        (false, true)
    );
    let double = evaluated_frac("$((a))/b$");
    assert!(double.num_deparenthesized);
    assert!(
        double.num.plain_text().contains('('),
        "inner pair must remain: {:?}",
        double.num
    );

    for source in ["$frac((a),b)$", "#math.frac([(a)], [b])"] {
        let elem = evaluated_frac(source);
        assert!(!elem.num_deparenthesized && !elem.denom_deparenthesized, "{source}");
    }
}

#[test]
fn p1427_observation_relation_compara_todos_os_campos() {
    let base = frac(None, false, false, true);
    assert_eq!(base.observation_relation(&base), ObservationRelation::Same);
    for changed in [
        frac(Some(MathFracStyle::Vertical), false, false, true),
        frac(None, true, false, true),
        frac(None, false, true, true),
        frac(None, false, false, false),
    ] {
        assert_eq!(base.observation_relation(&changed), ObservationRelation::Different);
        assert_eq!(changed.observation_relation(&base), ObservationRelation::Different);
    }
}
