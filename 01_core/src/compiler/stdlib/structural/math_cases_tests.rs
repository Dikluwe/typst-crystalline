//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math_cases_tests.md
//! @prompt-hash c5da0ed2
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes da convergência pública `math.cases` (P1430).

use super::*;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::func::Func;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;

#[derive(Default)]
struct NullWorld {
    library: Library,
    book: FontBook,
}

impl World for NullWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        file_id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
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

fn file_id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1430).unwrap())
}

fn math_func(name: &str) -> Func {
    let Value::Module(module) = make_math_module() else {
        panic!("math must be a module")
    };
    match module.scope().get(name) {
        Some(Value::Func(func)) => func.clone(),
        Some(other) => panic!("math.{name} must be callable, got {other:?}"),
        None => panic!("missing callable math.{name}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    let native = math_func("cases")
        .native_fn_addr()
        .expect("math.cases must be native");
    native(&mut EvalContext::new(), &args, &NullWorld::default(), file_id())
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn mv(text: &str) -> Value {
    Value::Content(Content::MathText(text.into()))
}

fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}

fn repr(args: Args) -> String {
    let Value::Content(content) = call(args).unwrap() else {
        panic!("math.cases must return content")
    };
    assert!(matches!(content, Content::MathCases(_)), "wrong carrier: {content:?}");
    repr_content(&content)
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else {
        panic!("math.cases must return content")
    };
    assert!(matches!(content, Content::MathCases(_)), "wrong carrier: {content:?}");
    content
}

fn diagnostic(args: Args) -> String {
    let errors = call(args).unwrap_err();
    assert_eq!(errors.len(), 1, "one primary diagnostic expected: {errors:?}");
    errors[0].message.clone()
}

fn duplicate(name: &str, first: Value, second: Value) -> Args {
    Args::from_occurrences(
        Span::detached(),
        vec![
            ArgOccurrence {
                name: None,
                value: cv("x"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some(name.into()),
                value: first,
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some(name.into()),
                value: second,
                span: Span::detached(),
                value_span: Span::detached(),
            },
        ],
    )
}

#[test]
fn p1430_binding_aridade_variadica_e_casts_de_children() {
    assert_eq!(math_func("cases").name(), Some("cases"));
    assert_eq!(repr(Args::positional(vec![])), "cases(children: ())");
    assert_eq!(
        repr(Args::positional(vec![Value::Str("x".into())])),
        "cases(children: ([x],))"
    );
    assert_eq!(
        repr(Args::positional(vec![cv("x"), Value::None, cv("y")])),
        "cases(children: ([x], [], [y]))"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
}

#[test]
fn p1430_delimitadores_opcionais_gap_e_explicitude() {
    assert_eq!(
        repr(named(Args::positional(vec![cv("x")]), "delim", Value::Str("[".into()),)),
        "cases(delim: (\"[\", \"]\"), children: ([x],))"
    );
    assert_eq!(
        repr(named(Args::positional(vec![cv("x")]), "delim", Value::None,)),
        "cases(delim: (none, none), children: ([x],))"
    );
    assert_eq!(
        repr(named(
            Args::positional(vec![cv("x")]),
            "delim",
            Value::Array(vec![Value::Str("[".into()), Value::None]),
        )),
        "cases(delim: (\"[\", none), children: ([x],))"
    );

    let all_explicit = Args::from_occurrences(
        Span::detached(),
        vec![
            ArgOccurrence {
                name: Some("gap".into()),
                value: Value::Relative(Rel::<Length>::from_percent(150.0)),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some("reverse".into()),
                value: Value::Bool(false),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some("delim".into()),
                value: Value::Str("{".into()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: cv("x"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        ],
    );
    assert_eq!(
        repr(all_explicit),
        "cases(delim: (\"{\", \"}\"), reverse: false, gap: 150% + 0pt, children: ([x],))"
    );

    let absent = content(Args::positional(vec![cv("x")]));
    let explicit_defaults = content(named(
        named(
            named(Args::positional(vec![cv("x")]), "delim", Value::Str("{".into())),
            "reverse",
            Value::Bool(false),
        ),
        "gap",
        Value::Length(Length::em(0.2)),
    ));
    assert_ne!(absent, explicit_defaults, "explicit default bits collapsed");
    assert_ne!(hash(&absent), hash(&explicit_defaults), "explicit bits absent from hash");
}

#[test]
fn p1430_rejeicoes_atribuiveis_e_duplicados() {
    for (args, expected) in [
        (
            named(Args::positional(vec![]), "children", cv("x")),
            "unexpected argument: children",
        ),
        (
            named(Args::positional(vec![]), "foo", Value::Int(1)),
            "unexpected argument: foo",
        ),
        (
            named(Args::positional(vec![]), "delim", Value::Int(1)),
            "expected array, none, symbol, or string, found integer",
        ),
        (
            named(Args::positional(vec![]), "reverse", Value::Int(1)),
            "expected boolean, found integer",
        ),
        (
            named(Args::positional(vec![]), "gap", Value::Int(1)),
            "expected relative length, found integer",
        ),
        (
            duplicate("delim", Value::Str("{".into()), Value::None),
            "duplicate argument: delim",
        ),
        (
            duplicate("reverse", Value::Bool(false), Value::Bool(true)),
            "duplicate argument: reverse",
        ),
        (
            duplicate(
                "gap",
                Value::Length(Length::em(0.2)),
                Value::Length(Length::em(1.0)),
            ),
            "duplicate argument: gap",
        ),
    ] {
        assert_eq!(diagnostic(args), expected);
    }
}

struct EvalWorld {
    library: Library,
    book: FontBook,
    source: Source,
}

impl EvalWorld {
    fn new(text: &str) -> Self {
        Self {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(file_id(), text.into()),
        }
    }
}

impl World for EvalWorld {
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

fn eval(text: &str) -> SourceResult<crate::entities::module::Module> {
    let world = EvalWorld::new(text);
    crate::compiler::eval::eval_for_test(&world, &world.source)
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
        Content::Source(..)
        | Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => None,
    }
}

fn eval_cases(text: &str) -> Content {
    let module =
        eval(text).unwrap_or_else(|errors| panic!("fixture {text:?}: {errors:?}"));
    let root = module.content().expect("module content");
    find_cases(root)
        .unwrap_or_else(|| panic!("missing MathCases in {text:?}: {root:?}"))
        .clone()
}

#[test]
fn p1430_rotas_bare_e_qualificada_convergem() {
    let direct = content(Args::positional(vec![mv("x"), mv("y")]));
    let bare = eval_cases("$cases(x, y)$");
    assert_eq!(bare, direct);
    assert_eq!(repr_content(&bare), "cases(children: (x, y))");

    let qualified =
        eval_cases("#math.cases([x], [y], delim: \"[\", reverse: true, gap: 1em)");
    assert_eq!(
        repr_content(&qualified),
        "cases(delim: (\"[\", \"]\"), reverse: true, gap: 0% + 1em, children: ([x], [y]))"
    );
}

#[test]
fn p1430_cascata_nao_fabrica_explicitude_e_explicit_override() {
    let defaults = eval_cases("$cases(x, y)$");
    let cascaded = eval_cases(
        "#set math.cases(delim: \"[\", reverse: true, gap: 1em); $cases(x, y)$",
    );
    assert_eq!(repr_content(&cascaded), "cases(children: (x, y))");
    assert_eq!(
        defaults, cascaded,
        "cascade fabricated values or explicit bits in the public carrier"
    );

    let override_cascaded = eval_cases(
        "#set math.cases(delim: \"[\", reverse: true, gap: 1em); \
         $cases(delim: #none, reverse: #false, gap: #150%, x, y)$",
    );
    let override_direct =
        eval_cases("$cases(delim: #none, reverse: #false, gap: #150%, x, y)$");
    assert_eq!(override_cascaded, override_direct);
    assert_eq!(
        repr_content(&override_cascaded),
        "cases(delim: (none, none), reverse: false, gap: 150% + 0pt, children: (x, y))"
    );

    for (source, fragment) in [
        ("#set math.cases(foo: 1); $cases(x)$", "unknown property"),
        ("#set math.cases(delim: 1); $cases(x)$", "array, none, symbol, or string"),
        ("#set math.cases(reverse: 1); $cases(x)$", "boolean"),
        ("#set math.cases(gap: 1); $cases(x)$", "relative length"),
    ] {
        let errors = eval(source).unwrap_err();
        assert!(
            errors.iter().any(|error| error.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}

fn hash(value: &Content) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

#[test]
fn p1430_carrier_clone_hash_e_walkers_preservam_causalidade() {
    let input = content(named(
        named(
            named(
                Args::positional(vec![cv("x"), cv("y")]),
                "delim",
                Value::Array(vec![Value::Str("[".into()), Value::None]),
            ),
            "reverse",
            Value::Bool(true),
        ),
        "gap",
        Value::Relative(Rel::from_percent(150.0)),
    ));
    assert_eq!(input.clone(), input);
    assert_eq!(hash(&input.clone()), hash(&input));

    let Content::MathCases(elem) = &input else {
        panic!("exactly one MathCases expected")
    };
    assert_eq!(elem.plain_text(), "x, y");
    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        Ok((child.plain_text() == "x").then(|| Content::text("z")))
    };
    let mapped = elem.map_content(&mut transform).unwrap();
    assert_eq!(
        repr_content(&mapped),
        "cases(delim: (\"[\", none), reverse: true, gap: 150% + 0pt, children: ([z], [y]))"
    );
    let mut terminal = |_: &str| "mutated".to_string();
    assert_eq!(elem.map_text(&mut terminal), input);
}

#[test]
fn batch_f_cases_finder_observes_actual_fixture_and_rejects_absence() {
    let module = eval("$cases(x,y)$").unwrap();
    let found = find_cases(module.content().unwrap()).expect("published fixture target");
    assert!(matches!(found, Content::MathCases(_)));
    assert!(find_cases(&Content::text("absent")).is_none());
}

#[test]
fn batch_f_cases_finder_keeps_order_and_opaque_descendant_boundary() {
    let target = |text| {
        let Value::Content(content) = call(Args::positional(vec![cv(text)])).unwrap()
        else {
            panic!("target content")
        };
        content
    };
    let first = target("first");
    let second = target("second");
    let opaque = Content::strong(first.clone());
    assert!(find_cases(&opaque).is_none());
    let sequence = Content::sequence(vec![opaque, second.clone(), first]);
    let found = find_cases(&sequence).expect("second is first visible target");
    assert_eq!(repr_content(found), repr_content(&second));
}
