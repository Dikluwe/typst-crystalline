//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/pdf.md
//! @prompt-hash 1628d781
//! @layer L1
//! @updated 2026-07-22
//!
//! Módulo `pdf` — namespace de metadados e acessibilidade PDF.
//!
//! Conteúdo medido do namespace vanilla (binário padrão): apenas
//! `pdf.attach` e `pdf.artifact` existem como funções. As funções
//! `pdf.table.summary` / `pdf.table.header-cell` / `pdf.table.data-cell`
//! estão gated na feature `A11yExtras` (off por omissão) — não existem
//! no binário de referência.
//!
//! - `pdf.attach(...)` produz um carrier semântico com os bytes e os
//!   metadados necessários para o exportador emitir um EmbeddedFile.
//! - `pdf.artifact(body, kind:)` preserva o corpo e transporta uma das
//!   12 classes de artefacto até à marcação do stream PDF.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::compiler_features::{Feature, Features};
use crate::entities::content::Content;
use crate::entities::elements::pdf_artifact::{ArtifactKind, PdfArtifactElem};
use crate::entities::elements::pdf_attach::{AttachedFileRelationship, PdfAttachElem};
use crate::entities::elements::table_cell::{TableCellKind, TableHeaderScope};
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Constrói o módulo `pdf` como `Value::Module` com `attach` e `artifact`
/// (paridade do conteúdo medido no namespace vanilla).
pub fn make_pdf_module(features: Features) -> Value {
    let mut scope = crate::entities::scope::Scope::new();
    scope.define("attach", Value::Func(Func::native("pdf.attach", native_pdf_attach)));
    scope.define(
        "artifact",
        Value::Func(Func::native("pdf.artifact", native_pdf_artifact)),
    );
    if features.contains(Feature::A11yExtras) {
        scope.define(
            "table-summary",
            Value::Func(Func::native("pdf.table-summary", native_pdf_table_summary)),
        );
        scope.define(
            "header-cell",
            Value::Func(Func::native("pdf.header-cell", native_pdf_header_cell)),
        );
        scope.define(
            "data-cell",
            Value::Func(Func::native("pdf.data-cell", native_pdf_data_cell)),
        );
    }
    Value::Module(crate::entities::module::Module::new("pdf", scope))
}

pub(crate) fn native_pdf_table_summary(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if key.as_str() != "summary" {
            return super::err(format!("unexpected argument: {key}"));
        }
    }
    let summary = match args.named.get("summary") {
        None => None,
        Some(Value::Str(value)) => Some(value.clone()),
        Some(other) => {
            return super::err(format!(
                "expected string, found {}",
                vanilla_type_name(other)
            ))
        }
    };
    let Some(table) = args.items.first() else {
        return super::err("missing argument: table");
    };
    let Value::Content(Content::Table(table)) = table else {
        return super::err("expected table");
    };
    if args.items.len() != 1 {
        return super::err("unexpected argument");
    }
    let mut table = (**table).clone();
    table.summary = summary;
    Ok(Value::Content(Content::Table(std::sync::Arc::new(table))))
}

pub(crate) fn native_pdf_header_cell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !matches!(key.as_str(), "level" | "scope") {
            return super::err(format!("unexpected argument: {key}"));
        }
    }
    let level = match args.named.get("level") {
        None => 1,
        Some(Value::Int(value)) if *value > 0 && *value <= u32::MAX as i64 => {
            *value as u32
        }
        Some(Value::Int(_)) => return super::err("number must be positive"),
        Some(other) => {
            return super::err(format!(
                "expected integer, found {}",
                vanilla_type_name(other)
            ))
        }
    };
    let scope = match args.named.get("scope") {
        None => TableHeaderScope::Column,
        Some(Value::Str(value)) if value.as_str() == "both" => TableHeaderScope::Both,
        Some(Value::Str(value)) if value.as_str() == "column" => TableHeaderScope::Column,
        Some(Value::Str(value)) if value.as_str() == "row" => TableHeaderScope::Row,
        Some(Value::Str(_)) => {
            return super::err("expected \"both\", \"column\", or \"row\"")
        }
        Some(other) => {
            return super::err(format!(
                "expected \"both\", \"column\", or \"row\", found {}",
                vanilla_type_name(other)
            ))
        }
    };
    let Some(value) = args.items.first() else {
        return super::err("missing argument: cell");
    };
    if args.items.len() != 1 {
        return super::err("unexpected argument");
    }
    let mut cell = crate::compiler::eval::table::normalize_table_cell(value)?;
    cell.kind = TableCellKind::Header { level, scope };
    Ok(Value::Content(Content::TableCell(std::sync::Arc::new(cell))))
}

