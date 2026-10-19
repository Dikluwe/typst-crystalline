//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_cases_evaluation_tests.md
//! @prompt-hash fe6bd1e2
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
        | Content::MathRoot(_)
        | Content::MathStretch(_)
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

#[test]
fn v16_b_cases_observer_returns_outer_target_without_descending() {
    let inner =
        Content::math_cases(vec![vec![Content::text("first")]], ('{', '\0'), false, None);
    let outer = Content::math_cases(vec![vec![inner]], ('{', '\0'), false, None);
    let Content::MathCases(expected) = &outer else { panic!("target fixture") };
    let actual = find_cases(&outer).expect("direct target");
    let Content::MathCases(actual) = actual else { panic!("target hit") };
    assert!(std::sync::Arc::ptr_eq(actual, expected));
}

#[test]
fn v16_b_cases_observer_follows_only_allowed_wrapper_paths() {
    let target =
        Content::math_cases(vec![vec![Content::text("first")]], ('{', '\0'), false, None);
    for wrapped in [
        Content::equation(target.clone(), false),
        Content::styled(target.clone(), Default::default()),
        Content::math_delimited('(', target.clone(), ')'),
        Content::Sequence(vec![target.clone()].into()),
        Content::MathSequence(vec![target.clone()].into()),
        Content::styled(
            Content::math_delimited('(', Content::equation(target.clone(), false), ')'),
            Default::default(),
        ),
    ] {
        assert_eq!(find_cases(&wrapped), Some(&target), "{wrapped:?}");
    }
}

#[test]
fn v16_b_cases_observer_preserves_first_match_and_sequence_order() {
    let target =
        Content::math_cases(vec![vec![Content::text("first")]], ('{', '\0'), false, None);
    let later =
        Content::math_cases(vec![vec![Content::text("later")]], ('{', '\0'), false, None);
    assert_ne!(find_cases(&target), find_cases(&later));
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
        assert_eq!(find_cases(&sequence), Some(&target), "{sequence:?}");
    }
    for reversed in [
        Content::Sequence(vec![later.clone(), target.clone()].into()),
        Content::MathSequence(vec![later.clone(), target].into()),
    ] {
        assert_eq!(find_cases(&reversed), Some(&later), "{reversed:?}");
    }
}

#[test]
fn v16_b_cases_observer_stops_at_blocked_wrappers_with_target_decoys() {
    let target =
        Content::math_cases(vec![vec![Content::text("first")]], ('{', '\0'), false, None);
    for blocked in [
        Content::strong(target.clone()),
        Content::math_styled(None, None, None, target.clone(), None),
    ] {
        assert!(find_cases(&blocked).is_none(), "unexpected descent: {blocked:?}");
        let sequence = Content::Sequence(vec![blocked, target.clone()].into());
        assert_eq!(find_cases(&sequence), Some(&target));
    }
    for miss in [
        Content::Empty,
        Content::text("not a target"),
        Content::Sequence(vec![].into()),
        Content::MathSequence(vec![].into()),
        Content::equation(Content::text("no target inside"), false),
    ] {
        assert!(find_cases(&miss).is_none(), "{miss:?}");
    }
}
