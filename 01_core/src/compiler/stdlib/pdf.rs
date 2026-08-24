//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/pdf.md
//! @prompt-hash e16570fa
//! @layer L1
//! @updated 2026-07-22
//!
//! Módulo `pdf` — namespace de metadados PDF (P735, paridade parcial
//! medida no vanilla 0.14).
//!
//! Conteúdo medido do namespace vanilla (binário padrão): apenas
//! `pdf.attach` e `pdf.artifact` existem como funções. As funções
//! `pdf.table.summary` / `pdf.table.header-cell` / `pdf.table.data-cell`
//! estão gated na feature `A11yExtras` (off por omissão) — não existem
//! no binário de referência.
//!
//! - `pdf.attach(...)` → **scope-out**: o exportador PDF cristalino não
//!   suporta ficheiros embutidos. Retorna erro explícito em vez de
//!   silenciar (o vanilla emite o attachment no PDF).
//! - `pdf.artifact(body, kind:)` → passthrough do `body`: paridade de
//!   render (o vanilla renderiza o body; a marcação de artefacto no tag
//!   tree é scope-out global do exportador). P826: o named `kind:` é
//!   aceite e validado contra a enumeração `ArtifactKind` do vanilla
//!   (12 valores; erro de cast verbatim).

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::expect_no_named;

/// Constrói o módulo `pdf` como `Value::Module` com `attach` e `artifact`
/// (paridade do conteúdo medido no namespace vanilla).
pub fn make_pdf_module() -> Value {
    let mut scope = crate::entities::scope::Scope::new();
    scope.define("attach", Value::Func(Func::native("pdf.attach", native_pdf_attach)));
    scope.define(
        "artifact",
        Value::Func(Func::native("pdf.artifact", native_pdf_artifact)),
    );
    Value::Module(crate::entities::module::Module::new("pdf", scope))
}

/// `pdf.attach(...)` — scope-out: sem suporte a ficheiros embutidos no
/// exportador PDF cristalino. Erro explícito (o observável é a mensagem).
pub(crate) fn native_pdf_attach(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "pdf.attach: o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)",
    )])
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

/// `pdf.artifact(body)` — passthrough do `body` (paridade de render; o
/// tagging de artefacto é scope-out do exportador).
///
/// P826 — arg nomeado `kind:` (default `"other"` no vanilla). O valor só
/// afecta o tag tree de acessibilidade (scope-out global do exportador
/// cristalino, sem tag tree em `03_infra`); aqui é aceite e **validado**
/// contra a enumeração do vanilla, para paridade de aceitação/erro.
pub(crate) fn native_pdf_artifact(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    match args.named.get("kind") {
        None => {}
        Some(Value::Str(s)) if ARTIFACT_KINDS.contains(&s.as_str()) => {}
        Some(Value::Str(_)) => return super::err(ARTIFACT_KIND_EXPECTED.to_string()),
        Some(other) => {
            return super::err(format!(
                "{ARTIFACT_KIND_EXPECTED}, found {}",
                vanilla_type_name(other)
            ))
        }
    }
    for key in args.named.keys() {
        if key.as_str() != "kind" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{key}'"),
            )]);
        }
    }
    match args.items.as_slice() {
        [body] => Ok(body.clone()),
        _ => super::err(format!(
            "pdf.artifact() requer 1 argumento (body), recebeu {}",
            args.items.len()
        )),
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
        let mut a = Args::positional(vec![Value::Str("corpo".into())]);
        a.named.insert("kind".into(), kind);
        a
    }

    /// P826 — `kind:` com valor válido é aceite (passthrough do body).
    /// Lista medida no vanilla 0.15.0 (`pdf/accessibility.rs:58-98`).
    #[test]
    fn p826_kind_header_aceite_passthrough() {
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Str("header".into())),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Str("corpo".into()));
    }

    #[test]
    fn p826_kind_pagination_other_aceite_passthrough() {
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args_com_kind(Value::Str("pagination-other".into())),
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Str("corpo".into()));
    }

    /// P826 — controlo de regressão: sem `kind:` continua a funcionar.
    #[test]
    fn p826_sem_kind_continua_passthrough() {
        let args = Args::positional(vec![Value::Str("corpo".into())]);
        let v = native_pdf_artifact(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Str("corpo".into()));
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