pub(crate) fn native_pdf_data_cell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("cell") {
        return super::err("the argument `cell` is positional");
    }
    if let Some(key) = args.named.keys().next() {
        return super::err(format!("unexpected argument: {key}"));
    }
    let Some(value) = args.items.first() else {
        return super::err("missing argument: cell");
    };
    if args.items.len() != 1 {
        return super::err("unexpected argument");
    }
    let mut cell = crate::compiler::eval::table::normalize_table_cell(value)?;
    cell.kind = TableCellKind::Data;
    Ok(Value::Content(Content::TableCell(std::sync::Arc::new(cell))))
}

/// Constrói o carrier semântico de `pdf.attach(...)`, lendo o caminho
/// apenas pela capacidade `World` injetada ou usando os bytes explícitos.
pub(crate) fn native_pdf_attach(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !matches!(key.as_str(), "relationship" | "mime-type" | "description") {
            return super::err(format!("unexpected argument: {key}"));
        }
    }
    let Some(path_value) = args.items.first() else {
        return super::err("missing argument: path");
    };
    let Value::Str(path) = path_value else {
        return super::err(format!(
            "expected path or string, found {}",
            vanilla_type_name(path_value)
        ));
    };
    if args.items.len() > 2 {
        return super::err(format!(
            "unexpected argument: {}",
            vanilla_type_name(&args.items[2])
        ));
    }
    let data = match args.items.get(1) {
        Some(Value::Bytes(bytes)) => std::sync::Arc::new(bytes.as_slice().to_vec()),
        Some(other) => {
            return super::err(format!(
                "expected bytes, found {}",
                vanilla_type_name(other)
            ))
        }
        None => world
            .read_bytes(current_file, path.as_str())
            .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])?,
    };
    let relationship = match args.named.get("relationship") {
        None | Some(Value::None) => None,
        Some(Value::Str(value)) => Some(
            AttachedFileRelationship::parse(value).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "expected \"source\", \"data\", \"alternative\", \"supplement\", or none",
                )]
            })?,
        ),
        Some(_) => {
            return super::err(
                "expected \"source\", \"data\", \"alternative\", \"supplement\", or none",
            )
        }
    };
    let mime_type = match args.named.get("mime-type") {
        None | Some(Value::None) => None,
        Some(Value::Str(value)) if valid_mime_type(value) => Some(value.clone()),
        Some(Value::Str(_)) => return super::err("invalid mime type"),
        Some(other) => {
            return super::err(format!(
                "expected string or none, found {}",
                vanilla_type_name(other)
            ))
        }
    };
    let description = match args.named.get("description") {
        None | Some(Value::None) => None,
        Some(Value::Str(value)) => Some(value.clone()),
        Some(other) => {
            return super::err(format!(
                "expected string or none, found {}",
                vanilla_type_name(other)
            ))
        }
    };

    Ok(Value::Content(Content::pdf_attach(PdfAttachElem {
        path: path.clone(),
        data,
        relationship,
        mime_type,
        description,
    })))
}

fn valid_mime_type(value: &str) -> bool {
    let Some((top, sub)) = value.split_once('/') else { return false };
    let valid_token = |token: &str| {
        !token.is_empty()
            && token.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(
                        byte,
                        b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
                    )
            })
    };
    !sub.contains('/') && valid_token(top) && valid_token(sub)
}

/// Valores válidos do `kind:` de `pdf.artifact`, medidos no vanilla 0.15.0
/// (`lab/typst-original/crates/typst-library/src/pdf/accessibility.rs:58-98`,
/// enum `ArtifactKind` com derive `Cast` — kebab-case).
const ARTIFACT_KINDS: &[&str] = &[
    "header",
    "footer",
    "watermark",
    "page-number",
    "line-number",
    "redaction",
    "bates",
    "page",
    "pagination-other",
    "layout",
    "background",
    "other",
];

/// Mensagem de cast do vanilla para valor fora do domínio de `ArtifactKind`
/// (medida em P826: string inválida sem sufixo "found"; outro tipo com
/// sufixo `, found {tipo}` — mesma convenção de `encoding:` em P824).
const ARTIFACT_KIND_EXPECTED: &str = "expected \"header\", \"footer\", \"watermark\", \
     \"page-number\", \"line-number\", \"redaction\", \"bates\", \"page\", \
     \"pagination-other\", \"layout\", \"background\", or \"other\"";

