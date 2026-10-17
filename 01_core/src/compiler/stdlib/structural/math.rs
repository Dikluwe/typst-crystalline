//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math.md
//! @prompt-hash 9e2638df
//! @layer L1
//! @updated 2026-09-30
//!
//! Catálogo estático do módulo `math`; cada família de comportamento
//! reside no seu módulo filho.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

#[cfg(test)]
use crate::compiler::eval::EvalContext;
#[cfg(test)]
use crate::entities::args::ArgOccurrence;
#[cfg(test)]
use crate::entities::args::Args;
use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::elements::math_attach::MathAttachSlot;
#[cfg(test)]
use crate::entities::file_id::FileId;
#[cfg(test)]
use crate::entities::source_result::SourceResult;
#[cfg(test)]
use crate::entities::span::Span;
use crate::entities::value::Value;

pub(super) mod abs;
pub(super) mod accent;
pub(super) mod annotations;
pub(super) mod attach;
pub(super) mod binom;
pub(super) mod cancel;
pub(super) mod cases;
pub(super) mod casts;
pub(super) mod class;
pub(super) mod equation;
pub(super) mod frac;
mod limits;
pub(super) mod lr;
pub(super) mod matrix;
pub(super) mod mid;
pub(super) mod norm;
pub(super) mod op;
pub(super) mod overline;
pub(super) mod primes;
pub(super) mod root;
pub(super) mod round;
pub(super) mod stretch;
pub(super) mod underline;
pub(super) mod underover;
mod vector;

use casts::vanilla_type_name_class;

pub(in crate::compiler) use annotations::{
    native_math_overbrace, native_math_overbracket, native_math_overparen,
    native_math_overshell, native_math_underbrace, native_math_underbracket,
    native_math_underparen, native_math_undershell,
};
pub(in crate::compiler) use attach::native_math_attach;
pub(in crate::compiler) use binom::native_math_binom;
pub(in crate::compiler) use cases::native_math_cases;
pub(in crate::compiler) use class::native_math_class as native_math_class_for_metadata;
pub(in crate::compiler) use frac::native_math_frac;
pub(in crate::compiler) use limits::{native_math_limits, native_math_scripts};
pub(in crate::compiler) use lr::native_math_lr;
pub(in crate::compiler) use mid::native_math_mid;
pub(in crate::compiler) use overline::native_math_overline;
pub(in crate::compiler) use primes::native_math_primes;
pub(in crate::compiler) use root::native_math_root;
pub(in crate::compiler) use stretch::native_math_stretch;
pub(in crate::compiler) use underline::native_math_underline;
pub(in crate::compiler) use vector::native_math_vec;

mod p1425_tests;

mod p1426_tests;

mod p1427_tests;

