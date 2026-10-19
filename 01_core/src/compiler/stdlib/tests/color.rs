//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/color.md
//! @prompt-hash 4f1f4fc5
//! @layer L1
//! @updated 2026-09-26

use super::*;
use crate::compiler::stdlib::color::{
    native_color_darken, native_color_desaturate, native_color_lighten, native_color_mix,
    native_color_negate, native_color_saturate,
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
    // **P742** — semântica corrigida por medição vanilla: negate opera em
    // Oklab por default (não complemento sRGB). Vermelho puro (1,0,0) →
    // `#005688` (medido no palette 0.7.6). Nome histórico mantido.
    null_ctx!(ctx);
    let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
    let args = p(vec![red]);
    let r = native_color_negate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Color(c) = r {
        assert_eq!(c.to_srgb(), (0, 86, 136, 255), "negate(vermelho puro) → #005688");
    } else {
        panic!("esperado Color");
    }
}

#[test]
fn p476_native_color_mix_dois_positional_sem_weight() {
    null_ctx!(ctx);
    let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
    let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
    let args = p(vec![red, blue]);
    let r = native_color_mix(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "mix sem weight: default 0.5; erro: {:?}", r.err());
    assert!(matches!(r.unwrap(), Value::Color(_)));
}

#[test]
fn p476_native_color_mix_com_weight_named() {
    null_ctx!(ctx);
    let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
    let blue = Value::Color(Color::srgb_f32(0.0, 0.0, 1.0, 1.0));
    let mut args = p(vec![red, blue]);
    args.named.insert("weight".into(), Value::Float(0.25));
    let r = native_color_mix(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "mix com weight: 0.25; erro: {:?}", r.err());
}

#[test]
fn p476_color_module_no_scope_tem_4_entradas() {
    // **P736** — era `make_color_module() -> Value::Dict`; passou a
    // `color_type_field` (fields do valor-tipo `color`).
    use crate::compiler::stdlib::color_type_field;
    assert!(color_type_field("lighten").is_some(), "falta lighten");
    assert!(color_type_field("darken").is_some(), "falta darken");
    assert!(color_type_field("mix").is_some(), "falta mix");
    assert!(color_type_field("negate").is_some(), "falta negate");
    // NOTE: P477 aumenta para 6; atualizado em p477_color_module_tem_6_entradas
}

#[test]
fn p477_native_color_saturate_retorna_color() {
    null_ctx!(ctx);
    let red = Value::Color(Color::srgb_f32(1.0, 0.0, 0.0, 1.0));
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
    // **P736** — era `make_color_module() -> Value::Dict`; passou a
    // `color_type_field` (fields do valor-tipo `color`).
    use crate::compiler::stdlib::color_type_field;
    assert!(color_type_field("saturate").is_some(), "falta saturate");
    assert!(color_type_field("desaturate").is_some(), "falta desaturate");
    let ops = ["lighten", "darken", "mix", "negate", "saturate", "desaturate"];
    assert!(
        ops.iter().all(|f| color_type_field(f).is_some()),
        "os 6 operadores P476/P477 devem ser fields do tipo color"
    );
}
