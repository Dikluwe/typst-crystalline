//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/label.md
//! @prompt-hash f7149845
//! @layer L1
//! @updated 2026-06-25
//!
//! `label(name)` — construtor do valor de linguagem `Label` (P1140.2).

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::label::Label;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `label(name)` — constrói um `Value::Label` a partir de uma string não vazia.
pub fn native_label(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    crate::compiler::stdlib::expect_no_named(&args.named)?;
    let name = match args.items.as_slice() {
        [Value::Str(s)] if !s.is_empty() => s.to_string(),
        [Value::Str(_)] => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "label() nome não pode ser vazio".to_string(),
            )])
        }
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("label() espera nome como string, recebeu {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "label() exige nome como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "unexpected argument".to_string(),
            )])
        }
    };

    Ok(Value::Label(Label(name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::eval::EvalContext;
    use crate::entities::args::Args;
    use crate::entities::content::Content;
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
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
    fn p11402_native_label_emite_valor_label() {
        let args = Args::positional(vec![Value::Str("sec1".into())]);
        let v = call_label(args).unwrap();
        let Value::Label(label) = v else {
            panic!("esperado Value::Label, recebeu {:?}", v);
        };
        assert_eq!(label.0, "sec1");
    }

    #[test]
    fn p11402_native_label_rejeita_segundo_posicional() {
        let args = Args::positional(vec![
            Value::Str("sec1".into()),
            Value::Content(Content::text("Corpo")),
        ]);
        assert!(call_label(args).is_err());
    }

    #[test]
    fn p11402_native_label_rejeita_vazio_ausente_tipo_errado_e_nomeado() {
        assert!(call_label(Args::positional(vec![Value::Str("".into())])).is_err());
        assert!(call_label(Args::positional(vec![])).is_err());
        assert!(call_label(Args::positional(vec![Value::Int(1)])).is_err());

        let mut args = Args::positional(vec![Value::Str("sec1".into())]);
        args.named
            .insert("body".into(), Value::Content(Content::text("Corpo")));
        assert!(call_label(args).is_err());
    }
}
