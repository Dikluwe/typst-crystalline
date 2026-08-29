//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md
//! @prompt-hash 3f27d67f
//! @layer L1
//! @updated 2026-06-22
//!
//! Constructor `tiling(...)` e helpers visuais.
//!
//! Passo 396 — materializa `tiling(body)` user-facing. Reusa `Value::Tiling`
//! (P395); zero tipo novo; zero I/O (excepto leitura de path via `world` quando
//! body é `Str`, graded).

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::axes::Axes;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Angle, Length, Pt, Size};
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::tiling::{Tiling, TilingBody, TilingRelative};
use crate::entities::value::Value;

/// `tiling(body, size:?, relative:?, spacing:?)` → `Value::Tiling`.
///
/// Paridade linguagem (ADR-0107): constrói um pattern fill a partir de cor,
/// imagem ou path. `Gradient` como body é rejeitado — scope-out ADR-0054.
pub fn native_tiling(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(v) => v,
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "tiling() requer 1 argumento posicional (body)".to_string(),
            )]);
        }
    };

    let size = extract_size(args.named.get("size"), "tiling", "size")?;
    let spacing = extract_size(args.named.get("spacing"), "tiling", "spacing")?;
    let offset = extract_offset(args.named.get("offset"))?;
    let angle = extract_angle(args.named.get("angle"))?;
    let relative = match args.named.get("relative") {
        Some(v) => Some(parse_relative(v, "tiling")?),
        None => None,
    };

    let body = match body {
        Value::Color(c) => TilingBody::Color(*c),
        Value::Content(content) => TilingBody::Content(Arc::new(content.clone())),
        Value::Str(path) => {
            // Graded: resolve path via world.read_bytes (reusa P72-74).
            let data = match world.read_bytes(current_file, path.as_str()) {
                Ok(arc) => arc,
                Err(msg) => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("tiling(): não foi possível ler '{}': {}", path, msg),
                    )]);
                }
            };
            TilingBody::Image(crate::entities::elements::image::ImageElem {
                path: path.to_string(),
                data: PtrEqArc(data),
                width: None,
                height: None,
                fit: "cover".into(),
            })
        }
        Value::Tiling(t) if args.named.is_empty() => {
            return Ok(Value::Tiling(Arc::clone(t)))
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "tiling(): body deve ser conteúdo, cor, imagem ou caminho, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };

    // Guarda contra Gradient placeholder (não deveria chegar aqui, mas explícito).
    if matches!(body, TilingBody::Gradient(_)) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient em tiling não suportado — scope-out ADR-0054".to_string(),
        )]);
    }

    let mut tiling = Tiling::new(body);
    if let Some(s) = size {
        tiling.size = Some(s);
    }
    if let Some(s) = spacing {
        tiling.spacing = Some(s);
    }
    if let Some(r) = relative {
        tiling.relative = r;
    }
    tiling.offset = offset;
    tiling.angle = angle;

    Ok(Value::Tiling(Arc::new(tiling)))
}

fn extract_offset(value: Option<&Value>) -> SourceResult<Axes<Rel<Length>>> {
    let Some(value) = value else {
        return Ok(Axes::new(Rel::zero(), Rel::zero()));
    };
    let pair = match value {
        Value::Array(values) if values.len() == 2 => values,
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "tiling(): offset deve ser array de 2 comprimentos relativos".to_string(),
            )]);
        }
    };
    Ok(Axes::new(
        relative_length(&pair[0], "offset")?,
        relative_length(&pair[1], "offset")?,
    ))
}

fn relative_length(value: &Value, field: &str) -> SourceResult<Rel<Length>> {
    let rel = match value {
        Value::Relative(value) => *value,
        Value::Length(value) => Rel { rel: 0.0, abs: *value },
        Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("tiling(): {field} deve conter length, ratio ou relative"),
            )]);
        }
    };
    if !rel.rel.is_finite()
        || !rel.abs.abs.0.is_finite()
        || !rel.abs.em.is_finite()
        || rel.abs.em != 0.0
    {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("tiling(): {field} deve ser finito e não font-relative"),
        )]);
    }
    Ok(rel)
}

fn extract_angle(value: Option<&Value>) -> SourceResult<Angle> {
    let angle = match value {
        None => Angle::rad(0.0),
        Some(Value::Angle(angle)) => *angle,
        Some(_) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "tiling(): angle deve ser angle".to_string(),
            )]);
        }
    };
    if !angle.to_rad().is_finite() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "tiling(): angle deve ser finito".to_string(),
        )]);
    }
    Ok(angle)
}

