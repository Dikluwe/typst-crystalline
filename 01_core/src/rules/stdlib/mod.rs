//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/_comum.md
//! @prompt-hash a0982782
//! @prompt 00_nucleo/prompts/rules/model/document.md
//! @layer L1
//! @updated 2026-06-22

//! Stdlib nativa mínima — Passo 17.
//!
//! Interface `fn(&mut EvalContext, &Args) -> SourceResult<Value>` (Passo 71, DEBT-24):
//! aceita positional e named args. Funções sem I/O usam `_ctx`.
//!
//! Reestruturado por cluster em Passo 96.5 conforme ADR-0037.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Submódulos por cluster (Passo 96.5, ADR-0037) ───────────────────────────
mod assert;
mod calc;
mod figure_image;
mod foundations;
mod label;
mod layout;
mod panic;
mod r#ref;
mod shapes;
mod structural;
mod text;
mod transforms;
// P262 — Gradient stdlib (Linear only per ADR-0087).
mod gradients;
// P311b.3 — 12 funções math style (bb/cal/frak/etc.).
mod math_style;
// P387 (ADR-0111) — data import: read/csv/json/yaml/toml/cbor/xml.
mod loading;
// P394 — runtime de re-avaliação `eval(source)`.
mod eval;
// P396 — constructor `tiling(...)` e helpers visuais.
mod visualize;
// P403 — constructors stdlib para tipos primitivos L1: decimal, duration, version.
mod primitives_constructors;
// P466 — métodos de instância para array, dict e str.
mod collections;
// P471 — módulo `sym` com tabela de símbolos Unicode.
mod sym;
// P476 — módulo `color` com operadores lighten/darken/mix/negate.
mod color;
// P694 — módulo `sys` (sys.version / sys.inputs).
mod sys;
// P506 — runtime state/counter/context.
pub(crate) mod state;
pub(crate) mod counter;
pub(crate) mod context;

// Re-exports públicos — preservam o path `crate::rules::stdlib::native_X` usado
// por `make_stdlib` em `eval/mod.rs`.
pub use crate::rules::stdlib::assert::native_assert;
pub use crate::rules::stdlib::calc::make_calc_module;
pub use crate::rules::stdlib::eval::native_eval;
pub use crate::rules::stdlib::figure_image::{native_figure, native_image};
pub use crate::rules::stdlib::foundations::{
    native_cmyk, native_counter_at, native_counter_display, native_counter_final,
    native_counter_step, native_float, native_here, native_hsl, native_hsv, native_int,
    native_len, native_linear_rgb, native_locate, native_luma, native_metadata,
    native_oklab, native_oklch, native_query, native_range, native_repr, native_rgb,
    native_selector, native_state_at, native_state_display, native_state_final,
    native_state_update, native_state_update_with, native_str, native_str_from_unicode,
    native_type,
};
// P506 — state/counter/context como valores de primeira classe.
pub use crate::rules::stdlib::counter::{native_counter, counter_at, counter_display, counter_get, counter_step, counter_update};
pub use crate::rules::stdlib::context::native_context;
pub use crate::rules::stdlib::state::{native_state, state_display, state_get, state_update, value_to_content};
pub use crate::rules::stdlib::label::native_label;
pub use crate::rules::stdlib::panic::native_panic;
pub use crate::rules::stdlib::r#ref::native_ref;
pub use crate::rules::stdlib::structural::{
    make_math_module, native_accent, native_asset, native_bibliography, native_cancel,
    native_cite, native_divider, native_document, native_emph, native_enum, native_footnote,
    native_grid_cell, native_grid_footer, native_grid_header, native_grid_hline, native_grid_vline, native_heading,
    native_link, native_list, native_lof, native_lot, native_op, native_outline, native_quote, native_raw, native_strong,
    native_table, native_table_cell, native_table_footer, native_table_header, native_table_hline, native_table_vline, native_terms,
    native_underover,
};
pub use crate::rules::stdlib::text::{
    native_highlight, native_lorem, native_lower, native_overline, native_regex, native_replace,
    native_smallcaps, native_smartquote, native_strike, native_subscript, native_superscript,
    native_text, native_underline, native_upper,
};
// P387 (ADR-0111) — data import.
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_colbreak, native_columns, native_grid,
    native_h, native_hide, native_measure, native_pad, native_pagebreak, native_place,
    native_repeat, native_stack, native_stroke, native_v,
};
pub use crate::rules::stdlib::loading::{
    native_cbor, native_csv, native_json, native_read, native_toml, native_xml,
    native_yaml,
};
pub use crate::rules::stdlib::shapes::{
    native_circle, native_curve, native_curve_close, native_curve_cubic, native_curve_line,
    native_curve_move, native_curve_quad, native_ellipse, native_line, native_polygon,
    native_rect, native_square,
};
pub use crate::rules::stdlib::transforms::{
    native_move, native_rotate, native_scale, native_skew,
};
// P262 — Gradient stdlib (Linear only per ADR-0087).
// P264 — Gradient stdlib Radial added per ADR-0088.
// P267 — Gradient stdlib Conic added per ADR-0089 (cluster 3/3).
pub use crate::rules::stdlib::gradients::{
    make_gradient_module, native_gradient_conic, native_gradient_linear,
    native_gradient_radial,
};
// P396 — constructor `tiling(...)`.
pub use crate::rules::stdlib::visualize::native_tiling;
// P403 — constructors stdlib para tipos primitivos L1.
pub use crate::rules::stdlib::primitives_constructors::{
    native_decimal, native_duration, native_version,
};
// P466 — dispatcher de métodos de array/dict/str.
pub(crate) use crate::rules::stdlib::collections::try_dispatch_collection_method;
// P471 — módulo sym.
pub use crate::rules::stdlib::sym::build_sym_dict;
// P476 — módulo color.
pub use crate::rules::stdlib::color::{make_color_module, predefined_color_bindings};
// P694 — módulo sys.
pub use crate::rules::stdlib::sys::make_sys_module;
// P311b.3 — 12 funções math style (paridade categoria 12/12 = 100%).
pub use crate::rules::stdlib::math_style::{
    native_bb, native_bold, native_cal, native_frak, native_math_italic, native_mono,
    native_sans, native_scr, native_script, native_serif, native_sscript, native_upright,
};

// ── Helpers partilhados ─────────────────────────────────────────────────────

