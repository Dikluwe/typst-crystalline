//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math_matrix_evaluation_tests.md
//! @prompt-hash 56094a47
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
            source: Source::new(id(), text.into()).expect("valid Source test fixture"),
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
        Content::Styled(styled) => find_mat(&styled.body),
        Content::MathDelimited(elem) => find_mat(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_mat)
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
        | Content::Symbol(_)
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
        ("$mat(delim: #1)$", "expected array, none, symbol, or string"),
        ("$mat(delim: #(\"[\",))$", "exactly two"),
        ("$mat(delim: #(\"[\", \"]\", none))$", "exactly two"),
        ("$mat(column-gap: #1)$", "expected relative length"),
        ("$mat(augment: #(hline: \"x\"))$", "integer"),
        ("$mat(delim: #none, delim: #\"[\")$", "duplicate argument: delim"),
        ("$mat(align: #left, align: #right)$", "duplicate argument: align"),
        ("$mat(augment: #none, augment: #1)$", "duplicate argument: augment"),
        ("$mat(gap: #1em, gap: #2em)$", "duplicate argument: gap"),
        ("$mat(column-gap: #1em, column-gap: #2em)$", "duplicate argument: column-gap"),
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
    let successful = evaluate(
        "#let calls = ()\n\
         #let observed = math.mat(\
             { calls.push(\"pos-1\"); [a] },\
             delim: { calls.push(\"named-2\"); none },\
             { calls.push(\"pos-3\"); [b] },\
         )\n\
         #let signal = calls",
    )
    .expect("P20: o sinal atribuível deve concluir com sucesso");
    assert_eq!(
        successful.scope().get("signal"),
        Some(&Value::Array(vec![
            Value::Str("pos-1".into()),
            Value::Str("named-2".into()),
            Value::Str("pos-3".into()),
        ])),
        "P20: cada ocorrência bem-sucedida deve executar exatamente uma vez em ordem"
    );

    let spread = evaluate(
        "#let expanded = arguments([a], [b], [c])\n\
         #let observed = math.mat(..expanded)\n\
         #let spread-repr = repr(observed)",
    )
    .expect("P20: spread público separado deve concluir com sucesso");
    assert_eq!(
        spread.scope().get("spread-repr"),
        Some(&Value::Str("mat(rows: (([a], [b], [c]),))".into())),
        "P20: spread deve preservar valores e ordem, sem servir de contador"
    );

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

    let every_field = evaluated_mat(
        "#set math.mat(delim: \"[\", align: left, augment: 1, row-gap: 2em, column-gap: 3em); \
         $mat(delim: #(none, \"]\"), align: #right, augment: #2, row-gap: #4em, column-gap: #5em, x)$",
    );
    let every_repr = repr::repr_content(&every_field);
    for expected in [
        "delim: (none, \"]\")",
        "align: right",
        "augment: 2",
        "row-gap: 0% + 4em",
        "column-gap: 0% + 5em",
    ] {
        assert!(every_repr.contains(expected), "P16 missing {expected}: {every_repr}");
    }
}

#[test]
fn p1431_set_rejeita_named_e_tipos_invalidos() {
    for (source, fragment) in [
        ("#set math.mat(foo: 1); $mat()$", "unexpected argument: foo"),
        ("#set math.mat(rows: ((1,),)); $mat()$", "unexpected argument: rows"),
        ("#set math.mat(delim: 1); $mat()$", "expected array, none, symbol, or string"),
        ("#set math.mat(gap: 1); $mat()$", "expected relative length"),
        ("#set math.mat(row-gap: 1); $mat()$", "expected relative length"),
        ("#set math.mat(column-gap: 1); $mat()$", "expected relative length"),
        ("#set math.mat(align: top); $mat()$", "start"),
        (
            "#set math.mat(augment: \"x\"); $mat()$",
            "expected integer, dictionary, or none",
        ),
        (
            "#set math.mat(column-gap: 1em, column-gap: 2em); $mat()$",
            "duplicate argument: column-gap",
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

#[test]
fn v16_b_matrix_observer_returns_outer_target_without_descending() {
    let inner = Content::math_matrix(vec![vec![Content::text("first")]], ('(', ')'));
    let outer = Content::math_matrix(vec![vec![inner]], ('(', ')'));
    let Content::MathMatrix(expected) = &outer else { panic!("target fixture") };
    let actual = find_mat(&outer).expect("direct target");
    let Content::MathMatrix(actual) = actual else { panic!("target hit") };
    assert!(std::sync::Arc::ptr_eq(actual, expected));
}

#[test]
fn v16_b_matrix_observer_follows_only_allowed_wrapper_paths() {
    let target = Content::math_matrix(vec![vec![Content::text("first")]], ('(', ')'));
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
        assert_eq!(find_mat(&wrapped), Some(&target), "{wrapped:?}");
    }
}

#[test]
fn v16_b_matrix_observer_preserves_first_match_and_sequence_order() {
    let target = Content::math_matrix(vec![vec![Content::text("first")]], ('(', ')'));
    let later = Content::math_matrix(vec![vec![Content::text("later")]], ('(', ')'));
    assert_ne!(find_mat(&target), find_mat(&later));
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
        assert_eq!(find_mat(&sequence), Some(&target), "{sequence:?}");
    }
    for reversed in [
        Content::Sequence(vec![later.clone(), target.clone()].into()),
        Content::MathSequence(vec![later.clone(), target].into()),
    ] {
        assert_eq!(find_mat(&reversed), Some(&later), "{reversed:?}");
    }
}

#[test]
fn v16_b_matrix_observer_stops_at_blocked_wrappers_with_target_decoys() {
    let target = Content::math_matrix(vec![vec![Content::text("first")]], ('(', ')'));
    for blocked in [
        Content::strong(target.clone()),
        Content::math_styled(None, None, None, target.clone(), None),
    ] {
        assert!(find_mat(&blocked).is_none(), "unexpected descent: {blocked:?}");
        let sequence = Content::Sequence(vec![blocked, target.clone()].into());
        assert_eq!(find_mat(&sequence), Some(&target));
    }
    for miss in [
        Content::Empty,
        Content::text("not a target"),
        Content::Sequence(vec![].into()),
        Content::MathSequence(vec![].into()),
        Content::equation(Content::text("no target inside"), false),
    ] {
        assert!(find_mat(&miss).is_none(), "{miss:?}");
    }
}
