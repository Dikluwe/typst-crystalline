# Shell Diagnostic — typst-shell::diagnostic
Hash do Código: 8f56aca5

## Módulo
`02_shell/src/diagnostic.rs`

## Propósito

Formatter de `SourceDiagnostic` para saída em terminal —
gcc/clang-compatível, com suporte opcional a cores ANSI.

Materializado no Passo 119 (ADR-0050) a partir de
`03_infra/src/diagnostic_format.rs` que foi removido. Razão:
decidir formato user-facing (palavras, cores, indentação) é
concern de apresentação — pertence a L2.

## Contrato

### `format_diagnostic` — API pública

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

Termina com `\n` final. Hints indentados com 2 espaços.

**`colored = false`**: output simples (formato Passo 111, ADR-0045).

**`colored = true`** (paleta ADR-0048):
- `error:` — vermelho bold (`\x1b[1;31m`).
- `warning:` — amarelo bold (`\x1b[1;33m`).
- `hint:` — ciano bold (`\x1b[1;36m`).
- `path:linha:coluna` — dim (`\x1b[2m`).
- message — bold (`\x1b[1m`).

Spans detached ou cross-file caem em `<path>:<detached>:`.

### Constantes ANSI — privadas

6 `const &str` com escapes ANSI. Privadas — usadas só pelo
formatter.

### Decisão `colored` — no caller

O valor do `colored` vem do `RunIntent::colored` (definido em
`typst_shell::cli`). L4 passa-o a cada chamada. Este módulo
**não** lê env vars, **não** verifica isatty.

## Dependências

- `typst_core::entities::source::Source` — resolução de span → linha:col.
- `typst_core::entities::source_result::{Severity, SourceDiagnostic}`.

Sem `clap`, sem `std::io::Write`, sem filesystem. L2 puro.

## Testes

9 testes em `#[cfg(test)] mod tests`:

- **3 sem cores**: `formato_warning_detached_sem_cores`,
  `formato_error_uniforme_sem_cores`, `formato_com_hints_sem_cores`.
- **6 com cores**: `formato_com_cores_contem_ansi_escapes`,
  `formato_com_cores_error_usa_vermelho_bold`,
  `formato_com_cores_warning_usa_amarelo_bold`,
  `formato_com_cores_hint_usa_ciano_bold`,
  `formato_com_cores_cada_span_fecha_com_reset`,
  `formato_com_cores_preserva_conteudo`.

## Evolução

Futuros formatters (JSON, SARIF, codespan-reporting) — novos
módulos em L2 (ex: `diagnostic_json.rs`). Este módulo fica para
saída texto simples.
