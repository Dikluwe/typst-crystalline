//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
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
    native_cmyk, native_counter_at, native_counter_display, native_counter_final, native_counter_step, native_float, native_here, native_hsl, native_hsv, native_int, native_len, native_linear_rgb, native_locate, native_luma, native_metadata, native_oklab, native_oklch, native_query, native_range, native_rgb,
    native_state, native_state_at, native_state_display, native_state_final, native_state_update, native_state_update_with, native_str, native_type,
};
pub use crate::rules::stdlib::calc::make_calc_module;
pub use crate::rules::stdlib::text::{native_lower, native_replace, native_upper};
pub use crate::rules::stdlib::assert::native_assert;
pub use crate::rules::stdlib::structural::{
    native_bibliography, native_cite, native_divider, native_emph, native_grid_cell, native_grid_footer, native_grid_header, native_heading, native_quote, native_raw, native_strong, native_table, native_table_cell, native_table_footer, native_table_header, native_terms,
};
pub use crate::rules::stdlib::figure_image::{native_figure, native_image};
pub use crate::rules::stdlib::shapes::{
    native_circle, native_ellipse, native_line, native_polygon, native_rect,
};
pub use crate::rules::stdlib::transforms::{native_move, native_rotate, native_scale, native_skew};
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_colbreak, native_columns, native_grid, native_h,
    native_hide, native_measure, native_pad, native_page, native_pagebreak, native_place,
    native_repeat, native_stack, native_stroke, native_v,
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
        // P257 (ADR-0083 PROPOSTO) — `luma()` agora constrói
        // `Color::Luma` (paridade vanilla D65Gray); paridade
        // observable preservada via `to_srgb()` que expande Luma
        // para sRGB cinza bit-equivalente.
        null_ctx!(ctx);
        let r = native_luma(&mut ctx, &p(vec![Value::Int(128)]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, a_) = c.to_srgb();
            assert_eq!(r_, 128);
            assert_eq!(g_, 128);
            assert_eq!(b_, 128);
            assert_eq!(a_, 255);
        } else { panic!("esperado Value::Color"); }
    }

    // ── P257 (ADR-0083 PROPOSTO) — stdlib funcs novas para espaços
    //     materializados (oklab/oklch/linear_rgb/cmyk/hsl/hsv) ──

    #[test]
    fn p257_native_oklab_3_args() {
        null_ctx!(ctx);
        let r = native_oklab(&mut ctx, &p(vec![
            Value::Float(0.5), Value::Float(0.0), Value::Float(0.0),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            // L=0.5 → cinza médio aproximado.
            let (r_, g_, b_, _) = c.to_srgb();
            assert!(r_ > 80 && r_ < 180,
                "oklab(0.5, 0, 0) → cinza médio; obtido r={}", r_);
            // a=b=0 → sem chroma → r ≈ g ≈ b.
            assert!((r_ as i32 - g_ as i32).abs() <= 3);
            assert!((g_ as i32 - b_ as i32).abs() <= 3);
        } else { panic!("esperado Value::Color"); }
    }

    #[test]
    fn p257_native_oklab_4_args_com_alpha() {
        null_ctx!(ctx);
        let r = native_oklab(&mut ctx, &p(vec![
            Value::Float(1.0), Value::Float(0.0), Value::Float(0.0), Value::Float(0.5),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (_, _, _, a_) = c.to_srgb();
            // alpha=0.5 → 127 ou 128 conforme arredondamento.
            assert!(a_ == 127 || a_ == 128, "alpha=0.5 → 127/128; obtido {}", a_);
        } else { panic!("esperado Value::Color"); }
    }

    #[test]
    fn p257_native_oklch_3_args() {
        null_ctx!(ctx);
        let r = native_oklch(&mut ctx, &p(vec![
            Value::Float(0.5), Value::Float(0.0), Value::Float(0.0),
        ]), &null_world(), test_file_id(), None).unwrap();
        assert!(matches!(r, Value::Color(_)));
    }

    #[test]
    fn p257_native_linear_rgb_3_args() {
        null_ctx!(ctx);
        let r = native_linear_rgb(&mut ctx, &p(vec![
            Value::Float(1.0), Value::Float(0.0), Value::Float(0.0),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 0);
            assert_eq!(b_, 0);
        } else { panic!("esperado Value::Color"); }
    }

    #[test]
    fn p257_native_cmyk_branco() {
        null_ctx!(ctx);
        let r = native_cmyk(&mut ctx, &p(vec![
            Value::Float(0.0), Value::Float(0.0), Value::Float(0.0), Value::Float(0.0),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 255);
            assert_eq!(b_, 255);
        } else { panic!("esperado Value::Color"); }
    }

    #[test]
    fn p257_native_cmyk_4_args_obrigatorios() {
        null_ctx!(ctx);
        let r = native_cmyk(&mut ctx, &p(vec![
            Value::Float(0.0), Value::Float(0.0), Value::Float(0.0),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "cmyk requer 4 args; 3 args deve falhar");
    }

    #[test]
    fn p257_native_hsl_vermelho_puro() {
        null_ctx!(ctx);
        let r = native_hsl(&mut ctx, &p(vec![
            Value::Float(0.0), Value::Float(1.0), Value::Float(0.5),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 0);
            assert_eq!(b_, 0);
        } else { panic!("esperado Value::Color"); }
    }

    #[test]
    fn p257_native_hsv_branco_s0_v1() {
        null_ctx!(ctx);
        let r = native_hsv(&mut ctx, &p(vec![
            Value::Float(0.0), Value::Float(0.0), Value::Float(1.0),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Color(c) = r {
            let (r_, g_, b_, _) = c.to_srgb();
            assert_eq!(r_, 255);
            assert_eq!(g_, 255);
            assert_eq!(b_, 255);
        } else { panic!("esperado Value::Color"); }
    }

    // ── P175 (M9 sub-passo 5) — query(kind_str) ─────────────────────────

    #[test]
    fn stdlib_query_em_introspector_vazio_retorna_array_vazio() {
        // P175 → P179 upgrade: query retorna Value::Array, não Int.
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Array(vec![]));
    }

    #[test]
    fn stdlib_query_em_introspector_populado_retorna_array_de_locations() {
        // P179 upgrade: array com Value::Location entries.
        null_ctx!(ctx);
        use crate::entities::element_kind::ElementKind;
        use crate::entities::location::Location;
        ctx.introspector.kind_index.entry(ElementKind::Heading).or_default()
            .extend(vec![Location::from_raw(1), Location::from_raw(2), Location::from_raw(3)]);

        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err(), "kind inválido deve retornar Err");
    }

    #[test]
    fn stdlib_query_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(), test_file_id(), None,
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Array(vec![]));
        // Populado.
        ctx.introspector.kind_index.entry(ElementKind::Outline)
            .or_default().push(Location::from_raw(10));
        let r = native_query(
            &mut ctx,
            &p(vec![Value::Str("outline".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
        let r = native_here(
            &mut ctx,
            &p(vec![]),
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err(), "here() sem current_location deve falhar");
        // Erro contém menção a "here()" ou similar — não validar texto exacto.
    }

    #[test]
    fn p208b_here_com_current_location_retorna_value_location() {
        // Setter conveniente: with_current_location.
        use crate::entities::location::Location;
        let mut ctx = EvalContext::new().with_current_location(
            Location::from_raw(42),
        );
        let r = native_here(
            &mut ctx,
            &p(vec![]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Location(Location::from_raw(42)));
        assert_eq!(r.type_name(), "location");
    }

    #[test]
    fn p208b_here_com_args_retorna_err() {
        // here() não aceita argumentos (paridade vanilla: sem args).
        use crate::entities::location::Location;
        let mut ctx = EvalContext::new().with_current_location(
            Location::from_raw(1),
        );
        let r = native_here(
            &mut ctx,
            &p(vec![Value::Int(99)]),
            &null_world(), test_file_id(), None,
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
        let r = native_here(
            &mut ctx,
            &p(vec![]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(20));

        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p208c_locate_kind_invalido_retorna_err() {
        // Kind inválido → erro contextual coerente.
        null_ctx!(ctx);
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into())]),
            &null_world(), test_file_id(), None,
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
            &null_world(), test_file_id(), None,
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
        ctx.introspector.labels.add(
            Label("intro".to_string()),
            Location::from_raw(7),
        );
        let r = native_locate(
            &mut ctx,
            &p(vec![Value::Str("<intro>".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(
            r,
            Value::Array(vec![Value::Location(Location::from_raw(42))]),
        );
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
        ctx.introspector.labels.add(
            Label("a".to_string()),
            Location::from_raw(1),
        );
        let r = ctx.introspector.query(
            &Selector::Label(Label("a".to_string())),
        );
        assert_eq!(r, vec![Location::from_raw(1)]);
        // Label inexistente → vazio.
        let r2 = ctx.introspector.query(
            &Selector::Label(Label("b".to_string())),
        );
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
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(7));
        ctx.introspector.kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector.labels.add(
            Label("a".to_string()),
            Location::from_raw(7),
        );

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
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(1));
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(2));
        ctx.introspector.kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(3));

        let r = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Kind(ElementKind::Figure),
        ])));
        assert_eq!(
            r,
            vec![
                Location::from_raw(1),
                Location::from_raw(2),
                Location::from_raw(3),
            ],
        );

        // Dedup: Or com mesma kind 2× não duplica.
        let r_dedup = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
            Selector::Kind(ElementKind::Heading),
            Selector::Kind(ElementKind::Heading),
        ])));
        assert_eq!(
            r_dedup,
            vec![Location::from_raw(1), Location::from_raw(2)],
        );
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        match r {
            Value::Content(Content::CounterUpdate { key, action }) => {
                assert_eq!(key, "foo");
                assert_eq!(action, CounterAction::Step);
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
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err(), "arg não-string deve falhar");
    }

    #[test]
    fn p210b_counter_step_sem_args_retorna_err() {
        null_ctx!(ctx);
        let r = native_counter_step(
            &mut ctx,
            &p(vec![]),
            &null_world(), test_file_id(), None,
        );
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        let r2 = native_counter_step(
            &mut ctx,
            &p(vec![Value::Str("h".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        // PartialEq sobre Content::CounterUpdate (per content.rs:1203).
        match (r1, r2) {
            (Value::Content(c1), Value::Content(c2)) => {
                assert!(matches!(
                    (&c1, &c2),
                    (Content::CounterUpdate { .. }, Content::CounterUpdate { .. })
                ));
                // Via match interno + comparison.
                let same = match (&c1, &c2) {
                    (Content::CounterUpdate { key: k1, action: a1 },
                     Content::CounterUpdate { key: k2, action: a2 }) => {
                        k1 == k2 && a1 == a2
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
        let r = ctx.introspector.query(&Selector::Regex(
            Regex::new("\\d+").unwrap(),
        ));
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
        ctx.introspector.kind_index
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
        ctx.introspector.kind_index
            .entry(ElementKind::Heading)
            .or_default()
            .push(Location::from_raw(7));
        ctx.introspector.kind_index
            .entry(ElementKind::Figure)
            .or_default()
            .push(Location::from_raw(10));
        ctx.introspector.labels.add(
            Label("a".to_string()),
            Location::from_raw(7),
        );

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

    // ── P176 (M9 sub-passo 6) — counter_final(key) ──────────────────────

    #[test]
    fn stdlib_counter_final_em_introspector_vazio_retorna_str_vazia() {
        null_ctx!(ctx);
        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_final_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_counter_final(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(), test_file_id(), None,
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_at_label_inexistente_retorna_str_vazia() {
        null_ctx!(ctx);
        // Popular counter mas label não registada.
        use crate::entities::location::Location;
        ctx.introspector.counters.apply_hierarchical_at(
            "heading".to_string(), 1, Location::from_raw(10),
        );
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Str("nonexistent".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Str("".into()));
    }

    #[test]
    fn stdlib_counter_at_label_associado_retorna_string_formatada() {
        null_ctx!(ctx);
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        // Popular: heading na loc 10 com counter [1], label "intro" → loc 10.
        ctx.introspector.counters.apply_hierarchical_at(
            "heading".to_string(), 1, Location::from_raw(10),
        );
        ctx.introspector.labels.add(Label("intro".to_string()), Location::from_raw(10));

        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Str("intro".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::Str("1".into()));
    }

    #[test]
    fn stdlib_counter_at_args_invalidos_retornam_err() {
        null_ctx!(ctx);
        // Primeiro arg não-string.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Int(1), Value::Str("intro".into())]),
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err());
        // Segundo arg não-string.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into()), Value::Int(1)]),
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err());
        // Número errado de args.
        let r = native_counter_at(
            &mut ctx,
            &p(vec![Value::Str("heading".into())]),
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err());
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

    // ── Passo 158A — auto-detecção de kind em native_figure ─────────────

    #[test]
    fn figure_auto_detect_image() {
        // P158A: figure(image(...)) sem `kind:` → kind="image"
        // via inferência (não via default).
        null_ctx!(ctx);
        use crate::entities::content::Content;
        use crate::entities::ptr_eq_arc::PtrEqArc;
        use std::sync::Arc;
        let img = Content::Image {
            path:   "a.png".into(),
            data:   PtrEqArc(Arc::new(Vec::new())),
            width:  None,
            height: None,
        };
        let args = p(vec![Value::Content(img)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert_eq!(kind.as_deref(), Some("image"), "auto-detect Image → kind=Some(\"image\")");
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
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert_eq!(kind.as_deref(), Some("table"), "auto-detect Table → kind=Some(\"table\")");
        } else {
            panic!("esperado Content::Figure");
        }
    }

    #[test]
    fn figure_auto_detect_raw() {
        // P158A: figure(raw(...)) sem `kind:` → kind=Some("raw").
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let raw = Content::Raw {
            text:  "fn x() {}".into(),
            lang:  None,
            block: false,
        };
        let args = p(vec![Value::Content(raw)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert_eq!(kind.as_deref(), Some("raw"), "auto-detect Raw → kind=Some(\"raw\")");
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
        let img = Content::Image {
            path:   "a.png".into(),
            data:   PtrEqArc(Arc::new(Vec::new())),
            width:  None,
            height: None,
        };
        let args = pn(vec![Value::Content(img)], "kind", Value::Str("custom-kind".into()));
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert_eq!(kind.as_deref(), Some("custom-kind"),
                "kind explícito vence auto-detecção (precedência absoluta)");
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
        let img = Content::Image {
            path:   "a.png".into(),
            data:   PtrEqArc(Arc::new(Vec::new())),
            width:  None,
            height: None,
        };
        let args = pn(vec![Value::Content(img)], "kind", Value::Auto);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert!(kind.is_none(),
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
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert!(kind.is_none(),
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
        let img = Content::Image {
            path:   "a.png".into(),
            data:   PtrEqArc(Arc::new(Vec::new())),
            width:  None,
            height: None,
        };
        // Sequence começa com Text (não detectável) e contém Image.
        let seq = Content::Sequence(Arc::from(vec![
            Content::text("legenda"),
            img,
        ]));
        let args = p(vec![Value::Content(seq)]);
        let r = native_figure(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Figure { kind, .. }) = r {
            assert_eq!(kind.as_deref(), Some("image"),
                "Sequence com Image dentro auto-detecta Some(\"image\") via recursão");
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

    // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──

    #[test]
    fn native_pad_defaults_sides_none() {
        // P156L: sem named args → todos os sides são None (per
        // ADR-0064 Caso C; None ↔ default vanilla zero resolvido em uso).
        null_ctx!(ctx);
        let body = Content::text("body");
        let args = p(vec![Value::Content(body)]);
        let result = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { body, sides }) = result {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(sides.left,   None);
            assert_eq!(sides.right,  None);
            assert_eq!(sides.top,    None);
            assert_eq!(sides.bottom, None);
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
        if let Value::Content(Content::Pad { sides, .. }) = result {
            assert_eq!(sides.left,   Some(Length::pt(1.0)));
            assert_eq!(sides.right,  Some(Length::pt(2.0)));
            assert_eq!(sides.top,    Some(Length::pt(3.0)));
            assert_eq!(sides.bottom, Some(Length::pt(4.0)));
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
        if let Value::Content(Content::Pad { sides, .. }) = result {
            assert_eq!(sides.left,   Some(Length::pt(5.0)));
            assert_eq!(sides.right,  Some(Length::pt(5.0)));
            assert_eq!(sides.top,    Some(Length::pt(7.0)));
            assert_eq!(sides.bottom, Some(Length::pt(7.0)));
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
        if let Value::Content(Content::Pad { sides, .. }) = result {
            assert_eq!(sides.left,   Some(Length::pt(8.0)));
            assert_eq!(sides.right,  Some(Length::pt(8.0)));
            assert_eq!(sides.top,    Some(Length::pt(8.0)));
            assert_eq!(sides.bottom, Some(Length::pt(8.0)));
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
        if let Value::Content(Content::Pad { sides, .. }) = result {
            // left vence (específico)
            assert_eq!(sides.left,   Some(Length::pt(1.0)));
            // right cai para x (eixo)
            assert_eq!(sides.right,  Some(Length::pt(2.0)));
            // top cai para rest (não há y nem específico)
            assert_eq!(sides.top,    Some(Length::pt(3.0)));
            assert_eq!(sides.bottom, Some(Length::pt(3.0)));
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
        assert!(result.is_err(), "padding negativo deve retornar Err em P156C/L");
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
        if let Value::Content(Content::Pad { sides, .. }) = r {
            assert_eq!(sides.left, Some(Length::pt(2.5)));
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
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { sides, .. }) = r {
            assert_eq!(sides.top,    Some(Length::pt(7.0)));
            assert_eq!(sides.left,   None);
            assert_eq!(sides.right,  None);
            assert_eq!(sides.bottom, None);
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
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { sides, .. }) = r {
            assert_eq!(sides.left,  Some(Length::ZERO),
                "left explicitamente declarado a zero é Some(ZERO), não None");
            assert_eq!(sides.right, None,
                "right não declarado é None, não Some(ZERO)");
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
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { sides, .. }) = r {
            assert_eq!(sides.left,   Some(Length::pt(4.0)));
            assert_eq!(sides.right,  Some(Length::pt(4.0)));
            assert_eq!(sides.top,    None);
            assert_eq!(sides.bottom, None);
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
        args.named.insert("top".into(),  Value::Length(Length::pt(10.0)));
        args.named.insert("y".into(),    Value::Length(Length::pt(20.0)));
        args.named.insert("rest".into(), Value::Length(Length::pt(30.0)));
        let r = native_pad(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Pad { sides, .. }) = r {
            assert_eq!(sides.top,    Some(Length::pt(10.0)),
                "top específico vence y e rest");
            assert_eq!(sides.bottom, Some(Length::pt(20.0)),
                "bottom cai para y (sem específico)");
            assert_eq!(sides.left,   Some(Length::pt(30.0)),
                "left cai para rest (sem específico nem x)");
            assert_eq!(sides.right,  Some(Length::pt(30.0)));
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
        if let Value::Content(Content::Block { body, width, height, inset, breakable, .. }) = r {
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
        // P247 — fill aceito (Color); Str("red") ainda inválido (tipo
        // errado). Test preserva intent: rejeitar tipo errado para fill.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill com tipo errado (Str em vez de Color) deve retornar Err");
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { fill, .. }) = r {
            assert_eq!(fill, Some(Color::rgb(255, 0, 0)));
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { stroke, .. }) = r {
            let s = stroke.expect("stroke deveria ser Some");
            assert_eq!(s.thickness, 2.0);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p247_native_block_fill_default_none() {
        null_ctx!(ctx);
        let r = native_block(&mut ctx, &p(vec![Value::Content(Content::text("x"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { fill, stroke, .. }) = r {
            assert_eq!(fill, None);
            assert!(stroke.is_none());
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p247_native_block_fill_tipo_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill com Bool deve retornar Err");
    }

    #[test]
    fn p247_native_box_aceita_fill_color() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { fill, .. }) = r {
            assert_eq!(fill, Some(Color::rgb(0, 255, 0)));
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_box_aceita_stroke_color_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { stroke, .. }) = r {
            let s = stroke.expect("stroke deveria ser Some");
            assert_eq!(s.paint, Color::rgb(0, 0, 255));
            assert_eq!(s.thickness, 1.0, "Color shorthand default 1pt thickness");
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_box_fill_default_none() {
        null_ctx!(ctx);
        let r = native_box(&mut ctx, &p(vec![Value::Content(Content::text("x"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { fill, stroke, .. }) = r {
            assert_eq!(fill, None);
            assert!(stroke.is_none());
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p247_native_block_combina_fill_stroke_radius_clip_outset() {
        null_ctx!(ctx);
        use crate::entities::layout_types::{Color, Length};
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("fill".into(),   Value::Color(Color::rgb(50, 100, 150)));
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.5)));
        args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
        args.named.insert("clip".into(),   Value::Bool(true));
        args.named.insert("outset".into(), Value::Length(Length::pt(2.0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { fill, stroke, radius, clip, outset, .. }) = r {
            assert_eq!(fill,   Some(Color::rgb(50, 100, 150)));
            assert_eq!(stroke.unwrap().thickness, 1.5);
            assert_eq!(radius.top_left, Length::pt(3.0));
            assert_eq!(clip,   true);
            assert_eq!(outset.left, Length::pt(2.0));
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { breakable, .. }) = r {
            assert_eq!(breakable, false,
                "P248 — native_block(breakable: false) propaga literal para variant");
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p248_native_block_breakable_default_true_p156g() {
        null_ctx!(ctx);
        let r = native_block(&mut ctx, &p(vec![Value::Content(Content::text("x"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { breakable, .. }) = r {
            assert_eq!(breakable, true, "P248 — default preservado P156G");
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
        args.named.insert("clip".into(),   Value::Bool(true));
        let r = native_box(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { height, clip, .. }) = r {
            assert_eq!(height, Some(Length::pt(50.0)));
            assert_eq!(clip,   true);
        } else {
            panic!("esperado Content::Boxed");
        }
    }

    #[test]
    fn p248_native_box_height_default_none_p156h() {
        null_ctx!(ctx);
        let r = native_box(&mut ctx, &p(vec![Value::Content(Content::text("x"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { height, clip, .. }) = r {
            assert_eq!(height, None);
            assert_eq!(clip,   false, "P248 — defaults preservados P156H");
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
        args.named.insert("above".into(),   Value::Length(Length::pt(20.0)));
        args.named.insert("below".into(),   Value::Length(Length::pt(8.0)));
        args.named.insert("sticky".into(),  Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { spacing, above, below, sticky, .. }) = r {
            assert_eq!(spacing, Some(Length::pt(12.0)));
            assert_eq!(above,   Some(Length::pt(20.0)));
            assert_eq!(below,   Some(Length::pt(8.0)));
            assert_eq!(sticky,  true);
        } else {
            panic!("esperado Content::Block");
        }
    }

    #[test]
    fn p250_native_block_defaults_4_fields() {
        null_ctx!(ctx);
        let r = native_block(&mut ctx, &p(vec![Value::Content(Content::text("x"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { spacing, above, below, sticky, .. }) = r {
            assert_eq!(spacing, None);
            assert_eq!(above,   None);
            assert_eq!(below,   None);
            assert_eq!(sticky,  false, "P250 defaults preservam pre-P250");
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "P250 spacing negativo deve retornar Err");
    }

    #[test]
    fn p250_native_block_sticky_nao_bool_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("sticky".into(), Value::Int(1));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "P250 sticky Int em vez de Bool deve retornar Err");
    }

    #[test]
    fn p250_native_block_above_tipo_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("above".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None);
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
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { stroke: Some(s), .. }) = r {
            assert_eq!(s.thickness, 2.5);
            assert_eq!(s.overhang, true,
                "P252 — Length atalho default overhang=true (paridade vanilla)");
        } else {
            panic!("esperado Content::Block com stroke");
        }
    }

    #[test]
    fn p252_native_block_stroke_color_atalho_overhang_default_true() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_block(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { stroke: Some(s), .. }) = r {
            assert_eq!(s.paint, Color::rgb(255, 0, 0));
            assert_eq!(s.thickness, 1.0);
            assert_eq!(s.overhang, true,
                "P252 — Color atalho default overhang=true (paridade vanilla)");
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
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
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
        let r = native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.overhang, true,
                "P252 — native_stroke default vanilla overhang=true");
        } else { panic!("esperado Value::Stroke"); }
    }

    #[test]
    fn p252_native_stroke_overhang_nao_bool_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("overhang".into(), Value::Int(1));
        let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "P252 overhang com Int em vez de Bool deve retornar Err");
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
        if let Value::Content(Content::Boxed { body, width, height, inset, baseline, .. }) = r {
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

    // ── P218 (DEBT-56 sub-fase b — Layout Fase 3) — columns ──────────

    #[test]
    fn p218_native_columns_count_valido_sem_gutter() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(2),
            Value::Content(Content::text("hello")),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Columns { count, gutter, body }) = r {
            assert_eq!(count, 2);
            assert_eq!(gutter, None);
            assert_eq!(body.plain_text(), "hello");
        } else {
            panic!("esperado Content::Columns");
        }
    }

    #[test]
    fn p218_native_columns_count_zero_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(0),
            Value::Content(Content::text(".")),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "count = 0 deve falhar");
    }

    #[test]
    fn p218_native_columns_count_negativo_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(-1),
            Value::Content(Content::text(".")),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "count = -1 deve falhar");
    }

    #[test]
    fn p218_native_columns_count_nao_int_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Str("foo".into()),
            Value::Content(Content::text(".")),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "count Str deve falhar");
    }

    #[test]
    fn p218_native_columns_count_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "sem args deve falhar");
    }

    #[test]
    fn p218_native_columns_body_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(2),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "só count sem body deve falhar");
    }

    #[test]
    fn p218_native_columns_body_str_aceita() {
        // Body Value::Str → convertido para Content::text.
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(3),
            Value::Str("texto".into()),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Columns { body, .. }) = r {
            assert_eq!(body.plain_text(), "texto");
        } else {
            panic!("esperado Content::Columns com body de Str");
        }
    }

    #[test]
    fn p218_native_columns_gutter_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![
            Value::Int(2),
            Value::Content(Content::text(".")),
        ]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(10.0)));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Columns { gutter, .. }) = r {
            assert_eq!(gutter, Some(Length::pt(10.0)));
        } else {
            panic!("esperado Content::Columns com gutter Some");
        }
    }

    #[test]
    fn p218_native_columns_gutter_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![
            Value::Int(2),
            Value::Content(Content::text(".")),
        ]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(-1.0)));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "gutter negativo deve falhar");
    }

    #[test]
    fn p218_native_columns_named_arg_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![
            Value::Int(2),
            Value::Content(Content::text(".")),
        ]);
        args.named.insert("foo".into(), Value::Int(42));
        let r = native_columns(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido deve falhar");
    }

    #[test]
    fn p218_native_columns_extra_positional_rejeita() {
        null_ctx!(ctx);
        let r = native_columns(&mut ctx, &p(vec![
            Value::Int(2),
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),  // extra
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), ">2 posicionais deve falhar");
    }

    // ── P220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak ───────────────

    #[test]
    fn p220_native_colbreak_sem_args_aceita() {
        null_ctx!(ctx);
        let r = native_colbreak(&mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Colbreak { weak }) = r {
            assert_eq!(weak, false, "default weak == false");
        } else {
            panic!("esperado Value::Content(Content::Colbreak)");
        }
    }

    #[test]
    fn p220_native_colbreak_weak_true_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Bool(true));
        let r = native_colbreak(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Colbreak { weak }) = r {
            assert_eq!(weak, true);
        } else {
            panic!("esperado Value::Content(Content::Colbreak {{ weak: true }})");
        }
    }

    #[test]
    fn p220_native_colbreak_posicional_rejeita() {
        null_ctx!(ctx);
        let r = native_colbreak(&mut ctx, &p(vec![Value::Str("oops".into())]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "posicional deve falhar");
    }

    #[test]
    fn p220_native_colbreak_weak_nao_bool_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("weak".into(), Value::Str("true".into()));
        let r = native_colbreak(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "weak não-Bool deve falhar");
    }

    #[test]
    fn p220_native_colbreak_named_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_colbreak(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named desconhecido deve falhar");
    }

    #[test]
    fn p220_native_colbreak_to_rejeita() {
        // Paridade Pagebreak mas SEM `to` em colbreak (vanilla
        // ColbreakElem não tem; paridade só faz sentido em páginas).
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("to".into(), Value::Str("even".into()));
        let r = native_colbreak(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "to: deve falhar (named desconhecido)");
    }

    // ── P222 (Fase 4 Layout candidata sub-1; ADR-0066 Bloco C) — measure ──

    #[test]
    fn p222_native_measure_body_content_aceita() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![
            Value::Content(Content::text("texto")),
        ]), &null_world(), test_file_id(), None).unwrap();
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
        let r = native_measure(&mut ctx, &p(vec![
            Value::Str("texto".into()),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Dict(d) = r {
            assert!(d.contains_key("width") && d.contains_key("height"));
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_body_ausente_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "body ausente deve falhar");
    }

    #[test]
    fn p222_native_measure_body_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![Value::Int(42)]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "body Int deve falhar");
    }

    #[test]
    fn p222_native_measure_extra_positional_rejeita() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), ">1 posicional deve falhar");
    }

    #[test]
    fn p222_native_measure_named_arg_rejeita() {
        // Opção β graded: width override scope-out per ADR-0054.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
        let r = native_measure(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg width deve falhar (scope-out graded)");
    }

    #[test]
    fn p222_native_measure_retorna_dict_com_width_height() {
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![
            Value::Content(Content::text("x")),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Dict(d) = r {
            assert!(matches!(d.get("width"),  Some(Value::Length(_))),
                "key 'width' deve ser Value::Length");
            assert!(matches!(d.get("height"), Some(Value::Length(_))),
                "key 'height' deve ser Value::Length");
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
        use crate::entities::layout_types::Length;
        use crate::entities::geometry::ShapeKind;
        let rect = Content::Shape {
            kind:   ShapeKind::Rect,
            width:  Some(Box::new(Value::Length(Length::pt(40.0)))),
            height: Some(Box::new(Value::Length(Length::pt(20.0)))),
            fill:   None,
            stroke: None,
        };
        let r = native_measure(&mut ctx, &p(vec![Value::Content(rect)]),
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Dict(d) = r {
            if let Some(Value::Length(w)) = d.get("width") {
                assert!(w.abs.0 > 0.0, "rect width > 0 esperado");
            } else { panic!("width não-Length"); }
            if let Some(Value::Length(h)) = d.get("height") {
                assert!(h.abs.0 > 0.0, "rect height > 0 esperado");
            } else { panic!("height não-Length"); }
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn p222_native_measure_dimensoes_zero_para_empty() {
        // Content::Empty → helper retorna (0, 0).
        null_ctx!(ctx);
        let r = native_measure(&mut ctx, &p(vec![
            Value::Content(Content::Empty),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Dict(d) = r {
            if let Some(Value::Length(w)) = d.get("width") {
                assert_eq!(w.abs.0, 0.0);
            } else { panic!("width não-Length"); }
            if let Some(Value::Length(h)) = d.get("height") {
                assert_eq!(h.abs.0, 0.0);
            } else { panic!("height não-Length"); }
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
        use std::sync::Arc;
        use crate::entities::layout_types::Length;
        use crate::entities::geometry::ShapeKind;
        let r1 = Content::Shape {
            kind:   ShapeKind::Rect,
            width:  Some(Box::new(Value::Length(Length::pt(30.0)))),
            height: Some(Box::new(Value::Length(Length::pt(10.0)))),
            fill:   None,
            stroke: None,
        };
        let r2 = Content::Shape {
            kind:   ShapeKind::Rect,
            width:  Some(Box::new(Value::Length(Length::pt(50.0)))),
            height: Some(Box::new(Value::Length(Length::pt(15.0)))),
            fill:   None,
            stroke: None,
        };
        let seq = Content::Sequence(Arc::from(vec![r1, r2]));
        let r = native_measure(&mut ctx, &p(vec![Value::Content(seq)]),
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Dict(d) = r {
            // max_w = 50, total_h = 25.
            if let Some(Value::Length(w)) = d.get("width") {
                assert!(w.abs.0 >= 50.0,
                    "Sequence max width >= 50, recebeu {}", w.abs.0);
            } else { panic!("width não-Length"); }
            if let Some(Value::Length(h)) = d.get("height") {
                assert!(h.abs.0 >= 25.0,
                    "Sequence total height >= 25, recebeu {}", h.abs.0);
            } else { panic!("height não-Length"); }
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
        use crate::entities::layout_types::Length;
        use crate::entities::geometry::ShapeKind;
        let rect = Content::Shape {
            kind:   ShapeKind::Rect,
            width:  Some(Box::new(Value::Length(Length::pt(20.0)))),
            height: Some(Box::new(Value::Length(Length::pt(40.0)))),
            fill:   None,
            stroke: None,
        };
        let r = native_measure(&mut ctx, &p(vec![Value::Content(rect)]),
            &null_world(), test_file_id(), None).unwrap();
        // Paridade vanilla: `dims.width` retorna Length.
        if let Value::Dict(d) = &r {
            let w = d.get("width").cloned().expect("key 'width' presente");
            let h = d.get("height").cloned().expect("key 'height' presente");
            assert!(matches!(w, Value::Length(_)),
                "width deve indexar como Length (paridade observable vanilla)");
            assert!(matches!(h, Value::Length(_)),
                "height deve indexar como Length");
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
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Place { float, .. }) = r {
            assert_eq!(float, true);
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_float_default_false() {
        // place(body) sem float → float == false.
        null_ctx!(ctx);
        let r = native_place(&mut ctx, &p(vec![
            Value::Content(Content::text("a")),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Place { float, .. }) = r {
            assert_eq!(float, false, "default float == false");
        } else {
            panic!("esperado Content::Place");
        }
    }

    #[test]
    fn p223_native_place_float_nao_bool_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("float".into(), Value::Str("yes".into()));
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "float não-Bool deve falhar");
    }

    #[test]
    fn p223_native_place_clearance_length_aceita() {
        // place(top, float: true, clearance: 5pt, body) → Some(5pt).
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("float".into(),     Value::Bool(true));
        args.named.insert("clearance".into(), Value::Length(Length::pt(5.0)));
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Place { clearance, .. }) = r {
            assert_eq!(clearance, Some(Length::pt(5.0)));
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
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "clearance negativo deve falhar");
    }

    #[test]
    fn p223_native_place_parent_sem_float_rejeita() {
        // DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("scope".into(), Value::Str("parent".into()));
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(),
            "scope 'parent' sem float deve falhar (paridade vanilla; DEBT-37 fechada)");
    }

    #[test]
    fn p223_native_place_parent_com_float_aceita() {
        // place(scope: "parent", float: true, body) → OK.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("scope".into(), Value::Str("parent".into()));
        args.named.insert("float".into(), Value::Bool(true));
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        use crate::entities::layout_types::PlaceScope;
        if let Value::Content(Content::Place { scope, float, .. }) = r {
            assert!(matches!(scope, PlaceScope::Parent));
            assert_eq!(float, true);
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
        let r = native_place(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_ok(), "scope 'column' sem float OK (não tem restrição vanilla)");
    }

    // ── P224 (Fase 4 Layout candidata sub-3) — grid refino + grid_cell/header/footer ──

    #[test]
    fn p224_native_grid_aceita_gutter() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(5.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { gutter, .. }) = r {
            assert_eq!(gutter, Some(Length::pt(5.0)));
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p224_native_grid_gutter_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("gutter".into(), Value::Length(Length::pt(-5.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "gutter negativo deve falhar");
    }

    #[test]
    fn p224_native_grid_inset_uniforme_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { inset, .. }) = r {
            assert_eq!(inset.left, Length::pt(3.0));
            assert_eq!(inset.right, Length::pt(3.0));
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p224_native_grid_header_footer_content_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("header".into(), Value::Content(Content::text("HDR")));
        args.named.insert("footer".into(), Value::Content(Content::text("FTR")));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { header, footer, .. }) = r {
            assert!(header.is_some(), "header presente");
            assert!(footer.is_some(), "footer presente");
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p224_native_grid_named_arg_desconhecido_rejeita() {
        // stroke/fill cosméticos scope-out — desconhecidos rejeitados.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Str("black".into()));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "stroke scope-out deve falhar");
    }

    #[test]
    fn p224_native_grid_cell_body_obrigatorio() {
        null_ctx!(ctx);
        let r = native_grid_cell(&mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "body obrigatório");
    }

    #[test]
    fn p224_native_grid_cell_x_y_colspan_rowspan_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("cell"))]);
        args.named.insert("x".into(),       Value::Int(1));
        args.named.insert("y".into(),       Value::Int(0));
        args.named.insert("colspan".into(), Value::Int(2));
        args.named.insert("rowspan".into(), Value::Int(3));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridCell { x, y, colspan, rowspan, .. }) = r {
            assert_eq!(x,       Some(1));
            assert_eq!(y,       Some(0));
            assert_eq!(colspan, Some(2));
            assert_eq!(rowspan, Some(3));
        } else { panic!("esperado GridCell"); }
    }

    #[test]
    fn p224_native_grid_cell_colspan_zero_rejeita() {
        // ADR-0064 Caso C — colspan >= 1.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("colspan".into(), Value::Int(0));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "colspan 0 deve falhar (paridade NonZeroUsize)");
    }

    #[test]
    fn p224_native_grid_header_aceita_body() {
        null_ctx!(ctx);
        let r = native_grid_header(&mut ctx, &p(vec![Value::Content(Content::text("hdr"))]),
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridHeader { body, repeat }) = r {
            assert_eq!(body.plain_text(), "hdr");
            assert_eq!(repeat, true, "default repeat == true (paridade vanilla)");
        } else { panic!("esperado GridHeader"); }
    }

    #[test]
    fn p224_native_grid_footer_aceita_body_repeat_false() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("ftr"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r = native_grid_footer(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridFooter { body, repeat }) = r {
            assert_eq!(body.plain_text(), "ftr");
            assert_eq!(repeat, false);
        } else { panic!("esperado GridFooter"); }
    }

    // ── P227 (Fase 5 Layout Categoria A.1) — Value::Stroke + native_stroke + stroke shorthand ──

    #[test]
    fn p227_value_stroke_type_name_e_eq() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let s = Stroke { paint: Color::rgb(255, 0, 0), thickness: 2.5, overhang: false };
        let v = Value::Stroke(s.clone());
        assert_eq!(v.type_name(), "stroke");
        let v2 = Value::Stroke(s);
        assert_eq!(v, v2);
    }

    #[test]
    fn p227_native_stroke_defaults_aceita() {
        // stroke() sem args → Stroke { BLACK, 1.0pt }.
        null_ctx!(ctx);
        let r = native_stroke(&mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.thickness, 1.0);
        } else { panic!("esperado Value::Stroke"); }
    }

    #[test]
    fn p227_native_stroke_thickness_2pt_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
        let r = native_stroke(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Stroke(s) = r {
            assert_eq!(s.thickness, 2.0);
        } else { panic!("esperado Value::Stroke"); }
    }

    #[test]
    fn p227_native_stroke_thickness_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(-1.0)));
        let r = native_stroke(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "thickness negativo deve falhar");
    }

    #[test]
    fn p227_native_stroke_thickness_zero_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![]);
        args.named.insert("thickness".into(), Value::Length(Length::pt(0.0)));
        let r = native_stroke(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "thickness 0 deve falhar (paridade vanilla)");
    }

    #[test]
    fn p227_native_stroke_named_desconhecido_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_stroke(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named desconhecido deve falhar");
    }

    #[test]
    fn p227_native_stroke_posicional_rejeita() {
        null_ctx!(ctx);
        let r = native_stroke(&mut ctx, &p(vec![Value::Bool(true)]),
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "posicional deve falhar");
    }

    #[test]
    fn p227_native_grid_stroke_length_shorthand() {
        // grid(stroke: 2pt) → Stroke{BLACK, 2pt}.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { stroke, .. }) = r {
            assert!(stroke.is_some());
            assert_eq!(stroke.unwrap().thickness, 2.0);
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p227_native_grid_stroke_color_shorthand() {
        // grid(stroke: red) → Stroke{red, 1pt}.
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { stroke, .. }) = r {
            let s = stroke.expect("stroke presente");
            assert_eq!(s.thickness, 1.0);
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p227_native_grid_stroke_value_stroke_passa() {
        // grid(stroke: stroke(thickness: 3pt)) → preserva Stroke.
        null_ctx!(ctx);
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let s = Stroke { paint: Color::rgb(0, 255, 0), thickness: 3.0, overhang: false };
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Stroke(s.clone()));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { stroke, .. }) = r {
            assert_eq!(stroke, Some(s));
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p227_native_table_stroke_paridade_grid() {
        // table(stroke: 1pt) paridade native_grid.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        let r = native_table(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { stroke, .. }) = r {
            assert!(stroke.is_some());
        } else { panic!("esperado Content::Table"); }
    }

    // ── P228 (Fase 5 Layout Categoria A.2) — fill Grid + Table ──

    #[test]
    fn p228_native_grid_fill_color_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(255, 200, 0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { fill, .. }) = r {
            assert!(fill.is_some());
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p228_native_grid_fill_default_none() {
        null_ctx!(ctx);
        let r = native_grid(&mut ctx, &p(vec![
            Value::Content(Content::text("a")),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { fill, .. }) = r {
            assert!(fill.is_none(), "default fill == None");
        } else { panic!("esperado Content::Grid"); }
    }

    #[test]
    fn p228_native_grid_fill_tipo_errado_rejeita() {
        // fill aceita só Color (não Length); rejeita explicitamente.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
    }

    #[test]
    fn p228_native_table_fill_paridade_grid() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(100, 100, 100)));
        let r = native_table(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { fill, .. }) = r {
            assert!(fill.is_some());
        } else { panic!("esperado Content::Table"); }
    }

    #[test]
    fn p228_native_grid_fill_e_stroke_simultaneos_aceita() {
        // Ambos fill + stroke aceitos no mesmo grid() call.
        null_ctx!(ctx);
        use crate::entities::layout_types::{Length, Color};
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("fill".into(),   Value::Color(Color::rgb(0, 255, 0)));
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Grid { fill, stroke, .. }) = r {
            assert!(fill.is_some() && stroke.is_some());
        } else { panic!("esperado Content::Grid"); }
    }

    // ── P230 (Fase 5 Layout Categoria A.3) — stroke/fill per-cell GridCell + TableCell ──

    #[test]
    fn p230_native_grid_cell_stroke_aceita_length_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridCell { stroke, .. }) = r {
            assert!(stroke.is_some());
            assert_eq!(stroke.unwrap().thickness, 2.0);
        } else { panic!("esperado GridCell"); }
    }

    #[test]
    fn p230_native_grid_cell_stroke_aceita_color_shorthand() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridCell { stroke, .. }) = r {
            assert!(stroke.is_some());
            assert_eq!(stroke.unwrap().thickness, 1.0);
        } else { panic!("esperado GridCell"); }
    }

    #[test]
    fn p230_native_grid_cell_fill_color_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridCell { fill, .. }) = r {
            assert!(fill.is_some());
        } else { panic!("esperado GridCell"); }
    }

    #[test]
    fn p230_native_grid_cell_fill_length_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("fill".into(), Value::Length(Length::pt(1.0)));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "fill Length deve falhar (semantic: fill é Color)");
    }

    #[test]
    fn p230_native_table_cell_paridade_gridcell() {
        // table_cell aceita stroke + fill (paridade native_grid_cell).
        null_ctx!(ctx);
        use crate::entities::layout_types::{Length, Color};
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
        args.named.insert("fill".into(),   Value::Color(Color::rgb(0, 0, 255)));
        let r = native_table_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableCell { stroke, fill, .. }) = r {
            assert!(stroke.is_some() && fill.is_some());
        } else { panic!("esperado TableCell"); }
    }

    #[test]
    fn p230_native_grid_cell_stroke_e_fill_simultaneos() {
        // Ambos aceitos no mesmo grid_cell call.
        null_ctx!(ctx);
        use crate::entities::layout_types::{Length, Color};
        let mut args = p(vec![Value::Content(Content::text("c"))]);
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        args.named.insert("fill".into(),   Value::Color(Color::rgb(100, 100, 100)));
        let r = native_grid_cell(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::GridCell { stroke, fill, .. }) = r {
            assert!(stroke.is_some() && fill.is_some());
        } else { panic!("esperado GridCell"); }
    }

    // ── P231 (Fase 5 Layout Categoria A.4) — Block/Boxed outset/radius/clip ──

    #[test]
    fn p231_native_block_outset_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(5.0)));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { outset, .. }) = r {
            assert_eq!(outset.left, Length::pt(5.0));
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p231_native_block_outset_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Length(Length::pt(-3.0)));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "outset negativo deve falhar");
    }

    #[test]
    fn p231_native_block_radius_length_aceita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { radius, .. }) = r {
            // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
            // Length uniforme via stdlib → `Corners::uniform`.
            assert_eq!(radius.top_left,     Length::pt(3.0));
            assert_eq!(radius.top_right,    Length::pt(3.0));
            assert_eq!(radius.bottom_right, Length::pt(3.0));
            assert_eq!(radius.bottom_left,  Length::pt(3.0));
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p231_native_block_radius_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None);
        assert!(r.is_err(), "radius negativo deve falhar");
    }

    #[test]
    fn p231_native_block_clip_bool_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("clip".into(), Value::Bool(true));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { clip, .. }) = r {
            assert_eq!(clip, true);
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p231_native_block_clip_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("clip".into(), Value::Str("yes".into()));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None);
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
        args.named.insert("clip".into(),   Value::Bool(true));
        let r = native_box(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { outset, radius, clip, .. }) = r {
            assert_eq!(outset.top, Length::pt(2.0));
            // P242 adapta: radius `Corners<Length>` via uniform.
            assert_eq!(radius.top_left, Length::pt(1.0));
            assert_eq!(clip, true);
        } else { panic!("esperado Boxed"); }
    }

    #[test]
    fn p231_native_block_3_fields_simultaneos() {
        // outset + radius + clip + breakable simultâneos.
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(),    Value::Length(Length::pt(1.0)));
        args.named.insert("radius".into(),    Value::Length(Length::pt(2.0)));
        args.named.insert("clip".into(),      Value::Bool(true));
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { outset, radius, clip, breakable, .. }) = r {
            assert_eq!(outset.left, Length::pt(1.0));
            // P242 adapta: radius `Corners<Length>` via uniform.
            assert_eq!(radius.top_left, Length::pt(2.0));
            assert_eq!(clip, true);
            assert_eq!(breakable, false);
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p231_native_block_outset_radius_clip_defaults() {
        // Sem args → outset=Sides::uniform(zero); radius=None; clip=false.
        null_ctx!(ctx);
        let r = native_block(&mut ctx, &p(vec![
            Value::Content(Content::text("body")),
        ]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { outset, radius, clip, .. }) = r {
            assert_eq!(outset.left, crate::entities::layout_types::Length::ZERO);
            // P242 adapta: radius default `Corners::uniform(Length::ZERO)`.
            assert_eq!(radius.top_left,     crate::entities::layout_types::Length::ZERO);
            assert_eq!(radius.top_right,    crate::entities::layout_types::Length::ZERO);
            assert_eq!(radius.bottom_right, crate::entities::layout_types::Length::ZERO);
            assert_eq!(radius.bottom_left,  crate::entities::layout_types::Length::ZERO);
            assert_eq!(clip, false);
        } else { panic!("esperado Block"); }
    }

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table ─────────────────

    #[test]
    fn native_table_defaults_columns_rows_auto() {
        // P157A: defaults — columns/rows omitidos caem em [Auto]
        // (paridade com Grid em P83).
        null_ctx!(ctx);
        use crate::entities::layout_types::TrackSizing;
        let r = native_table(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { columns, rows, children, .. }) = r {
            assert_eq!(columns, vec![TrackSizing::Auto]);
            assert_eq!(rows,    vec![TrackSizing::Auto]);
            assert!(children.is_empty());
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
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { columns, .. }) = r {
            assert_eq!(columns, vec![TrackSizing::Auto, TrackSizing::Auto, TrackSizing::Auto]);
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
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { columns, .. }) = r {
            assert_eq!(columns.len(), 2);
            assert!(matches!(columns[0], TrackSizing::Fixed(v) if (v - 10.0).abs() < 1e-6));
            assert!(matches!(columns[1], TrackSizing::Fixed(v) if (v - 20.0).abs() < 1e-6));
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
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { children, .. }) = r {
            assert_eq!(children.len(), 4);
            assert_eq!(children[0].plain_text(), "a");
            assert_eq!(children[3].plain_text(), "d");
        } else {
            panic!("esperado Content::Table");
        }
    }

    #[test]
    fn native_table_aceita_str_como_child() {
        null_ctx!(ctx);
        let r = native_table(&mut ctx, &p(vec![Value::Str("hello".into())]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Table { children, .. }) = r {
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].plain_text(), "hello");
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
        let r = native_table(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em table() deve retornar Err (atributos avançados scope-out per ADR-0054 graded)");
    }

    #[test]
    fn native_table_rejeita_child_int() {
        null_ctx!(ctx);
        let r = native_table(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id(), None);
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
        let g = native_grid(&mut ctx, &g_args, &null_world(), test_file_id(), None).unwrap();

        let mut t_args = p(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        t_args.named.insert("columns".into(), Value::Int(2));
        let t = native_table(&mut ctx, &t_args, &null_world(), test_file_id(), None).unwrap();

        // Variants diferentes — não são iguais por PartialEq.
        assert_ne!(g, t);
        // Mas ambos têm 2 cells/children e 2 columns.
        if let (Value::Content(Content::Grid { cells: gc, columns: gcols, .. }),
                Value::Content(Content::Table { children: tc, columns: tcols, .. })) = (g, t) {
            assert_eq!(gc.len(), tc.len());
            assert_eq!(gcols.len(), tcols.len());
        } else {
            panic!("esperado Grid + Table");
        }
    }

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table_cell ────────────

    #[test]
    fn native_table_cell_defaults_todos_none() {
        // P157B: defaults — body required; outros fields None.
        null_ctx!(ctx);
        let r = native_table_cell(&mut ctx, &p(vec![Value::Content(Content::text("body"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableCell { body, x, y, colspan, rowspan, .. }) = r {
            assert_eq!(body.plain_text(), "body");
            assert_eq!(x, None);
            assert_eq!(y, None);
            assert_eq!(colspan, None);
            assert_eq!(rowspan, None);
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
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableCell { x, y, .. }) = r {
            assert_eq!(x, Some(2));
            assert_eq!(y, Some(3));
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
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableCell { x, .. }) = r {
            assert_eq!(x, None, "Value::Auto deve traduzir para None per ADR-0064 Caso A");
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
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableCell { colspan, rowspan, .. }) = r {
            assert_eq!(colspan, Some(2));
            assert_eq!(rowspan, Some(3));
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
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "colspan = 0 deve retornar Err (mínimo 1)");
    }

    #[test]
    fn native_table_cell_colspan_negativo_rejeitado() {
        // P157B: int negativo rejeitado.
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("colspan".into(), Value::Int(-1));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "colspan negativo deve retornar Err");
    }

    #[test]
    fn native_table_cell_x_negativo_rejeitado() {
        // P157B: x negativo rejeitado (mínimo 0; sem ints negativos
        // mesmo para campos com min=0).
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("x".into(), Value::Int(-1));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "x negativo deve retornar Err");
    }

    #[test]
    fn native_table_cell_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        // P235 — `inset` agora conhecido (Categoria B.3); test usa
        // `outset` que continua scope-out per ADR-0054 graded.
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("outset".into(), Value::Int(5));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em table_cell() deve retornar Err");
    }

    #[test]
    fn native_table_cell_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_cell(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "table_cell() sem body deve retornar Err");
    }

    // ── Passo 235 (Fase 5 Layout Categoria B.3) — algorítmicos per-cell ──

    #[test]
    fn p235_native_grid_cell_align_aceita() {
        use crate::entities::layout_types::Align2D;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("align".into(), Value::Align(Align2D::from_string("center")));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        match r {
            Ok(Value::Content(Content::GridCell { align, .. })) => assert!(align.is_some()),
            other => panic!("esperado GridCell align Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_grid_cell_inset_length_uniforme_aceita() {
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(5.0)));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        match r {
            Ok(Value::Content(Content::GridCell { inset, .. })) => assert!(inset.is_some()),
            other => panic!("esperado GridCell inset Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_grid_cell_breakable_bool_aceita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("breakable".into(), Value::Bool(false));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        match r {
            Ok(Value::Content(Content::GridCell { breakable, .. })) => assert_eq!(breakable, Some(false)),
            other => panic!("esperado GridCell breakable Some(false), recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_grid_cell_breakable_tipo_errado_rejeita() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("breakable".into(), Value::Int(1));
        let r = native_grid_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "breakable Int (não Bool) deve rejeitar pós-P235");
    }

    #[test]
    fn p235_native_table_cell_align_paridade_gridcell() {
        use crate::entities::layout_types::Align2D;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("align".into(), Value::Align(Align2D::from_string("right")));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        match r {
            Ok(Value::Content(Content::TableCell { align, .. })) => assert!(align.is_some()),
            other => panic!("esperado TableCell align Some, recebeu {:?}", other),
        }
    }

    #[test]
    fn p235_native_table_cell_inset_paridade_gridcell() {
        use crate::entities::layout_types::Length;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("a"))]);
        args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
        let r = native_table_cell(&mut ctx, &args, &null_world(), test_file_id(), None);
        match r {
            Ok(Value::Content(Content::TableCell { inset, .. })) => assert!(inset.is_some()),
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p236_state_final_arg_nao_string_retorna_err() {
        null_ctx!(ctx);
        let r = native_state_final(
            &mut ctx,
            &p(vec![Value::Int(42)]),
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err(), "arg não-string deve retornar Err");
    }

    #[test]
    fn p236_state_final_zero_args_retorna_err() {
        null_ctx!(ctx);
        let r = native_state_final(
            &mut ctx,
            &p(vec![]),
            &null_world(), test_file_id(), None,
        );
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        // Paridade counter_at empty default: state_at retorna Value::None.
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p237_state_at_key_inexistente_retorna_none() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular label + outro state (não a key consultada).
        ctx.introspector.labels.add(Label("intro".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "outra".to_string(), Value::Int(7), Location::from_raw(5),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("inexistente".into()), Value::Str("intro".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        assert_eq!(r, Value::None);
    }

    #[test]
    fn p237_state_at_resolve_label_retorna_init() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        // Popular label "intro" → Location 5; state init em location 1.
        ctx.introspector.labels.add(Label("intro".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "k".to_string(), Value::Int(42), Location::from_raw(1),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("intro".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        // Sem updates entre init e label → init wins.
        assert_eq!(r, Value::Int(42));
    }

    #[test]
    fn p237_state_at_updates_antes_location_visivel() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector.labels.add(Label("at_5".to_string()), Location::from_raw(5));
        ctx.introspector.state.init(
            "k".to_string(), Value::Int(1), Location::from_raw(1),
        );
        // 2 updates antes da location consultada (raw < 5).
        ctx.introspector.state.update(
            "k".to_string(), Value::Int(2), Location::from_raw(2),
        );
        ctx.introspector.state.update(
            "k".to_string(), Value::Int(3), Location::from_raw(3),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("at_5".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        // Valor em location=5 reflecte último update <= 5: Int(3).
        assert_eq!(r, Value::Int(3));
    }

    #[test]
    fn p237_state_at_updates_depois_location_nao_visiveis() {
        use crate::entities::label::Label;
        use crate::entities::location::Location;
        null_ctx!(ctx);
        ctx.introspector.labels.add(Label("at_2".to_string()), Location::from_raw(2));
        ctx.introspector.state.init(
            "k".to_string(), Value::Int(10), Location::from_raw(1),
        );
        // Update em location 5 (depois da location consultada raw=2).
        ctx.introspector.state.update(
            "k".to_string(), Value::Int(99), Location::from_raw(5),
        );
        let r = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("at_2".into())]),
            &null_world(), test_file_id(), None,
        ).unwrap();
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
            &null_world(), test_file_id(), None,
        );
        assert!(r.is_err(), "key não-string deve retornar Err");
        // Label Int (não Str).
        let r2 = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(7)]),
            &null_world(), test_file_id(), None,
        );
        assert!(r2.is_err(), "label não-string deve retornar Err");
    }

    #[test]
    fn p237_state_at_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 = native_state_at(
            &mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None,
        );
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 1 arg.
        let r1 = native_state_at(
            &mut ctx, &p(vec![Value::Str("k".into())]),
            &null_world(), test_file_id(), None,
        );
        assert!(r1.is_err(), "1 arg deve retornar Err");
        // 3 args.
        let r3 = native_state_at(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Str("l".into()), Value::Int(1)]),
            &null_world(), test_file_id(), None,
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
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { radius, .. }) = r {
            assert_eq!(radius.top_left,     Length::pt(5.0));
            assert_eq!(radius.top_right,    Length::pt(5.0));
            assert_eq!(radius.bottom_right, Length::pt(5.0));
            assert_eq!(radius.bottom_left,  Length::pt(5.0));
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p242_native_block_radius_dict_por_canto() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("top-left".into(),     Value::Length(Length::pt(1.0)));
        d.insert("top-right".into(),    Value::Length(Length::pt(2.0)));
        d.insert("bottom-right".into(), Value::Length(Length::pt(3.0)));
        d.insert("bottom-left".into(),  Value::Length(Length::pt(4.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { radius, .. }) = r {
            assert_eq!(radius.top_left,     Length::pt(1.0));
            assert_eq!(radius.top_right,    Length::pt(2.0));
            assert_eq!(radius.bottom_right, Length::pt(3.0));
            assert_eq!(radius.bottom_left,  Length::pt(4.0));
        } else { panic!("esperado Block"); }
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
        d.insert("top".into(),    Value::Length(Length::pt(10.0)));
        // left específico → top-left/bottom-left mas top vence em top-left.
        d.insert("left".into(),   Value::Length(Length::pt(20.0)));
        // rest fallback para o que sobra (bottom-right).
        d.insert("rest".into(),   Value::Length(Length::pt(99.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Block { radius, .. }) = r {
            // top-left: top OR left (precedência top vence: top:10pt).
            assert_eq!(radius.top_left, Length::pt(10.0));
            // top-right: top.
            assert_eq!(radius.top_right, Length::pt(10.0));
            // bottom-left: left vence (bottom não-set; rest fallback).
            assert_eq!(radius.bottom_left, Length::pt(20.0));
            // bottom-right: rest fallback.
            assert_eq!(radius.bottom_right, Length::pt(99.0));
        } else { panic!("esperado Block"); }
    }

    #[test]
    fn p242_native_block_radius_negativo_rejeita() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Length;
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None);
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
        let r = native_block(&mut ctx, &args,
            &null_world(), test_file_id(), None);
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
        d.insert("top-left".into(),  Value::Length(Length::pt(7.0)));
        d.insert("rest".into(),       Value::Length(Length::pt(0.0)));
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("radius".into(), Value::Dict(d));
        let r = native_box(&mut ctx, &args,
            &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Boxed { radius, .. }) = r {
            assert_eq!(radius.top_left,     Length::pt(7.0));
            assert_eq!(radius.top_right,    Length::pt(0.0));
            assert_eq!(radius.bottom_right, Length::pt(0.0));
            assert_eq!(radius.bottom_left,  Length::pt(0.0));
        } else { panic!("esperado Boxed"); }
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        if let Value::Content(Content::StateDisplay { key, callback }) = r {
            assert_eq!(key, "k");
            assert!(callback.is_none(), "1-arg → callback=None");
        } else {
            panic!("esperado Content::StateDisplay");
        }
    }

    #[test]
    fn p240_native_state_display_com_callback_constroi_some() {
        use crate::entities::func::Func;
        null_ctx!(ctx);
        let identity_fn = Func::native("identity", |_, args, _, _, _| {
            Ok(args.items.first().cloned().unwrap_or(Value::None))
        });
        let r = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Func(identity_fn)]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        if let Value::Content(Content::StateDisplay { key, callback }) = r {
            assert_eq!(key, "k");
            assert!(callback.is_some(), "2-arg → callback=Some");
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
            &null_world(), test_file_id(), None,
        );
        assert!(r1.is_err(), "key não-string deve retornar Err");
        // 2-arg segundo arg não-Func.
        let r2 = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(2)]),
            &null_world(), test_file_id(), None,
        );
        assert!(r2.is_err(), "callback não-Func deve retornar Err");
    }

    #[test]
    fn p240_native_state_display_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 = native_state_display(
            &mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None,
        );
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 3 args.
        let r3 = native_state_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
            &null_world(), test_file_id(), None,
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
            &null_world(), test_file_id(), None,
        ).unwrap();
        if let Value::Content(Content::CounterDisplayCallback { key, callback }) = r {
            assert_eq!(key, "heading");
            assert!(callback.is_none(), "1-arg → callback=None");
        } else {
            panic!("esperado Content::CounterDisplayCallback");
        }
    }

    #[test]
    fn p241_native_counter_display_com_callback_constroi_some() {
        use crate::entities::func::Func;
        null_ctx!(ctx);
        let identity_fn = Func::native("identity", |_, args, _, _, _| {
            Ok(args.items.first().cloned().unwrap_or(Value::None))
        });
        let r = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("figure".into()), Value::Func(identity_fn)]),
            &null_world(), test_file_id(), None,
        ).unwrap();
        if let Value::Content(Content::CounterDisplayCallback { key, callback }) = r {
            assert_eq!(key, "figure");
            assert!(callback.is_some(), "2-arg → callback=Some");
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
            &null_world(), test_file_id(), None,
        );
        assert!(r1.is_err(), "key não-string deve retornar Err");
        // 2-arg segundo arg não-Func.
        let r2 = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(2)]),
            &null_world(), test_file_id(), None,
        );
        assert!(r2.is_err(), "callback não-Func deve retornar Err");
    }

    #[test]
    fn p241_native_counter_display_arity_errada_rejeita() {
        null_ctx!(ctx);
        // 0 args.
        let r0 = native_counter_display(
            &mut ctx, &p(vec![]),
            &null_world(), test_file_id(), None,
        );
        assert!(r0.is_err(), "0 args deve retornar Err");
        // 3 args.
        let r3 = native_counter_display(
            &mut ctx,
            &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
            &null_world(), test_file_id(), None,
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
        let r = native_table_header(&mut ctx, &p(vec![Value::Content(Content::text("body"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableHeader { body, repeat }) = r {
            assert_eq!(body.plain_text(), "body");
            assert!(repeat, "default vanilla repeat=true (Caso D)");
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn native_table_footer_default_repeat_true() {
        // Par simétrico — paridade absoluta com header.
        null_ctx!(ctx);
        let r = native_table_footer(&mut ctx, &p(vec![Value::Content(Content::text("body"))]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableFooter { body, repeat }) = r {
            assert_eq!(body.plain_text(), "body");
            assert!(repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn native_table_header_repeat_false_explicito() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableHeader { repeat, .. }) = r {
            assert!(!repeat);
        } else {
            panic!("esperado Content::TableHeader");
        }
    }

    #[test]
    fn native_table_footer_repeat_false_explicito() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Bool(false));
        let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::TableFooter { repeat, .. }) = r {
            assert!(!repeat);
        } else {
            panic!("esperado Content::TableFooter");
        }
    }

    #[test]
    fn native_table_header_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_header(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "table_header() sem body deve retornar Err");
    }

    #[test]
    fn native_table_footer_sem_body_rejeitado() {
        null_ctx!(ctx);
        let r = native_table_footer(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "table_footer() sem body deve retornar Err");
    }

    #[test]
    fn native_table_header_repeat_int_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Int(1));
        let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "repeat=Int em table_header() deve retornar Err");
    }

    #[test]
    fn native_table_footer_repeat_int_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("repeat".into(), Value::Int(1));
        let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "repeat=Int em table_footer() deve retornar Err");
    }

    #[test]
    fn native_table_header_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("level".into(), Value::Int(2));
        let r = native_table_header(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "level (scope-out) em table_header() deve retornar Err");
    }

    #[test]
    fn native_table_footer_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("body"))]);
        args.named.insert("foo".into(), Value::Bool(true));
        let r = native_table_footer(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "named arg desconhecido em table_footer() deve retornar Err");
    }

    // ── Passo 159A — Bibliography + Cite par acoplado ────────────────────

    fn make_bib_dict(key: &str, author: &str, title: &str, year: i64) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(),    Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(),  Value::Str(title.into()));
        d.insert("year".into(),   Value::Int(year));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_default_vazia() {
        // P159A: bibliography() sem args produz Bibliography vazia.
        null_ctx!(ctx);
        let r = native_bibliography(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, title }) = r {
            assert!(entries.is_empty());
            assert!(title.is_none());
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_com_entries_posicional() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![
            make_bib_dict("smith2024", "Smith, J.", "On Crystal Math", 2024),
        ]);
        let args = p(vec![entries_arr]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].key, "smith2024");
            assert_eq!(entries[0].author, "Smith, J.");
            assert_eq!(entries[0].year, 2024);
        } else {
            panic!("esperado Content::Bibliography");
        }
    }

    #[test]
    fn native_bibliography_com_title() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Array(vec![])]);
        args.named.insert("title".into(), Value::Str("Referências".into()));
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { title, .. }) = r {
            assert_eq!(title.as_ref().map(|t| t.plain_text()).as_deref(), Some("Referências"));
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
        d.insert("key".into(),    Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(),  Value::Str("T".into()));
        // year ausente intencionalmente.
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "dict sem 'year' deve retornar Err");
    }

    #[test]
    fn native_bibliography_year_negativo_rejeitado() {
        null_ctx!(ctx);
        let args = p(vec![Value::Array(vec![
            make_bib_dict("k", "A", "T", -5),
        ])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "year negativo deve retornar Err");
    }

    #[test]
    fn native_bibliography_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Array(vec![])]);
        args.named.insert("style".into(), Value::Str("apa".into()));
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "style (scope-out per ADR-0054 graded) deve retornar Err");
    }

    #[test]
    fn native_cite_so_key_posicional() {
        null_ctx!(ctx);
        let r = native_cite(&mut ctx, &p(vec![Value::Str("smith2024".into())]), &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { key, supplement, form }) = r {
            assert_eq!(key, "smith2024");
            assert!(supplement.is_none());
            assert!(form.is_none());
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_com_supplement() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("smith2024".into())]);
        args.named.insert("supplement".into(), Value::Str("p. 42".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { supplement, .. }) = r {
            assert_eq!(supplement.as_ref().map(|s| s.plain_text()).as_deref(), Some("p. 42"));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_sem_key_rejeitado() {
        null_ctx!(ctx);
        let r = native_cite(&mut ctx, &p(vec![]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "cite() sem key deve retornar Err");
    }

    #[test]
    fn native_cite_key_vazia_rejeitada() {
        null_ctx!(ctx);
        let r = native_cite(&mut ctx, &p(vec![Value::Str("".into())]), &null_world(), test_file_id(), None);
        assert!(r.is_err(), "cite() key vazia deve retornar Err");
    }

    #[test]
    fn native_cite_named_arg_desconhecido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("style".into(), Value::Str("apa".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "style (scope-out per ADR-0054 graded) deve retornar Err");
    }

    // ── Passo 159C — Cite.form variants ─────────────────────────────────────

    #[test]
    fn native_cite_form_normal_parse() {
        use crate::entities::citation_form::CitationForm;
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("normal".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { form, .. }) = r {
            assert_eq!(form, Some(CitationForm::Normal));
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
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { form, .. }) = r {
            assert_eq!(form, Some(CitationForm::Prose));
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
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { form, .. }) = r {
            assert_eq!(form, Some(CitationForm::Author));
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
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { form, .. }) = r {
            assert_eq!(form, Some(CitationForm::Year));
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_auto_devolve_none() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Auto);
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Cite { form, .. }) = r {
            assert!(form.is_none(), "form=auto deve produzir None (resolvido a Normal default em layout)");
        } else {
            panic!("esperado Content::Cite");
        }
    }

    #[test]
    fn native_cite_form_invalido_rejeitado() {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Str("k".into())]);
        args.named.insert("form".into(), Value::Str("xpto".into()));
        let r = native_cite(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "form inválido deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("normal") && msg.contains("prose") && msg.contains("author") && msg.contains("year"),
            "mensagem deve listar forms válidas: {}", msg);
    }

    // ── Passo 159D — BibEntry fields opcionais ──────────────────────────────

    fn make_bib_dict_full(
        key: &str, author: &str, title: &str, year: i64,
        volume: &str, pages: &str, journal: &str, publisher: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(),       Value::Str(key.into()));
        d.insert("author".into(),    Value::Str(author.into()));
        d.insert("title".into(),     Value::Str(title.into()));
        d.insert("year".into(),      Value::Int(year));
        d.insert("volume".into(),    Value::Str(volume.into()));
        d.insert("pages".into(),     Value::Str(pages.into()));
        d.insert("journal".into(),   Value::Str(journal.into()));
        d.insert("publisher".into(), Value::Str(publisher.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_fields_opcionais_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![
            make_bib_dict_full("smith2024", "Smith, J.", "On Crystal Math", 2024,
                "12", "1-10", "Nature Communications", "ACM"),
        ]);
        let args = p(vec![entries_arr]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
            assert_eq!(e.volume.as_deref(),    Some("12"));
            assert_eq!(e.pages.as_deref(),     Some("1-10"));
            assert_eq!(e.journal.as_deref(),   Some("Nature Communications"));
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
        let args = p(vec![Value::Array(vec![
            make_bib_dict("smith2024", "Smith, J.", "On Crystal Math", 2024),
        ])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
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
        d.insert("key".into(),    Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(),  Value::Str("T".into()));
        d.insert("year".into(),   Value::Int(2024));
        // volume com tipo errado (Int em vez de Str).
        d.insert("volume".into(), Value::Int(42));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "volume com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("volume"),
            "mensagem deve mencionar field 'volume': {}", msg);
    }

    // ── Passo 159E — par natural url/doi ────────────────────────────────────

    fn make_bib_dict_with_url_doi(
        key: &str, author: &str, title: &str, year: i64,
        url: &str, doi: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(),    Value::Str(key.into()));
        d.insert("author".into(), Value::Str(author.into()));
        d.insert("title".into(),  Value::Str(title.into()));
        d.insert("year".into(),   Value::Int(year));
        d.insert("url".into(),    Value::Str(url.into()));
        d.insert("doi".into(),    Value::Str(doi.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_url_doi_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![
            make_bib_dict_with_url_doi(
                "smith2024", "Smith, J.", "On Crystal Math", 2024,
                "https://example.com/paper", "10.1234/abc",
            ),
        ]);
        let args = p(vec![entries_arr]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
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
        let args = p(vec![Value::Array(vec![
            make_bib_dict("smith2024", "Smith, J.", "On Crystal Math", 2024),
        ])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
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
        d.insert("key".into(),    Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(),  Value::Str("T".into()));
        d.insert("year".into(),   Value::Int(2024));
        // doi com tipo errado (Int em vez de Str).
        d.insert("doi".into(), Value::Int(42));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "doi com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("doi"),
            "mensagem deve mencionar field 'doi': {}", msg);
    }

    // ── Passo 159G — 6 fields restantes comuns hayagriva ────────────────────

    fn make_bib_dict_full_p159g(
        key: &str, author: &str, title: &str, year: i64,
        editor: &str, series: &str, note: &str,
        isbn: &str, location: &str, organization: &str,
    ) -> Value {
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        d.insert("key".into(),          Value::Str(key.into()));
        d.insert("author".into(),       Value::Str(author.into()));
        d.insert("title".into(),        Value::Str(title.into()));
        d.insert("year".into(),         Value::Int(year));
        d.insert("editor".into(),       Value::Str(editor.into()));
        d.insert("series".into(),       Value::Str(series.into()));
        d.insert("note".into(),         Value::Str(note.into()));
        d.insert("isbn".into(),         Value::Str(isbn.into()));
        d.insert("location".into(),     Value::Str(location.into()));
        d.insert("organization".into(), Value::Str(organization.into()));
        Value::Dict(d)
    }

    #[test]
    fn native_bibliography_parse_p159g_fields_presentes() {
        null_ctx!(ctx);
        let entries_arr = Value::Array(vec![
            make_bib_dict_full_p159g(
                "smith2024", "Smith, J.", "On Crystal Math", 2024,
                "Doe, A.", "Crystal Studies", "See also Smith 2023",
                "978-0-1234-5678-9", "New York", "ACM",
            ),
        ]);
        let args = p(vec![entries_arr]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
            assert_eq!(e.editor.as_deref(),       Some("Doe, A."));
            assert_eq!(e.series.as_deref(),       Some("Crystal Studies"));
            assert_eq!(e.note.as_deref(),         Some("See also Smith 2023"));
            assert_eq!(e.isbn.as_deref(),         Some("978-0-1234-5678-9"));
            assert_eq!(e.location.as_deref(),     Some("New York"));
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
        d.insert("key".into(),    Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(),  Value::Str("T".into()));
        d.insert("year".into(),   Value::Int(2024));
        d.insert("editor".into(), Value::Str("Ed1".into()));
        d.insert("isbn".into(),   Value::Str("978-0-1".into()));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
            assert_eq!(e.editor.as_deref(), Some("Ed1"));
            assert_eq!(e.isbn.as_deref(),   Some("978-0-1"));
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
        let args = p(vec![Value::Array(vec![
            make_bib_dict("smith2024", "Smith, J.", "On Crystal Math", 2024),
        ])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None).unwrap();
        if let Value::Content(Content::Bibliography { entries, .. }) = r {
            let e = &entries[0];
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
        d.insert("key".into(),    Value::Str("k".into()));
        d.insert("author".into(), Value::Str("A".into()));
        d.insert("title".into(),  Value::Str("T".into()));
        d.insert("year".into(),   Value::Int(2024));
        // isbn com tipo errado (Int em vez de Str).
        d.insert("isbn".into(), Value::Int(978));
        let args = p(vec![Value::Array(vec![Value::Dict(d)])]);
        let r = native_bibliography(&mut ctx, &args, &null_world(), test_file_id(), None);
        assert!(r.is_err(), "isbn com tipo Int deve retornar Err");
        let msg = r.unwrap_err()[0].message.clone();
        assert!(msg.contains("isbn"),
            "mensagem deve mencionar field 'isbn': {}", msg);
    }
}
