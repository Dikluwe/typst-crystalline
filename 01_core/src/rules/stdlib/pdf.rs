//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/pdf.md
//! @prompt-hash d30e1058
//! @layer L1
//! @updated 2026-07-14
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
//! - `pdf.artifact(body)` → passthrough do `body`: paridade de render
//!   (o vanilla renderiza o body; a marcação de artefacto no tag tree
//!   é scope-out global do exportador).

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

use super::expect_no_named;

/// Constrói o módulo `pdf` como `Value::Module` com `attach` e `artifact`
/// (paridade do conteúdo medido no namespace vanilla).
pub fn make_pdf_module() -> Value {
    let mut scope = crate::entities::scope::Scope::new();
    scope.define("attach", Value::Func(Func::native("pdf.attach", native_pdf_attach)));
    scope.define("artifact", Value::Func(Func::native("pdf.artifact", native_pdf_artifact)));
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

/// `pdf.artifact(body)` — passthrough do `body` (paridade de render; o
/// tagging de artefacto é scope-out do exportador).
pub(crate) fn native_pdf_artifact(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [body] => Ok(body.clone()),
        _ => super::err(format!(
            "pdf.artifact() requer 1 argumento (body), recebeu {}",
            args.items.len()
        )),
    }
}
