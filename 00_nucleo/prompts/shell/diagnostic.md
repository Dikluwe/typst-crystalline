# Shell Diagnostic — typst-shell::diagnostic
Hash do Código: 0096c215

## Módulo

`02_shell/src/diagnostic.rs`

## Propósito

Formatter humano de `SourceDiagnostic` para terminal, espelhado no vanilla
ratificado `a51e02804`. Pertence a L2 porque decide apresentação user-facing;
não realiza I/O.

Materializado originalmente no Passo 119 (ADR-0050). P1139 substitui o default
gcc/clang curto da ADR-0045 pelo formato humano, preservando a motivação
histórica de usar uma convenção externa estabelecida.

## Tipos públicos

```rust
pub struct DiagnosticSource {
    source: Source,
    name: String,
}

impl DiagnosticSource {
    pub fn new(source: Source, name: impl Into<String>) -> Self;
    pub fn source(&self) -> &Source;
    pub fn name(&self) -> &str;
}
```

`source.id()` é a identidade usada para resolver labels e tracepoints. `name`
é somente a representação user-facing já decidida pelo caller. O tipo possui
dados em memória; não contém path operacional, `World` ou callback.

## Contrato público

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    sources: &[DiagnosticSource],
    colored: bool,
) -> String;
```

### Resolução

- O span principal e cada tracepoint são resolvidos por `span.id()` contra
  `DiagnosticSource::source().id()`.
- Fonte ausente ou span não resolvível não provoca panic nem posição
  inventada; o label ou tracepoint correspondente é omitido.
- Sources repetidos são aceites, mas o primeiro `FileId` vence. L4 deve
  deduplicá-los antes da chamada.
- L2 nunca carrega fontes.

### Formato humano

O bloco principal usa `codespan-reporting 0.11.1` e
`term::Config { tab_width: 2, ..Default::default() }`, equivalentes ao vanilla:

```text
error: unclosed delimiter
  ┌─ input.typ:1:9
  │
1 │ #let x = (
  │          ^
```

- `Severity::Error` e `Severity::Warning` mapeiam para a severidade homónima.
- `diag.message` é emitida sem reescrita textual.
- O span principal resolvível é label primário.
- Como `SourceDiagnostic::hints` é `Vec<String>`, cada hint é uma nota
  `hint: <texto>`; P1139 não fabrica spans.
- O texto termina com newline.
- Um diagnóstico detached continua legível, mas não mostra localização falsa.

### Trace

Somente no formato humano, depois do bloco principal, cada tracepoint
resolvível é emitido na ordem armazenada:

```text
  while including `chapter.typ` at main.typ:4:10
    include "chapter.typ"
```

As palavras vêm de `Tracepoint` sem reescrita por regex. O snippet é o texto do
próprio span. Em span multilinha, usa primeira linha e, se aplicável, `…` mais o
último caractere não whitespace, conforme
`lab/typst-original/crates/typst-kit/src/diagnostics.rs`.

### Cores

- `colored = false`: writer sem ANSI.
- `colored = true`: writer ANSI do `codespan-reporting`.
- Remover ANSI do modo colorido preserva palavras e estrutura do modo sem cor.
- Este módulo não lê env nem detecta TTY; a decisão continua em `cli` conforme
  ADR-0048.

## Dependências

- `typst_core::{Source, SourceDiagnostic, Severity, Tracepoint, FileId}`;
- `codespan-reporting = 0.11.1` em L2;
- `termcolor` pela API reexportada por `codespan-reporting` ou dependência
  direta somente se exigida pelo compilador.

Sem `clap`, `SystemWorld`, filesystem ou escrita em stderr.

## Testes RED obrigatórios

- bloco principal com source e caret;
- warning e error;
- hint sem span como nota;
- tab com largura 2 e Unicode antes do span;
- span multilinha e detached;
- trace `Call`, `Show`, `Import` e `Include`;
- trace cuja fonte difere da fonte principal;
- source ausente omitido sem panic;
- modo colorido equivalente após stripping ANSI.

## Scope-out

- `DiagnosticFormat::Short` e flag pública de formato;
- JSON/SARIF;
- hints spanned no domínio;
- qualquer alteração às mensagens produzidas por L1.