/// Constrói o módulo `math` como `Value::Module` com 41 operadores
/// vanilla pré-definidos (paralelo `make_calc_module()` P283).
pub fn make_math_module() -> Value {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    // Scripts-style operators (29) — limits: false.
    for name in [
        "arccos", "arcsin", "arctan", "arg", "cos", "cosh", "cot", "coth", "csc", "csch",
        "ctg", "deg", "dim", "exp", "hom", "id", "im", "ker", "lg", "ln", "log", "mod",
        "sec", "sech", "sin", "sinc", "sinh", "tan", "tanh", "tg", "tr",
    ] {
        dict.insert(name.into(), op::op_value(name, false));
    }

    // Limits-style operators (12) — limits: true. Multi-word usam
    // o espaço fino U+2009 do literal vanila (e.g. `liminf` → "lim inf").
    for (name, text) in [
        ("det", "det"),
        ("gcd", "gcd"),
        ("lcm", "lcm"),
        ("inf", "inf"),
        ("lim", "lim"),
        ("liminf", "lim\u{2009}inf"),
        ("limsup", "lim\u{2009}sup"),
        ("max", "max"),
        ("min", "min"),
        ("Pr", "Pr"),
        ("sup", "sup"),
    ] {
        dict.insert(name.into(), op::op_value(text, true));
    }

    // P1140.3-B — primeira fase pública do elemento (body + block).
    dict.insert(
        "equation".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.equation",
            equation::native_math_equation,
        )),
    );

    // P1140.3-A — função pública antes da cópia sym→math; a cópia nunca
    // sobrescreve bindings já definidos.
    dict.insert(
        "sqrt".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.sqrt",
            root::native_math_sqrt,
        )),
    );
    dict.insert(
        "root".into(),
        Value::Func(crate::entities::func::Func::native("root", root::native_math_root)),
    );
    dict.insert(
        "frac".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.frac",
            frac::native_math_frac,
        )),
    );
    dict.insert(
        "accent".into(),
        Value::Func(crate::entities::func::Func::native("accent", accent::native_accent)),
    );

    // P1292-A — a função canónica existente também é membro de `math`.
    // O binding precede o espelho sym→math, que nunca o sobrescreve.
    dict.insert(
        "cancel".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.cancel",
            cancel::native_cancel,
        )),
    );

    dict.insert(
        "underline".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.underline",
            underline::native_math_underline,
        )),
    );

    dict.insert(
        "vec".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.vec",
            vector::native_math_vec,
        )),
    );
    dict.insert(
        "mat".into(),
        Value::Func(crate::entities::func::Func::native("mat", matrix::native_math_mat)),
    );
    dict.insert(
        "cases".into(),
        Value::Func(crate::entities::func::Func::native(
            "cases",
            cases::native_math_cases,
        )),
    );
    dict.insert(
        "lr".into(),
        Value::Func(crate::entities::func::Func::native("lr", lr::native_math_lr)),
    );
    dict.insert(
        "mid".into(),
        Value::Func(crate::entities::func::Func::native("mid", mid::native_math_mid)),
    );
    dict.insert(
        "norm".into(),
        Value::Func(crate::entities::func::Func::native("norm", norm::native_math_norm)),
    );
    dict.insert(
        "round".into(),
        Value::Func(crate::entities::func::Func::native(
            "round",
            round::native_math_round,
        )),
    );
    dict.insert(
        "abs".into(),
        Value::Func(crate::entities::func::Func::native("abs", abs::native_math_abs)),
    );
    dict.insert(
        "stretch".into(),
        Value::Func(crate::entities::func::Func::native(
            "stretch",
            stretch::native_math_stretch,
        )),
    );
    dict.insert(
        "overbrace".into(),
        Value::Func(crate::entities::func::Func::native(
            "overbrace",
            annotations::native_math_overbrace,
        )),
    );
    dict.insert(
        "underbrace".into(),
        Value::Func(crate::entities::func::Func::native(
            "underbrace",
            annotations::native_math_underbrace,
        )),
    );
    dict.insert(
        "underbracket".into(),
        Value::Func(crate::entities::func::Func::native(
            "underbracket",
            annotations::native_math_underbracket,
        )),
    );
    dict.insert(
        "underparen".into(),
        Value::Func(crate::entities::func::Func::native(
            "underparen",
            annotations::native_math_underparen,
        )),
    );
    dict.insert(
        "undershell".into(),
        Value::Func(crate::entities::func::Func::native(
            "undershell",
            annotations::native_math_undershell,
        )),
    );
    dict.insert(
        "overbracket".into(),
        Value::Func(crate::entities::func::Func::native(
            "overbracket",
            annotations::native_math_overbracket,
        )),
    );
    dict.insert(
        "overline".into(),
        Value::Func(crate::entities::func::Func::native(
            "math.overline",
            overline::native_math_overline,
        )),
    );
    dict.insert(
        "overparen".into(),
        Value::Func(crate::entities::func::Func::native(
            "overparen",
            annotations::native_math_overparen,
        )),
    );
    dict.insert(
        "overshell".into(),
        Value::Func(crate::entities::func::Func::native(
            "overshell",
            annotations::native_math_overshell,
        )),
    );
    dict.insert(
        "primes".into(),
        Value::Func(crate::entities::func::Func::native(
            "primes",
            primes::native_math_primes,
        )),
    );

    // P1293-B — quatro membros próprios antes do espelho sym→math.
    dict.insert(
        "attach".into(),
        Value::Func(crate::entities::func::Func::native(
            "attach",
            attach::native_math_attach,
        )),
    );
    dict.insert(
        "binom".into(),
        Value::Func(crate::entities::func::Func::native(
            "binom",
            binom::native_math_binom,
        )),
    );
    for (name, native) in [
        ("mono", crate::compiler::stdlib::math_style::native_mono as _),
        ("script", crate::compiler::stdlib::math_style::native_script as _),
    ] {
        dict.insert(
            name.into(),
            Value::Func(crate::entities::func::Func::native(name, native)),
        );
    }

    // P795 — dif e Dif operadores em modo math (expostos no modulo math)
    // **P962** — com wrapper `upright` (`MathStyled { italic: Some(false) }`,
    // o mesmo que `upright(d)` produz): sem ele, `apply_math_default`
    // italicava o "d" para 𝑑 (U+1D451) — o vanilla define
    // `dif = HElem(THIN, weak) + ClassElem(Unary, upright(SymbolElem('d')))`
    // (`math/op.rs:52-56`) e o "d" do diferencial é reto. Ver
    // `stdlib/structural.md` §P962 (o espaço fino fraco + classe Unary do
    // vanilla ficam registados como scope-out nessa secção).
    let differential = |letter: &str| {
        Content::sequence(vec![
            Content::h_space(crate::entities::layout_types::Length::em(1.0 / 6.0), true),
            Content::math_class_override(
                crate::entities::math_class::MathClass::Unary,
                Content::math_styled(
                    None,
                    None,
                    Some(false),
                    Content::MathText(letter.into()),
                    None,
                ),
            ),
        ])
    };
    dict.insert("dif".into(), Value::Content(differential("d")));
    dict.insert("Dif".into(), Value::Content(differential("D")));

    // **P772y** — `math.class(class, body)`: override manual de `MathClass`
    // para efeitos de espaçamento automático. Vive no scope do módulo
    // `math` (não no scope global, ao contrário de `cancel`/`accent`).
    dict.insert(
        "class".into(),
        Value::Func(crate::entities::func::Func::native(
            "class",
            class::native_math_class,
        )),
    );

    // **P895** — `math.op(...)`: mesma função nativa `op::native_op` já registada
    // no scope global (`eval/mod.rs::scope.define("op", ...)`) também
    // acessível via `math.op(...)`, paridade vanilla (namespace `math`
    // reexpõe as funções de operador — achado do catálogo de terceiros em
    // `typst-passo-895-relatorio.md`, Parte B).
    dict.insert(
        "op".into(),
        Value::Func(crate::entities::func::Func::native_with_engine("op", op::native_op)),
    );

    // P1291 — bindings próprios medidos do namespace math. Os quatro
    // styles reutilizam exactamente as nativas donas; `scripts` é somente
    // o adapter ABI privado para o constructor estrutural existente.
    for (name, native) in [
        ("bb", crate::compiler::stdlib::math_style::native_bb as _),
        ("frak", crate::compiler::stdlib::math_style::native_frak as _),
        ("inline", crate::compiler::stdlib::math_style::native_inline as _),
        ("serif", crate::compiler::stdlib::math_style::native_serif as _),
    ] {
        dict.insert(
            name.into(),
            Value::Func(crate::entities::func::Func::native(name, native)),
        );
    }
    // P1472 — oito rotas qualificadas restantes de estilo math (`bold`, `cal`,
    // `display`, `italic`, `sans`, `scr`, `sscript`, `upright`): reutilizam
    // exatamente as mesmas nativas de `math_style.rs` já expostas como globais
    // bare (`eval/mod.rs:8226-8238`), com o mesmo nome bare — `Func::PartialEq`
    // compara nativas por nome (`entities/func.rs:384-397`), logo o nome
    // idêntico já garante `math.bold == bold` etc. sem partilha de `Arc`.
    for (name, native) in [
        ("bold", crate::compiler::stdlib::math_style::native_bold as _),
        ("cal", crate::compiler::stdlib::math_style::native_cal as _),
        ("display", crate::compiler::stdlib::math_style::native_display as _),
        ("italic", crate::compiler::stdlib::math_style::native_math_italic as _),
        ("sans", crate::compiler::stdlib::math_style::native_sans as _),
        ("scr", crate::compiler::stdlib::math_style::native_scr as _),
        ("sscript", crate::compiler::stdlib::math_style::native_sscript as _),
        ("upright", crate::compiler::stdlib::math_style::native_upright as _),
    ] {
        dict.insert(
            name.into(),
            Value::Func(crate::entities::func::Func::native(name, native)),
        );
    }
    // P1472 — `math.text`: `math.text == text` (medido) — a mesma função de
    // estilo de texto documental (`native_text`), não um carrier matemático
    // novo. Reutiliza `native_text` com o nome bare `"text"` para satisfazer
    // a mesma igualdade por nome usada acima.
    dict.insert(
        "text".into(),
        Value::Func(crate::entities::func::Func::native(
            "text",
            crate::compiler::stdlib::native_text,
        )),
    );
    dict.insert(
        "scripts".into(),
        Value::Func(crate::entities::func::Func::native(
            "scripts",
            limits::native_math_scripts,
        )),
    );
    dict.insert(
        "limits".into(),
        Value::Func(crate::entities::func::Func::native(
            "limits",
            limits::native_math_limits,
        )),
    );

    // **P895** — espaçamentos nomeados de modo math, nunca registados
    // (paridade vanilla `math/mod.rs:36-40,98-102`: `THIN`/`MEDIUM`/`THICK`
    // = mesmas fracções de em já usadas em `spacing.rs` para o espaçamento
    // automático por `MathClass`; `QUAD`/`WIDE` são 1em/2em). `lookup_math_op`
    // (`eval/math.rs`) resolve identificadores bare em modo math via
    // `Value::Content` no scope do módulo `math` — o mesmo mecanismo que já
    // resolve `dif`/`Dif` acima serve estes cinco sem mudança de código.
    for (name, em) in [
        ("thin", 1.0 / 6.0),
        ("med", 2.0 / 9.0),
        ("thick", 5.0 / 18.0),
        ("quad", 1.0),
        ("wide", 2.0),
    ] {
        dict.insert(
            name.into(),
            Value::Content(Content::h_space(
                crate::entities::layout_types::Length::em(em),
                false,
            )),
        );
    }

    // **P731** — `Value::Module` (paridade vanilla — medido: `type(math)` →
    // `module`), não `Value::Dict`. `eval/math.rs::lookup_math_op` lê o
    // scope do módulo.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    // Vanilla Typst expõe todos os símbolos do módulo `sym` também no módulo `math`.
    if let Value::Module(sym_m) = crate::compiler::stdlib::sym::build_sym_module() {
        for (k, binding) in sym_m.scope().iter() {
            if scope.get(k).is_none() {
                scope.define(k, binding.value().clone());
            }
        }
    }
    Value::Module(crate::entities::module::Module::new("math", scope))
}