/// Constrói um `SourceResult::Err` com mensagem única e span detached.
pub(super) fn err(msg: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Verifica que não foram passados argumentos nomeados não esperados (Passo 64).
///
/// O Typst original é rigoroso: argumentos nomeados desconhecidos são
/// erros semânticos, não silenciosos. Ignorá-los criaria uma linguagem
/// permissiva que esconde typos do utilizador.
pub(super) fn expect_no_named(
    named: &IndexMap<EcoString, Value, FxBuildHasher>,
) -> SourceResult<()> {
    if let Some((key, _)) = named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("argumento nomeado inesperado: '{}'", key),
        )]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::calc::{
        calc_abs,
        calc_acos,
        calc_acosh,
        calc_asin,
        calc_asinh,
        calc_atan,
        calc_atan2,
        calc_atanh,
        calc_binom,
        calc_ceil,
        calc_clamp,
        calc_cos,
        calc_cosh,
        calc_div_euclid,
        // P308 — função erro de Gauss (paridade calc 41/41).
        calc_erf,
        calc_even,
        calc_exp,
        calc_fact,
        calc_floor,
        calc_fract,
        calc_gcd,
        calc_lcm,
        calc_ln,
        calc_log,
        calc_max,
        calc_min,
        calc_norm,
        calc_odd,
        calc_perm,
        calc_pow,
        calc_quo,
        calc_rem,
        calc_rem_euclid,
        calc_root,
        calc_round,
        // P283 — trig/hyperbolic/log/exp.
        calc_sin,
        calc_sinh,
        calc_sqrt,
        calc_tan,
        calc_tanh,
        // P306 — aritmética inteira, combinatória, norma, raiz.
        calc_trunc,
        make_calc_module as p283_make_calc_module,
    };
    use super::shapes::parse_color;
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::args::Args;
    use crate::entities::content::Content;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Color, Length};
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };
    use crate::rules::eval::EvalContext;
    use crate::rules::scopes::Scopes;
    use comemo::{Track, TrackedMut};
    use std::num::NonZeroU16;
    use std::sync::Arc;

    /// Helper de teste: cria Args apenas com posicionais.
    fn p(items: Vec<Value>) -> Args {
        Args::positional(items)
    }

    /// Helper de teste: cria Args com um named arg.
    fn pn(items: Vec<Value>, key: &str, val: Value) -> Args {
        let mut a = Args::positional(items);
        a.named.insert(key.into(), val);
        a
    }

    /// Mundo nulo para testes de stdlib que não precisam de I/O.
    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
        files: std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
    }
    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
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
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            self.files
                .get(path)
                .map(std::sync::Arc::clone)
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    /// Helper que cria um EvalContext nulo para tests que não usam o ctx.
    /// Passo 109 (ADR-0044): `world` passou para o Engine/ABI;
    /// `EvalContext::new()` deixou de o receber.
    macro_rules! null_ctx {
        ($ctx:ident) => {
            let mut $ctx = EvalContext::new();
        };
    }

    /// Helper que cria um Engine mínimo para tests de `native_eval`.
    ///
    /// Usa um callback em vez de macro para contornar a higiene de macros do
    /// Rust: as variáveis `world`, `styles`, etc. são owned pela closure e
    /// emprestadas ao `Engine` apenas durante a execução de `f`.
    fn with_engine<R>(f: impl FnOnce(&mut Engine<'_>, &NullWorld) -> R) -> R {
        let world = null_world();
        let route = Route::root();
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards: Vec<RuleId> = Vec::new();
        let mut sink_local = Sink::new();
        let mut sink = sink_local.track_mut();
        let mut engine = Engine {
            world: &world,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: test_file_id(),
            sink: &mut sink,
        };
        f(&mut engine, &world)
    }

    /// Helper que cria um World nulo para tests que passam para o ABI.
    fn null_world() -> NullWorld {
        NullWorld::default()
    }

    /// Helper: `FileId` dummy para tests que não fazem I/O.
    fn test_file_id() -> crate::entities::file_id::FileId {
        crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn native_type_directo() {
        null_ctx!(ctx);
        // P685 — type() devolve Value::Type, não Value::Str.
        assert_eq!(
            native_type(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Type(crate::entities::value::Type::Int)
        );
        assert_eq!(
            native_type(
                &mut ctx,
                &p(vec![Value::Bool(true)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Type(crate::entities::value::Type::Bool)
        );
        assert_eq!(
            native_type(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id())
                .unwrap(),
            Value::Type(crate::entities::value::Type::None)
        );
        assert!(native_type(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
        assert!(native_type(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn native_type_named_arg_retorna_err() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Int(1)], "extra", Value::Bool(true));
        assert!(
            native_type(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
            "named arg inesperado deve retornar Err"
        );
    }

    #[test]
    fn native_len_directo() {
        null_ctx!(ctx);
        assert_eq!(
            native_len(
                &mut ctx,
                &p(vec![Value::Str("abc".into())]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
        assert_eq!(
            native_len(
                &mut ctx,
                &p(vec![Value::Array(vec![Value::Int(1), Value::Int(2)])]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(2)
        );
        assert!(native_len(
            &mut ctx,
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(native_len(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    }

    // ── Passo 25 — rgb/luma ──────────────────────────────────────────────────

    #[test]
    fn stdlib_rgb_tres_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(
            &mut ctx,
            &p(vec![Value::Int(255), Value::Int(0), Value::Int(128)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Color(Color::rgb(255, 0, 128)));
    }

    #[test]
    fn stdlib_rgb_quatro_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(
            &mut ctx,
            &p(vec![Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Color(Color::rgba(255, 0, 0, 200)));
    }

    #[test]
    fn stdlib_rgb_out_of_range() {
        null_ctx!(ctx);
        assert!(native_rgb(
            &mut ctx,
            &p(vec![Value::Int(300), Value::Int(0), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn stdlib_luma() {
        // P257 (ADR-0083 PROPOSTO) — `luma()` agora constrói
        // `Color::Luma` (paridade vanilla D65Gray); paridade
        // observable preservada via `to_srgb()` que expande Luma
        // para sRGB cinza bit-equivalente.
        null_ctx!(ctx);
        let r = native_luma(
            &mut ctx,
            &p(vec![Value::Int(128)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, a_) = c.to_srgb();
            assert_eq!(r_, 128);
            assert_eq!(g_, 128);
            assert_eq!(b_, 128);
            assert_eq!(a_, 255);
        } else {
            panic!("esperado Value::Color");
        }
    }

    // ── P257 (ADR-0083 PROPOSTO) — stdlib funcs novas para espaços
    //     materializados (oklab/oklch/linear_rgb/cmyk/hsl/hsv) ──

    #[test]
    fn p257_native_oklab_3_args() {
        null_ctx!(ctx);
        let r = native_oklab(
            &mut ctx,
            &p(vec![Value::Float(0.5), Value::Float(0.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            // L=0.5 → cinza médio aproximado.
            let (r_, g_, b_, _) = c.to_srgb();
            assert!(
                r_ > 80 && r_ < 180,
                "oklab(0.5, 0, 0) → cinza médio; obtido r={}",
                r_
            );
            // a=b=0 → sem chroma → r ≈ g ≈ b.
            assert!((r_ as i32 - g_ as i32).abs() <= 3);
            assert!((g_ as i32 - b_ as i32).abs() <= 3);
        } else {
            panic!("esperado Value::Color");
        }
    }

    #[test]
    fn p257_native_oklab_4_args_com_alpha() {
        null_ctx!(ctx);
        let r = native_oklab(
            &mut ctx,
            &p(vec![
                Value::Float(1.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.5),
            ]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (_, _, _, a_) = c.to_srgb();
            // alpha=0.5 → 127 ou 128 conforme arredondamento.
            assert!(a_ == 127 || a_ == 128, "alpha=0.5 → 127/128; obtido {}", a_);
        } else {
            panic!("esperado Value::Color");
        }
    }

    #[test]
    fn p257_native_oklch_3_args() {
        null_ctx!(ctx);
        let r = native_oklch(
            &mut ctx,
            &p(vec![Value::Float(0.5), Value::Float(0.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(r, Value::Color(_)));
    }

    #[test]
    fn p257_native_linear_rgb_3_args() {
        null_ctx!(ctx);
        let r = native_linear_rgb(
            &mut ctx,
            &p(vec![Value::Float(1.0), Value::Float(0.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 0);
            assert_eq!(b_, 0);
        } else {
            panic!("esperado Value::Color");
        }
    }

    #[test]
    fn p257_native_cmyk_branco() {
        null_ctx!(ctx);
        let r = native_cmyk(
            &mut ctx,
            &p(vec![
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 255);
            assert_eq!(b_, 255);
        } else {
            panic!("esperado Value::Color");
        }
    }

    #[test]
    fn p257_native_cmyk_4_args_obrigatorios() {
        null_ctx!(ctx);
        let r = native_cmyk(
            &mut ctx,
            &p(vec![Value::Float(0.0), Value::Float(0.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "cmyk requer 4 args; 3 args deve falhar");
    }

    #[test]
    fn p257_native_hsl_vermelho_puro() {
        null_ctx!(ctx);
        let r = native_hsl(
            &mut ctx,
            &p(vec![Value::Float(0.0), Value::Float(1.0), Value::Float(0.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 0);
            assert_eq!(b_, 0);
        } else {
            panic!("esperado Value::Color");
        }
    }

    #[test]
    fn p257_native_hsv_branco_s0_v1() {
        null_ctx!(ctx);
        let r = native_hsv(
            &mut ctx,
            &p(vec![Value::Float(0.0), Value::Float(0.0), Value::Float(1.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 255);
            assert_eq!(b_, 255);
        } else {
            panic!("esperado Value::Color");
        }
    }

    // ── P175 (M9 sub-passo 5) — query(kind_str) ─────────────────────────

    #[test]
    fn stdlib_query_em_introspector_vazio_retorna_array_vazio() {
        // P175 → P179 upgrade: query retorna Value::Array, não Int.
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Array(vec![]));
    }

    #[test]
    fn stdlib_query_em_introspector_populado_retorna_array_de_locations() {
        // P179 upgrade: array com Value::Location entries.
        null_ctx!(ctx);
        use crate::entities::element_kind::ElementKind;
        use crate::entities::location::Location;
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .extend(vec![
                Location::from_raw(1),
                Location::from_raw(2),
                Location::from_raw(3),
            ]);

        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let expected = Value::Array(vec![
            Value::Location(Location::from_raw(1)),
            Value::Location(Location::from_raw(2)),
            Value::Location(Location::from_raw(3)),
        ]);
        assert_eq!(r, expected);
    }

    #[test]
    fn stdlib_query_kind_invalido_retorna_err() {
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("nao_existe".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "kind inválido deve retornar Err");
    }

    #[test]
    fn stdlib_query_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "arg não-string deve retornar Err");
    }

    #[test]
    fn stdlib_query_outline_funciona_pos_p178_p179() {
        // P178: query("outline") agora aceita kind.
        // P179: retorna array, não count.
        null_ctx!(ctx);
        use crate::entities::element_kind::ElementKind;
        use crate::entities::location::Location;
        // Vazio.
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("outline".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Array(vec![]));
        // Populado.
        ctx.introspector
            .kind_index
            .entry(ElementKind::Outline)
            .or_default()
            .push(Location::from_raw(10));
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("outline".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Array(vec![Value::Location(Location::from_raw(10))]));
    }

    #[test]
    fn stdlib_query_p179_upgrade_value_location_type_name() {
        // P179: Value::Location existe e tem type_name correcto.
        use crate::entities::location::Location;
        let v = Value::Location(Location::from_raw(42));
        assert_eq!(v.type_name(), "location");
    }

    // ── P208B (M9c Bloco IV) — here() infra minimal ─────────────────────

    #[test]
    fn p208b_here_sem_current_location_retorna_err_contextual() {
        // Pre-population: EvalContext::new() default current_location = None.
        // here() retorna erro contextual coerente (não panic).
        null_ctx!(ctx);
        let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "here() sem current_location deve falhar");
        // Erro contém menção a "here()" ou similar — não validar texto exacto.
    }

    #[test]
    fn p208b_here_com_current_location_retorna_value_location() {
        // Setter conveniente: with_current_location.
        use crate::entities::location::Location;
        let mut ctx = EvalContext::new().with_current_location(Location::from_raw(42));
        let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(42)));
        assert_eq!(r.type_name(), "location");
    }

    #[test]
    fn p208b_here_com_args_retorna_err() {
        // here() não aceita argumentos (paridade vanilla: sem args).
        use crate::entities::location::Location;
        let mut ctx = EvalContext::new().with_current_location(Location::from_raw(1));
        let r = native_here(
            &mut ctx,
            &p(vec![Value::Int(99)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "here(99) deve falhar — sem args");
    }

    #[test]
    fn p208b_here_field_writable_directamente() {
        // Pattern alternativo: write directo ao field (sem setter).
        // Útil quando consumer já tem &mut EvalContext.
        use crate::entities::location::Location;
        null_ctx!(ctx);
        assert!(ctx.current_location.is_none());

        ctx.current_location = Some(Location::from_raw(7));
        let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(7)));
    }

    // ── P208C (M9c Bloco IV) — locate(kind) ──────────────────────────

    #[test]
    fn p208c_locate_kind_existente_retorna_some_location() {
        // Popula introspector com 2 headings; locate("heading")
        // retorna a PRIMEIRA Location (Vec::first).
        use crate::entities::element_kind::ElementKind;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(20));

        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(10)));
        assert_eq!(r.type_name(), "location");
    }

    #[test]
    fn p208c_locate_kind_inexistente_retorna_none() {
        // Introspector vazio; locate("figure") retorna Value::None
        // (não Err — kind é válido, só não há matches).
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("figure".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p208c_locate_kind_invalido_retorna_err() {
        // Kind inválido → erro contextual coerente.
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "kind inválido deve falhar");
    }

    #[test]
    fn p208c_locate_arg_nao_string_retorna_err_com_hint_p209() {
        // Arg não-string e não-Location → erro com hint.
        // **Actualizado em P209B**: `Value::Location` agora é
        // dispatched (não é mais erro). `Value::Int` continua a
        // ser erro — hint actual menciona Regex pendente (P209D)
        // e And/Or Rust-only.
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "arg não-string/location deve falhar");
    }

    // ── P209B (M9c Bloco VI) — Selector::Label + ::Location dispatch ──

    #[test]
    fn p209b_locate_label_syntax_retorna_some_location() {
        // <nome> syntax → Selector::Label. Popula introspector
        // com label "intro" → loc(7); locate("<intro>") deve
        // retornar Value::Location(loc(7)).
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector
            .labels
            .add(Label("intro".to_string()), Location::from_raw(7));
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("<intro>".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(7)));
    }

    #[test]
    fn p209b_locate_label_inexistente_retorna_none() {
        // <nome> que não existe → query devolve Vec vazio →
        // locate() devolve Value::None.
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("<ausente>".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p209b_query_location_arg_devolve_singleton() {
        // Value::Location → Selector::Location → singleton
        // [loc]. query() retorna Value::Array com 1 entry.
        use crate::entities::location::Location;
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Location(Location::from_raw(42))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Array(vec![Value::Location(Location::from_raw(42))]),);
    }

    #[test]
    fn p209b_locate_location_arg_retorna_propria_location() {
        // Selector::Location é singleton trivial → locate(loc)
        // retorna Value::Location(loc).
        use crate::entities::location::Location;
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Location(Location::from_raw(99))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(99)));
    }

    #[test]
    fn p209b_query_label_via_introspector_directo() {
        // Test API directo: Introspector::query(Selector::Label)
        // delega a query_by_label.
        use crate::entities::introspector::Introspector;
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        null_ctx!(ctx);
        ctx.introspector
            .labels
            .add(Label("a".to_string()), Location::from_raw(1));
        let r = ctx.introspector.query(&Selector::Label(Label("a".to_string())));
        assert_eq!(r, vec![Location::from_raw(1)]);
        // Label inexistente → vazio.
        let r2 = ctx.introspector.query(&Selector::Label(Label("b".to_string())));
        assert!(r2.is_empty());
    }

    // ── P209C (M9c Bloco VI) — Selector::And + Or query semantics ─────

    #[test]
    fn p209c_query_and_vazio_devolve_empty() {
        // Opção A: And(vec![]) → empty Vec.
        use crate::entities::introspector::Introspector;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        let r = ctx.introspector.query(&Selector::And(EcoVec::new()));
        assert_eq!(r, Vec::<Location>::new());
    }

    #[test]
    fn p209c_query_or_vazio_devolve_empty() {
        use crate::entities::introspector::Introspector;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        let r = ctx.introspector.query(&Selector::Or(EcoVec::new()));
        assert_eq!(r, Vec::<Location>::new());
    }

    #[test]
    fn p209c_query_and_interseccao_de_dois() {
        // Popular: heading na loc 7 com label "a"; figure na loc 10
        // sem label. And([Kind(heading), Label(a)]) → [loc(7)].
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(7));
        ctx.introspector
            .kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector
            .labels
            .add(Label("a".to_string()), Location::from_raw(7));

        let r = ctx.introspector.query(&Selector::And(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Label(Label("a".to_string())),
        ])));
        // Apenas loc(7) está em ambos.
        assert_eq!(r, vec![Location::from_raw(7)]);

        // Intersecção vazia: label que não existe.
        let r_empty = ctx.introspector.query(&Selector::And(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Label(Label("zzz".to_string())),
        ])));
        assert!(r_empty.is_empty());
    }

    #[test]
    fn p209c_query_or_uniao_dedupliquada() {
        // Popular: 2 headings (loc 1, 2), 1 figure (loc 3).
        // Or([Kind(heading), Kind(figure)]) → união ordenada
        // [1, 2, 3].
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(1));
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(2));
        ctx.introspector
            .kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(3));

        let r = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Kind(ElementKind::Figure),
        ])));
        assert_eq!(
            r,
            vec![Location::from_raw(1), Location::from_raw(2), Location::from_raw(3),],
        );

        // Dedup: Or com mesma kind 2× não duplica.
        let r_dedup = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Kind(ElementKind::Heading),
        ])));
        assert_eq!(r_dedup, vec![Location::from_raw(1), Location::from_raw(2)],);
    }

    // ── P210B (M9c Bloco V) — counter_step Q1=β subset ──────────────

    #[test]
    fn p210b_counter_step_basico() {
        // counter_step("foo") devolve Value::Content(Content::CounterUpdate
        // { key: "foo", action: Step }).
        use crate::entities::content::Content;
        use crate::entities::counter_update::CounterUpdate as CounterAction;
        null_ctx!(ctx);
        let r = native_counter_step(
            &mut ctx,
            &p(vec![Value::Str("foo".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        match r {
            Value::Content(Content::CounterUpdate(e)) => {
                assert_eq!(e.key, "foo");
                assert_eq!(e.action, CounterAction::Step);
            }
            _ => panic!("expected Value::Content(CounterUpdate)"),
        }
    }

    #[test]
    fn p210b_counter_step_arg_invalido_retorna_err() {
        // counter_step(42) — Value::Int não é Value::Str.
        null_ctx!(ctx);
        let r = native_counter_step(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "arg não-string deve falhar");
    }

    #[test]
    fn p210b_counter_step_sem_args_retorna_err() {
        null_ctx!(ctx);
        let r = native_counter_step(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "sem args deve falhar");
    }

    #[test]
    fn p210b_counter_step_multipla_invocacao_estruturalmente_igual() {
        // Duas invocações com mesma key produzem Content
        // estructuralmente iguais (sem state shared no stdlib func).
        use crate::entities::content::Content;
        null_ctx!(ctx);
        let r1 = native_counter_step(
            &mut ctx,
            &p(vec![Value::Str("h".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let r2 = native_counter_step(
            &mut ctx,
            &p(vec![Value::Str("h".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // PartialEq sobre Content::CounterUpdate (per content.rs:1203).
        match (r1, r2) {
            (Value::Content(c1), Value::Content(c2)) => {
                assert!(matches!(
                    (&c1, &c2),
                    (Content::CounterUpdate(_), Content::CounterUpdate(_))
                ));
                // Via match interno + comparison.
                let same = match (&c1, &c2) {
                    (Content::CounterUpdate(e1), Content::CounterUpdate(e2)) => {
                        e1.key == e2.key && e1.action == e2.action
                    }
                    _ => false,
                };
                assert!(same, "invocações idempotentes esperadas");
            }
            _ => panic!("expected Value::Content"),
        }
    }

    // ── P209D (M9c Bloco VI) — Selector::Regex query stub ───────────

    #[test]
    fn p209d_introspector_query_regex_devolve_empty_stub() {
        // Stub documentado: Regex variant retorna vec![] em query.
        use crate::entities::introspector::Introspector;
        use crate::entities::location::Location;
        use crate::entities::regex::Regex;
        use crate::entities::selector::Selector;
        null_ctx!(ctx);
        let r = ctx.introspector.query(&Selector::Regex(Regex::new("\\d+").unwrap()));
        assert_eq!(r, Vec::<Location>::new());
    }

    #[test]
    fn p209d_introspector_query_regex_in_or_compoe_com_kind() {
        // Composição estrutural funciona: Or([Regex, Kind]) →
        // união. Regex retorna empty (stub); Kind devolve seus
        // matches. Result = matches do Kind apenas.
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::location::Location;
        use crate::entities::regex::Regex;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(1));

        let r = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
            Selector::Regex(Regex::new("abc").unwrap()),
            Selector::Kind(ElementKind::Heading),
        ])));
        // Regex stub: empty. Kind: [loc(1)]. Union dedup: [loc(1)].
        assert_eq!(r, vec![Location::from_raw(1)]);
    }

    #[test]
    fn p209c_query_nested_or_dentro_de_and() {
        // And([Or([Kind(heading), Kind(figure)]), Label("a")]).
        // Popular: heading loc 7 com label "a"; figure loc 10 sem
        // label. Resultado: [loc(7)] (a ∈ {heading, figure} E label a).
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        use crate::entities::selector::Selector;
        use ecow::EcoVec;
        null_ctx!(ctx);
        ctx.introspector
            .kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(7));
        ctx.introspector
            .kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector
            .labels
            .add(Label("a".to_string()), Location::from_raw(7));

        let inner_or = Selector::Or(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Kind(ElementKind::Figure),
        ]));
        let nested = Selector::And(EcoVec::from(vec![
            inner_or,
            Selector::Label(Label("a".to_string())),
        ]));
        let r = ctx.introspector.query(&nested);
        assert_eq!(r, vec![Location::from_raw(7)]);
    }

    // ── P393 — `regex(pattern)` (construtor L1 para show rules regex) ───

    #[test]
    fn p393_regex_construtor_valido_devolve_value_regex() {
        null_ctx!(ctx);
        let r = native_regex(
            &mut ctx,
            &p(vec![Value::Str("\\d+".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(
            matches!(r, Value::Regex(_)),
            "regex(\"\\d+\") deve devolver Value::Regex: {:?}",
            r
        );
    }

    #[test]
    fn p393_regex_construtor_invalido_erro() {
        null_ctx!(ctx);
        let r = native_regex(
            &mut ctx,
            &p(vec![Value::Str("[".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "regex(\"[\") deve falhar");
    }

    #[test]
    fn p393_regex_construtor_tipo_errado_erro() {
        null_ctx!(ctx);
        let r = native_regex(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "regex(42) deve falhar");
    }

    #[test]
    fn p393_regex_construtor_arg_nomeado_rejeitado() {
        null_ctx!(ctx);
        let r = native_regex(
            &mut ctx,
            &pn(vec![Value::Str("\\d+".into())], "foo", Value::Int(1)),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "regex com arg nomeado deve falhar");
    }

    // ── P176 (M9 sub-passo 6) — counter_final(key) ──────────────────────

    #[test]
    fn stdlib_counter_final_em_introspector_vazio_retorna_str_vazia() {
        null_ctx!(ctx);
        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_final_em_introspector_populado_retorna_string_formatada() {
        null_ctx!(ctx);
        // Popular CounterRegistry directamente com hierarquia [1, 2, 1].
        ctx.introspector.counters.apply_hierarchical("heading".to_string(), 1);
        ctx.introspector.counters.apply_hierarchical("heading".to_string(), 2);
        ctx.introspector.counters.apply_hierarchical("heading".to_string(), 1);

        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // P170: hierarchical "1.2.1" → "2" após [1,2,1] (último top-level).
        // Ajustar conforme paridade real — confirma que retorna string
        // não vazia formatada.
        match r {
            Value::Str(s) => {
                assert!(!s.is_empty(), "esperado string não-vazia, recebido vazia");
            }
            other => panic!("esperado Value::Str, recebido {:?}", other),
        }
    }

    #[test]
    fn stdlib_counter_final_key_inexistente_retorna_str_vazia() {
        null_ctx!(ctx);
        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_final_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "arg não-string deve retornar Err");
    }

    // ── P177 (M9 sub-passo 7) — counter_at(key, label) ──────────────────

    #[test]
    fn stdlib_counter_at_em_introspector_vazio_retorna_str_vazia() {
        null_ctx!(ctx);
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_at_label_inexistente_retorna_str_vazia() {
        null_ctx!(ctx);
        // Popular counter mas label não registada.
        use crate::entities::location::Location;
        ctx.introspector.counters.apply_hierarchical_at(
            "heading".to_string(),
            1,
            Location::from_raw(10),
        );
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Str("nonexistent".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_at_label_associado_retorna_string_formatada() {
        null_ctx!(ctx);
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        // Popular: heading na loc 10 com counter [1], label "intro" → loc 10.
        ctx.introspector.counters.apply_hierarchical_at(
            "heading".to_string(),
            1,
            Location::from_raw(10),
        );
        ctx.introspector
            .labels
            .add(Label("intro".to_string()), Location::from_raw(10));

        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("1".into()));
    }

    #[test]
    fn stdlib_counter_at_args_invalidos_retornam_err() {
        null_ctx!(ctx);
        // Primeiro arg não-string.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
        // Segundo arg não-string.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Int(1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
        // Número errado de args.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
    }

    // ── Passo 27 — str/int/float ─────────────────────────────────────────────

    #[test]
    fn native_str_de_int() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("42".into())
        );
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_str_de_float() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(
                &mut ctx,
                &p(vec![Value::Float(3.14)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Str("3.14".into())
        );
    }

    #[test]
    fn native_str_de_bool() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(
                &mut ctx,
                &p(vec![Value::Bool(true)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Str("true".into())
        );
        assert_eq!(
            native_str(
                &mut ctx,
                &p(vec![Value::Bool(false)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Str("false".into())
        );
    }

    #[test]
    fn native_str_identity() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(
                &mut ctx,
                &p(vec![Value::Str("hello".into())]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Str("hello".into())
        );
    }

    #[test]
    fn native_str_de_none() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("none".into())
        );
    }

    // ── P495 — str(base:) ────────────────────────────────────────────────────

    #[test]
    fn p495_str_base_named() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(&mut ctx, &pn(vec![Value::Int(255)], "base", Value::Int(16)), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("ff".into())
        );
        assert_eq!(
            native_str(&mut ctx, &pn(vec![Value::Int(42)], "base", Value::Int(2)), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("101010".into())
        );
        assert_eq!(
            native_str(&mut ctx, &p(vec![Value::Int(255)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("255".into())
        );
    }

    #[test]
    fn p495_str_base_negativo() {
        null_ctx!(ctx);
        assert_eq!(
            native_str(&mut ctx, &pn(vec![Value::Int(-255)], "base", Value::Int(16)), &null_world(), test_file_id())
                .unwrap(),
            Value::Str("-ff".into())
        );
    }

    #[test]
    fn p495_str_base_invalida() {
        null_ctx!(ctx);
        assert!(native_str(&mut ctx, &pn(vec![Value::Int(255)], "base", Value::Int(37)), &null_world(), test_file_id()).is_err());
        assert!(native_str(&mut ctx, &pn(vec![Value::Int(255)], "base", Value::Int(1)), &null_world(), test_file_id()).is_err());
        assert!(native_str(&mut ctx, &pn(vec![Value::Float(3.14)], "base", Value::Int(16)), &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn p495_str_named_desconhecido_rejeitado() {
        null_ctx!(ctx);
        assert!(native_str(&mut ctx, &pn(vec![Value::Int(255)], "foo", Value::Int(16)), &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn native_int_de_int() {
        null_ctx!(ctx);
        assert_eq!(
            native_int(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(42)
        );
    }

    #[test]
    fn native_int_de_str() {
        null_ctx!(ctx);
        assert_eq!(
            native_int(
                &mut ctx,
                &p(vec![Value::Str("42".into())]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(42)
        );
        assert!(native_int(
            &mut ctx,
            &p(vec![Value::Str("abc".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn native_int_de_bool() {
        null_ctx!(ctx);
        assert_eq!(
            native_int(
                &mut ctx,
                &p(vec![Value::Bool(true)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            native_int(
                &mut ctx,
                &p(vec![Value::Bool(false)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
    }

    #[test]
    fn native_int_float_retorna_err() {
        null_ctx!(ctx);
        assert!(native_int(
            &mut ctx,
            &p(vec![Value::Float(3.7)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn native_float_de_int() {
        null_ctx!(ctx);
        assert_eq!(
            native_float(
                &mut ctx,
                &p(vec![Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(3.0)
        );
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_float_de_str() {
        null_ctx!(ctx);
        assert_eq!(
            native_float(
                &mut ctx,
                &p(vec![Value::Str("3.14".into())]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(3.14)
        );
        assert!(native_float(
            &mut ctx,
            &p(vec![Value::Str("abc".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── Passo 27 — calc ──────────────────────────────────────────────────────

    #[test]
    fn calc_abs_int() {
        null_ctx!(ctx);
        assert_eq!(
            calc_abs(&mut ctx, &p(vec![Value::Int(-5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            calc_abs(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            calc_abs(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(0)
        );
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn calc_abs_float() {
        null_ctx!(ctx);
        assert_eq!(
            calc_abs(
                &mut ctx,
                &p(vec![Value::Float(-3.14)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(3.14)
        );
    }

    #[test]
    fn calc_pow_int() {
        null_ctx!(ctx);
        assert_eq!(
            calc_pow(
                &mut ctx,
                &p(vec![Value::Int(2), Value::Int(10)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1024)
        );
        assert_eq!(
            calc_pow(
                &mut ctx,
                &p(vec![Value::Int(2), Value::Int(0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
    }

    #[test]
    fn calc_pow_float() {
        null_ctx!(ctx);
        let r = calc_pow(
            &mut ctx,
            &p(vec![Value::Float(2.0), Value::Float(0.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(
            matches!(r, Value::Float(f) if (f - std::f64::consts::SQRT_2).abs() < 1e-10)
        );
    }

    #[test]
    fn calc_pow_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_pow(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(-1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_sqrt_positivo() {
        null_ctx!(ctx);
        assert_eq!(
            calc_sqrt(
                &mut ctx,
                &p(vec![Value::Float(4.0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(2.0)
        );
        assert_eq!(
            calc_sqrt(&mut ctx, &p(vec![Value::Int(4)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Float(2.0)
        );
    }

    #[test]
    fn calc_sqrt_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_sqrt(
            &mut ctx,
            &p(vec![Value::Float(-1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_floor_ceil_round() {
        null_ctx!(ctx);
        assert_eq!(
            calc_floor(
                &mut ctx,
                &p(vec![Value::Float(3.7)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
        assert_eq!(
            calc_ceil(
                &mut ctx,
                &p(vec![Value::Float(3.2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(4)
        );
        assert_eq!(
            calc_round(
                &mut ctx,
                &p(vec![Value::Float(3.5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(4)
        );
        assert_eq!(
            calc_round(
                &mut ctx,
                &p(vec![Value::Float(3.4)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
    }

    // ── P495 — calc.round(digits:) ───────────────────────────────────────────

    #[test]
    fn p495_calc_round_digits_named() {
        null_ctx!(ctx);
        approx_float(
            calc_round(
                &mut ctx,
                &pn(vec![Value::Float(3.567)], "digits", Value::Int(2)),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.57,
        );
        approx_float(
            calc_round(
                &mut ctx,
                &pn(vec![Value::Float(1234.0)], "digits", Value::Int(-2)),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1200.0,
        );
        // default 0 preservado
        assert_eq!(
            calc_round(&mut ctx, &p(vec![Value::Float(3.5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(4)
        );
    }

    #[test]
    fn p495_calc_round_digits_int_input() {
        null_ctx!(ctx);
        approx_float(
            calc_round(
                &mut ctx,
                &pn(vec![Value::Int(255)], "digits", Value::Int(2)),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            255.0,
        );
    }

    #[test]
    fn p495_calc_round_named_desconhecido_rejeitado() {
        null_ctx!(ctx);
        assert!(calc_round(&mut ctx, &pn(vec![Value::Float(3.5)], "foo", Value::Int(2)), &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn calc_min_max_int() {
        null_ctx!(ctx);
        assert_eq!(
            calc_min(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_max(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
    }

    #[test]
    fn calc_min_vazio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_min(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
        assert!(calc_max(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn calc_clamp_int() {
        null_ctx!(ctx);
        assert_eq!(
            calc_clamp(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(0), Value::Int(10)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            calc_clamp(
                &mut ctx,
                &p(vec![Value::Int(-5), Value::Int(0), Value::Int(10)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
        assert_eq!(
            calc_clamp(
                &mut ctx,
                &p(vec![Value::Int(15), Value::Int(0), Value::Int(10)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(10)
        );
    }

    #[test]
    fn calc_clamp_min_maior_max_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_clamp(
            &mut ctx,
            &p(vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── Passo 283 — calc trig / hiperbólicas / log / exp ─────────────────────
    //
    // Helper: float ≈ esperado dentro de 1e-10 (tolerância empírica suficiente
    // para o ruído de `f64::sin/cos/...`).
    fn approx_float(v: Value, expected: f64) {
        match v {
            Value::Float(f) => {
                assert!((f - expected).abs() < 1e-10, "esperado {expected}, obtido {f}",)
            }
            other => panic!("esperado Value::Float, obtido {other:?}"),
        }
    }

    // ── Trigonometria ────────────────────────────────────────────────────────

    #[test]
    fn calc_sin_zero_e_pi() {
        null_ctx!(ctx);
        approx_float(
            calc_sin(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_sin(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            0.0,
        );
        approx_float(
            calc_sin(
                &mut ctx,
                &p(vec![Value::Float(std::f64::consts::PI)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
    }

    #[test]
    fn calc_sin_arity_errada_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_sin(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
        assert!(calc_sin(
            &mut ctx,
            &p(vec![Value::Float(1.0), Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_cos_zero_e_pi() {
        null_ctx!(ctx);
        approx_float(
            calc_cos(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
        approx_float(
            calc_cos(
                &mut ctx,
                &p(vec![Value::Float(std::f64::consts::PI)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            -1.0,
        );
    }

    #[test]
    fn calc_tan_zero_e_pi_quarto() {
        null_ctx!(ctx);
        approx_float(
            calc_tan(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_tan(
                &mut ctx,
                &p(vec![Value::Float(std::f64::consts::FRAC_PI_4)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
    }

    #[test]
    fn calc_asin_dominio_valido() {
        null_ctx!(ctx);
        approx_float(
            calc_asin(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_asin(
                &mut ctx,
                &p(vec![Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::FRAC_PI_2,
        );
        approx_float(
            calc_asin(
                &mut ctx,
                &p(vec![Value::Float(-1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            -std::f64::consts::FRAC_PI_2,
        );
    }

    #[test]
    fn calc_asin_fora_dominio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_asin(
            &mut ctx,
            &p(vec![Value::Float(1.5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_asin(
            &mut ctx,
            &p(vec![Value::Float(-1.5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_acos_dominio_valido() {
        null_ctx!(ctx);
        approx_float(
            calc_acos(
                &mut ctx,
                &p(vec![Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_acos(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::FRAC_PI_2,
        );
        approx_float(
            calc_acos(
                &mut ctx,
                &p(vec![Value::Float(-1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::PI,
        );
    }

    #[test]
    fn calc_acos_fora_dominio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_acos(
            &mut ctx,
            &p(vec![Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_acos(
            &mut ctx,
            &p(vec![Value::Float(-2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_atan_basico() {
        null_ctx!(ctx);
        approx_float(
            calc_atan(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_atan(
                &mut ctx,
                &p(vec![Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::FRAC_PI_4,
        );
    }

    #[test]
    fn calc_atan2_ordem_xy_paridade_vanilla() {
        null_ctx!(ctx);
        // calc.atan2(x=1, y=1) → π/4 (paridade vanilla).
        approx_float(
            calc_atan2(
                &mut ctx,
                &p(vec![Value::Float(1.0), Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::FRAC_PI_4,
        );
        // calc.atan2(x=0, y=1) → π/2 (eixo +y).
        approx_float(
            calc_atan2(
                &mut ctx,
                &p(vec![Value::Float(0.0), Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            std::f64::consts::FRAC_PI_2,
        );
    }

    #[test]
    fn calc_atan2_arity_errada_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_atan2(
            &mut ctx,
            &p(vec![Value::Float(1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_atan2(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    }

    // ── Hiperbólicas ─────────────────────────────────────────────────────────

    #[test]
    fn calc_sinh_cosh_tanh_zero() {
        null_ctx!(ctx);
        approx_float(
            calc_sinh(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        approx_float(
            calc_cosh(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
        approx_float(
            calc_tanh(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
    }

    #[test]
    fn calc_sinh_cosh_identidade_pitagorica() {
        null_ctx!(ctx);
        // cosh²(x) - sinh²(x) = 1 (identidade hiperbólica fundamental).
        let s = match calc_sinh(
            &mut ctx,
            &p(vec![Value::Float(1.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap()
        {
            Value::Float(f) => f,
            _ => panic!(),
        };
        let c = match calc_cosh(
            &mut ctx,
            &p(vec![Value::Float(1.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap()
        {
            Value::Float(f) => f,
            _ => panic!(),
        };
        assert!((c * c - s * s - 1.0).abs() < 1e-10, "cosh²-sinh² = {}", c * c - s * s);
    }

    #[test]
    fn calc_asinh_inverso_de_sinh() {
        null_ctx!(ctx);
        approx_float(
            calc_asinh(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        // sinh(asinh(2)) = 2.
        let v = calc_asinh(
            &mut ctx,
            &p(vec![Value::Float(2.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let back =
            calc_sinh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
        approx_float(back, 2.0);
    }

    #[test]
    fn calc_acosh_dominio_valido() {
        null_ctx!(ctx);
        approx_float(
            calc_acosh(
                &mut ctx,
                &p(vec![Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        // cosh(acosh(2.5)) = 2.5.
        let v = calc_acosh(
            &mut ctx,
            &p(vec![Value::Float(2.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let back =
            calc_cosh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
        approx_float(back, 2.5);
    }

    #[test]
    fn calc_acosh_fora_dominio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_acosh(
            &mut ctx,
            &p(vec![Value::Float(0.5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_acosh(
            &mut ctx,
            &p(vec![Value::Float(-1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_atanh_dominio_valido() {
        null_ctx!(ctx);
        approx_float(
            calc_atanh(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
        // tanh(atanh(0.5)) = 0.5.
        let v = calc_atanh(
            &mut ctx,
            &p(vec![Value::Float(0.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let back =
            calc_tanh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
        approx_float(back, 0.5);
    }

    #[test]
    fn calc_atanh_fora_dominio_retorna_err() {
        null_ctx!(ctx);
        // Fronteiras (-1 e 1) são exclusivas — Err em ambos.
        assert!(calc_atanh(
            &mut ctx,
            &p(vec![Value::Float(1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_atanh(
            &mut ctx,
            &p(vec![Value::Float(-1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_atanh(
            &mut ctx,
            &p(vec![Value::Float(1.5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── Exponencial / logaritmos ─────────────────────────────────────────────

    #[test]
    fn calc_exp_zero_e_um() {
        null_ctx!(ctx);
        approx_float(
            calc_exp(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
        approx_float(
            calc_exp(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
                .unwrap(),
            std::f64::consts::E,
        );
    }

    #[test]
    fn calc_exp_overflow_retorna_err() {
        null_ctx!(ctx);
        // exp(1e10) → Inf → guard_float devolve Err.
        assert!(calc_exp(
            &mut ctx,
            &p(vec![Value::Float(1e10)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_ln_e_e_dominio() {
        null_ctx!(ctx);
        approx_float(
            calc_ln(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
                .unwrap(),
            0.0,
        );
        approx_float(
            calc_ln(
                &mut ctx,
                &p(vec![Value::Float(std::f64::consts::E)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
    }

    #[test]
    fn calc_ln_nao_positivo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_ln(
            &mut ctx,
            &p(vec![Value::Float(0.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_ln(
            &mut ctx,
            &p(vec![Value::Float(-1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_log_base_default_dez() {
        null_ctx!(ctx);
        approx_float(
            calc_log(
                &mut ctx,
                &p(vec![Value::Float(100.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            2.0,
        );
        approx_float(
            calc_log(
                &mut ctx,
                &p(vec![Value::Float(1000.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
    }

    #[test]
    fn calc_log_base_explicita() {
        null_ctx!(ctx);
        approx_float(
            calc_log(
                &mut ctx,
                &p(vec![Value::Float(8.0), Value::Float(2.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
        approx_float(
            calc_log(
                &mut ctx,
                &p(vec![Value::Float(27.0), Value::Float(3.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
    }

    #[test]
    fn calc_log_dominio_e_base_invalidos_retorna_err() {
        null_ctx!(ctx);
        // valor ≤ 0.
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(0.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(-1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        // base inválida (≤0, 1, Inf, NaN).
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(10.0), Value::Float(1.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(10.0), Value::Float(0.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(10.0), Value::Float(-2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_log(
            &mut ctx,
            &p(vec![Value::Float(10.0), Value::Float(f64::INFINITY)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── P495 — calc.log(base:) ───────────────────────────────────────────────

    #[test]
    fn p495_calc_log_base_named() {
        null_ctx!(ctx);
        approx_float(
            calc_log(
                &mut ctx,
                &pn(vec![Value::Float(100.0)], "base", Value::Int(10)),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            2.0,
        );
        approx_float(
            calc_log(
                &mut ctx,
                &pn(vec![Value::Float(8.0)], "base", Value::Int(2)),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
        // posicional preservado
        approx_float(
            calc_log(
                &mut ctx,
                &p(vec![Value::Float(27.0), Value::Float(3.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
    }

    #[test]
    fn p495_calc_log_base_e_posicional_sao_mutuamente_exclusivos() {
        null_ctx!(ctx);
        assert!(calc_log(
            &mut ctx,
            &Args { items: vec![Value::Float(100.0), Value::Float(10.0)], named: [("base".into(), Value::Int(10))].into_iter().collect() },
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn p495_calc_log_named_desconhecido_rejeitado() {
        null_ctx!(ctx);
        assert!(calc_log(&mut ctx, &pn(vec![Value::Float(100.0)], "foo", Value::Int(10)), &null_world(), test_file_id()).is_err());
    }

    // ── P306 — aritmética inteira, combinatória, norma, raiz ────────────────

    #[test]
    fn calc_trunc_int_e_float() {
        null_ctx!(ctx);
        assert_eq!(
            calc_trunc(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            calc_trunc(
                &mut ctx,
                &p(vec![Value::Float(3.7)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
        assert_eq!(
            calc_trunc(
                &mut ctx,
                &p(vec![Value::Float(-3.7)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(-3)
        );
        assert!(calc_trunc(
            &mut ctx,
            &p(vec![Value::Str("x".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_trunc(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn calc_fract_int_e_float() {
        null_ctx!(ctx);
        assert_eq!(
            calc_fract(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Float(0.0)
        );
        approx_float(
            calc_fract(
                &mut ctx,
                &p(vec![Value::Float(3.7)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.7,
        );
        approx_float(
            calc_fract(
                &mut ctx,
                &p(vec![Value::Float(-3.7)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            -0.7,
        );
        assert!(calc_fract(
            &mut ctx,
            &p(vec![Value::Bool(true)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_even_odd_int_apenas() {
        null_ctx!(ctx);
        assert_eq!(
            calc_even(&mut ctx, &p(vec![Value::Int(4)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            calc_even(&mut ctx, &p(vec![Value::Int(-3)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            calc_even(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(true)
        );
        assert!(calc_even(
            &mut ctx,
            &p(vec![Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());

        assert_eq!(
            calc_odd(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            calc_odd(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            calc_odd(&mut ctx, &p(vec![Value::Int(-3)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Bool(true)
        );
        assert!(calc_odd(
            &mut ctx,
            &p(vec![Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_rem_truncado() {
        null_ctx!(ctx);
        assert_eq!(
            calc_rem(
                &mut ctx,
                &p(vec![Value::Int(7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_rem(
                &mut ctx,
                &p(vec![Value::Int(-7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(-1)
        );
        approx_float(
            calc_rem(
                &mut ctx,
                &p(vec![Value::Float(7.5), Value::Float(2.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.5,
        );
        assert!(calc_rem(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_rem(
            &mut ctx,
            &p(vec![Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_rem_euclid_sempre_positivo() {
        null_ctx!(ctx);
        assert_eq!(
            calc_rem_euclid(
                &mut ctx,
                &p(vec![Value::Int(-7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(2)
        );
        assert_eq!(
            calc_rem_euclid(
                &mut ctx,
                &p(vec![Value::Int(7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        approx_float(
            calc_rem_euclid(
                &mut ctx,
                &p(vec![Value::Float(-7.5), Value::Float(2.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.5,
        );
        assert!(calc_rem_euclid(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_div_euclid_arredonda_para_menos_inf() {
        null_ctx!(ctx);
        assert_eq!(
            calc_div_euclid(
                &mut ctx,
                &p(vec![Value::Int(-7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(-3)
        );
        assert_eq!(
            calc_div_euclid(
                &mut ctx,
                &p(vec![Value::Int(7), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(2)
        );
        assert!(calc_div_euclid(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_quo_truncado() {
        null_ctx!(ctx);
        assert_eq!(
            calc_quo(
                &mut ctx,
                &p(vec![Value::Int(7), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
        assert_eq!(
            calc_quo(
                &mut ctx,
                &p(vec![Value::Int(-7), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(-3)
        );
        assert_eq!(
            calc_quo(
                &mut ctx,
                &p(vec![Value::Float(7.5), Value::Float(2.0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(3)
        );
        assert!(calc_quo(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_gcd_lcm_basico() {
        null_ctx!(ctx);
        assert_eq!(
            calc_gcd(
                &mut ctx,
                &p(vec![Value::Int(12), Value::Int(18)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(6)
        );
        assert_eq!(
            calc_gcd(
                &mut ctx,
                &p(vec![Value::Int(0), Value::Int(5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            calc_gcd(
                &mut ctx,
                &p(vec![Value::Int(0), Value::Int(0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
        assert_eq!(
            calc_gcd(
                &mut ctx,
                &p(vec![Value::Int(-12), Value::Int(18)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(6)
        );

        assert_eq!(
            calc_lcm(
                &mut ctx,
                &p(vec![Value::Int(4), Value::Int(6)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(12)
        );
        assert_eq!(
            calc_lcm(
                &mut ctx,
                &p(vec![Value::Int(0), Value::Int(5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
        assert!(calc_lcm(
            &mut ctx,
            &p(vec![Value::Int(i64::MAX), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_fact_basico_e_overflow() {
        null_ctx!(ctx);
        assert_eq!(
            calc_fact(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_fact(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_fact(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(120)
        );
        assert_eq!(
            calc_fact(&mut ctx, &p(vec![Value::Int(20)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Int(2_432_902_008_176_640_000)
        );
        assert!(calc_fact(
            &mut ctx,
            &p(vec![Value::Int(-1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_fact(
            &mut ctx,
            &p(vec![Value::Int(21)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_fact(
            &mut ctx,
            &p(vec![Value::Float(5.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_perm_arranjos() {
        null_ctx!(ctx);
        assert_eq!(
            calc_perm(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(20)
        );
        assert_eq!(
            calc_perm(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_perm(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(120)
        );
        assert_eq!(
            calc_perm(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(8)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
        assert!(calc_perm(
            &mut ctx,
            &p(vec![Value::Int(-1), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_binom_combinacoes() {
        null_ctx!(ctx);
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(2)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(10)
        );
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(5), Value::Int(8)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(0)
        );
        // C(20, 10) = 184756 — caso clássico, fica longe do limite i64.
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(20), Value::Int(10)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(184_756)
        );
        // Simetria: C(30, 28) deve usar k_eff = 2 internamente.
        assert_eq!(
            calc_binom(
                &mut ctx,
                &p(vec![Value::Int(30), Value::Int(28)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Int(435)
        );
        assert!(calc_binom(
            &mut ctx,
            &p(vec![Value::Int(-1), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        // Overflow real (n grande): C(67, 33) sai do alcance i64.
        assert!(calc_binom(
            &mut ctx,
            &p(vec![Value::Int(67), Value::Int(33)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_norm_p2_default_pitagoras() {
        null_ctx!(ctx);
        // Sem args: norma de vector vazio = 0.0.
        assert_eq!(
            calc_norm(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap(),
            Value::Float(0.0)
        );
        // Pitágoras: norma 2 de (3, 4) = 5.
        approx_float(
            calc_norm(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(4)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            5.0,
        );
        // abs interno: norma 2 de (-3, 4) = 5.
        approx_float(
            calc_norm(
                &mut ctx,
                &p(vec![Value::Float(-3.0), Value::Float(4.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            5.0,
        );
    }

    #[test]
    fn calc_norm_p_named() {
        null_ctx!(ctx);
        // Taxicab (p=1) de (1, 1, 1) = 3.
        approx_float(
            calc_norm(
                &mut ctx,
                &pn(
                    vec![Value::Int(1), Value::Int(1), Value::Int(1)],
                    "p",
                    Value::Float(1.0),
                ),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
        // Chebyshev-aproximação (p grande) de (3, 4) ≈ 4.
        let v = calc_norm(
            &mut ctx,
            &pn(vec![Value::Int(3), Value::Int(4)], "p", Value::Float(100.0)),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        match v {
            Value::Float(f) => assert!((f - 4.0).abs() < 1e-2, "esperado ≈4, obtido {f}"),
            _ => panic!(),
        }
    }

    #[test]
    fn calc_norm_rejeita_named_desconhecido() {
        null_ctx!(ctx);
        assert!(calc_norm(
            &mut ctx,
            &pn(vec![Value::Int(3)], "q", Value::Float(2.0)),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_root_raiz_quadrada_e_cubica() {
        null_ctx!(ctx);
        approx_float(
            calc_root(
                &mut ctx,
                &p(vec![Value::Int(2), Value::Int(9)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            3.0,
        );
        approx_float(
            calc_root(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(8)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            2.0,
        );
        approx_float(
            calc_root(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(-8)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            -2.0,
        );
        approx_float(
            calc_root(
                &mut ctx,
                &p(vec![Value::Int(2), Value::Float(0.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.0,
        );
    }

    #[test]
    fn calc_root_erros() {
        null_ctx!(ctx);
        assert!(calc_root(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(-1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_root(
            &mut ctx,
            &p(vec![Value::Int(0), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_root(
            &mut ctx,
            &p(vec![Value::Float(2.0), Value::Int(9)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(calc_root(
            &mut ctx,
            &p(vec![Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── Passo 308 — função erro de Gauss (calc.erf) ──────────────────────────
    //
    // Caminho A: Abramowitz & Stegun 7.1.26 — erro máximo absoluto 1,5e-7.
    // Tolerância dos testes usa esse limite + margem (`2e-7`) em vez do
    // `1e-10` do helper `approx_float` partilhado.

    /// Helper específico para `erf`: tolerância 2e-7 (envelope A&S 7.1.26).
    fn approx_float_erf(v: Value, expected: f64) {
        match v {
            Value::Float(f) => assert!(
                (f - expected).abs() < 2e-7,
                "esperado ≈{expected} (tol 2e-7), obtido {f}",
            ),
            other => panic!("esperado Value::Float, obtido {other:?}"),
        }
    }

    #[test]
    fn calc_erf_identidade_e_int_coerce() {
        null_ctx!(ctx);
        // erf(0) = 0 — identidade exacta (sign·0·exp(0) = 0).
        assert_eq!(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(0.0)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(0.0),
        );
        // Int coerce: erf(Int 0) idêntico a erf(Float 0.0).
        assert_eq!(
            calc_erf(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
                .unwrap(),
            Value::Float(0.0),
        );
        // Int 1 coerce: deve coincidir com Float 1.0 dentro da tolerância.
        let a =
            calc_erf(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
                .unwrap();
        let b = calc_erf(
            &mut ctx,
            &p(vec![Value::Float(1.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        match (a, b) {
            (Value::Float(fa), Value::Float(fb)) => assert!(
                (fa - fb).abs() < 1e-15,
                "Int/Float coerce deve produzir o mesmo bit-pattern: fa={fa}, fb={fb}",
            ),
            _ => panic!("ambos devem ser Value::Float"),
        }
    }

    #[test]
    fn calc_erf_valores_conhecidos() {
        null_ctx!(ctx);
        // Valores tabulares clássicos (Abramowitz & Stegun Tabela 7.1).
        approx_float_erf(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(0.5)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.520_499_877_813_046_5,
        );
        approx_float_erf(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(1.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.842_700_792_949_715_0,
        );
        approx_float_erf(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(2.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            0.995_322_265_018_952_7,
        );
    }

    #[test]
    fn calc_erf_simetria_impar() {
        null_ctx!(ctx);
        for x in [0.1_f64, 0.5, 1.0, 1.7, 3.3] {
            let pos = calc_erf(
                &mut ctx,
                &p(vec![Value::Float(x)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap();
            let neg = calc_erf(
                &mut ctx,
                &p(vec![Value::Float(-x)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap();
            match (pos, neg) {
                (Value::Float(p_), Value::Float(n_)) => assert!(
                    (p_ + n_).abs() < 2e-7,
                    "simetria ímpar violada em x={x}: erf(x)={p_}, erf(-x)={n_}",
                ),
                _ => panic!("erf de Float deve retornar Float"),
            }
        }
    }

    #[test]
    fn calc_erf_limites_finitos() {
        null_ctx!(ctx);
        // |x| grande mas finito: erf(±5) ≈ ±1 dentro da tolerância A&S.
        approx_float_erf(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(5.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            1.0,
        );
        approx_float_erf(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(-5.0)]),
                &null_world(),
                test_file_id(),
            )
            .unwrap(),
            -1.0,
        );
    }

    #[test]
    fn calc_erf_infinito_short_circuit() {
        null_ctx!(ctx);
        // Short-circuit no infinito: retorna ±1.0 exacto sem passar pela
        // fórmula (que produziria 0·∞ = NaN).
        assert_eq!(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(f64::INFINITY)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(1.0),
        );
        assert_eq!(
            calc_erf(
                &mut ctx,
                &p(vec![Value::Float(f64::NEG_INFINITY)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Float(-1.0),
        );
    }

    #[test]
    fn calc_erf_nan_err() {
        null_ctx!(ctx);
        // Divergência consciente vs vanilla `libm::erf(NaN) = NaN`:
        // cristalino mapeia para Err (paridade convenção `guard_float`).
        assert!(calc_erf(
            &mut ctx,
            &p(vec![Value::Float(f64::NAN)]),
            &null_world(),
            test_file_id()
        )
        .is_err(),);
    }

    #[test]
    fn calc_erf_arity_e_tipos_invalidos() {
        null_ctx!(ctx);
        // Arity 0 → Err.
        assert!(calc_erf(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
        // Arity 2 → Err.
        assert!(calc_erf(
            &mut ctx,
            &p(vec![Value::Float(1.0), Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        // Tipo inválido (Str) → Err.
        assert!(calc_erf(
            &mut ctx,
            &p(vec![Value::Str("x".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        // Tipo inválido (Bool) → Err.
        assert!(calc_erf(
            &mut ctx,
            &p(vec![Value::Bool(true)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn calc_erf_named_arg_rejeitado() {
        null_ctx!(ctx);
        // Paridade `expect_no_named` — nenhum named arg é aceite.
        let args = pn(vec![Value::Float(1.0)], "p", Value::Float(0.5));
        assert!(calc_erf(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn calc_erf_registado_no_modulo() {
        // Paridade `make_calc_module`: chave "erf" presente como Func.
        let module = p283_make_calc_module();
        let dict = match module {
            Value::Dict(d) => d,
            other => panic!("esperado Dict, obtido {other:?}"),
        };
        assert!(
            matches!(dict.get("erf"), Some(Value::Func(_))),
            "calc.erf não está registado como Func no módulo (P308)",
        );
        // P501: módulo tem 44 funções + 4 constantes = 48 entradas.
        assert_eq!(
            dict.len(),
            48,
            "esperava 44 funções + 4 constantes = 48, obtido {}",
            dict.len()
        );
    }

    #[test]
    fn calc_modulo_contem_funcoes_p306() {
        let module = p283_make_calc_module();
        let dict = match module {
            Value::Dict(d) => d,
            other => panic!("esperado Dict, obtido {other:?}"),
        };
        for key in [
            "trunc",
            "fract",
            "even",
            "odd",
            "rem",
            "rem-euclid",
            "div-euclid",
            "quo",
            "gcd",
            "lcm",
            "fact",
            "perm",
            "binom",
            "norm",
            "root",
        ] {
            assert!(
                matches!(dict.get(key), Some(Value::Func(_))),
                "calc.{key} não está registado como Func"
            );
        }
    }

    // ── Constantes + integração com make_calc_module ────────────────────────

    #[test]
    fn calc_constantes_pi_tau_e_inf() {
        let module = p283_make_calc_module();
        let dict = match module {
            Value::Dict(d) => d,
            other => panic!("esperado Dict, obtido {other:?}"),
        };
        assert_eq!(dict.get("pi").cloned(), Some(Value::Float(std::f64::consts::PI)));
        assert_eq!(dict.get("tau").cloned(), Some(Value::Float(std::f64::consts::TAU)));
        assert_eq!(dict.get("e").cloned(), Some(Value::Float(std::f64::consts::E)));
        assert_eq!(dict.get("inf").cloned(), Some(Value::Float(f64::INFINITY)));
    }

    #[test]
    fn calc_modulo_expoe_41_funcoes() {
        let module = p283_make_calc_module();
        let dict = match module {
            Value::Dict(d) => d,
            other => panic!("esperado Dict, obtido {other:?}"),
        };
        // 9 herdadas (abs/pow/sqrt/floor/ceil/round/min/max/clamp)
        // + 16 P283 (trig/hyperbolic/log/exp)
        // + 15 P306 (trunc/fract/even/odd/rem/rem-euclid/div-euclid/quo/
        //            gcd/lcm/fact/perm/binom/norm/root)
        // + 1  P308 (erf)
        // + 3  P501 (log10/deg/rad) = 44 funções.
        let n_funcs = dict.values().filter(|v| matches!(v, Value::Func(_))).count();
        assert_eq!(n_funcs, 44, "esperava 44 funções calc, encontrei {n_funcs}");
        // + 4 constantes (pi/tau/e/inf).
        let n_floats = dict.values().filter(|v| matches!(v, Value::Float(_))).count();
        assert_eq!(n_floats, 4, "esperava 4 constantes Float, encontrei {n_floats}");
    }

    #[test]
    fn native_range_directo() {
        null_ctx!(ctx);
        assert_eq!(
            native_range(
                &mut ctx,
                &p(vec![Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])
        );
        assert_eq!(
            native_range(
                &mut ctx,
                &p(vec![Value::Int(2), Value::Int(5)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)])
        );
        assert_eq!(
            native_range(
                &mut ctx,
                &p(vec![Value::Int(3), Value::Int(3)]),
                &null_world(),
                test_file_id()
            )
            .unwrap(),
            Value::Array(vec![])
        );
        assert!(native_range(
            &mut ctx,
            &p(vec![Value::Int(-1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── Passo 64 — native_figure (DEBT-16) ──────────────────────────────────

    #[test]
    fn native_figure_com_body_e_caption() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let body_content = Content::text("Gráfico");
        let caption_content = Content::text("Legenda");
        let args = pn(
            vec![Value::Content(body_content)],
            "caption",
            Value::Content(caption_content),
        );
        let result =
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_some()),
            "figure com caption deve ter Some(caption): {:?}",
            result
        );
    }

    #[test]
    fn native_figure_sem_caption() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let body_content = Content::text("Diagrama");
        let args = p(vec![Value::Content(body_content)]);
        let result =
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_none()),
            "figure sem caption deve ter None: {:?}",
            result
        );
    }

    #[test]
    fn native_figure_caption_none_value() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        // caption: none → ausência de legenda
        let body_content = Content::text("Corpo");
        let args = pn(vec![Value::Content(body_content)], "caption", Value::None);
        let result =
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(&result, Value::Content(Content::Figure(e)) if e.caption.is_none()),
            "figure com caption: none deve ter caption None"
        );
    }

    #[test]
    fn native_figure_sem_body_retorna_err() {
        null_ctx!(ctx);
        let args = p(vec![]);
        assert!(
            native_figure(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
            "figure sem body deve retornar Err"
        );
    }

    // ── Passo 158A — auto-detecção de kind em native_figure ─────────────

    #[test]
    fn figure_auto_detect_image() {
        // P158A: figure(image(...)) sem `kind:` → kind="image"
        // via inferência (não via default).
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::ptr_eq_arc::PtrEqArc;
        use std::sync::Arc;
        let img = Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
        let args = p(vec![Value::Content(img)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert_eq!(
                e.kind.as_deref(),
                Some("image"),
                "auto-detect Image → kind=Some(\"image\")"
            );
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_auto_detect_table() {
        // P158A: figure(table(...)) sem `kind:` → kind=Some("table").
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::layout_types::TrackSizing;
        let tab = Content::table(vec![TrackSizing::Auto], vec![], vec![]);
        let args = p(vec![Value::Content(tab)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert_eq!(
                e.kind.as_deref(),
                Some("table"),
                "auto-detect Table → kind=Some(\"table\")"
            );
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_auto_detect_raw() {
        // P158A: figure(raw(...)) sem `kind:` → kind=Some("raw").
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let raw = Content::raw("fn x() {}", None, false);
        let args = p(vec![Value::Content(raw)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert_eq!(
                e.kind.as_deref(),
                Some("raw"),
                "auto-detect Raw → kind=Some(\"raw\")"
            );
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_kind_explicit_override_auto_detect() {
        // P158A: `kind:` explícito vence auto-detecção
        // (precedência absoluta).
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::ptr_eq_arc::PtrEqArc;
        use std::sync::Arc;
        let img = Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
        let args =
            pn(vec![Value::Content(img)], "kind", Value::Str("custom-kind".into()));
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert_eq!(
                e.kind.as_deref(),
                Some("custom-kind"),
                "kind explícito vence auto-detecção (precedência absoluta)"
            );
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_kind_auto_explicito_devolve_none() {
        // P158C: `kind: auto` explícito produz None (paridade
        // ADR-0064 Caso A: vanilla Auto ↔ cristalino None).
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::ptr_eq_arc::PtrEqArc;
        use std::sync::Arc;
        let img = Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
        let args = pn(vec![Value::Content(img)], "kind", Value::Auto);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert!(e.kind.is_none(),
                "kind=auto explícito produz None (Caso A); auto-detect ignorado quando explícito");
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_kind_none_quando_body_nao_detectavel() {
        // P158C: body=Text sem `kind:` → kind=None directo (default
        // "image" resolvido em uso, não em construção; ADR-0064 Caso A
        // estrito refactor de String → Option<String>).
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let args = p(vec![Value::Content(Content::text("apenas texto"))]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert!(e.kind.is_none(),
                "body Text sem auto-detect produz kind=None (default 'image' resolvido em uso)");
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_auto_detect_image_dentro_de_sequence() {
        // P158A §8: recursão limitada a Sequence — figure([
        // ..., image(...), ...]) detecta Image via primeiro
        // child detectável.
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::ptr_eq_arc::PtrEqArc;
        use std::sync::Arc;
        let img = Content::image("a.png", PtrEqArc(Arc::new(Vec::new())), None, None, "cover");
        // Sequence começa com Text (não detectável) e contém Image.
        let seq = Content::Sequence(Arc::from(vec![Content::text("legenda"), img]));
        let args = p(vec![Value::Content(seq)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Figure(e)) = r {
            assert_eq!(
                e.kind.as_deref(),
                Some("image"),
                "Sequence com Image dentro auto-detecta Some(\"image\") via recursão"
            );
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn expect_no_named_retorna_err_com_named_arg() {
        let mut a = p(vec![]);
        a.named.insert("foo".into(), Value::Int(1));
        let result: SourceResult<()> = expect_no_named(&a.named);
        assert!(result.is_err());
        let err_msg = &result.unwrap_err()[0].message;
        assert!(err_msg.contains("inesperado"), "mensagem: {:?}", err_msg);
    }

    #[test]
    fn expect_no_named_ok_com_vazio() {
        let a = p(vec![]);
        assert!(expect_no_named(&a.named).is_ok());
    }

    // ── Passo 66 — native_assert (prova de fogo de named args) ───────────────

    #[test]
    fn native_assert_true_nao_gera_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Bool(true)]);
        assert!(
            native_assert(&mut ctx, &args, &null_world(), test_file_id()).is_ok(),
            "assert(true) deve ter sucesso"
        );
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_padrao() {
        null_ctx!(ctx);
        let args = p(vec![Value::Bool(false)]);
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("falhou") || err[0].message.contains("Asser"),
            "mensagem de erro padrão deve mencionar a asserção: {:?}",
            err[0].message
        );
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_personalizada() {
        null_ctx!(ctx);
        // Mensagem sem acentos para evitar problemas de codificação em CI.
        let args = pn(
            vec![Value::Bool(false)],
            "message",
            Value::Str("Matematica falhou".into()),
        );
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err());
        assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
    }

    #[test]
    fn native_assert_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Bool(true)], "bla", Value::Str("bla".into()));
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("inesperado") && err[0].message.contains("bla"),
            "named arg desconhecido deve gerar erro: {:?}",
            err[0].message
        );
    }

    // ── Passo 392 — native_panic ──────────────────────────────────────────────

    #[test]
    fn native_panic_aborta_com_mensagem() {
        null_ctx!(ctx);
        let result = native_panic(
            &mut ctx,
            &p(vec![Value::Str("fail".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err[0].message, "fail");
    }

    #[test]
    fn native_panic_aceita_mensagem_vazia() {
        null_ctx!(ctx);
        let result = native_panic(
            &mut ctx,
            &p(vec![Value::Str("".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err[0].message, "");
    }

    #[test]
    fn native_panic_rejeita_tipo_errado() {
        null_ctx!(ctx);
        let result = native_panic(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn native_panic_rejeita_named_arg() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Str("x".into())], "foo", Value::Int(1));
        assert!(native_panic(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    // ── Passo 71 — native_image ───────────────────────────────────────────────

    #[test]
    fn native_image_retorna_content_image() {
        let mut world = NullWorld::default();
        world
            .files
            .insert("foto.png".to_string(), std::sync::Arc::new(vec![1, 2, 3]));
        let _dummy_id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let mut ctx = EvalContext::new();
        let args = p(vec![Value::Str("foto.png".into())]);
        let result = native_image(&mut ctx, &args, &world, test_file_id()).unwrap();
        assert!(matches!(result, Value::Content(Content::Image(_))));
    }

    #[test]
    fn native_image_ficheiro_inexistente_gera_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("naoexiste.png".into())]);
        assert!(native_image(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn native_image_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args =
            pn(vec![Value::Str("foto.png".into())], "cor", Value::Str("red".into()));
        assert!(native_image(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    // ── Passo 76 — primitivas geométricas ────────────────────────────────────

    #[test]
    fn rect_sem_cores_tem_stroke_preta_1pt() {
        // #rect() sem fill nem stroke → stroke preta de 1pt.
        // Confirma que a stdlib é o único local onde este fallback existe.
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        let result =
            native_rect(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(e.fill.is_none(), "rect sem fill deve ter fill: None");
            let s = e.stroke.clone().expect("rect sem cores deve ter stroke de fallback");
            assert_eq!(
                s.paint,
                Paint::Solid(Color::rgb(0, 0, 0)),
                "stroke de fallback deve ser preta"
            );
            assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn rect_com_fill_nao_tem_stroke_fallback() {
        null_ctx!(ctx);
        let mut args = Args::positional(vec![]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let result = native_rect(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(e.fill.is_some(), "fill red deve estar presente");
            assert!(
                e.stroke.is_none(),
                "sem stroke explícito e com fill → stroke deve ser None"
            );
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn square_posicional_1cm_produz_rect_lados_iguais() {
        use crate::entities::geometry::ShapeKind;
        null_ctx!(ctx);
        let w = Value::Length(Length::pt(28.346));
        let result =
            native_square(&mut ctx, &p(vec![w.clone()]), &null_world(), test_file_id())
                .unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(matches!(e.kind, ShapeKind::Rect));
            assert_eq!(e.width.as_deref(), Some(&w));
            assert_eq!(e.height.as_deref(), Some(&w));
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn square_com_height_diferente_aceita_fallback() {
        use crate::entities::geometry::ShapeKind;
        null_ctx!(ctx);
        let w = Value::Length(Length::pt(28.346));
        let h = Value::Length(Length::pt(56.692));
        let mut args = Args::positional(vec![w.clone()]);
        args.named.insert("height".into(), h.clone());
        let result =
            native_square(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(matches!(e.kind, ShapeKind::Rect));
            assert_eq!(e.width.as_deref(), Some(&w));
            assert_eq!(e.height.as_deref(), Some(&h));
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn square_sem_width_gera_erro() {
        null_ctx!(ctx);
        let result = native_square(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(result.is_err(), "square() sem width deve retornar Err");
    }

    #[test]
    fn square_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Length(Length::pt(10.0))], "foo", Value::Int(1));
        assert!(native_square(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn square_sem_cores_tem_stroke_preta_1pt() {
        use crate::entities::geometry::ShapeKind;
        use crate::entities::paint::Paint;
        null_ctx!(ctx);
        let result = native_square(
            &mut ctx,
            &p(vec![Value::Length(Length::pt(10.0))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(matches!(e.kind, ShapeKind::Rect));
            assert!(e.fill.is_none(), "square sem fill deve ter fill: None");
            let s = e
                .stroke
                .clone()
                .expect("square sem cores deve ter stroke de fallback");
            assert_eq!(
                s.paint,
                Paint::Solid(Color::rgb(0, 0, 0)),
                "stroke de fallback deve ser preta"
            );
            assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn line_tem_kind_line_e_stroke_preta_por_omissao() {
        use crate::entities::geometry::ShapeKind;
        null_ctx!(ctx);
        let mut args = Args::positional(vec![]);
        args.named.insert("dx".into(), Value::Float(100.0));
        args.named.insert("dy".into(), Value::Float(50.0));
        let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(
                matches!(e.kind, ShapeKind::Line { dx, dy } if dx == 100.0 && dy == 50.0)
            );
            assert!(e.fill.is_none(), "linha não tem fill");
            assert!(e.stroke.is_some(), "linha tem stroke por omissão");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn polygon_sem_pontos_gera_erro() {
        null_ctx!(ctx);
        let args = Args::positional(vec![]);
        let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err(), "polygon() sem pontos deve retornar Err");
        let msg = result.unwrap_err();
        let msg_str = format!("{:?}", msg);
        assert!(
            msg_str.contains("pelo menos um ponto"),
            "Mensagem de erro deve mencionar 'pelo menos um ponto', obteve: {}",
            msg_str
        );
    }

    #[test]
    fn polygon_com_um_ponto_gera_moveto_e_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Float(10.0),
            Value::Float(20.0),
        ])]);
        let result =
            native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 2, "Um ponto deve gerar MoveTo + ClosePath");
            assert!(
                matches!(items[0], PathItem::MoveTo(_)),
                "Primeiro item deve ser MoveTo"
            );
            assert!(
                matches!(items[1], PathItem::ClosePath),
                "Último item deve ser ClosePath"
            );
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn polygon_triangulo_gera_moveto_lineto_lineto_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(25.0), Value::Float(50.0)]),
        ]);
        let result =
            native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 4); // MoveTo + 2×LineTo + ClosePath
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::LineTo(_)));
            assert!(matches!(items[2], PathItem::LineTo(_)));
            assert!(matches!(items[3], PathItem::ClosePath));
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    // ── Passo 293 — `curve(...)` activação posterior PathItem::CubicTo ──

    #[test]
    fn p293_curve_move_line_basico() {
        // curve com move + line + close — equivalente a polygon mas via
        // sintaxe descritiva ("move"/"line"/"close").
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("line".into()),
                Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![Value::Str("close".into())]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 3);
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::LineTo(_)));
            assert!(matches!(items[2], PathItem::ClosePath));
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn p293_curve_cubic_activa_pathitem_cubicto() {
        // **CRÍTICO** — activação inaugural de `PathItem::CubicTo` via
        // stdlib (H6 descoberta empírica A.0.0). Pré-P293, nenhuma
        // stdlib construía CubicTo (apenas P277 `path_bbox` consume +
        // emit já existia em export.rs).
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("cubic".into()),
                Value::Array(vec![Value::Float(25.0), Value::Float(0.0)]), // c1
                Value::Array(vec![Value::Float(75.0), Value::Float(50.0)]), // c2
                Value::Array(vec![Value::Float(100.0), Value::Float(50.0)]), // end
            ]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 2);
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            // **Activação pos-P293 — CubicTo construído via stdlib**
            assert!(
                matches!(items[1], PathItem::CubicTo(_, _, _)),
                "P293 H6: CubicTo deve ser construível via curve('cubic', c1, c2, end)"
            );
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn p293_curve_cubic_preserva_control_points() {
        // Verificar literalmente que c1/c2/end são preservados na ordem
        // certa no PathItem::CubicTo construído.
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Str("cubic".into()),
            Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
            Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(60.0)]),
        ])]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 1);
            if let PathItem::CubicTo(c1, c2, end) = items[0] {
                assert_eq!((c1.x.val(), c1.y.val()), (10.0, 20.0));
                assert_eq!((c2.x.val(), c2.y.val()), (30.0, 40.0));
                assert_eq!((end.x.val(), end.y.val()), (50.0, 60.0));
            } else {
                panic!("Esperado PathItem::CubicTo");
            }
        }
    }

    // ── Passo 294 — `curve(... "quadratic" ...)` activação via conversão q→c ──
    //
    // P294 H1' (refutação significativa A.0.0 N=2 da spec): vanilla
    // typst converte quadratic→cubic em construct-time via
    // `control_q2c(p, c) = (p + 2c) / 3`. Cristalino adopta o mesmo
    // padrão — sem variant novo `QuadraticTo`; hash export.rs
    // preservado pelo 11º passo consecutivo.

    #[test]
    fn p294_curve_quadratic_activa_via_conversao_q2c() {
        // Substitui o anterior p293_curve_quadratic_scope_out — o scope-out
        // foi removido em P294 H1' por descoberta empírica vanilla.
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("quadratic".into()),
                Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]), // control
                Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]), // end
            ]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        // Esperado: 2 PathItems — MoveTo + CubicTo (conversão q→c).
        if let Value::Content(Content::Shape(e)) = &result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 2, "esperava 2 items (MoveTo + CubicTo)");
            assert!(matches!(items[0], crate::entities::geometry::PathItem::MoveTo(_)));
            assert!(
                matches!(items[1], crate::entities::geometry::PathItem::CubicTo(_, _, _)),
                "quadratic deve materializar como CubicTo via conversão q→c"
            );
        } else {
            panic!("esperava Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn p294_curve_quadratic_aplica_formula_q2c_exacta() {
        // Verifica fórmula matemática exacta:
        // P0 = (0, 0), Q = (10, 20), P2 = (30, 40)
        // C1 = (P0 + 2Q) / 3 = (20/3, 40/3)
        // C2 = (P2 + 2Q) / 3 = (50/3, 80/3)
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("quadratic".into()),
                Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
                Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]),
            ]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = &result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            if let PathItem::CubicTo(c1, c2, end) = items[1] {
                let eps = 1e-9;
                assert!((c1.x.0 - 20.0 / 3.0).abs() < eps, "C1.x: {}", c1.x.0);
                assert!((c1.y.0 - 40.0 / 3.0).abs() < eps, "C1.y: {}", c1.y.0);
                assert!((c2.x.0 - 50.0 / 3.0).abs() < eps, "C2.x: {}", c2.x.0);
                assert!((c2.y.0 - 80.0 / 3.0).abs() < eps, "C2.y: {}", c2.y.0);
                assert!((end.x.0 - 30.0).abs() < eps);
                assert!((end.y.0 - 40.0).abs() < eps);
            } else {
                panic!("esperava CubicTo");
            }
        }
    }

    #[test]
    fn p294_curve_quadratic_sem_move_anterior_usa_origem() {
        // Caso fronteira: quadratic antes de qualquer move.
        // last_point fallback (0,0); curva é válida.
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(6.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(12.0), Value::Float(0.0)]),
        ])]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = &result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 1);
            if let PathItem::CubicTo(c1, _, _) = items[0] {
                // P0 = (0,0), Q = (6,0) → C1 = (0+12)/3 = 4
                let eps = 1e-9;
                assert!((c1.x.0 - 4.0).abs() < eps);
                assert!(c1.y.0.abs() < eps);
            }
        }
    }

    #[test]
    fn p294_curve_quadratic_aridade_errada_retorna_err() {
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(1.0), Value::Float(2.0)]),
            // falta end
        ])]);
        let err =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
        assert!(format!("{:?}", err).contains("quadratic"));
    }

    #[test]
    fn p294_curve_quadratic_encadeada_actualiza_last_point() {
        // Duas quadratics consecutivas: a segunda usa o end da primeira
        // como P0 (last_point).
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("quadratic".into()),
                Value::Array(vec![Value::Float(3.0), Value::Float(0.0)]),
                Value::Array(vec![Value::Float(6.0), Value::Float(0.0)]), // end = (6,0)
            ]),
            Value::Array(vec![
                Value::Str("quadratic".into()),
                Value::Array(vec![Value::Float(9.0), Value::Float(0.0)]),
                Value::Array(vec![Value::Float(12.0), Value::Float(0.0)]),
            ]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = &result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 3);
            // Segunda quadratic: P0=(6,0), Q=(9,0) → C1 = (6 + 18)/3 = 8
            if let PathItem::CubicTo(c1, _, _) = items[2] {
                let eps = 1e-9;
                assert!(
                    (c1.x.0 - 8.0).abs() < eps,
                    "last_point tracking falhou: c1.x={} (esperava 8.0)",
                    c1.x.0
                );
            }
        }
    }

    #[test]
    fn p294_curve_quadratic_e_cubic_misturados() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("quadratic".into()),
                Value::Array(vec![Value::Float(5.0), Value::Float(10.0)]),
                Value::Array(vec![Value::Float(10.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("cubic".into()),
                Value::Array(vec![Value::Float(15.0), Value::Float(5.0)]),
                Value::Array(vec![Value::Float(20.0), Value::Float(-5.0)]),
                Value::Array(vec![Value::Float(25.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![Value::Str("close".into())]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = &result {
            let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
                panic!("esperado ShapeKind::Path");
            };
            assert_eq!(items.len(), 4);
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::CubicTo(_, _, _)));
            assert!(matches!(items[2], PathItem::CubicTo(_, _, _)));
            assert!(matches!(items[3], PathItem::ClosePath));
        }
    }

    #[test]
    fn p293_curve_kind_desconhecido_retorna_err() {
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![Value::Str("bogus".into())])]);
        let err =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
        assert!(format!("{:?}", err).contains("desconhecido"));
    }

    #[test]
    fn p293_curve_vazia_retorna_err() {
        null_ctx!(ctx);
        let args = Args::positional(vec![]);
        let err =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
        assert!(format!("{:?}", err).contains("pelo menos um segmento"));
    }

    #[test]
    fn p293_curve_bbox_analitica_via_path_bbox_p277() {
        // Verificar que bbox cubic é calculada via P277 analítica
        // (não via min/max dos control points).
        use crate::entities::geometry::ShapeKind;
        let _ = ShapeKind::Rect; // sanity
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("cubic".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(100.0)]), // c1 alto
                Value::Array(vec![Value::Float(100.0), Value::Float(100.0)]), // c2 alto
                Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]), // end
            ]),
        ]);
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            // e.height da curva deve ser < 100 (extremo analítico, não 100 control point)
            // P277 calcula extremo da Bézier que é menor que max control point.
            let h = match e.height.as_deref() {
                Some(Value::Float(f)) => *f,
                _ => 0.0,
            };
            let w = match e.width.as_deref() {
                Some(Value::Float(f)) => *f,
                _ => 0.0,
            };
            assert!(w > 0.0 && h > 0.0, "bbox deve ser positiva; got w={w} h={h}");
            assert!(h < 100.0,
                "P277 bbox analítica: extremo da Bézier < max control point (100); got h={h}");
        }
    }

    #[test]
    fn p293_curve_named_fill_e_stroke() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let mut args = Args::positional(vec![
            Value::Array(vec![
                Value::Str("move".into()),
                Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            ]),
            Value::Array(vec![
                Value::Str("line".into()),
                Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
            ]),
        ]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
        let result =
            native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            assert!(e.fill.is_some(), "fill deve ser parseado");
            assert!(e.stroke.is_some(), "stroke deve ser parseado");
        }
    }

    // ── Passo 513 — `curve.move/line/cubic/quad/close` ────────────────────

    #[test]
    fn p513_curve_move_devolve_content_curve() {
        use crate::entities::elements::curve::CurveSegment;
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Float(10.0),
            Value::Length(Length::pt(20.0)),
        ])]);
        let result = native_curve_move(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Curve(e)) = result {
            assert_eq!(e.segments.len(), 1);
            assert!(
                matches!(&e.segments[0], CurveSegment::Move(p) if p.x == Length::pt(10.0) && p.y == Length::pt(20.0))
            );
        } else {
            panic!("esperado Content::Curve");
        }
    }

    #[test]
    fn p513_curve_line_devolve_content_curve() {
        use crate::entities::elements::curve::CurveSegment;
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let args = Args::positional(vec![Value::Array(vec![
            Value::Int(5),
            Value::Int(15),
        ])]);
        let result = native_curve_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Curve(e)) = result {
            assert!(matches!(&e.segments[0], CurveSegment::Line(p) if p.x == Length::pt(5.0) && p.y == Length::pt(15.0)));
        } else {
            panic!("esperado Content::Curve");
        }
    }

    #[test]
    fn p513_curve_cubic_devolve_content_curve() {
        use crate::entities::elements::curve::CurveSegment;
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
            Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]),
        ]);
        let result = native_curve_cubic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Curve(e)) = result {
            assert!(matches!(&e.segments[0], CurveSegment::Cubic(c1, c2, end)
                if c1.x == Length::pt(0.0) && c1.y == Length::pt(0.0)
                && c2.x == Length::pt(50.0) && c2.y == Length::pt(50.0)
                && end.x == Length::pt(100.0) && end.y == Length::pt(0.0)));
        } else {
            panic!("esperado Content::Curve");
        }
    }

    #[test]
    fn p513_curve_quad_devolve_content_curve() {
        use crate::entities::elements::curve::CurveSegment;
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(25.0), Value::Float(25.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(0.0)]),
        ]);
        let result = native_curve_quad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Curve(e)) = result {
            assert!(matches!(&e.segments[0], CurveSegment::Quad(c, end)
                if c.x == Length::pt(25.0) && c.y == Length::pt(25.0)
                && end.x == Length::pt(50.0) && end.y == Length::pt(0.0)));
        } else {
            panic!("esperado Content::Curve");
        }
    }

    #[test]
    fn p513_curve_close_devolve_content_curve() {
        use crate::entities::elements::curve::CurveSegment;
        null_ctx!(ctx);
        let args = Args::positional(vec![]);
        let result = native_curve_close(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Curve(e)) = result {
            assert!(matches!(e.segments[0], CurveSegment::Close));
        } else {
            panic!("esperado Content::Curve");
        }
    }

    #[test]
    fn p513_curve_aceita_content_curve_como_argumento() {
        // `curve(curve.move(...), curve.line(...), curve.close())` deve
        // concatenar os segmentos num `Content::Shape` final.
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let seg1 = native_curve_move(
            &mut ctx,
            &Args::positional(vec![Value::Array(vec![Value::Float(0.0), Value::Float(0.0)])]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let seg2 = native_curve_line(
            &mut ctx,
            &Args::positional(vec![Value::Array(vec![Value::Float(100.0), Value::Float(0.0)])]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let seg3 = native_curve_close(
            &mut ctx,
            &Args::positional(vec![]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let args = Args::positional(vec![seg1, seg2, seg3]);
        let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Shape(e)) = result {
            let ShapeKind::Path(items) = &e.kind else { panic!("esperado Path") };
            assert_eq!(items.len(), 3);
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::LineTo(_)));
            assert!(matches!(items[2], PathItem::ClosePath));
        } else {
            panic!("esperado Content::Shape");
        }
    }

    #[test]
    fn parse_color_nomes_conhecidos() {
        assert_eq!(parse_color(&Value::Str("red".into())), Some(Color::rgb(255, 0, 0)));
        assert_eq!(parse_color(&Value::Str("green".into())), Some(Color::rgb(0, 128, 0)));
        assert_eq!(parse_color(&Value::Str("blue".into())), Some(Color::rgb(0, 0, 255)));
        assert_eq!(parse_color(&Value::Str("black".into())), Some(Color::rgb(0, 0, 0)));
        assert_eq!(
            parse_color(&Value::Str("white".into())),
            Some(Color::rgb(255, 255, 255))
        );
        assert_eq!(parse_color(&Value::Str("purple".into())), Some(Color::rgb(128, 0, 128))); // P477
        assert_eq!(parse_color(&Value::Int(42)), None);
    }

    // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──

    #[test]
    fn native_pad_defaults_sides_none() {
        // P156L: sem named args → todos os sides são None (per
        // ADR-0064 Caso C; None ↔ default vanilla zero resolvido em uso).
        null_ctx!(ctx);
        let body = Content::text("body");
        let args = p(vec![Value::Content(body)]);
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = result {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.sides.left, None);
            assert_eq!(e.sides.right, None);
            assert_eq!(e.sides.top, None);
            assert_eq!(e.sides.bottom, None);
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_lados_individuais() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("left".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("right".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("top".into(), Value::Length(Length::pt(3.0)));
        args.named.insert("bottom".into(), Value::Length(Length::pt(4.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = result {
            assert_eq!(e.sides.left, Some(Length::pt(1.0)));
            assert_eq!(e.sides.right, Some(Length::pt(2.0)));
            assert_eq!(e.sides.top, Some(Length::pt(3.0)));
            assert_eq!(e.sides.bottom, Some(Length::pt(4.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_atalhos_x_e_y() {
        // x cobre left+right; y cobre top+bottom.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("x".into(), Value::Length(Length::pt(5.0)));
        args.named.insert("y".into(), Value::Length(Length::pt(7.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = result {
            assert_eq!(e.sides.left, Some(Length::pt(5.0)));
            assert_eq!(e.sides.right, Some(Length::pt(5.0)));
            assert_eq!(e.sides.top, Some(Length::pt(7.0)));
            assert_eq!(e.sides.bottom, Some(Length::pt(7.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_atalho_rest() {
        // rest cobre todos os 4 lados.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(8.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = result {
            assert_eq!(e.sides.left, Some(Length::pt(8.0)));
            assert_eq!(e.sides.right, Some(Length::pt(8.0)));
            assert_eq!(e.sides.top, Some(Length::pt(8.0)));
            assert_eq!(e.sides.bottom, Some(Length::pt(8.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_precedencia_especifico_eixo_rest() {
        // left explícito sobrepõe-se a x; x sobrepõe-se a rest.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("left".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("x".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("rest".into(), Value::Length(Length::pt(3.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = result {
            // left vence (específico)
            assert_eq!(e.sides.left, Some(Length::pt(1.0)));
            // right cai para x (eixo)
            assert_eq!(e.sides.right, Some(Length::pt(2.0)));
            // top cai para rest (não há y nem específico)
            assert_eq!(e.sides.top, Some(Length::pt(3.0)));
            assert_eq!(e.sides.bottom, Some(Length::pt(3.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_rejeita_padding_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("left".into(), Value::Length(Length::pt(-1.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err(), "padding negativo deve retornar Err em P156C/L");
    }

    #[test]
    fn native_pad_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("strange".into(), Value::Length(Length::pt(1.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err(), "named arg desconhecido em pad() deve retornar Err");
    }

    #[test]
    fn native_pad_aceita_int_e_float_como_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Float(2.5));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = r {
            assert_eq!(e.sides.left, Some(Length::pt(2.5)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    // ── P156L: tests novos para refino sides individualizadas ────────────

    #[test]
    fn native_pad_p156l_apenas_um_lado_outros_none() {
        // P156L: declarar só `top` deixa os outros 3 sides como None
        // (distinção semântica vs P156C onde ficavam Length::ZERO).
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("top".into(), Value::Length(Length::pt(7.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = r {
            assert_eq!(e.sides.top, Some(Length::pt(7.0)));
            assert_eq!(e.sides.left, None);
            assert_eq!(e.sides.right, None);
            assert_eq!(e.sides.bottom, None);
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_p156l_some_zero_distinct_from_none() {
        // P156L: declarar `left: 0pt` produz Some(zero), distinto de
        // não declarar (None). Per ADR-0064 Caso C distinção semântica.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("left".into(), Value::Length(Length::ZERO));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = r {
            assert_eq!(
                e.sides.left,
                Some(Length::ZERO),
                "left explicitamente declarado a zero é Some(ZERO), não None"
            );
            assert_eq!(e.sides.right, None, "right não declarado é None, não Some(ZERO)");
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_p156l_x_axis_apenas() {
        // Atalho `x` declara left+right; top/bottom ficam None.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("x".into(), Value::Length(Length::pt(4.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = r {
            assert_eq!(e.sides.left, Some(Length::pt(4.0)));
            assert_eq!(e.sides.right, Some(Length::pt(4.0)));
            assert_eq!(e.sides.top, None);
            assert_eq!(e.sides.bottom, None);
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_p156l_top_overrides_y_overrides_rest() {
        // Cadeia de precedência: top > y > rest. Decorações:
        // top=10, y=20, rest=30 → top=10 (especifico), bottom=20 (eixo).
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("top".into(), Value::Length(Length::pt(10.0)));
        args.named.insert("y".into(), Value::Length(Length::pt(20.0)));
        args.named.insert("rest".into(), Value::Length(Length::pt(30.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pad(e)) = r {
            assert_eq!(
                e.sides.top,
                Some(Length::pt(10.0)),
                "top específico vence y e rest"
            );
            assert_eq!(
                e.sides.bottom,
                Some(Length::pt(20.0)),
                "bottom cai para y (sem específico)"
            );
            assert_eq!(
                e.sides.left,
                Some(Length::pt(30.0)),
                "left cai para rest (sem específico nem x)"
            );
            assert_eq!(e.sides.right, Some(Length::pt(30.0)));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_sem_body_retorna_err() {
        null_ctx!(ctx);
        let result = native_pad(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(result.is_err(), "pad() sem body deve retornar Err");
    }

    #[test]
    fn native_hide_envolve_body() {
        null_ctx!(ctx);
        let body = Content::text("invisivel");
        let args = p(vec![Value::Content(body)]);
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Hide(e)) = result {
            assert_eq!(e.body.plain_text(), "invisivel");
        } else {
            panic!("esperado Content::Hide");
        }
    }

    #[test]
    fn native_hide_aceita_string() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("placeholder".into())]);
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(result, Value::Content(Content::Hide(_))));
    }

    #[test]
    fn native_hide_rejeita_named_arg() {
        null_ctx!(ctx);
        let args =
            pn(vec![Value::Content(Content::text("x"))], "weak", Value::Bool(true));
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id());
        assert!(result.is_err(), "hide() não aceita named args (P156C)");
    }

    #[test]
    fn native_hide_sem_body_retorna_err() {
        null_ctx!(ctx);
        let result = native_hide(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(result.is_err(), "hide() sem body deve retornar Err");
    }

    // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ──────────

    #[test]
    fn native_h_aceita_length() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let args = p(vec![Value::Length(Length::pt(12.0))]);
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::HSpace(e)) = r {
            assert_eq!(e.amount, Length::pt(12.0));
            assert!(!e.weak); // default
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_aceita_int_e_float_como_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Int interpretado em pt.
        let r =
            native_h(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
                .unwrap();
        if let Value::Content(Content::HSpace(e)) = r {
            assert_eq!(e.amount, Length::pt(5.0));
        } else {
            panic!("esperado Content::HSpace");
        }
        // Float interpretado em pt.
        let r = native_h(
            &mut ctx,
            &p(vec![Value::Float(2.5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::HSpace(e)) = r {
            assert_eq!(e.amount, Length::pt(2.5));
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_aceita_weak_true() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Length(Length::pt(3.0))]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::HSpace(e)) = r {
            assert!(e.weak);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_aceita_amount_zero() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_h(
            &mut ctx,
            &p(vec![Value::Length(Length::ZERO)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::HSpace(e)) = r {
            assert_eq!(e.amount, Length::ZERO);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_rejeita_amount_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_h(
            &mut ctx,
            &p(vec![Value::Length(Length::pt(-1.0))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
    }

    #[test]
    fn native_h_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Length(Length::pt(1.0))]);
        args.named.insert("attached".into(), Value::Bool(true));
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em h() deve retornar Err");
    }

    #[test]
    fn native_h_rejeita_weak_nao_bool() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Length(Length::pt(1.0))]);
        args.named.insert("weak".into(), Value::Int(1)); // tipo errado
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "weak não-bool deve retornar Err");
    }

    #[test]
    fn native_h_sem_amount_retorna_err() {
        null_ctx!(ctx);
        let r = native_h(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "h() sem amount deve retornar Err");
    }

    #[test]
    fn native_v_aceita_length_e_weak() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Caso composto: amount + weak; cobre paths principais de native_v
        // (que partilha lógica com native_h via build_spacing).
        let mut args = p(vec![Value::Length(Length::pt(15.0))]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_v(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::VSpace(e)) = r {
            assert_eq!(e.amount, Length::pt(15.0));
            assert!(e.weak);
        } else {
            panic!("esperado Content::VSpace");
        }
    }

    #[test]
    fn native_v_rejeita_amount_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_v(
            &mut ctx,
            &p(vec![Value::Length(Length::pt(-2.0))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
    }

    #[test]
    fn native_v_sem_amount_retorna_err() {
        null_ctx!(ctx);
        let r = native_v(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "v() sem amount deve retornar Err");
    }

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak manual ───────

    #[test]
    fn native_pagebreak_defaults() {
        null_ctx!(ctx);
        let r = native_pagebreak(&mut ctx, &p(vec![]), &null_world(), test_file_id())
            .unwrap();
        if let Value::Content(Content::Pagebreak(e)) = r {
            assert!(!e.weak);
            assert_eq!(e.to, None);
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_com_weak_true() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pagebreak(e)) = r {
            assert!(e.weak);
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_com_to_even() {
        null_ctx!(ctx);
        use crate::entities::parity::Parity;
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("even".into()));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pagebreak(e)) = r {
            assert_eq!(e.to, Some(Parity::Even));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_com_to_odd() {
        null_ctx!(ctx);
        use crate::entities::parity::Parity;
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("odd".into()));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pagebreak(e)) = r {
            assert_eq!(e.to, Some(Parity::Odd));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_combina_weak_e_to() {
        null_ctx!(ctx);
        use crate::entities::parity::Parity;
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Bool(true));
        args.named.insert("to".into(), Value::Str("even".into()));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Pagebreak(e)) = r {
            assert!(e.weak);
            assert_eq!(e.to, Some(Parity::Even));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_rejeita_to_invalido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("middle".into()));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "pagebreak(to: \"middle\") deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_to_nao_string() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Int(2));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "pagebreak(to:) com tipo errado deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("strange".into(), Value::Bool(false));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em pagebreak() deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_argumento_posicional() {
        null_ctx!(ctx);
        let r = native_pagebreak(
            &mut ctx,
            &p(vec![Value::Bool(true)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "pagebreak() não aceita argumentos posicionais");
    }

    #[test]
    fn native_pagebreak_rejeita_weak_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Int(1));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "weak não-Bool deve retornar Err");
    }

    // ── Passo 156F (ADR-0061 Fase 1, sub-passo 4) — skew ─────────────────

    #[test]
    fn native_skew_defaults_produz_identidade() {
        // Sem ax nem ay, skew produz matriz identidade.
        null_ctx!(ctx);
        use crate::entities::layout_types::TransformMatrix;
        let body = Content::text("body");
        let r = native_skew(
            &mut ctx,
            &p(vec![Value::Content(body)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            assert_eq!(e.matrix, TransformMatrix::identity());
        } else {
            panic!("esperado Content::Transform");
        }
    }

    #[test]
    fn native_skew_com_ax_angle() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Angle;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Angle(Angle::deg(30.0)));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            // c = tan(30°) ≈ 0.5774; a = 1; b = 0; d = 1.
            assert!((e.matrix.a - 1.0).abs() < 1e-9);
            assert!((e.matrix.b - 0.0).abs() < 1e-9);
            assert!(
                (e.matrix.c - 0.5774).abs() < 0.001,
                "c esperado tan(30°), obteve {}",
                e.matrix.c
            );
            assert!((e.matrix.d - 1.0).abs() < 1e-9);
        } else {
            panic!("esperado Content::Transform");
        }
    }

    #[test]
    fn native_skew_com_ay_angle() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Angle;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ay".into(), Value::Angle(Angle::deg(30.0)));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            // b = tan(30°) ≈ 0.5774; c = 0.
            assert!(
                (e.matrix.b - 0.5774).abs() < 0.001,
                "b esperado tan(30°), obteve {}",
                e.matrix.b
            );
            assert!((e.matrix.c - 0.0).abs() < 1e-9);
        } else {
            panic!("esperado Content::Transform");
        }
    }

    #[test]
    fn native_skew_combina_ax_e_ay() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Angle;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Angle(Angle::deg(15.0)));
        args.named.insert("ay".into(), Value::Angle(Angle::deg(45.0)));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            assert!((e.matrix.c - 15.0_f64.to_radians().tan()).abs() < 1e-9);
            assert!(
                (e.matrix.b - 1.0).abs() < 1e-9,
                "tan(45°) ≈ 1.0; obteve {}",
                e.matrix.b
            );
        } else {
            panic!("esperado Content::Transform");
        }
    }

    #[test]
    fn native_skew_aceita_float_radianos() {
        // Consistente com native_rotate: float é radianos directos.
        null_ctx!(ctx);
        use crate::entities::layout_types::TransformMatrix;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Float(0.0));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            // tan(0) = 0 → identidade.
            assert_eq!(e.matrix, TransformMatrix::identity());
        } else {
            panic!("esperado Content::Transform");
        }
    }

    #[test]
    fn native_skew_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Angle;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("origin".into(), Value::Str("center".into()));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "origin scope-out — named arg desconhecido em P156F");
        let _ = Angle::deg(0.0); // suprime warning de import não usado se ramo skip
    }

    #[test]
    fn native_skew_rejeita_ax_proximo_de_pi_meio() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        // 89.999° → ax_rad ≈ π/2 - 1.7e-5; abaixo do threshold (π/2 - 1e-3).
        // Mas usemos exactamente 90°: tan diverge.
        args.named
            .insert("ax".into(), Value::Float(std::f64::consts::FRAC_PI_2));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "skew(ax: π/2) deve retornar Err (tan diverge)");
    }

    #[test]
    fn native_skew_rejeita_ax_nao_angle_nem_float() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Str("30deg".into()));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "skew(ax:) com tipo errado deve retornar Err");
    }

    #[test]
    fn native_skew_sem_body_retorna_err() {
        null_ctx!(ctx);
        let r = native_skew(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "skew() sem body deve retornar Err");
    }

    // ── Passo 156F regression tests: move/rotate/scale ainda funcionam ──

    #[test]
    fn native_move_continua_a_produzir_transform_apos_p156f() {
        null_ctx!(ctx);
        use crate::entities::layout_types::TransformMatrix;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("dx".into(), Value::Float(10.0));
        args.named.insert("dy".into(), Value::Float(5.0));
        let r = native_move(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            assert_eq!(e.matrix, TransformMatrix::translate(10.0, 5.0));
        } else {
            panic!("regressão: native_move deveria produzir Content::Transform");
        }
    }

    #[test]
    fn native_rotate_continua_a_produzir_transform_apos_p156f() {
        null_ctx!(ctx);
        use crate::entities::layout_types::{Angle, TransformMatrix};
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("angle".into(), Value::Angle(Angle::deg(90.0)));
        let r = native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            // Comparação de matriz aproximada (rotate usa cos/sin).
            let expected = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
            assert!((e.matrix.a - expected.a).abs() < 1e-9);
            assert!((e.matrix.b - expected.b).abs() < 1e-9);
            assert!((e.matrix.c - expected.c).abs() < 1e-9);
            assert!((e.matrix.d - expected.d).abs() < 1e-9);
        } else {
            panic!("regressão: native_rotate deveria produzir Content::Transform");
        }
    }

    #[test]
    fn native_scale_continua_a_produzir_transform_apos_p156f() {
        null_ctx!(ctx);
        use crate::entities::layout_types::TransformMatrix;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("x".into(), Value::Float(2.0));
        let r = native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Transform(e)) = r {
            // x: 2.0; y default = sx = 2.0.
            assert_eq!(e.matrix, TransformMatrix::scale(2.0, 2.0));
        } else {
            panic!("regressão: native_scale deveria produzir Content::Transform");
        }
    }

    // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block ──────────────────

    #[test]
    fn native_block_defaults_sem_args_named() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_block(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.width, None);
            assert_eq!(e.height, None);
            assert_eq!(e.inset.left, Length::ZERO);
            assert!(e.breakable, "default breakable é true");
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_sem_body_aceita_empty() {
        // Vanilla aceita block() sem body; cristalino igualmente
        // (Content::Empty como fallback).
        null_ctx!(ctx);
        let r =
            native_block(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert!(e.body.is_empty());
        } else {
            panic!("esperado Content::Block com body Empty");
        }
    }

    #[test]
    fn native_block_com_width_length() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(100.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.width, Some(Length::pt(100.0)));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_com_height_int_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("height".into(), Value::Int(50));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.height, Some(Length::pt(50.0)));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_com_inset_uniforme() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(8.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.inset.left, Length::pt(8.0));
            assert_eq!(e.inset.right, Length::pt(8.0));
            assert_eq!(e.inset.top, Length::pt(8.0));
            assert_eq!(e.inset.bottom, Length::pt(8.0));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_com_breakable_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert!(!e.breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_combina_atributos() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(200.0)));
        args.named.insert("height".into(), Value::Length(Length::pt(80.0)));
        args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.width, Some(Length::pt(200.0)));
            assert_eq!(e.height, Some(Length::pt(80.0)));
            assert_eq!(e.inset.top, Length::pt(4.0));
            assert!(!e.breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_rejeita_named_arg_avancado() {
        // P247 — fill aceito (Color); Str("red") ainda inválido (tipo
        // errado). Test preserva intent: rejeitar tipo errado para fill.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(
            r.is_err(),
            "fill com tipo errado (Str em vez de Color) deve retornar Err"
        );
    }

    // ── Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4) ──────────────────
    //     native_block + native_box aceitam fill (Color) + stroke
    //     (Length/Color/Stroke shorthand reusando extract_stroke P227).

    #[test]
    fn p247_native_block_aceita_fill_color() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.fill, Some(Color::rgb(255, 0, 0)));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p247_native_block_aceita_stroke_length_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            let s = e.stroke.clone().expect("stroke deveria ser Some");
            assert_eq!(s.thickness, 2.0);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p247_native_block_fill_default_none() {
        null_ctx!(ctx);
        let r = native_block(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.fill, None);
            assert!(e.stroke.is_none());
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p247_native_block_fill_tipo_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "fill com Bool deve retornar Err");
    }

    #[test]
    fn p247_native_box_aceita_fill_color() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.fill, Some(Color::rgb(0, 255, 0)));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_box_aceita_stroke_color_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            let s = e.stroke.clone().expect("stroke deveria ser Some");
            assert_eq!(s.paint, Paint::Solid(Color::rgb(0, 0, 255)));
            assert_eq!(s.thickness, 1.0, "Color shorthand default 1pt thickness");
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_box_fill_default_none() {
        null_ctx!(ctx);
        let r = native_box(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.fill, None);
            assert!(e.stroke.is_none());
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_block_combina_fill_stroke_radius_clip_outset() {
        null_ctx!(ctx);
        use crate::entities::layout_types::{Color, Length};
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named
            .insert("fill".into(), Value::Color(Color::rgb(50, 100, 150)));
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.5)));
        args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
        args.named.insert("clip".into(), Value::Bool(true));
        args.named.insert("outset".into(), Value::Length(Length::pt(2.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.fill, Some(Color::rgb(50, 100, 150)));
            assert_eq!(e.stroke.clone().unwrap().thickness, 1.5);
            assert_eq!(e.radius.top_left, Length::pt(3.0));
            assert_eq!(e.clip, true);
            assert_eq!(e.outset.left, Length::pt(2.0));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_rejeita_width_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(-10.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "width negativo deve retornar Err em P156G");
    }

    // ── Passo 248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa) ──────
    //     Activação semantic real Block.breakable + Boxed.height; tests
    //     stdlib confirmam que parsing preserva valores (sem regressão
    //     defaults P156G/H).

    #[test]
    fn p248_native_block_breakable_false_propagado_a_variant() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(
                e.breakable, false,
                "P248 — native_block(breakable: false) propaga literal para variant"
            );
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p248_native_block_breakable_default_true_p156g() {
        null_ctx!(ctx);
        let r = native_block(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.breakable, true, "P248 — default preservado P156G");
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p248_native_box_height_some_propagado_a_variant() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("height".into(), Value::Length(Length::pt(50.0)));
        args.named.insert("clip".into(), Value::Bool(true));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.height, Some(Length::pt(50.0)));
            assert_eq!(e.clip, true);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p248_native_box_height_default_none_p156h() {
        null_ctx!(ctx);
        let r = native_box(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.height, None);
            assert_eq!(e.clip, false, "P248 — defaults preservados P156H");
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    // ── Passo 250 (M9d / M7+5; ADR-0079 Categoria A.4 Block COMPLETO;
    //     cita ADR-0082 PROPOSTO N=1 primeira aplicação citante) ──────
    //     native_block aceita 4 named args novos (spacing, above, below,
    //     sticky); defaults preservam P249.

    #[test]
    fn p250_native_block_aceita_spacing_above_below_sticky() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("spacing".into(), Value::Length(Length::pt(12.0)));
        args.named.insert("above".into(), Value::Length(Length::pt(20.0)));
        args.named.insert("below".into(), Value::Length(Length::pt(8.0)));
        args.named.insert("sticky".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.spacing, Some(Length::pt(12.0)));
            assert_eq!(e.above, Some(Length::pt(20.0)));
            assert_eq!(e.below, Some(Length::pt(8.0)));
            assert_eq!(e.sticky, true);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p250_native_block_defaults_4_fields() {
        null_ctx!(ctx);
        let r = native_block(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.spacing, None);
            assert_eq!(e.above, None);
            assert_eq!(e.below, None);
            assert_eq!(e.sticky, false, "P250 defaults preservam pre-P250");
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p250_native_block_spacing_negativo_rejeitado() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("spacing".into(), Value::Length(Length::pt(-5.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "P250 spacing negativo deve retornar Err");
    }

    #[test]
    fn p250_native_block_sticky_nao_bool_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("sticky".into(), Value::Int(1));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "P250 sticky Int em vez de Bool deve retornar Err");
    }

    #[test]
    fn p250_native_block_above_tipo_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("above".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "P250 above com Bool deve retornar Err");
    }

    // ── Passo 252 (M9d / M7+5; ADR-0079 Categoria A.4 Boxed COMPLETO
    //     6/6; cita ADR-0082 PROPOSTO N=3 terceira citante — limiar
    //     atingido) — extract_stroke Length/Color atalhos default
    //     vanilla overhang=true.

    #[test]
    fn p252_native_block_stroke_length_atalho_overhang_default_true() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.5)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            let s = e.stroke.clone().expect("stroke deveria ser Some");
            assert_eq!(s.thickness, 2.5);
            assert_eq!(
                s.overhang, true,
                "P252 — Length atalho default overhang=true (paridade vanilla)"
            );
        } else {
            panic!("esperado Content::Block com stroke");
        }
    }

    #[test]
    fn p252_native_block_stroke_color_atalho_overhang_default_true() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            let s = e.stroke.clone().expect("stroke deveria ser Some");
            assert_eq!(s.paint, Paint::Solid(Color::rgb(255, 0, 0)));
            assert_eq!(s.thickness, 1.0);
            assert_eq!(
                s.overhang, true,
                "P252 — Color atalho default overhang=true (paridade vanilla)"
            );
        } else {
            panic!("esperado Content::Block com stroke");
        }
    }

    #[test]
    fn p252_native_stroke_overhang_explicit_false() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // stroke(thickness: 2pt, overhang: false) → Stroke com
        // overhang=false explícito.
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("overhang".into(), Value::Bool(false));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.thickness, 2.0);
            assert_eq!(s.overhang, false);
        } else {
            panic!("esperado Value::Stroke");
        }
    }

    #[test]
    fn p252_native_stroke_overhang_default_true_paridade_vanilla() {
        null_ctx!(ctx);
        // stroke() sem args → Stroke { BLACK, 1.0pt, overhang: true }.
        let r =
            native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(
                s.overhang, true,
                "P252 — native_stroke default vanilla overhang=true"
            );
        } else {
            panic!("esperado Value::Stroke");
        }
    }

    #[test]
    fn p252_native_stroke_overhang_nao_bool_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("overhang".into(), Value::Int(1));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "P252 overhang com Int em vez de Bool deve retornar Err");
    }

    #[test]
    fn native_block_rejeita_inset_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(-2.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "inset negativo deve retornar Err em P156G");
    }

    #[test]
    fn native_block_rejeita_breakable_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("breakable".into(), Value::Int(1));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "breakable não-Bool deve retornar Err");
    }

    // Regression test: outros containers (Pad/Hide) continuam a funcionar
    // após adicionar Block (cobertura arms exaustiva foi correctamente
    // adicionada em todos os 9 sítios).
    #[test]
    fn native_pad_e_hide_continuam_a_funcionar_apos_p156g() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Pad regression
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(5.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(r, Value::Content(Content::Pad(e))),
            "regressão: native_pad deveria produzir Content::Pad"
        );
        // Hide regression
        let r = native_hide(
            &mut ctx,
            &p(vec![Value::Content(Content::text("y"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(
            matches!(r, Value::Content(Content::Hide(_))),
            "regressão: native_hide deveria produzir Content::Hide"
        );
    }

    // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box ────────────────────

    #[test]
    fn native_box_defaults_sem_args_named() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_box(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.width, None);
            assert_eq!(e.height, None);
            assert_eq!(e.inset.left, Length::ZERO);
            assert_eq!(e.baseline, Length::ZERO);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_sem_body_aceita_empty() {
        null_ctx!(ctx);
        let r = native_box(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert!(e.body.is_empty());
        } else {
            panic!("esperado Content::Boxed com body Empty");
        }
    }

    #[test]
    fn native_box_com_width_length() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(80.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.width, Some(Length::pt(80.0)));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_com_height_int_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("height".into(), Value::Int(20));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.height, Some(Length::pt(20.0)));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_com_inset_uniforme() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.inset.left, Length::pt(4.0));
            assert_eq!(e.inset.right, Length::pt(4.0));
            assert_eq!(e.inset.top, Length::pt(4.0));
            assert_eq!(e.inset.bottom, Length::pt(4.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_com_baseline() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(3.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.baseline, Length::pt(3.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_baseline_negativo_aceito() {
        // Diferente de width/height/inset: baseline negativo move
        // box para cima — semantic legítima.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(-5.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.baseline, Length::pt(-5.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_combina_atributos() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(100.0)));
        args.named.insert("height".into(), Value::Length(Length::pt(30.0)));
        args.named.insert("inset".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("baseline".into(), Value::Length(Length::pt(1.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.width, Some(Length::pt(100.0)));
            assert_eq!(e.height, Some(Length::pt(30.0)));
            assert_eq!(e.inset.top, Length::pt(2.0));
            assert_eq!(e.baseline, Length::pt(1.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_rejeita_atributo_avancado() {
        // fill/stroke/etc são scope-out per ADR-0054 graded.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "fill é scope-out em P156H; deve retornar Err");
    }

    #[test]
    fn native_box_rejeita_width_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(-5.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "width negativo deve retornar Err em P156H");
    }

    #[test]
    fn native_box_rejeita_inset_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(-1.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "inset negativo deve retornar Err em P156H");
    }

    #[test]
    fn native_box_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("alignment".into(), Value::Str("center".into()));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em box() deve retornar Err");
    }

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack ─────────────────

    #[test]
    fn native_stack_defaults_sem_args() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        let r =
            native_stack(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert!(e.children.is_empty());
            assert_eq!(e.dir, Dir::TTB); // default
            assert_eq!(e.spacing, None);
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_aceita_dir_ltr() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("dir".into(), Value::Str("ltr".into()));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.dir, Dir::LTR);
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_aceita_todas_4_direcoes() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        for (s, d) in
            [("ltr", Dir::LTR), ("rtl", Dir::RTL), ("ttb", Dir::TTB), ("btt", Dir::BTT)]
        {
            let mut args = p(vec![]);
            args.named.insert("dir".into(), Value::Str(s.into()));
            let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
            if let Value::Content(Content::Stack(e)) = r {
                assert_eq!(e.dir, d, "dir={s}");
            } else {
                panic!("esperado Content::Stack para dir={s}");
            }
        }
    }

    #[test]
    fn native_stack_aceita_spacing() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("spacing".into(), Value::Length(Length::pt(8.0)));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.spacing, Some(Length::pt(8.0)));
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_com_children_variadicos() {
        null_ctx!(ctx);
        let args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
            Value::Content(Content::text("c")),
        ]);
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.children.len(), 3);
            assert_eq!(e.children[0].plain_text(), "a");
            assert_eq!(e.children[1].plain_text(), "b");
            assert_eq!(e.children[2].plain_text(), "c");
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_aceita_str_como_child() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("hello".into())]);
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.children.len(), 1);
            assert_eq!(e.children[0].plain_text(), "hello");
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_rejeita_dir_invalido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("dir".into(), Value::Str("middle".into()));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "dir inválido deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_spacing_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("spacing".into(), Value::Length(Length::pt(-1.0)));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "spacing negativo deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("baseline".into(), Value::Bool(true));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em stack() deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_child_nao_content() {
        null_ctx!(ctx);
        let r = native_stack(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "child Int deve retornar Err");
    }

    #[test]
    fn native_stack_combina_dir_spacing_children() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;
        let mut args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        args.named.insert("dir".into(), Value::Str("ltr".into()));
        args.named.insert("spacing".into(), Value::Length(Length::pt(4.0)));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.children.len(), 2);
            assert_eq!(e.dir, Dir::LTR);
            assert_eq!(e.spacing, Some(Length::pt(4.0)));
        } else {
            panic!("esperado Content::Stack");
        }
    }

    // Regression: containers existentes (Block, Pad, Hide) continuam a
    // funcionar após adicionar Boxed (cobertura arms feita em 9 sítios).
    #[test]
    fn native_block_pad_hide_continuam_a_funcionar_apos_p156h() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Block regression
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(r, Value::Content(Content::Block(e))),
            "regressão: native_block deveria produzir Content::Block"
        );
        // Pad regression
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(5.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(
            matches!(r, Value::Content(Content::Pad(e))),
            "regressão: native_pad deveria produzir Content::Pad"
        );
        // Hide regression
        let r = native_hide(
            &mut ctx,
            &p(vec![Value::Content(Content::text("y"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(
            matches!(r, Value::Content(Content::Hide(_))),
            "regressão: native_hide deveria produzir Content::Hide"
        );
    }

    // P156I regression: Block + Boxed + Pad + Hide continuam a funcionar
    // após adicionar Stack (cobertura arms feita em 9 sítios mais Vec
    // adaptation).
    #[test]
    fn native_block_box_pad_hide_continuam_a_funcionar_apos_p156i() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Block
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Block(e))));
        // Box (Boxed)
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Boxed(e))));
        // Pad
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad(e))));
        // Hide
        let r = native_hide(
            &mut ctx,
            &p(vec![Value::Content(Content::text("y"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(r, Value::Content(Content::Hide(_))));
    }

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ────────────────

    #[test]
    fn native_repeat_defaults_gap_none_justify_true() {
        null_ctx!(ctx);
        let r = native_repeat(
            &mut ctx,
            &p(vec![Value::Content(Content::text("."))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Repeat(e)) = r {
            assert_eq!(e.body.plain_text(), ".");
            assert_eq!(e.gap, None);
            assert!(e.justify, "default justify == true (paridade vanilla)");
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_aceita_str_como_body() {
        null_ctx!(ctx);
        let r = native_repeat(
            &mut ctx,
            &p(vec![Value::Str(".".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Repeat(e)) = r {
            assert_eq!(e.body.plain_text(), ".");
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_com_gap_length() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("gap".into(), Value::Length(Length::pt(5.0)));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Repeat(e)) = r {
            assert_eq!(e.gap, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_com_justify_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("justify".into(), Value::Bool(false));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Repeat(e)) = r {
            assert!(!e.justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_combina_gap_e_justify() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("o"))]);
        args.named.insert("gap".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("justify".into(), Value::Bool(false));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Repeat(e)) = r {
            assert_eq!(e.body.plain_text(), "o");
            assert_eq!(e.gap, Some(Length::pt(2.0)));
            assert!(!e.justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_rejeita_sem_body() {
        null_ctx!(ctx);
        let r = native_repeat(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "repeat() sem body deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_body_int() {
        null_ctx!(ctx);
        let r = native_repeat(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "repeat() com body Int deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_gap_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("gap".into(), Value::Length(Length::pt(-1.0)));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "gap negativo deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_gap_nao_length() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("gap".into(), Value::Str("x".into()));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "gap não-length deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_justify_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("justify".into(), Value::Int(1));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "justify não-bool deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("count".into(), Value::Int(3));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em repeat() deve retornar Err");
    }

    // P156J regression: Stack + Block + Boxed + Pad + Hide continuam a
    // funcionar após adicionar Repeat (cobertura arms feita em 9 sítios).
    #[test]
    fn native_stack_block_box_pad_hide_continuam_a_funcionar_apos_p156j() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Stack
        let args = p(vec![Value::Content(Content::text("a"))]);
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Stack(e))));
        // Block
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Block(e))));
        // Box
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Boxed(e))));
        // Pad
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad(e))));
        // Hide
        let r = native_hide(
            &mut ctx,
            &p(vec![Value::Content(Content::text("y"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(r, Value::Content(Content::Hide(_))));
    }

    // ── P218 (DEBT-56 sub-fase b — Layout Fase 3) — columns ──────────

    #[test]
    fn p218_native_columns_count_valido_sem_gutter() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Content(Content::text("hello"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Columns(e)) = r {
            assert_eq!(e.count, 2);
            assert_eq!(e.gutter, None);
            assert_eq!(e.body.plain_text(), "hello");
        } else {
            panic!("esperado Content::Columns");
        }
    }

    #[test]
    fn p218_native_columns_count_zero_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Int(0), Value::Content(Content::text("."))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "count = 0 deve falhar");
    }

    #[test]
    fn p218_native_columns_count_negativo_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Int(-1), Value::Content(Content::text("."))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "count = -1 deve falhar");
    }

    #[test]
    fn p218_native_columns_count_nao_int_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Str("foo".into()), Value::Content(Content::text("."))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "count Str deve falhar");
    }

    #[test]
    fn p218_native_columns_count_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "sem args deve falhar");
    }

    #[test]
    fn p218_native_columns_body_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Int(2)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "só count sem body deve falhar");
    }

    #[test]
    fn p218_native_columns_body_str_aceita() {
        // Body Value::Str → convertido para Content::text.
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![Value::Int(3), Value::Str("texto".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Columns(e)) = r {
            assert_eq!(e.body.plain_text(), "texto");
        } else {
            panic!("esperado Content::Columns com body de Str");
        }
    }

    #[test]
    fn p218_native_columns_gutter_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(10.0)));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Columns(e)) = r {
            assert_eq!(e.gutter, Some(Length::pt(10.0)));
        } else {
            panic!("esperado Content::Columns com gutter Some");
        }
    }

    #[test]
    fn p218_native_columns_gutter_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(-1.0)));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "gutter negativo deve falhar");
    }

    #[test]
    fn p218_native_columns_named_arg_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
        args.named.insert("foo".into(), Value::Int(42));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido deve falhar");
    }

    #[test]
    fn p218_native_columns_extra_positional_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(
            &mut ctx,
            &p(vec![
                Value::Int(2),
                Value::Content(Content::text("a")),
                Value::Content(Content::text("b")), // extra
            ]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), ">2 posicionais deve falhar");
    }

    // ── P220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak ───────────────

    #[test]
    fn p220_native_colbreak_sem_args_aceita() {
        null_ctx!(ctx);
        let r =
            native_colbreak(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Colbreak(e)) = r {
            assert_eq!(e.weak, false, "default weak == false");
        } else {
            panic!("esperado Value::Content(Content::Colbreak)");
        }
    }

    #[test]
    fn p220_native_colbreak_weak_true_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Colbreak(e)) = r {
            assert_eq!(e.weak, true);
        } else {
            panic!("esperado Value::Content(Content::Colbreak {{ weak: true }})");
        }
    }

    #[test]
    fn p220_native_colbreak_posicional_rejeita() {
        null_ctx!(ctx);
        let r = native_colbreak(
            &mut ctx,
            &p(vec![Value::Str("oops".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "posicional deve falhar");
    }

    #[test]
    fn p220_native_colbreak_weak_nao_bool_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Str("true".into()));
        let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "weak não-Bool deve falhar");
    }

    #[test]
    fn p220_native_colbreak_named_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named desconhecido deve falhar");
    }

    #[test]
    fn p220_native_colbreak_to_rejeita() {
        // Paridade Pagebreak mas SEM `to` em colbreak (vanilla
        // ColbreakElem não tem; paridade só faz sentido em páginas).
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("even".into()));
        let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "to: deve falhar (named desconhecido)");
    }

    // ── P222 (Fase 4 Layout candidata sub-1; ADR-0066 Bloco C) — measure ──

    #[test]
    fn p222_native_measure_body_content_aceita() {
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(Content::text("texto"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            assert!(d.contains_key("width"));
            assert!(d.contains_key("height"));
        } else {
            panic!("esperado Value::Dict, recebeu {:?}", r);
        }
    }

    #[test]
    fn p222_native_measure_body_str_aceita() {
        // Str shortcut → Content::text wrapping.
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Str("texto".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            assert!(d.contains_key("width") && d.contains_key("height"));
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_body_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "body ausente deve falhar");
    }

    #[test]
    fn p222_native_measure_body_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "body Int deve falhar");
    }

    #[test]
    fn p222_native_measure_extra_positional_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![
                Value::Content(Content::text("a")),
                Value::Content(Content::text("b")),
            ]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), ">1 posicional deve falhar");
    }

    #[test]
    fn p222_native_measure_named_arg_rejeita() {
        // Opção β graded: width override scope-out per ADR-0054.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
        let r = native_measure(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg width deve falhar (scope-out graded)");
    }

    #[test]
    fn p222_native_measure_retorna_dict_com_width_height() {
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            assert!(
                matches!(d.get("width"), Some(Value::Length(_))),
                "key 'width' deve ser Value::Length"
            );
            assert!(
                matches!(d.get("height"), Some(Value::Length(_))),
                "key 'height' deve ser Value::Length"
            );
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_dimensoes_para_shape_rect() {
        // Helper measure_content tem suporte explícito para Shape::Rect
        // (retorna width/height resolvidos). Texto simples retorna (0, 0)
        // per limitação documentada do helper.
        null_ctx!(ctx);
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::Length;
        let rect = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(40.0)))),
            Some(Box::new(Value::Length(Length::pt(20.0)))),
            None,
            None,
        );
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(rect)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            if let Some(Value::Length(w)) = d.get("width") {
                assert!(w.abs.0 > 0.0, "rect width > 0 esperado");
            } else {
                panic!("width não-Length");
            }
            if let Some(Value::Length(h)) = d.get("height") {
                assert!(h.abs.0 > 0.0, "rect height > 0 esperado");
            } else {
                panic!("height não-Length");
            }
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_dimensoes_zero_para_empty() {
        // Content::Empty → helper retorna (0, 0).
        null_ctx!(ctx);
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(Content::Empty)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            if let Some(Value::Length(w)) = d.get("width") {
                assert_eq!(w.abs.0, 0.0);
            } else {
                panic!("width não-Length");
            }
            if let Some(Value::Length(h)) = d.get("height") {
                assert_eq!(h.abs.0, 0.0);
            } else {
                panic!("height não-Length");
            }
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_sequence_compose_dimensoes() {
        // Integration: helper trata Sequence acumulando height + max width
        // de children. Sequence de 2 Shape rect verticalmente → height
        // soma; width max.
        null_ctx!(ctx);
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::Length;
        use std::sync::Arc;
        let r1 = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(30.0)))),
            Some(Box::new(Value::Length(Length::pt(10.0)))),
            None,
            None,
        );
        let r2 = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(50.0)))),
            Some(Box::new(Value::Length(Length::pt(15.0)))),
            None,
            None,
        );
        let seq = Content::Sequence(Arc::from(vec![r1, r2]));
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(seq)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Dict(d) = r {
            // max_w = 50, total_h = 25.
            if let Some(Value::Length(w)) = d.get("width") {
                assert!(w.abs.0 >= 50.0, "Sequence max width >= 50, recebeu {}", w.abs.0);
            } else {
                panic!("width não-Length");
            }
            if let Some(Value::Length(h)) = d.get("height") {
                assert!(
                    h.abs.0 >= 25.0,
                    "Sequence total height >= 25, recebeu {}",
                    h.abs.0
                );
            } else {
                panic!("height não-Length");
            }
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_round_trip_dict_access_shape_observable() {
        // Round-trip: simula `let d = measure([rect]); d.width` —
        // verifica que Dict permite indexação por key paridade vanilla
        // `measure(body).width` observable.
        null_ctx!(ctx);
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::Length;
        let rect = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(20.0)))),
            Some(Box::new(Value::Length(Length::pt(40.0)))),
            None,
            None,
        );
        let r = native_measure(
            &mut ctx,
            &p(vec![Value::Content(rect)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Paridade vanilla: `dims.width` retorna Length.
        if let Value::Dict(d) = &r {
            let w = d.get("width").cloned().expect("key 'width' presente");
            let h = d.get("height").cloned().expect("key 'height' presente");
            assert!(
                matches!(w, Value::Length(_)),
                "width deve indexar como Length (paridade observable vanilla)"
            );
            assert!(matches!(h, Value::Length(_)), "height deve indexar como Length");
            // Dict tem exactamente 2 keys.
            assert_eq!(d.len(), 2, "Dict deve ter exactamente 2 keys (width + height)");
        } else {
            panic!("esperado Value::Dict, recebeu {:?}", r);
        }
    }

    // ── P223 (Fase 4 Layout candidata sub-2; refino native_place +float +clearance) ──

    #[test]
    fn p223_native_place_float_aceita() {
        // place(top, float: true, body) → float armazenado.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("float".into(), Value::Bool(true));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Place(e)) = r {
            assert_eq!(e.float, true);
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_float_default_false() {
        // place(body) sem float → float == false.
        null_ctx!(ctx);
        let r = native_place(
            &mut ctx,
            &p(vec![Value::Content(Content::text("a"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Place(e)) = r {
            assert_eq!(e.float, false, "default float == false");
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_float_nao_bool_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("float".into(), Value::Str("yes".into()));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "float não-Bool deve falhar");
    }

    #[test]
    fn p223_native_place_clearance_length_aceita() {
        // place(top, float: true, clearance: 5pt, body) → Some(5pt).
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("float".into(), Value::Bool(true));
        args.named.insert("clearance".into(), Value::Length(Length::pt(5.0)));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Place(e)) = r {
            assert_eq!(e.clearance, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_clearance_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("clearance".into(), Value::Length(Length::pt(-5.0)));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "clearance negativo deve falhar");
    }

    #[test]
    fn p223_native_place_parent_sem_float_rejeita() {
        // DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("scope".into(), Value::Str("parent".into()));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
        assert!(
            r.is_err(),
            "scope 'parent' sem float deve falhar (paridade vanilla; DEBT-37 fechada)"
        );
    }

    #[test]
    fn p223_native_place_parent_com_float_aceita() {
        // place(scope: "parent", float: true, body) → OK.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("scope".into(), Value::Str("parent".into()));
        args.named.insert("float".into(), Value::Bool(true));
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        use crate::entities::layout_types::PlaceScope;
        if let Value::Content(Content::Place(e)) = r {
            assert!(matches!(e.scope, PlaceScope::Parent));
            assert_eq!(e.float, true);
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_column_sem_restricao() {
        // scope: "column" OK independente de float (default scope).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("scope".into(), Value::Str("column".into()));
        // sem float
        let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "scope 'column' sem float OK (não tem restrição vanilla)");
    }

    // ── P224 (Fase 4 Layout candidata sub-3) — grid refino + grid_cell/header/footer ──

    #[test]
    fn p224_native_grid_aceita_gutter() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(5.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert_eq!(e.gutter, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p224_native_grid_gutter_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(-5.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "gutter negativo deve falhar");
    }

    #[test]
    fn p224_native_grid_inset_uniforme_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert_eq!(e.inset.left, Length::pt(3.0));
            assert_eq!(e.inset.right, Length::pt(3.0));
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p224_native_grid_header_footer_content_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named
            .insert("header".into(), Value::Content(Content::text("HDR")));
        args.named
            .insert("footer".into(), Value::Content(Content::text("FTR")));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert!(e.header.is_some(), "header presente");
            assert!(e.footer.is_some(), "footer presente");
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p224_native_grid_named_arg_desconhecido_rejeita() {
        // stroke/fill cosméticos scope-out — desconhecidos rejeitados.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Str("black".into()));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "stroke scope-out deve falhar");
    }

    #[test]
    fn p224_native_grid_cell_body_obrigatorio() {
        null_ctx!(ctx);
        let r = native_grid_cell(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "body obrigatório");
    }

    #[test]
    fn p224_native_grid_cell_x_y_colspan_rowspan_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("cell"))]);
        args.named.insert("x".into(), Value::Int(1));
        args.named.insert("y".into(), Value::Int(0));
        args.named.insert("colspan".into(), Value::Int(2));
        args.named.insert("rowspan".into(), Value::Int(3));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridCell(e)) = r {
            assert_eq!(e.x, Some(1));
            assert_eq!(e.y, Some(0));
            assert_eq!(e.colspan, Some(2));
            assert_eq!(e.rowspan, Some(3));
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p224_native_grid_cell_colspan_zero_rejeita() {
        // ADR-0064 Caso C — colspan >= 1.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("colspan".into(), Value::Int(0));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "colspan 0 deve falhar (paridade NonZeroUsize)");
    }

    #[test]
    fn p224_native_grid_header_aceita_body() {
        null_ctx!(ctx);
        let r = native_grid_header(
            &mut ctx,
            &p(vec![Value::Content(Content::text("hdr"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::GridHeader(e)) = r {
            assert_eq!(e.body.plain_text(), "hdr");
            assert_eq!(e.repeat, true, "default repeat == true (paridade vanilla)");
        } else {
            panic!("esperado GridHeader");
        }
    }

    #[test]
    fn p224_native_grid_footer_aceita_body_repeat_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("ftr"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r =
            native_grid_footer(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridFooter(e)) = r {
            assert_eq!(e.body.plain_text(), "ftr");
            assert_eq!(e.repeat, false);
        } else {
            panic!("esperado GridFooter");
        }
    }

    // ── P227 (Fase 5 Layout Categoria A.1) — Value::Stroke + native_stroke + stroke shorthand ──

    #[test]
    fn p227_value_stroke_type_name_e_eq() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(255, 0, 0)),
            thickness: 2.5,
            overhang: false,
        };
        let v = Value::Stroke(s.clone());
        assert_eq!(v.type_name(), "stroke");
        let v2 = Value::Stroke(s);
        assert_eq!(v, v2);
    }

    #[test]
    fn p227_native_stroke_defaults_aceita() {
        // stroke() sem args → Stroke { BLACK, 1.0pt }.
        null_ctx!(ctx);
        let r =
            native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.thickness, 1.0);
        } else {
            panic!("esperado Value::Stroke");
        }
    }

    #[test]
    fn p227_native_stroke_thickness_2pt_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.thickness, 2.0);
        } else {
            panic!("esperado Value::Stroke");
        }
    }

    #[test]
    fn p227_native_stroke_thickness_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(-1.0)));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "thickness negativo deve falhar");
    }

    #[test]
    fn p227_native_stroke_thickness_zero_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(0.0)));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "thickness 0 deve falhar (paridade vanilla)");
    }

    #[test]
    fn p227_native_stroke_named_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named desconhecido deve falhar");
    }

    #[test]
    fn p227_native_stroke_posicional_rejeita() {
        null_ctx!(ctx);
        let r = native_stroke(
            &mut ctx,
            &p(vec![Value::Bool(true)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "posicional deve falhar");
    }

    #[test]
    fn p227_native_grid_stroke_length_shorthand() {
        // grid(stroke: 2pt) → Stroke{BLACK, 2pt}.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert!(e.stroke.is_some());
            assert_eq!(e.stroke.clone().unwrap().thickness, 2.0);
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p227_native_grid_stroke_color_shorthand() {
        // grid(stroke: red) → Stroke{red, 1pt}.
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            let s = e.stroke.clone().expect("stroke presente");
            assert_eq!(s.thickness, 1.0);
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p227_native_grid_stroke_value_stroke_passa() {
        // grid(stroke: stroke(thickness: 3pt)) → preserva Stroke.
        null_ctx!(ctx);
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(0, 255, 0)),
            thickness: 3.0,
            overhang: false,
        };
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Stroke(s.clone()));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert_eq!(e.stroke, Some(s));
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p227_native_table_stroke_paridade_grid() {
        // table(stroke: 1pt) paridade native_grid.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert!(e.stroke.is_some());
        } else {
            panic!("esperado Content::Table");
        }
    }

    // ── P228 (Fase 5 Layout Categoria A.2) — fill Grid + Table ──

    #[test]
    fn p228_native_grid_fill_color_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named
            .insert("fill".into(), Value::Color(Color::rgb(255, 200, 0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p228_native_grid_fill_default_none() {
        null_ctx!(ctx);
        let r = native_grid(
            &mut ctx,
            &p(vec![Value::Content(Content::text("a"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert!(e.fill.is_none(), "default fill == None");
        } else {
            panic!("esperado Content::Grid");
        }
    }

    #[test]
    fn p228_native_grid_fill_tipo_errado_rejeita() {
        // fill aceita só Color (não Length); rejeita explicitamente.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
    }

    #[test]
    fn p228_native_table_fill_paridade_grid() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named
            .insert("fill".into(), Value::Color(Color::rgb(100, 100, 100)));
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn p228_native_grid_fill_e_stroke_simultaneos_aceita() {
        // Ambos fill + stroke aceitos no mesmo grid() call.
        null_ctx!(ctx);
        use crate::entities::layout_types::{Color, Length};
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Grid(e)) = r {
            assert!(e.fill.is_some() && e.stroke.is_some());
        } else {
            panic!("esperado Content::Grid");
        }
    }

    // ── P230 (Fase 5 Layout Categoria A.3) — stroke/fill per-cell GridCell + TableCell ──

    #[test]
    fn p230_native_grid_cell_stroke_aceita_length_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridCell(e)) = r {
            assert!(e.stroke.is_some());
            assert_eq!(e.stroke.clone().unwrap().thickness, 2.0);
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p230_native_grid_cell_stroke_aceita_color_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridCell(e)) = r {
            assert!(e.stroke.is_some());
            assert_eq!(e.stroke.clone().unwrap().thickness, 1.0);
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p230_native_grid_cell_fill_color_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridCell(e)) = r {
            assert!(e.fill.is_some());
        } else {
            panic!("esperado GridCell");
        }
    }

    #[test]
    fn p230_native_grid_cell_fill_length_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
    }

    #[test]
    fn p230_native_table_cell_paridade_gridcell() {
        // table_cell aceita stroke + fill (paridade native_grid_cell).
        null_ctx!(ctx);
        use crate::entities::layout_types::{Color, Length};
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 0, 255)));
        let r =
            native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableCell(e)) = r {
            assert!(e.stroke.is_some() && e.fill.is_some());
        } else {
            panic!("esperado TableCell");
        }
    }

    #[test]
    fn p230_native_grid_cell_stroke_e_fill_simultaneos() {
        // Ambos aceitos no mesmo grid_cell call.
        null_ctx!(ctx);
        use crate::entities::layout_types::{Color, Length};
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        args.named
            .insert("fill".into(), Value::Color(Color::rgb(100, 100, 100)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::GridCell(e)) = r {
            assert!(e.stroke.is_some() && e.fill.is_some());
        } else {
            panic!("esperado GridCell");
        }
    }

    // ── P231 (Fase 5 Layout Categoria A.4) — Block/Boxed outset/radius/clip ──

    #[test]
    fn p231_native_block_outset_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(5.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.outset.left, Length::pt(5.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p231_native_block_outset_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(-3.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "outset negativo deve falhar");
    }

    #[test]
    fn p231_native_block_radius_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            // P242 adapta: e.radius `Option<Length>` → `Corners<Length>`.
            // Length uniforme via stdlib → `Corners::uniform`.
            assert_eq!(e.radius.top_left, Length::pt(3.0));
            assert_eq!(e.radius.top_right, Length::pt(3.0));
            assert_eq!(e.radius.bottom_right, Length::pt(3.0));
            assert_eq!(e.radius.bottom_left, Length::pt(3.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p231_native_block_radius_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "radius negativo deve falhar");
    }

    #[test]
    fn p231_native_block_clip_bool_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("clip".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.clip, true);
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p231_native_block_clip_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("clip".into(), Value::Str("yes".into()));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "clip Str deve falhar (espera Bool)");
    }

    #[test]
    fn p231_native_box_paridade_block() {
        // native_box aceita os 3 cosméticos paralelo native_block.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("radius".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("clip".into(), Value::Bool(true));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.outset.top, Length::pt(2.0));
            // P242 adapta: e.radius `Corners<Length>` via uniform.
            assert_eq!(e.radius.top_left, Length::pt(1.0));
            assert_eq!(e.clip, true);
        } else {
            panic!("esperado Boxed");
        }
    }

    #[test]
    fn p231_native_block_3_fields_simultaneos() {
        // outset + radius + clip + breakable simultâneos.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("radius".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("clip".into(), Value::Bool(true));
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.outset.left, Length::pt(1.0));
            // P242 adapta: e.radius `Corners<Length>` via uniform.
            assert_eq!(e.radius.top_left, Length::pt(2.0));
            assert_eq!(e.clip, true);
            assert_eq!(e.breakable, false);
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p231_native_block_outset_radius_clip_defaults() {
        // Sem args → outset=Sides::uniform(zero); radius=None; clip=false.
        null_ctx!(ctx);
        let r = native_block(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.outset.left, crate::entities::layout_types::Length::ZERO);
            // P242 adapta: e.radius default `Corners::uniform(Length::ZERO)`.
            assert_eq!(e.radius.top_left, crate::entities::layout_types::Length::ZERO);
            assert_eq!(e.radius.top_right, crate::entities::layout_types::Length::ZERO);
            assert_eq!(
                e.radius.bottom_right,
                crate::entities::layout_types::Length::ZERO
            );
            assert_eq!(e.radius.bottom_left, crate::entities::layout_types::Length::ZERO);
            assert_eq!(e.clip, false);
        } else {
            panic!("esperado Block");
        }
    }

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table ─────────────────

    #[test]
    fn native_table_defaults_columns_rows_auto() {
        // P157A: defaults — columns/rows omitidos caem em [Auto]
        // (paridade com Grid em P83).
        null_ctx!(ctx);
        use crate::entities::layout_types::TrackSizing;
        let r =
            native_table(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert_eq!(e.columns, vec![TrackSizing::Auto]);
            assert_eq!(e.rows, vec![TrackSizing::Auto]);
            assert!(e.children.is_empty());
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_columns_int() {
        // `#table(columns: 3)` → 3 tracks Auto (paridade Grid).
        null_ctx!(ctx);
        use crate::entities::layout_types::TrackSizing;
        let mut args = p(vec![]);
        args.named.insert("columns".into(), Value::Int(3));
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert_eq!(
                e.columns,
                vec![TrackSizing::Auto, TrackSizing::Auto, TrackSizing::Auto]
            );
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_columns_array_lengths() {
        // `#table(columns: (10pt, 20pt))` → 2 tracks Fixed.
        null_ctx!(ctx);
        use crate::entities::layout_types::{Length, TrackSizing};
        let mut args = p(vec![]);
        args.named.insert(
            "columns".into(),
            Value::Array(vec![
                Value::Length(Length::pt(10.0)),
                Value::Length(Length::pt(20.0)),
            ]),
        );
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert_eq!(e.columns.len(), 2);
            assert!(
                matches!(e.columns[0], TrackSizing::Fixed(v) if (v - 10.0).abs() < 1e-6)
            );
            assert!(
                matches!(e.columns[1], TrackSizing::Fixed(v) if (v - 20.0).abs() < 1e-6)
            );
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_children_variadicos() {
        // `#table(columns: 2)[a][b][c][d]` → 4 children.
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
            Value::Content(Content::text("c")),
            Value::Content(Content::text("d")),
        ]);
        args.named.insert("columns".into(), Value::Int(2));
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert_eq!(e.children.len(), 4);
            assert_eq!(e.children[0].plain_text(), "a");
            assert_eq!(e.children[3].plain_text(), "d");
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_aceita_str_como_child() {
        null_ctx!(ctx);
        let r = native_table(
            &mut ctx,
            &p(vec![Value::Str("hello".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Table(e)) = r {
            assert_eq!(e.children.len(), 1);
            assert_eq!(e.children[0].plain_text(), "hello");
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("inset".into(), Value::Length(Length::pt(5.0)));
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em table() deve retornar Err (atributos avançados scope-out per ADR-0054 graded)");
    }

    #[test]
    fn native_table_rejeita_child_int() {
        null_ctx!(ctx);
        let r = native_table(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "child Int em table() deve retornar Err");
    }

    #[test]
    fn native_table_paridade_estrutural_com_grid() {
        // P157A: `table` e `grid` com mesmos args produzem variants
        // diferentes (semântica distinta) mas estrutura paralela
        // (mesmas tracks + mesmo número de cells/children).
        null_ctx!(ctx);
        let mut g_args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        g_args.named.insert("columns".into(), Value::Int(2));
        let g = native_grid(&mut ctx, &g_args, &null_world(), test_file_id()).unwrap();

        let mut t_args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        t_args.named.insert("columns".into(), Value::Int(2));
        let t = native_table(&mut ctx, &t_args, &null_world(), test_file_id()).unwrap();

        // Variants diferentes — não são iguais por PartialEq.
        assert_ne!(g, t);
        // Mas ambos têm 2 cells/children e 2 columns.
        if let (Value::Content(Content::Grid(ge)), Value::Content(Content::Table(te))) =
            (g, t)
        {
            assert_eq!(ge.cells.len(), te.children.len());
            assert_eq!(ge.columns.len(), te.columns.len());
        } else {
            panic!("esperado Grid + Table");
        }
    }

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table_cell ────────────

    #[test]
    fn native_table_cell_defaults_todos_none() {
        // P157B: defaults — body required; outros fields None.
        null_ctx!(ctx);
        let r = native_table_cell(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::TableCell(e)) = r {
            assert_eq!(e.body.plain_text(), "body");
            assert_eq!(e.x, None);
            assert_eq!(e.y, None);
            assert_eq!(e.colspan, None);
            assert_eq!(e.rowspan, None);
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn native_table_cell_x_y_explicitos() {
        // ADR-0064 Caso A: Some(n) ↔ posição explícita.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("x".into(), Value::Int(2));
        args.named.insert("y".into(), Value::Int(3));
        let r =
            native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableCell(e)) = r {
            assert_eq!(e.x, Some(2));
            assert_eq!(e.y, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn native_table_cell_x_auto_traduz_a_none() {
        // P157B ADR-0064 Caso A: Value::Auto → None.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("x".into(), Value::Auto);
        let r =
            native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableCell(e)) = r {
            assert_eq!(
                e.x, None,
                "Value::Auto deve traduzir para None per ADR-0064 Caso A"
            );
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn native_table_cell_colspan_rowspan_explicitos() {
        // ADR-0064 Caso C: Some(n) ↔ span explícito.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("colspan".into(), Value::Int(2));
        args.named.insert("rowspan".into(), Value::Int(3));
        let r =
            native_table_cell(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableCell(e)) = r {
            assert_eq!(e.colspan, Some(2));
            assert_eq!(e.rowspan, Some(3));
        } else {
            panic!("esperado Content::TableCell");
        }
    }

    #[test]
    fn native_table_cell_colspan_zero_rejeitado() {
        // P157B: colspan = 0 rejeitado (paridade vanilla NonZeroUsize).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("colspan".into(), Value::Int(0));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "colspan = 0 deve retornar Err (mínimo 1)");
    }

    #[test]
    fn native_table_cell_colspan_negativo_rejeitado() {
        // P157B: int negativo rejeitado.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("colspan".into(), Value::Int(-1));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "colspan negativo deve retornar Err");
    }

    #[test]
    fn native_table_cell_x_negativo_rejeitado() {
        // P157B: x negativo rejeitado (mínimo 0; sem ints negativos
        // mesmo para campos com min=0).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("x".into(), Value::Int(-1));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "x negativo deve retornar Err");
    }

    #[test]
    fn native_table_cell_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        // P235 — `inset` agora conhecido (Categoria B.3); test usa
        // `outset` que continua scope-out per ADR-0054 graded.
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Int(5));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em table_cell() deve retornar Err");
    }

    #[test]
    fn native_table_cell_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_cell(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "table_cell() sem body deve retornar Err");
    }

    // ── Passo 235 (Fase 5 Layout Categoria B.3) — algorítmicos per-cell ──

    #[test]
    fn p235_native_grid_cell_align_aceita() {
        use crate::entities::layout_types::Align2D;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named
            .insert("align".into(), Value::Align(Align2D::from_string("center")));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        match r {
            Ok(Value::Content(Content::GridCell(e))) => assert!(e.align.is_some()),
            other => panic!("esperado GridCell align Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_grid_cell_inset_length_uniforme_aceita() {
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(5.0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        match r {
            Ok(Value::Content(Content::GridCell(e))) => assert!(e.inset.is_some()),
            other => panic!("esperado GridCell inset Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_grid_cell_breakable_bool_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        match r {
            Ok(Value::Content(Content::GridCell(e))) => {
                assert_eq!(e.breakable, Some(false))
            }
            other => {
                panic!("esperado GridCell breakable Some(false), recebeu {:?}", other)
            }
        }
    }

    #[test]
    fn p235_native_grid_cell_breakable_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("breakable".into(), Value::Int(1));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "breakable Int (não Bool) deve rejeitar pós-P235");
    }

    #[test]
    fn p235_native_table_cell_align_paridade_gridcell() {
        use crate::entities::layout_types::Align2D;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named
            .insert("align".into(), Value::Align(Align2D::from_string("right")));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        match r {
            Ok(Value::Content(Content::TableCell(e))) => assert!(e.align.is_some()),
            other => panic!("esperado TableCell align Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_table_cell_inset_paridade_gridcell() {
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id());
        match r {
            Ok(Value::Content(Content::TableCell(e))) => assert!(e.inset.is_some()),
            other => panic!("esperado TableCell inset Some, recebeu {:?}", other),
        }
    }

    // ── Passo 236 (Fase 5 Layout candidata Categoria D 1/? — refino aditivo
    //     pós-P236.div-1; state runtime já materializado P171+M9+M9c) ──

    #[test]
    fn p236_state_final_introspector_vazio_retorna_none() {
        null_ctx!(ctx);
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Str("counter_x".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Iter 0 fixpoint: introspector vazio → None.
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p236_state_final_apos_init_retorna_init_value() {
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular StateRegistry directamente via init.
        ctx.introspector.state.init(
            "k".to_string(),
            Value::Int(42),
            Location::from_raw(1),
        );
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Str("k".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Sem updates → final = init.
        assert_eq!(r, Value::Int(42));
    }

    #[test]
    fn p236_state_final_apos_updates_retorna_ultimo_valor() {
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector.state.init(
            "k".to_string(),
            Value::Int(1),
            Location::from_raw(1),
        );
        ctx.introspector.state.update(
            "k".to_string(),
            Value::Int(2),
            Location::from_raw(2),
        );
        ctx.introspector.state.update(
            "k".to_string(),
            Value::Int(99),
            Location::from_raw(3),
        );
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Str("k".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Último update vence — paridade vanilla state.final().
        assert_eq!(r, Value::Int(99));
    }

    #[test]
    fn p236_state_final_key_inexistente_retorna_none() {
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular outra key (não a key consultada).
        ctx.introspector.state.init(
            "outra".to_string(),
            Value::Int(7),
            Location::from_raw(1),
        );
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p236_state_final_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "arg não-string deve retornar Err");
    }

    #[test]
    fn p236_state_final_zero_args_retorna_err() {
        null_ctx!(ctx);
        let r = native_state_final(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "zero args deve retornar Err");
    }

    // ── Passo 237 (Fase 5 Layout candidata Categoria D 1/? — refino estendido;
    //     paralelo absoluto state_at ↔ counter_at P177) ──

    #[test]
    fn p237_state_at_label_inexistente_retorna_none() {
        null_ctx!(ctx);
        // Sem labels nem state populados; label_str não resolve.
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Paridade counter_at empty default: state_at retorna Value::None.
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p237_state_at_key_inexistente_retorna_none() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular label + outro state (não a key consultada).
        ctx.introspector
            .labels
            .add(Label("intro".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "outra".to_string(),
            Value::Int(7),
            Location::from_raw(5),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into()), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p237_state_at_resolve_label_retorna_init() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular label "intro" → Location 5; state init em location 1.
        ctx.introspector
            .labels
            .add(Label("intro".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "k".to_string(),
            Value::Int(42),
            Location::from_raw(1),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Sem updates entre init e label → init wins.
        assert_eq!(r, Value::Int(42));
    }

    #[test]
    fn p237_state_at_updates_antes_location_visivel() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector
            .labels
            .add(Label("at_5".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "k".to_string(),
            Value::Int(1),
            Location::from_raw(1),
        );
        // 2 updates antes da location consultada (raw < 5).
        ctx.introspector.state.update(
            "k".to_string(),
            Value::Int(2),
            Location::from_raw(2),
        );
        ctx.introspector.state.update(
            "k".to_string(),
            Value::Int(3),
            Location::from_raw(3),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("at_5".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Valor em location=5 reflecte último update <= 5: Int(3).
        assert_eq!(r, Value::Int(3));
    }

    #[test]
    fn p237_state_at_updates_depois_location_nao_visiveis() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector
            .labels
            .add(Label("at_2".to_string()), Location::from_raw(2));
        ctx.introspector.state.init(
            "k".to_string(),
            Value::Int(10),
            Location::from_raw(1),
        );
        // Update em location 5 (depois da location consultada raw=2).
        ctx.introspector.state.update(
            "k".to_string(),
            Value::Int(99),
            Location::from_raw(5),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("at_2".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        // Em location=2: só init (raw 1) visível; update raw 5 invisível.
        assert_eq!(r, Value::Int(10));
    }

    #[test]
    fn p237_state_at_arg_nao_string_rejeita() {
        null_ctx!(ctx);
        // Key Int (não Str).
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Int(42), Value::Str("intro".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "key não-string deve retornar Err");
        // Label Int (não Str).
        let r2 = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(7)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r2.is_err(), "label não-string deve retornar Err");
    }

    #[test]
    fn p237_state_at_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 = native_state_at(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 1 arg.
        let r1 = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r1.is_err(), "1 arg deve retornar Err");
        // 3 args.
        let r3 = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("l".into()), Value::Int(1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r3.is_err(), "3 args deve retornar Err");
    }

    // ── Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5) —
    //     block/box radius `Corners<Length>` dict por canto + helper
    //     extract_corners_length_value ──

    #[test]
    fn p242_native_block_radius_length_uniforme() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(5.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.radius.top_left, Length::pt(5.0));
            assert_eq!(e.radius.top_right, Length::pt(5.0));
            assert_eq!(e.radius.bottom_right, Length::pt(5.0));
            assert_eq!(e.radius.bottom_left, Length::pt(5.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p242_native_block_radius_dict_por_canto() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("top-left".into(), Value::Length(Length::pt(1.0)));
        d.insert("top-right".into(), Value::Length(Length::pt(2.0)));
        d.insert("bottom-right".into(), Value::Length(Length::pt(3.0)));
        d.insert("bottom-left".into(), Value::Length(Length::pt(4.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            assert_eq!(e.radius.top_left, Length::pt(1.0));
            assert_eq!(e.radius.top_right, Length::pt(2.0));
            assert_eq!(e.radius.bottom_right, Length::pt(3.0));
            assert_eq!(e.radius.bottom_left, Length::pt(4.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p242_native_block_radius_dict_precedencia_eixo_rest() {
        // Spec Decisão 4: precedência específico > eixo > rest.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        // top específico → ambos top-left/top-right.
        d.insert("top".into(), Value::Length(Length::pt(10.0)));
        // left específico → top-left/bottom-left mas top vence em top-left.
        d.insert("left".into(), Value::Length(Length::pt(20.0)));
        // rest fallback para o que sobra (bottom-right).
        d.insert("rest".into(), Value::Length(Length::pt(99.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Block(e)) = r {
            // top-left: top OR left (precedência top vence: top:10pt).
            assert_eq!(e.radius.top_left, Length::pt(10.0));
            // top-right: top.
            assert_eq!(e.radius.top_right, Length::pt(10.0));
            // bottom-left: left vence (bottom não-set; rest fallback).
            assert_eq!(e.radius.bottom_left, Length::pt(20.0));
            // bottom-right: rest fallback.
            assert_eq!(e.radius.bottom_right, Length::pt(99.0));
        } else {
            panic!("esperado Block");
        }
    }

    #[test]
    fn p242_native_block_radius_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "radius negativo deve falhar (preservado P231→P242)");
    }

    #[test]
    fn p242_native_block_radius_chave_canto_invalida_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("northwest".into(), Value::Length(Length::pt(1.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "chave canto inválida deve falhar");
    }

    #[test]
    fn p242_native_box_radius_paridade_block() {
        // native_box aceita radius paralelo native_block (Decisão 1+4).
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("top-left".into(), Value::Length(Length::pt(7.0)));
        d.insert("rest".into(), Value::Length(Length::pt(0.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Boxed(e)) = r {
            assert_eq!(e.radius.top_left, Length::pt(7.0));
            assert_eq!(e.radius.top_right, Length::pt(0.0));
            assert_eq!(e.radius.bottom_right, Length::pt(0.0));
            assert_eq!(e.radius.bottom_left, Length::pt(0.0));
        } else {
            panic!("esperado Boxed");
        }
    }

    // ── Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ) — state_display
    //     render-mediated walk-time real via apply_state_displays paralelo
    //     apply_state_funcs ──

    #[test]
    fn p240_native_state_display_sem_callback_constroi_variant() {
        null_ctx!(ctx);
        let r = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::StateDisplay(e)) = r {
            assert_eq!(e.key, "k");
            assert!(e.callback.is_none(), "1-arg → callback=None");
        } else {
            panic!("esperado Content::StateDisplay");
        }
    }

    #[test]
    fn p240_native_state_display_com_callback_constroi_some() {
        use crate::entities::func::Func;
        null_ctx!(ctx);
        let identity_fn = Func::native("identity", |_, args, _, _| {
            Ok(args.items.first().cloned().unwrap_or(Value::None))
        });
        let r = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Func(identity_fn)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::StateDisplay(e)) = r {
            assert_eq!(e.key, "k");
            assert!(e.callback.is_some(), "2-arg → callback=Some");
        } else {
            panic!("esperado Content::StateDisplay com callback");
        }
    }

    #[test]
    fn p240_native_state_display_arg_tipo_errado_rejeita() {
        null_ctx!(ctx);
        // 1-arg key não-string.
        let r1 = native_state_display(
            &mut ctx,
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r1.is_err(), "key não-string deve retornar Err");
        // 2-arg segundo arg não-Func.
        let r2 = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r2.is_err(), "callback não-Func deve retornar Err");
    }

    #[test]
    fn p240_native_state_display_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 =
            native_state_display(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 3 args.
        let r3 = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r3.is_err(), "3 args deve retornar Err");
    }

    // ── Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto
    //     P240 M7+1) — counter_display render-mediated walk-time real via
    //     apply_counter_displays paralelo apply_state_displays ──

    #[test]
    fn p241_native_counter_display_sem_callback_constroi_variant() {
        null_ctx!(ctx);
        let r = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::CounterDisplayCallback(e)) = r {
            assert_eq!(e.key, "heading");
            assert!(e.callback.is_none(), "1-arg → callback=None");
        } else {
            panic!("esperado Content::CounterDisplayCallback");
        }
    }

    #[test]
    fn p241_native_counter_display_com_callback_constroi_some() {
        use crate::entities::func::Func;
        null_ctx!(ctx);
        let identity_fn = Func::native("identity", |_, args, _, _| {
            Ok(args.items.first().cloned().unwrap_or(Value::None))
        });
        let r = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("figure".into()), Value::Func(identity_fn)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::CounterDisplayCallback(e)) = r {
            assert_eq!(e.key, "figure");
            assert!(e.callback.is_some(), "2-arg → callback=Some");
        } else {
            panic!("esperado Content::CounterDisplayCallback com callback");
        }
    }

    #[test]
    fn p241_native_counter_display_arg_tipo_errado_rejeita() {
        null_ctx!(ctx);
        // 1-arg key não-string.
        let r1 = native_counter_display(
            &mut ctx,
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r1.is_err(), "key não-string deve retornar Err");
        // 2-arg segundo arg não-Func.
        let r2 = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r2.is_err(), "callback não-Func deve retornar Err");
    }

    #[test]
    fn p241_native_counter_display_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 =
            native_counter_display(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 3 args.
        let r3 = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r3.is_err(), "3 args deve retornar Err");
    }

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
    // Tests simétricos table_header ↔ table_footer (paridade interna
    // absoluta excepto naming).

    #[test]
    fn native_table_header_default_repeat_true() {
        // P157C ADR-0064 Caso D: default vanilla repeat=true.
        null_ctx!(ctx);
        let r = native_table_header(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::TableHeader(e)) = r {
            assert_eq!(e.body.plain_text(), "body");
            assert!(e.repeat, "default vanilla repeat=true (Caso D)");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn native_table_footer_default_repeat_true() {
        // Par simétrico — paridade absoluta com header.
        null_ctx!(ctx);
        let r = native_table_footer(
            &mut ctx,
            &p(vec![Value::Content(Content::text("body"))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::TableFooter(e)) = r {
            assert_eq!(e.body.plain_text(), "body");
            assert!(e.repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn native_table_header_repeat_false_explicito() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r =
            native_table_header(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableHeader(e)) = r {
            assert!(!e.repeat);
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn native_table_footer_repeat_false_explicito() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r =
            native_table_footer(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::TableFooter(e)) = r {
            assert!(!e.repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn native_table_header_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_header(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "table_header() sem body deve retornar Err");
    }

    #[test]
    fn native_table_footer_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_footer(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "table_footer() sem body deve retornar Err");
    }

    #[test]
    fn native_table_header_repeat_int_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Int(1));
        let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "repeat=Int em table_header() deve retornar Err");
    }

    #[test]
    fn native_table_footer_repeat_int_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Int(1));
        let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "repeat=Int em table_footer() deve retornar Err");
    }

    #[test]
    fn native_table_header_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("level".into(), Value::Int(2));
        let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "level (scope-out) em table_header() deve retornar Err");
    }

    #[test]
    fn native_table_footer_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido em table_footer() deve retornar Err");
    }

    // ── Passo 159A — Bibliography + Cite par acoplado ────────────────────

    fn make_bib_dict(key: &str, author: &str, title: &str, year: i64) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(), Value::Str(title.into()));
        d.insert("year".into(), Value::Int(year));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_default_vazia() {
        // P479: bibliography() sem args produz título default Content::heading(1, "Bibliography").
        null_ctx!(ctx);
        let r = native_bibliography(&mut ctx, &p(vec![]), &null_world(), test_file_id())
            .unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert!(e.entries.is_empty());
            // P479: título default é heading nível 1 "Bibliography" (não None).
            assert!(e.title.is_some(), "P479: título default deve ser Some");
            let title_text = e.title.as_ref().unwrap().plain_text();
            assert_eq!(title_text, "Bibliography", "P479: texto do título default");
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_com_entries_posicional() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![make_bib_dict(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
        )]);
        let args = p(vec![entries_arr]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert_eq!(e.entries.len(), 1);
            assert_eq!(e.entries[0].key, "smith2024");
            assert_eq!(e.entries[0].author, "Smith, J.");
            assert_eq!(e.entries[0].year, 2024);
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_com_title() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Array(vec![])]);
        args.named.insert("title".into(), Value::Str("Referências".into()));
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert_eq!(
                e.title.as_ref().map(|t| t.plain_text()).as_deref(),
                Some("Referências")
            );
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    // ── P479 — testes de título padrão de bibliography ────────────────────────
    #[test]
    fn p479_native_bibliography_default_titulo_e_heading() {
        null_ctx!(ctx);
        let r = native_bibliography(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let title = e.title.as_ref().expect("P479: default title deve ser Some");
            assert!(
                matches!(title, Content::Heading(_)),
                "P479: default title deve ser Content::Heading, obtido {:?}", title
            );
            assert_eq!(title.plain_text(), "Bibliography");
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn p479_native_bibliography_title_none_suprime_titulo() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("title".into(), Value::None);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert!(e.title.is_none(), "P479: title: none deve suprimir o título");
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn p479_native_bibliography_title_explicito_preservado() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("title".into(), Value::Str("Referências".into()));
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let title = e.title.as_ref().expect("P479: título explícito deve ser Some");
            assert_eq!(title.plain_text(), "Referências");
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_dict_sem_field_obrigatorio_rejeitado() {
        // P159A: dict sem 'year' (ou outro field obrigatório) → erro hard.
        null_ctx!(ctx);
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(), Value::Str("T".into()));
        // year ausente intencionalmente.
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "dict sem 'year' deve retornar Err");
    }

    #[test]
    fn native_bibliography_year_negativo_rejeitado() {
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![make_bib_dict("k", "A", "T", -5)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "year negativo deve retornar Err");
    }

    #[test]
    fn native_bibliography_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Array(vec![])]);
        args.named.insert("full".into(), Value::Bool(true));
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "full (scope-out per ADR-0054 graded) deve retornar Err");
    }

    #[test]
    fn native_bibliography_style_ieee_aceite() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Array(vec![])]);
        args.named.insert("style".into(), Value::Str("ieee".into()));
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(r, Value::Content(Content::Bibliography(_))));
        // P429: style resolvido é registado no EvalContext, não no elemento.
        assert_eq!(ctx.bibliography_styles.len(), 1);
        let style = ctx.bibliography_styles.values().next().unwrap();
        assert!(style.info.title.value.contains("IEEE"));
    }

    #[test]
    fn native_bibliography_path_bib_carrega_entries() {
        null_ctx!(ctx);
        let mut world = null_world();
        world.files.insert(
            "refs.bib".into(),
            std::sync::Arc::new(
                b"@article{smith2024, author = {Smith, J.}, title = {On Crystal Math}, year = {2024}}"
                    .to_vec(),
            ),
        );
        let args = p(vec![Value::Str("refs.bib".into())]);
        let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert_eq!(e.entries.len(), 1);
            assert_eq!(e.entries[0].key, "smith2024");
            assert_eq!(e.path.as_deref(), Some("refs.bib"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_path_yaml_carrega_entries() {
        null_ctx!(ctx);
        let mut world = null_world();
        world.files.insert(
            "refs.yaml".into(),
            std::sync::Arc::new(
                b"smith2024:\n  type: article\n  author: Smith, J.\n  title: On Crystal Math\n  year: 2024"
                    .to_vec(),
            ),
        );
        let args = p(vec![Value::Str("refs.yaml".into())]);
        let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert_eq!(e.entries.len(), 1);
            assert_eq!(e.entries[0].key, "smith2024");
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_path_nao_encontrado_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("inexistente.bib".into())]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "path inexistente deve retornar erro");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("não encontrado") || msg.contains("not found"), "{msg}");
    }

    #[test]
    fn native_bibliography_path_e_style_preservam() {
        null_ctx!(ctx);
        let mut world = null_world();
        world.files.insert(
            "refs.bib".into(),
            std::sync::Arc::new(
                b"@article{smith2024, author = {Smith, J.}, title = {T}, year = {2024}}"
                    .to_vec(),
            ),
        );
        let mut args = p(vec![Value::Str("refs.bib".into())]);
        args.named.insert("style".into(), Value::Str("ieee".into()));
        args.named.insert("locale".into(), Value::Str("en-US".into()));
        let r = native_bibliography(&mut ctx, &args, &world, test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            assert_eq!(e.path.as_deref(), Some("refs.bib"));
            assert_eq!(e.style.as_deref(), Some("ieee"));
            assert_eq!(e.locale.as_deref(), Some("en-US"));
            // P429: style resolvido viaja no EvalContext (e depois no Module/BibStore),
            // indexado pela chave determinística do elemento.
            let key = e.style_key();
            assert!(ctx.bibliography_styles.contains_key(&key));
            let style = ctx.bibliography_styles.get(&key).unwrap();
            assert!(style.info.title.value.contains("IEEE"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_cite_so_key_posicional() {
        null_ctx!(ctx);
        let r = native_cite(
            &mut ctx,
            &p(vec![Value::Str("smith2024".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(e.key, "smith2024");
            assert!(e.supplement.is_none());
            assert!(e.form.is_none());
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_com_supplement() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("smith2024".into())]);
        args.named.insert("supplement".into(), Value::Str("p. 42".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(
                e.supplement.as_ref().map(|s| s.plain_text()).as_deref(),
                Some("p. 42")
            );
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_sem_key_rejeitado() {
        null_ctx!(ctx);
        let r = native_cite(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err(), "cite() sem key deve retornar Err");
    }

    #[test]
    fn native_cite_key_vazia_rejeitada() {
        null_ctx!(ctx);
        let r = native_cite(
            &mut ctx,
            &p(vec![Value::Str("".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "cite() key vazia deve retornar Err");
    }

    #[test]
    fn native_cite_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("style".into(), Value::Str("apa".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "style (scope-out per ADR-0054 graded) deve retornar Err");
    }

    // ── Passo 159C — Cite.form variants ─────────────────────────────────────

    #[test]
    fn native_cite_form_normal_parse() {
        use crate::entities::citation_form::CitationForm;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("normal".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(e.form, Some(CitationForm::Normal));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_prose_parse() {
        use crate::entities::citation_form::CitationForm;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("prose".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(e.form, Some(CitationForm::Prose));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_author_parse() {
        use crate::entities::citation_form::CitationForm;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("author".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(e.form, Some(CitationForm::Author));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_year_parse() {
        use crate::entities::citation_form::CitationForm;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("year".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert_eq!(e.form, Some(CitationForm::Year));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_auto_devolve_none() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Auto);
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Cite(e)) = r {
            assert!(
                e.form.is_none(),
                "form=auto deve produzir None (resolvido a Normal default em layout)"
            );
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("xpto".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "form inválido deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(
            msg.contains("normal")
                && msg.contains("prose")
                && msg.contains("author")
                && msg.contains("year"),
            "mensagem deve listar forms válidas: {}",
            msg
        );
    }

    // ── Passo 159D — BibEntry fields opcionais ──────────────────────────────

    fn make_bib_dict_full(
        key: &str,
        author: &str,
        title: &str,
        year: i64,
        volume: &str,
        pages: &str,
        journal: &str,
        publisher: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(), Value::Str(title.into()));
        d.insert("year".into(), Value::Int(year));
        d.insert("volume".into(), Value::Str(volume.into()));
        d.insert("pages".into(), Value::Str(pages.into()));
        d.insert("journal".into(), Value::Str(journal.into()));
        d.insert("publisher".into(), Value::Str(publisher.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_fields_opcionais_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![make_bib_dict_full(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
            "12",
            "1-10",
            "Nature Communications",
            "ACM",
        )]);
        let args = p(vec![entries_arr]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert_eq!(e.volume.as_deref(), Some("12"));
            assert_eq!(e.pages.as_deref(), Some("1-10"));
            assert_eq!(e.journal.as_deref(), Some("Nature Communications"));
            assert_eq!(e.publisher.as_deref(), Some("ACM"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_parse_sem_fields_opcionais_regression_p159a() {
        // Regression: dict só com 4 obrigatórios produz entry com fields
        // opcionais None (output P159A inalterado).
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![make_bib_dict(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
        )])]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert!(e.volume.is_none());
            assert!(e.pages.is_none());
            assert!(e.journal.is_none());
            assert!(e.publisher.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_field_opcional_tipo_errado_rejeitado() {
        null_ctx!(ctx);
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(), Value::Str("T".into()));
        d.insert("year".into(), Value::Int(2024));
        // volume com tipo errado (Int em vez de Str).
        d.insert("volume".into(), Value::Int(42));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "volume com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(
            msg.contains("volume"),
            "mensagem deve mencionar field 'volume': {}",
            msg
        );
    }

    // ── Passo 159E — par natural url/doi ────────────────────────────────────

    fn make_bib_dict_with_url_doi(
        key: &str,
        author: &str,
        title: &str,
        year: i64,
        url: &str,
        doi: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(), Value::Str(title.into()));
        d.insert("year".into(), Value::Int(year));
        d.insert("url".into(), Value::Str(url.into()));
        d.insert("doi".into(), Value::Str(doi.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_url_doi_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![make_bib_dict_with_url_doi(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
            "https://example.com/paper",
            "10.1234/abc",
        )]);
        let args = p(vec![entries_arr]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert_eq!(e.url.as_deref(), Some("https://example.com/paper"));
            assert_eq!(e.doi.as_deref(), Some("10.1234/abc"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_parse_sem_url_doi_regression_p159d() {
        // Regression: dict só com 4 obrigatórios + 4 P159D opcionais
        // produz entry com url/doi None (output P159D inalterado).
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![make_bib_dict(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
        )])]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert!(e.url.is_none());
            assert!(e.doi.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_url_doi_tipo_errado_rejeitado() {
        null_ctx!(ctx);
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(), Value::Str("T".into()));
        d.insert("year".into(), Value::Int(2024));
        // doi com tipo errado (Int em vez de Str).
        d.insert("doi".into(), Value::Int(42));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "doi com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("doi"), "mensagem deve mencionar field 'doi': {}", msg);
    }

    // ── Passo 159G — 6 fields restantes comuns hayagriva ────────────────────

    fn make_bib_dict_full_p159g(
        key: &str,
        author: &str,
        title: &str,
        year: i64,
        editor: &str,
        series: &str,
        note: &str,
        isbn: &str,
        location: &str,
        organization: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(), Value::Str(title.into()));
        d.insert("year".into(), Value::Int(year));
        d.insert("editor".into(), Value::Str(editor.into()));
        d.insert("series".into(), Value::Str(series.into()));
        d.insert("note".into(), Value::Str(note.into()));
        d.insert("isbn".into(), Value::Str(isbn.into()));
        d.insert("location".into(), Value::Str(location.into()));
        d.insert("organization".into(), Value::Str(organization.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_p159g_fields_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![make_bib_dict_full_p159g(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
            "Doe, A.",
            "Crystal Studies",
            "See also Smith 2023",
            "978-0-1234-5678-9",
            "New York",
            "ACM",
        )]);
        let args = p(vec![entries_arr]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert_eq!(e.editor.as_deref(), Some("Doe, A."));
            assert_eq!(e.series.as_deref(), Some("Crystal Studies"));
            assert_eq!(e.note.as_deref(), Some("See also Smith 2023"));
            assert_eq!(e.isbn.as_deref(), Some("978-0-1234-5678-9"));
            assert_eq!(e.location.as_deref(), Some("New York"));
            assert_eq!(e.organization.as_deref(), Some("ACM"));
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_parse_subset_p159g_fields() {
        // Parse com só editor + isbn presentes (3 fields P159G ausentes).
        null_ctx!(ctx);
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(), Value::Str("T".into()));
        d.insert("year".into(), Value::Int(2024));
        d.insert("editor".into(), Value::Str("Ed1".into()));
        d.insert("isbn".into(), Value::Str("978-0-1".into()));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert_eq!(e.editor.as_deref(), Some("Ed1"));
            assert_eq!(e.isbn.as_deref(), Some("978-0-1"));
            // Outros P159G permanecem None.
            assert!(e.series.is_none());
            assert!(e.note.is_none());
            assert!(e.location.is_none());
            assert!(e.organization.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_parse_sem_p159g_fields_regression_p159e() {
        // Regression: dict só com 4 obrigatórios + 4 P159D + 2 P159E
        // produz entry com 6 P159G fields None.
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![make_bib_dict(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
        )])]);
        let r =
            native_bibliography(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Bibliography(e)) = r {
            let e = &e.entries[0];
            assert!(e.editor.is_none());
            assert!(e.series.is_none());
            assert!(e.note.is_none());
            assert!(e.isbn.is_none());
            assert!(e.location.is_none());
            assert!(e.organization.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_isbn_tipo_errado_rejeitado() {
        null_ctx!(ctx);
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(), Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(), Value::Str("T".into()));
        d.insert("year".into(), Value::Int(2024));
        // isbn com tipo errado (Int em vez de Str).
        d.insert("isbn".into(), Value::Int(978));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "isbn com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("isbn"), "mensagem deve mencionar field 'isbn': {}", msg);
    }

    // ── P262 (ADR-0087 Gradient Linear-only) ─────────────────────────

    #[test]
    fn p262_gradient_linear_2_color_stops_no_offset() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.stops.len(), 2);
            assert_eq!(l.angle.to_rad(), 0.0);
            assert_eq!(l.stops[0].offset, None);
            assert_eq!(l.stops[1].offset, None);
        } else {
            panic!("esperado Value::Gradient(Linear)");
        }
    }

    #[test]
    fn p262_gradient_linear_explicit_offsets() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Array(vec![
                Value::Color(Color::rgb(255, 0, 0)),
                Value::Ratio(Ratio(0.0)),
            ]),
            Value::Array(vec![
                Value::Color(Color::rgb(0, 0, 255)),
                Value::Ratio(Ratio(1.0)),
            ]),
        ]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.stops[0].offset, Some(Ratio(0.0)));
            assert_eq!(l.stops[1].offset, Some(Ratio(1.0)));
        } else {
            panic!("esperado Value::Gradient(Linear)");
        }
    }

    #[test]
    fn p262_gradient_linear_angle_named() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Angle, Color};
        null_ctx!(ctx);
        let args = pn(
            vec![
                Value::Color(Color::rgb(0, 0, 0)),
                Value::Color(Color::rgb(255, 255, 255)),
            ],
            "angle",
            Value::Angle(Angle::rad(std::f64::consts::FRAC_PI_2)),
        );
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert!((l.angle.to_rad() - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
        } else {
            panic!("esperado Value::Gradient(Linear)");
        }
    }

    #[test]
    fn p262_gradient_linear_zero_stops_erro() {
        null_ctx!(ctx);
        let args = p(vec![]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "stops vazios deve retornar Err");
    }

    #[test]
    fn p262_gradient_linear_offset_out_of_range_erro() {
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![
            Value::Color(Color::rgb(0, 0, 0)),
            Value::Ratio(Ratio(1.5)),
        ])]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "offset 1.5 fora de [0,1] deve retornar Err");
    }

    #[test]
    fn p262_gradient_linear_named_invalido_erro() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = pn(vec![Value::Color(Color::rgb(0, 0, 0))], "unknown", Value::Int(1));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido deve retornar Err");
    }

    #[test]
    fn p262_gradient_linear_value_type_name() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(0, 0, 0)),
            Value::Color(Color::rgb(255, 255, 255)),
        ]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        assert_eq!(r.type_name(), "gradient");
    }

    // ── P264 (ADR-0088 Gradient Radial-only) ────────────────────────

    #[test]
    fn p264_gradient_radial_2_color_stops_defaults() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.stops.len(), 2);
            // Defaults: center (50%, 50%); radius 50%.
            assert_eq!(rad.center.x, Ratio(0.5));
            assert_eq!(rad.center.y, Ratio(0.5));
            assert_eq!(rad.radius, Ratio(0.5));
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p264_gradient_radial_custom_center_radius() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert(
            "center".into(),
            Value::Array(vec![Value::Ratio(Ratio(0.25)), Value::Ratio(Ratio(0.75))]),
        );
        args.named.insert("radius".into(), Value::Ratio(Ratio(0.4)));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.center.x, Ratio(0.25));
            assert_eq!(rad.center.y, Ratio(0.75));
            assert_eq!(rad.radius, Ratio(0.4));
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p264_gradient_radial_zero_stops_erro() {
        null_ctx!(ctx);
        let args = p(vec![]);
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "stops vazios deve retornar Err");
    }

    #[test]
    fn p264_gradient_radial_radius_out_of_range_erro() {
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named.insert("radius".into(), Value::Ratio(Ratio(1.5)));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "radius 1.5 fora de [0,1] deve retornar Err");
    }

    #[test]
    fn p264_gradient_radial_value_type_name() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(0, 0, 0)),
            Value::Color(Color::rgb(255, 255, 255)),
        ]);
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        assert_eq!(r.type_name(), "gradient");
    }

    // ── P269 (ADR-0088 §focal_* revogado parcialmente) — Radial focal_center + focal_radius

    #[test]
    fn p269_stdlib_radial_focal_center_named() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert(
            "focal_center".into(),
            Value::Array(vec![Value::Ratio(Ratio(0.3)), Value::Ratio(Ratio(0.4))]),
        );
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.focal_center.x, Ratio(0.3));
            assert_eq!(rad.focal_center.y, Ratio(0.4));
            assert_eq!(rad.focal_radius, Ratio(0.0), "default focal_radius=0");
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p269_stdlib_radial_focal_radius_named() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("focal_radius".into(), Value::Ratio(Ratio(0.1)));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.focal_radius, Ratio(0.1));
            assert_eq!(rad.focal_center, rad.center, "default focal_center=center");
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p269_stdlib_radial_focal_ambos_named() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert(
            "focal_center".into(),
            Value::Array(vec![Value::Ratio(Ratio(0.25)), Value::Ratio(Ratio(0.35))]),
        );
        args.named.insert("focal_radius".into(), Value::Ratio(Ratio(0.08)));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.focal_center.x, Ratio(0.25));
            assert_eq!(rad.focal_center.y, Ratio(0.35));
            assert_eq!(rad.focal_radius, Ratio(0.08));
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p269_stdlib_radial_focal_defaults_preserva_p264() {
        // Sem named args focal_* → comportamento P264: focal=(center, 0).
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.focal_center, rad.center);
            assert_eq!(rad.focal_radius, Ratio(0.0));
        } else {
            panic!("esperado Value::Gradient(Radial)");
        }
    }

    #[test]
    fn p269_stdlib_radial_focal_radius_maior_radius_erro() {
        // Validação vanilla portada: focal_radius > radius → erro.
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(0, 0, 0)),
            Value::Color(Color::rgb(255, 255, 255)),
        ]);
        args.named.insert("radius".into(), Value::Ratio(Ratio(0.3)));
        args.named.insert("focal_radius".into(), Value::Ratio(Ratio(0.4))); // > radius
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "focal_radius 0.4 > radius 0.3 deve retornar Err");
    }

    // ── P270 (ADR-0091 EM VIGOR — ColorSpace runtime cross-variant) ──

    // Linear

    #[test]
    fn p270_stdlib_linear_space_default_oklab_preserva_p262() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(
                l.space,
                ColorSpace::Oklab,
                "default sem named arg deve ser Oklab"
            );
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p270_stdlib_linear_space_named_hsl() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("space".into(), Value::Str(EcoString::from("hsl")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.space, ColorSpace::Hsl);
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p270_stdlib_linear_space_named_luma() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("space".into(), Value::Str(EcoString::from("luma")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.space, ColorSpace::Luma);
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p270_stdlib_linear_space_named_invalido_erro() {
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named
            .insert("space".into(), Value::Str(EcoString::from("xyz_unknown")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "space inválido deve retornar Err");
    }

    // Radial

    #[test]
    fn p270_stdlib_radial_space_default_oklab_preserva_p264() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.space, ColorSpace::Oklab);
        } else {
            panic!("expected Radial");
        }
    }

    #[test]
    fn p270_stdlib_radial_space_named_oklch() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named
            .insert("space".into(), Value::Str(EcoString::from("oklch")));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.space, ColorSpace::Oklch);
        } else {
            panic!("expected Radial");
        }
    }

    #[test]
    fn p270_stdlib_radial_space_named_cmyk() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("space".into(), Value::Str(EcoString::from("cmyk")));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.space, ColorSpace::Cmyk);
        } else {
            panic!("expected Radial");
        }
    }

    #[test]
    fn p270_stdlib_radial_space_named_invalido_erro() {
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named
            .insert("space".into(), Value::Str(EcoString::from("invalid_space")));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "space inválido deve retornar Err");
    }

    // Conic

    #[test]
    fn p270_stdlib_conic_space_default_oklab_preserva_p267() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.space, ColorSpace::Oklab);
        } else {
            panic!("expected Conic");
        }
    }

    #[test]
    fn p270_stdlib_conic_space_named_hsv() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("space".into(), Value::Str(EcoString::from("hsv")));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.space, ColorSpace::Hsv);
        } else {
            panic!("expected Conic");
        }
    }

    #[test]
    fn p270_stdlib_conic_space_named_srgb() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, ColorSpace};
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert("space".into(), Value::Str(EcoString::from("srgb")));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.space, ColorSpace::Srgb);
        } else {
            panic!("expected Conic");
        }
    }

    #[test]
    fn p270_stdlib_conic_space_named_invalido_erro() {
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named.insert("space".into(), Value::Str(EcoString::from("???")));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "space inválido deve retornar Err");
    }

    // ── P273 (ADR-0091 §"Anotação cumulativa P273") — relative: RelativeTo cross-variant ──

    #[test]
    fn p273_stdlib_linear_relative_self() {
        use crate::entities::gradient::{Gradient, RelativeTo};
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(255, 0, 0))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("self")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.relative, Some(RelativeTo::Self_));
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p273_stdlib_linear_relative_parent() {
        use crate::entities::gradient::{Gradient, RelativeTo};
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 255, 0))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("parent")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.relative, Some(RelativeTo::Parent));
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p273_stdlib_linear_relative_auto_default_none() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 255))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("auto")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.relative, None, "auto explícito = None (Auto sentinel)");
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p273_stdlib_linear_relative_default_none_preserva_p270() {
        // Sem named arg `relative` → None (preserve P270.x behavior).
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = p(vec![Value::Color(Color::rgb(255, 255, 0))]);
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Linear(l)) = r {
            assert_eq!(l.relative, None);
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p273_stdlib_radial_relative_parent() {
        use crate::entities::gradient::{Gradient, RelativeTo};
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(255, 0, 255))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("parent")));
        let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Radial(rad)) = r {
            assert_eq!(rad.relative, Some(RelativeTo::Parent));
        } else {
            panic!("expected Radial");
        }
    }

    #[test]
    fn p273_stdlib_conic_relative_self() {
        use crate::entities::gradient::{Gradient, RelativeTo};
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 255, 255))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("self")));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.relative, Some(RelativeTo::Self_));
        } else {
            panic!("expected Conic");
        }
    }

    #[test]
    fn p273_stdlib_relative_invalido_erro() {
        use crate::entities::layout_types::Color;
        use ecow::EcoString;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named
            .insert("relative".into(), Value::Str(EcoString::from("inválido")));
        let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "relative inválido deve retornar Err");
    }

    // ── P267 (ADR-0089 Gradient Conic-only) ────────────────────────

    #[test]
    fn p267_gradient_conic_2_color_stops_defaults() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Color, Ratio};
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.stops.len(), 2);
            assert_eq!(c.center.x, Ratio(0.5));
            assert_eq!(c.center.y, Ratio(0.5));
            assert_eq!(c.angle.to_rad(), 0.0);
        } else {
            panic!("esperado Value::Gradient(Conic)");
        }
    }

    #[test]
    fn p267_gradient_conic_custom_center_angle() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::{Angle, Color, Ratio};
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Color(Color::rgb(0, 0, 255)),
        ]);
        args.named.insert(
            "center".into(),
            Value::Array(vec![Value::Ratio(Ratio(0.3)), Value::Ratio(Ratio(0.7))]),
        );
        args.named.insert("angle".into(), Value::Angle(Angle::deg(45.0)));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        if let Value::Gradient(Gradient::Conic(c)) = r {
            assert_eq!(c.center.x, Ratio(0.3));
            assert_eq!(c.center.y, Ratio(0.7));
            assert!((c.angle.to_rad() - std::f64::consts::FRAC_PI_4).abs() < 1e-9);
        } else {
            panic!("esperado Value::Gradient(Conic)");
        }
    }

    #[test]
    fn p267_gradient_conic_zero_stops_erro() {
        null_ctx!(ctx);
        let args = p(vec![]);
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "stops vazios deve retornar Err");
    }

    #[test]
    fn p267_gradient_conic_named_invalido_erro() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
        args.named.insert("unknown".into(), Value::Int(1));
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido deve retornar Err");
    }

    #[test]
    fn p267_gradient_conic_value_type_name() {
        use crate::entities::layout_types::Color;
        null_ctx!(ctx);
        let args = p(vec![
            Value::Color(Color::rgb(0, 0, 0)),
            Value::Color(Color::rgb(255, 255, 255)),
        ]);
        let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap();
        assert_eq!(r.type_name(), "gradient");
    }

    // ── Passo 284 — text decoration (underline / strike / overline) ─────

    #[test]
    fn p284_native_underline_envolve_body_sem_named() {
        null_ctx!(ctx);
        let args = p(vec![Value::Content(Content::text("hello"))]);
        let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Underline(e)) = r {
            assert_eq!(e.body.plain_text(), "hello");
            assert!(
                e.stroke.is_none() && e.offset.is_none() && e.extent.is_none(),
                "sem named: cosméticos preservados em None"
            );
        } else {
            panic!("esperado Value::Content(Content::Underline {{ .. }})");
        }
    }

    #[test]
    fn p284_native_strike_e_overline_idem_underline() {
        null_ctx!(ctx);
        let args = p(vec![Value::Content(Content::text("x"))]);
        let s = native_strike(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        let o = native_overline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert!(matches!(s, Value::Content(Content::Strike(_))));
        assert!(matches!(o, Value::Content(Content::Overline(_))));
    }

    #[test]
    fn p284_native_underline_aceita_string_como_body() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("important".into())]);
        let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        match r {
            Value::Content(Content::Underline(e)) => {
                assert_eq!(e.body.plain_text(), "important")
            }
            other => panic!("esperado Underline, obtido {other:?}"),
        }
    }

    #[test]
    fn p284_native_underline_named_stroke_offset_extent() {
        use crate::entities::layout_types::{Color, Length};
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("y"))]);
        args.named
            .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        args.named.insert("offset".into(), Value::Length(Length::pt(2.5)));
        args.named.insert("extent".into(), Value::Length(Length::pt(-1.0)));
        let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Underline(e)) = r {
            assert_eq!(e.stroke, Some(Color::rgb(255, 0, 0)));
            assert_eq!(e.offset, Some(Length::pt(2.5)));
            assert_eq!(e.extent, Some(Length::pt(-1.0)));
        } else {
            panic!("esperado Underline com cosméticos");
        }
    }

    #[test]
    fn p284_native_decoration_sem_body_retorna_err() {
        null_ctx!(ctx);
        let args = p(vec![]);
        assert!(native_underline(&mut ctx, &args, &null_world(), test_file_id()).is_err());
        assert!(native_strike(&mut ctx, &args, &null_world(), test_file_id()).is_err());
        assert!(native_overline(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn p284_native_decoration_scope_out_evade_e_background_erro_explicito() {
        // Mensagem de erro deve referenciar a decisão graded (não erro
        // genérico "argumento inesperado"). Cobre diagnóstico §A.1.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("evade".into(), Value::Bool(true));
        let err =
            native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
        let msg = format!("{:?}", err);
        assert!(
            msg.contains("evade") && msg.contains("ADR-0054"),
            "mensagem deve referir 'evade' e ADR-0054 graded: {msg}"
        );
        let mut args2 = p(vec![Value::Content(Content::text("x"))]);
        args2.named.insert("background".into(), Value::Bool(true));
        let err2 =
            native_overline(&mut ctx, &args2, &null_world(), test_file_id()).unwrap_err();
        assert!(format!("{:?}", err2).contains("background"));
    }

    // ── Passo 287 — native_smartquote ─────────────────────────────────

    #[test]
    fn p287_native_smartquote_sem_args_emite_variant_double_true() {
        use super::native_smartquote;
        null_ctx!(ctx);
        let r = native_smartquote(&mut ctx, &p(vec![]), &null_world(), test_file_id())
            .unwrap();
        assert_eq!(
            r,
            Value::Content(Content::smartquote(true)),
            "smartquote() sem args → SmartQuote {{ double: true }} (vanilla default)"
        );
    }

    #[test]
    fn p287_native_smartquote_double_false_emite_variant_single() {
        use super::native_smartquote;
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("double".into(), Value::Bool(false));
        let r =
            native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert_eq!(r, Value::Content(Content::smartquote(false)));
    }

    #[test]
    fn p287_native_smartquote_enabled_false_emite_text_ascii_directo() {
        // Paridade vanilla `set smartquote(enabled: false)` — ASCII literal,
        // não passa pelo variant SmartQuote (consumer Layouter ignora).
        use super::native_smartquote;
        null_ctx!(ctx);
        // enabled=false + double=true → Text("\"").
        let mut args = p(vec![]);
        args.named.insert("enabled".into(), Value::Bool(false));
        let r =
            native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        assert_eq!(r, Value::Content(Content::text("\"")));
        // enabled=false + double=false → Text("'").
        let mut args2 = p(vec![]);
        args2.named.insert("enabled".into(), Value::Bool(false));
        args2.named.insert("double".into(), Value::Bool(false));
        let r2 =
            native_smartquote(&mut ctx, &args2, &null_world(), test_file_id()).unwrap();
        assert_eq!(r2, Value::Content(Content::text("'")));
    }

    #[test]
    fn p287_native_smartquote_alternative_rejeitado_com_erro_educacional() {
        // Scope-out per ADR-0054 graded — erro menciona ADR.
        use super::native_smartquote;
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("alternative".into(), Value::Bool(true));
        let err = native_smartquote(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap_err();
        let msg = format!("{:?}", err);
        assert!(
            msg.contains("alternative") && msg.contains("ADR-0054"),
            "erro deve referir 'alternative' e ADR-0054: {msg}"
        );
    }

    #[test]
    fn p287_native_smartquote_quotes_custom_scope_out_erro_explicito() {
        // `quotes` (custom string/array/dict) é scope-out per §A.1.3.
        use super::native_smartquote;
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("quotes".into(), Value::Str("()".into()));
        let err = native_smartquote(&mut ctx, &args, &null_world(), test_file_id())
            .unwrap_err();
        assert!(format!("{:?}", err).contains("quotes"));
    }

    #[test]
    fn p287_native_smartquote_arg_posicional_retorna_err() {
        use super::native_smartquote;
        null_ctx!(ctx);
        let r = native_smartquote(
            &mut ctx,
            &p(vec![Value::Bool(true)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "args posicionais rejeitados (vanilla usa só named)");
    }

    // ── Passo 408 — `smallcaps(body)` ──────────────────────────────────

    #[test]
    fn p408_native_smallcaps_envolve_content() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let args = p(vec![Value::Content(Content::text("Hello"))]);
        let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        match r {
            Value::Content(Content::SmallCaps { body }) => {
                assert_eq!(body.plain_text(), "Hello");
            }
            other => panic!("esperado Content::SmallCaps, obtido {other:?}"),
        }
    }

    #[test]
    fn p408_native_smallcaps_aceita_string() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let args = p(vec![Value::Str("World".into())]);
        let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        match r {
            Value::Content(Content::SmallCaps { body }) => {
                assert_eq!(body.plain_text(), "World");
            }
            other => panic!("esperado Content::SmallCaps, obtido {other:?}"),
        }
    }

    #[test]
    fn p408_native_smallcaps_sem_body_retorna_err() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let args = p(vec![]);
        assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn p408_native_smallcaps_rejeita_named_arg() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("stroke".into(), Value::Int(1));
        assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn p408_native_smallcaps_rejeita_tipo_errado() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let args = p(vec![Value::Int(42)]);
        assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn p408_native_smallcaps_body_acessivel_via_get_field() {
        use super::native_smallcaps;
        null_ctx!(ctx);
        let args = p(vec![Value::Content(Content::text("abc"))]);
        let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(c) = r {
            assert_eq!(
                c.get_field("body"),
                Some(Value::Content(Content::text("abc"))),
                "it.body deve devolver o body original"
            );
        } else {
            panic!("esperado Value::Content");
        }
    }

    // ── Passo 391 — `lorem(n)` ─────────────────────────────────────────

    #[test]
    fn lorem_0_retorna_string_vazia() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn lorem_1_retorna_uma_palavra_sem_espaco() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Str(s) = r {
            assert_eq!(s.split_whitespace().count(), 1);
            assert!(!s.ends_with(' '));
        } else {
            panic!("esperava Value::Str");
        }
    }

    #[test]
    fn lorem_5_retorna_5_palavras() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(5)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Str(s) = r {
            assert_eq!(s.split_whitespace().count(), 5);
        } else {
            panic!("esperava Value::Str");
        }
    }

    #[test]
    fn lorem_100_retorna_100_palavras() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(100)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Str(s) = r {
            assert_eq!(s.split_whitespace().count(), 100);
        } else {
            panic!("esperava Value::Str");
        }
    }

    #[test]
    fn lorem_negativo_retorna_erro() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(-1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "lorem(-1) deve retornar erro");
    }

    #[test]
    fn lorem_rejeita_named_arg() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Int(5)]);
        args.named.insert("foo".into(), Value::Int(1));
        assert!(native_lorem(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn lorem_rejeita_tipo_errado() {
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Str("x".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err(), "lorem(string) deve retornar erro");
    }

    // ── Passo 295 — `footnote()` cluster Fase 1 (marker only) ──────────
    //
    // HE Fase 1: variant Content::footnote(body) + stdlib
    // native_footnote + Layouter walker counter (marker [N]).
    // Body armazenado mas não renderizado (P295.1/P295.2 sub-passos).

    #[test]
    fn p295_native_footnote_body_posicional() {
        use super::native_footnote;
        null_ctx!(ctx);
        let body = Value::Content(Content::text("nota"));
        let r = native_footnote(&mut ctx, &p(vec![body]), &null_world(), test_file_id())
            .unwrap();
        if let Value::Content(Content::Footnote(e)) = r {
            assert_eq!(e.body.plain_text(), "nota");
        } else {
            panic!("esperava Content::Footnote");
        }
    }

    #[test]
    fn p295_native_footnote_body_string_converte_para_text() {
        use super::native_footnote;
        null_ctx!(ctx);
        let r = native_footnote(
            &mut ctx,
            &p(vec![Value::Str("texto".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::Footnote(e)) = r {
            assert_eq!(e.body.plain_text(), "texto");
        } else {
            panic!("esperava Content::Footnote");
        }
    }

    #[test]
    fn p295_native_footnote_sem_body_retorna_err() {
        use super::native_footnote;
        null_ctx!(ctx);
        let r = native_footnote(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("posicional"));
    }

    #[test]
    fn p295_native_footnote_numbering_aceite_p502() {
        use super::native_footnote;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("a".into())]);
        args.named.insert("numbering".into(), Value::Str("*".into()));
        let r = native_footnote(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "P502: numbering deve ser aceite em footnote()");
        if let Value::Content(Content::Footnote(e)) = r.unwrap() {
            assert_eq!(e.numbering.as_deref(), Some("*"));
        } else {
            panic!("esperado Content::Footnote");
        }
    }

    #[test]
    fn p295_native_footnote_named_arg_desconhecido_rejeitado() {
        use super::native_footnote;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("a".into())]);
        args.named.insert("foo".into(), Value::Str("bar".into()));
        let r = native_footnote(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named arg desconhecido deve ser rejeitado");
    }

    #[test]
    fn p295_native_footnote_body_content_complexo_preservado() {
        use super::native_footnote;
        // Body é Sequence de Content; preserved estructuralmente.
        null_ctx!(ctx);
        let body = Value::Content(Content::sequence(vec![
            Content::text("hello "),
            Content::text("world"),
        ]));
        let r = native_footnote(&mut ctx, &p(vec![body]), &null_world(), test_file_id())
            .unwrap();
        if let Value::Content(Content::Footnote(e)) = r {
            assert_eq!(e.body.plain_text(), "hello world");
        }
    }

    #[test]
    fn p295_footnote_partial_eq_por_body() {
        let a = Content::footnote(Content::text("x"));
        let b = Content::footnote(Content::text("x"));
        let c = Content::footnote(Content::text("y"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn p295_footnote_is_empty_sempre_false() {
        // Marker [N] é sempre observable; footnote nunca vazia.
        let f_empty_body = Content::footnote(Content::Empty);
        assert!(
            !f_empty_body.is_empty(),
            "footnote nunca é is_empty mesmo com body vazio (marker sempre observable)"
        );
    }

    // ── Passo 296 — accent + cancel math (HIV + (a) minimal) ──────────
    //
    // A.0.0 N=4 refuta classificação Tabela A.4 (parcial → ausente
    // → implementado). Padrão "variant rico" N=4 preservado.

    #[test]
    fn p296_native_accent_base_e_accent_posicionais() {
        use super::native_accent;
        null_ctx!(ctx);
        let r = native_accent(
            &mut ctx,
            &p(vec![
                Value::Content(Content::MathIdent("a".into())),
                Value::Content(Content::MathText("^".into())),
            ]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathAccent(e)) = r {
            assert_eq!(e.base.plain_text(), "a");
            assert_eq!(e.accent.plain_text(), "^");
        } else {
            panic!("esperava Content::MathAccent");
        }
    }

    #[test]
    fn p296_native_accent_strings_convertidas_para_text() {
        use super::native_accent;
        null_ctx!(ctx);
        let r = native_accent(
            &mut ctx,
            &p(vec![Value::Str("x".into()), Value::Str("~".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathAccent(e)) = r {
            assert_eq!(e.base.plain_text(), "x");
            assert_eq!(e.accent.plain_text(), "~");
        }
    }

    #[test]
    fn p296_native_accent_sem_base_retorna_err() {
        use super::native_accent;
        null_ctx!(ctx);
        let r = native_accent(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("base"));
    }

    #[test]
    fn p296_native_accent_sem_accent_retorna_err() {
        use super::native_accent;
        null_ctx!(ctx);
        let r = native_accent(
            &mut ctx,
            &p(vec![Value::Str("a".into())]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("accent"));
    }

    #[test]
    fn p296_native_accent_named_arg_rejeitado() {
        use super::native_accent;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("a".into()), Value::Str("^".into())]);
        args.named.insert("size".into(), Value::Float(0.5));
        let r = native_accent(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "size/dotless cosméticos scope-out P296");
    }

    #[test]
    fn p296_native_cancel_body_posicional() {
        use super::native_cancel;
        null_ctx!(ctx);
        let r = native_cancel(
            &mut ctx,
            &p(vec![Value::Content(Content::MathIdent("x".into()))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathCancel(e)) = r {
            assert_eq!(e.body.plain_text(), "x");
        } else {
            panic!("esperava Content::MathCancel");
        }
    }

    #[test]
    fn p296_native_cancel_body_string_convertido() {
        use super::native_cancel;
        null_ctx!(ctx);
        let r = native_cancel(
            &mut ctx,
            &p(vec![Value::Str("y".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathCancel(e)) = r {
            assert_eq!(e.body.plain_text(), "y");
        }
    }

    #[test]
    fn p296_native_cancel_sem_body_retorna_err() {
        use super::native_cancel;
        null_ctx!(ctx);
        let r = native_cancel(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("body"));
    }

    #[test]
    fn p296_native_cancel_named_arg_rejeitado() {
        use super::native_cancel;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("x".into())]);
        args.named.insert("inverted".into(), Value::Bool(true));
        let r = native_cancel(&mut ctx, &args, &null_world(), test_file_id());
        assert!(
            r.is_err(),
            "inverted/cross/length/angle/stroke cosméticos scope-out P296"
        );
    }

    #[test]
    fn p296_math_accent_partial_eq_e_is_empty() {
        let a = Content::math_accent(
            Content::MathIdent("a".into()),
            Content::MathText("^".into()),
        );
        let b = a.clone();
        assert_eq!(a, b);
        // is_empty fallback é false (math structural sempre observable).
        assert!(!a.is_empty());
    }

    #[test]
    fn p296_math_cancel_partial_eq() {
        let a = Content::math_cancel(Content::MathIdent("x".into()));
        let b = Content::math_cancel(Content::MathIdent("x".into()));
        let c = Content::math_cancel(Content::MathIdent("y".into()));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 297 — `underover()` math (P296.1) ─────────────────────────
    //
    // HV'.a + (b) Option fields: A.0.0 N=5 refutou spec (vanilla
    // fragmenta em 12 elementos; cristalino agrega per ADR-0054 graded).
    // Primeira qualificação genuína "variant rico" N=5 — adiada.

    #[test]
    fn p297_native_underover_so_base_posicional() {
        use super::native_underover;
        null_ctx!(ctx);
        let r = native_underover(
            &mut ctx,
            &p(vec![Value::Content(Content::MathIdent("x".into()))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathUnderover(e)) = r {
            assert_eq!(e.base.plain_text(), "x");
            assert!(e.under.is_none(), "under None se não fornecido");
            assert!(e.over.is_none(), "over None se não fornecido");
        } else {
            panic!("esperava Content::MathUnderover");
        }
    }

    #[test]
    fn p297_native_underover_com_under_named() {
        use super::native_underover;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("base".into())]);
        args.named.insert("under".into(), Value::Str("u".into()));
        let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::MathUnderover(e)) = r {
            assert!(e.under.is_some());
            assert!(e.over.is_none());
            assert_eq!(e.under.as_ref().unwrap().plain_text(), "u");
        }
    }

    #[test]
    fn p297_native_underover_com_over_named() {
        use super::native_underover;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("base".into())]);
        args.named.insert("over".into(), Value::Str("o".into()));
        let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::MathUnderover(e)) = r {
            assert!(e.under.is_none());
            assert!(e.over.is_some());
            assert_eq!(e.over.as_ref().unwrap().plain_text(), "o");
        }
    }

    #[test]
    fn p297_native_underover_com_ambos() {
        use super::native_underover;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("b".into())]);
        args.named.insert("under".into(), Value::Str("u".into()));
        args.named.insert("over".into(), Value::Str("o".into()));
        let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::MathUnderover(e)) = r {
            assert_eq!(e.base.plain_text(), "b");
            assert_eq!(e.under.as_ref().unwrap().plain_text(), "u");
            assert_eq!(e.over.as_ref().unwrap().plain_text(), "o");
        }
    }

    #[test]
    fn p297_native_underover_sem_base_retorna_err() {
        use super::native_underover;
        null_ctx!(ctx);
        let r = native_underover(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("base"));
    }

    #[test]
    fn p297_native_underover_named_arg_invalido_retorna_err() {
        use super::native_underover;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("b".into())]);
        args.named.insert("style".into(), Value::Str("x".into()));
        let r = native_underover(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "named args além de under/over rejeitados");
    }

    #[test]
    fn p297_math_underover_plain_text_ordem_visual() {
        // Ordem visual: over + base + under.
        let u = Content::math_underover(
            Content::text("BASE"),
            Some(Content::text("U")),
            Some(Content::text("O")),
        );
        assert_eq!(u.plain_text(), "OBASEU");
    }

    #[test]
    fn p297_math_underover_partial_eq_structural() {
        let a =
            Content::math_underover(Content::text("x"), Some(Content::text("u")), None);
        let b = a.clone();
        assert_eq!(a, b);
        let c = Content::math_underover(Content::text("x"), None, None); // diferente em under
        assert_ne!(a, c);
    }

    #[test]
    fn p297_math_underover_ambos_none_equivale_so_base() {
        // Caso degenerate: ambos None → plain_text == base.
        let u = Content::math_underover(Content::text("x"), None, None);
        assert_eq!(u.plain_text(), "x");
    }

    // ── Passo 298 — `op()` math (P296.2 fecho cluster math 4/4) ─────────
    //
    // HV'' adaptado: cristalino tinha heurística limits hardcoded;
    // P298 estende para suportar `MathOp { limits: true }` user-facing.
    // Cross-variant interaction: MathOp.limits afecta MathAttach layout.

    #[test]
    fn p298_native_op_text_posicional_default_limits_false() {
        use super::native_op;
        null_ctx!(ctx);
        let r = native_op(
            &mut ctx,
            &p(vec![Value::Str("lim".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathOp(e)) = r {
            assert_eq!(e.text.plain_text(), "lim");
            assert!(!e.limits, "limits default false");
        } else {
            panic!("esperava Content::MathOp");
        }
    }

    #[test]
    fn p298_native_op_com_limits_true_named() {
        use super::native_op;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("lim".into())]);
        args.named.insert("limits".into(), Value::Bool(true));
        let r = native_op(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::MathOp(e)) = r {
            assert!(e.limits, "limits=true respeitado");
        }
    }

    #[test]
    fn p298_native_op_text_content_preservado() {
        use super::native_op;
        null_ctx!(ctx);
        let r = native_op(
            &mut ctx,
            &p(vec![Value::Content(Content::MathIdent("Σ".into()))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        if let Value::Content(Content::MathOp(e)) = r {
            // Content posicional preservado estructuralmente.
            assert_eq!(e.text.plain_text(), "Σ");
        }
    }

    #[test]
    fn p298_native_op_sem_text_retorna_err() {
        use super::native_op;
        null_ctx!(ctx);
        let r = native_op(&mut ctx, &p(vec![]), &null_world(), test_file_id());
        assert!(r.is_err());
        assert!(format!("{:?}", r).contains("text"));
    }

    #[test]
    fn p298_native_op_named_invalido_retorna_err() {
        use super::native_op;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("lim".into())]);
        args.named.insert("style".into(), Value::Str("italic".into()));
        let r = native_op(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "só 'limits' permitido");
    }

    #[test]
    fn p298_native_op_limits_nao_bool_retorna_err() {
        use super::native_op;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("lim".into())]);
        args.named.insert("limits".into(), Value::Int(1));
        let r = native_op(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "limits espera bool");
    }

    #[test]
    fn p298_math_op_partial_eq_structural_com_limits() {
        let a = Content::math_op(Content::text("lim"), true);
        let b = a.clone();
        assert_eq!(a, b);
        // limits diferente → desigual.
        let c = Content::math_op(Content::text("lim"), false);
        assert_ne!(a, c);
    }

    #[test]
    fn p298_math_op_plain_text_so_text_sem_limits() {
        let o = Content::math_op(Content::text("lim"), true);
        // limits é discriminador layout — plain_text só retorna text.
        assert_eq!(o.plain_text(), "lim");
    }

    // ── Passo 299 — `math` module operadores pré-definidos (P298.X) ─────
    //
    // 1ª aplicação prática Content::MathOp (P298). 42 operadores
    // vanilla registados via SSoT (Content::MathOp { text, limits }).
    // Acesso namespaced: math.sin, math.lim, etc.

    fn lookup_math(name: &str) -> Option<Value> {
        use super::make_math_module;
        if let Value::Dict(d) = make_math_module() {
            d.get(name).cloned()
        } else {
            None
        }
    }

    #[test]
    fn p299_math_module_contem_sin_scripts_style() {
        let v = lookup_math("sin").expect("math.sin deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(e.text.plain_text(), "sin");
            assert!(!e.limits, "sin é scripts-style (limits=false)");
        } else {
            panic!("math.sin deve ser MathOp");
        }
    }

    #[test]
    fn p299_math_module_contem_lim_limits_style() {
        let v = lookup_math("lim").expect("math.lim deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(e.text.plain_text(), "lim");
            assert!(e.limits, "lim é limits-style (limits=true)");
        }
    }

    #[test]
    fn p299_math_module_contem_det_limits_style_novo() {
        // det NÃO estava em is_limit_function pré-P299 hardcoded.
        // P299 acrescenta via registo explícito.
        let v = lookup_math("det").expect("math.det deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(e.text.plain_text(), "det");
            assert!(e.limits, "det é limits-style vanilla");
        }
    }

    #[test]
    fn p299_math_module_contem_liminf_multi_word_text() {
        // liminf vanilla → text "lim inf" (multi-word).
        let v = lookup_math("liminf").expect("math.liminf deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(
                e.text.plain_text(),
                "lim inf",
                "multi-word: name 'liminf' mapeia para text 'lim inf'"
            );
            assert!(e.limits);
        }
    }

    #[test]
    fn p299_math_module_contem_limsup_multi_word_text() {
        let v = lookup_math("limsup").expect("math.limsup deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(e.text.plain_text(), "lim sup");
            assert!(e.limits);
        }
    }

    #[test]
    fn p299_math_module_total_42_operadores() {
        use super::make_math_module;
        if let Value::Dict(d) = make_math_module() {
            assert_eq!(
                d.len(),
                43,
                "P299+: 31 scripts + 11 limits = 42 vanilla + 1 adicionado pós-P299"
            );
        } else {
            panic!("make_math_module deve retornar Value::Dict");
        }
    }

    #[test]
    fn p299_math_module_todos_scripts_style_limits_false() {
        // Amostra de scripts-style: sin/cos/tan/ln/log/exp todos limits=false.
        for name in ["sin", "cos", "tan", "ln", "log", "exp", "arccos"] {
            let v = lookup_math(name).expect(name);
            if let Value::Content(Content::MathOp(e)) = v {
                assert!(!e.limits, "{} deve ser scripts-style", name);
            } else {
                panic!("{} deve ser MathOp", name);
            }
        }
    }

    #[test]
    fn p299_math_module_pr_case_sensitive() {
        // vanilla `Pr` (probabilidade) é case-sensitive — capitalizado.
        let v = lookup_math("Pr").expect("math.Pr deve existir");
        if let Value::Content(Content::MathOp(e)) = v {
            assert_eq!(e.text.plain_text(), "Pr");
            assert!(e.limits, "Pr é limits-style vanilla");
        }
        // 'pr' minúsculo NÃO existe.
        assert!(lookup_math("pr").is_none(), "case sensitive: 'pr' minúsculo não existe");
    }

    #[test]
    fn p299_math_module_nome_inexistente_retorna_none() {
        assert!(lookup_math("foo_bar_qux").is_none());
    }

    #[test]
    fn p299_regressao_math_ident_lim_preservado_pre_p299() {
        // Heurística pré-P299 (is_limit_function) hardcoded preservada.
        // MathIdent("lim") sem usar math.lim continua a funcionar.
        // Este teste NÃO chama math.lim — só verifica que MathIdent existe.
        let m = Content::MathIdent("lim".into());
        // Não panic; pode usar plain_text e estar bem formado.
        assert_eq!(m.plain_text(), "lim");
    }

    // ── Passo 311b.3 — 12 funções math style (paridade 12/12) ─────────────────
    //
    // Caminho I per P311a: cada função wrap o body em Content::MathStyled
    // com kind/bold/italic/cramped específicos. Composição (outer-wins)
    // resolvida em P311b.4.

    use super::math_style::{
        native_bb, native_bold, native_cal, native_frak, native_math_italic, native_mono,
        native_sans, native_scr, native_script, native_serif, native_sscript,
        native_upright,
    };
    use crate::entities::math_style::MathStyleKind;

    fn call_math_style(
        f: fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>,
        items: Vec<Value>,
    ) -> SourceResult<Value> {
        null_ctx!(ctx);
        f(&mut ctx, &p(items), &null_world(), test_file_id())
    }

    fn assert_styled(
        v: SourceResult<Value>,
        exp_kind: Option<MathStyleKind>,
        exp_bold: Option<bool>,
        exp_italic: Option<bool>,
        exp_cramped: Option<bool>,
    ) {
        match v.unwrap() {
            Value::Content(Content::MathStyled(m)) => {
                assert_eq!(m.kind, exp_kind, "kind mismatch");
                assert_eq!(m.bold, exp_bold, "bold mismatch");
                assert_eq!(m.italic, exp_italic, "italic mismatch");
                assert_eq!(m.cramped, exp_cramped, "cramped mismatch");
            }
            other => panic!("esperado Content::MathStyled, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b_bb_wraps_double_struck() {
        let v = call_math_style(
            native_bb,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::DoubleStruck), None, None, None);
    }

    #[test]
    fn p311b_bold_is_orthogonal_flag() {
        let v = call_math_style(
            native_bold,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, None, Some(true), None, None);
    }

    #[test]
    fn p311b_cal_wraps_chancery() {
        let v = call_math_style(
            native_cal,
            vec![Value::Content(Content::MathIdent("L".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Chancery), None, None, None);
    }

    #[test]
    fn p311b_frak_wraps_fraktur() {
        let v = call_math_style(
            native_frak,
            vec![Value::Content(Content::MathIdent("g".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Fraktur), None, None, None);
    }

    #[test]
    fn p311b_italic_is_orthogonal_flag() {
        let v = call_math_style(
            native_math_italic,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, None, None, Some(true), None);
    }

    #[test]
    fn p311b_mono_wraps_monospace() {
        let v = call_math_style(
            native_mono,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Monospace), None, None, None);
    }

    #[test]
    fn p311b_sans_wraps_sans_serif() {
        let v = call_math_style(
            native_sans,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::SansSerif), None, None, None);
    }

    #[test]
    fn p311b_scr_wraps_roundhand() {
        let v = call_math_style(
            native_scr,
            vec![Value::Content(Content::MathIdent("L".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Roundhand), None, None, None);
    }

    #[test]
    fn p311b_script_with_cramped() {
        let v = call_math_style(
            native_script,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Script), None, None, Some(true));
    }

    #[test]
    fn p311b_serif_forces_plain() {
        let v = call_math_style(
            native_serif,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::Plain), None, None, None);
    }

    #[test]
    fn p311b_sscript_with_cramped() {
        let v = call_math_style(
            native_sscript,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, Some(MathStyleKind::SScript), None, None, Some(true));
    }

    #[test]
    fn p311b_upright_suppresses_italic() {
        let v = call_math_style(
            native_upright,
            vec![Value::Content(Content::MathIdent("x".into()))],
        );
        assert_styled(v, None, None, Some(false), None);
    }

    #[test]
    fn p311b_accepts_string_body() {
        let v = call_math_style(native_bb, vec![Value::Str("abc".into())]).unwrap();
        match v {
            Value::Content(Content::MathStyled(m)) => match &m.body {
                Content::Text(s) => assert_eq!(s.as_str(), "abc"),
                other => panic!("esperado Text, obteve {other:?}"),
            },
            other => panic!("esperado MathStyled, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b_empty_args_produces_empty_body() {
        let v = call_math_style(native_bb, vec![]).unwrap();
        match v {
            Value::Content(Content::MathStyled(m)) => {
                assert!(matches!(m.body, Content::Empty));
            }
            other => panic!("esperado MathStyled, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b_two_args_errors() {
        let r = call_math_style(
            native_bb,
            vec![
                Value::Content(Content::MathIdent("x".into())),
                Value::Content(Content::MathIdent("y".into())),
            ],
        );
        assert!(r.is_err(), "esperava erro por arity > 1");
    }

    #[test]
    fn p311b_int_arg_errors() {
        let r = call_math_style(native_bb, vec![Value::Int(1)]);
        assert!(r.is_err(), "esperava erro por tipo incoercível");
    }

    // ── P394 — eval(source) runtime de re-avaliação ────────────────────────────

    /// Helper de teste: avalia `source` através de `native_eval` com um scope/engine mínimos.
    fn run_eval(source: &str) -> SourceResult<Value> {
        with_engine(|engine, world| {
            let mut ctx = EvalContext::new();
            let mut scopes = Scopes::new(None);
            native_eval(
                &mut ctx,
                &p(vec![Value::Str(source.into())]),
                world,
                test_file_id(),
                &mut scopes,
                engine,
            )
        })
    }

    /// Helper de teste: avalia `source` com uma variável `name` pré-definida no scope.
    fn run_eval_with_var(source: &str, name: &str, value: Value) -> SourceResult<Value> {
        with_engine(|engine, world| {
            let mut ctx = EvalContext::new();
            let mut scopes = Scopes::new(None);
            scopes.define(name, value);
            native_eval(
                &mut ctx,
                &p(vec![Value::Str(source.into())]),
                world,
                test_file_id(),
                &mut scopes,
                engine,
            )
        })
    }

    #[test]
    fn p394_eval_retorna_valor_da_ultima_expressao() {
        assert_eq!(run_eval("1 + 2").unwrap(), Value::Int(3));
        assert_eq!(run_eval("7 * 8").unwrap(), Value::Int(56));
        assert_eq!(run_eval("\"foo\"").unwrap(), Value::Str("foo".into()));
    }

    #[test]
    fn p394_eval_ve_escopo_exterior() {
        assert_eq!(
            run_eval_with_var("x + 3", "x", Value::Int(7)).unwrap(),
            Value::Int(10)
        );
    }

    #[test]
    fn p394_eval_content_block_produz_content() {
        let r = run_eval("[*bold*]").unwrap();
        match r {
            Value::Content(c) => {
                // O content block deve conter texto strong com "bold".
                assert_eq!(c.plain_text(), "bold");
            }
            other => panic!("esperado Value::Content, obteve {other:?}"),
        }
    }

    #[test]
    fn p394_eval_identificador_desconhecido_erro() {
        let r = run_eval("variavel_inexistente");
        assert!(r.is_err(), "identificador desconhecido deve falhar");
    }

    #[test]
    fn p394_eval_sintaxe_invalida_erro() {
        let r = run_eval("let x =");
        assert!(r.is_err(), "sintaxe inválida deve falhar");
    }

    #[test]
    fn p394_eval_tipo_errado_erro() {
        let r = with_engine(|engine, world| {
            native_eval(
                &mut EvalContext::new(),
                &p(vec![Value::Int(42)]),
                world,
                test_file_id(),
                &mut Scopes::new(None),
                engine,
            )
        });
        assert!(r.is_err(), "eval com argumento não-string deve falhar");
    }

    #[test]
    fn p394_eval_arity_errado_erro() {
        let r = with_engine(|engine, world| {
            native_eval(
                &mut EvalContext::new(),
                &p(vec![]),
                world,
                test_file_id(),
                &mut Scopes::new(None),
                engine,
            )
        });
        assert!(r.is_err(), "eval sem argumentos deve falhar");
    }

    #[test]
    fn p394_eval_named_arg_inesperado_erro() {
        let mut args = p(vec![Value::Str("1".into())]);
        args.named.insert("mode".into(), Value::Str("code".into()));
        let r = with_engine(|engine, world| {
            native_eval(
                &mut EvalContext::new(),
                &args,
                world,
                test_file_id(),
                &mut Scopes::new(None),
                engine,
            )
        });
        assert!(r.is_err(), "named arg inesperado deve falhar");
    }

    // ── P471 — highlight radius/extent ──────────────────────────────────────

    #[test]
    fn p471_highlight_sem_args_ok() {
        null_ctx!(ctx);
        let r = text::native_highlight(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_ok());
        assert!(matches!(r.unwrap(), Value::Content(_)));
    }

    #[test]
    fn p471_highlight_radius_produz_campo() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
        let r = text::native_highlight(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Styled(_, styles)) = r {
            assert_eq!(styles.delta().highlight_radius, Some(Length::pt(3.0)));
        } else {
            panic!("esperado Content::Styled");
        }
    }

    #[test]
    fn p471_highlight_extent_produz_campo() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("extent".into(), Value::Length(Length::pt(2.0)));
        let r = text::native_highlight(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Styled(_, styles)) = r {
            assert_eq!(styles.delta().highlight_extent, Some(Length::pt(2.0)));
        } else {
            panic!("esperado Content::Styled");
        }
    }

    #[test]
    fn p471_highlight_named_desconhecido_erro() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("color".into(), Value::Length(Length::pt(1.0)));
        let r = text::native_highlight(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err());
    }

    // ── P471 — sub/super size ────────────────────────────────────────────────

    #[test]
    fn p471_sub_sem_size_sem_campo() {
        null_ctx!(ctx);
        let r = text::native_subscript(
            &mut ctx,
            &p(vec![Value::Content(Content::text("x"))]),
            &null_world(),
            test_file_id(),
        ).unwrap();
        if let Value::Content(Content::Styled(_, styles)) = r {
            assert_eq!(styles.delta().subscript_size, None);
        } else {
            panic!("esperado Content::Styled");
        }
    }

    #[test]
    fn p471_sub_com_size_produz_campo() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("size".into(), Value::Length(Length::pt(8.0)));
        let r = text::native_subscript(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Styled(_, styles)) = r {
            assert_eq!(styles.delta().subscript_size, Some(Length::pt(8.0)));
        } else {
            panic!("esperado Content::Styled");
        }
    }

    #[test]
    fn p471_super_com_size_produz_campo() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("size".into(), Value::Length(Length::pt(9.0)));
        let r = text::native_superscript(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Styled(_, styles)) = r {
            assert_eq!(styles.delta().superscript_size, Some(Length::pt(9.0)));
        } else {
            panic!("esperado Content::Styled");
        }
    }

    #[test]
    fn p471_sub_named_desconhecido_erro() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("offset".into(), Value::Length(Length::pt(1.0)));
        let r = text::native_subscript(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err());
    }

    // ── P471 — Value::Symbol + sym module ────────────────────────────────────

    #[test]
    fn p471_value_symbol_type_name() {
        use crate::entities::symbol::Symbol;
        let s = Symbol::new('α', "alpha");
        assert_eq!(Value::Symbol(s).type_name(), "symbol");
    }

    #[test]
    fn p471_sym_dict_contem_arrow() {
        use super::sym::build_sym_dict;
        let d = build_sym_dict();
        if let Value::Dict(map) = d {
            assert!(map.contains_key("arrow"));
            assert!(map.contains_key("alpha"));
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p471_sym_dict_arrow_e_simbolo_correcto() {
        use super::sym::build_sym_dict;
        use crate::entities::symbol::Symbol;
        let d = build_sym_dict();
        if let Value::Dict(map) = d {
            if let Some(Value::Symbol(s)) = map.get("arrow") {
                assert_eq!(s.ch, '→');
            } else {
                panic!("esperado Value::Symbol para arrow");
            }
        } else {
            panic!("esperado Value::Dict");
        }
    }

    // ── P472 — back-refs + ibid + LoF/LoT ────────────────────────────────────

    #[test]
    fn p472_back_refs_acumula_posicoes() {
        use crate::entities::bib_store::BibStore;
        let mut store = BibStore::empty();
        store.record_citation("a".to_string());
        store.record_citation("b".to_string());
        store.record_citation("a".to_string()); // segunda citação de "a"
        assert_eq!(store.back_refs_for_key("a"), vec![1, 3]);
        assert_eq!(store.back_refs_for_key("b"), vec![2]);
        assert!(store.back_refs_for_key("c").is_empty());
    }

    #[test]
    fn p472_back_refs_nao_afecta_citation_order() {
        use crate::entities::bib_store::BibStore;
        let mut store = BibStore::empty();
        store.record_citation("b".to_string());
        store.record_citation("a".to_string());
        store.record_citation("b".to_string());
        // citation_order preserva primeira aparição
        assert_eq!(store.citation_order(), &["b", "a"]);
        // back_refs regista todas as chamadas
        assert_eq!(store.back_refs_for_key("b"), vec![1, 3]);
    }

    #[test]
    fn p472_native_lof_sem_titulo() {
        null_ctx!(ctx);
        let result = super::structural::native_lof(
            &mut ctx, &p(vec![]), &null_world(), test_file_id(),
        );
        assert!(result.is_ok());
        match result.unwrap() {
            Value::Content(c) => match c {
                crate::entities::content::Content::Outline(e) => {
                    assert_eq!(e.target, crate::entities::elements::outline::OutlineTarget::Figures);
                    assert!(e.title.is_none());
                }
                _ => panic!("esperado Outline"),
            },
            _ => panic!("esperado Content"),
        }
    }

    #[test]
    fn p472_native_lot_sem_titulo() {
        null_ctx!(ctx);
        let result = super::structural::native_lot(
            &mut ctx, &p(vec![]), &null_world(), test_file_id(),
        );
        assert!(result.is_ok());
        match result.unwrap() {
            Value::Content(c) => match c {
                crate::entities::content::Content::Outline(e) => {
                    assert_eq!(e.target, crate::entities::elements::outline::OutlineTarget::Tables);
                    assert!(e.title.is_none());
                }
                _ => panic!("esperado Outline"),
            },
            _ => panic!("esperado Content"),
        }
    }

    #[test]
    fn p472_native_outline_target_figures() {
        null_ctx!(ctx);
        let args = pn(vec![], "target", Value::Str("figures".into()));
        let result = super::structural::native_outline(
            &mut ctx, &args, &null_world(), test_file_id(),
        );
        assert!(result.is_ok());
        match result.unwrap() {
            Value::Content(c) => match c {
                crate::entities::content::Content::Outline(e) => {
                    assert_eq!(e.target, crate::entities::elements::outline::OutlineTarget::Figures);
                }
                _ => panic!("esperado Outline"),
            },
            _ => panic!("esperado Content"),
        }
    }

    #[test]
    fn p472_native_outline_target_tables() {
        null_ctx!(ctx);
        let args = pn(vec![], "target", Value::Str("tables".into()));
        let result = super::structural::native_outline(
            &mut ctx, &args, &null_world(), test_file_id(),
        );
        assert!(result.is_ok());
        match result.unwrap() {
            Value::Content(c) => match c {
                crate::entities::content::Content::Outline(e) => {
                    assert_eq!(e.target, crate::entities::elements::outline::OutlineTarget::Tables);
                }
                _ => panic!("esperado Outline"),
            },
            _ => panic!("esperado Content"),
        }
    }

    #[test]
    fn p472_native_outline_target_invalido_erro() {
        null_ctx!(ctx);
        let args = pn(vec![], "target", Value::Str("unknown".into()));
        let result = super::structural::native_outline(
            &mut ctx, &args, &null_world(), test_file_id(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn p472_outline_target_headings_default() {
        null_ctx!(ctx);
        let result = super::structural::native_outline(
            &mut ctx, &p(vec![]), &null_world(), test_file_id(),
        );
        assert!(result.is_ok());
        match result.unwrap() {
            Value::Content(c) => match c {
                crate::entities::content::Content::Outline(e) => {
                    assert_eq!(e.target, crate::entities::elements::outline::OutlineTarget::Headings);
                }
                _ => panic!("esperado Outline"),
            },
            _ => panic!("esperado Content"),
        }
    }

    // ── P475 — inset/outset Sides dict + Value::Relative em extract_length ────

    #[test]
    fn p475_block_inset_dict_per_side() {
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        null_ctx!(ctx);
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("left".into(),  Value::Length(Length::pt(3.0)));
        d.insert("right".into(), Value::Length(Length::pt(5.0)));
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("inset".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(crate::entities::content::Content::Block(e)) = r {
            assert_eq!(e.inset.left,   Length::pt(3.0), "inset.left");
            assert_eq!(e.inset.right,  Length::pt(5.0), "inset.right");
            assert_eq!(e.inset.top,    Length::ZERO,    "inset.top default zero");
            assert_eq!(e.inset.bottom, Length::ZERO,    "inset.bottom default zero");
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p475_block_outset_dict_x_axis() {
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        null_ctx!(ctx);
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("x".into(), Value::Length(Length::pt(2.0)));
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("outset".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(crate::entities::content::Content::Block(e)) = r {
            assert_eq!(e.outset.left,  Length::pt(2.0), "outset.left via x");
            assert_eq!(e.outset.right, Length::pt(2.0), "outset.right via x");
            assert_eq!(e.outset.top,   Length::ZERO,    "outset.top zero (y não declarado)");
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p475_box_inset_dict_per_side() {
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        null_ctx!(ctx);
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("left".into(), Value::Length(Length::pt(3.0)));
        d.insert("top".into(),  Value::Length(Length::pt(5.0)));
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("inset".into(), Value::Dict(d));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(crate::entities::content::Content::Boxed(e)) = r {
            assert_eq!(e.inset.left,   Length::pt(3.0), "inset.left");
            assert_eq!(e.inset.top,    Length::pt(5.0), "inset.top");
            assert_eq!(e.inset.right,  Length::ZERO,    "inset.right default zero");
            assert_eq!(e.inset.bottom, Length::ZERO,    "inset.bottom default zero");
        } else { panic!("esperado Boxed"); }
    }

    #[test]
    fn p475_block_inset_uniforme_ainda_funciona() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(crate::entities::content::Content::Block(e)) = r {
            assert_eq!(e.inset.left,  Length::pt(4.0), "inset uniforme left");
            assert_eq!(e.inset.right, Length::pt(4.0), "inset uniforme right");
            assert_eq!(e.inset.top,   Length::pt(4.0), "inset uniforme top");
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p475_relative_aceite_em_extract_length_parte_abs() {
        use crate::entities::rel::Rel;
        null_ctx!(ctx);
        // Rel { rel: 0.5, abs: 2pt } → extract_length → Some(2pt); parte rel truncada.
        let rel = Rel { rel: 0.5, abs: Length::pt(2.0) };
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("inset".into(), Value::Relative(rel));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(crate::entities::content::Content::Block(e)) = r {
            assert_eq!(e.inset.left, Length::pt(2.0),
                "Relative {{ rel: 0.5, abs: 2pt }} → inset = 2pt (rel truncado)");
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p475_block_inset_dict_chave_invalida_erro() {
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        null_ctx!(ctx);
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("diagonal".into(), Value::Length(Length::pt(1.0)));
        let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
        args.named.insert("inset".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_err(), "chave desconhecida no dict de inset deve dar erro");
    }

    // ── P476 — módulo color (lighten / darken / mix / negate) ────────────────

    use super::color::{
        native_color_darken, native_color_lighten, native_color_mix, native_color_negate,
    };

    #[test]
    fn p476_native_color_lighten_retorna_color() {
        null_ctx!(ctx);
        let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
        let args = p(vec![red, Value::Float(0.2)]);
        let r = native_color_lighten(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "lighten deve retornar Ok; erro: {:?}", r.err());
        assert!(matches!(r.unwrap(), Value::Color(_)), "resultado deve ser Color");
    }

    #[test]
    fn p476_native_color_darken_retorna_color() {
        null_ctx!(ctx);
        let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
        let args = p(vec![blue, Value::Float(0.2)]);
        let r = native_color_darken(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok());
        assert!(matches!(r.unwrap(), Value::Color(_)));
    }

    #[test]
    fn p476_native_color_negate_vermelho_da_ciano() {
        null_ctx!(ctx);
        let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
        let args = p(vec![red]);
        let r = native_color_negate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Color(c) = r {
            let (r, g, b, _) = c.to_rgba_f32();
            assert!((r - 0.0).abs() < 1e-5);
            assert!((g - 1.0).abs() < 1e-5);
            assert!((b - 1.0).abs() < 1e-5);
        } else { panic!("esperado Color"); }
    }

    #[test]
    fn p476_native_color_mix_dois_positional_sem_weight() {
        null_ctx!(ctx);
        let red  = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
        let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
        let args = p(vec![red, blue]);
        let r = native_color_mix(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "mix sem weight: default 0.5; erro: {:?}", r.err());
        assert!(matches!(r.unwrap(), Value::Color(_)));
    }

    #[test]
    fn p476_native_color_mix_com_weight_named() {
        null_ctx!(ctx);
        let red  = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
        let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
        let mut args = p(vec![red, blue]);
        args.named.insert("weight".into(), Value::Float(0.25));
        let r = native_color_mix(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "mix com weight: 0.25; erro: {:?}", r.err());
    }

    #[test]
    fn p476_color_module_no_scope_tem_4_entradas() {
        use crate::rules::stdlib::make_color_module;
        let m = make_color_module();
        if let Value::Dict(d) = m {
            assert!(d.contains_key("lighten"), "falta lighten");
            assert!(d.contains_key("darken"),  "falta darken");
            assert!(d.contains_key("mix"),     "falta mix");
            assert!(d.contains_key("negate"),  "falta negate");
            // NOTE: P477 aumenta para 6; atualizado em p477_color_module_tem_6_entradas
        } else { panic!("make_color_module deve retornar Dict"); }
    }

    // ── P477 — saturate/desaturate + constantes nomeadas ─────────────────────

    use super::color::{native_color_desaturate, native_color_saturate};

    #[test]
    fn p477_native_color_saturate_retorna_color() {
        null_ctx!(ctx);
        let red  = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
        let args = p(vec![red, Value::Float(0.2)]);
        let r = native_color_saturate(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok(), "saturate deve retornar Ok; erro: {:?}", r.err());
        assert!(matches!(r.unwrap(), Value::Color(_)));
    }

    #[test]
    fn p477_native_color_desaturate_retorna_color() {
        null_ctx!(ctx);
        let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
        let args = p(vec![blue, Value::Float(0.3)]);
        let r = native_color_desaturate(&mut ctx, &args, &null_world(), test_file_id());
        assert!(r.is_ok());
        assert!(matches!(r.unwrap(), Value::Color(_)));
    }

    #[test]
    fn p477_color_module_tem_6_entradas() {
        use crate::rules::stdlib::make_color_module;
        let m = make_color_module();
        if let Value::Dict(d) = m {
            assert!(d.contains_key("saturate"),   "falta saturate");
            assert!(d.contains_key("desaturate"), "falta desaturate");
            assert_eq!(d.len(), 6, "módulo color deve ter exactamente 6 entradas pós-P477");
        } else { panic!("make_color_module deve retornar Dict"); }
    }

    #[test]
    fn p477_parse_color_yellow() {
        let v = Value::Str("yellow".into());
        let c = parse_color(&v);
        assert_eq!(c, Some(Color::rgb(255, 255, 0)), "yellow deve ser (255,255,0)");
    }

    #[test]
    fn p477_parse_color_gray_grey_aliases() {
        let gray = parse_color(&Value::Str("gray".into()));
        let grey = parse_color(&Value::Str("grey".into()));
        assert_eq!(gray, grey, "gray e grey devem ser idênticos");
        assert_eq!(gray, Some(Color::rgb(128, 128, 128)));
    }

    #[test]
    fn p477_parse_color_aqua_cyan_aliases() {
        let aqua = parse_color(&Value::Str("aqua".into()));
        let cyan = parse_color(&Value::Str("cyan".into()));
        assert_eq!(aqua, cyan, "aqua e cyan devem ser idênticos");
        assert_eq!(aqua, Some(Color::rgb(0, 255, 255)));
    }

    #[test]
    fn p477_parse_color_navy_maroon_teal() {
        assert_eq!(parse_color(&Value::Str("navy".into())),   Some(Color::rgb(0, 0, 128)));
        assert_eq!(parse_color(&Value::Str("maroon".into())), Some(Color::rgb(128, 0, 0)));
        assert_eq!(parse_color(&Value::Str("teal".into())),   Some(Color::rgb(0, 128, 128)));
    }

    #[test]
    fn p477_parse_color_original_5_sem_regressao() {
        assert_eq!(parse_color(&Value::Str("red".into())),   Some(Color::rgb(255, 0,   0)));
        assert_eq!(parse_color(&Value::Str("green".into())), Some(Color::rgb(0,   128, 0)));
        assert_eq!(parse_color(&Value::Str("blue".into())),  Some(Color::rgb(0,   0,   255)));
        assert_eq!(parse_color(&Value::Str("black".into())), Some(Color::rgb(0,   0,   0)));
        assert_eq!(parse_color(&Value::Str("white".into())), Some(Color::rgb(255, 255, 255)));
    }

    #[test]
    fn p477_parse_color_desconhecida_retorna_none() {
        assert_eq!(parse_color(&Value::Str("ultraviolet".into())), None);
    }
}
