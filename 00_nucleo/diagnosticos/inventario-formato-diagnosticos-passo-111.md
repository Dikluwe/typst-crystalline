# Passo 111.A — Inventário do formato de diagnósticos

**Data**: 2026-04-23
**Propósito**: mapear a API existente de Source/Span/FileId para
planear `span_to_line_col` (L1) e o formatter L3.

---

## Parte 1 — `Source`

**Ficheiro**: `01_core/src/entities/source.rs`

**Campos privados**:
- `id: FileId`
- `text: String`
- `root: SyntaxNode` (CST parseada em `new()`)
- `content_hash: u64` (ADR-0031)

**Métodos públicos relevantes**:
- `new(id: FileId, text: String) -> Self`
- `detached(text: impl Into<String>) -> Self` — usa FileId sentinel = 1.
- `id() -> FileId`
- `text() -> &str`
- `root() -> &SyntaxNode`
- `len_bytes() -> usize`

**Não tem**: `span_to_offset`, `span_to_line_col`, nem resolução de Span.

### Descoberta crítica

Para resolver span → offset, `SyntaxNode` oferece o mecanismo:
- **`LinkedNode::new(root)`** (em `syntax_node.rs:652`) — constrói handle com offset absoluto.
- **`LinkedNode::find(span) -> Option<LinkedNode>`** (syntax_node.rs:687) — busca descendente pelo número do Span.
- **`LinkedNode::offset() -> usize`** (syntax_node.rs:667) — byte offset absoluto do nó.

Para spans raw-range (menos comum), `Span::range()` dá `Option<Range<usize>>` directamente.

---

## Parte 2 — `Span`

**Ficheiro**: `01_core/src/entities/span.rs`

**Campos**: `NonZeroU64` com layout `| 16 bits file_id | 48 bits number |`.

**Tipos de span** (dois formatos):

1. **Numbered span** (`Span::from_number(id, number)`) — usado por
   AST nodes. `number` identifica o nó; offset real resolvido via
   `SyntaxNode::find`.
2. **Raw range span** (`Span::from_range(id, range)`) — codifica o
   range directamente. `Span::range()` devolve `Some(Range<usize>)`.

**Métodos públicos**:
- `detached() -> Span` — sentinel, `DETACHED = 1`.
- `is_detached() -> bool`
- `id() -> Option<FileId>`
- `range() -> Option<Range<usize>>` — **só para raw-range spans**;
  para numbered spans retorna `None`.
- `number()` é `pub(crate)` — não acessível fora do crate.

### Implicações

- `span_to_line_col` pode usar `span.range()` se `Some` (raw-range).
- Caso contrário (numbered), usa `LinkedNode::new(root).find(span)`
  → `linked.offset()`.
- Detached: `Span::is_detached()` → devolver `None`.
- File mismatch: `span.id() != Some(source.id())` → devolver `None`.

---

## Parte 3 — `FileId`

**Ficheiro**: `01_core/src/entities/file_id.rs`

Opaque wrapper `NonZeroU16`. Sem método `path()` em L1.

**Implicação**: o path literal (`"input.typ"`, etc.) fica do lado do
caller L3. O formatter recebe `source_path: &str` como parâmetro.
**Sem mudança em L1** (consistente com ADR-0043: L1 data, L3
apresentação).

---

## Parte 4 — Consumo actual

**`drain_warnings_to_stderr`** em `03_infra/src/integration_tests.rs:108`:

```rust
fn drain_warnings_to_stderr(
    warnings: &[SourceDiagnostic],
) {
    for diag in warnings {
        eprintln!("warning: {:?} {}", diag.span, diag.message);
    }
}
```

Só warnings. Sem Source. Sem path. Só `{:?}` no Span (produz
`Span(N)` opaco).

**Call sites**: nenhum chama `drain_warnings_to_stderr` em produção
hoje — é helper disponibilizado para a CLI futura. `do_eval_with_sink`
devolve `(SourceResult, Vec<SourceDiagnostic>)`; o caller de teste
inspecciona os campos directamente.

**Testes que dependem do formato actual**:
- `sink_canal_formato_minimo` em `integration_tests.rs:2195` —
  **reproduz manualmente** o formato `"warning: {:?} {}"` e valida.
  Actualizar para o formato novo (gcc/clang-compatível).

---

## Parte 5 — SourceDiagnostic

**Ficheiro**: `01_core/src/entities/source_result.rs`

Struct pública:

```rust
pub struct SourceDiagnostic {
    pub severity: Severity,  // enum { Error, Warning }
    pub span: Span,
    pub message: String,
    pub hints: Vec<String>,
    pub trace: Vec<Spanned<Tracepoint>>,
}
```

Formatter tem acesso directo a `severity`, `span`, `message`, `hints`.
`trace` **fica de fora** neste passo.

---

## Parte 6 — Convenção gcc/clang

Formato-alvo literal:

```
input.typ:42:10: error: cannot find value `x` in this scope
input.typ:50:5: warning: unused variable: `y`
  hint: consider using `_y` instead
```

- **Linha 1-indexada**, **coluna 1-indexada** (convenção editor).
- **Coluna em Unicode chars** (code points), não bytes.
- **Hints indentados com 2 espaços** e prefixo `hint:`.
- **Uma linha por diagnóstico principal**, depois N linhas de hints.
- **Severity em minúsculas** (`error`, `warning`).

### Casos especiais

- **Detached span** → substituir `ficheiro:linha:coluna` por
  `ficheiro:<detached>` (honesto sobre a ausência de posição).
- **Cross-file span** (span.id() ≠ source.id()) → mesmo tratamento
  que detached por agora. Aceitar como limitação (spec 111 fora do
  escopo).

---

## Parte 7 — Plano final (inputs para 111.B e 111.C)

### L1 — `Source::span_to_line_col(span) -> Option<(u32, u32)>`

Lógica:

1. Se `span.is_detached()` → `None`.
2. Se `span.id() != Some(self.id())` → `None`.
3. Resolver offset:
   - Se `span.range()` retorna `Some(range)` (raw-range) → usar
     `range.start` como byte offset.
   - Caso contrário, usar `LinkedNode::new(self.root()).find(span)`
     → `linked.offset()`. Se `None`, `None`.
4. Com offset, percorrer `self.text()` contando chars (não bytes):
   - Começar em `(line=1, col=1)`.
   - Para cada `(i, ch)` em `char_indices()`, parar em `i >= offset`.
   - `'\n'` → `line += 1; col = 1`.
   - outros → `col += 1`.

Complexidade: O(text.len()) por chamada.

### L3 — `format_diagnostic(diag, source, source_path) -> String`

Lógica:

1. Derivar severity string.
2. `source.span_to_line_col(diag.span)` → `Some((l, c))` ou `None`.
3. Location string: `"path:l:c"` ou `"path:<detached>"`.
4. Primeira linha: `"{location}: {severity}: {message}\n"`.
5. Para cada hint: `"  hint: {hint}\n"`.

### L3 — `drain_diagnostics_to_stderr(diags, source, source_path)`

Loop `eprint!("{}", format_diagnostic(d, source, source_path))`.
Renomeado de `drain_warnings_to_stderr` (agora cobre errors).

---

## Gate 111.A

**Não disparado**. `Span::range()` + `LinkedNode::find` expõem o
byte offset publicamente. Plano directo.

**Pronto para 111.B (ADR) e 111.C (implementação)**.
