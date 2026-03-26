# Prompt L0 — entities/source_result

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/source_result.rs`
**ADRs relevantes**: ADR-0015 (ecow → String), ADR-0016 (adiamento eval)

## Contexto

`SourceResult<T>` é o tipo de retorno do pipeline do compilador Typst.
`SourceDiagnostic` representa um erro ou aviso emitido durante a compilação.
`Tracepoint` representa um ponto no stack trace de um erro.

Estes tipos são domínio puro: Span + mensagem + hints. Sem I/O.

## Interface pública

```rust
pub enum Severity { Error, Warning }

pub enum Tracepoint {
    Call(Option<String>),
    Show(String),
    Import(String),
    Include(String),
}

pub struct SourceDiagnostic {
    pub severity: Severity,
    pub span:     Span,
    pub message:  String,
    pub hints:    Vec<String>,
    pub trace:    Vec<Spanned<Tracepoint>>,
}

impl SourceDiagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self
    pub fn warning(span: Span, message: impl Into<String>) -> Self
    pub fn with_hint(self, hint: impl Into<String>) -> Self
}

pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;
```

## Substituições aplicadas (ADR-0015)

| Original | Substituição |
|----------|-------------|
| `EcoString` | `String` |
| `EcoVec<SourceDiagnostic>` | `Vec<SourceDiagnostic>` |
| `EcoVec<Spanned<EcoString>>` (hints) | `Vec<String>` |
| `EcoVec<Spanned<Tracepoint>>` (trace) | `Vec<Spanned<Tracepoint>>` |

Hints simplificados para `Vec<String>` (sem Spanned) — span pode ser
adicionado quando eval() real migrar.

## Critérios de Verificação

```
Dado SourceDiagnostic::error(Span::detached(), "msg")
Quando severity for lido
Então Severity::Error

Dado SourceDiagnostic::error(Span::detached(), "msg").with_hint("h")
Quando hints for lido
Então vec!["h"]

Dado SourceResult::Err(vec![diag])
Quando is_err() for chamado
Então true

Dado SourceResult::Ok(42u32)
Quando unwrap() for chamado
Então 42
```