/// Extrai `Size` a partir de `Length` ou array `[Length, Length]`.
fn extract_size(
    value: Option<&Value>,
    fn_name: &str,
    field: &str,
) -> SourceResult<Option<Size>> {
    let value = match value {
        Some(v) => v,
        None => return Ok(None),
    };

    let size = match value {
        Value::None | Value::Auto => return Ok(None),
        Value::Length(l) => {
            let length = validated_absolute_length(*l, fn_name, field)?;
            Size {
                width: Pt(length.abs.to_pt()),
                height: Pt(length.abs.to_pt()),
            }
        }
        Value::Array(arr) if arr.len() == 2 => {
            let w = length_from_value(&arr[0], fn_name, field)?;
            let h = length_from_value(&arr[1], fn_name, field)?;
            Size {
                width: Pt(w.abs.to_pt()),
                height: Pt(h.abs.to_pt()),
            }
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}(): {} deve ser length, array de 2 lengths, none ou auto",
                    fn_name, field
                ),
            )]);
        }
    };

    if field == "size" && (size.width.0 <= 0.0 || size.height.0 <= 0.0) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): size deve ser estritamente positivo", fn_name),
        )]);
    }
    Ok(Some(size))
}

fn length_from_value(value: &Value, fn_name: &str, field: &str) -> SourceResult<Length> {
    match value {
        Value::Length(l) => validated_absolute_length(*l, fn_name, field),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): {} deve ser length", fn_name, field),
        )]),
    }
}

fn validated_absolute_length(
    length: Length,
    fn_name: &str,
    field: &str,
) -> SourceResult<Length> {
    if length.em != 0.0 || !length.abs.0.is_finite() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): {} deve ser finito e não font-relative", fn_name, field),
        )]);
    }
    Ok(length)
}