#[cfg(test)]
mod p1291_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::math_style::MathStyleKind;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    type NativeFn =
        fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>;

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
            test_file_id()
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
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

    fn call_math(name: &str, args: Args) -> SourceResult<Value> {
        let call = math_func(name)
            .native_fn_addr()
            .unwrap_or_else(|| panic!("math.{name} must be a native callable"));
        call(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    fn body() -> Value {
        Value::Content(Content::MathIdent("x".into()))
    }

    fn diagnostic_message(name: &str, args: Args) -> String {
        let diagnostics = call_math(name, args).unwrap_err();
        assert_eq!(diagnostics.len(), 1, "math.{name} must emit one primary diagnostic");
        diagnostics.into_iter().next().unwrap().message
    }

    #[test]
    fn p1291_namespace_expoe_cinco_funcoes_com_nome_curto() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["bb", "frak", "inline", "scripts", "serif"] {
            let Some(Value::Func(func)) = module.scope().get(name) else {
                panic!("math.{name} must be callable")
            };
            assert_eq!(func.name(), Some(name), "math.{name} public name");
        }
    }

    #[test]
    fn p1291_styles_reutilizam_as_quatro_nativas_donas() {
        let expected: [(&str, NativeFn); 4] = [
            ("bb", crate::compiler::stdlib::math_style::native_bb),
            ("frak", crate::compiler::stdlib::math_style::native_frak),
            ("inline", crate::compiler::stdlib::math_style::native_inline),
            ("serif", crate::compiler::stdlib::math_style::native_serif),
        ];
        for (name, expected) in expected {
            let actual = math_func(name).native_fn_addr().unwrap();
            assert!(
                std::ptr::fn_addr_eq(actual, expected),
                "math.{name} points to the wrong global function"
            );
        }
    }

    #[test]
    fn p1291_styles_produzem_morfologia_e_defaults_medidos() {
        for (name, kind, cramped) in [
            ("bb", MathStyleKind::DoubleStruck, None),
            ("frak", MathStyleKind::Fraktur, None),
            ("inline", MathStyleKind::Inline, Some(false)),
            ("serif", MathStyleKind::Plain, None),
        ] {
            let value = call_math(name, Args::positional(vec![body()])).unwrap();
            let Value::Content(Content::MathStyled(elem)) = value else {
                panic!("math.{name} must produce MathStyled")
            };
            assert_eq!(elem.kind, Some(kind), "math.{name} style kind");
            assert_eq!(elem.cramped, cramped, "math.{name} cramped default");
            assert!(matches!(&elem.body, Content::MathIdent(x) if x == "x"));
        }

        let mut explicit = Args::positional(vec![body()]);
        explicit.named.insert("cramped".into(), Value::Bool(true));
        let Value::Content(Content::MathStyled(elem)) =
            call_math("inline", explicit).unwrap()
        else {
            panic!("math.inline(cramped: true) must produce MathStyled")
        };
        assert_eq!(elem.cramped, Some(true));
    }

    #[test]
    fn p1291_styles_rejeitam_aridade_tipo_e_named_invalidos() {
        for name in ["bb", "frak", "inline", "serif"] {
            assert!(call_math(name, Args::positional(vec![])).is_err());
            assert!(call_math(name, Args::positional(vec![Value::Int(1)])).is_err());
            assert!(call_math(name, Args::positional(vec![body(), body()])).is_err());
            let mut unknown = Args::positional(vec![body()]);
            unknown.named.insert("unknown".into(), Value::Bool(true));
            assert!(call_math(name, unknown).is_err());
        }
        let mut wrong_cramped = Args::positional(vec![body()]);
        wrong_cramped.named.insert("cramped".into(), Value::Int(1));
        assert!(call_math("inline", wrong_cramped).is_err());
    }

    #[test]
    fn p1291_scripts_constroi_override_e_valida_argumentos() {
        let value = call_math("scripts", Args::positional(vec![body()])).unwrap();
        let Value::Content(Content::MathLimitsOverride(elem)) = value else {
            panic!("math.scripts must produce MathLimitsOverride")
        };
        assert!(matches!(&elem.body, Content::MathIdent(x) if x == "x"));
        assert!(!elem.limits, "scripts must force lateral attachments");
        assert!(elem.inline, "canonical ignored inline transport must be true");

        assert!(call_math("scripts", Args::positional(vec![])).is_err());
        assert!(call_math("scripts", Args::positional(vec![Value::Int(1)])).is_err());
        assert!(call_math("scripts", Args::positional(vec![body(), body()])).is_err());
        let mut unknown = Args::positional(vec![body()]);
        unknown.named.insert("foo".into(), Value::Int(1));
        let err = call_math("scripts", unknown).unwrap_err();
        assert!(format!("{err:?}").contains("unexpected argument: foo"));
    }

    #[test]
    fn p1291_red_errors_excesso_e_unexpected_argument_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                (name, diagnostic_message(name, Args::positional(vec![body(), body()])))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "unexpected argument".to_string()),
                ("frak", "unexpected argument".to_string()),
                ("inline", "unexpected argument".to_string()),
                ("scripts", "unexpected argument".to_string()),
                ("serif", "unexpected argument".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_named_nope_e_unexpected_argument_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                let mut args = Args::positional(vec![body()]);
                args.named.insert("nope".into(), Value::Bool(true));
                (name, diagnostic_message(name, args))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "unexpected argument: nope".to_string()),
                ("frak", "unexpected argument: nope".to_string()),
                ("inline", "unexpected argument: nope".to_string()),
                ("scripts", "unexpected argument: nope".to_string()),
                ("serif", "unexpected argument: nope".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_body_integer_e_expected_content_verbatim() {
        let actual: Vec<_> = ["bb", "frak", "inline", "scripts", "serif"]
            .into_iter()
            .map(|name| {
                (name, diagnostic_message(name, Args::positional(vec![Value::Int(1)])))
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                ("bb", "expected content, found integer".to_string()),
                ("frak", "expected content, found integer".to_string()),
                ("inline", "expected content, found integer".to_string()),
                ("scripts", "expected content, found integer".to_string()),
                ("serif", "expected content, found integer".to_string()),
            ]
        );
    }

    #[test]
    fn p1291_red_errors_inline_cramped_integer_e_boolean_verbatim() {
        let mut args = Args::positional(vec![body()]);
        args.named.insert("cramped".into(), Value::Int(1));
        assert_eq!(diagnostic_message("inline", args), "expected boolean, found integer");
    }

    #[test]
    fn p1291_red_errors_styles_continuam_a_aceitar_string() {
        for name in ["bb", "frak", "inline", "serif"] {
            let value = call_math(name, Args::positional(vec![Value::Str("x".into())]))
                .unwrap_or_else(|diagnostics| {
                    panic!("math.{name} rejected string: {diagnostics:?}")
                });
            let Value::Content(Content::MathStyled(elem)) = value else {
                panic!("math.{name} string must produce MathStyled")
            };
            assert!(
                matches!(&elem.body, Content::MathText(text) if text == "x"),
                "math.{name} string must be converted to MathText"
            );
        }
    }

    #[test]
    fn p1291_scope_fechado_nao_copia_extras_globais() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["lorem", "read", "eval", "panic", "assert"] {
            assert!(
                module.scope().get(name).is_none(),
                "global extra math.{name} leaked"
            );
        }
    }

    #[test]
    fn p1293_b_namespace_expoe_quatro_funcoes_com_nomes_curtos() {
        let Value::Module(module) = make_math_module() else {
            panic!("math must be a module")
        };
        for name in ["attach", "binom", "mono", "script"] {
            let Some(Value::Func(func)) = module.scope().get(name) else {
                panic!("missing callable math.{name}")
            };
            assert_eq!(func.name(), Some(name), "math.{name} public name");
        }
    }

    #[test]
    fn p1293_b_attach_preserva_sete_slots_e_none_explicito() {
        let values = ["x", "t", "b", "tl", "bl", "tr", "br"];
        let mut args =
            Args::positional(vec![Value::Content(Content::MathIdent(values[0].into()))]);
        for (name, text) in [
            ("t", values[1]),
            ("b", values[2]),
            ("tl", values[3]),
            ("bl", values[4]),
            ("tr", values[5]),
            ("br", values[6]),
        ] {
            args.named
                .insert(name.into(), Value::Content(Content::MathIdent(text.into())));
        }
        let Value::Content(Content::MathAttach(elem)) =
            call_math("attach", args).unwrap()
        else {
            panic!("math.attach must produce MathAttach")
        };
        assert!(matches!(&elem.base, Content::MathIdent(x) if x == "x"));
        for (actual, expected) in [
            (&elem.t, "t"),
            (&elem.b, "b"),
            (&elem.tl, "tl"),
            (&elem.bl, "bl"),
            (&elem.tr, "tr"),
            (&elem.br, "br"),
        ] {
            assert!(
                matches!(actual, MathAttachSlot::Present(Content::MathIdent(x)) if x == expected)
            );
        }

        let mut explicit_none = Args::positional(vec![body()]);
        explicit_none.named.insert("t".into(), Value::None);
        let Value::Content(Content::MathAttach(elem)) =
            call_math("attach", explicit_none).unwrap()
        else {
            panic!("math.attach(t: none) must produce MathAttach")
        };
        assert!(matches!(elem.t, MathAttachSlot::ExplicitNone));
        assert!(matches!(elem.b, MathAttachSlot::Omitted));
    }

    #[test]
    fn p1293_b_attach_none_explicito_tem_layout_equivalente_ao_omitido() {
        use crate::compiler::layout::FixedMetrics;
        use crate::compiler::math::layout::MathLayouter;
        use crate::entities::layout_types::{Pt, TextStyle};

        let style = TextStyle::regular(Pt(12.0));
        let layouter = MathLayouter::new(&FixedMetrics, true, &style);
        let omitted = attach::math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        let explicit_none = attach::math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::ExplicitNone,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );

        let (omitted_items, omitted_extent) =
            layouter.layout_equation_measured(&omitted, &style);
        let (explicit_items, explicit_extent) =
            layouter.layout_equation_measured(&explicit_none, &style);
        assert_eq!(explicit_extent, omitted_extent);
        assert_eq!(explicit_items.len(), omitted_items.len());
    }

    #[test]
    fn p1293_b_binom_preserva_lower_variadico_e_sem_barra() {
        let args = Args::positional(vec![
            Value::Content(Content::MathIdent("n".into())),
            Value::Content(Content::MathIdent("k".into())),
            Value::Content(Content::MathIdent("j".into())),
            Value::Content(Content::MathIdent("m".into())),
        ]);
        let Value::Content(Content::MathDelimited(delimited)) =
            call_math("binom", args).unwrap()
        else {
            panic!("math.binom must produce stretched parentheses")
        };
        assert_eq!((delimited.open, delimited.close), ('(', ')'));
        let Content::MathFrac(frac) = &delimited.body else {
            panic!("math.binom must use the canonical fraction payload")
        };
        assert!(!frac.line, "binom must not draw a fraction bar");
        assert!(matches!(&frac.num, Content::MathIdent(x) if x == "n"));
        let Content::MathSequence(lower) = &frac.den else {
            panic!("variadic lower must preserve comma morphology")
        };
        assert_eq!(lower.len(), 5);
        assert!(matches!(&lower[0], Content::MathIdent(x) if x == "k"));
        assert!(matches!(&lower[1], Content::MathText(x) if x == ", "));
        assert!(matches!(&lower[2], Content::MathIdent(x) if x == "j"));
        assert!(matches!(&lower[3], Content::MathText(x) if x == ", "));
        assert!(matches!(&lower[4], Content::MathIdent(x) if x == "m"));
    }

    #[test]
    fn p1293_b_mono_script_reusam_nativas_e_defaults() {
        let expected: [(&str, NativeFn); 2] = [
            ("mono", crate::compiler::stdlib::math_style::native_mono),
            ("script", crate::compiler::stdlib::math_style::native_script),
        ];
        for (name, expected) in expected {
            let actual = math_func(name).native_fn_addr().unwrap();
            assert!(
                std::ptr::fn_addr_eq(actual, expected),
                "math.{name} must reuse its global native"
            );
        }

        let Value::Content(Content::MathStyled(mono)) =
            call_math("mono", Args::positional(vec![body()])).unwrap()
        else {
            panic!("math.mono must produce MathStyled")
        };
        assert_eq!(mono.kind, Some(MathStyleKind::Monospace));

        for (explicit, expected) in
            [(None, true), (Some(true), true), (Some(false), false)]
        {
            let mut args = Args::positional(vec![body()]);
            if let Some(value) = explicit {
                args.named.insert("cramped".into(), Value::Bool(value));
            }
            let Value::Content(Content::MathStyled(script)) =
                call_math("script", args).unwrap()
            else {
                panic!("math.script must produce MathStyled")
            };
            assert_eq!(script.kind, Some(MathStyleKind::Script));
            assert_eq!(script.cramped, Some(expected));
        }
    }

    #[test]
    fn p1293_b_oito_vetores_alcancam_o_layout_vigente() {
        use crate::compiler::layout::FixedMetrics;
        use crate::compiler::math::layout::MathLayouter;
        use crate::entities::layout_types::{MathSize, Pt, TextStyle};

        let inline_style = TextStyle::regular(Pt(12.0));
        let display_style = TextStyle {
            math_size: MathSize::Display,
            ..inline_style.clone()
        };
        let display = MathLayouter::new(&FixedMetrics, true, &display_style);
        let inline = MathLayouter::new(&FixedMetrics, false, &inline_style);
        let attach = attach::math_attach_content(
            Content::MathIdent("x".into()),
            MathAttachSlot::Present(Content::MathIdent("t".into())),
            MathAttachSlot::Present(Content::MathIdent("b".into())),
            MathAttachSlot::Present(Content::MathIdent("l".into())),
            MathAttachSlot::Present(Content::MathIdent("l".into())),
            MathAttachSlot::Present(Content::MathIdent("r".into())),
            MathAttachSlot::Present(Content::MathIdent("r".into())),
        );
        let binom = binom::math_binom_content(
            Content::MathIdent("n".into()),
            vec![
                Content::MathIdent("k".into()),
                Content::MathIdent("j".into()),
                Content::MathIdent("m".into()),
            ],
        );
        let mono = Content::math_styled(
            Some(MathStyleKind::Monospace),
            None,
            None,
            Content::MathIdent("mono".into()),
            None,
        );
        let mono_in_script = Content::math_styled(
            Some(MathStyleKind::Script),
            None,
            None,
            mono.clone(),
            Some(true),
        );
        let scripted_body = Content::math_attach_scripts(
            Content::MathIdent("x".into()),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Present(Content::MathText("1".into())),
        );
        let scripted = |cramped| {
            Content::math_styled(
                Some(MathStyleKind::Script),
                None,
                None,
                scripted_body.clone(),
                Some(cramped),
            )
        };
        let dimensions = |layouter: &MathLayouter<'_, FixedMetrics>,
                          content: &Content,
                          style: &TextStyle| {
            let (_, extent) = layouter.layout_equation_measured(content, style);
            (extent.width, extent.ascent + extent.descent)
        };

        let vectors = [
            dimensions(&display, &attach, &display_style),
            dimensions(&inline, &attach, &inline_style),
            dimensions(&display, &binom, &display_style),
            dimensions(&inline, &binom, &inline_style),
            dimensions(&display, &mono, &display_style),
            dimensions(&display, &mono_in_script, &display_style),
            dimensions(&display, &scripted(true), &display_style),
            dimensions(&display, &scripted(false), &display_style),
        ];
        assert!(vectors.iter().all(|(width, height)| {
            width.is_finite() && *width > 0.0 && height.is_finite() && *height > 0.0
        }));
        assert!(vectors[2].1 > vectors[3].1, "binom display versus inline: {vectors:?}");
        assert!(vectors[5].0 < vectors[4].0, "mono dentro de script deve reduzir");
        assert_ne!(vectors[6].1, vectors[7].1, "cramped deve permanecer observável");
    }

    #[test]
    fn p1293_b_assinaturas_sao_fechadas() {
        let span = Span::from_range(test_file_id(), 17..18);
        let assert_diag = |name: &str, mut args: Args, expected: &str| {
            args.span = span;
            let diagnostics = call_math(name, args).unwrap_err();
            assert_eq!(diagnostics.len(), 1, "math.{name}");
            assert_eq!(diagnostics[0].message, expected, "math.{name}");
            assert_eq!(diagnostics[0].span, span, "math.{name} selected span");
        };

        assert_diag("attach", Args::positional(vec![]), "missing argument: base");
        assert_diag(
            "attach",
            Args::positional(vec![Value::Int(1)]),
            "expected content, found integer",
        );
        assert_diag(
            "attach",
            Args::positional(vec![body(), body()]),
            "unexpected argument",
        );
        let mut attach_unknown = Args::positional(vec![body()]);
        attach_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("attach", attach_unknown, "unexpected argument: unknown");
        let mut attach_bad_slot = Args::positional(vec![body()]);
        attach_bad_slot.named.insert("t".into(), Value::Int(1));
        assert_diag("attach", attach_bad_slot, "expected content or none, found integer");

        assert_diag("binom", Args::positional(vec![]), "missing argument: upper");
        assert_diag("binom", Args::positional(vec![body()]), "missing argument: lower");
        assert_diag(
            "binom",
            Args::positional(vec![Value::Int(1), body()]),
            "expected content, found integer",
        );
        let mut binom_unknown = Args::positional(vec![body(), body()]);
        binom_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("binom", binom_unknown, "unexpected argument: unknown");
        let mut upper_named = Args::positional(vec![]);
        upper_named.named.insert("upper".into(), body());
        assert_diag("binom", upper_named, "the argument `upper` is positional");

        for name in ["mono", "script"] {
            assert_diag(name, Args::positional(vec![]), "missing argument: body");
            assert_diag(
                name,
                Args::positional(vec![Value::Int(1)]),
                "expected content, found integer",
            );
            assert_diag(
                name,
                Args::positional(vec![body(), body()]),
                "unexpected argument",
            );
            let mut body_named = Args::positional(vec![]);
            body_named.named.insert("body".into(), body());
            assert_diag(name, body_named, "the argument `body` is positional");
        }

        let mut wrong_cramped = Args::positional(vec![body()]);
        wrong_cramped.named.insert("cramped".into(), Value::Int(1));
        assert_diag("script", wrong_cramped, "expected boolean, found integer");
        let mut script_unknown = Args::positional(vec![body()]);
        script_unknown.named.insert("unknown".into(), Value::Bool(true));
        assert_diag("script", script_unknown, "unexpected argument: unknown");
    }

    #[test]
    fn p1293_b_independent_red_named_positional_preserva_hints() {
        let span = Span::from_range(test_file_id(), 17..18);
        let assert_named_positional = |name: &str, mut args: Args, parameter: &str| {
            args.span = span;
            let diagnostics = call_math(name, args).unwrap_err();
            assert_eq!(diagnostics.len(), 1, "math.{name}");
            assert_eq!(
                diagnostics[0].message,
                format!("the argument `{parameter}` is positional"),
                "math.{name}"
            );
            assert_eq!(
                diagnostics[0].hints,
                vec![format!("try removing `{parameter}:`")],
                "math.{name}"
            );
            assert_eq!(diagnostics[0].span, span, "math.{name} selected span");
        };

        let mut attach = Args::positional(vec![]);
        attach.named.insert("base".into(), body());
        assert_named_positional("attach", attach, "base");

        let mut binom = Args::positional(vec![]);
        binom.named.insert("upper".into(), body());
        assert_named_positional("binom", binom, "upper");

        for name in ["mono", "script"] {
            let mut args = Args::positional(vec![]);
            args.named.insert("body".into(), body());
            assert_named_positional(name, args, "body");
        }
    }
}

