//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/color.md
//! @prompt-hash 6b57c538
//! @layer L1
//! @updated 2026-09-12
//!
//! Construtores de cor: rgb, luma, oklab, oklch, linear_rgb, cmyk, hsl, hsv.
//! Fatiado de `foundations.rs` no Passo 1032.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Color, ColorSpace, Length};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use crate::compiler::stdlib::{err, expect_no_named};

/// `rgb(r, g, b)` ou `rgb(r, g, b, a)` → Color.
///
/// **P740D** — paridade vanilla `Component` (`visualize/color.rs:2678-2692`):
/// cada componente aceita Int [0, 255] ou Ratio [0%, 100%] (mesmo padrão
/// aplicado a `linear-rgb` em P736). Ratio → u8 via `(fracção × 255).round()`
/// — medido: `rgb(50%, 0%, 0%)` → `rgb("#800000")` (127.5 → 128 = 0x80).
/// Erros verbatim do cast `Component`: Int fora de gama → "number must be
/// between 0 and 255"; Ratio fora de gama → "ratio must be between 0% and
/// 100%"; Float (e outros tipos) → "expected integer or ratio, found {type}".
pub fn native_rgb(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => parse_hex_color(s.as_str()),
        [Value::Color(color)] => {
            let (r, g, b, a) = rgb_constructor_bytes(color.clone());
            Ok(Value::Color(Color::rgba(r, g, b, a)))
        }
        [r, g, b] => Ok(Value::Color(Color::rgb(as_u8(r)?, as_u8(g)?, as_u8(b)?))),
        [r, g, b, a] => {
            Ok(Value::Color(Color::rgba(as_u8(r)?, as_u8(g)?, as_u8(b)?, as_u8(a)?)))
        }
        _ => err(format!(
            "rgb() requer 3 ou 4 componentes (Int/Ratio), recebeu {} args",
            args.items.len()
        )),
    }
}

/// Bridge do construtor `rgb(color)`. O perfil ICC CMYK completo permanece
/// fora de L1; os termos cruzados selam o vetor observado em P1286 sem mudar
/// a conversão canônica usada pelos demais consumers de `Color`.
fn rgb_constructor_bytes(color: Color) -> (u8, u8, u8, u8) {
    let Color::Cmyk { c, m, y, k } = color else {
        return color.to_srgb();
    };
    let remaining = 1.0 - k;
    let r = (1.0 - c) * remaining + 0.3344623 * m * y * remaining;
    let g = (1.0 - m) * remaining - 0.15901421 * c * y * remaining;
    let b = (1.0 - y) * remaining + 0.063455805 * c * m * remaining;
    (
        Color::f32_to_u8_ties_even(r),
        Color::f32_to_u8_ties_even(g),
        Color::f32_to_u8_ties_even(b),
        255,
    )
}

/// **P703** — `rgb(hex)`: cor a partir de notação hexadecimal (3/4/6/8
/// dígitos, `#` opcional, maiúsculas/minúsculas indiferentes). Paridade
/// vanilla verbatim — algoritmo e mensagens de `visualize/color.rs:2072-2107`
/// do vanilla (`impl FromStr for Rgb`).
fn parse_hex_color(s: &str) -> SourceResult<Value> {
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.chars().any(|c| !c.is_ascii_hexdigit()) {
        return err("color string contains non-hexadecimal letters");
    }
    let len = hex.len();
    let long = len == 6 || len == 8;
    let short = len == 3 || len == 4;
    let has_alpha = len == 4 || len == 8;
    if !long && !short {
        return err("color string has wrong length");
    }
    let count = if has_alpha { 4 } else { 3 };
    let item_len = if long { 2 } else { 1 };
    let mut values = [255u8; 4];
    for (i, value) in values.iter_mut().enumerate().take(count) {
        let pos = i * item_len;
        let item = &hex[pos..pos + item_len];
        let mut v = u8::from_str_radix(item, 16).unwrap();
        if short {
            v += v * 16;
        }
        *value = v;
    }
    Ok(Value::Color(Color::rgba(values[0], values[1], values[2], values[3])))
}