/// Constrói o carrier semântico de `pdf.artifact(body, kind:)`.
///
/// `kind:` usa `"other"` por omissão e é validado contra a enumeração
/// fechada observada no vanilla; o layout e o exportador preservam essa
/// classificação sem alterar a morfologia do corpo.
pub(crate) fn native_pdf_artifact(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let kind = match args.named.get("kind") {
        None => ArtifactKind::Other,
        Some(Value::Str(s)) if ARTIFACT_KINDS.contains(&s.as_str()) => {
            ArtifactKind::parse(s).unwrap_or(ArtifactKind::Other)
        }
        Some(Value::Str(_)) => return super::err(ARTIFACT_KIND_EXPECTED.to_string()),
        Some(other) => {
            return super::err(format!(
                "{ARTIFACT_KIND_EXPECTED}, found {}",
                vanilla_type_name(other)
            ))
        }
    };
    for key in args.named.keys() {
        if key.as_str() != "kind" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{key}'"),
            )]);
        }
    }
    match args.items.as_slice() {
        [Value::Content(body)] => {
            Ok(Value::Content(Content::pdf_artifact(PdfArtifactElem {
                kind,
                body: body.clone(),
            })))
        }
        [other] => {
            super::err(format!("expected content, found {}", vanilla_type_name(other)))
        }
        [] => super::err("missing argument: body"),
        _ => super::err("unexpected argument"),
    }
}

#[cfg(test)]
mod tests_p826 {
    use super::*;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

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
            test_file_id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<crate::entities::world_types::Bytes> {
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
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn args_com_kind(kind: Value) -> Args {
        let mut a = Args::positional(vec![Value::Content(Content::text("corpo"))]);
        a.named.insert("kind".into(), kind);
        a
    }

    /// P826/P1286 — `kind:` válido preserva body e identidade do artifact.
    /// Lista medida no vanilla 0.15.0 (`pdf/accessibility.rs:58-98`).
    #[test]
    fn p826_kind_header_aceite_com_carrier() {
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Str("header".into())),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        match v {
            Value::Content(Content::PdfArtifact(e)) => {
                assert_eq!(e.kind, ArtifactKind::Header);
                assert_eq!(e.body.plain_text(), "corpo");
            }
            other => panic!("esperado PdfArtifact Header, obtido {other:?}"),
        }
    }

    #[test]
    fn p826_kind_pagination_other_aceite_com_carrier() {
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Str("pagination-other".into())),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        match v {
            Value::Content(Content::PdfArtifact(e)) => {
                assert_eq!(e.kind, ArtifactKind::PaginationOther);
                assert_eq!(e.body.plain_text(), "corpo");
            }
            other => panic!("esperado PdfArtifact PaginationOther, obtido {other:?}"),
        }
    }

    /// P826/P1286 — sem `kind:` usa `Other`, sem colapsar para o body.
    #[test]
    fn p826_sem_kind_usa_other_com_carrier() {
        let args = Args::positional(vec![Value::Content(Content::text("corpo"))]);
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        match v {
            Value::Content(Content::PdfArtifact(e)) => {
                assert_eq!(e.kind, ArtifactKind::Other);
                assert_eq!(e.body.plain_text(), "corpo");
            }
            other => panic!("esperado PdfArtifact Other, obtido {other:?}"),
        }
    }

    /// P826 — string fora do domínio → erro verbatim do vanilla (cast de
    /// `ArtifactKind`; sem sufixo "found", como `encoding:` em P824).
    #[test]
    fn p826_kind_invalido_erro_verbatim_vanilla() {
        let e = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Str("banana".into())),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "expected \"header\", \"footer\", \"watermark\", \"page-number\", \
             \"line-number\", \"redaction\", \"bates\", \"page\", \
             \"pagination-other\", \"layout\", \"background\", or \"other\""
        );
    }

    /// P826 — tipo errado → sufixo `, found {tipo}` (nomes do vanilla).
    #[test]
    fn p826_kind_tipo_errado_erro_com_found() {
        let e = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Int(42)),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap_err();
        assert!(e[0].message.ends_with(", found integer"), "msg: {}", e[0].message);
    }

    /// P826 — named arg desconhecido continua rejeitado.
    #[test]
    fn p826_named_desconhecido_rejeitado() {
        let mut a = args_com_kind(Value::Str("header".into()));
        a.named.insert("foo".into(), Value::Int(1));
        let e = native_pdf_artifact(
            &mut EvalContext::new(),
            &a,
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap_err();
        assert!(e[0].message.contains("foo"), "msg: {}", e[0].message);
    }
}
