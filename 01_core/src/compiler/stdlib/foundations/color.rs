//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/color.md
//! @prompt-hash 323f9005
//! @layer L1
//! @updated 2026-08-13
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

/// `luma()`, `luma(l, alpha: alpha)` ou `luma(color)` → Color::Luma.
///
/// **P257 (ADR-0083 PROPOSTO)** — refactor: constrói `Color::Luma`
/// dedicado (em vez de `Color::Srgb` cinzento). Aceita Int [0, 255]
/// como paridade construtor anterior; converte para f32 [0.0, 1.0]
/// internamente. PDF output bit-equivalente via `to_srgb()` que
/// expande Luma para sRGB cinzento.
///
/// **P705** — aceita também `Ratio` [0%, 100%] (paridade vanilla
/// `Component`, `visualize/color.rs:2677-2692`). Fallback silencioso
/// verbatim: qualquer valor que não caste (tipo errado, fora de gama) ou a
/// ausência do argumento devolve **branco**, não erro — replica
/// `args.expect(...).unwrap_or(Component(Ratio::one()))` do vanilla,
/// medido directamente (`luma("bad")`, `luma(300)`, `luma(150%)`,
/// `luma()` → todos `luma(100%)` no vanilla). P1252 completa a forma
/// `luma(l, alpha: alpha)` e a conversão `luma(color)`; nesta última, o cristalino
/// preserva alpha como correção `Known-Upstream-Bug`.
/// ADR-0107: comportamento
/// observável da língua, não mecânica — paridade exige replicar, não
/// substituir por erro.
pub fn native_luma(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let alpha = match args.named.get("alpha") {
        Some(value) => Some(component_to_ratio(value).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                "luma(lightness, alpha: alpha): alpha inválido",
            )]
        })?),
        None => None,
    };
    for key in args.named.keys() {
        if key.as_str() != "alpha" {
            return err(format!("luma(): argumento nomeado inesperado '{}'", key));
        }
    }
    match args.items.as_slice() {
        [] if alpha.is_none() => Ok(Value::Color(Color::luma(1.0))),
        [Value::Color(color)] if alpha.is_none() => {
            Ok(Value::Color((*color).to_space(ColorSpace::Luma)))
        }
        [v] => Ok(Value::Color(Color::Luma {
            l: component_to_ratio(v).unwrap_or(1.0),
            a: alpha.unwrap_or(1.0),
        })),
        _ => err(format!(
            "luma() requer 0 ou 1 argumento posicional, recebeu {} args",
            args.items.len()
        )),
    }
}

fn component_to_ratio(v: &Value) -> Option<f32> {
    match v {
        Value::Int(i) if (0..=255).contains(i) => Some(*i as f32 / 255.0),
        Value::Ratio(r) if (0.0..=1.0).contains(&r.get()) => Some(r.get() as f32),
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
            Some(rel.rel as f32)
        }
        _ => None, // neutro: N16[β] — Value não-numérico/ratio retorna None na coerção de componente de cor
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
            as_f32(l, "l")?,
            as_f32(a, "a")?,
            as_f32(b, "b")?,
            1.0,
        ))),
        [l, a, b, alpha] => Ok(Value::Color(Color::oklab(
            as_f32(l, "l")?,
            as_f32(a, "a")?,
            as_f32(b, "b")?,
            as_f32(alpha, "alpha")?,
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
            as_f32(l, "l")?,
            as_f32(c, "c")?,
            as_f32(h, "h")?,
            1.0,
        ))),
        [l, c, h, alpha] => Ok(Value::Color(Color::oklch(
            as_f32(l, "l")?,
            as_f32(c, "c")?,
            as_f32(h, "h")?,
            as_f32(alpha, "alpha")?,
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
    match args.items.as_slice() {
        [c, m, y, k] => Ok(Value::Color(Color::cmyk(
            as_f32(c, "c")?,
            as_f32(m, "m")?,
            as_f32(y, "y")?,
            as_f32(k, "k")?,
        ))),
        _ => err(format!("cmyk() requer 4 Float/Int, recebeu {} args", args.items.len())),
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
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(l, "l")?,
            1.0,
        ))),
        [h, s, l, a] => Ok(Value::Color(Color::hsl(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(l, "l")?,
            as_f32(a, "a")?,
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
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(v, "v")?,
            1.0,
        ))),
        [h, s, v, a] => Ok(Value::Color(Color::hsv(
            as_f32(h, "h")?,
            as_f32(s, "s")?,
            as_f32(v, "v")?,
            as_f32(a, "a")?,
        ))),
        _ => err(format!(
            "hsv() requer 3 ou 4 Float/Int, recebeu {} args",
            args.items.len()
        )),
    }
}

fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
    match v {
        Value::Float(f) => Ok(*f as f32),
        Value::Int(i) => Ok(*i as f32),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: espera Float/Int, recebeu {}", name, other.type_name()),
        )]),
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
        let mut args = Args::positional(vec![Value::Ratio(Ratio(0.5))]);
        args.named.insert("alpha".into(), Value::Ratio(Ratio(0.4)));
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
    fn percentagem_com_parte_absoluta_cai_no_fallback() {
        // `50% + 1pt` não é um componente de cor válido (nem no vanilla) —
        // deve cair no fallback branco, não ser tratado como 50%.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        assert_eq!(luma(vec![rel]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn int_valido_sem_regressao() {
        assert_eq!(luma(vec![Value::Int(0)]), Value::Color(Color::luma(0.0)));
        assert_eq!(luma(vec![Value::Int(255)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn int_fora_de_gama_devolve_branco_nao_erro() {
        // P705 — corrigido: paridade vanilla medida (luma(300) -> branco).
        assert_eq!(luma(vec![Value::Int(300)]), Value::Color(Color::luma(1.0)));
        assert_eq!(luma(vec![Value::Int(256)]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn ratio_fora_de_gama_devolve_branco_nao_erro() {
        assert_eq!(luma(vec![Value::Ratio(Ratio(1.5))]), Value::Color(Color::luma(1.0)));
        assert_eq!(luma(vec![Value::Ratio(Ratio(-0.2))]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn tipo_errado_devolve_branco_nao_erro() {
        assert_eq!(luma(vec![Value::Str("bad".into())]), Value::Color(Color::luma(1.0)));
    }

    #[test]
    fn sem_argumentos_devolve_branco() {
        assert_eq!(luma(vec![]), Value::Color(Color::luma(1.0)));
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
        assert!(
            e[0].message.contains("requer 0 ou 1 argumento posicional"),
            "msg: {}",
            e[0].message
        );
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
        assert_eq!(component_to_ratio(&v), Some(0.5));
    }

    #[test]
    fn p1043_color_float_component_relative_out_of_range_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 155 - C1=T, C2=F: rel.abs == Length::ZERO && !contains -> None
        let v = Value::Relative(Rel { rel: 1.5, abs: Length::ZERO });
        assert_eq!(component_to_ratio(&v), None);
    }

    #[test]
    fn p1043_color_float_component_relative_nonzero_abs_isolada() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 155 - C1=F, C2=_: rel.abs != Length::ZERO -> None
        let v = Value::Relative(Rel { rel: 0.5, abs: Length::pt(1.0) });
        assert_eq!(component_to_ratio(&v), None);
    }
}