#[cfg(test)]
mod p1283_tests {
    use super::*;

    fn assert_mirror(
        sym: &crate::entities::module::Module,
        math: &crate::entities::module::Module,
    ) {
        for (name, binding) in sym.scope().iter() {
            let mirrored = math
                .scope()
                .get(name)
                .unwrap_or_else(|| panic!("missing math.{name}"));
            match (binding.value(), mirrored) {
                (Value::Symbol(left), Value::Symbol(right)) => {
                    assert_eq!(left.value, right.value, "wrong math mirror base: {name}");
                    assert_eq!(
                        left.variants, right.variants,
                        "wrong math mirror variants: {name}"
                    );
                }
                (Value::Module(left), Value::Module(right)) => assert_mirror(left, right),
                // Bindings próprios de math ganham de símbolos homónimos.
                (_, Value::Func(_))
                    if matches!(name, "sqrt" | "root" | "class" | "equation" | "op") => {}
                (left, right) => {
                    panic!("wrong math mirror kind for {name}: {left:?} vs {right:?}")
                }
            }
        }
    }

    #[test]
    fn p1283_math_espelha_sym_sem_sobrescrever_funcoes() {
        let Value::Module(sym) = crate::compiler::stdlib::sym::build_sym_module() else {
            panic!("sym must be module")
        };
        let Value::Module(math) = make_math_module() else {
            panic!("math must be module")
        };
        assert_mirror(&sym, &math);
        for name in ["sqrt", "root", "class", "equation", "op"] {
            assert!(
                matches!(math.scope().get(name), Some(Value::Func(_))),
                "math.{name} overwritten"
            );
        }
        assert!(matches!(math.scope().get("gender"), Some(Value::Module(_))));
        assert!(matches!(math.scope().get("control"), Some(Value::Module(_))));
    }
    #[test]
    fn p1496_new_symbol_modules_mirror_without_overwriting_math_sqrt() {
        let Value::Module(sym) = crate::compiler::stdlib::sym::build_sym_module() else {
            panic!("sym must be module")
        };
        let Value::Module(math) = make_math_module() else {
            panic!("math must be module")
        };
        for name in ["chess", "keyboard", "ordinal"] {
            let Some(Value::Module(sym_child)) = sym.scope().get(name) else {
                panic!("missing sym.{name} module")
            };
            let Some(Value::Module(math_child)) = math.scope().get(name) else {
                panic!("missing math.{name} module")
            };
            assert_eq!(math_child.scope().len(), sym_child.scope().len(), "math.{name}");
        }
        for name in ["join", "gt", "lt", "registered"] {
            assert_eq!(
                math.scope().get(name),
                sym.scope().get(name),
                "math.{name} mirror"
            );
        }
        assert!(sym.scope().get("sqrt").is_none());
        assert!(matches!(math.scope().get("sqrt"), Some(Value::Func(_))));
    }
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

// ── Passo 397: `document(...)` e `asset(...)` ────────────────────────────────

/// P1472 (Agente B) — contrato RED das nove rotas qualificadas `math.*`
/// ainda ausentes: `bold`, `cal`, `display`, `italic`, `sans`, `scr`,
/// `sscript`, `upright`, `text`. Fonte: `00_nucleo/materialization/
/// typst-passo-1472.md` + `00_nucleo/prompts/compiler/stdlib/structural/
/// math.md` §P1472 + `00_nucleo/prompts/compiler/stdlib/math_style.md`
/// §P1472. Escrito sem ver a implementação (isolamento Agente A/B) — usa
/// exclusivamente observáveis de língua (`Value`/`Func` `==`) e as nativas
/// públicas já existentes como oráculo Rust, nunca reconstrução do código
/// de `make_math_module()`.
#[cfg(test)]
mod p1472_tests {
    use super::*;
    use crate::compiler::stdlib::math_style;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
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
            test_file_id()
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1472).unwrap())
    }

    type NativeFn =
        fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>;

    fn call_native(native: NativeFn, args: Args) -> SourceResult<Value> {
        native(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    /// Procura `math.<name>` sem entrar em pânico — falhas viram
    /// `Err(razão)` para os testes agregarem várias rotas ausentes numa
    /// única asserção (em vez de parar na primeira, como faria um
    /// `.unwrap()`/`panic!`).
    fn math_lookup(name: &str) -> Result<Func, String> {
        let Value::Module(module) = make_math_module() else {
            return Err("make_math_module() não produziu Value::Module".into());
        };
        match module.scope().get(name) {
            Some(Value::Func(f)) => Ok(f.clone()),
            Some(other) => {
                Err(format!("math.{name} -> {} (esperava function)", other.type_name()))
            }
            // Mensagem verbatim do sintoma que motivou o Passo 1472.
            None => Err(format!("module math does not contain {name}")),
        }
    }

    fn math_call(name: &str, args: Args) -> Result<SourceResult<Value>, String> {
        let func = math_lookup(name)?;
        let native = func
            .native_fn_addr()
            .ok_or_else(|| format!("math.{name} não é native callable"))?;
        Ok(native(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id()))
    }

    fn cv(text: &str) -> Value {
        Value::Content(Content::text(text))
    }

    /// Compara dois `SourceResult<Value>` pelo observável de língua: `Ok`
    /// por igualdade de `Value`/`Content`, `Err` pela igualdade das
    /// mensagens de diagnóstico (`SourceDiagnostic` não deriva
    /// `PartialEq` — padrão já usado por P1427/P1451: a mensagem é o
    /// observável estável).
    fn same_result(
        oracle: &SourceResult<Value>,
        actual: &SourceResult<Value>,
    ) -> Result<(), String> {
        match (oracle, actual) {
            (Ok(a), Ok(b)) if a == b => Ok(()),
            (Ok(a), Ok(b)) => Err(format!("Ok diverge: oracle={a:?} actual={b:?}")),
            (Err(a), Err(b)) => {
                let am: Vec<_> = a.iter().map(|d| d.message.clone()).collect();
                let bm: Vec<_> = b.iter().map(|d| d.message.clone()).collect();
                if am == bm {
                    Ok(())
                } else {
                    Err(format!("diagnostics diverge: oracle={am:?} actual={bm:?}"))
                }
            }
            (oracle, actual) => {
                Err(format!("forma Ok/Err diverge: oracle={oracle:?} actual={actual:?}"))
            }
        }
    }

    const NOVE_ROTAS: [&str; 9] =
        ["bold", "cal", "display", "italic", "sans", "scr", "sscript", "upright", "text"];

    /// Observável 1 (passo §1) — presença e tipo `function` das nove
    /// rotas. RED esperado: as nove ausentes (`module math does not
    /// contain <nome>`).
    #[test]
    fn p1472_nove_rotas_presentes_e_function() {
        let mut failures = Vec::new();
        for name in NOVE_ROTAS {
            if let Err(reason) = math_lookup(name) {
                failures.push(reason);
            }
        }
        assert!(failures.is_empty(), "rotas ausentes/erradas: {failures:#?}");
    }

    /// Observável 2 — identidade `math.<x> == <x>` para os oito estilos,
    /// `==` de língua sobre `Value::Func` (`Func::PartialEq` compara
    /// nativas por nome, `entities/func.rs:384-397`). O oráculo `<x>` é
    /// reconstruído com `Func::native(<nome bare>, <native pública>)` —
    /// o mesmo par nome+fn-ptr que `eval/mod.rs:8226-8238` usa para
    /// registar o global; não há acesso de teste ao root scope real
    /// (`make_stdlib` é privado a `compiler::eval`, não alcançável a
    /// partir de `structural::math` sem tocar produção), mas a igualdade
    /// por nome garante que a reconstrução é observacionalmente idêntica.
    #[test]
    fn p1472_identidade_funcional_com_globais_de_estilo() {
        let cases: [(&str, NativeFn); 8] = [
            ("bold", math_style::native_bold as NativeFn),
            ("cal", math_style::native_cal as NativeFn),
            ("display", math_style::native_display as NativeFn),
            ("italic", math_style::native_math_italic as NativeFn),
            ("sans", math_style::native_sans as NativeFn),
            ("scr", math_style::native_scr as NativeFn),
            ("sscript", math_style::native_sscript as NativeFn),
            ("upright", math_style::native_upright as NativeFn),
        ];
        let mut failures = Vec::new();
        for (name, native) in cases {
            let oracle = Value::Func(Func::native(name, native));
            match math_lookup(name) {
                Ok(actual) => {
                    if Value::Func(actual) != oracle {
                        failures.push(format!("math.{name} != {name} (oracle)"));
                    }
                }
                Err(reason) => failures.push(reason),
            }
        }
        assert!(failures.is_empty(), "{failures:#?}");
    }

    /// Observável 2 (`math.text`) — `math.text == text`; `text` reutiliza
    /// `native_text` bare (decisão registada em `structural/math.md`
    /// §P1472 — não é `Content::MathText`).
    #[test]
    fn p1472_identidade_funcional_math_text_com_global_text() {
        let oracle =
            Value::Func(Func::native("text", crate::compiler::stdlib::native_text));
        match math_lookup("text") {
            Ok(actual) => {
                assert_eq!(Value::Func(actual), oracle, "math.text != text (oracle)")
            }
            Err(reason) => panic!("{reason}"),
        }
    }

    /// Observável 3 — `math.text` reproduz bit a bit o comportamento do
    /// `native_text` **deste baseline** (oráculo Rust real, não a
    /// paráfrase vanilla-flavored citada em `structural/math.md` §P1472:
    /// medido diretamente em 2026-09-21 contra
    /// `01_core/src/compiler/stdlib/text/constructor.rs::native_text`,
    /// as mensagens atuais são "text() requer body como argumento" e
    /// "text() espera content ou string como body, recebeu <tipo>" — não
    /// "missing argument: body"/"expected content, found integer"; e
    /// `text(none)` erra (`Some(Value::None)` cai no braço `Some(other)`
    /// de `constructor.rs`), não produz sequência vazia. `native_text`
    /// não muda neste passo (§7.2/7.4 do passo) — o contrato real é
    /// reuso bit-a-bit da mesma native, não uma wording específica;
    /// comparar contra o oráculo Rust é robusto a essa divergência de
    /// paráfrase e não gera RED por motivo errado.
    #[test]
    fn p1472_math_text_espelha_comportamento_do_text_oracle() {
        let scenarios: [(&str, Args); 5] = [
            ("sem argumentos", Args::positional(vec![])),
            ("integer", Args::positional(vec![Value::Int(1)])),
            ("none", Args::positional(vec![Value::None])),
            ("string", Args::positional(vec![Value::Str("abc".into())])),
            ("content", Args::positional(vec![cv("abc")])),
        ];
        let mut failures = Vec::new();
        for (label, args) in scenarios {
            let oracle = call_native(crate::compiler::stdlib::native_text, args.clone());
            match math_call("text", args) {
                Ok(actual) => {
                    if let Err(msg) = same_result(&oracle, &actual) {
                        failures.push(format!("{label}: {msg}"));
                    }
                }
                Err(reason) => failures.push(format!("{label}: {reason}")),
            }
        }
        assert!(failures.is_empty(), "{failures:#?}");
    }

    /// Observável 4 — amostra de 3 estilos (`bold`, `display`, `sscript`
    /// — cobre os dois defaults `cramped` distintos, `Some(false)` e
    /// `Some(true)`) produz exatamente o mesmo `Content::MathStyled` que
    /// a chamada ao global correspondente.
    #[test]
    fn p1472_estilos_qualificados_produzem_mesmo_content_que_global() {
        let samples: [(&str, NativeFn); 3] = [
            ("bold", math_style::native_bold as NativeFn),
            ("display", math_style::native_display as NativeFn),
            ("sscript", math_style::native_sscript as NativeFn),
        ];
        let mut failures = Vec::new();
        for (name, native) in samples {
            let args = Args::positional(vec![cv("a")]);
            let oracle = call_native(native, args.clone());
            match math_call(name, args) {
                Ok(actual) => {
                    if let Err(msg) = same_result(&oracle, &actual) {
                        failures.push(format!("{name}: {msg}"));
                    }
                }
                Err(reason) => failures.push(format!("{name}: {reason}")),
            }
        }
        assert!(failures.is_empty(), "{failures:#?}");
    }

    /// Observável 5a — não-regressão: os quatro estilos já qualificados
    /// em P1291 (`bb`/`frak`/`inline`/`serif`) continuam presentes e
    /// `function`. Deve passar já neste baseline (não faz parte do RED
    /// de P1472).
    #[test]
    fn p1472_nao_regressao_quatro_estilos_p1291_continuam_function() {
        for name in ["bb", "frak", "inline", "serif"] {
            if let Err(reason) = math_lookup(name) {
                panic!("regressão P1291: {reason}");
            }
        }
    }

    /// Observável 5b — não-regressão dos globais bare fora de `math.*`
    /// (a remoção é P1474, fora de escopo aqui). `eval/mod.rs` (onde os
    /// oito globais estão registados, linhas 8226-8238) não é tocado por
    /// P1472 — só `structural/math.rs` muda — e o seu root scope é
    /// privado a `compiler::eval`, inalcançável a partir daqui sem tocar
    /// produção. Como proxy fiel e suficiente dentro da fronteira do
    /// diff, fixa-se que a native pública por trás de cada global
    /// continua a existir com o mesmo nome bare que também sustenta
    /// `math.<x> == <x>` (observável 2) — condição necessária para os
    /// dois bindings (global e `math.*`) continuarem a resolver para a
    /// mesma identidade de língua depois de P1472.
    #[test]
    fn p1472_nao_regressao_globais_bare_de_estilo_continuam_nativos() {
        let cases: [(&str, NativeFn); 8] = [
            ("bold", math_style::native_bold as NativeFn),
            ("cal", math_style::native_cal as NativeFn),
            ("display", math_style::native_display as NativeFn),
            ("italic", math_style::native_math_italic as NativeFn),
            ("sans", math_style::native_sans as NativeFn),
            ("scr", math_style::native_scr as NativeFn),
            ("sscript", math_style::native_sscript as NativeFn),
            ("upright", math_style::native_upright as NativeFn),
        ];
        for (name, native) in cases {
            let f = Func::native(name, native);
            assert_eq!(f.name(), Some(name), "native de {name} devolve nome errado");
            assert!(f.native_fn_addr().is_some(), "native de {name} não é function ptr");
        }
    }

    /// Observável 6 — espelho `sym → math` preservado: um símbolo fora
    /// das nove novas rotas continua acessível como `math.<símbolo>`
    /// depois da mudança (prova que os novos bindings não quebraram o
    /// espelho nem foram sobrescritos por ele).
    #[test]
    fn p1472_espelho_sym_math_preservado_para_simbolo_nao_novo() {
        assert!(
            !NOVE_ROTAS.contains(&"alpha"),
            "escolher símbolo de amostra fora das nove rotas novas"
        );
        let Value::Module(math) = make_math_module() else { panic!("math module") };
        let Value::Module(sym) = crate::compiler::stdlib::sym::build_sym_module() else {
            panic!("sym module")
        };
        let expected = sym.scope().get("alpha").cloned().expect("sym.alpha deve existir");
        let actual = math.scope().get("alpha").cloned();
        assert_eq!(actual, Some(expected), "math.alpha deve espelhar sym.alpha");
    }
}