fn as_u8(v: &Value) -> SourceResult<u8> {
    match v {
        Value::Int(i) if (0..=255).contains(i) => Ok(*i as u8),
        Value::Int(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "number must be between 0 and 255".to_string(),
        )]),
        Value::Relative(r)
            if matches!(
                (r.abs.is_zero(), (0.0..=1.0).contains(&r.rel)),
                (true, true)
            ) =>
        {
            Ok((r.rel * 255.0).round() as u8)
        }
        Value::Relative(r) if r.abs.is_zero() => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "ratio must be between 0% and 100%".to_string(),
        )]),
        // P842 (#32) — o literal percentual é agora `Value::Ratio`
        // (antes `Relative` com abs zero). Mesma validação do vanilla.
        Value::Ratio(r) if (0.0..=1.0).contains(&r.get()) => {
            Ok((r.get() * 255.0).round() as u8)
        }
        Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "ratio must be between 0% and 100%".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected integer or ratio, found {}", other.type_name()),
        )]),
    }
}

/// `luma(lightness[, alpha])` ou `luma(color)` → Color::Luma.
///
/// **P257 (ADR-0083 PROPOSTO)** — refactor: constrói `Color::Luma`
/// dedicado (em vez de `Color::Srgb` cinzento). Aceita Int [0, 255]
/// como paridade construtor anterior; converte para f32 [0.0, 1.0]
/// internamente. PDF output bit-equivalente via `to_srgb()` que
/// expande Luma para sRGB cinzento.
///
/// **P1358** — lightness é obrigatório e alpha é o segundo positional
/// opcional. Ambos usam `Component`: Int [0, 255] ou Ratio [0%, 100%], com
/// diagnósticos públicos para tipo/range inválido. Named args são rejeitados.
/// A conversão `luma(color)` permanece integralmente a de P1252 e preserva
/// alpha como correção `Known-Upstream-Bug`.
/// ADR-0107: comportamento
/// observável da língua, não mecânica — paridade exige replicar, não
/// substituir por erro.
pub fn native_luma(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            named_span(args, key),
            format!("unexpected argument: {key}"),
        )]);
    }
    match args.items.as_slice() {
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: lightness component",
        )
        .with_hint("try writing `white` or `luma(100%)`")]),
        [Value::Color(color)] => {
            Ok(Value::Color(color.clone().to_space(ColorSpace::Luma).map_err(|_| {
                vec![SourceDiagnostic::error(args.span, "cannot convert color to luma")]
            })?))
        }
        [lightness] => Ok(Value::Color(Color::Luma {
            l: luma_component(lightness, positional_span(args, 0))?,
            a: 1.0,
        })),
        [Value::Color(_), _] => Err(vec![SourceDiagnostic::error(
            positional_span(args, 1),
            "unexpected argument",
        )]),
        [lightness, alpha] => Ok(Value::Color(Color::Luma {
            l: luma_component(lightness, positional_span(args, 0))?,
            a: luma_component(alpha, positional_span(args, 1))?,
        })),
        _ => Err(vec![SourceDiagnostic::error(
            positional_span(args, 2),
            "unexpected argument",
        )]),
    }
}

fn positional_span(args: &Args, index: usize) -> Span {
    args.occurrences
        .as_ref()
        .and_then(|occurrences| {
            occurrences.iter().filter(|item| item.name.is_none()).nth(index)
        })
        .map(|item| item.value_span)
        .unwrap_or(Span::detached())
}

fn named_span(args: &Args, name: &str) -> Span {
    args.occurrences
        .as_ref()
        .and_then(|occurrences| {
            occurrences.iter().find(|item| item.name.as_deref() == Some(name))
        })
        .map(|item| item.span)
        .unwrap_or(Span::detached())
}

