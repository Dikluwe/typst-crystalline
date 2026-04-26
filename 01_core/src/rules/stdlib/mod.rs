//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23

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
mod foundations;
mod calc;
mod text;
mod assert;
mod structural;
mod figure_image;
mod shapes;
mod transforms;
mod layout;

// Re-exports públicos — preservam o path `crate::rules::stdlib::native_X` usado
// por `make_stdlib` em `eval/mod.rs`.
pub use crate::rules::stdlib::foundations::{
    native_float, native_int, native_len, native_luma, native_range, native_rgb,
    native_str, native_type,
};
pub use crate::rules::stdlib::calc::make_calc_module;
pub use crate::rules::stdlib::text::{native_lower, native_replace, native_upper};
pub use crate::rules::stdlib::assert::native_assert;
pub use crate::rules::stdlib::structural::{
    native_divider, native_emph, native_heading, native_quote, native_raw, native_strong, native_terms,
};
pub use crate::rules::stdlib::figure_image::{native_figure, native_image};
pub use crate::rules::stdlib::shapes::{
    native_circle, native_ellipse, native_line, native_polygon, native_rect,
};
pub use crate::rules::stdlib::transforms::{native_move, native_rotate, native_scale, native_skew};
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_grid, native_h, native_hide,
    native_pad, native_page, native_pagebreak, native_place, native_repeat, native_stack, native_v,
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
pub(super) fn expect_no_named(named: &IndexMap<EcoString, Value, FxBuildHasher>) -> SourceResult<()> {
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
    use super::*;
    use super::shapes::parse_color;
    use super::calc::{
        calc_abs, calc_ceil, calc_clamp, calc_floor, calc_max, calc_min, calc_pow,
        calc_round, calc_sqrt,
    };
    use crate::entities::args::Args;
    use crate::entities::content::Content;
    use crate::entities::layout_types::Color;
    use crate::rules::eval::EvalContext;
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

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
        book:    FontBook,
        files:   std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
    }
    impl World for NullWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
        fn read_bytes(&self, _current_file: FileId, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
            self.files.get(path)
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
        }
    }

    /// Helper que cria um World nulo para tests que passam para o ABI.
    fn null_world() -> NullWorld {
        NullWorld::default()
    }

    /// Helper: `FileId` dummy para tests que não fazem I/O.
    fn test_file_id() -> crate::entities::file_id::FileId {
        crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap()
        )
    }

    #[test]
    fn native_type_directo() {
        null_ctx!(ctx);
        assert_eq!(native_type(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id(), None).unwrap(),     Value::Str("int".into()));
        assert_eq!(native_type(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id(), None).unwrap(), Value::Str("bool".into()));
        assert_eq!(native_type(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id(), None).unwrap(),       Value::Str("none".into()));
        assert!(native_type(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).is_err());
        assert!(native_type(&mut ctx, &p(vec![Value::Int(1), Value::Int(2)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn native_type_named_arg_retorna_err() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Int(1)], "extra", Value::Bool(true));
        assert!(native_type(&mut ctx, &args, &null_world(), test_file_id(), None).is_err(), "named arg inesperado deve retornar Err");
    }

    #[test]
    fn native_len_directo() {
        null_ctx!(ctx);
        assert_eq!(native_len(&mut ctx, &p(vec![Value::Str("abc".into())]), &null_world(), test_file_id(), None).unwrap(), Value::Int(3));
        assert_eq!(
            native_len(&mut ctx, &p(vec![Value::Array(vec![Value::Int(1), Value::Int(2)])]), &null_world(), test_file_id(), None).unwrap(),
            Value::Int(2)
        );
        assert!(native_len(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id(), None).is_err());
        assert!(native_len(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).is_err());
    }

    // ── Passo 25 — rgb/luma ──────────────────────────────────────────────────

    #[test]
    fn stdlib_rgb_tres_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(&mut ctx, &p(vec![Value::Int(255), Value::Int(0), Value::Int(128)]), &null_world(), test_file_id(), None).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(255, 0, 128)));
    }

    #[test]
    fn stdlib_rgb_quatro_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(&mut ctx, &p(vec![Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)]), &null_world(), test_file_id(), None).unwrap();
        assert_eq!(r, Value::Color(Color::rgba(255, 0, 0, 200)));
    }

    #[test]
    fn stdlib_rgb_out_of_range() {
        null_ctx!(ctx);
        assert!(native_rgb(&mut ctx, &p(vec![Value::Int(300), Value::Int(0), Value::Int(0)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn stdlib_luma() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_luma(&mut ctx, &p(vec![Value::Int(128)]), &null_world(), test_file_id(), None).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(128, 128, 128)));
    }

    // ── Passo 27 — str/int/float ─────────────────────────────────────────────

    #[test]
    fn native_str_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id(), None).unwrap(), Value::Str("42".into()));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_str_de_float() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Float(3.14)]), &null_world(), test_file_id(), None).unwrap(), Value::Str("3.14".into()));
    }

    #[test]
    fn native_str_de_bool() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id(), None).unwrap(),  Value::Str("true".into()));
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Bool(false)]), &null_world(), test_file_id(), None).unwrap(), Value::Str("false".into()));
    }

    #[test]
    fn native_str_identity() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Str("hello".into())]), &null_world(), test_file_id(), None).unwrap(), Value::Str("hello".into()));
    }

    #[test]
    fn native_str_de_none() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id(), None).unwrap(), Value::Str("none".into()));
    }

    #[test]
    fn native_int_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(42));
    }

    #[test]
    fn native_int_de_str() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Str("42".into())]), &null_world(), test_file_id(), None).unwrap(), Value::Int(42));
        assert!(native_int(&mut ctx, &p(vec![Value::Str("abc".into())]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn native_int_de_bool() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id(), None).unwrap(),  Value::Int(1));
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Bool(false)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(0));
    }

    #[test]
    fn native_int_float_retorna_err() {
        null_ctx!(ctx);
        assert!(native_int(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn native_float_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_float(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id(), None).unwrap(), Value::Float(3.0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_float_de_str() {
        null_ctx!(ctx);
        assert_eq!(native_float(&mut ctx, &p(vec![Value::Str("3.14".into())]), &null_world(), test_file_id(), None).unwrap(), Value::Float(3.14));
        assert!(native_float(&mut ctx, &p(vec![Value::Str("abc".into())]), &null_world(), test_file_id(), None).is_err());
    }

    // ── Passo 27 — calc ──────────────────────────────────────────────────────

    #[test]
    fn calc_abs_int() {
        null_ctx!(ctx);
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(-5)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(5));
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id(), None).unwrap(),  Value::Int(5));
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id(), None).unwrap(),  Value::Int(0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn calc_abs_float() {
        null_ctx!(ctx);
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Float(-3.14)]), &null_world(), test_file_id(), None).unwrap(), Value::Float(3.14));
    }

    #[test]
    fn calc_pow_int() {
        null_ctx!(ctx);
        assert_eq!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(10)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(1024));
        assert_eq!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(0)]), &null_world(), test_file_id(), None).unwrap(),  Value::Int(1));
    }

    #[test]
    fn calc_pow_float() {
        null_ctx!(ctx);
        let r = calc_pow(&mut ctx, &p(vec![Value::Float(2.0), Value::Float(0.5)]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Float(f) if (f - std::f64::consts::SQRT_2).abs() < 1e-10));
    }

    #[test]
    fn calc_pow_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(-1)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn calc_sqrt_positivo() {
        null_ctx!(ctx);
        assert_eq!(calc_sqrt(&mut ctx, &p(vec![Value::Float(4.0)]), &null_world(), test_file_id(), None).unwrap(), Value::Float(2.0));
        assert_eq!(calc_sqrt(&mut ctx, &p(vec![Value::Int(4)]), &null_world(), test_file_id(), None).unwrap(),     Value::Float(2.0));
    }

    #[test]
    fn calc_sqrt_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_sqrt(&mut ctx, &p(vec![Value::Float(-1.0)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn calc_floor_ceil_round() {
        null_ctx!(ctx);
        assert_eq!(calc_floor(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(3));
        assert_eq!(calc_ceil(&mut ctx, &p(vec![Value::Float(3.2)]), &null_world(), test_file_id(), None).unwrap(),  Value::Int(4));
        assert_eq!(calc_round(&mut ctx, &p(vec![Value::Float(3.5)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(4));
        assert_eq!(calc_round(&mut ctx, &p(vec![Value::Float(3.4)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_max_int() {
        null_ctx!(ctx);
        assert_eq!(calc_min(&mut ctx, &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(1));
        assert_eq!(calc_max(&mut ctx, &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_vazio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_min(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).is_err());
        assert!(calc_max(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn calc_clamp_int() {
        null_ctx!(ctx);
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(5),  Value::Int(0), Value::Int(10)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(5));
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(-5), Value::Int(0), Value::Int(10)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(0));
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(15), Value::Int(0), Value::Int(10)]), &null_world(), test_file_id(), None).unwrap(), Value::Int(10));
    }

    #[test]
    fn calc_clamp_min_maior_max_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_clamp(&mut ctx, &p(vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)]), &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn native_range_directo() {
        null_ctx!(ctx);
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id(), None).unwrap(),
                   Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(2), Value::Int(5)]), &null_world(), test_file_id(), None).unwrap(),
                   Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)]));
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(3), Value::Int(3)]), &null_world(), test_file_id(), None).unwrap(),
                   Value::Array(vec![]));
        assert!(native_range(&mut ctx, &p(vec![Value::Int(-1)]), &null_world(), test_file_id(), None).is_err());
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
        let result = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: Some(_), .. })),
            "figure com caption deve ter Some(caption): {:?}", result);
    }

    #[test]
    fn native_figure_sem_caption() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let body_content = Content::text("Diagrama");
        let args = p(vec![Value::Content(body_content)]);
        let result = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure sem caption deve ter None: {:?}", result);
    }

    #[test]
    fn native_figure_caption_none_value() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        // caption: none → ausência de legenda
        let body_content = Content::text("Corpo");
        let args = pn(vec![Value::Content(body_content)], "caption", Value::None);
        let result = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure com caption: none deve ter caption None");
    }

    #[test]
    fn native_figure_sem_body_retorna_err() {
        null_ctx!(ctx);
        let args = p(vec![]);
        assert!(native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).is_err(), "figure sem body deve retornar Err");
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
        assert!(native_assert(&mut ctx, &args, &null_world(), test_file_id(), None).is_ok(), "assert(true) deve ter sucesso");
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_padrao() {
        null_ctx!(ctx);
        let args = p(vec![Value::Bool(false)]);
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("falhou") || err[0].message.contains("Asser"),
            "mensagem de erro padrão deve mencionar a asserção: {:?}", err[0].message
        );
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_personalizada() {
        null_ctx!(ctx);
        // Mensagem sem acentos para evitar problemas de codificação em CI.
        let args = pn(vec![Value::Bool(false)], "message", Value::Str("Matematica falhou".into()));
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err());
        assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
    }

    #[test]
    fn native_assert_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Bool(true)], "bla", Value::Str("bla".into()));
        let result = native_assert(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("inesperado") && err[0].message.contains("bla"),
            "named arg desconhecido deve gerar erro: {:?}", err[0].message
        );
    }

    // ── Passo 71 — native_image ───────────────────────────────────────────────

    #[test]
    fn native_image_retorna_content_image() {
        let mut world = NullWorld::default();
        world.files.insert("foto.png".to_string(), std::sync::Arc::new(vec![1, 2, 3]));
        let _dummy_id = crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let mut ctx = EvalContext::new();
        let args = p(vec![Value::Str("foto.png".into())]);
        let result = native_image(&mut ctx, &args, &world, test_file_id(), None).unwrap();
        assert!(matches!(result, Value::Content(Content::Image { .. })));
    }

    #[test]
    fn native_image_ficheiro_inexistente_gera_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("naoexiste.png".into())]);
        assert!(native_image(&mut ctx, &args, &null_world(), test_file_id(), None).is_err());
    }

    #[test]
    fn native_image_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Str("foto.png".into())], "cor", Value::Str("red".into()));
        assert!(native_image(&mut ctx, &args, &null_world(), test_file_id(), None).is_err());
    }

    // ── Passo 76 — primitivas geométricas ────────────────────────────────────

    #[test]
    fn rect_sem_cores_tem_stroke_preta_1pt() {
        // #rect() sem fill nem stroke → stroke preta de 1pt.
        // Confirma que a stdlib é o único local onde este fallback existe.
        null_ctx!(ctx);
        let result = native_rect(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Shape { fill, stroke, .. }) = result {
            assert!(fill.is_none(), "rect sem fill deve ter fill: None");
            let s = stroke.expect("rect sem cores deve ter stroke de fallback");
            assert_eq!(s.paint, Color::rgb(0, 0, 0), "stroke de fallback deve ser preta");
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
        let result = native_rect(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Shape { fill, stroke, .. }) = result {
            assert!(fill.is_some(), "fill red deve estar presente");
            assert!(stroke.is_none(), "sem stroke explícito e com fill → stroke deve ser None");
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
        let result = native_line(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Shape { kind, fill, stroke, .. }) = result {
            assert!(matches!(kind, ShapeKind::Line { dx, dy } if dx == 100.0 && dy == 50.0));
            assert!(fill.is_none(), "linha não tem fill");
            assert!(stroke.is_some(), "linha tem stroke por omissão");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn polygon_sem_pontos_gera_erro() {
        null_ctx!(ctx);
        let args = Args::positional(vec![]);
        let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err(), "polygon() sem pontos deve retornar Err");
        let msg = result.unwrap_err();
        let msg_str = format!("{:?}", msg);
        assert!(msg_str.contains("pelo menos um ponto"),
            "Mensagem de erro deve mencionar 'pelo menos um ponto', obteve: {}", msg_str);
    }

    #[test]
    fn polygon_com_um_ponto_gera_moveto_e_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
        ]);
        let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Shape { kind: ShapeKind::Path(items), .. }) = result {
            assert_eq!(items.len(), 2, "Um ponto deve gerar MoveTo + ClosePath");
            assert!(matches!(items[0], PathItem::MoveTo(_)), "Primeiro item deve ser MoveTo");
            assert!(matches!(items[1], PathItem::ClosePath), "Último item deve ser ClosePath");
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn polygon_triangulo_gera_moveto_lineto_lineto_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(0.0),  Value::Float(0.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(25.0), Value::Float(50.0)]),
        ]);
        let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Shape { kind: ShapeKind::Path(items), .. }) = result {
            assert_eq!(items.len(), 4); // MoveTo + 2×LineTo + ClosePath
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::LineTo(_)));
            assert!(matches!(items[2], PathItem::LineTo(_)));
            assert!(matches!(items[3], PathItem::ClosePath));
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn parse_color_nomes_conhecidos() {
        assert_eq!(parse_color(&Value::Str("red".into())),   Some(Color::rgb(255, 0, 0)));
        assert_eq!(parse_color(&Value::Str("green".into())), Some(Color::rgb(0, 128, 0)));
        assert_eq!(parse_color(&Value::Str("blue".into())),  Some(Color::rgb(0, 0, 255)));
        assert_eq!(parse_color(&Value::Str("black".into())), Some(Color::rgb(0, 0, 0)));
        assert_eq!(parse_color(&Value::Str("white".into())), Some(Color::rgb(255, 255, 255)));
        assert_eq!(parse_color(&Value::Str("purple".into())), None);
        assert_eq!(parse_color(&Value::Int(42)), None);
    }

    // ── Passo 156C (ADR-0061 Fase 1, sub-passo 1) — pad + hide ─────────────

    #[test]
    fn native_pad_defaults_padding_zero() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let body = Content::text("body");
        let args = p(vec![Value::Content(body)]);
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { body, padding }) = result {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(padding.left,   Length::ZERO);
            assert_eq!(padding.right,  Length::ZERO);
            assert_eq!(padding.top,    Length::ZERO);
            assert_eq!(padding.bottom, Length::ZERO);
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_lados_individuais() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("left".into(),   Value::Length(Length::pt(1.0)));
        args.named.insert("right".into(),  Value::Length(Length::pt(2.0)));
        args.named.insert("top".into(),    Value::Length(Length::pt(3.0)));
        args.named.insert("bottom".into(), Value::Length(Length::pt(4.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { padding, .. }) = result {
            assert_eq!(padding.left,   Length::pt(1.0));
            assert_eq!(padding.right,  Length::pt(2.0));
            assert_eq!(padding.top,    Length::pt(3.0));
            assert_eq!(padding.bottom, Length::pt(4.0));
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
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { padding, .. }) = result {
            assert_eq!(padding.left,   Length::pt(5.0));
            assert_eq!(padding.right,  Length::pt(5.0));
            assert_eq!(padding.top,    Length::pt(7.0));
            assert_eq!(padding.bottom, Length::pt(7.0));
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
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { padding, .. }) = result {
            assert_eq!(padding.left,   Length::pt(8.0));
            assert_eq!(padding.right,  Length::pt(8.0));
            assert_eq!(padding.top,    Length::pt(8.0));
            assert_eq!(padding.bottom, Length::pt(8.0));
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
        args.named.insert("x".into(),    Value::Length(Length::pt(2.0)));
        args.named.insert("rest".into(), Value::Length(Length::pt(3.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { padding, .. }) = result {
            // left vence (específico)
            assert_eq!(padding.left,   Length::pt(1.0));
            // right cai para x (eixo)
            assert_eq!(padding.right,  Length::pt(2.0));
            // top cai para rest (não há y nem específico)
            assert_eq!(padding.top,    Length::pt(3.0));
            assert_eq!(padding.bottom, Length::pt(3.0));
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
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err(), "padding negativo deve retornar Err em P156C");
    }

    #[test]
    fn native_pad_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("strange".into(), Value::Length(Length::pt(1.0)));
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err(), "named arg desconhecido em pad() deve retornar Err");
    }

    #[test]
    fn native_pad_aceita_int_e_float_como_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Float(2.5));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { padding, .. }) = r {
            assert_eq!(padding.left, Length::pt(2.5));
        } else {
            panic!("esperado Content::Pad");
        }
    }

    #[test]
    fn native_pad_sem_body_retorna_err() {
        null_ctx!(ctx);
        let result = native_pad(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(result.is_err(), "pad() sem body deve retornar Err");
    }

    #[test]
    fn native_hide_envolve_body() {
        null_ctx!(ctx);
        let body = Content::text("invisivel");
        let args = p(vec![Value::Content(body)]);
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Hide { body }) = result {
            assert_eq!(body.plain_text(), "invisivel");
        } else {
            panic!("esperado Content::Hide");
        }
    }

    #[test]
    fn native_hide_aceita_string() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("placeholder".into())]);
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(result, Value::Content(Content::Hide { .. })));
    }

    #[test]
    fn native_hide_rejeita_named_arg() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Content(Content::text("x"))], "weak", Value::Bool(true));
        let result = native_hide(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(result.is_err(), "hide() não aceita named args (P156C)");
    }

    #[test]
    fn native_hide_sem_body_retorna_err() {
        null_ctx!(ctx);
        let result = native_hide(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(result.is_err(), "hide() sem body deve retornar Err");
    }

    // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ──────────

    #[test]
    fn native_h_aceita_length() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let args = p(vec![Value::Length(Length::pt(12.0))]);
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::HSpace { amount, weak }) = r {
            assert_eq!(amount, Length::pt(12.0));
            assert!(!weak); // default
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_aceita_int_e_float_como_pt() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        // Int interpretado em pt.
        let r = native_h(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::HSpace { amount, .. }) = r {
            assert_eq!(amount, Length::pt(5.0));
        } else {
            panic!("esperado Content::HSpace");
        }
        // Float interpretado em pt.
        let r = native_h(&mut ctx, &p(vec![Value::Float(2.5)]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::HSpace { amount, .. }) = r {
            assert_eq!(amount, Length::pt(2.5));
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
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::HSpace { weak, .. }) = r {
            assert!(weak);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_aceita_amount_zero() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_h(&mut ctx, &p(vec![Value::Length(Length::ZERO)]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::HSpace { amount, .. }) = r {
            assert_eq!(amount, Length::ZERO);
        } else {
            panic!("esperado Content::HSpace");
        }
    }

    #[test]
    fn native_h_rejeita_amount_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_h(&mut ctx, &p(vec![Value::Length(Length::pt(-1.0))]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
    }

    #[test]
    fn native_h_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Length(Length::pt(1.0))]);
        args.named.insert("attached".into(), Value::Bool(true));
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em h() deve retornar Err");
    }

    #[test]
    fn native_h_rejeita_weak_nao_bool() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Length(Length::pt(1.0))]);
        args.named.insert("weak".into(), Value::Int(1)); // tipo errado
        let r = native_h(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "weak não-bool deve retornar Err");
    }

    #[test]
    fn native_h_sem_amount_retorna_err() {
        null_ctx!(ctx);
        let r = native_h(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
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
        let r = native_v(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::VSpace { amount, weak }) = r {
            assert_eq!(amount, Length::pt(15.0));
            assert!(weak);
        } else {
            panic!("esperado Content::VSpace");
        }
    }

    #[test]
    fn native_v_rejeita_amount_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_v(&mut ctx, &p(vec![Value::Length(Length::pt(-2.0))]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
    }

    #[test]
    fn native_v_sem_amount_retorna_err() {
        null_ctx!(ctx);
        let r = native_v(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "v() sem amount deve retornar Err");
    }

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak manual ───────

    #[test]
    fn native_pagebreak_defaults() {
        null_ctx!(ctx);
        let r = native_pagebreak(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pagebreak { weak, to }) = r {
            assert!(!weak);
            assert_eq!(to, None);
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_com_weak_true() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pagebreak { weak, .. }) = r {
            assert!(weak);
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
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pagebreak { to, .. }) = r {
            assert_eq!(to, Some(Parity::Even));
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
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pagebreak { to, .. }) = r {
            assert_eq!(to, Some(Parity::Odd));
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
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pagebreak { weak, to }) = r {
            assert!(weak);
            assert_eq!(to, Some(Parity::Even));
        } else {
            panic!("esperado Content::Pagebreak");
        }
    }

    #[test]
    fn native_pagebreak_rejeita_to_invalido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("middle".into()));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "pagebreak(to: \"middle\") deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_to_nao_string() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Int(2));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "pagebreak(to:) com tipo errado deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("strange".into(), Value::Bool(false));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em pagebreak() deve retornar Err");
    }

    #[test]
    fn native_pagebreak_rejeita_argumento_posicional() {
        null_ctx!(ctx);
        let r = native_pagebreak(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "pagebreak() não aceita argumentos posicionais");
    }

    #[test]
    fn native_pagebreak_rejeita_weak_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Int(1));
        let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "weak não-Bool deve retornar Err");
    }

    // ── Passo 156F (ADR-0061 Fase 1, sub-passo 4) — skew ─────────────────

    #[test]
    fn native_skew_defaults_produz_identidade() {
        // Sem ax nem ay, skew produz matriz identidade.
        null_ctx!(ctx);
        use crate::entities::layout_types::TransformMatrix;
        let body = Content::text("body");
        let r = native_skew(&mut ctx, &p(vec![Value::Content(body)]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            assert_eq!(matrix, TransformMatrix::identity());
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
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            // c = tan(30°) ≈ 0.5774; a = 1; b = 0; d = 1.
            assert!((matrix.a - 1.0).abs() < 1e-9);
            assert!((matrix.b - 0.0).abs() < 1e-9);
            assert!((matrix.c - 0.5774).abs() < 0.001, "c esperado tan(30°), obteve {}", matrix.c);
            assert!((matrix.d - 1.0).abs() < 1e-9);
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
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            // b = tan(30°) ≈ 0.5774; c = 0.
            assert!((matrix.b - 0.5774).abs() < 0.001, "b esperado tan(30°), obteve {}", matrix.b);
            assert!((matrix.c - 0.0).abs() < 1e-9);
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
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            assert!((matrix.c - 15.0_f64.to_radians().tan()).abs() < 1e-9);
            assert!((matrix.b - 1.0).abs() < 1e-9, "tan(45°) ≈ 1.0; obteve {}", matrix.b);
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
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            // tan(0) = 0 → identidade.
            assert_eq!(matrix, TransformMatrix::identity());
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
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "origin scope-out — named arg desconhecido em P156F");
        let _ = Angle::deg(0.0); // suprime warning de import não usado se ramo skip
    }

    #[test]
    fn native_skew_rejeita_ax_proximo_de_pi_meio() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        // 89.999° → ax_rad ≈ π/2 - 1.7e-5; abaixo do threshold (π/2 - 1e-3).
        // Mas usemos exactamente 90°: tan diverge.
        args.named.insert("ax".into(), Value::Float(std::f64::consts::FRAC_PI_2));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "skew(ax: π/2) deve retornar Err (tan diverge)");
    }

    #[test]
    fn native_skew_rejeita_ax_nao_angle_nem_float() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("ax".into(), Value::Str("30deg".into()));
        let r = native_skew(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "skew(ax:) com tipo errado deve retornar Err");
    }

    #[test]
    fn native_skew_sem_body_retorna_err() {
        null_ctx!(ctx);
        let r = native_skew(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
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
        let r = native_move(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            assert_eq!(matrix, TransformMatrix::translate(10.0, 5.0));
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
        let r = native_rotate(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            // Comparação de matriz aproximada (rotate usa cos/sin).
            let expected = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
            assert!((matrix.a - expected.a).abs() < 1e-9);
            assert!((matrix.b - expected.b).abs() < 1e-9);
            assert!((matrix.c - expected.c).abs() < 1e-9);
            assert!((matrix.d - expected.d).abs() < 1e-9);
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
        let r = native_scale(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Transform { matrix, .. }) = r {
            // x: 2.0; y default = sx = 2.0.
            assert_eq!(matrix, TransformMatrix::scale(2.0, 2.0));
        } else {
            panic!("regressão: native_scale deveria produzir Content::Transform");
        }
    }

    // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block ──────────────────

    #[test]
    fn native_block_defaults_sem_args_named() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_block(&mut ctx, &p(vec![Value::Content(Content::text("body"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { body, width, height, inset, breakable }) = r {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(width,  None);
            assert_eq!(height, None);
            assert_eq!(inset.left, Length::ZERO);
            assert!(breakable, "default breakable é true");
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_sem_body_aceita_empty() {
        // Vanilla aceita block() sem body; cristalino igualmente
        // (Content::Empty como fallback).
        null_ctx!(ctx);
        let r = native_block(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { body, .. }) = r {
            assert!(body.is_empty());
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { width, .. }) = r {
            assert_eq!(width, Some(Length::pt(100.0)));
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { height, .. }) = r {
            assert_eq!(height, Some(Length::pt(50.0)));
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { inset, .. }) = r {
            assert_eq!(inset.left,   Length::pt(8.0));
            assert_eq!(inset.right,  Length::pt(8.0));
            assert_eq!(inset.top,    Length::pt(8.0));
            assert_eq!(inset.bottom, Length::pt(8.0));
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_com_breakable_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { breakable, .. }) = r {
            assert!(!breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_combina_atributos() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(),  Value::Length(Length::pt(200.0)));
        args.named.insert("height".into(), Value::Length(Length::pt(80.0)));
        args.named.insert("inset".into(),  Value::Length(Length::pt(4.0)));
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { width, height, inset, breakable, .. }) = r {
            assert_eq!(width,  Some(Length::pt(200.0)));
            assert_eq!(height, Some(Length::pt(80.0)));
            assert_eq!(inset.top, Length::pt(4.0));
            assert!(!breakable);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn native_block_rejeita_named_arg_avancado() {
        // Atributos avançados (fill, stroke, radius, clip, ...) scope-out
        // per ADR-0054 graded; rejeitar com erro hard até refino futuro.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill é scope-out em P156G; deve retornar Err");
    }

    #[test]
    fn native_block_rejeita_width_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(-10.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "width negativo deve retornar Err em P156G");
    }

    #[test]
    fn native_block_rejeita_inset_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(-2.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "inset negativo deve retornar Err em P156G");
    }

    #[test]
    fn native_block_rejeita_breakable_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("breakable".into(), Value::Int(1));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
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
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad { .. })),
            "regressão: native_pad deveria produzir Content::Pad");
        // Hide regression
        let r = native_hide(&mut ctx, &p(vec![Value::Content(Content::text("y"))]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Hide { .. })),
            "regressão: native_hide deveria produzir Content::Hide");
    }

    // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box ────────────────────

    #[test]
    fn native_box_defaults_sem_args_named() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let r = native_box(&mut ctx, &p(vec![Value::Content(Content::text("body"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { body, width, height, inset, baseline }) = r {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(width,  None);
            assert_eq!(height, None);
            assert_eq!(inset.left, Length::ZERO);
            assert_eq!(baseline, Length::ZERO);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_sem_body_aceita_empty() {
        null_ctx!(ctx);
        let r = native_box(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { body, .. }) = r {
            assert!(body.is_empty());
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { width, .. }) = r {
            assert_eq!(width, Some(Length::pt(80.0)));
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { height, .. }) = r {
            assert_eq!(height, Some(Length::pt(20.0)));
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { inset, .. }) = r {
            assert_eq!(inset.left,   Length::pt(4.0));
            assert_eq!(inset.right,  Length::pt(4.0));
            assert_eq!(inset.top,    Length::pt(4.0));
            assert_eq!(inset.bottom, Length::pt(4.0));
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { baseline, .. }) = r {
            assert_eq!(baseline, Length::pt(3.0));
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { baseline, .. }) = r {
            assert_eq!(baseline, Length::pt(-5.0));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn native_box_combina_atributos() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(),    Value::Length(Length::pt(100.0)));
        args.named.insert("height".into(),   Value::Length(Length::pt(30.0)));
        args.named.insert("inset".into(),    Value::Length(Length::pt(2.0)));
        args.named.insert("baseline".into(), Value::Length(Length::pt(1.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { width, height, inset, baseline, .. }) = r {
            assert_eq!(width,  Some(Length::pt(100.0)));
            assert_eq!(height, Some(Length::pt(30.0)));
            assert_eq!(inset.top, Length::pt(2.0));
            assert_eq!(baseline, Length::pt(1.0));
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
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill é scope-out em P156H; deve retornar Err");
    }

    #[test]
    fn native_box_rejeita_width_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(-5.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "width negativo deve retornar Err em P156H");
    }

    #[test]
    fn native_box_rejeita_inset_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(-1.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "inset negativo deve retornar Err em P156H");
    }

    #[test]
    fn native_box_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("alignment".into(), Value::Str("center".into()));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em box() deve retornar Err");
    }

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack ─────────────────

    #[test]
    fn native_stack_defaults_sem_args() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        let r = native_stack(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { children, dir, spacing }) = r {
            assert!(children.is_empty());
            assert_eq!(dir, Dir::TTB);  // default
            assert_eq!(spacing, None);
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
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { dir, .. }) = r {
            assert_eq!(dir, Dir::LTR);
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_aceita_todas_4_direcoes() {
        null_ctx!(ctx);
        use crate::entities::dir::Dir;
        for (s, d) in [("ltr", Dir::LTR), ("rtl", Dir::RTL),
                       ("ttb", Dir::TTB), ("btt", Dir::BTT)] {
            let mut args = p(vec![]);
            args.named.insert("dir".into(), Value::Str(s.into()));
            let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
            if let Value::Content(Content::Stack { dir, .. }) = r {
                assert_eq!(dir, d, "dir={s}");
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
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { spacing, .. }) = r {
            assert_eq!(spacing, Some(Length::pt(8.0)));
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
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { children, .. }) = r {
            assert_eq!(children.len(), 3);
            assert_eq!(children[0].plain_text(), "a");
            assert_eq!(children[1].plain_text(), "b");
            assert_eq!(children[2].plain_text(), "c");
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_aceita_str_como_child() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("hello".into())]);
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { children, .. }) = r {
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].plain_text(), "hello");
        } else {
            panic!("esperado Content::Stack");
        }
    }

    #[test]
    fn native_stack_rejeita_dir_invalido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("dir".into(), Value::Str("middle".into()));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "dir inválido deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_spacing_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("spacing".into(), Value::Length(Length::pt(-1.0)));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "spacing negativo deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("baseline".into(), Value::Bool(true));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em stack() deve retornar Err");
    }

    #[test]
    fn native_stack_rejeita_child_nao_content() {
        null_ctx!(ctx);
        let r = native_stack(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id(), None);
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
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Stack { children, dir, spacing }) = r {
            assert_eq!(children.len(), 2);
            assert_eq!(dir, Dir::LTR);
            assert_eq!(spacing, Some(Length::pt(4.0)));
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Block { .. })),
            "regressão: native_block deveria produzir Content::Block");
        // Pad regression
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(5.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad { .. })),
            "regressão: native_pad deveria produzir Content::Pad");
        // Hide regression
        let r = native_hide(&mut ctx, &p(vec![Value::Content(Content::text("y"))]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Hide { .. })),
            "regressão: native_hide deveria produzir Content::Hide");
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Block { .. })));
        // Box (Boxed)
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Boxed { .. })));
        // Pad
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad { .. })));
        // Hide
        let r = native_hide(&mut ctx, &p(vec![Value::Content(Content::text("y"))]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Hide { .. })));
    }

    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ────────────────

    #[test]
    fn native_repeat_defaults_gap_none_justify_true() {
        null_ctx!(ctx);
        let r = native_repeat(&mut ctx, &p(vec![Value::Content(Content::text("."))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Repeat { body, gap, justify }) = r {
            assert_eq!(body.plain_text(), ".");
            assert_eq!(gap, None);
            assert!(justify, "default justify == true (paridade vanilla)");
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_aceita_str_como_body() {
        null_ctx!(ctx);
        let r = native_repeat(&mut ctx, &p(vec![Value::Str(".".into())]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Repeat { body, .. }) = r {
            assert_eq!(body.plain_text(), ".");
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
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Repeat { gap, .. }) = r {
            assert_eq!(gap, Some(Length::pt(5.0)));
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_com_justify_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("justify".into(), Value::Bool(false));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Repeat { justify, .. }) = r {
            assert!(!justify);
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
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Repeat { body, gap, justify }) = r {
            assert_eq!(body.plain_text(), "o");
            assert_eq!(gap, Some(Length::pt(2.0)));
            assert!(!justify);
        } else {
            panic!("esperado Content::Repeat");
        }
    }

    #[test]
    fn native_repeat_rejeita_sem_body() {
        null_ctx!(ctx);
        let r = native_repeat(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "repeat() sem body deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_body_int() {
        null_ctx!(ctx);
        let r = native_repeat(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "repeat() com body Int deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_gap_negativo() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("gap".into(), Value::Length(Length::pt(-1.0)));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "gap negativo deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_gap_nao_length() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("gap".into(), Value::Str("x".into()));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "gap não-length deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_justify_nao_bool() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("justify".into(), Value::Int(1));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "justify não-bool deve retornar Err");
    }

    #[test]
    fn native_repeat_rejeita_named_arg_desconhecido() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("."))]);
        args.named.insert("count".into(), Value::Int(3));
        let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id(), None);
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
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Stack { .. })));
        // Block
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Block { .. })));
        // Box
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Boxed { .. })));
        // Pad
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Pad { .. })));
        // Hide
        let r = native_hide(&mut ctx, &p(vec![Value::Content(Content::text("y"))]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Content(Content::Hide { .. })));
    }
}
