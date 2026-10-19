from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-result.json'))['functional_red'];scoop=json.load(open(out/'scope.json'))
# The pure syntax policy is shared by all entrypoints; sinks remain caller-owned.
Path('01_core/src/compiler/eval/syntax_diagnostics.rs').write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/syntax_diagnostics.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-10-17

use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::{LinkedNode, SyntaxErrorKind, SyntaxNode};

/// Validate an untyped tree before evaluation, preserving source diagnostics.
pub(super) fn validate_syntax(
    root: &SyntaxNode,
    mut emit_warning: impl FnMut(SourceDiagnostic),
) -> SourceResult<()> {
    let mut errors = Vec::new();
    for error in root.errors() {
        let warning_parent = match error.kind {
            SyntaxErrorKind::NoTextWithinStars => Some(SyntaxKind::Strong),
            SyntaxErrorKind::NoTextWithinUnderscores => Some(SyntaxKind::Emph),
            SyntaxErrorKind::Other
            | SyntaxErrorKind::InvalidHexNumber
            | SyntaxErrorKind::InvalidUnicodeCodepoint => None,
        };
        let mut diagnostic = if let Some(kind) = warning_parent {
            let span = LinkedNode::new(root)
                .find(error.span)
                .filter(|node| node.kind() == SyntaxKind::Error && node.is_empty())
                .and_then(|node| {
                    node.parent()
                        .filter(|parent| parent.kind() == kind && parent.len() == 2)
                        .map(|parent| parent.span())
                })
                .unwrap_or(error.span);
            SourceDiagnostic::warning(span, error.message.to_string())
        } else {
            SourceDiagnostic::error(error.span, error.message.to_string())
        };
        diagnostic.hints = error.hints.iter().map(ToString::to_string).collect();
        if warning_parent.is_some() {
            emit_warning(diagnostic);
        } else {
            errors.push(diagnostic);
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
''')
p=Path('01_core/src/compiler/eval/mod.rs');s=p.read_text();assert s.count('mod modules;')==1;s=s.replace('mod modules;','mod modules;\nmod syntax_diagnostics;',1).replace('use crate::entities::syntax_node::{LinkedNode, SyntaxErrorKind, SyntaxNode};','use crate::entities::syntax_node::SyntaxNode;',1)
start=s.index('        if source.root().erroneous() {');end=s.index('        } else {',start);s=s[:start]+'''        if let Err(errors) = syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
            engine.sink.warn_diagnostic(diagnostic);
        }) {
            Err(errors)
'''+s[end:]
start=s.index('    let mut syntax_errors: Vec<SourceDiagnostic>');end=s.index('    // Passo 106 (ADR-0043): canal de warnings activo.',start);s=s[:start]+'''    syntax_diagnostics::validate_syntax(root, |diagnostic| {
        sink.warn_diagnostic(diagnostic);
    })?;

'''+s[end:];p.write_text(s)
p=Path('01_core/src/compiler/eval/modules.rs');s=p.read_text();s=s.replace('/// Avalia um ficheiro importado num **módulo isolado** e devolve o seu [`Module`].','/// Avalia um ficheiro sintaticamente validado num módulo isolado.',1);s=s.replace('''    local_ctx.full_error = ctx.full_error;
    local_ctx.apply_show_rules = false;''','''    local_ctx.full_error = ctx.full_error;
    local_ctx.features = ctx.features;
    local_ctx.target = ctx.target;
    local_ctx.apply_show_rules = false;''',1)
# Trace helper is private to this source-boundary owner.
index=s.index('pub(super) fn eval_module_import(');s=s[:index]+'''fn trace_source_errors(
    mut errors: Vec<SourceDiagnostic>,
    point: Tracepoint,
    span: Span,
) -> Vec<SourceDiagnostic> {
    for error in &mut errors {
        error.trace.push(Spanned::new(point.clone(), span));
    }
    errors
}

'''+s[index:]
mark='''            // Nome do módulo (file_stem) — usado em `Module::new` e no bare import.''';assert s.count(mark)==1;s=s.replace(mark,'''            super::syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
                engine.sink.warn_diagnostic(diagnostic);
            }).map_err(|errors| trace_source_errors(errors, Tracepoint::Import(path.to_string()), import.span()))?;

'''+mark,1)
a='''            let module = eval_imported_file(&source, &module_name, ctx, engine)?;''';assert s.count(a)==1;s=s.replace(a,'''            let module = eval_imported_file(&source, &module_name, ctx, engine)
                .map_err(|errors| trace_source_errors(errors, Tracepoint::Import(path.to_string()), import.span()))?;''',1)
mark='''    // Frame filho: segmento de rota com `id` do módulo incluído.''';assert s.count(mark)==1;s=s.replace(mark,'''    super::syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
        engine.sink.warn_diagnostic(diagnostic);
    }).map_err(|errors| trace_source_errors(errors, Tracepoint::Include(trace_path.to_string()), include.span()))?;

'''+mark,1)
a='''    eval_markup(source.root(), scopes, ctx, &mut local_engine).map_err(|mut errors| {
        for error in &mut errors {
            error.trace.push(Spanned::new(
                Tracepoint::Include(trace_path.to_string()),
                include.span(),
            ));
        }
        errors
    })''';assert s.count(a)==1;s=s.replace(a,'''    eval_markup(source.root(), scopes, ctx, &mut local_engine).map_err(|errors| {
        trace_source_errors(errors, Tracepoint::Include(trace_path.to_string()), include.span())
    })''',1);p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources=scoop['sources']),indent=2)+'\n');print('Shared validation and module context implemented')
