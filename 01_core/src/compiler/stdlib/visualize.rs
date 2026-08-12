//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md
//! @prompt-hash 89487756
//! @layer L1
//! @updated 2026-06-22
//!
//! Constructor `tiling(...)` e helpers visuais.
//!
//! Passo 396 — materializa `tiling(body)` user-facing. Reusa `Value::Tiling`
//! (P395); zero tipo novo; zero I/O (excepto leitura de path via `world` quando
//! body é `Str`, graded).

use std::sync::Arc;

use crate::contracts::world::World;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Length, Pt, Size};
use crate::entities::ptr_eq_arc::PtrEqArc;
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
    let relative = match args.named.get("relative") {
        Some(v) => Some(parse_relative(v, "tiling")?),
        None => None,
    };

    let body = match body {
        Value::Color(c) => TilingBody::Color(*c),
        Value::Content(Content::Image(img)) => TilingBody::Image((**img).clone()),
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
        Value::Tiling(t) => return Ok(Value::Tiling(Arc::clone(t))),
        Value::Gradient(_) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "gradient em tiling não suportado — scope-out ADR-0054".to_string(),
            )]);
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "tiling(): body deve ser cor, imagem ou caminho, recebeu {}",
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

    Ok(Value::Tiling(Arc::new(tiling)))
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

    match value {
        Value::None | Value::Auto => Ok(None),
        Value::Length(l) => Ok(Some(Size {
            width: Pt(l.abs.to_pt()),
            height: Pt(l.abs.to_pt()),
        })),
        Value::Array(arr) if arr.len() == 2 => {
            let w = length_from_value(&arr[0], fn_name, field)?;
            let h = length_from_value(&arr[1], fn_name, field)?;
            Ok(Some(Size {
                width: Pt(w.abs.to_pt()),
                height: Pt(h.abs.to_pt()),
            }))
        }
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}(): {} deve ser length, array de 2 lengths, none ou auto",
                fn_name, field
            ),
        )]),
    }
}

fn length_from_value(value: &Value, fn_name: &str, field: &str) -> SourceResult<Length> {
    match value {
        Value::Length(l) => Ok(*l),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): {} deve ser length", fn_name, field),
        )]),
    }
}

/// Parse do argumento `relative`.
fn parse_relative(value: &Value, fn_name: &str) -> SourceResult<TilingRelative> {
    match value {
        Value::Str(s) => match s.as_str() {
            "self" => Ok(TilingRelative::Itself),
            "parent" => Ok(TilingRelative::Parent),
            _ => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(): relative deve ser 'self' ou 'parent'", fn_name),
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
            _: Option<i64>,
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
        assert!(r.unwrap_err()[0].message.contains("ADR-0054"));
    }
}
