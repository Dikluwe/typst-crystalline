# Diagnostic Format — L3 formatter
Hash do Código: c6d6118d

## Módulo
`03_infra/src/diagnostic_format.rs`

## Propósito

Formata `SourceDiagnostic` em texto gcc/clang-compatível para
consumo por terminais e editores (ADR-0045). Aplica cores ANSI
opcionais (ADR-0048) quando `colored: bool` é `true`.

Materializado no Passo 113 (ADR-0046); ganhou cores no Passo 116
(ADR-0048); **perdeu `ColorWhen` e `resolve_colored_with` no
Passo 117 (ADR-0049)** — estes migraram para L2
(`02_shell/src/cli.rs`) onde a CLI vive.

## Contrato

### `format_diagnostic`

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
    colored: bool,
) -> String;
```

Produz:

```text
<source_path>:<linha>:<coluna>: <severity>: <message>
  hint: <hint 1>
  hint: <hint 2>
```

- `colored = false` — texto simples (formato Passo 111, ADR-0045).
- `colored = true` — escapes ANSI (paleta Passo 116, ADR-0048):
  - `error:` — vermelho bold (`\x1b[1;31m`).
  - `warning:` — amarelo bold (`\x1b[1;33m`).
  - `hint:` — ciano bold (`\x1b[1;36m`).
  - `path:linha:coluna` — dim (`\x1b[2m`).
  - `message` — bold (`\x1b[1m`).
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
    colored: bool,
);
```

Loop + `eprint!(format_diagnostic(...))`. Cobre warnings e
errors uniformemente (nome reflecte a uniformidade — ADR-0045
diferencia só pela `severity`).

### Decisão de `colored` — **em L2**

O *valor* de `colored` vem de L2 (`typst_shell::cli::RunIntent`).
L3 recebe o bool; não conhece flags, env vars ou isatty. Separação
honra ADR-0049.

## Limitações (ADR-0045, ADR-0048)

- Sem JSON / SARIF (passos futuros).
- Sem `trace` (stack de Tracepoint — raramente populado hoje).
- Cross-file spans caem em `<detached>`.
- Windows legacy consoles mostram escapes literais (aceite).
- Paleta fixa (sem `--color=256`).

## Layering

Depende apenas de:
- `typst_core::entities::source::Source`
- `typst_core::entities::source_result::{Severity, SourceDiagnostic}`

**Não depende de `clap`** (Passo 117 removeu). L3 é I/O puro.

Sem dependências de `export`, `layout`, `world`. Módulo coeso
pelo domínio "formatação de diagnósticos" (ADR-0037).
