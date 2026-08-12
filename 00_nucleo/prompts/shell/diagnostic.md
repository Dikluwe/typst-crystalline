# Shell Diagnostic — typst-shell::diagnostic
Hash do Código: 0c6a6a2a

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
  while calling `<nome>` at <source_path>:<linha>:<coluna>
    <texto fonte do span do tracepoint>
```

Termina com `\n` final. Hints indentados com 2 espaços.

**Call trace (P846, achado #57)**: por cada `Spanned<Tracepoint>` em
`diag.trace` (populado em L1 por `trace_call` — ver `prompts/compiler/eval.md`
§P846), uma linha com 2 espaços no formato verbatim do vanilla
(`typst-kit/src/diagnostics.rs:105-146`): `while calling \`<nome>\`` /
`while calling function` (`Call(None)`) / `while showing <nome> element` /
`while importing \`<nome>\`` / `while including \`<nome>\``, seguida de
` at <path>:<linha>:<col>`; e uma segunda linha com 4 espaços e o texto
fonte do span do tracepoint (span multi-linha: primeira linha + `…` +
último char, se não whitespace). Tracepoints cujo span não resolve no
`Source` passado são omitidos (comportamento do `emit_trace` vanilla).
A ordem é a do vanilla: innermost primeiro. Em modo `colored`, a
localização e o snippet levam `dim` (paleta ADR-0048 — decisão P846; o
vanilla usa underline/cinza, fora da paleta cristalina).

**`colored = false`**: output simples (formato Passo 111, ADR-0045).

**`colored = true`** (paleta ADR-0048):
- `error:` — vermelho bold (`\x1b[1;31m`).
- `warning:` — amarelo bold (`\x1b[1;33m`).
- `hint:` — ciano bold (`\x1b[1;36m`).
- `path:linha:coluna` — dim (`\x1b[2m`).
- message — bold (`\x1b[1m`).

Spans detached ou cujo `span.id()` não corresponde ao `source` passado
caem em `<path>:<detached>:` (ponto final dentro dos backticks — formato
exacto). A resolução do `Source` correcto para spans cross-file é
responsabilidade do caller (L4).

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