/// Parse do argumento `relative`.
fn parse_relative(value: &Value, fn_name: &str) -> SourceResult<TilingRelative> {
    match value {
        Value::Str(s) => match s.as_str() {
            "auto" => Ok(TilingRelative::Auto),
            "self" => Ok(TilingRelative::Itself),
            "parent" => Ok(TilingRelative::Parent),
            _ => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(): relative deve ser 'auto', 'self' ou 'parent'", fn_name),
            )]),
        },
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): relative deve ser string", fn_name),
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::color::Color;
    use crate::entities::layout_types::Length;
    use crate::entities::value::Value;

    fn args(body: Value) -> Args {
        Args::positional(vec![body])
    }

    fn null_world() -> NullWorld {
        NullWorld::default()
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    fn run_tiling(body: Value) -> SourceResult<Value> {
        native_tiling(&mut EvalContext::new(), &args(body), &null_world(), test_file_id())
    }

    /// Mundo nulo para testes que não fazem I/O.
    #[derive(Default)]
    struct NullWorld;

    impl World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            unimplemented!()
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            unimplemented!()
        }
        fn main(&self) -> FileId {
            test_file_id()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            unimplemented!()
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            unimplemented!()
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
        fn read_bytes(&self, _: FileId, _: &str) -> Result<Arc<Vec<u8>>, String> {
            Err("NullWorld".to_string())
        }
    }

    #[test]
    fn tiling_color_body() {
        let v = run_tiling(Value::Color(Color::rgb(255, 0, 0))).unwrap();
        match v {
            Value::Tiling(t) => match &t.body {
                TilingBody::Color(c) => assert_eq!(*c, Color::rgb(255, 0, 0)),
                other => panic!("esperado TilingBody::Color, obteve {:?}", other),
            },
            other => panic!("esperado Value::Tiling, obteve {:?}", other),
        }
    }

    #[test]
    fn p1245_tiling_aceita_content_arbitrario_offset_angle_auto() {
        let body =
            Content::Sequence(Arc::from(vec![Content::text("A"), Content::text("B")]));
        let mut a = args(Value::Content(body.clone()));
        a.named.insert(
            "offset".into(),
            Value::Array(vec![
                Value::Relative(Rel::from_percent(50.0)),
                Value::Length(Length::pt(3.0)),
            ]),
        );
        a.named.insert("angle".into(), Value::Angle(Angle::deg(30.0)));
        a.named.insert("relative".into(), Value::Str("auto".into()));
        let Value::Tiling(t) =
            native_tiling(&mut EvalContext::new(), &a, &null_world(), test_file_id())
                .unwrap()
        else {
            panic!("esperado tiling")
        };
        assert!(matches!(&t.body, TilingBody::Content(content) if **content == body));
        assert_eq!(t.offset.x.rel, 0.5);
        assert_eq!(t.offset.y.abs.abs.0, 3.0);
        assert!((t.angle.to_deg() - 30.0).abs() < 1e-9);
        assert_eq!(t.relative, TilingRelative::Auto);
    }

    #[test]
    fn tiling_size_length_uniform() {
        let mut a = args(Value::Color(Color::rgb(0, 0, 255)));
        a.named.insert("size".into(), Value::Length(Length::pt(50.0)));
        let v = native_tiling(&mut EvalContext::new(), &a, &null_world(), test_file_id())
            .unwrap();
        match v {
            Value::Tiling(t) => {
                let s = t.size.unwrap();
                assert_eq!(s.width, Pt(50.0));
                assert_eq!(s.height, Pt(50.0));
            }
            other => panic!("esperado Value::Tiling, obteve {:?}", other),
        }
    }

    #[test]
    fn tiling_size_array() {
        let mut a = args(Value::Color(Color::rgb(0, 0, 255)));
        a.named.insert(
            "size".into(),
            Value::Array(vec![
                Value::Length(Length::pt(50.0)),
                Value::Length(Length::pt(30.0)),
            ]),
        );
        let v = native_tiling(&mut EvalContext::new(), &a, &null_world(), test_file_id())
            .unwrap();
        match v {
            Value::Tiling(t) => {
                let s = t.size.unwrap();
                assert_eq!(s.width, Pt(50.0));
                assert_eq!(s.height, Pt(30.0));
            }
            other => panic!("esperado Value::Tiling, obteve {:?}", other),
        }
    }

    #[test]
    fn p1245_tiling_rejeita_size_nulo() {
        let mut a = args(Value::Content(Content::text("x")));
        a.named.insert("size".into(), Value::Length(Length::pt(0.0)));
        assert!(native_tiling(
            &mut EvalContext::new(),
            &a,
            &null_world(),
            test_file_id(),
        )
        .is_err());
    }

    #[test]
    fn p1245_tiling_rejeita_medida_font_relative() {
        let mut a = args(Value::Content(Content::text("x")));
        a.named.insert(
            "spacing".into(),
            Value::Array(vec![
                Value::Length(Length::pt(1.0)),
                Value::Length(Length::em(1.0)),
            ]),
        );
        assert!(native_tiling(
            &mut EvalContext::new(),
            &a,
            &null_world(),
            test_file_id(),
        )
        .is_err());
    }

    #[test]
    fn tiling_relative_parent() {
        let mut a = args(Value::Color(Color::rgb(0, 0, 255)));
        a.named.insert("relative".into(), Value::Str("parent".into()));
        let v = native_tiling(&mut EvalContext::new(), &a, &null_world(), test_file_id())
            .unwrap();
        match v {
            Value::Tiling(t) => assert_eq!(t.relative, TilingRelative::Parent),
            other => panic!("esperado Value::Tiling, obteve {:?}", other),
        }
    }

    #[test]
    fn tiling_identity() {
        let t = Tiling::new(TilingBody::Color(Color::rgb(1, 2, 3)));
        let first = Arc::new(t);
        let v = run_tiling(Value::Tiling(Arc::clone(&first))).unwrap();
        match v {
            Value::Tiling(second) => assert!(Arc::ptr_eq(&first, &second)),
            other => panic!("esperado Value::Tiling, obteve {:?}", other),
        }
    }

    #[test]
    fn tiling_invalid_body_errors() {
        let r = run_tiling(Value::Int(123));
        assert!(r.is_err(), "tiling(123) deve falhar");
    }

    #[test]
    fn tiling_gradient_body_errors() {
        use crate::entities::gradient::Gradient;
        use crate::entities::layout_types::Angle;
        let g = Gradient::linear(vec![], Angle::deg(0.0));
        let r = run_tiling(Value::Gradient(g));
        assert!(r.is_err(), "tiling(gradient) deve falhar");
    }
}
