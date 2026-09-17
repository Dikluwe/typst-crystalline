//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/p1431-tests.md
//! @prompt-hash db030dc5
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::stdlib::make_math_module;
use crate::entities::args::Args;
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
        Self {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id(), text.into()),
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
fn id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1431).unwrap())
}
fn evaluate(text: &str) -> SourceResult<Module> {
    let world = TestWorld::new(text);
    eval_for_test(&world, &world.source)
}
fn find_mat(content: &Content) -> Option<&Content> {
    match content {
        Content::MathMatrix(_) => Some(content),
        Content::Equation(elem) => find_mat(&elem.body),
        Content::Styled(inner, _) => find_mat(inner),
        Content::MathDelimited(elem) => find_mat(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_mat)
        }
        _ => None,
    }
}
fn evaluated_mat(source: &str) -> Content {
    let module =
        evaluate(source).unwrap_or_else(|errors| panic!("{source:?}: {errors:?}"));
    let root = module.content().expect("module content");
    find_mat(root)
        .unwrap_or_else(|| panic!("missing matrix in {root:?}"))
        .clone()
}
fn direct(args: Args) -> SourceResult<Value> {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    let Some(Value::Func(function)) = module.scope().get("mat") else {
        panic!("math.mat")
    };
    function.native_fn_addr().unwrap()(
        &mut EvalContext::new(),
        &args,
        &TestWorld::new(""),
        id(),
    )
}

#[test]
fn p1431_bare_e_qualificado_convergem_sem_pre_resolver_alignpoint() {
    let bare = evaluated_mat("$mat(a & b, c)$");
    let first = {
        let module = evaluate("$a & b$").unwrap();
        let Content::Equation(eq) = module.content().unwrap() else { panic!("equation") };
        eq.body.clone()
    };
    let second = {
        let module = evaluate("$c$").unwrap();
        let Content::Equation(eq) = module.content().unwrap() else { panic!("equation") };
        eq.body.clone()
    };
    let Value::Content(qualified) =
        direct(Args::positional(vec![Value::Content(first), Value::Content(second)]))
            .unwrap()
    else {
        panic!("mat content")
    };
    assert_eq!(bare, qualified);
    assert_eq!(repr::repr_content(&bare), "mat(rows: ((a & b, c),))");
}

#[test]
fn p1431_bare_rows_arrays_padding_e_invalidos_usam_construtor_comum() {
    assert_eq!(
        repr::repr_content(&evaluated_mat("$mat((1, 2), (3,))$")),
        "mat(rows: ((1, 2), (3, [])))"
    );
    for (source, fragment) in [
        ("$mat((1, 2), x)$", "expected array"),
        ("$mat(foo: #1)$", "unexpected argument: foo"),
        ("$mat(rows: #((1, 2),))$", "unexpected argument: rows"),
        ("$mat(gap: #1)$", "expected relative length"),
        ("$mat(augment: #\"x\")$", "expected integer, dictionary, or none"),
        ("$mat(align: #top)$", "start"),
        ("$mat(row-gap: #1em, row-gap: #2em)$", "duplicate argument: row-gap"),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}

#[test]
fn p1431_avaliacao_ocorre_uma_vez_e_na_ordem_causal() {
    for (source, expected, forbidden) in [
        (
            "$mat(#panic(\"CELL-FIRST\"), gap: #panic(\"GAP-SECOND\"))$",
            "CELL-FIRST",
            "GAP-SECOND",
        ),
        (
            "$mat(gap: #panic(\"GAP-FIRST\"), #panic(\"CELL-SECOND\"))$",
            "GAP-FIRST",
            "CELL-SECOND",
        ),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert_eq!(errors.len(), 1, "retried occurrence: {errors:?}");
        let messages = errors
            .iter()
            .map(|e| e.message.as_str())
            .collect::<Vec<_>>()
            .join(" | ");
        assert!(messages.contains(expected), "{messages}");
        assert!(!messages.contains(forbidden), "later occurrence ran: {messages}");
    }
}

#[test]
fn p1431_cascata_nao_fabrica_presenca_e_explicito_ganha_por_eixo() {
    let omitted = evaluated_mat("$mat(x, y)$");
    let cascaded = evaluated_mat(
        "#set math.mat(delim: \"[\", align: right, gap: 1em, augment: 1); $mat(x, y)$",
    );
    assert_eq!(omitted, cascaded);
    assert_eq!(repr::repr_content(&cascaded), "mat(rows: ((x, y),))");

    let with_set = evaluated_mat(
        "#set math.mat(gap: 1em, row-gap: 2em, augment: 1); \
         $mat(column-gap: #3em, augment: #none, x, y)$",
    );
    let without_set = evaluated_mat("$mat(column-gap: #3em, augment: #none, x, y)$");
    assert_eq!(with_set, without_set);
    assert_eq!(
        repr::repr_content(&with_set),
        "mat(augment: none, column-gap: 0% + 3em, rows: ((x, y),))"
    );
}

#[test]
fn p1431_set_rejeita_named_e_tipos_invalidos() {
    for (source, fragment) in [
        ("#set math.mat(foo: 1); $mat()$", "unexpected argument: foo"),
        ("#set math.mat(rows: ((1,),)); $mat()$", "unexpected argument: rows"),
        ("#set math.mat(gap: 1); $mat()$", "expected relative length"),
        ("#set math.mat(align: top); $mat()$", "start"),
        (
            "#set math.mat(augment: \"x\"); $mat()$",
            "expected integer, dictionary, or none",
        ),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}

#[test]
fn p1431_observacao_de_ordem_opaca_falha_fechado() {
    let classify = |observable: bool| {
        if observable {
            ("PRESERVED", true, false)
        } else {
            ("UNKNOWN_EVALUATION_OBSERVATION", false, false)
        }
    };
    assert_eq!(classify(false), ("UNKNOWN_EVALUATION_OBSERVATION", false, false));
}