fn luma_component(v: &Value, span: Span) -> SourceResult<f32> {
    match v {
        Value::Int(i) if (0..=255).contains(i) => Ok(*i as f32 / 255.0),
        Value::Int(_) => {
            Err(vec![SourceDiagnostic::error(span, "number must be between 0 and 255")])
        }
        Value::Ratio(r) if (0.0..=1.0).contains(&r.get()) => Ok(r.get() as f32),
        Value::Ratio(_) => {
            Err(vec![SourceDiagnostic::error(span, "ratio must be between 0% and 100%")])
        }
        // P705 — percentagens simples (`50%`, `v * 1%`) avaliavam para
        // Value::Relative neste cristalino (unificado com `length`);
        // desde P842 (#32) avaliam para `Value::Ratio` (braço acima) —
        // este braço fica para `Relative` construídos por outras vias.
        // Só conta como componente de cor se não tiver parte absoluta
        // (`50% + 1pt` não é um componente válido, cai no fallback).
        Value::Relative(rel)
            if matches!(
                (rel.abs == Length::ZERO, (0.0..=1.0).contains(&rel.rel)),
                (true, true)
            ) =>
        {
            Ok(rel.rel as f32)
        }
        Value::Relative(rel) if rel.abs == Length::ZERO => {
            Err(vec![SourceDiagnostic::error(span, "ratio must be between 0% and 100%")])
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer or ratio, found {}",
                if matches!(other, Value::Str(_)) { "string" } else { other.type_name() }
            ),
        )]),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklab(l, a, b[, alpha])` →
/// `Color::Oklab`. Componentes f32 (Float ou Int).
pub fn native_oklab(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [l, a, b] => Ok(Value::Color(Color::oklab(
            ratio_component(l)?,
            chroma_component(a)?,
            chroma_component(b)?,
            1.0,
        ))),
        [l, a, b, alpha] => Ok(Value::Color(Color::oklab(
            ratio_component(l)?,
            chroma_component(a)?,
            chroma_component(b)?,
            ratio_component(alpha)?,
        ))),
        _ => err(format!(
            "oklab() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklch(l, c, h[, alpha])` →
