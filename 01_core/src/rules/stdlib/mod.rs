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
    native_divider, native_emph, native_heading, native_raw, native_strong, native_terms,
};
pub use crate::rules::stdlib::figure_image::{native_figure, native_image};
pub use crate::rules::stdlib::shapes::{
    native_circle, native_ellipse, native_line, native_polygon, native_rect,
};
pub use crate::rules::stdlib::transforms::{native_move, native_rotate, native_scale};
pub use crate::rules::stdlib::layout::{native_align, native_grid, native_page, native_place};

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
}
