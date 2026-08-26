//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/ref.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-06-25
//!
//! `ref(name, supplement: ?)` — referência cruzada resolvida no layout (P462).

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `ref(name, supplement: ?)` — P462. Emite `Content::Ref { name, supplement }`.
///
/// - 1º arg posicional: nome do label (`Str`).
/// - Named arg `supplement`: conteúdo prefixado ao número (`Content`).
///   Se omitido, usa o supplement default do tipo de elemento referenciado.
pub fn native_ref(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let name = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.clone(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ref() nome não pode ser vazio".to_string(),
            )])
        }
        Some(Value::Label(label)) => label.0.as_str().into(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("ref() espera label ou string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "ref() exige nome como argumento posicional".to_string(),
            )])
        }
    };

    let supplement = match args.named.get("supplement") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.clone())),
        Some(Value::None) => Some(Content::Empty),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "ref() espera supplement como content, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => None,
    };

    let form = match args.named.get("form") {
        None => crate::entities::elements::r#ref::RefForm::Normal,
        Some(Value::Str(s)) if s.as_str() == "normal" => {
            crate::entities::elements::r#ref::RefForm::Normal
        }
        Some(Value::Str(s)) if s.as_str() == "page" => {
            crate::entities::elements::r#ref::RefForm::Page
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "ref() espera form como \"normal\" ou \"page\", recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };

    Ok(Value::Content(Content::reference_with_form(name, supplement, form)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::eval::EvalContext;
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    #[test]
    fn p1140_25_ref_form_page_aceite_e_invalido_rejeitado() {
        let mut ctx = EvalContext::new();
        let world = NullWorld::default();
        let mut args = Args::positional(vec![Value::Label(
            crate::entities::label::Label("x".into()),
        )]);
        args.named.insert("form".into(), Value::Str("page".into()));
        let value = native_ref(
            &mut ctx,
            &args,
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert!(
            matches!(value, Value::Content(Content::Ref(ref elem)) if elem.form == crate::entities::elements::r#ref::RefForm::Page)
        );
        args.named.insert("form".into(), Value::Str("other".into()));
        assert!(native_ref(
            &mut ctx,
            &args,
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        )
        .is_err());
    }

    fn call_ref(args: Args) -> SourceResult<Value> {
        native_ref(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
    }

    #[test]
    fn native_ref_emite_content_ref() {
        let args = Args::positional(vec![Value::Str("sec1".into())]);
        let v = call_ref(args).unwrap();
        let Value::Content(Content::Ref(e)) = v else {
            panic!("esperado Content::Ref, recebeu {:?}", v);
        };
        assert_eq!(e.name.as_str(), "sec1");
        assert!(e.supplement.is_none());
    }

    #[test]
    fn native_ref_preserva_supplement() {
        let sup = Content::text("Section ");
        let mut args = Args::positional(vec![Value::Str("sec1".into())]);
        args.named.insert("supplement".into(), Value::Content(sup.clone()));
        let v = call_ref(args).unwrap();
        let Value::Content(Content::Ref(e)) = v else {
            panic!("esperado Content::Ref, recebeu {:?}", v);
        };
        assert_eq!(e.name.as_str(), "sec1");
        assert_eq!(e.supplement, Some(sup));
    }

    #[test]
    fn native_ref_nome_vazio_erro() {
        let args = Args::positional(vec![Value::Str("".into())]);
        assert!(call_ref(args).is_err());
    }

    #[test]
    fn native_ref_aceita_label() {
        use crate::entities::label::Label;
        let args = Args::positional(vec![Value::Label(Label("sec1".to_string()))]);
        let v = call_ref(args).unwrap();
        let Value::Content(Content::Ref(e)) = v else {
            panic!("esperado Content::Ref, recebeu {:?}", v);
        };
        assert_eq!(e.name.as_str(), "sec1");
        assert!(e.supplement.is_none());
    }

    #[test]
    fn native_ref_arg_nao_label_nem_string_erro() {
        let args = Args::positional(vec![Value::Int(42)]);
        assert!(call_ref(args).is_err());
    }
}