/// `Color::Oklch`. `h` em graus.
pub fn native_oklch(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [l, c, h] => Ok(Value::Color(Color::oklch(
            ratio_component(l)?,
            chroma_component(c)?,
            angle_component(h)?,
            1.0,
        ))),
        [l, c, h, alpha] => Ok(Value::Color(Color::oklch(
            ratio_component(l)?,
            chroma_component(c)?,
            angle_component(h)?,
            ratio_component(alpha)?,
        ))),
        _ => err(format!(
            "oklch() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `linear_rgb(r, g, b[, alpha])`
/// → `Color::LinearRgb`. Componentes f32 [0.0, 1.0].
pub fn native_linear_rgb(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    // **P736** — paridade vanilla medida: `linear-rgb` aceita Int [0, 255]
    // (÷255) ou Ratio (percentagem); **rejeita Float** com a mensagem
    // verbatim "expected integer or ratio, found float".
    fn component(v: &Value) -> SourceResult<f32> {
        match v {
            Value::Int(i) => Ok(*i as f32 / 255.0),
            Value::Relative(r) if r.abs.is_zero() => Ok(r.rel as f32),
            // P842 (#32) — o literal percentual é agora `Value::Ratio`.
            Value::Ratio(r) => Ok(r.get() as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer or ratio, found {}", other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [r, g, b] => Ok(Value::Color(Color::linear_rgb(
            component(r)?,
            component(g)?,
            component(b)?,
            1.0,
        ))),
        [r, g, b, a] => Ok(Value::Color(Color::linear_rgb(
            component(r)?,
            component(g)?,
            component(b)?,
            component(a)?,
        ))),
        _ => err(format!(
            "linear_rgb() requer 3 ou 4 argumentos (Int/Ratio), recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `cmyk(c, m, y, k)` →
/// `Color::Cmyk`. Componentes f32 [0.0, 1.0].
pub fn native_cmyk(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let component = |value: &Value| -> SourceResult<f32> {
        match value {
            Value::Ratio(ratio) if (0.0..=1.0).contains(&ratio.get()) => {
                Ok(ratio.get() as f32)
            }
            Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ratio must be between 0% and 100%",
            )]),
            // Compatibilidade com árvores anteriores à materialização de
            // `Value::Ratio`; a superfície atual produz o braço acima.
            Value::Relative(relative)
                if relative.abs.is_zero() && (0.0..=1.0).contains(&relative.rel) =>
            {
                Ok(relative.rel as f32)
            }
            Value::Relative(relative) if relative.abs.is_zero() => {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "ratio must be between 0% and 100%",
                )])
            }
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected ratio, found {}", other.type_name()),
            )]),
        }
    };
    match args.items.as_slice() {
        [c, m, y, k] => Ok(Value::Color(Color::cmyk(
            component(c)?,
            component(m)?,
            component(y)?,
            component(k)?,
        ))),
        _ => err(format!("cmyk() requer 4 Ratio, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsl(h, s, l[, alpha])` →
/// `Color::Hsl`. `h` em graus.
pub fn native_hsl(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [h, s, l] => Ok(Value::Color(Color::hsl(
            angle_component(h)?,
            color_component(s)?,
            color_component(l)?,
            1.0,
        ))),
        [h, s, l, a] => Ok(Value::Color(Color::hsl(
            angle_component(h)?,
            color_component(s)?,
            color_component(l)?,
            color_component(a)?,
        ))),
        _ => err(format!(
            "hsl() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsv(h, s, v[, alpha])` →
/// `Color::Hsv`. `h` em graus.
pub fn native_hsv(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [h, s, v] => Ok(Value::Color(Color::hsv(
            angle_component(h)?,
            color_component(s)?,
            color_component(v)?,
            1.0,
        ))),
        [h, s, v, a] => Ok(Value::Color(Color::hsv(
            angle_component(h)?,
            color_component(s)?,
            color_component(v)?,
            color_component(a)?,
        ))),
        _ => err(format!(
            "hsv() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

fn ratio_component(value: &Value) -> SourceResult<f32> {
    match value {
        Value::Ratio(ratio) if (0.0..=1.0).contains(&ratio.get()) => {
            Ok(ratio.get() as f32)
        }
        Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "ratio must be between 0% and 100%",
        )]),
        Value::Relative(relative)
            if relative.abs.is_zero() && (0.0..=1.0).contains(&relative.rel) =>
        {
            Ok(relative.rel as f32)
        }
        Value::Relative(relative) if relative.abs.is_zero() => {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ratio must be between 0% and 100%",
            )])
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected ratio, found {}", other.type_name()),
        )]),
    }
}

fn color_component(value: &Value) -> SourceResult<f32> {
    match value {
        Value::Int(integer) if (0..=255).contains(integer) => Ok(*integer as f32 / 255.0),
        Value::Int(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "number must be between 0 and 255",
        )]),
        Value::Ratio(_) | Value::Relative(_) => ratio_component(value),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected integer or ratio, found {}", other.type_name()),
        )]),
    }
}

fn chroma_component(value: &Value) -> SourceResult<f32> {
    match value {
        Value::Float(float) => Ok(*float as f32),
        Value::Int(integer) => Ok(*integer as f32),
        Value::Ratio(ratio) => Ok(ratio.get() as f32 * 0.4),
        Value::Relative(relative) if relative.abs.is_zero() => {
            Ok(relative.rel as f32 * 0.4)
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected float or ratio, found {}", other.type_name()),
        )]),
    }
}

fn angle_component(value: &Value) -> SourceResult<f32> {
    match value {
        Value::Angle(angle) => Ok(angle.to_deg() as f32),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected angle, found {}", other.type_name()),
        )]),
    }
}

#[cfg(test)]
mod tests_p1284_constructor_components {
    use super::*;
    use crate::entities::layout_types::{Angle, Ratio};

    #[test]
    fn p1284_ratio_component_e_chroma_preservam_unidades() {
        assert_eq!(ratio_component(&Value::Ratio(Ratio(0.5))).unwrap(), 0.5);
        assert!(ratio_component(&Value::Int(50)).is_err());
        assert_eq!(chroma_component(&Value::Ratio(Ratio(0.5))).unwrap(), 0.2);
        assert_eq!(chroma_component(&Value::Float(0.5)).unwrap(), 0.5);
    }

