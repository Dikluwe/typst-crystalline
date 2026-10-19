//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_cases_evaluation_tests.md
//! @prompt-hash 41fd0597
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
    FileId::from_raw(NonZeroU16::new(1430).unwrap())
}

fn evaluate(text: &str) -> SourceResult<Module> {
    let world = TestWorld::new(text);
    eval_for_test(&world, &world.source)
}

fn find_cases(content: &Content) -> Option<&Content> {
    match content {
        Content::MathCases(_) => Some(content),
        Content::Equation(elem) => find_cases(&elem.body),
        Content::Styled(styled) => find_cases(&styled.body),
        Content::MathDelimited(elem) => find_cases(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_cases)
        }
        _ => None,
    }
}

fn evaluated_cases(source: &str) -> Content {
    let module =
        evaluate(source).unwrap_or_else(|errors| panic!("{source:?}: {errors:?}"));
    let root = module.content().expect("module content");
    find_cases(root)
        .unwrap_or_else(|| panic!("missing MathCases in {source:?}: {root:?}"))
        .clone()
}

fn direct_result(args: Args) -> SourceResult<Value> {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    let Some(Value::Func(function)) = module.scope().get("cases") else {
        panic!("missing callable math.cases")
    };
    function.native_fn_addr().unwrap()(
        &mut EvalContext::new(),
        &args,
        &TestWorld::new(""),
        id(),
    )
}

fn direct_math_content(children: Vec<Content>) -> Content {
    let args = Args::positional(children.into_iter().map(Value::Content).collect());
    let Value::Content(content) = direct_result(args).unwrap() else {
        panic!("math.cases content")
    };
    content
}

#[test]
fn p1430_delimitadores_malformados_sao_rejeitados() {
    for (value, fragment) in [
        (Value::Array(vec![Value::Str("[".into())]), "exactly two"),
        (
            Value::Array(vec![
                Value::Str("[".into()),
                Value::Str("]".into()),
                Value::None,
            ]),
            "exactly two",
        ),
        (Value::Str("[[".into()), "single"),
        (Value::Str("a".into()), "invalid delimiter"),
    ] {
        let mut args = Args::positional(vec![]);
        args.named.insert("delim".into(), value);
        let errors = direct_result(args).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "expected {fragment:?}: {errors:?}"
        );
    }
}

fn math_body(source: &str) -> Content {
    let module = evaluate(source).unwrap();
    let root = module.content().unwrap();
    let Content::Equation(equation) = root else {
        panic!("expected equation from {source:?}: {root:?}")
    };
    equation.body.clone()
}

#[test]
fn p1430_bare_converge_e_preserva_alignpoint_no_carrier() {
    let bare = evaluated_cases("$cases(a & b, c)$");
    assert_eq!(
        repr::repr_content(&bare),
        "cases(children: (a & b, c))",
        "align point was pre-resolved before the layout consumer"
    );
    let qualified = direct_math_content(vec![math_body("$a & b$"), math_body("$c$")]);
    assert_eq!(bare, qualified);
}

#[test]
fn p1430_ordem_de_avaliacao_propaga_primeiro_erro() {
    for (source, expected, forbidden) in [
        (
            "$cases(#panic(\"CHILD-FIRST\"), delim: #panic(\"DELIM-SECOND\"))$",
            "CHILD-FIRST",
            "DELIM-SECOND",
        ),
        (
            "$cases(delim: #panic(\"DELIM-FIRST\"), #panic(\"CHILD-SECOND\"))$",
            "DELIM-FIRST",
            "CHILD-SECOND",
        ),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert_eq!(
            errors.len(),
            1,
            "occurrence was retried or error duplicated: {errors:?}"
        );
        let messages = errors
            .iter()
            .map(|error| error.message.as_str())
            .collect::<Vec<_>>()
            .join(" | ");
        assert!(messages.contains(expected), "{source}: {messages}");
        assert!(
            !messages.contains(forbidden),
            "later occurrence was evaluated: {messages}"
        );
    }
}

#[test]
fn p1430_avaliacao_opaca_falha_fechado() {
    let classify = |observable: bool| {
        if observable {
            ("PRESERVED", true, false)
        } else {
            ("UNKNOWN_EVALUATION_OBSERVATION", false, false)
        }
    };
    assert_eq!(classify(false), ("UNKNOWN_EVALUATION_OBSERVATION", false, false));
    assert_eq!(classify(true), ("PRESERVED", true, false));
}

#[test]
fn p1430_cascata_nao_muta_carrier_e_explicito_prevalece() {
    let omitted = evaluated_cases("$cases(x, y)$");
    let cascaded = evaluated_cases(
        "#set math.cases(delim: \"[\", reverse: true, gap: 1em); $cases(x, y)$",
    );
    assert_eq!(omitted, cascaded);
    assert_eq!(repr::repr_content(&cascaded), "cases(children: (x, y))");

    let with_set = evaluated_cases(
        "#set math.cases(delim: \"[\", reverse: true, gap: 1em); \
         $cases(delim: #none, reverse: #false, gap: #150%, x, y)$",
    );
    let without_set =
        evaluated_cases("$cases(delim: #none, reverse: #false, gap: #150%, x, y)$");
    assert_eq!(with_set, without_set);
    assert_eq!(
        repr::repr_content(&with_set),
        "cases(delim: (none, none), reverse: false, gap: 150% + 0pt, children: (x, y))"
    );
}

#[test]
fn p1430_bare_rejeita_invalidos_sem_ignorar_ocorrencias() {
    for (source, fragment) in [
        ("$cases(#1)$", "expected content, found integer"),
        ("$cases(foo: #1, x)$", "unexpected argument: foo"),
        ("$cases(reverse: #false, reverse: #true, x)$", "duplicate argument: reverse"),
        ("$cases(gap: #1, x)$", "expected relative length, found integer"),
    ] {
        let errors = evaluate(source).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}
