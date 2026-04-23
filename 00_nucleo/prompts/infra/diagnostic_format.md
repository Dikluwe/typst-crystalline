# Diagnostic Format — L3 formatter
Hash do Código: b35dd41e

## Módulo
`03_infra/src/diagnostic_format.rs`

## Propósito

Formata `SourceDiagnostic` em texto gcc/clang-compatível para
consumo por terminais e editores (ADR-0045).

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs` (que surgiram no Passo 111).

## Contrato

### `format_diagnostic`

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String;
```

Produz:

```text
<source_path>:<linha>:<coluna>: <severity>: <message>
  hint: <hint 1>
  hint: <hint 2>
```

- `severity` em minúsculas: `"error"` ou `"warning"`.
- Linha/coluna resolvidas via `Source::span_to_line_col` (L1).
- Spans detached ou cross-file → `<source_path>:<detached>:`.
- Hints indentados com 2 espaços.
- Output termina com `\n` final.

### `drain_diagnostics_to_stderr`

```rust
pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
);
```

Loop + `eprint!(format_diagnostic(...))`. Cobre warnings e
errors uniformemente (nome reflecte a uniformidade — ADR-0045
diferencia só pela `severity`).

## Limitações (ADR-0045)

- Sem cores ANSI (passo futuro).
- Sem JSON / SARIF (passos futuros).
- Sem `trace` (stack de Tracepoint — raramente populado hoje).
- Cross-file spans caem em `<detached>` — caller com mapa
  multi-Source é trabalho futuro.

## Layering

Depende apenas de:
- `typst_core::entities::source::Source`
- `typst_core::entities::source_result::{Severity, SourceDiagnostic}`

Sem dependências de `export`, `layout`, `world`. Módulo coeso
pelo domínio "formatação de diagnósticos" (ADR-0037).