    #[test]
    fn p1284_component_hsl_e_angulo_seguem_casts_vanilla() {
        assert_eq!(color_component(&Value::Int(255)).unwrap(), 1.0);
        assert_eq!(color_component(&Value::Ratio(Ratio(0.25))).unwrap(), 0.25);
        assert!(color_component(&Value::Float(0.25)).is_err());
        assert_eq!(angle_component(&Value::Angle(Angle::deg(90.0))).unwrap(), 90.0);
    }
}

#[cfg(test)]
mod tests_p703_rgb_hex {
    use super::*;
    use crate::entities::layout_types::Color;

    fn hex(s: &str) -> Value {
        parse_hex_color(s).unwrap()
    }

    #[test]
    fn hex_6_digitos_com_hash() {
        assert_eq!(hex("#FF0000"), Value::Color(Color::rgba(255, 0, 0, 255)));
    }

    #[test]
    fn hex_6_digitos_sem_hash() {
        assert_eq!(hex("FF0000"), Value::Color(Color::rgba(255, 0, 0, 255)));
    }

    #[test]
    fn hex_8_digitos_com_alpha() {
        assert_eq!(hex("#FF0000FF"), Value::Color(Color::rgba(255, 0, 0, 255)));
        assert_eq!(hex("FF000080"), Value::Color(Color::rgba(255, 0, 0, 0x80)));
    }

    #[test]
    fn hex_3_digitos_curto_duplica() {
        assert_eq!(hex("F00"), Value::Color(Color::rgba(255, 0, 0, 255)));
        assert_eq!(hex("abc"), Value::Color(Color::rgba(0xaa, 0xbb, 0xcc, 255)));
    }

    #[test]
    fn hex_4_digitos_curto_com_alpha() {
        // P703 — 4º dígito curto é alpha, não mais um componente de cor.
        // Medido contra o vanilla: rgb("FF00") -> rgb("#ffff0000").
        assert_eq!(hex("FF00"), Value::Color(Color::rgba(255, 255, 0, 0)));
    }

    #[test]
    fn hex_invalido_erro_verbatim() {
        let e = parse_hex_color("nothex").unwrap_err();
        assert!(
            e[0].message.contains("color string contains non-hexadecimal letters"),
            "msg: {}",
            e[0].message,
        );
    }

    #[test]
    fn hex_nome_de_cor_erro_como_no_vanilla() {
        // "red" não é hex válido — o vanilla também erra aqui, não é feature.
        let e = parse_hex_color("red").unwrap_err();
        assert!(
            e[0].message.contains("color string contains non-hexadecimal letters"),
            "msg: {}",
            e[0].message,
        );
    }

    #[test]
    fn hex_comprimento_errado_erro_verbatim() {
        let e = parse_hex_color("FFFFF").unwrap_err(); // 5 dígitos
        assert!(
            e[0].message.contains("color string has wrong length"),
            "msg: {}",
            e[0].message,
        );
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    #[test]
    fn native_rgb_forma_numerica_sem_regressao() {
        let args =
            Args::positional(vec![Value::Int(255), Value::Int(0), Value::Int(128)]);
        let v = native_rgb(
            &mut crate::compiler::eval::EvalContext::new(),
            &args,
            &NullWorld::default(),
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Color(Color::rgb(255, 0, 128)));
    }
}
#[cfg(test)]
mod tests_p705_luma_ratio {
    use super::*;
    use crate::entities::layout_types::{Color, Ratio};

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn luma(items: Vec<Value>) -> Value {
        native_luma(&mut ctx(), &Args::positional(items), &NullWorld::default(), tfid())
            .unwrap()
    }

