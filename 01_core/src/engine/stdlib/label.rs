//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/label.md
//! @layer L1
//! @updated 2026-06-25
//!
//! `label(name, body)` — destino nomeado para referências cruzadas (P460).

use crate::contracts::world::World;
use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `label(name, body)` — P460. Emite `Content::Label { name, body }`.
///
/// - 1º arg posicional: nome do destino (`Str`).
/// - 2º arg posicional: body (`Content`). Se omitido, body é `Content::Empty`.
/// - Named args não suportados.
pub fn native_label(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    crate::engine::stdlib::expect_no_named(&args.named)?;
    let name = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.clone(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "label() nome não pode ser vazio".to_string(),
            )])
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("label() espera nome como string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "label() exige nome como argumento posicional".to_string(),
            )])
        }
    };

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "label() espera body como content, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty,
    };

    Ok(Value::Content(Content::label(name, body)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::eval::EvalContext;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
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
    }

    fn call_label(args: Args) -> SourceResult<Value> {
        native_label(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
    }

    #[test]
    fn native_label_emite_content_label() {
        let args = Args::positional(vec![
            Value::Str("sec1".into()),
            Value::Content(Content::text("Corpo")),
        ]);
        let v = call_label(args).unwrap();
        let Value::Content(Content::Label(e)) = v else {
            panic!("esperado Content::Label, recebeu {:?}", v);
        };
        assert_eq!(e.name.as_str(), "sec1");
        assert_eq!(e.body.plain_text(), "Corpo");
    }

    #[test]
    fn native_label_body_vazio_e_valido() {
        let args = Args::positional(vec![Value::Str("empty".into())]);
        let v = call_label(args).unwrap();
        let Value::Content(Content::Label(e)) = v else {
            panic!("esperado Content::Label, recebeu {:?}", v);
        };
        assert_eq!(e.name.as_str(), "empty");
        assert!(e.body.is_empty());
    }
}
