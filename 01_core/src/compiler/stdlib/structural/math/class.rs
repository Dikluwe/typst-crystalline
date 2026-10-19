//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/class.md
//! @prompt-hash 1eabe3c8
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.class`.

use super::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// P772y — `math.class` força a classe do body para o espaçamento automático;
// o body mantém o layout normal. Esta unidade só constrói o carrier.

/// `class(class, body)` — emite `Content::MathClassOverride { class, body }`.
/// `class` é obrigatório e uma das **10 strings do cast vanilla**
/// (P825 — `foundations/cast.rs:502-520`; as outras 5 variantes do enum —
/// `alphabetic`, `diacritic`, `glyph-part`, `space`, `special` — são
/// internas e rejeitadas no cast); `body` é obrigatório (content ou
/// string).
pub fn native_math_class(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // **P825** — domínio do cast do vanilla + mensagem verbatim.
    const CAST_DOMAIN: [&str; 10] = [
        "normal",
        "punctuation",
        "opening",
        "closing",
        "fence",
        "large",
        "relation",
        "unary",
        "binary",
        "vary",
    ];
    const CAST_MSG: &str = "expected \"normal\", \"punctuation\", \"opening\", \
         \"closing\", \"fence\", \"large\", \"relation\", \"unary\", \"binary\", \
         or \"vary\"";

    let class_name = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}, found {}", CAST_MSG, vanilla_type_name_class(other)),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige o nome da classe como 1.º argumento posicional"
                    .to_string(),
            )])
        }
    };
    if !CAST_DOMAIN.contains(&class_name.as_str()) {
        return Err(vec![SourceDiagnostic::error(Span::detached(), CAST_MSG)]);
    }
    // Domínio verificado acima — `parse_math_class` reconhece necessariamente.
    let class = crate::entities::math_class::parse_math_class(class_name.as_str())
        .expect("domínio do cast vanilla é subset dos 15 nomes reconhecidos");

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        // P772y — `math.class("relation", sym.suit.heart)`: símbolo
        // Unicode como body (paridade com a conversão de markup, P471).
        Some(Value::Symbol(s)) => Content::Text(s.value.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "class() body espera content, string ou symbol, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige body como 2.º argumento posicional".to_string(),
            )])
        }
    };

    if let Some(k) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("class(): argumento nomeado '{}' não suportado", k),
        )]);
    }

    Ok(Value::Content(Content::math_class_override(class, body)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::content::Content;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl World for NullWorld {
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
    fn internal_class_name_is_rejected_by_public_cast() {
        let args = Args::positional(vec![
            Value::Str("alphabetic".into()),
            Value::Content(Content::text("x")),
        ]);
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let error = native_math_class(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            file,
        )
        .unwrap_err();
        assert_eq!(
            error[0].message,
            r#"expected "normal", "punctuation", "opening", "closing", "fence", "large", "relation", "unary", "binary", or "vary""#
        );
    }
}