    #[test]
    fn p1252_luma_lightness_alpha_preserva_alpha_publico() {
        let args =
            Args::positional(vec![Value::Ratio(Ratio(0.5)), Value::Ratio(Ratio(0.4))]);
        assert_eq!(
            native_luma(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap(),
            Value::Color(Color::Luma { l: 0.5, a: 0.4 })
        );
    }

    #[test]
    fn p1252_luma_color_preserva_alpha_known_upstream_bug() {
        let source = Color::srgb_f32(1.0, 0.254902, 0.211765, 0.4);
        let Value::Color(Color::Luma { l, a }) = luma(vec![Value::Color(source)]) else {
            panic!("luma(color) deve produzir Color::Luma");
        };
        assert_eq!(a.to_bits(), 0.4_f32.to_bits());
        assert!(
            (l - 0.5402).abs() < 0.00005,
            "P1239 fecha luminância separadamente: {l}"
        );
    }

    #[test]
    fn ratio_valido_mapeia_diretamente() {
        assert_eq!(luma(vec![Value::Ratio(Ratio(0.5))]), Value::Color(Color::luma(0.5)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(0.0))]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(1.0))]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn percentagem_literal_via_relative_mapeia_diretamente() {
        // P705 — causa raiz real: `50%`/`v * 1%` avaliam para Value::Relative
        // neste cristalino (unificado com `length`), não Value::Ratio.
        // Reproduz exactamente o caminho de cetz: `range(...).map(v => luma(v * 1%))`.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel = |pct: f64| Value::Relative(Rel { rel: pct, abs: Length::ZERO });
        assert_eq!(luma(vec![rel(0.9)]), Value::Color(Color::luma(0.9)));
        assert_eq!(luma(vec![rel(0.5)]), Value::Color(Color::luma(0.5)));
        assert_eq!(luma(vec![rel(0.0)]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![rel(1.0)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn percentagem_com_parte_absoluta_e_tipo_invalido() {
        // `50% + 1pt` não é um componente de cor válido (nem no vanilla) e
        // deve propagar erro de tipo, não ser tratado como 50%.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        let error = p1358_error(Args::positional(vec![rel]));
        assert_eq!(error.message, "expected integer or ratio, found relative length");
    }

    #[test]
    fn int_valido_sem_regressao() {
        assert_eq!(luma(vec![Value::Int(0)]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![Value::Int(255)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn int_fora_de_gama_propaga_erro() {
        for value in [300, 256] {
            assert_eq!(
                p1358_error(Args::positional(vec![Value::Int(value)])).message,
                "number must be between 0 and 255"
            );
        }
    }

    #[test]
    fn ratio_fora_de_gama_propaga_erro() {
        for value in [1.5, -0.2] {
            assert_eq!(
                p1358_error(Args::positional(vec![Value::Ratio(Ratio(value))])).message,
                "ratio must be between 0% and 100%"
            );
        }
    }

    #[test]
    fn tipo_errado_propaga_erro() {
        assert_eq!(
            p1358_error(Args::positional(vec![Value::Str("bad".into())])).message,
            "expected integer or ratio, found string"
        );
    }

    #[test]
    fn sem_argumentos_propaga_erro_com_hint() {
        let error = p1358_error(Args::positional(vec![]));
        assert_eq!(error.message, "missing argument: lightness component");
        assert_eq!(error.hints, ["try writing `white` or `luma(100%)`"]);
    }

    #[test]
    fn dois_ou_mais_argumentos_erro_estrutural() {
        let e = native_luma(
            &mut ctx(),
            &Args::positional(vec![Value::Int(0), Value::Int(1), Value::Int(2)]),
            &NullWorld::default(),
            tfid(),
        )
        .unwrap_err();
        assert!(e[0].message.starts_with("unexpected argument"), "msg: {}", e[0].message);
    }

    // ── P1043: Pares de independência testcase() para color.rs:87 e color.rs:155 ─

    #[test]
    fn p1043_color_component_relative_valid_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 87 - C1=T, C2=T: r.abs.is_zero() && (0.0..=1.0).contains(&r.rel) -> Ok(128)
        let v = Value::Relative(Rel { rel: 0.5, abs: Length::ZERO });
        assert_eq!(as_u8(&v).unwrap(), 128);
    }

    #[test]
    fn p1043_color_component_relative_out_of_range_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 87 - C1=T, C2=F: r.abs.is_zero() && !contains -> Err(ratio must be between...)
        let v = Value::Relative(Rel { rel: 1.5, abs: Length::ZERO });
        let err = as_u8(&v).unwrap_err();
        assert!(err[0].message.contains("ratio must be between 0% and 100%"));
    }

    #[test]
    fn p1043_color_component_relative_nonzero_abs_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 87 - C1=F, C2=_: !r.abs.is_zero() -> Err(expected integer or ratio)
        let v = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        let err = as_u8(&v).unwrap_err();
        assert!(err[0].message.contains("expected integer or ratio"));
    }

    #[test]
    fn p1043_color_float_component_relative_valid_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 155 - C1=T, C2=T: rel.abs == Length::ZERO && (0.0..=1.0).contains -> Some(0.5)
        let v = Value::Relative(Rel { rel: 0.5, abs: Length::ZERO });
        assert_eq!(luma_component(&v, Span::detached()).ok(), Some(0.5));
    }

    #[test]
    fn p1043_color_float_component_relative_out_of_range_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 155 - C1=T, C2=F: rel.abs == Length::ZERO && !contains -> None
        let v = Value::Relative(Rel { rel: 1.5, abs: Length::ZERO });
        assert!(luma_component(&v, Span::detached()).is_err());
    }

    #[test]
    fn p1043_color_float_component_relative_nonzero_abs_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 155 - C1=F, C2=_: rel.abs != Length::ZERO -> None
        let v = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        assert!(luma_component(&v, Span::detached()).is_err());
    }

    fn p1358_error(args: Args) -> SourceDiagnostic {
        native_luma(&mut ctx(), &args, &NullWorld::default(), tfid())
            .unwrap_err()
            .remove(0)
    }

    #[test]
    fn p1358_alpha_e_segundo_component_posicional() {
        assert_eq!(
            luma(vec![Value::Int(40), Value::Int(127)]),
            Value::Color(Color::Luma { l: 40.0 / 255.0, a: 127.0 / 255.0 })
        );
        assert_eq!(
            luma(vec![Value::Int(40), Value::Ratio(Ratio(0.5))]),
            Value::Color(Color::Luma { l: 40.0 / 255.0, a: 0.5 })
        );
    }

    #[test]
    fn p1358_lightness_ausente_tem_mensagem_e_hint_publicos() {
        let error = p1358_error(Args::positional(vec![]));
        assert_eq!(error.message, "missing argument: lightness component");
        assert_eq!(error.hints, ["try writing `white` or `luma(100%)`"]);
    }

    #[test]
    fn p1358_component_propaga_tipo_e_ranges() {
        for (items, expected) in [
            (
                vec![Value::Str("invalid".into())],
                "expected integer or ratio, found string",
            ),
            (vec![Value::Int(-1)], "number must be between 0 and 255"),
            (vec![Value::Int(256)], "number must be between 0 and 255"),
            (vec![Value::Ratio(Ratio(-0.01))], "ratio must be between 0% and 100%"),
            (vec![Value::Ratio(Ratio(1.01))], "ratio must be between 0% and 100%"),
            (
                vec![Value::Ratio(Ratio(0.1)), Value::Float(0.5)],
                "expected integer or ratio, found float",
            ),
        ] {
            assert_eq!(p1358_error(Args::positional(items)).message, expected);
        }
    }

    #[test]
    fn p1358_named_e_excesso_sao_unexpected_argument() {
        let mut named = Args::positional(vec![Value::Int(128)]);
        named.named.insert("alpha".into(), Value::Ratio(Ratio(0.4)));
        assert_eq!(p1358_error(named).message, "unexpected argument: alpha");

        let excessive =
            Args::positional(vec![Value::Int(25), Value::Int(50), Value::Int(1)]);
        assert!(p1358_error(excessive).message.starts_with("unexpected argument"));

        let color_alpha = Args::positional(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Ratio(Ratio(0.5)),
        ]);
        assert!(p1358_error(color_alpha).message.starts_with("unexpected argument"));
    }
}
